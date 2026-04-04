//! Network server coordination and management.
//!
//! This module provides the main network server that coordinates all networking
//! components including P2P, synchronization, and RPC services.

use crate::rpc::{RpcConfig as InternalRpcConfig, RpcServer};
use crate::shutdown_impl::{
    DatabaseShutdown, NetworkServerShutdown, RpcServerShutdown, TransactionPoolShutdown,
};
use crate::{
    NetworkConfig, NetworkError, NetworkStats, P2PConfig, P2PEvent, P2pNode, PeerManager, Result,
    SyncEvent, SyncManager,
};
use neo_core::{ShutdownCoordinator, SignalHandler, UInt160};
use neo_ledger::{Blockchain, Ledger};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time::interval;
use tracing::{info, warn};

/// Default Neo network ports
/// Network server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServerConfig {
    /// Node ID
    pub node_id: UInt160,
    /// Network magic number
    pub magic: u32,
    /// P2P configuration
    pub p2p_config: P2PConfig,
    /// RPC configuration
    pub rpc_config: Option<InternalRpcConfig>,
    /// Enable automatic synchronization
    pub enable_auto_sync: bool,
    /// Sync check interval in seconds
    pub sync_check_interval: u64,
    /// Statistics update interval in seconds
    pub stats_interval: u64,
    /// Seed nodes for initial connection
    pub seed_nodes: Vec<SocketAddr>,
}

impl Default for NetworkServerConfig {
    fn default() -> Self {
        Self {
            node_id: UInt160::zero(),
            magic: 0x334f454e, // Neo N3 mainnet magic
            p2p_config: P2PConfig::default(),
            rpc_config: Some(InternalRpcConfig::default()),
            enable_auto_sync: true,
            sync_check_interval: 30,
            stats_interval: 10,
            seed_nodes: vec![
                "127.0.0.1:10333"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                "127.0.0.1:10334"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                "127.0.0.1:10335"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
            ],
        }
    }
}

impl NetworkServerConfig {
    /// Creates a testnet configuration
    pub fn testnet() -> Self {
        Self {
            magic: 0x3554334e, // Neo N3 testnet magic
            p2p_config: P2PConfig {
                listen_address: "127.0.0.1:20333"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                ..Default::default()
            },
            rpc_config: Some(InternalRpcConfig {
                http_address: "127.0.0.1:20332"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                ws_address: Some(
                    "127.0.0.1:20334"
                        .parse()
                        .expect("Failed to parse hardcoded address"),
                ),
                ..Default::default()
            }),
            seed_nodes: vec![
                "127.0.0.1:20333"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                "127.0.0.1:20334"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
            ],
            ..Default::default()
        }
    }

    /// Creates a private network configuration
    pub fn private() -> Self {
        Self {
            magic: 0x12345678, // Custom magic for private network
            p2p_config: P2PConfig {
                listen_address: "127.0.0.1:30333"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                max_peers: 10,
                ..Default::default()
            },
            rpc_config: Some(InternalRpcConfig {
                http_address: "127.0.0.1:30332"
                    .parse()
                    .expect("Failed to parse hardcoded address"),
                ws_address: Some(
                    "127.0.0.1:30334"
                        .parse()
                        .expect("Failed to parse hardcoded address"),
                ),
                ..Default::default()
            }),
            seed_nodes: vec![], // No seed nodes for private network
            ..Default::default()
        }
    }
}

/// Network server events
#[derive(Debug, Clone)]
pub enum NetworkServerEvent {
    /// Server started
    Started,
    /// Server stopped
    Stopped,
    /// P2P event
    P2P(P2PEvent),
    /// Sync event
    Sync(SyncEvent),
    /// Statistics updated
    StatsUpdated(NetworkStats),
}

/// Main network server
pub struct NetworkServer {
    /// Configuration
    config: NetworkServerConfig,
    /// Blockchain reference
    blockchain: Arc<Blockchain>,
    /// P2P node
    p2p_node: Arc<P2pNode>,
    /// Sync manager
    sync_manager: Arc<SyncManager>,
    /// RPC server
    rpc_server: Option<Arc<RpcServer>>,
    /// Event broadcaster
    event_tx: broadcast::Sender<NetworkServerEvent>,
    /// Network statistics
    stats: Arc<RwLock<NetworkStats>>,
    /// Running state
    running: Arc<RwLock<bool>>,
    /// Shutdown coordinator
    shutdown_coordinator: Arc<ShutdownCoordinator>,
}

impl NetworkServer {
    /// Creates a new network server
    pub fn new(config: NetworkServerConfig, blockchain: Arc<Blockchain>) -> Result<Self> {
        // Create network config from P2P config
        let network_config = NetworkConfig {
            magic: config.magic,
            p2p_config: config.p2p_config.clone(),
            listen_address: config.p2p_config.listen_address,
            ..NetworkConfig::default()
        };

        let (_command_sender, command_receiver) = tokio::sync::mpsc::channel(100);

        // Create production P2P node with proper event handling
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(1000);
        let p2p_node = Arc::new(P2pNode::new(network_config.clone(), event_rx)?);

        // Create sync manager with production P2P integration
        let sync_manager = Arc::new(SyncManager::new(blockchain.clone(), p2p_node.clone()));

        // Create composite message handler
        let default_handler = Arc::new(crate::p2p::protocol::DefaultMessageHandler);
        let mut composite_handler =
            crate::composite_handler::CompositeMessageHandler::new(default_handler);

        // Register sync manager for sync-related messages
        composite_handler.register_handlers(vec![
            (
                crate::messages::MessageCommand::Headers,
                sync_manager.clone() as Arc<dyn crate::p2p::protocol::MessageHandler>,
            ),
            (
                crate::messages::MessageCommand::Block,
                sync_manager.clone() as Arc<dyn crate::p2p::protocol::MessageHandler>,
            ),
            (
                crate::messages::MessageCommand::Inv,
                sync_manager.clone() as Arc<dyn crate::p2p::protocol::MessageHandler>,
            ),
        ]);

        // Create the real P2P node with the composite handler
        let p2p_node = Arc::new(P2pNode::new_with_handler(
            network_config,
            command_receiver,
            Arc::new(composite_handler),
        )?);

        let rpc_server = config.rpc_config.as_ref().map(|rpc_config| {
            Arc::new(RpcServer::with_p2p_node(
                rpc_config.clone(),
                blockchain.clone(),
                p2p_node.clone(),
            ))
        });

        let (event_tx, _) = broadcast::channel(1000);
        let shutdown_coordinator = Arc::new(ShutdownCoordinator::new());

        Ok(Self {
            config,
            blockchain,
            p2p_node,
            sync_manager,
            rpc_server,
            event_tx,
            stats: Arc::new(RwLock::new(NetworkStats::default())),
            running: Arc::new(RwLock::new(false)),
            shutdown_coordinator,
        })
    }

    /// Starts the network server
    pub async fn start(&self) -> Result<()> {
        *self.running.write().await = true;

        info!("Starting network server");

        // Register components with shutdown coordinator
        self.register_shutdown_handlers().await?;

        let signal_handler = SignalHandler::new(Arc::clone(&self.shutdown_coordinator));
        signal_handler.start().await;

        // Set sync manager reference in P2P node before starting
        self.p2p_node
            .set_sync_manager(self.sync_manager.clone())
            .await;
        info!("Connected P2P node to sync manager for height updates");

        // Start P2P node
        self.p2p_node.start().await?;

        // Start sync manager
        self.sync_manager.start().await?;

        if let Some(rpc_server) = &self.rpc_server {
            rpc_server.start().await?;
        }

        // Connect to seed nodes
        self.connect_to_seed_nodes().await;

        // Spawn event handlers
        self.spawn_event_handlers().await;

        // Spawn statistics updater
        self.spawn_stats_updater().await;

        // Spawn sync checker
        if self.config.enable_auto_sync {
            self.spawn_sync_checker().await;
        }

        let _ = self.event_tx.send(NetworkServerEvent::Started);

        info!("Network server started successfully");

        Ok(())
    }

    /// Stops the network server
    pub async fn stop(&self) {
        info!("Stopping network server");

        // Initiate graceful shutdown through the coordinator
        if let Err(e) = self
            .shutdown_coordinator
            .initiate_shutdown("NetworkServer stop requested".to_string())
            .await
        {
            warn!("Failed to initiate graceful shutdown: {}", e);

            // Fallback to direct shutdown
            *self.running.write().await = false;

            // Stop components directly
            self.sync_manager.stop().await;
            self.p2p_node.stop().await;

            if let Some(rpc_server) = &self.rpc_server {
                rpc_server.stop().await;
            }
        }

        let _ = self.event_tx.send(NetworkServerEvent::Stopped);

        info!("Network server stopped");
    }

    /// Registers all components with the shutdown coordinator
    async fn register_shutdown_handlers(&self) -> Result<()> {
        info!("Registering shutdown handlers");

        self.shutdown_coordinator
            .register_component(Arc::clone(&self.sync_manager) as Arc<dyn neo_core::Shutdown>)
            .await;

        self.shutdown_coordinator
            .register_component(Arc::clone(&self.p2p_node) as Arc<dyn neo_core::Shutdown>)
            .await;

        let network_wrapper = Arc::new(NetworkServerShutdown::new(
            Arc::clone(&self.p2p_node),
            Arc::clone(&self.sync_manager),
        ));
        self.shutdown_coordinator
            .register_component(network_wrapper)
            .await;

        if let Some(_rpc_server) = &self.rpc_server {
            let rpc_shutdown = Arc::new(RpcServerShutdown::new(
                Arc::clone(&self.running),
                format!(
                    "{}",
                    self.config
                        .rpc_config
                        .as_ref()
                        .ok_or_else(|| NetworkError::configuration(
                            "rpc_config",
                            "RPC configuration not found"
                        ))?
                        .http_address
                ),
            ));
            self.shutdown_coordinator
                .register_component(rpc_shutdown)
                .await;
        }

        let db_shutdown = Arc::new(DatabaseShutdown::new("BlockchainDB".to_string()));
        self.shutdown_coordinator
            .register_component(db_shutdown)
            .await;

        let tx_pool_shutdown = Arc::new(TransactionPoolShutdown::new(
            Arc::new(RwLock::new(0)), // In real implementation, this would track actual pending tx count
        ));
        self.shutdown_coordinator
            .register_component(tx_pool_shutdown)
            .await;

        info!("All shutdown handlers registered");
        Ok(())
    }

    /// Gets the P2P node
    pub fn p2p_node(&self) -> &Arc<P2pNode> {
        &self.p2p_node
    }

    /// Gets the sync manager
    pub fn sync_manager(&self) -> &Arc<SyncManager> {
        &self.sync_manager
    }

    /// Gets the RPC server
    pub fn rpc_server(&self) -> Option<&Arc<RpcServer>> {
        self.rpc_server.as_ref()
    }

    /// Gets network statistics
    pub async fn stats(&self) -> NetworkStats {
        self.stats.read().await.clone()
    }

    /// Gets event receiver
    pub fn event_receiver(&self) -> broadcast::Receiver<NetworkServerEvent> {
        self.event_tx.subscribe()
    }

    /// Gets the shutdown signal that can be awaited
    pub fn shutdown_signal(&self) -> Arc<tokio::sync::Notify> {
        self.shutdown_coordinator.get_shutdown_signal()
    }

    /// Subscribes to shutdown events
    pub fn shutdown_events(&self) -> broadcast::Receiver<neo_core::ShutdownEvent> {
        self.shutdown_coordinator.subscribe_to_events()
    }

    /// Connects to seed nodes
    async fn connect_to_seed_nodes(&self) {
        info!("Connecting to {} seed nodes", self.config.seed_nodes.len());

        for seed_node in &self.config.seed_nodes {
            if let Err(e) = self.p2p_node.connect_peer(*seed_node).await {
                warn!("Failed to connect to seed node {}: {}", seed_node, e);
            } else {
                info!("Connected to seed node: {}", seed_node);
            }
        }
    }

    /// Spawns event handlers
    async fn spawn_event_handlers(&self) {
        let running = self.running.clone();
        let event_tx = self.event_tx.clone();
        let _sync_manager = self.sync_manager.clone();

        // Handle P2P events
        let mut p2p_events = self.p2p_node.event_receiver();
        let p2p_event_tx = event_tx.clone();
        let p2p_running = running.clone();

        tokio::spawn(async move {
            while *p2p_running.read().await {
                match p2p_events.recv().await {
                    Ok(event) => {
                        let _ = p2p_event_tx.send(NetworkServerEvent::P2P(event));
                    }
                    Err(_) => break,
                }
            }
        });

        // Handle sync events
        let mut sync_events = self.sync_manager.event_receiver();
        let sync_event_tx = event_tx.clone();
        let sync_running = running.clone();

        tokio::spawn(async move {
            while *sync_running.read().await {
                match sync_events.recv().await {
                    Ok(event) => {
                        let _ = sync_event_tx.send(NetworkServerEvent::Sync(event));
                    }
                    Err(_) => break,
                }
            }
        });
    }

    /// Spawns statistics updater
    async fn spawn_stats_updater(&self) {
        let running = self.running.clone();
        let stats = self.stats.clone();
        let event_tx = self.event_tx.clone();
        let blockchain = self.blockchain.clone();
        let p2p_node = self.p2p_node.clone();
        let sync_manager = self.sync_manager.clone();
        let interval_secs = self.config.stats_interval;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs));

            while *running.read().await {
                interval.tick().await;

                // Collect statistics
                let current_height = blockchain.get_height().await;
                let peer_stats = p2p_node.peer_manager().get_stats().await;
                let sync_stats = sync_manager.stats().await;

                let network_stats = NetworkStats {
                    peer_count: 0, // Would be calculated from connected peers
                    inbound_connections: peer_stats.inbound_connections as usize,
                    outbound_connections: peer_stats.outbound_connections as usize,
                    bytes_sent: peer_stats.bytes_sent,
                    bytes_received: peer_stats.bytes_received,
                    messages_sent_per_sec: 0.0, // Would calculate from rate
                    messages_received_per_sec: 0.0, // Would calculate from rate
                    average_latency_ms: 0.0,    // Would calculate from ping data
                    sync_status: sync_stats.state.to_string(),
                    current_height,
                    best_known_height: sync_stats.best_known_height,
                };

                *stats.write().await = network_stats.clone();

                let _ = event_tx.send(NetworkServerEvent::StatsUpdated(network_stats));
            }
        });
    }

    /// Spawns sync checker
    async fn spawn_sync_checker(&self) {
        let running = self.running.clone();
        let sync_manager = self.sync_manager.clone();
        let interval_secs = self.config.sync_check_interval;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs));

            while *running.read().await {
                interval.tick().await;

                let sync_state = sync_manager.state().await;
                if sync_state == crate::sync::SyncState::Idle {
                    if let Err(e) = sync_manager.start_sync().await {
                        warn!("Failed to start automatic sync: {}", e);
                    }
                }
            }
        });
    }
}

/// Network server builder for easier configuration
pub struct NetworkServerBuilder {
    config: NetworkServerConfig,
}

impl NetworkServerBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            config: NetworkServerConfig::default(),
        }
    }

    /// Sets the node ID
    pub fn node_id(mut self, node_id: UInt160) -> Self {
        self.config.node_id = node_id;
        self
    }

    /// Sets the network magic
    pub fn magic(mut self, magic: u32) -> Self {
        self.config.magic = magic;
        self
    }

    /// Sets the P2P listen address
    pub fn p2p_address(mut self, address: SocketAddr) -> Self {
        self.config.p2p_config.listen_address = address;
        self
    }

    /// Sets the RPC address
    pub fn rpc_address(mut self, address: SocketAddr) -> Self {
        if let Some(ref mut rpc_config) = self.config.rpc_config {
            rpc_config.http_address = address;
        }
        self
    }

    /// Enables or disables RPC
    pub fn enable_rpc(mut self, enable: bool) -> Self {
        if enable && self.config.rpc_config.is_none() {
            self.config.rpc_config = Some(InternalRpcConfig::default());
        } else if !enable {
            self.config.rpc_config = None;
        }
        self
    }

    /// Sets seed nodes
    pub fn seed_nodes(mut self, seed_nodes: Vec<SocketAddr>) -> Self {
        self.config.seed_nodes = seed_nodes;
        self
    }

    /// Uses testnet configuration
    pub fn testnet(mut self) -> Self {
        self.config = NetworkServerConfig::testnet();
        self
    }

    /// Uses private network configuration
    pub fn private(mut self) -> Self {
        self.config = NetworkServerConfig::private();
        self
    }

    /// Builds the network server
    pub fn build(self, blockchain: Arc<Blockchain>) -> Result<NetworkServer> {
        NetworkServer::new(self.config, blockchain)
    }
}

impl Default for NetworkServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, NetworkResult, PeerInfo};
    use neo_core::UInt160;

    #[test]
    fn test_network_config() {
        let config = NetworkServerConfig::default();
        assert_eq!(config.magic, 0x334f454e);
        assert!(config.enable_auto_sync);
        assert!(!config.seed_nodes.is_empty());

        let testnet_config = NetworkServerConfig::testnet();
        assert_eq!(testnet_config.magic, 0x3554334e);

        let private_config = NetworkServerConfig::private();
        assert_eq!(private_config.magic, 0x12345678);
        assert!(private_config.seed_nodes.is_empty());
    }

    #[test]
    fn test_network_server_builder() -> NetworkResult<()> {
        let builder =
            NetworkServerBuilder::new()
                .node_id(UInt160::zero())
                .magic(0x12345678)
                .p2p_address("127.0.0.1:10333".parse().map_err(|_| {
                    NetworkError::Configuration {
                        parameter: "address".to_string(),
                        reason: "Failed to parse address".to_string(),
                    }
                })?)
                .enable_rpc(true);

        // Would need a blockchain instance to complete the test
        Ok(())
    }
}

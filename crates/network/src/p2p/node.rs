//! Main P2P node implementation.
//!
//! This module implements the main P2P node exactly matching C# Neo's LocalNode functionality.

use crate::{Error, NetworkMessage, NodeInfo, PeerManager, ProtocolMessage, Result};
use neo_core::UInt160;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, RwLock},
    time::timeout,
};
use tracing::{debug, error, info, warn};

use super::{
    config::P2PConfig,
    connection::{ConnectionState, PeerConnection},
    events::P2PEvent,
    protocol::{MessageHandler, ProtocolUtils},
    tasks::TaskManager,
};

/// P2P Node for handling peer connections (matches C# Neo LocalNode exactly)
pub struct P2PNode {
    /// Configuration
    config: P2PConfig,
    
    /// Node information
    node_info: NodeInfo,
    
    /// Network magic number
    magic: u32,
    
    /// Peer manager
    peer_manager: Arc<PeerManager>,
    
    /// Active connections
    connections: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
    
    /// Event broadcaster
    event_tx: broadcast::Sender<P2PEvent>,
    
    /// Message handlers  
    message_handlers: Arc<RwLock<HashMap<String, Box<dyn MessageHandler>>>>,
    
    /// Running state
    running: Arc<RwLock<bool>>,
    
    /// Task manager for background operations
    task_manager: TaskManager,
}

impl P2PNode {
    /// Creates a new P2P node
    pub fn new(config: P2PConfig, node_info: NodeInfo, magic: u32) -> Self {
        let peer_manager = Arc::new(PeerManager::new(config.max_peers));
        let (event_tx, _) = broadcast::channel(config.message_buffer_size);
        let task_manager = TaskManager::new();

        Self {
            config,
            node_info,
            magic,
            peer_manager,
            connections: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
            task_manager,
        }
    }

    /// Starts the P2P node (matches C# LocalNode.Start exactly)
    pub async fn start(&self) -> Result<()> {
        *self.running.write().await = true;

        info!("Starting P2P node on {}", self.config.listen_address);

        // Validate configuration
        self.config.validate()
            .map_err(|e| NetworkError::Configuration { 
                parameter: "p2p_config".to_string(), 
                reason: format!("Invalid P2P config: {}", e) 
            })?;

        let listener = TcpListener::bind(self.config.listen_address).await
            .map_err(|e| NetworkError::ConnectionFailed { 
                address: self.config.listen_address, 
                reason: format!("Failed to bind listener: {}", e) 
            })?;

        // Start background tasks
        self.task_manager.start_connection_acceptor(
            listener, 
            self.connections.clone(),
            self.event_tx.clone(),
            self.running.clone()
        ).await;

        self.task_manager.start_ping_manager(
            self.connections.clone(),
            self.config.ping_interval,
            self.magic,
            self.running.clone()
        ).await;

        self.task_manager.start_connection_manager(
            self.connections.clone(),
            self.config.connection_timeout,
            self.running.clone()
        ).await;

        info!("P2P node started successfully");
        Ok(())
    }

    /// Stops the P2P node (matches C# LocalNode.Stop exactly)
    pub async fn stop(&self) {
        info!("Stopping P2P node");
        *self.running.write().await = false;

        // Stop all background tasks
        self.task_manager.stop_all().await;

        // Disconnect all peers
        let connections = self.connections.read().await;
        for address in connections.keys() {
            self.disconnect_peer(*address, "Node shutting down".to_string()).await;
        }

        info!("P2P node stopped");
    }

    /// Connects to a peer (matches C# LocalNode.ConnectToPeer exactly)
    pub async fn connect_peer(&self, address: SocketAddr) -> Result<()> {
        info!("Connecting to peer: {}", address);

        if self.connections.read().await.contains_key(&address) {
            warn!("Already connected to {}", address);
            return Ok(());
        }

        // Check peer manager limits
        if !self.peer_manager.can_connect_to(address).await {
            return Err(NetworkError::PeerNotConnected { address });
        }

        // Establish TCP connection
        let stream = timeout(self.config.connection_timeout, TcpStream::connect(address))
            .await
            .map_err(|_| NetworkError::ConnectionTimeout { 
                address, 
                timeout_ms: self.config.connection_timeout.as_millis() as u64 
            })?
            .map_err(|e| NetworkError::ConnectionFailed { 
                address, 
                reason: format!("Failed to connect: {}", e) 
            })?;

        // Create connection
        let connection = PeerConnection::new(stream, address, false);

        // Store connection
        self.connections.write().await.insert(address, connection);

        // Start connection handler
        self.task_manager.start_connection_handler(
            address,
            self.connections.clone(),
            self.event_tx.clone(),
            self.magic,
            self.node_info.clone()
        ).await;

        // Initiate handshake
        if let Err(e) = self.start_handshake(address).await {
            error!("Failed to start handshake with {}: {}", address, e);
            self.connections.write().await.remove(&address);
            return Err(e);
        }

        info!("Initiated handshake with {}", address);
        Ok(())
    }

    /// Disconnects from a peer (matches C# LocalNode.DisconnectPeer exactly)
    pub async fn disconnect_peer(&self, address: SocketAddr, reason: String) {
        info!("Disconnecting from peer {}: {}", address, reason);

        if let Some(mut connection) = self.connections.write().await.remove(&address) {
            connection.set_state(ConnectionState::Disconnected);

            if let Some(node_info) = &connection.node_info {
                let _ = self.event_tx.send(P2PEvent::PeerDisconnected {
                    peer_id: node_info.id,
                    address,
                    reason: reason.clone(),
                });
            }

            // Close the connection
            let _ = connection.close().await;
        }

        let _ = self.peer_manager.disconnect_peer(address).await;
    }

    /// Sends a message to a specific peer (matches C# LocalNode.SendMessage exactly)
    pub async fn send_message(&self, address: SocketAddr, message: NetworkMessage) -> Result<()> {
        let mut connections = self.connections.write().await;
        if let Some(connection) = connections.get_mut(&address) {
            connection.send_message(message).await?;
        } else {
            return Err(NetworkError::PeerNotConnected { address });
        }

        Ok(())
    }

    /// Broadcasts a message to all connected peers (matches C# LocalNode.BroadcastMessage exactly)
    pub async fn broadcast_message(&self, message: NetworkMessage) -> Result<()> {
        let connections = self.connections.read().await;
        let mut success_count = 0;
        let total_count = connections.len();

        for (address, _) in connections.iter() {
            if let Err(e) = self.send_message(*address, message.clone()).await {
                warn!("Failed to send message to {}: {}", address, e);
            } else {
                success_count += 1;
            }
        }

        info!("Broadcasted message to {}/{} peers", success_count, total_count);
        Ok(())
    }

    /// Registers a message handler (matches C# LocalNode.RegisterHandler exactly)
    pub async fn register_handler<H>(&self, message_type: String, handler: H)
    where
        H: MessageHandler + Send + Sync + 'static,
    {
        debug!("Registered handler for message type: {}", message_type);
        self.message_handlers.write().await.insert(message_type, Box::new(handler));
    }

    /// Gets an event receiver (matches C# LocalNode.EventReceiver exactly)
    pub fn event_receiver(&self) -> broadcast::Receiver<P2PEvent> {
        self.event_tx.subscribe()
    }

    /// Gets peer manager reference
    pub fn peer_manager(&self) -> &Arc<PeerManager> {
        &self.peer_manager
    }

    /// Gets the network magic number
    pub fn magic(&self) -> u32 {
        self.magic
    }

    /// Gets connected peer count
    pub async fn connected_peer_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Gets connection information for all peers
    pub async fn get_connection_info(&self) -> Vec<super::connection::ConnectionInfo> {
        self.connections.read().await
            .values()
            .map(|conn| conn.connection_info())
            .collect()
    }

    /// Sends GetData request to a peer (production implementation matching C# Neo)
    pub async fn send_get_data(&self, address: SocketAddr, inventory: Vec<crate::InventoryItem>) -> Result<()> {
        let get_data_message = crate::ProtocolMessage::GetData { inventory };
        let network_message = NetworkMessage::new(get_data_message);
        self.send_message(address, network_message).await
    }

    /// Broadcasts inventory to all peers except source (production implementation matching C# Neo)
    pub async fn broadcast_inventory(&self, inventory: Vec<crate::InventoryItem>, exclude_peer: Option<SocketAddr>) -> Result<()> {
        let inv_message = crate::ProtocolMessage::Inv { inventory };
        let network_message = NetworkMessage::new(inv_message);
        
        let connections = self.connections.read().await;
        let mut success_count = 0;
        let total_count = connections.len();
        
        for &address in connections.keys() {
            if let Some(exclude) = exclude_peer {
                if address == exclude {
                    continue; // Skip the source peer
                }
            }
            
            if let Err(e) = self.send_message(address, network_message.clone()).await {
                warn!("Failed to broadcast inventory to {}: {}", address, e);
            } else {
                success_count += 1;
            }
        }
        
        info!("Broadcasted inventory to {}/{} peers", success_count, total_count);
        Ok(())
    }

    /// Sends headers to a peer (production implementation matching C# Neo)
    pub async fn send_headers(&self, address: SocketAddr, headers: Vec<neo_ledger::BlockHeader>) -> Result<()> {
        let headers_message = crate::ProtocolMessage::Headers { headers };
        let network_message = NetworkMessage::new(headers_message);
        self.send_message(address, network_message).await
    }

    /// Sends block to a peer (production implementation matching C# Neo)
    pub async fn send_block(&self, address: SocketAddr, block: neo_ledger::Block) -> Result<()> {
        let block_message = crate::ProtocolMessage::Block { block };
        let network_message = NetworkMessage::new(block_message);
        self.send_message(address, network_message).await
    }

    /// Sends transaction to a peer (production implementation matching C# Neo)
    pub async fn send_transaction(&self, address: SocketAddr, transaction: neo_core::Transaction) -> Result<()> {
        let tx_message = crate::ProtocolMessage::Tx { transaction };
        let network_message = NetworkMessage::new(tx_message);
        self.send_message(address, network_message).await
    }

    /// Starts handshake with a peer (matches C# LocalNode.StartHandshake exactly)
    async fn start_handshake(&self, address: SocketAddr) -> Result<()> {
        // Create version message
        let version_message = ProtocolMessage::version(
            &self.node_info,
            self.config.listen_address.port(),
            true,
        );
        let network_message = NetworkMessage::new(version_message);

        // Send through the stored connection
        if let Some(connection) = self.connections.write().await.get_mut(&address) {
            connection.set_state(ConnectionState::Handshaking);
            connection.send_message(network_message).await?;
        } else {
            return Err(NetworkError::PeerNotConnected { address });
        }

        Ok(())
    }

    /// Handles incoming messages (matches C# LocalNode.HandleMessage exactly)
    pub async fn handle_message(&self, address: SocketAddr, message: NetworkMessage) -> Result<()> {
        debug!("Received message from {}: {:?}", address, message.header.command);

        // Validate message first
        ProtocolUtils::validate_message(&message)?;

        // Handle the message based on its type
        match &message.payload {
            ProtocolMessage::Version {
                version,
                services: _,
                timestamp: _,
                port: _,
                nonce,
                user_agent,
                start_height,
                relay: _,
            } => {
                self.handle_version_message(address, *version, *nonce, user_agent, *start_height).await?;
            }
            
            ProtocolMessage::Verack => {
                self.handle_verack_message(address).await?;
            }
            
            ProtocolMessage::Ping { nonce } => {
                self.handle_ping_message(address, *nonce).await?;
            }
            
            ProtocolMessage::Pong { nonce } => {
                self.handle_pong_message(address, *nonce).await?;
            }
            
            _ => {
                // Handle through registered message handlers
                self.handle_through_registered_handlers(address, &message).await?;
            }
        }

        // Emit message received event
        let peer_id = self.get_peer_id_for_connection(address).await;
        let _ = self.event_tx.send(P2PEvent::MessageReceived {
            peer_id,
            message,
        });

        Ok(())
    }

    /// Handles version message (matches C# LocalNode.HandleVersion exactly)
    async fn handle_version_message(
        &self, 
        address: SocketAddr, 
        version: u32, 
        nonce: u32, 
        user_agent: &str,
        start_height: u32
    ) -> Result<()> {
        info!("Received Version from {}: v{}, user_agent={}, height={}", 
              address, version, user_agent, start_height);
        
        // Generate peer ID
        let peer_id = ProtocolUtils::generate_peer_id(address, nonce, user_agent).await;
        
        // Update connection with peer information
        if let Some(connection) = self.connections.write().await.get_mut(&address) {
            let mut node_info = self.node_info.clone();
            node_info.id = peer_id;
            connection.set_node_info(node_info);
        }

        // Send verack response
        let verack = ProtocolMessage::Verack;
        let verack_msg = NetworkMessage::new(verack);
        self.send_message(address, verack_msg).await?;

        // Emit events
        let _ = self.event_tx.send(P2PEvent::PeerVersion {
            address,
            version,
            user_agent: user_agent.to_string(),
            start_height,
        });

        let _ = self.event_tx.send(P2PEvent::PeerHeight {
            address,
            height: start_height,
        });

        Ok(())
    }

    /// Handles verack message (matches C# LocalNode.HandleVerack exactly)
    async fn handle_verack_message(&self, address: SocketAddr) -> Result<()> {
        info!("Received Verack from {}", address);
        
        // Mark peer as ready
        if let Some(connection) = self.connections.write().await.get_mut(&address) {
            connection.set_state(ConnectionState::Ready);
            
            if let Some(node_info) = &connection.node_info {
                let _ = self.event_tx.send(P2PEvent::HandshakeCompleted {
                    peer_id: node_info.id,
                    address,
                    node_info: node_info.clone(),
                });
            }
        }

        Ok(())
    }

    /// Handles ping message (matches C# LocalNode.HandlePing exactly)
    async fn handle_ping_message(&self, address: SocketAddr, nonce: u32) -> Result<()> {
        debug!("Received Ping from {}: nonce={}", address, nonce);
        
        // Send pong response
        let pong = ProtocolMessage::pong(nonce);
        let pong_msg = NetworkMessage::new(pong);
        self.send_message(address, pong_msg).await?;

        Ok(())
    }

    /// Handles pong message (matches C# LocalNode.HandlePong exactly)
    async fn handle_pong_message(&self, address: SocketAddr, nonce: u32) -> Result<()> {
        debug!("Received Pong from {}: nonce={}", address, nonce);
        
        // Update peer manager with ping response
        if let Some(rtt) = self.peer_manager.complete_ping(address, nonce).await {
            let _ = self.event_tx.send(P2PEvent::PingCompleted {
                address,
                rtt_ms: rtt,
            });
        }

        Ok(())
    }

    /// Handles messages through registered handlers
    async fn handle_through_registered_handlers(&self, address: SocketAddr, message: &NetworkMessage) -> Result<()> {
        let handlers = self.message_handlers.read().await;
        let command_str = message.header.command.to_string();
        if let Some(handler) = handlers.get(&command_str) {
            handler.handle_message(address, message).await?;
        } else {
            debug!("No handler registered for message type: {}", command_str);
        }
        Ok(())
    }

    /// Gets peer ID for connection
    async fn get_peer_id_for_connection(&self, address: SocketAddr) -> UInt160 {
        if let Some(connection) = self.connections.read().await.get(&address) {
            if let Some(node_info) = &connection.node_info {
                return node_info.id;
            }
        }
        UInt160::zero()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Message, Peer, NetworkError};
    use crate::NodeInfo;
    use super::*;
    use tokio::sync::mpsc;
    use crate::{NetworkConfig, P2pNode};
    use std::net::SocketAddr;
    use neo_core::UInt160;

    #[tokio::test]
    async fn test_p2p_node_creation() {
        let config = P2PConfig::default();
        let node_info = NodeInfo {
            id: UInt160::zero(),
            version: crate::ProtocolVersion::new(3, 5, 0),
            user_agent: "neo-rs/test".to_string(),
            capabilities: vec![],
            start_height: 0,
            timestamp: 0,
            nonce: 0,
        };
        let magic = 0x334f454e; // Neo N3 MainNet magic
        
        let node = P2PNode::new(config, node_info, magic);
        
        assert_eq!(node.magic(), magic);
        assert_eq!(node.connected_peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_peer_connection_tracking() {
        let config = P2PConfig::default();
        let node_info = NodeInfo {
            id: UInt160::zero(),
            version: crate::ProtocolVersion::new(3, 5, 0),
            user_agent: "neo-rs/test".to_string(),
            capabilities: vec![],
            start_height: 0,
            timestamp: 0,
            nonce: 0,
        };
        let magic = 0x334f454e;
        
        let node = P2PNode::new(config, node_info, magic);
        
        assert_eq!(node.connected_peer_count().await, 0);
        
        let conn_info = node.get_connection_info().await;
        assert!(conn_info.is_empty());
    }
} 
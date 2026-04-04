//! Network Component Shutdown Implementations
//!
//! This module provides shutdown implementations for all network components,
//! ensuring graceful shutdown in the correct order.

use crate::{P2pNode, PeerManager, SyncManager};
use neo_core::{Shutdown, ShutdownError};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Default Neo network ports
/// Shutdown implementation for PeerManager
#[async_trait::async_trait]
impl Shutdown for PeerManager {
    fn name(&self) -> &str {
        "PeerManager"
    }

    fn shutdown_priority(&self) -> u32 {
        40 // Stop during network shutdown stage
    }

    async fn can_shutdown(&self) -> bool {
        let connected_peers = self.get_connected_peers().await;
        let active_peers = connected_peers.len();

        if active_peers > 0 {
            debug!("PeerManager has {} active peers", active_peers);
        }

        true
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down PeerManager");

        // 1. Stop the peer manager (this handles setting is_running to false)
        if let Err(e) = self.stop().await {
            warn!("Error during PeerManager stop: {}", e);
        }

        // 2. Wait for error handler maintenance to complete
        let error_handler = self.error_handler();
        debug!("Performing final error handler maintenance");
        error_handler.perform_maintenance().await;

        info!("PeerManager shutdown complete");
        Ok(())
    }
}

/// Shutdown implementation for P2pNode
#[async_trait::async_trait]
impl Shutdown for P2pNode {
    fn name(&self) -> &str {
        "P2pNode"
    }

    fn shutdown_priority(&self) -> u32 {
        35 // Shutdown before PeerManager
    }

    async fn can_shutdown(&self) -> bool {
        let status = self.get_status().await;

        match status {
            crate::NodeStatus::Starting => {
                debug!("P2pNode is still starting up");
                false
            }
            _ => true,
        }
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down P2pNode");

        // Call the stop method which handles graceful shutdown
        if let Err(e) = self.stop().await {
            return Err(ShutdownError::ComponentError(format!("P2pNode: {}", e)));
        }

        info!("P2pNode shutdown complete");
        Ok(())
    }
}

/// Shutdown implementation for SyncManager
#[async_trait::async_trait]
impl Shutdown for SyncManager {
    fn name(&self) -> &str {
        "SyncManager"
    }

    fn shutdown_priority(&self) -> u32 {
        30 // Shutdown before P2P components
    }

    async fn can_shutdown(&self) -> bool {
        let status = self.stats().await;

        match status.state {
            crate::sync::SyncState::SyncingHeaders | crate::sync::SyncState::SyncingBlocks => {
                let progress = if status.best_known_height > 0 {
                    status.current_height as f64 / status.best_known_height as f64 * 100.0
                } else {
                    0.0
                };
                debug!("SyncManager is syncing: {:.1}% complete", progress);

                if progress < 95.0 {
                    warn!("Shutting down while sync is only {:.1}% complete", progress);
                }
                true
            }
            _ => true,
        }
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down SyncManager");

        // 1. Stop the sync manager (this handles the shutdown internally)
        self.stop().await;

        // 2. Save final sync status
        let final_status = self.stats().await;
        info!(
            "Final sync status: height={}/{}, state={}",
            final_status.current_height, final_status.best_known_height, final_status.state
        );

        info!("SyncManager shutdown complete");
        Ok(())
    }
}

/// Network server shutdown wrapper
/// Represents a data structure.
pub struct NetworkServerShutdown {
    p2p_node: Arc<P2pNode>,
    sync_manager: Arc<SyncManager>,
}

impl NetworkServerShutdown {
    /// Creates a new network server shutdown wrapper
    /// Creates a new instance.
    pub fn new(p2p_node: Arc<P2pNode>, sync_manager: Arc<SyncManager>) -> Self {
        Self {
            p2p_node,
            sync_manager,
        }
    }
}

#[async_trait::async_trait]
impl Shutdown for NetworkServerShutdown {
    fn name(&self) -> &str {
        "NetworkServer"
    }

    fn shutdown_priority(&self) -> u32 {
        25 // Coordinate network component shutdown
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down NetworkServer components");

        // Components will be shut down in priority order by the coordinator

        info!("NetworkServer shutdown coordination complete");
        Ok(())
    }
}

/// RPC server shutdown implementation
/// Represents a data structure.
pub struct RpcServerShutdown {
    is_running: Arc<RwLock<bool>>,
    bind_address: String,
}

impl RpcServerShutdown {
    /// Creates a new instance.
    pub fn new(is_running: Arc<RwLock<bool>>, bind_address: String) -> Self {
        Self {
            is_running,
            bind_address,
        }
    }
}

#[async_trait::async_trait]
impl Shutdown for RpcServerShutdown {
    fn name(&self) -> &str {
        "RpcServer"
    }

    fn shutdown_priority(&self) -> u32 {
        60 // Stop during RPC shutdown stage
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down RpcServer on {}", self.bind_address);

        // Stop accepting new RPC requests
        *self.is_running.write().await = false;

        // In a full implementation, this would:
        // 1. Stop the HTTP/WebSocket server
        // 2. Complete any in-flight requests
        // 3. Close all connections

        info!("RpcServer shutdown complete");
        Ok(())
    }
}

/// Database shutdown implementation
/// Represents a data structure.
pub struct DatabaseShutdown {
    name: String,
    // In a real implementation, this would hold the database connection
}

impl DatabaseShutdown {
    /// Creates a new instance.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait::async_trait]
impl Shutdown for DatabaseShutdown {
    fn name(&self) -> &str {
        &self.name
    }

    fn shutdown_priority(&self) -> u32 {
        120 // Close during database shutdown stage
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down database: {}", self.name);

        // In a full implementation, this would:
        // 1. Flush any pending writes
        // 2. Close all database connections
        // 3. Ensure data integrity

        info!("Database {} shutdown complete", self.name);
        Ok(())
    }
}

/// Transaction pool shutdown implementation
/// Represents a data structure.
pub struct TransactionPoolShutdown {
    pending_count: Arc<RwLock<usize>>,
}

impl TransactionPoolShutdown {
    /// Creates a new instance.
    pub fn new(pending_count: Arc<RwLock<usize>>) -> Self {
        Self { pending_count }
    }
}

#[async_trait::async_trait]
impl Shutdown for TransactionPoolShutdown {
    fn name(&self) -> &str {
        "TransactionPool"
    }

    fn shutdown_priority(&self) -> u32 {
        80 // Flush during transaction flush stage
    }

    async fn can_shutdown(&self) -> bool {
        let count = *self.pending_count.read().await;
        if count > 0 {
            debug!("TransactionPool has {} pending transactions", count);
        }
        true // Can always shutdown, but log if we have pending transactions
    }

    async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
        info!("Shutting down TransactionPool");

        let pending = *self.pending_count.read().await;
        if pending > 0 {
            warn!("Shutting down with {} pending transactions", pending);

            // In a full implementation, this would:
            // 1. Stop accepting new transactions
            // 2. Try to persist pending transactions
            // 3. Clear the pool
        }

        *self.pending_count.write().await = 0;

        info!("TransactionPool shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::NetworkConfig;
    use crate::{NetworkError, NetworkResult};
    use crate::{P2pNode, PeerManager, SyncManager};
    use neo_core::{Shutdown, ShutdownCoordinator};
    use neo_ledger::Blockchain;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::mpsc;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_peer_manager_shutdown() {
        let config = NetworkConfig::testnet();
        let peer_manager =
            PeerManager::new(config).expect("Failed to create peer manager for test");

        // Test shutdown
        let result = peer_manager.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_peer_manager_can_shutdown() {
        let config = NetworkConfig::testnet();
        let peer_manager = PeerManager::new(config).expect("operation should succeed");

        // Should be able to shutdown when no peers are connected
        let can_shutdown = peer_manager.can_shutdown().await;
        assert!(can_shutdown);
    }

    #[tokio::test]
    async fn test_peer_manager_shutdown_name() {
        let config = NetworkConfig::testnet();
        let peer_manager = PeerManager::new(config).expect("operation should succeed");

        assert_eq!(peer_manager.name(), "PeerManager");
        assert_eq!(peer_manager.shutdown_priority(), 40);
    }

    #[tokio::test]
    async fn test_p2p_node_shutdown() {
        let config = NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = P2pNode::new(config, command_receiver).expect("operation should succeed");

        assert_eq!(p2p_node.name(), "P2pNode");
        assert_eq!(p2p_node.shutdown_priority(), 35);

        // Test shutdown
        let result = p2p_node.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_p2p_node_can_shutdown() {
        let config = NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = P2pNode::new(config, command_receiver).expect("operation should succeed");

        // Should be able to shutdown when not starting
        let can_shutdown = p2p_node.can_shutdown().await;
        assert!(can_shutdown);
    }

    #[tokio::test]
    async fn test_sync_manager_shutdown() {
        let suffix = format!("shutdown-{}", uuid::Uuid::new_v4());
        let blockchain = std::sync::Arc::new(
            neo_ledger::Blockchain::new_with_storage_suffix(
                neo_config::NetworkType::TestNet,
                Some(&suffix),
            )
            .await
            .expect("Failed to create test blockchain"),
        );
        let config = NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = std::sync::Arc::new(
            P2pNode::new(config, command_receiver).expect("Failed to create P2P node for test"),
        );
        let sync_manager = SyncManager::new(blockchain, p2p_node);

        assert_eq!(sync_manager.name(), "SyncManager");
        assert_eq!(sync_manager.shutdown_priority(), 30);

        // Test shutdown
        let result = sync_manager.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sync_manager_can_shutdown() {
        let suffix = format!("shutdown-{}", uuid::Uuid::new_v4());
        let blockchain = std::sync::Arc::new(
            neo_ledger::Blockchain::new_with_storage_suffix(
                neo_config::NetworkType::TestNet,
                Some(&suffix),
            )
            .await
            .expect("Failed to create test blockchain"),
        );
        let config = NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = std::sync::Arc::new(
            P2pNode::new(config, command_receiver).expect("Failed to create P2P node for test"),
        );
        let sync_manager = SyncManager::new(blockchain, p2p_node);

        // Should be able to shutdown when idle
        let can_shutdown = sync_manager.can_shutdown().await;
        assert!(can_shutdown);
    }

    #[tokio::test]
    async fn test_shutdown_priority_order() {
        // Verify components have correct priorities
        let suffix = format!("shutdown-{}", uuid::Uuid::new_v4());
        let blockchain = std::sync::Arc::new(
            neo_ledger::Blockchain::new_with_storage_suffix(
                neo_config::NetworkType::TestNet,
                Some(&suffix),
            )
            .await
            .expect("Failed to create test blockchain"),
        );
        let config = NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = std::sync::Arc::new(
            P2pNode::new(config.clone(), command_receiver).expect("clone should succeed"),
        );
        let sync_manager = SyncManager::new(blockchain, p2p_node.clone());
        let peer_manager = PeerManager::new(config).expect("operation should succeed");

        assert!(sync_manager.shutdown_priority() < p2p_node.shutdown_priority());
        assert!(p2p_node.shutdown_priority() < peer_manager.shutdown_priority());
    }

    #[tokio::test]
    async fn test_network_server_shutdown_wrapper() {
        let suffix = format!("shutdown-{}", uuid::Uuid::new_v4());
        let blockchain = std::sync::Arc::new(
            neo_ledger::Blockchain::new_with_storage_suffix(
                neo_config::NetworkType::TestNet,
                Some(&suffix),
            )
            .await
            .expect("Failed to create test blockchain"),
        );
        let config = NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = std::sync::Arc::new(
            P2pNode::new(config.clone(), command_receiver).expect("clone should succeed"),
        );
        let sync_manager = std::sync::Arc::new(SyncManager::new(blockchain, p2p_node.clone()));

        let wrapper = NetworkServerShutdown::new(p2p_node, sync_manager);

        assert_eq!(wrapper.name(), "NetworkServer");
        assert_eq!(wrapper.shutdown_priority(), 25);

        let result = wrapper.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_rpc_server_shutdown() {
        let is_running = std::sync::Arc::new(tokio::sync::RwLock::new(true));
        let bind_address = "DEFAULT_RPC_PORT".to_string();
        let rpc_shutdown = RpcServerShutdown::new(is_running.clone(), bind_address);

        assert_eq!(rpc_shutdown.name(), "RpcServer");
        assert_eq!(rpc_shutdown.shutdown_priority(), 60);

        // Verify it's initially running
        assert!(*is_running.read().await);

        let result = rpc_shutdown.shutdown().await;
        assert!(result.is_ok());

        // Verify it's now stopped
        assert!(!*is_running.read().await);
    }

    #[tokio::test]
    async fn test_database_shutdown() {
        let db_shutdown = DatabaseShutdown::new("TestDB".to_string());

        assert_eq!(db_shutdown.name(), "TestDB");
        assert_eq!(db_shutdown.shutdown_priority(), 120);

        let result = db_shutdown.shutdown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transaction_pool_shutdown() {
        let pending_count = std::sync::Arc::new(tokio::sync::RwLock::new(5));
        let tx_pool_shutdown = TransactionPoolShutdown::new(pending_count.clone());

        assert_eq!(tx_pool_shutdown.name(), "TransactionPool");
        assert_eq!(tx_pool_shutdown.shutdown_priority(), 80);

        // Verify initial state
        assert_eq!(*pending_count.read().await, 5);

        // Test can_shutdown with pending transactions
        let can_shutdown = tx_pool_shutdown.can_shutdown().await;
        assert!(can_shutdown); // Should allow shutdown even with pending

        let result = tx_pool_shutdown.shutdown().await;
        assert!(result.is_ok());

        // Verify transactions were cleared
        assert_eq!(*pending_count.read().await, 0);
    }

    #[tokio::test]
    async fn test_shutdown_coordinator_integration() {
        let coordinator = std::sync::Arc::new(ShutdownCoordinator::new());

        // Create some test components
        let is_running = std::sync::Arc::new(tokio::sync::RwLock::new(true));
        let rpc_shutdown = std::sync::Arc::new(RpcServerShutdown::new(
            is_running.clone(),
            "test".to_string(),
        ));
        let db_shutdown = std::sync::Arc::new(DatabaseShutdown::new("TestDB".to_string()));

        // Register components
        coordinator.register_component(rpc_shutdown.clone()).await;
        coordinator.register_component(db_shutdown.clone()).await;

        // Initiate shutdown
        let shutdown_result = timeout(
            Duration::from_secs(5),
            coordinator.initiate_shutdown("Test shutdown".to_string()),
        )
        .await;

        assert!(shutdown_result.is_ok());
        assert!(shutdown_result.expect("operation should succeed").is_ok());

        // Verify RPC server was stopped
        assert!(!*is_running.read().await);
    }

    #[test]
    fn test_shutdown_error_conversion() {
        // Test that we can create ShutdownError properly
        let error = neo_core::ShutdownError::ComponentError("Test error".to_string());
        assert!(error.to_string().contains("Test error"));

        let timeout_error = neo_core::ShutdownError::Timeout;
        assert!(timeout_error.to_string().contains("timeout"));
    }
}

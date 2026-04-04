//! Local P2P Testing Framework
//!
//! This module provides a comprehensive local testing framework for Neo N3 P2P functionality.
//! It simulates multiple Neo N3 nodes and network behavior for testing purposes.

use crate::{
    NetworkConfig, NetworkError, NetworkMessage, NetworkResult as Result, P2pNode, ProtocolMessage,
    SyncManager,
};
use neo_core::{Transaction, UInt160, UInt256};
use neo_ledger::{Block, BlockHeader, Blockchain};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time::{interval, Duration, Instant};
use tracing::{debug, error, info, warn};

/// Test framework constants
const TEST_NODE_1: &str = "127.0.0.1:20001";
const TEST_NODE_2: &str = "127.0.0.1:20002";
const TEST_NODE_3: &str = "127.0.0.1:20003";

/// Test node that simulates a Neo N3 peer
#[derive(Debug)]
pub struct TestNode {
    /// Node address
    pub address: SocketAddr,
    /// Node height
    pub height: u32,
    /// Blocks this node has
    pub blocks: Arc<RwLock<HashMap<u32, Block>>>,
    /// Headers this node has
    pub headers: Arc<RwLock<HashMap<u32, BlockHeader>>>,
    /// Message receiver
    pub message_rx: mpsc::Receiver<NetworkMessage>,
    /// Message sender
    pub message_tx: mpsc::Sender<NetworkMessage>,
    /// Running state
    pub running: Arc<RwLock<bool>>,
}

impl TestNode {
    /// Creates a new test node
    pub fn new(address: SocketAddr, height: u32) -> Self {
        let (message_tx, message_rx) = mpsc::channel(1000);

        Self {
            address,
            height,
            blocks: Arc::new(RwLock::new(HashMap::new())),
            headers: Arc::new(RwLock::new(HashMap::new())),
            message_rx,
            message_tx,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Starts the test node
    pub async fn start(&mut self) -> Result<()> {
        *self.running.write().await = true;

        info!(
            "Started test node {} at height {}",
            self.address, self.height
        );

        self.generate_test_blocks().await?;

        Ok(())
    }

    /// Stops the test node
    pub async fn stop(&self) {
        *self.running.write().await = false;
        info!("Stopped test node {}", self.address);
    }

    /// Generates test blocks for the node
    async fn generate_test_blocks(&self) -> Result<()> {
        let mut blocks = self.blocks.write().await;
        let mut headers = self.headers.write().await;

        for i in 0..=self.height {
            let block = self.create_test_block(i).await?;
            let header = block.header.clone();

            blocks.insert(i, block);
            headers.insert(i, header);
        }

        info!(
            "Generated {} test blocks for node {}",
            self.height + 1,
            self.address
        );
        Ok(())
    }

    /// Creates a test block at the given height
    async fn create_test_block(&self, height: u32) -> Result<Block> {
        let previous_hash = if height == 0 {
            UInt256::zero()
        } else {
            UInt256::from_bytes(&[height as u8; 32])?
        };

        let header = BlockHeader {
            version: 0,
            previous_hash,
            merkle_root: UInt256::from_bytes(&[0x42; 32])?, // Test merkle root
            timestamp: 1640995200000 + (height as u64 * 15000), // 15 second intervals
            nonce: height as u64,
            index: height,
            primary_index: 0,
            next_consensus: UInt160::from_bytes(&[0x33; 20])?,
            witnesses: Vec::new(),
        };

        // Create a simple test transaction
        let test_tx = self.create_test_transaction(height).await?;
        let transactions = vec![test_tx];

        Ok(Block::new(header, transactions))
    }

    /// Creates a test transaction
    async fn create_test_transaction(&self, height: u32) -> Result<Transaction> {
        let mut tx = Transaction::new();
        tx.set_version(0);
        tx.set_nonce(height);
        tx.set_system_fee(1000000); // 0.01 GAS
        tx.set_network_fee(1000000); // 0.01 GAS
        tx.set_valid_until_block(height + 1000);
        tx.set_script(vec![0x40, 0x41, 0x42]); // Simple test script

        Ok(tx)
    }

    /// Handles incoming messages
    pub async fn handle_message(&self, message: NetworkMessage) -> Result<Option<NetworkMessage>> {
        match message.payload {
            ProtocolMessage::GetHeaders {
                index_start,
                count: _,
            } => {
                debug!("Test node {} received GetHeaders request", self.address);

                // Use index_start directly
                let start_height = index_start;

                let headers = self.headers.read().await;
                let mut response_headers = Vec::new();

                // Return up to 2000 headers starting from start_height
                for i in start_height..=self.height.min(start_height + 1999) {
                    if let Some(header) = headers.get(&i) {
                        response_headers.push(header.clone());
                    }
                }

                info!(
                    "Test node {} returning {} headers starting from height {}",
                    self.address,
                    response_headers.len(),
                    start_height
                );

                let response = ProtocolMessage::Headers {
                    headers: response_headers,
                };

                Ok(Some(NetworkMessage::new(response)))
            }

            ProtocolMessage::GetBlockByIndex { index_start, count } => {
                debug!(
                    "Test node {} received GetBlockByIndex for height {} (count: {})",
                    self.address, index_start, count
                );

                let blocks = self.blocks.read().await;

                if let Some(block) = blocks.get(&index_start) {
                    let response = ProtocolMessage::Block {
                        block: block.clone(),
                    };

                    info!(
                        "Test node {} returning block at height {}",
                        self.address, index_start
                    );
                    Ok(Some(NetworkMessage::new(response)))
                } else {
                    warn!(
                        "Test node {} does not have block at height {}",
                        self.address, index_start
                    );
                    Ok(None)
                }
            }

            ProtocolMessage::Version {
                version,
                services,
                timestamp: _,
                port: _,
                nonce,
                user_agent: _,
                start_height,
                relay,
            } => {
                info!(
                    "Test node {} received version message from peer (height: {})",
                    self.address, start_height
                );

                // Respond with our version
                let response = ProtocolMessage::Version {
                    version,
                    services,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                    port: self.address.port(),
                    nonce: nonce + 1, // Different nonce
                    user_agent: "TestNode/1.0".to_string(),
                    start_height: self.height,
                    relay,
                };

                Ok(Some(NetworkMessage::new(response)))
            }

            ProtocolMessage::Verack => {
                info!("Test node {} received verack", self.address);
                Ok(Some(NetworkMessage::new(ProtocolMessage::Verack)))
            }

            _ => {
                debug!(
                    "Test node {} received unhandled message: {:?}",
                    self.address, message.payload
                );
                Ok(None)
            }
        }
    }
}

/// Local P2P testing framework
pub struct LocalTestFramework {
    /// Test nodes
    nodes: HashMap<SocketAddr, TestNode>,
    /// Message routing
    message_router: Arc<RwLock<HashMap<SocketAddr, mpsc::Sender<NetworkMessage>>>>,
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl LocalTestFramework {
    /// Creates a new local test framework
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            message_router: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Adds a test node to the framework
    pub async fn add_node(&mut self, address: SocketAddr, height: u32) -> Result<()> {
        let mut node = TestNode::new(address, height);
        node.start().await?;

        let mut router = self.message_router.write().await;
        router.insert(address, node.message_tx.clone());

        self.nodes.insert(address, node);

        info!("Added test node {} with height {}", address, height);
        Ok(())
    }

    /// Starts the testing framework
    pub async fn start(&self) -> Result<()> {
        *self.running.write().await = true;

        info!(
            "Started local P2P testing framework with {} nodes",
            self.nodes.len()
        );

        for (address, _node) in &self.nodes {
            self.spawn_node_handler(*address).await;
        }

        Ok(())
    }

    /// Stops the testing framework
    pub async fn stop(&self) {
        *self.running.write().await = false;

        for node in self.nodes.values() {
            node.stop().await;
        }

        info!("Stopped local P2P testing framework");
    }

    /// Spawns a message handler for a node
    async fn spawn_node_handler(&self, _address: SocketAddr) {
        let running = self.running.clone();
        let _router = self.message_router.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(100));

            while *running.read().await {
                interval.tick().await;
            }
        });
    }

    /// Simulates sending a message to a node
    pub async fn send_message_to_node(
        &self,
        target: SocketAddr,
        message: NetworkMessage,
    ) -> Result<()> {
        let router = self.message_router.read().await;

        if let Some(sender) = router.get(&target) {
            sender
                .send(message)
                .await
                .map_err(|_| NetworkError::MessageSendFailed {
                    peer: target,
                    message_type: "NetworkMessage".to_string(),
                    reason: "Failed to send message to test node".to_string(),
                })?;
            Ok(())
        } else {
            Err(NetworkError::PeerNotConnected { address: target })
        }
    }

    /// Gets the highest block height among all test nodes
    pub fn get_max_height(&self) -> u32 {
        self.nodes
            .values()
            .map(|node| node.height)
            .max()
            .unwrap_or(0)
    }

    /// Creates a test blockchain sync scenario
    pub async fn create_sync_test_scenario(&mut self) -> Result<TestSyncScenario> {
        // Create nodes with different heights to simulate sync scenario
        let addresses = vec![
            TEST_NODE_1.parse().map_err(|e| {
                NetworkError::configuration("address", &format!("Parse error: {}", e))
            })?,
            TEST_NODE_2.parse().map_err(|e| {
                NetworkError::configuration("address", &format!("Parse error: {}", e))
            })?,
            TEST_NODE_3.parse().map_err(|e| {
                NetworkError::configuration("address", &format!("Parse error: {}", e))
            })?,
        ];

        self.add_node(addresses[0], 100).await?;

        self.add_node(addresses[1], 150).await?;

        self.add_node(addresses[2], 200).await?;

        Ok(TestSyncScenario {
            behind_node: addresses[0],
            current_node: addresses[1],
            ahead_node: addresses[2],
            target_height: 200,
        })
    }
}

/// Test scenario for blockchain synchronization
#[derive(Debug, Clone)]
pub struct TestSyncScenario {
    /// Node that is behind
    pub behind_node: SocketAddr,
    /// Node that is current
    pub current_node: SocketAddr,
    /// Node that is ahead
    pub ahead_node: SocketAddr,
    /// Target sync height
    pub target_height: u32,
}

impl TestSyncScenario {
    /// Runs the sync test scenario
    pub async fn run_sync_test(&self, sync_manager: &SyncManager) -> Result<SyncTestResult> {
        let start_time = Instant::now();

        info!("Starting sync test scenario:");
        info!("  Behind node: {} (height: ~100)", self.behind_node);
        info!("  Current node: {} (height: ~150)", self.current_node);
        info!("  Ahead node: {} (height: ~200)", self.ahead_node);
        info!("  Target height: {}", self.target_height);

        // Update best known height to trigger sync
        sync_manager
            .update_best_height(self.target_height, self.ahead_node)
            .await;

        // Monitor sync progress
        let mut stats_interval = interval(Duration::from_secs(2));
        let mut max_wait = 60; // 60 seconds max

        loop {
            stats_interval.tick().await;
            max_wait -= 2;

            let stats = sync_manager.stats().await;
            info!(
                "Sync progress: {:.1}% ({}/{})",
                stats.progress_percentage, stats.current_height, stats.best_known_height
            );

            if stats.current_height >= self.target_height {
                let duration = start_time.elapsed();
                info!("Sync test completed successfully in {:?}", duration);

                return Ok(SyncTestResult {
                    success: true,
                    final_height: stats.current_height,
                    duration,
                    blocks_synced: stats.current_height,
                    average_speed: stats.sync_speed,
                });
            }

            if max_wait <= 0 {
                warn!("Sync test timed out after 60 seconds");
                let duration = start_time.elapsed();

                return Ok(SyncTestResult {
                    success: false,
                    final_height: sync_manager.stats().await.current_height,
                    duration,
                    blocks_synced: sync_manager.stats().await.current_height,
                    average_speed: 0.0,
                });
            }
        }
    }
}

/// Result of a sync test
#[derive(Debug, Clone)]
pub struct SyncTestResult {
    /// Whether the sync was successful
    pub success: bool,
    /// Final height reached
    pub final_height: u32,
    /// Duration of the test
    pub duration: Duration,
    /// Number of blocks synced
    pub blocks_synced: u32,
    /// Average sync speed (blocks per second)
    pub average_speed: f64,
}

impl SyncTestResult {
    /// Prints a detailed test report
    pub fn print_report(&self) {
        info!("=== Sync Test Results ===");
        info!("Success: {}", self.success);
        info!("Final height: {}", self.final_height);
        info!("Duration: {:?}", self.duration);
        info!("Blocks synced: {}", self.blocks_synced);
        info!("Average speed: {:.2} blocks/sec", self.average_speed);

        if self.success {
            info!("✅ Sync test PASSED");
        } else {
            error!("❌ Sync test FAILED");
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_framework_creation() {
        let mut framework = LocalTestFramework::new();

        let addr: SocketAddr = TEST_NODE_1.parse().unwrap();
        framework.add_node(addr, 100).await.unwrap();

        assert_eq!(framework.get_max_height(), 100);
    }

    #[tokio::test]
    async fn test_node_message_handling() {
        let addr: SocketAddr = TEST_NODE_1.parse().expect("value should parse");
        let node = TestNode::new(addr, 50);

        let version_msg = NetworkMessage::new_with_magic(
            ProtocolMessage::Version {
                version: 0,
                services: 1,
                timestamp: 1640995200,
                port: 20333,
                nonce: 12345,
                user_agent: "Test".to_string(),
                start_height: 0,
                relay: true,
            },
            0x3554334e,
        );

        let response = node
            .handle_message(version_msg)
            .await
            .expect("operation should succeed");
        assert!(response.is_some());
    }
}

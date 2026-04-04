#![cfg(feature = "compat_tests")]
//! Integration tests for block synchronization
//!
//! This test ensures the block sync functionality works correctly
//! including requesting blocks, handling responses, and storing them.

use neo_config::NetworkType;
use neo_ledger::{Block, BlockHeader, Blockchain};
use neo_network::{
    server::{NetworkServer, NetworkServerConfig},
    sync::{SyncEvent, SyncManager, SyncState},
    NetworkConfig, NetworkMessage, P2PConfig, P2pNode, ProtocolMessage,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Creates a test blockchain instance
async fn create_test_blockchain() -> Arc<Blockchain> {
    let suffix = format!("bsi-{}", uuid::Uuid::new_v4());
    Arc::new(
        Blockchain::new_with_storage_suffix(NetworkType::TestNet, Some(&suffix))
            .await
            .expect("Failed to create blockchain"),
    )
}

/// Creates a test network configuration
fn create_test_network_config(port: u16) -> NetworkServerConfig {
    NetworkServerConfig {
        node_id: neo_core::UInt160::zero(),
        magic: 0x334F454E, // TestNet magic
        p2p_config: P2PConfig {
            listen_address: format!("127.0.0.1:{}", port)
                .parse()
                .expect("Valid address"),
            max_peers: 10,
            ..Default::default()
        },
        seed_nodes: vec![],
        enable_auto_sync: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn test_block_sync_basic_flow() {
    // Setup test blockchain
    let blockchain = create_test_blockchain().await;

    // Create network server
    let config = create_test_network_config(30400);
    let server =
        NetworkServer::new(config, blockchain.clone()).expect("Failed to create network server");

    // Start the server
    server.start().await.expect("Failed to start server");

    // Get sync manager
    let sync_manager = server.sync_manager();

    // Subscribe to sync events
    let mut event_rx = sync_manager.event_receiver();

    // Simulate peer announcing new height
    sync_manager
        .update_best_height(100, "127.0.0.1:30401".parse().unwrap())
        .await;

    // Wait for sync to start
    let timeout_duration = Duration::from_secs(5);
    let result = timeout(timeout_duration, async {
        while let Ok(event) = event_rx.recv().await {
            match event {
                SyncEvent::SyncStarted { target_height } => {
                    assert_eq!(target_height, 100);
                    return true;
                }
                _ => continue,
            }
        }
        false
    })
    .await;

    assert!(
        result.is_ok() && result.unwrap(),
        "Sync should have started"
    );

    // Verify sync state changed
    let state = sync_manager.state().await;
    assert!(
        matches!(state, SyncState::SyncingHeaders | SyncState::SyncingBlocks),
        "Sync state should be active"
    );

    // Stop the server
    server.stop().await;
}

#[tokio::test]
async fn test_block_sync_with_mock_peer() {
    use neo_network::p2p::MessageHandler;

    // Setup test blockchain
    let blockchain = create_test_blockchain().await;

    // Create P2P node and sync manager
    let network_config = NetworkConfig {
        magic: 0x334F454E,
        listen_address: "127.0.0.1:30402".parse().unwrap(),
        ..Default::default()
    };

    let (_cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(100);
    let p2p_node =
        Arc::new(P2pNode::new(network_config, cmd_rx).expect("Failed to create P2P node"));

    let sync_manager = Arc::new(SyncManager::new(blockchain.clone(), p2p_node.clone()));

    // Set sync manager in P2P node
    p2p_node.set_sync_manager(sync_manager.clone()).await;

    // Start sync manager
    sync_manager
        .start()
        .await
        .expect("Failed to start sync manager");

    // Simulate receiving a block
    let test_block = Block::new(
        BlockHeader {
            version: 0,
            previous_hash: neo_core::UInt256::zero(),
            merkle_root: neo_core::UInt256::zero(),
            timestamp: 1234567890,
            index: 1,
            nonce: 0,
            primary_index: 0,
            next_consensus: neo_core::UInt160::zero(),
            witnesses: vec![neo_core::Witness::default()],
        },
        vec![],
    );

    let block_message = NetworkMessage::new(ProtocolMessage::Block {
        block: test_block.clone(),
    });

    // Handle the block message
    let peer_addr: SocketAddr = "127.0.0.1:30403".parse().unwrap();
    sync_manager
        .handle_message(peer_addr, &block_message)
        .await
        .expect("Failed to handle block message");

    // Give time for processing
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Stop sync manager
    sync_manager.stop().await;
}

#[tokio::test]
async fn test_block_sync_error_handling() {
    // Setup test blockchain
    let blockchain = create_test_blockchain().await;

    // Create network server with no peers
    let config = create_test_network_config(30404);
    let server =
        NetworkServer::new(config, blockchain.clone()).expect("Failed to create network server");

    // Start the server
    server.start().await.expect("Failed to start server");

    // Get sync manager
    let sync_manager = server.sync_manager();

    // Try to sync with no peers available - should handle gracefully
    sync_manager
        .update_best_height(100, "127.0.0.1:30405".parse().unwrap())
        .await;

    // Give time for sync attempt
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Should remain in idle state since no peers
    let state = sync_manager.state().await;
    assert_eq!(state, SyncState::Idle, "Should remain idle with no peers");

    // Stop the server
    server.stop().await;
}

#[tokio::test]
async fn test_block_sync_stats() {
    // Setup test blockchain
    let blockchain = create_test_blockchain().await;

    // Create network server
    let config = create_test_network_config(30406);
    let server =
        NetworkServer::new(config, blockchain.clone()).expect("Failed to create network server");

    // Start the server
    server.start().await.expect("Failed to start server");

    // Get sync manager
    let sync_manager = server.sync_manager();

    // Get initial stats
    let stats = sync_manager.stats().await;
    assert_eq!(stats.state, SyncState::Idle);
    assert_eq!(stats.current_height, 0);
    assert_eq!(stats.best_known_height, 0);
    assert_eq!(stats.progress_percentage, 0.0);

    // Update best height
    sync_manager
        .update_best_height(1000, "127.0.0.1:30407".parse().unwrap())
        .await;

    // Get updated stats
    let stats = sync_manager.stats().await;
    assert_eq!(stats.best_known_height, 1000);

    // Stop the server
    server.stop().await;
}

#[test]
fn test_sync_state_display() {
    assert_eq!(format!("{}", SyncState::Idle), "Idle");
    assert_eq!(format!("{}", SyncState::SyncingHeaders), "Syncing Headers");
    assert_eq!(format!("{}", SyncState::SyncingBlocks), "Syncing Blocks");
    assert_eq!(format!("{}", SyncState::Synchronized), "Synchronized");
    assert_eq!(format!("{}", SyncState::Failed), "Failed");
}

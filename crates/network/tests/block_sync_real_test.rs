#![cfg(feature = "compat_tests")]
//! Real integration tests for block sync functionality
//!
//! These tests verify block sync works with actual network components

use neo_config::NetworkType;
use neo_core::{UInt160, UInt256, Witness};
use neo_ledger::{Block, BlockHeader, Blockchain};
use neo_network::{
    messages::{InventoryItem, InventoryType},
    sync::{SyncEvent, SyncManager, SyncState},
    NetworkConfig, NetworkMessage, P2pNode, ProtocolMessage,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, timeout};

/// Creates a simple test block
fn create_test_block(index: u32, previous_hash: UInt256) -> Block {
    let header = BlockHeader {
        version: 0,
        previous_hash,
        merkle_root: UInt256::from_bytes(&[index as u8; 32]).unwrap(),
        timestamp: 1609459200 + (index * 15) as u64,
        index,
        nonce: index as u64,
        primary_index: 0,
        next_consensus: UInt160::zero(),
        witnesses: vec![Witness::default()],
    };
    Block::new(header, vec![])
}

#[tokio::test]
async fn test_sync_manager_block_handling() {
    // Setup blockchain
    let suffix = format!("bsr-{}", uuid::Uuid::new_v4());
    let blockchain = Arc::new(
        Blockchain::new_with_storage_suffix(NetworkType::TestNet, Some(&suffix))
            .await
            .expect("Failed to create blockchain"),
    );

    // Create P2P node
    let network_config = NetworkConfig {
        magic: 0x334F454E,
        listen_address: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let (_tx, rx) = tokio::sync::mpsc::channel(100);
    let p2p_node = Arc::new(P2pNode::new(network_config, rx).expect("Failed to create P2P node"));

    // Create sync manager
    let sync_manager = Arc::new(SyncManager::new(blockchain.clone(), p2p_node.clone()));

    // Set sync manager in P2P node
    p2p_node.set_sync_manager(sync_manager.clone()).await;

    // Start sync manager
    sync_manager
        .start()
        .await
        .expect("Failed to start sync manager");

    // Test 1: Handle a single block
    let test_block = create_test_block(1, UInt256::zero());
    let _block_hash = test_block.hash();

    sync_manager
        .handle_block(test_block.clone(), "127.0.0.1:20333".parse().unwrap())
        .await
        .expect("Failed to handle block");

    // Give time for processing
    sleep(Duration::from_millis(100)).await;

    // Verify blockchain updated
    let height = blockchain.get_height().await;
    assert_eq!(height, 1, "Blockchain should have processed block 1");

    // Test 2: Update best height triggers sync
    let mut event_rx = sync_manager.event_receiver();

    sync_manager
        .update_best_height(10, "127.0.0.1:20333".parse().unwrap())
        .await;

    // Should receive sync start event
    let event_result = timeout(Duration::from_secs(2), event_rx.recv()).await;
    match event_result {
        Ok(Ok(SyncEvent::NewBestHeight { height, .. })) => {
            assert_eq!(height, 10);
        }
        _ => panic!("Expected NewBestHeight event"),
    }

    // Stop sync manager
    sync_manager.stop().await;
}

#[tokio::test]
async fn test_sync_manager_inventory_handling() {
    // Setup
    let suffix = format!("bsr-{}", uuid::Uuid::new_v4());
    let blockchain = Arc::new(
        Blockchain::new_with_storage_suffix(NetworkType::TestNet, Some(&suffix))
            .await
            .expect("Failed to create blockchain"),
    );

    let network_config = NetworkConfig {
        magic: 0x334F454E,
        listen_address: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let (_tx, rx) = tokio::sync::mpsc::channel(100);
    let p2p_node = Arc::new(P2pNode::new(network_config, rx).unwrap());
    let sync_manager = Arc::new(SyncManager::new(blockchain.clone(), p2p_node.clone()));

    p2p_node.set_sync_manager(sync_manager.clone()).await;
    sync_manager.start().await.unwrap();

    // Create inventory message with block announcements
    let inv_items = vec![
        InventoryItem {
            item_type: InventoryType::Block,
            hash: UInt256::from_bytes(&[1; 32]).unwrap(),
        },
        InventoryItem {
            item_type: InventoryType::Block,
            hash: UInt256::from_bytes(&[2; 32]).unwrap(),
        },
    ];

    let inv_message = NetworkMessage::new(ProtocolMessage::Inv {
        inventory: inv_items,
    });

    // Handle inventory
    use neo_network::p2p::MessageHandler;
    sync_manager
        .handle_message("127.0.0.1:20333".parse().unwrap(), &inv_message)
        .await
        .expect("Failed to handle inventory");

    // The sync manager should request these blocks
    // In a real scenario, it would send GetData messages

    sync_manager.stop().await;
}

#[tokio::test]
async fn test_sync_state_progression() {
    let suffix = format!("bsr-{}", uuid::Uuid::new_v4());
    let blockchain = Arc::new(
        Blockchain::new_with_storage_suffix(NetworkType::TestNet, Some(&suffix))
            .await
            .expect("Failed to create blockchain"),
    );

    let (_tx, rx) = tokio::sync::mpsc::channel(100);
    let network_config = NetworkConfig {
        magic: 0x334F454E,
        listen_address: "127.0.0.1:0".parse().unwrap(),
        ..Default::default()
    };

    let p2p_node = Arc::new(P2pNode::new(network_config, rx).unwrap());
    let sync_manager = Arc::new(SyncManager::new(blockchain, p2p_node.clone()));

    p2p_node.set_sync_manager(sync_manager.clone()).await;

    // Initial state should be Idle
    assert_eq!(sync_manager.state().await, SyncState::Idle);

    // Start sync manager
    sync_manager.start().await.unwrap();

    // Update best height to trigger sync
    sync_manager
        .update_best_height(100, "127.0.0.1:20333".parse().unwrap())
        .await;

    // Give it a moment to transition
    sleep(Duration::from_millis(100)).await;

    // State should change from Idle
    let state = sync_manager.state().await;
    assert!(
        state == SyncState::SyncingHeaders || state == SyncState::SyncingBlocks,
        "State should be syncing, got: {:?}",
        state
    );

    sync_manager.stop().await;
}

#[tokio::test]
async fn test_sync_statistics() {
    let suffix = format!("bsr-{}", uuid::Uuid::new_v4());
    let blockchain = Arc::new(
        Blockchain::new_with_storage_suffix(NetworkType::TestNet, Some(&suffix))
            .await
            .expect("Failed to create blockchain"),
    );

    let (_tx, rx) = tokio::sync::mpsc::channel(100);
    let network_config = NetworkConfig::default();
    let p2p_node = Arc::new(P2pNode::new(network_config, rx).unwrap());
    let sync_manager = Arc::new(SyncManager::new(blockchain, p2p_node.clone()));

    p2p_node.set_sync_manager(sync_manager.clone()).await;
    sync_manager.start().await.unwrap();

    // Initial stats
    let stats = sync_manager.stats().await;
    assert_eq!(stats.state, SyncState::Idle);
    assert_eq!(stats.current_height, 0);
    assert_eq!(stats.best_known_height, 0);

    // Update best height
    sync_manager
        .update_best_height(1000, "127.0.0.1:20333".parse().unwrap())
        .await;

    // Check updated stats
    let stats = sync_manager.stats().await;
    assert_eq!(stats.best_known_height, 1000);
    assert!(stats.progress_percentage >= 0.0);

    sync_manager.stop().await;
}

#[tokio::test]
async fn test_headers_message_handling() {
    let blockchain = Arc::new(
        Blockchain::new(NetworkType::TestNet)
            .await
            .expect("Failed to create blockchain"),
    );

    let (_tx, rx) = tokio::sync::mpsc::channel(100);
    let network_config = NetworkConfig::default();
    let p2p_node = Arc::new(P2pNode::new(network_config, rx).unwrap());
    let sync_manager = Arc::new(SyncManager::new(blockchain, p2p_node.clone()));

    p2p_node.set_sync_manager(sync_manager.clone()).await;
    sync_manager.start().await.unwrap();

    // Create headers message
    let headers = vec![
        create_test_block(1, UInt256::zero()).header,
        create_test_block(2, UInt256::from_bytes(&[1; 32]).unwrap()).header,
    ];

    let headers_message = NetworkMessage::new(ProtocolMessage::Headers { headers });

    // Handle headers
    use neo_network::p2p::MessageHandler;
    let result = sync_manager
        .handle_message("127.0.0.1:20333".parse().unwrap(), &headers_message)
        .await;

    assert!(result.is_ok(), "Should handle headers successfully");

    sync_manager.stop().await;
}

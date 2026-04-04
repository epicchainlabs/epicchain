//! Transaction Relay Integration Tests

use neo_core::{Transaction, UInt256};
use neo_ledger::{MemoryPool, MempoolConfig};
use neo_network::{
    InventoryItem, InventoryType, RelayCache, TransactionRelay, TransactionRelayConfig,
    TransactionRelayEvent,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_transaction_relay_creation() {
    let config = TransactionRelayConfig::default();
    let mempool_config = MempoolConfig::default();
    let mempool = Arc::new(RwLock::new(MemoryPool::new(mempool_config)));

    let relay = TransactionRelay::new(config, mempool);

    // Test basic functionality
    assert_eq!(relay.get_connected_peer_count().await, 0);
    assert_eq!(relay.get_mempool_transaction_count().await, 0);
}

#[tokio::test]
async fn test_relay_cache_basic_operations() {
    let mut cache = RelayCache::new(10, 300); // 10 capacity, 5 minute TTL

    // Test insertion and contains
    let hash = UInt256::zero();
    assert!(!cache.contains(&hash));

    cache.insert(hash);
    assert!(cache.contains(&hash));
    assert_eq!(cache.len(), 1);

    // Test capacity limit
    for i in 1..12 {
        let mut bytes = [0u8; 32];
        bytes[0] = i;
        let test_hash = UInt256::from_bytes(&bytes).unwrap();
        cache.insert(test_hash);
    }

    // Should have evicted the oldest entry due to capacity
    assert_eq!(cache.len(), 10);
    assert!(!cache.contains(&hash)); // Original hash should be evicted
}

#[tokio::test]
async fn test_inventory_handling() {
    let config = TransactionRelayConfig::default();
    let mempool_config = MempoolConfig::default();
    let mempool = Arc::new(RwLock::new(MemoryPool::new(mempool_config)));

    let relay = TransactionRelay::new(config, mempool);

    // Create test inventory
    let inventory = vec![InventoryItem {
        item_type: InventoryType::Transaction,
        hash: UInt256::zero(),
    }];

    let peer_addr = "127.0.0.1:8080".parse().unwrap();
    let result = relay.handle_inventory(inventory, peer_addr).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_event_subscription() {
    let config = TransactionRelayConfig::default();
    let mempool_config = MempoolConfig::default();
    let mempool = Arc::new(RwLock::new(MemoryPool::new(mempool_config)));

    let relay = TransactionRelay::new(config, mempool);

    // Test event subscription
    let mut event_receiver = relay.subscribe_to_events();

    // Event receiver should be available
    assert!(event_receiver.is_empty());
}

#[tokio::test]
async fn test_relay_statistics() {
    let config = TransactionRelayConfig::default();
    let mempool_config = MempoolConfig::default();
    let mempool = Arc::new(RwLock::new(MemoryPool::new(mempool_config)));

    let relay = TransactionRelay::new(config, mempool);

    // Test statistics
    let stats = relay.get_statistics().await;
    assert_eq!(stats.transactions_received, 0);
    assert_eq!(stats.transactions_validated, 0);
    assert_eq!(stats.transactions_added_to_mempool, 0);
    assert_eq!(stats.transactions_relayed, 0);
    assert_eq!(stats.transactions_rejected, 0);
}

#[tokio::test]
async fn test_mempool_request_handling() {
    let config = TransactionRelayConfig::default();
    let mempool_config = MempoolConfig::default();
    let mempool = Arc::new(RwLock::new(MemoryPool::new(mempool_config)));

    let relay = TransactionRelay::new(config, mempool);

    // Test mempool request handling
    let peer_addr = "127.0.0.1:8080".parse().unwrap();
    let result = relay.handle_mempool_request(peer_addr).await;

    // Should succeed even with empty mempool
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cleanup_relay_cache() {
    let config = TransactionRelayConfig::default();
    let mempool_config = MempoolConfig::default();
    let mempool = Arc::new(RwLock::new(MemoryPool::new(mempool_config)));

    let relay = TransactionRelay::new(config, mempool);

    relay.cleanup_relay_cache().await;
}

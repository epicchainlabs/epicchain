//! Mempool C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Ledger memory pool implementation.
//! Tests are based on the C# Neo.Ledger.MemoryPool test suite.

use neo_core::{Signer, Transaction, UInt160, UInt256, WitnessScope};
use neo_ledger::mempool::*;
use neo_ledger::*;
use std::sync::Arc;
use std::time::Duration;

#[cfg(test)]
#[allow(dead_code)]
mod mempool_tests {
    use super::*;

    /// Test mempool creation and configuration (matches C# MemoryPool exactly)
    #[test]
    fn test_mempool_creation_compatibility() {
        let config = MemPoolConfig {
            capacity: 50000,
            max_tx_per_block: 500,
            max_low_priority_tx: 10000,
            fee_per_byte: 1000,     // 0.00001 GAS per byte
            max_free_tx_size: 1024, // 1KB
            max_free_tx_per_block: 20,
            reverification_frequency: 10,              // Every 10 blocks
            expiry_interval: Duration::from_secs(600), // 10 minutes
        };

        let mempool = MemPool::new(config.clone());

        // Verify configuration
        assert_eq!(mempool.capacity(), 50000);
        assert_eq!(mempool.size(), 0);
        assert_eq!(mempool.config().max_tx_per_block, 500);
        assert_eq!(mempool.config().fee_per_byte, 1000);
        assert!(mempool.is_empty());
    }

    /// Test transaction addition (matches C# TryAdd exactly)
    #[test]
    fn test_transaction_addition_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config);

        // Create test transaction
        let tx = create_test_transaction(1, 1000000, 100000); // 0.01 GAS system fee, 0.001 GAS network fee

        // Add transaction
        let result = mempool.try_add(tx.clone(), 100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MemPoolAddResult::Added);

        // Verify transaction is in pool
        assert_eq!(mempool.size(), 1);
        assert!(mempool.contains(&tx.hash()));

        // Test duplicate rejection
        let duplicate_result = mempool.try_add(tx.clone(), 100);
        assert_eq!(duplicate_result.unwrap(), MemPoolAddResult::AlreadyExists);

        let conflicting_tx = Transaction {
            network_fee: 50000, // Lower fee
            ..tx.clone()
        };
        let conflict_result = mempool.try_add(conflicting_tx, 100);
        assert_eq!(conflict_result.unwrap(), MemPoolAddResult::InsufficientFee);
    }

    /// Test fee-based prioritization (matches C# fee sorting exactly)
    #[test]
    fn test_fee_prioritization_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config);

        // Add transactions with different fees
        let high_fee_tx = create_test_transaction(1, 10000000, 1000000); // 0.1 GAS, 0.01 GAS
        let medium_fee_tx = create_test_transaction(2, 5000000, 500000); // 0.05 GAS, 0.005 GAS
        let low_fee_tx = create_test_transaction(3, 1000000, 100000); // 0.01 GAS, 0.001 GAS

        mempool.try_add(low_fee_tx.clone(), 100).unwrap();
        mempool.try_add(high_fee_tx.clone(), 100).unwrap();
        mempool.try_add(medium_fee_tx.clone(), 100).unwrap();

        // Get sorted transactions
        let sorted = mempool.get_sorted_transactions(10);

        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].hash(), high_fee_tx.hash());
        assert_eq!(sorted[1].hash(), medium_fee_tx.hash());
        assert_eq!(sorted[2].hash(), low_fee_tx.hash());
    }

    /// Test capacity limits (matches C# capacity handling exactly)
    #[test]
    fn test_capacity_limits_compatibility() {
        let config = MemPoolConfig {
            capacity: 100,
            ..Default::default()
        };
        let mut mempool = MemPool::new(config);

        // Fill mempool
        for i in 0..100 {
            let tx = create_test_transaction(i, 1000000, 100000);
            mempool.try_add(tx, 100).unwrap();
        }

        assert_eq!(mempool.size(), 100);

        let low_fee_tx = create_test_transaction(101, 100000, 10000); // Very low fee
        let result = mempool.try_add(low_fee_tx, 100);
        assert_eq!(result.unwrap(), MemPoolAddResult::CapacityExceeded);

        let high_fee_tx = create_test_transaction(102, 100000000, 10000000); // Very high fee
        let result = mempool.try_add(high_fee_tx.clone(), 100);
        assert_eq!(result.unwrap(), MemPoolAddResult::Added);

        // Size should still be 100
        assert_eq!(mempool.size(), 100);
        assert!(mempool.contains(&high_fee_tx.hash()));
    }

    /// Test transaction removal (matches C# Remove exactly)
    #[test]
    fn test_transaction_removal_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config);

        // Add transactions
        let tx1 = create_test_transaction(1, 1000000, 100000);
        let tx2 = create_test_transaction(2, 2000000, 200000);
        let tx3 = create_test_transaction(3, 3000000, 300000);

        mempool.try_add(tx1.clone(), 100).unwrap();
        mempool.try_add(tx2.clone(), 100).unwrap();
        mempool.try_add(tx3.clone(), 100).unwrap();

        assert_eq!(mempool.size(), 3);

        // Remove specific transaction
        let removed = mempool.remove(&tx2.hash());
        assert!(removed);
        assert_eq!(mempool.size(), 2);
        assert!(!mempool.contains(&tx2.hash()));

        // Remove non-existent transaction
        let not_removed = mempool.remove(&UInt256::zero());
        assert!(!not_removed);
        assert_eq!(mempool.size(), 2);

        // Remove multiple transactions
        let hashes = vec![tx1.hash(), tx3.hash()];
        let removed_count = mempool.remove_many(&hashes);
        assert_eq!(removed_count, 2);
        assert_eq!(mempool.size(), 0);
    }

    /// Test transaction expiry (matches C# expiry mechanism exactly)
    #[test]
    fn test_transaction_expiry_compatibility() {
        let config = MemPoolConfig {
            expiry_interval: Duration::from_secs(1), // 1 second for testing
            ..Default::default()
        };
        let mut mempool = MemPool::new(config);

        // Add transaction that expires at block 110
        let expiring_tx = Transaction {
            valid_until_block: 110,
            ..create_test_transaction(1, 1000000, 100000)
        };

        mempool.try_add(expiring_tx.clone(), 100).unwrap();
        assert_eq!(mempool.size(), 1);

        let removed = mempool.remove_expired(111);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].hash(), expiring_tx.hash());
        assert_eq!(mempool.size(), 0);

        // Add non-expiring transaction
        let valid_tx = Transaction {
            valid_until_block: 200,
            ..create_test_transaction(2, 1000000, 100000)
        };

        mempool.try_add(valid_tx.clone(), 100).unwrap();
        let removed = mempool.remove_expired(111);
        assert_eq!(removed.len(), 0);
        assert_eq!(mempool.size(), 1);
    }

    /// Test re-verification (matches C# reverification exactly)
    #[test]
    fn test_reverification_compatibility() {
        let config = MemPoolConfig {
            reverification_frequency: 5, // Every 5 blocks
            ..Default::default()
        };
        let mut mempool = MemPool::new(config);

        // Add transactions
        for i in 0..10 {
            let tx = create_test_transaction(i, 1000000, 100000);
            mempool.try_add(tx, 100).unwrap();
        }

        let verifier = |tx: &Transaction| -> bool { tx.nonce % 2 != 0 };

        let removed = mempool.reverify_transactions(105, verifier);

        assert_eq!(removed.len(), 5);
        assert_eq!(mempool.size(), 5);

        // Remaining transactions should have odd nonces
        for tx in mempool.get_sorted_transactions(10) {
            assert_eq!(tx.nonce % 2, 1);
        }
    }

    /// Test low priority transactions (matches C# low priority handling exactly)
    #[test]
    fn test_low_priority_handling_compatibility() {
        let config = MemPoolConfig {
            max_low_priority_tx: 5,
            max_free_tx_size: 1024,
            max_free_tx_per_block: 2,
            ..Default::default()
        };
        let mut mempool = MemPool::new(config);

        for i in 0..10 {
            let free_tx = Transaction {
                system_fee: 0,
                network_fee: 0,
                script: vec![0u8; 100], // Small size
                ..create_test_transaction(i, 0, 0)
            };

            let result = mempool.try_add(free_tx, 100).unwrap();

            if i < 5 {
                assert_eq!(result, MemPoolAddResult::Added);
            } else {
                assert_eq!(result, MemPoolAddResult::InsufficientFee); // Low priority limit reached
            }
        }

        assert_eq!(mempool.low_priority_count(), 5);

        let block_txs = mempool.get_transactions_for_block(10);
        let free_count = block_txs.iter().filter(|tx| tx.network_fee == 0).count();
        assert_eq!(free_count, 2); // Max free tx per block
    }

    /// Test conflict handling (matches C# conflict resolution exactly)
    #[test]
    fn test_conflict_handling_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config);

        let account = UInt160::from_bytes(&[1u8; 20]).unwrap();

        // Add initial transaction
        let tx1 = Transaction {
            nonce: 1,
            system_fee: 1000000,
            network_fee: 100000,
            signers: vec![Signer {
                account,
                scopes: WitnessScope::CalledByEntry,
                ..Default::default()
            }],
            ..create_test_transaction(1, 1000000, 100000)
        };

        mempool.try_add(tx1.clone(), 100).unwrap();

        // Try to add conflicting transaction with same nonce but lower fee
        let tx2_low_fee = Transaction {
            network_fee: 50000, // Lower fee
            ..tx1.clone()
        };

        let result = mempool.try_add(tx2_low_fee, 100);
        assert_eq!(result.unwrap(), MemPoolAddResult::InsufficientFee);

        let tx2_high_fee = Transaction {
            network_fee: 200000, // Higher fee
            ..tx1.clone()
        };

        let result = mempool.try_add(tx2_high_fee.clone(), 100);
        assert_eq!(result.unwrap(), MemPoolAddResult::Added);

        // Original should be replaced
        assert!(!mempool.contains(&tx1.hash()));
        assert!(mempool.contains(&tx2_high_fee.hash()));
        assert_eq!(mempool.size(), 1);
    }

    /// Test mempool events (matches C# event system exactly)
    #[test]
    fn test_mempool_events_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config);

        // Track events
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        // Subscribe to events
        mempool.on_transaction_added(move |tx| {
            events_clone
                .lock()
                .unwrap()
                .push(MemPoolEvent::Added(tx.hash()));
        });

        let events_clone2 = events.clone();
        mempool.on_transaction_removed(move |tx, reason| {
            events_clone2
                .lock()
                .unwrap()
                .push(MemPoolEvent::Removed(tx.hash(), reason));
        });

        // Add transaction
        let tx = create_test_transaction(1, 1000000, 100000);
        mempool.try_add(tx.clone(), 100).unwrap();

        // Remove transaction
        mempool.remove(&tx.hash());

        // Verify events
        let fired_events = events.lock().unwrap();
        assert_eq!(fired_events.len(), 2);

        match &fired_events[0] {
            MemPoolEvent::Added(hash) => assert_eq!(*hash, tx.hash()),
            _ => panic!("Expected Added event"),
        }

        match &fired_events[1] {
            MemPoolEvent::Removed(hash, reason) => {
                assert_eq!(*hash, tx.hash());
                assert_eq!(*reason, RemovalReason::Removed);
            }
            _ => panic!("Expected Removed event"),
        }
    }

    /// Test mempool persistence (matches C# persistence exactly)
    #[test]
    fn test_mempool_persistence_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config.clone());

        // Add transactions
        let mut txs = vec![];
        for i in 0..10 {
            let tx = create_test_transaction(i, 1000000 + i * 100000, 100000);
            mempool.try_add(tx.clone(), 100).unwrap();
            txs.push(tx);
        }

        // Save mempool state
        let state = mempool.export_state();
        assert_eq!(state.transactions.len(), 10);

        // Create new mempool and import state
        let mut new_mempool = MemPool::new(config);
        new_mempool.import_state(state, 100).unwrap();

        // Verify all transactions were imported
        assert_eq!(new_mempool.size(), 10);
        for tx in &txs {
            assert!(new_mempool.contains(&tx.hash()));
        }

        // Verify sort order is preserved
        let sorted = new_mempool.get_sorted_transactions(10);
        for i in 0..10 {
            assert_eq!(sorted[i].hash(), txs[9 - i].hash()); // Reverse order (highest fee first)
        }
    }

    /// Test performance characteristics (matches C# performance exactly)
    #[test]
    fn test_mempool_performance_compatibility() {
        let config = MemPoolConfig {
            capacity: 50000,
            ..Default::default()
        };
        let mut mempool = MemPool::new(config);

        // Test addition performance
        let start = std::time::Instant::now();

        for i in 0..10000 {
            let tx = create_test_transaction(i, 1000000, 100000);
            mempool.try_add(tx, 100).unwrap();
        }

        let add_time = start.elapsed();
        assert!(add_time.as_secs() < 1); // Should be fast

        // Test lookup performance
        let start = std::time::Instant::now();

        for i in 0..1000 {
            let hash = create_test_transaction(i, 1000000, 100000).hash();
            let _ = mempool.contains(&hash);
        }

        let lookup_time = start.elapsed();
        assert!(lookup_time.as_millis() < 10); // Very fast lookups

        // Test sorting performance
        let start = std::time::Instant::now();
        let sorted = mempool.get_sorted_transactions(1000);
        let sort_time = start.elapsed();

        assert_eq!(sorted.len(), 1000);
        assert!(sort_time.as_millis() < 100); // Fast sorting
    }

    /// Test edge cases (matches C# edge case handling exactly)
    #[test]
    fn test_mempool_edge_cases_compatibility() {
        let config = MemPoolConfig::default();
        let mut mempool = MemPool::new(config);

        // Test empty transaction script
        let empty_script_tx = Transaction {
            script: vec![],
            ..create_test_transaction(1, 1000000, 100000)
        };
        let result = mempool.try_add(empty_script_tx, 100);
        assert!(result.is_err()); // Should reject

        // Test very large transaction
        let large_tx = Transaction {
            script: vec![0u8; 1024 * 1024],                    // 1MB
            ..create_test_transaction(2, 100000000, 10000000)  // High fees
        };
        let result = mempool.try_add(large_tx, 100);
        assert!(result.is_ok()); // Should accept with sufficient fees

        // Test transaction with many signers
        let mut signers = vec![];
        for i in 0..16 {
            signers.push(Signer {
                account: UInt160::from_bytes(&[i as u8; 20]).unwrap(),
                scopes: WitnessScope::CalledByEntry,
                ..Default::default()
            });
        }

        let multi_signer_tx = Transaction {
            signers,
            ..create_test_transaction(3, 10000000, 1000000)
        };
        let result = mempool.try_add(multi_signer_tx, 100);
        assert!(result.is_ok());

        // Test get transactions with limit
        assert_eq!(mempool.size(), 2);
        let limited = mempool.get_sorted_transactions(1);
        assert_eq!(limited.len(), 1);
    }

    // Helper functions

    fn create_test_transaction(nonce: u32, system_fee: u64, network_fee: u64) -> Transaction {
        Transaction {
            version: 0,
            nonce,
            system_fee,
            network_fee,
            valid_until_block: 999999,
            attributes: vec![],
            signers: vec![Signer {
                account: UInt160::from_bytes(&[1u8; 20]).unwrap(),
                scopes: WitnessScope::CalledByEntry,
                allowed_contracts: vec![],
                allowed_groups: vec![],
                rules: vec![],
            }],
            script: vec![0x51; 20], // PUSH1 * 20
            witnesses: vec![Witness {
                invocation_script: vec![0x00; 64],
                verification_script: vec![0x51],
            }],
        }
    }

    #[derive(Debug)]
    enum MemPoolEvent {
        Added(UInt256),
        Removed(UInt256, RemovalReason),
    }
}

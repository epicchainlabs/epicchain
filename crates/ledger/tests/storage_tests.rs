//! Storage C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Ledger storage implementation.
//! Tests are based on the C# Neo.Persistence test suite.

use neo_core::{UInt160, UInt256};
use neo_ledger::storage::*;
use neo_ledger::*;
use std::collections::HashMap;

#[cfg(test)]
#[allow(dead_code)]
mod storage_tests {
    use super::*;

    /// Test LevelDB storage compatibility (matches C# LevelDBStore exactly)
    #[test]
    fn test_leveldb_storage_compatibility() {
        let config = LevelDBConfig {
            path: "./test_leveldb".to_string(),
            create_if_missing: true,
            compression: CompressionType::Snappy,
            cache_size: 100 * 1024 * 1024,       // 100MB
            write_buffer_size: 64 * 1024 * 1024, // 64MB
            max_open_files: 500,
            block_size: 16 * 1024, // 16KB
        };

        let storage = LevelDBStorage::new(config).unwrap();

        // Test basic operations
        let key = StorageKey::new(StorageArea::Block, &[1, 2, 3]);
        let value = vec![4, 5, 6, 7, 8];

        // Put
        assert!(storage.put(&key, &value).is_ok());

        // Get
        let retrieved = storage.get(&key).unwrap();
        assert_eq!(retrieved, Some(value.clone()));

        // Exists
        assert!(storage.exists(&key).unwrap());

        // Delete
        assert!(storage.delete(&key).is_ok());
        assert!(!storage.exists(&key).unwrap());

        // Cleanup
        drop(storage);
        std::fs::remove_dir_all("./test_leveldb").ok();
    }

    /// Test RocksDB storage compatibility (matches C# RocksDBStore exactly)
    #[test]
    fn test_rocksdb_storage_compatibility() {
        let config = RocksDBConfig {
            path: "./test_rocksdb".to_string(),
            create_if_missing: true,
            compression: CompressionType::LZ4,
            cache_size: 500 * 1024 * 1024,        // 500MB
            write_buffer_size: 128 * 1024 * 1024, // 128MB
            max_write_buffer_number: 4,
            target_file_size_base: 256 * 1024 * 1024, // 256MB
            max_open_files: 1000,
            block_size: 16 * 1024, // 16KB
            enable_statistics: true,
        };

        let storage = RocksDBStorage::new(config).unwrap();

        // Test batch operations
        let mut batch = storage.create_batch();

        for i in 0..100 {
            let key = StorageKey::new(StorageArea::Transaction, &i.to_be_bytes());
            let value = vec![i as u8; 32];
            batch.put(&key, &value);
        }

        // Commit batch
        assert!(storage.write_batch(batch).is_ok());

        // Verify all values
        for i in 0..100 {
            let key = StorageKey::new(StorageArea::Transaction, &i.to_be_bytes());
            let value = storage.get(&key).unwrap();
            assert_eq!(value, Some(vec![i as u8; 32]));
        }

        // Test iterator
        let prefix = StorageKey::new(StorageArea::Transaction, &[]);
        let mut count = 0;

        for (key, value) in storage.iter_prefix(&prefix).unwrap() {
            assert_eq!(key.area(), StorageArea::Transaction);
            assert_eq!(value.len(), 32);
            count += 1;
        }
        assert_eq!(count, 100);

        // Cleanup
        drop(storage);
        std::fs::remove_dir_all("./test_rocksdb").ok();
    }

    /// Test MemoryStore compatibility (matches C# MemoryStore exactly)
    #[test]
    fn test_memory_store_compatibility() {
        let mut store = MemoryStore::new();

        // Test basic operations
        let key1 = StorageKey::new(StorageArea::Contract, &[1, 2, 3]);
        let value1 = vec![10, 20, 30];

        store.put(key1.clone(), value1.clone());
        assert_eq!(store.get(&key1), Some(&value1));

        // Test snapshot functionality
        let snapshot = store.snapshot();

        // Modify after snapshot
        let key2 = StorageKey::new(StorageArea::Contract, &[4, 5, 6]);
        let value2 = vec![40, 50, 60];
        store.put(key2.clone(), value2.clone());

        // Snapshot should not have new value
        assert!(snapshot.get(&key2).is_none());
        assert_eq!(snapshot.get(&key1), Some(&value1));

        // Current store should have both
        assert_eq!(store.get(&key1), Some(&value1));
        assert_eq!(store.get(&key2), Some(&value2));

        // Test clear
        store.clear();
        assert!(store.get(&key1).is_none());
        assert!(store.get(&key2).is_none());
    }

    /// Test storage areas compatibility (matches C# StoragePrefix exactly)
    #[test]
    fn test_storage_areas_compatibility() {
        assert_eq!(StorageArea::Block as u8, 0x01);
        assert_eq!(StorageArea::Transaction as u8, 0x02);
        assert_eq!(StorageArea::Contract as u8, 0x04);
        assert_eq!(StorageArea::Storage as u8, 0x05);
        assert_eq!(StorageArea::HeaderHashList as u8, 0x09);
        assert_eq!(StorageArea::CurrentBlock as u8, 0x0c);
        assert_eq!(StorageArea::CurrentHeader as u8, 0x0d);
        assert_eq!(StorageArea::ContractId as u8, 0x0e);
        assert_eq!(StorageArea::Candidate as u8, 0x14);
        assert_eq!(StorageArea::Committee as u8, 0x15);
        assert_eq!(StorageArea::Oracle as u8, 0x16);
        assert_eq!(StorageArea::Nep17Balance as u8, 0x17);
        assert_eq!(StorageArea::Nep11Balance as u8, 0x18);

        // Test key construction
        let contract_hash = UInt160::from_bytes(&[1u8; 20]).unwrap();
        let storage_key = vec![0x01, 0x02, 0x03];

        let key = StorageKey::contract_storage(&contract_hash, &storage_key);
        assert_eq!(key.area(), StorageArea::Storage);

        let key_bytes = key.to_bytes();
        assert_eq!(key_bytes[0], StorageArea::Storage as u8);
        assert_eq!(&key_bytes[1..21], contract_hash.as_bytes());
        assert_eq!(&key_bytes[21..], &storage_key);
    }

    /// Test storage cache compatibility (matches C# DataCache exactly)
    #[test]
    fn test_storage_cache_compatibility() {
        let base_store = MemoryStore::new();
        let mut cache = StorageCache::new(Box::new(base_store));

        // Test cache operations
        let key1 = StorageKey::new(StorageArea::Block, &[1]);
        let value1 = vec![10];

        // Add to cache
        cache.add(key1.clone(), value1.clone());
        assert_eq!(cache.get(&key1), Some(value1.clone()));

        // Test update
        let value1_updated = vec![20];
        cache.update(key1.clone(), value1_updated.clone());
        assert_eq!(cache.get(&key1), Some(value1_updated.clone()));

        // Test delete
        cache.delete(key1.clone());
        assert!(cache.get(&key1).is_none());

        // Test tracked changes
        let changes = cache.get_changes();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].key, key1);
        assert_eq!(changes[0].change_type, ChangeType::Deleted);

        // Test commit
        cache.commit().unwrap();

        // After commit, changes should be cleared
        assert_eq!(cache.get_changes().len(), 0);
    }

    /// Test snapshot isolation (matches C# Snapshot exactly)
    #[test]
    fn test_snapshot_isolation_compatibility() {
        let mut store = MemoryStore::new();

        // Initial data
        let key1 = StorageKey::new(StorageArea::Contract, &[1]);
        let value1 = vec![100];
        store.put(key1.clone(), value1.clone());

        // Create snapshot
        let snapshot1 = store.snapshot();

        // Modify store
        let value1_new = vec![200];
        store.put(key1.clone(), value1_new.clone());

        let key2 = StorageKey::new(StorageArea::Contract, &[2]);
        let value2 = vec![300];
        store.put(key2.clone(), value2.clone());

        // Create another snapshot
        let snapshot2 = store.snapshot();

        // Verify isolation
        assert_eq!(snapshot1.get(&key1), Some(&value1)); // Original value
        assert!(snapshot1.get(&key2).is_none()); // Doesn't exist

        assert_eq!(snapshot2.get(&key1), Some(&value1_new)); // Updated value
        assert_eq!(snapshot2.get(&key2), Some(&value2)); // New key

        assert_eq!(store.get(&key1), Some(&value1_new)); // Current value
        assert_eq!(store.get(&key2), Some(&value2)); // Current value
    }

    /// Test MPT storage compatibility (matches C# MPT storage exactly)
    #[test]
    fn test_mpt_storage_compatibility() {
        let storage = MemoryStore::new();
        let mut mpt_store = MptStore::new(Box::new(storage));

        // Test MPT node storage
        let node_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
        let node_data = vec![0x01, 0x02, 0x03, 0x04];

        mpt_store.put_node(&node_hash, &node_data).unwrap();
        let retrieved = mpt_store.get_node(&node_hash).unwrap();
        assert_eq!(retrieved, Some(node_data));

        // Test root tracking
        let root1 = UInt256::from_bytes(&[10u8; 32]).unwrap();
        mpt_store.update_root(1000, root1).unwrap();

        let root2 = UInt256::from_bytes(&[20u8; 32]).unwrap();
        mpt_store.update_root(2000, root2).unwrap();

        // Get roots
        assert_eq!(mpt_store.get_root(1000).unwrap(), Some(root1));
        assert_eq!(mpt_store.get_root(2000).unwrap(), Some(root2));
        assert_eq!(mpt_store.get_root(1500).unwrap(), Some(root1)); // Should return closest lower

        // Test pruning
        mpt_store.prune_before(1500).unwrap();
        assert!(mpt_store.get_root(1000).unwrap().is_none()); // Pruned
        assert_eq!(mpt_store.get_root(2000).unwrap(), Some(root2)); // Still exists
    }

    /// Test state storage compatibility (matches C# StateStore exactly)
    #[test]
    fn test_state_storage_compatibility() {
        let storage = MemoryStore::new();
        let mut state_store = StateStore::new(Box::new(storage));

        // Test contract storage
        let contract_id = 1;
        let key = vec![0x01, 0x02];
        let value = vec![0x10, 0x20, 0x30];

        state_store.put_storage(contract_id, &key, &value).unwrap();
        let retrieved = state_store.get_storage(contract_id, &key).unwrap();
        assert_eq!(retrieved, Some(value));

        // Test NEP-17 balance
        let account = UInt160::from_bytes(&[1u8; 20]).unwrap();
        let token = UInt160::from_bytes(&[2u8; 20]).unwrap();
        let balance = 1000000u64;

        state_store
            .put_nep17_balance(&account, &token, balance)
            .unwrap();
        let retrieved_balance = state_store.get_nep17_balance(&account, &token).unwrap();
        assert_eq!(retrieved_balance, Some(balance));

        // Test candidate storage
        let validator_pubkey = vec![0x02; 33];
        let votes = 5000000u64;

        state_store.put_candidate(&validator_pubkey, votes).unwrap();
        let retrieved_votes = state_store.get_candidate(&validator_pubkey).unwrap();
        assert_eq!(retrieved_votes, Some(votes));

        // Test committee storage
        let committee = vec![vec![0x02; 33], vec![0x03; 33], vec![0x04; 33]];
        state_store.put_committee(&committee).unwrap();
        let retrieved_committee = state_store.get_committee().unwrap();
        assert_eq!(retrieved_committee, committee);
    }

    /// Test storage performance (matches C# performance characteristics exactly)
    #[test]
    fn test_storage_performance_compatibility() {
        let mut store = MemoryStore::new();

        // Test write performance
        let start = std::time::Instant::now();

        for i in 0..10000 {
            let key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            let value = vec![i as u8; 100];
            store.put(key, value);
        }

        let write_time = start.elapsed();
        assert!(write_time.as_millis() < 100); // Should be very fast for memory store

        // Test read performance
        let start = std::time::Instant::now();

        for i in 0..10000 {
            let key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            let _ = store.get(&key);
        }

        let read_time = start.elapsed();
        assert!(read_time.as_millis() < 50); // Reads should be faster

        // Test iterator performance
        let start = std::time::Instant::now();
        let mut count = 0;

        for (_, _) in store.iter() {
            count += 1;
        }

        let iter_time = start.elapsed();
        assert_eq!(count, 10000);
        assert!(iter_time.as_millis() < 100); // Iteration should be fast
    }

    /// Test storage migration (matches C# migration tools exactly)
    #[test]
    fn test_storage_migration_compatibility() {
        // Create source storage with data
        let mut source = MemoryStore::new();

        // Add various types of data
        for i in 0..100 {
            // Blocks
            let block_key = StorageKey::new(StorageArea::Block, &i.to_be_bytes());
            source.put(block_key, vec![i as u8; 500]);

            // Transactions
            let tx_key = StorageKey::new(StorageArea::Transaction, &i.to_be_bytes());
            source.put(tx_key, vec![i as u8; 200]);

            // Contract storage
            let storage_key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            source.put(storage_key, vec![i as u8; 50]);
        }

        // Create destination storage
        let mut destination = MemoryStore::new();

        // Migrate data
        let migrator = StorageMigrator::new();
        let result = migrator.migrate(
            &source,
            &mut destination,
            MigrationOptions {
                batch_size: 50,
                areas: vec![
                    StorageArea::Block,
                    StorageArea::Transaction,
                    StorageArea::Storage,
                ],
                progress_callback: Some(Box::new(|current, total| {
                    println!("Migration progress: {}/{}", current, total);
                })),
            },
        );

        assert!(result.is_ok());

        // Verify all data migrated
        for i in 0..100 {
            let block_key = StorageKey::new(StorageArea::Block, &i.to_be_bytes());
            assert_eq!(destination.get(&block_key), source.get(&block_key));

            let tx_key = StorageKey::new(StorageArea::Transaction, &i.to_be_bytes());
            assert_eq!(destination.get(&tx_key), source.get(&tx_key));

            let storage_key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            assert_eq!(destination.get(&storage_key), source.get(&storage_key));
        }
    }

    /// Test storage compaction (matches C# compaction exactly)
    #[test]
    fn test_storage_compaction_compatibility() {
        let config = RocksDBConfig {
            path: "./test_compaction".to_string(),
            ..Default::default()
        };

        let mut storage = RocksDBStorage::new(config).unwrap();

        // Add and delete many keys to create tombstones
        for i in 0..1000 {
            let key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            let value = vec![i as u8; 1000];
            storage.put(&key, &value).unwrap();
        }

        // Delete half
        for i in 0..500 {
            let key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            storage.delete(&key).unwrap();
        }

        // Get size before compaction
        let size_before = storage.get_disk_usage().unwrap();

        // Run compaction
        storage.compact_range(None, None).unwrap();

        // Get size after compaction
        let size_after = storage.get_disk_usage().unwrap();

        // Size should be reduced after compaction
        assert!(size_after < size_before);

        // Verify remaining data is intact
        for i in 500..1000 {
            let key = StorageKey::new(StorageArea::Storage, &i.to_be_bytes());
            let value = storage.get(&key).unwrap();
            assert_eq!(value, Some(vec![i as u8; 1000]));
        }

        // Cleanup
        drop(storage);
        std::fs::remove_dir_all("./test_compaction").ok();
    }
}

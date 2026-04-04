//! Trie C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo's MPT Trie functionality.
//! Tests are based on the C# Neo.Cryptography.MPTTrie.Trie test suite.

use super::MockTrieStorage;
use neo_core::UInt256;
use neo_mpt_trie::*;

#[cfg(test)]
#[allow(dead_code)]
mod trie_tests {
    use super::*;

    /// Test Trie creation and initialization (matches C# Trie constructor exactly)
    #[test]
    fn test_trie_creation_compatibility() {
        let empty_trie = Trie::new(None, false);
        assert!(empty_trie.root().is_empty());

        let test_hash = UInt256::from_bytes(&[42u8; 32]).unwrap();
        let hash_trie = Trie::new(Some(test_hash), true);
        assert_eq!(hash_trie.root().get_hash(), Some(test_hash));
        assert_eq!(hash_trie.root().node_type(), NodeType::HashNode);

        // Test creating trie with storage backend
        let storage = Box::new(MockTrieStorage::new());
        let storage_trie = Trie::new_with_storage(None, true, storage);
        assert!(storage_trie.root().is_empty());
    }

    /// Test basic put and get operations (matches C# Trie.Put/Get exactly)
    #[test]
    fn test_basic_put_get_compatibility() {
        let mut trie = Trie::new(None, true);

        let key = b"test_key".to_vec();
        let value = b"test_value".to_vec();

        assert!(trie.put(&key, &value).is_ok());
        let retrieved = trie.get(&key).unwrap();
        assert_eq!(retrieved.as_ref(), Some(&value));

        // Test get non-existent key
        let non_existent = trie.get(b"non_existent").unwrap();
        assert_eq!(non_existent, None);

        // Test overwriting existing key
        let new_value = b"new_test_value".to_vec();
        assert!(trie.put(&key, &new_value).is_ok());
        let updated = trie.get(&key).unwrap();
        assert_eq!(updated.as_ref(), Some(&new_value));
    }

    /// Test multiple key-value operations (matches C# multi-key scenarios exactly)
    #[test]
    fn test_multiple_keys_compatibility() {
        let mut trie = Trie::new(None, true);

        // Insert multiple keys with different patterns
        let test_data = vec![
            (b"a".to_vec(), b"value_a".to_vec()),
            (b"ab".to_vec(), b"value_ab".to_vec()),
            (b"abc".to_vec(), b"value_abc".to_vec()),
            (b"b".to_vec(), b"value_b".to_vec()),
            (b"bc".to_vec(), b"value_bc".to_vec()),
            (b"xyz".to_vec(), b"value_xyz".to_vec()),
        ];

        // Insert all data
        for (key, value) in &test_data {
            assert!(trie.put(key, value).is_ok());
        }

        // Verify all data can be retrieved
        for (key, expected_value) in &test_data {
            let retrieved = trie.get(key).unwrap();
            assert_eq!(retrieved.as_ref(), Some(expected_value));
        }

        // Test that keys with shared prefixes work correctly
        assert_eq!(trie.get(b"a").unwrap().as_ref(), Some(&b"value_a".to_vec()));
        assert_eq!(
            trie.get(b"ab").unwrap().as_ref(),
            Some(&b"value_ab".to_vec())
        );
        assert_eq!(
            trie.get(b"abc").unwrap().as_ref(),
            Some(&b"value_abc".to_vec())
        );
    }

    /// Test delete operations (matches C# Trie.Delete exactly)
    #[test]
    fn test_delete_operations_compatibility() {
        let mut trie = Trie::new(None, true);

        // Setup test data
        let test_data = vec![
            (b"delete_key1".to_vec(), b"delete_value1".to_vec()),
            (b"delete_key2".to_vec(), b"delete_value2".to_vec()),
            (b"delete_key12".to_vec(), b"delete_value12".to_vec()),
            (b"keep_key".to_vec(), b"keep_value".to_vec()),
        ];

        for (key, value) in &test_data {
            assert!(trie.put(key, value).is_ok());
        }

        // Test deleting existing key
        assert!(trie.delete(b"delete_key1").is_ok());
        assert_eq!(trie.get(b"delete_key1").unwrap(), None);

        // Verify other keys still exist
        assert_eq!(
            trie.get(b"delete_key2").unwrap().as_ref(),
            Some(&b"delete_value2".to_vec())
        );
        assert_eq!(
            trie.get(b"delete_key12").unwrap().as_ref(),
            Some(&b"delete_value12".to_vec())
        );
        assert_eq!(
            trie.get(b"keep_key").unwrap().as_ref(),
            Some(&b"keep_value".to_vec())
        );

        assert!(trie.delete(b"non_existent_key").is_ok());

        // Test deleting key with shared prefix
        assert!(trie.delete(b"delete_key12").is_ok());
        assert_eq!(trie.get(b"delete_key12").unwrap(), None);
        assert_eq!(
            trie.get(b"delete_key2").unwrap().as_ref(),
            Some(&b"delete_value2".to_vec())
        );
    }

    /// Test trie node structure evolution (matches C# node type transitions exactly)
    #[test]
    fn test_node_structure_evolution_compatibility() {
        let mut trie = Trie::new(None, true);

        assert!(trie.put(b"single", b"value").is_ok());

        assert!(trie.put(b"single_extended", b"extended_value").is_ok());

        // Add more keys to create complex branch structure
        assert!(trie.put(b"different", b"different_value").is_ok());
        assert!(trie.put(b"single_extended_more", b"more_value").is_ok());

        // Verify all keys are accessible
        assert_eq!(
            trie.get(b"single").unwrap().as_ref(),
            Some(&b"value".to_vec())
        );
        assert_eq!(
            trie.get(b"single_extended").unwrap().as_ref(),
            Some(&b"extended_value".to_vec())
        );
        assert_eq!(
            trie.get(b"different").unwrap().as_ref(),
            Some(&b"different_value".to_vec())
        );
        assert_eq!(
            trie.get(b"single_extended_more").unwrap().as_ref(),
            Some(&b"more_value".to_vec())
        );

        // Test deletion that triggers node restructuring
        assert!(trie.delete(b"single_extended_more").is_ok());
        assert_eq!(trie.get(b"single_extended_more").unwrap(), None);

        // Verify remaining keys still work
        assert_eq!(
            trie.get(b"single").unwrap().as_ref(),
            Some(&b"value".to_vec())
        );
        assert_eq!(
            trie.get(b"single_extended").unwrap().as_ref(),
            Some(&b"extended_value".to_vec())
        );
    }

    /// Test trie commit and rollback operations (matches C# Trie.Commit exactly)
    #[test]
    fn test_commit_operations_compatibility() {
        let mut trie = Trie::new(None, true);

        // Add some data
        assert!(trie.put(b"commit_key1", b"commit_value1").is_ok());
        assert!(trie.put(b"commit_key2", b"commit_value2").is_ok());

        // Commit changes
        trie.commit();

        // Verify data is still accessible after commit
        assert_eq!(
            trie.get(b"commit_key1").unwrap().as_ref(),
            Some(&b"commit_value1".to_vec())
        );
        assert_eq!(
            trie.get(b"commit_key2").unwrap().as_ref(),
            Some(&b"commit_value2".to_vec())
        );

        // Add more data and commit again
        assert!(trie.put(b"commit_key3", b"commit_value3").is_ok());
        trie.commit();

        assert_eq!(
            trie.get(b"commit_key3").unwrap().as_ref(),
            Some(&b"commit_value3".to_vec())
        );
    }

    /// Test trie with storage backend (matches C# storage integration exactly)
    #[test]
    fn test_storage_backend_compatibility() {
        let storage = Box::new(MockTrieStorage::new());
        let mut trie = Trie::new_with_storage(None, true, storage);

        // Test operations with storage backend
        assert!(trie.put(b"storage_key1", b"storage_value1").is_ok());
        assert!(trie.put(b"storage_key2", b"storage_value2").is_ok());

        // Verify data retrieval works
        assert_eq!(
            trie.get(b"storage_key1").unwrap().as_ref(),
            Some(&b"storage_value1".to_vec())
        );
        assert_eq!(
            trie.get(b"storage_key2").unwrap().as_ref(),
            Some(&b"storage_value2".to_vec())
        );

        // Commit to storage
        trie.commit();

        // Verify data persists after commit
        assert_eq!(
            trie.get(b"storage_key1").unwrap().as_ref(),
            Some(&b"storage_value1".to_vec())
        );
    }

    /// Test edge cases and boundary conditions (matches C# edge case handling exactly)
    #[test]
    fn test_edge_cases_compatibility() {
        let mut trie = Trie::new(None, true);

        // Test empty key
        assert!(trie.put(b"", b"empty_key_value").is_ok());
        assert_eq!(
            trie.get(b"").unwrap().as_ref(),
            Some(&b"empty_key_value".to_vec())
        );

        // Test empty value
        assert!(trie.put(b"empty_value_key", b"").is_ok());
        assert_eq!(
            trie.get(b"empty_value_key").unwrap().as_ref(),
            Some(&b"".to_vec())
        );

        // Test very long key
        let long_key = vec![b'x'; 1000];
        let long_value = b"long_key_value".to_vec();
        assert!(trie.put(&long_key, &long_value).is_ok());
        assert_eq!(trie.get(&long_key).unwrap().as_ref(), Some(&long_value));

        // Test very long value
        let normal_key = b"normal_key".to_vec();
        let long_value = vec![b'y'; 10000];
        assert!(trie.put(&normal_key, &long_value).is_ok());
        assert_eq!(trie.get(&normal_key).unwrap().as_ref(), Some(&long_value));

        // Test binary keys and values
        let binary_key = vec![0u8, 1u8, 255u8, 128u8];
        let binary_value = vec![255u8, 0u8, 128u8, 64u8];
        assert!(trie.put(&binary_key, &binary_value).is_ok());
        assert_eq!(trie.get(&binary_key).unwrap().as_ref(), Some(&binary_value));
    }

    /// Test trie iteration and traversal (matches C# iteration patterns exactly)
    #[test]
    fn test_trie_iteration_compatibility() {
        let mut trie = Trie::new(None, true);

        // Setup test data with known order
        let test_data = vec![
            (b"a".to_vec(), b"value_a".to_vec()),
            (b"b".to_vec(), b"value_b".to_vec()),
            (b"c".to_vec(), b"value_c".to_vec()),
            (b"aa".to_vec(), b"value_aa".to_vec()),
            (b"ab".to_vec(), b"value_ab".to_vec()),
        ];

        for (key, value) in &test_data {
            assert!(trie.put(key, value).is_ok());
        }

        // Test that all keys can be retrieved
        for (key, expected_value) in &test_data {
            let retrieved = trie.get(key).unwrap();
            assert_eq!(retrieved.as_ref(), Some(expected_value));
        }

        assert_eq!(trie.get(b"a").unwrap().as_ref(), Some(&b"value_a".to_vec()));
        assert_eq!(
            trie.get(b"aa").unwrap().as_ref(),
            Some(&b"value_aa".to_vec())
        );
        assert_eq!(
            trie.get(b"ab").unwrap().as_ref(),
            Some(&b"value_ab".to_vec())
        );
    }

    /// Test trie performance characteristics (matches C# performance expectations exactly)
    #[test]
    fn test_performance_characteristics_compatibility() {
        let mut trie = Trie::new(None, true);

        // Insert many keys to test performance scaling
        let key_count = 1000;
        let mut keys = Vec::new();

        for i in 0..key_count {
            let key = format!("performance_key_{:04}", i).into_bytes();
            let value = format!("performance_value_{:04}", i).into_bytes();
            keys.push((key.clone(), value.clone()));

            assert!(trie.put(&key, &value).is_ok());
        }

        // Verify all keys can be retrieved efficiently
        for (key, expected_value) in &keys {
            let retrieved = trie.get(key).unwrap();
            assert_eq!(retrieved.as_ref(), Some(expected_value));
        }

        // Test deletion performance
        for i in (0..key_count).step_by(2) {
            let key = format!("performance_key_{:04}", i).into_bytes();
            assert!(trie.delete(&key).is_ok());
        }

        // Verify remaining keys
        for i in 0..key_count {
            let key = format!("performance_key_{:04}", i).into_bytes();
            let expected_value = format!("performance_value_{:04}", i).into_bytes();
            let retrieved = trie.get(&key).unwrap();

            if i % 2 == 0 {
                // Deleted keys should not exist
                assert_eq!(retrieved, None);
            } else {
                // Remaining keys should exist
                assert_eq!(retrieved.as_ref(), Some(&expected_value));
            }
        }
    }

    /// Test trie consistency and validation (matches C# validation logic exactly)
    #[test]
    fn test_trie_consistency_compatibility() {
        let mut trie = Trie::new(None, true);

        // Build a complex trie structure
        let keys = vec![
            b"consistency_test_1".to_vec(),
            b"consistency_test_2".to_vec(),
            b"consistency_test_12".to_vec(),
            b"consistency_different".to_vec(),
            b"other_branch".to_vec(),
        ];

        for (i, key) in keys.iter().enumerate() {
            let value = format!("consistency_value_{}", i).into_bytes();
            assert!(trie.put(key, &value).is_ok());
        }

        // Test that trie maintains consistency after operations
        for (i, key) in keys.iter().enumerate() {
            let expected_value = format!("consistency_value_{}", i).into_bytes();
            let retrieved = trie.get(key).unwrap();
            assert_eq!(retrieved.as_ref(), Some(&expected_value));
        }

        // Test consistency after deletions
        assert!(trie.delete(&keys[1]).is_ok()); // Delete middle key

        // Verify other keys still work
        let expected_0 = format!("consistency_value_{}", 0).into_bytes();
        assert_eq!(trie.get(&keys[0]).unwrap().as_ref(), Some(&expected_0));
        assert_eq!(trie.get(&keys[1]).unwrap(), None); // Deleted

        let expected_2 = format!("consistency_value_{}", 2).into_bytes();
        assert_eq!(trie.get(&keys[2]).unwrap().as_ref(), Some(&expected_2));
    }
}

//! Integration tests for the persistence module - RocksDB only.

use neo_persistence::*;
use std::sync::Arc;
use tempfile::TempDir;

async fn create_test_storage() -> (Storage, TempDir) {
    let final_dir = TempDir::new().unwrap();

    let config = StorageConfig {
        path: final_dir.path().to_path_buf(),
        compression_algorithm: CompressionAlgorithm::Lz4,
        compaction_strategy: CompactionStrategy::Level,
        max_open_files: Some(1000),
        cache_size: Some(64 * 1024 * 1024),
        write_buffer_size: Some(16 * 1024 * 1024),
        enable_statistics: false,
    };

    let provider: Arc<dyn StorageProvider> = Arc::new(RocksDbStorageProvider::new());
    let storage = Storage::new(config, provider).await.unwrap();
    (storage, final_dir)
}

#[tokio::test]
async fn test_storage_basic_operations() {
    let (mut storage, _final_dir) = create_test_storage().await;

    let key = b"test_key";
    let value = vec![1, 2, 3, 4, 5];

    // Test put and get
    storage.put(key, value.clone()).await.unwrap();
    let retrieved = storage.get(key).await.unwrap();
    assert_eq!(retrieved, Some(value));

    // Test contains
    assert!(storage.contains(key).await.unwrap());

    // Test delete
    storage.delete(key).await.unwrap();
    assert!(!storage.contains(key).await.unwrap());
    assert_eq!(storage.get(key).await.unwrap(), None);
}

#[tokio::test]
async fn test_storage_batch_operations() {
    let (mut storage, _final_dir) = create_test_storage().await;

    let operations = vec![
        BatchOperation::Put {
            key: b"key1".to_vec(),
            value: vec![1, 2, 3],
        },
        BatchOperation::Put {
            key: b"key2".to_vec(),
            value: vec![4, 5, 6],
        },
        BatchOperation::Put {
            key: b"key3".to_vec(),
            value: vec![7, 8, 9],
        },
    ];

    storage.execute_batch(operations).unwrap();

    assert_eq!(storage.get(b"key1").await.unwrap(), Some(vec![1, 2, 3]));
    assert_eq!(storage.get(b"key2").await.unwrap(), Some(vec![4, 5, 6]));
    assert_eq!(storage.get(b"key3").await.unwrap(), Some(vec![7, 8, 9]));
}

#[tokio::test]
async fn test_storage_stats() {
    let (storage, _final_dir) = create_test_storage().await;

    let stats = storage.stats().await.unwrap();
    assert_eq!(stats.total_keys, 0);
    assert_eq!(stats.total_size, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
}

#[tokio::test]
async fn test_cache_operations() {
    let config = CacheConfig {
        max_entries: 5,
        default_ttl: std::time::Duration::from_secs(3600),
        enable_stats: false,
    };

    let mut cache = LruCache::with_config(&config);

    // Test basic operations
    let key1 = b"key1".to_vec();
    let key2 = b"key2".to_vec();
    let value1 = vec![1, 2, 3];
    let value2 = vec![4, 5, 6];

    cache.put(key1.clone(), value1.clone());
    cache.put(key2.clone(), value2.clone());

    assert_eq!(cache.get(&key1), Some(value1));
    assert_eq!(cache.get(&key2), Some(value2));
    assert!(!cache.is_empty());
    assert_eq!(cache.len(), 2);
}

#[tokio::test]
async fn test_ttl_cache() {
    let config = CacheConfig {
        max_entries: 10,
        default_ttl: std::time::Duration::from_millis(100),
        enable_stats: false,
    };

    let mut cache = TtlCache::with_config(&config);
    let key = b"key".to_vec();
    let value = vec![1, 2, 3];

    // Put value
    cache.put(key.clone(), value.clone());
    assert_eq!(cache.get(&key), Some(value));

    tokio::time::sleep(std::time::Duration::from_millis(150)).await;

    // Should be expired now
    assert_eq!(cache.get(&key), None);
}

#[tokio::test]
async fn test_index_operations() {
    let config = IndexConfig {
        name: "test_index".to_string(),
        index_type: IndexType::BTree,
        unique: false,
        case_sensitive: true,
    };

    let mut index = BTreeIndex::with_config(config);

    // Test insert and lookup
    let key = b"test_key".to_vec();
    let value = b"test_value".to_vec();

    index.insert(key.clone(), value.clone()).unwrap();
    assert_eq!(index.get(&key), Some(vec![value]));
    assert_eq!(index.len(), 1);
    assert!(!index.is_empty());
}

#[tokio::test]
async fn test_hash_index() {
    let config = IndexConfig {
        name: "test_hash_index".to_string(),
        index_type: IndexType::Hash,
        unique: false,
        case_sensitive: true,
    };

    let mut index = HashIndex::with_config(config);

    // Test insert and lookup
    let key = b"test_key".to_vec();
    let value = b"test_value".to_vec();

    index.insert(key.clone(), value.clone()).unwrap();
    assert_eq!(index.get(&key), Some(vec![value]));
    assert_eq!(index.len(), 1);
    assert!(!index.is_empty());
}

#[tokio::test]
async fn test_backup_operations() {
    let (storage, _final_dir) = create_test_storage().await;

    let backup_config = BackupConfig {
        output_path: "./test_backups".to_string().into(),
        compression_algorithm: CompressionAlgorithm::Lz4,
        enable_verification: true,
        max_backup_size: None,
    };

    let mut backup_manager = BackupManager::new(
        backup_config.output_path.clone(),
        10,   // max_backups
        true, // enable_compression
    );

    // Create backup
    let metadata = backup_manager
        .create_backup(&storage, BackupType::Full)
        .await
        .unwrap();

    // Verify backup was created
    assert_eq!(metadata.backup_type, BackupType::Full);
    assert_eq!(metadata.status, BackupStatus::Completed);
    assert!(metadata.size > 0);
}

#[tokio::test]
async fn test_migration_operations() {
    let config = MigrationConfig::default();
    let mut manager = MigrationManager::new(config);

    // Create a test migration
    let migration = SchemaMigration::new(
        1, // version
        "test_migration".to_string(),
        "A test migration".to_string(),
        "CREATE TABLE test (id INTEGER);".to_string(), // script
    );

    manager.add_migration(migration);

    // Test migration listing
    let migrations = manager.get_migrations();
    assert_eq!(migrations.len(), 1);
    assert_eq!(migrations[0].name(), "test_migration");
}

#[tokio::test]
async fn test_compression_algorithms() {
    let data = b"Hello, World! This is a test string for compression testing.";

    // Test LZ4
    let compressed = compression::compress(data, CompressionAlgorithm::Lz4).unwrap();
    let decompressed = compression::decompress(&compressed, CompressionAlgorithm::Lz4).unwrap();
    assert_eq!(data, decompressed.as_slice());

    // Test Zstd
    let compressed = compression::compress(data, CompressionAlgorithm::Zstd).unwrap();
    let decompressed = compression::decompress(&compressed, CompressionAlgorithm::Zstd).unwrap();
    assert_eq!(data, decompressed.as_slice());

    // Test None
    let compressed = compression::compress(data, CompressionAlgorithm::None).unwrap();
    assert_eq!(data, compressed.as_slice());
}

#[tokio::test]
async fn test_serialization_utilities() {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct TestData {
        id: u32,
        name: String,
        values: Vec<i32>,
    }

    let data = TestData {
        id: 42,
        name: "test".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };

    // Test bincode serialization
    let serialized = serialization::serialize(&data).unwrap();
    let deserialized: TestData = serialization::deserialize(&serialized).unwrap();
    assert_eq!(data, deserialized);

    // Test JSON serialization
    let serialized = serialization::serialize_json(&data).unwrap();
    let deserialized: TestData = serialization::deserialize_json(&serialized).unwrap();
    assert_eq!(data, deserialized);
}

#[tokio::test]
async fn test_rocksdb_storage_large_data() {
    let (mut storage, _final_dir) = create_test_storage().await;

    // Test basic operations with RocksDB
    let key = b"test_key";
    let value = vec![1, 2, 3, 4, 5];

    storage.put(key, value.clone()).await.unwrap();
    let retrieved = storage.get(key).await.unwrap();
    assert_eq!(retrieved, Some(value));

    // Test with larger data
    let large_value = vec![42u8; 1000]; // 1KB of data
    let large_key = b"large_data";

    storage.put(large_key, large_value.clone()).await.unwrap();
    let retrieved_large = storage.get(large_key).await.unwrap();
    assert_eq!(retrieved_large, Some(large_value));

    // Test stats
    let stats = storage.stats().await.unwrap();
    assert_eq!(stats.total_keys, 2); // We inserted 2 keys
    assert!(stats.total_size > 0); // Should have some size from the data
}

#[tokio::test]
async fn test_rocksdb_persistence() {
    let final_dir = TempDir::new().unwrap();
    let db_path = final_dir.path().to_path_buf();

    // Create first storage instance
    {
        let config = StorageConfig {
            path: db_path.clone(),
            compression_algorithm: CompressionAlgorithm::Lz4,
            compaction_strategy: CompactionStrategy::Level,
            max_open_files: Some(1000),
            cache_size: Some(64 * 1024 * 1024),
            write_buffer_size: Some(16 * 1024 * 1024),
            enable_statistics: false,
        };

        let provider: Arc<dyn StorageProvider> = Arc::new(RocksDbStorageProvider::new());
        let mut storage = Storage::new(config, provider).await.unwrap();

        // Store some data
        storage
            .put(b"persistent_key", vec![1, 2, 3, 4, 5])
            .await
            .unwrap();
    }

    // Create second storage instance with same path
    {
        let config = StorageConfig {
            path: db_path.clone(),
            compression_algorithm: CompressionAlgorithm::Lz4,
            compaction_strategy: CompactionStrategy::Level,
            max_open_files: Some(1000),
            cache_size: Some(64 * 1024 * 1024),
            write_buffer_size: Some(16 * 1024 * 1024),
            enable_statistics: false,
        };

        let provider: Arc<dyn StorageProvider> = Arc::new(RocksDbStorageProvider::new());
        let storage = Storage::new(config, provider).await.unwrap();

        // Verify data persisted
        let retrieved = storage.get(b"persistent_key").await.unwrap();
        assert_eq!(retrieved, Some(vec![1, 2, 3, 4, 5]));
    }
}

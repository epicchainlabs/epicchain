//! Storage interfaces and types.
//!
//! This module defines the core storage abstractions that exactly match C# Neo persistence interfaces.

use neo_config::MAX_SCRIPT_SIZE;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Seek direction for iteration (matches C# Neo SeekDirection)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekDirection {
    Forward,
    Backward,
}

/// Compression algorithms (matches C# Neo compression support)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    None,
    Lz4,
    Zstd,
}

/// Compaction strategy for database optimization (matches C# Neo)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompactionStrategy {
    Level,
    Universal,
    Fifo,
}

/// Batch operation for bulk database operations (matches C# Neo)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchOperation {
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
}

/// Storage configuration (matches C# Neo storage configuration, RocksDB only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub path: PathBuf,
    pub compression_algorithm: CompressionAlgorithm,
    pub compaction_strategy: CompactionStrategy,
    pub max_open_files: Option<u32>,
    pub cache_size: Option<usize>,
    pub write_buffer_size: Option<usize>,
    pub enable_statistics: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./data"),
            compression_algorithm: CompressionAlgorithm::Lz4,
            compaction_strategy: CompactionStrategy::Level,
            max_open_files: Some(1000),
            cache_size: Some(64 * MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE),
            write_buffer_size: Some(16 * MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE),
            enable_statistics: false,
        }
    }
}

/// Read-only store interface (matches C# IReadOnlyStore<TKey, TValue>)
pub trait IReadOnlyStore<TKey, TValue> {
    /// Tries to get a value by key (matches C# TryGet)
    fn try_get(&self, key: &TKey) -> Option<TValue>;

    /// Checks if a key exists (matches C# Contains)
    fn contains(&self, key: &TKey) -> bool;

    /// Finds entries with optional key prefix (matches C# Find)
    fn find(
        &self,
        key_or_prefix: Option<&[u8]>,
        direction: SeekDirection,
    ) -> Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)>>;
}

/// Write store interface (matches C# IWriteStore<TKey, TValue>)
pub trait IWriteStore<TKey, TValue> {
    /// Puts a key-value pair (matches C# Put)
    fn put(&mut self, key: TKey, value: TValue);

    /// Deletes a key (matches C# Delete)
    fn delete(&mut self, key: &TKey);

    /// Puts a key-value pair synchronously (matches C# PutSync)
    fn put_sync(&mut self, key: TKey, value: TValue) {
        self.put(key, value);
    }
}

/// Store interface (matches C# IStore)
pub trait IStore: IReadOnlyStore<Vec<u8>, Vec<u8>> + IWriteStore<Vec<u8>, Vec<u8>> {
    /// Creates a snapshot of the database (matches C# GetSnapshot)
    fn get_snapshot(&self) -> Box<dyn IStoreSnapshot>;
}

/// Store snapshot interface (matches C# IStoreSnapshot)
pub trait IStoreSnapshot: IReadOnlyStore<Vec<u8>, Vec<u8>> + IWriteStore<Vec<u8>, Vec<u8>> {
    /// Gets the store this snapshot belongs to (matches C# Store property)
    fn store(&self) -> &dyn IStore;

    /// Commits all changes in the snapshot to the database (matches C# Commit)
    fn commit(&mut self);
}

/// Storage provider interface (matches C# IStoreProvider) - RocksDB only
pub trait StorageProvider: Send + Sync {
    /// Gets the name of the storage provider
    fn name(&self) -> &str;

    /// Creates a new store instance
    fn create_store(&self, config: &StorageConfig) -> crate::Result<Box<dyn IStore>>;
}

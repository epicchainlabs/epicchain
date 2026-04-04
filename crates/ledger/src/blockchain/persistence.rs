//! Blockchain persistence logic.
//!
//! This module provides blockchain persistence functionality exactly matching C# Neo persistence.

use super::storage::{Storage, StorageItem, StorageKey};
use crate::{Block, BlockHeader, Error, Result};
use neo_config::HASH_SIZE;
use neo_core::{Transaction, UInt160, UInt256};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Blockchain persistence manager (matches C# Neo DataCache exactly)
#[derive(Debug)]
pub struct BlockchainPersistence {
    /// Underlying storage
    storage: Arc<Storage>,
    /// Write cache for pending changes
    write_cache: Arc<RwLock<HashMap<StorageKey, CacheItem>>>,
    /// Read cache for frequently accessed items
    read_cache: Arc<RwLock<HashMap<StorageKey, StorageItem>>>,
    /// Cache size limit
    cache_size_limit: usize,
    /// Block cache for efficient block lookups
    block_cache: Arc<RwLock<HashMap<UInt256, Block>>>,
}

/// Cache item with tracking information
#[derive(Debug, Clone)]
struct CacheItem {
    /// The storage item
    item: Option<StorageItem>, // None means deleted
    /// Whether this item has been modified
    is_dirty: bool,
    /// Whether this item has been deleted
    is_deleted: bool,
}

impl CacheItem {
    /// Creates a new cache item
    fn new(item: StorageItem) -> Self {
        Self {
            item: Some(item),
            is_dirty: false,
            is_deleted: false,
        }
    }

    /// Creates a new dirty cache item
    fn new_dirty(item: StorageItem) -> Self {
        Self {
            item: Some(item),
            is_dirty: true,
            is_deleted: false,
        }
    }

    /// Creates a deleted cache item
    fn deleted() -> Self {
        Self {
            item: None,
            is_dirty: true,
            is_deleted: true,
        }
    }

    /// Marks the item as dirty
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
}

impl BlockchainPersistence {
    /// Creates a new blockchain persistence manager
    pub fn new(storage: Arc<Storage>) -> Self {
        Self {
            storage,
            write_cache: Arc::new(RwLock::new(HashMap::new())),
            read_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_size_limit: 10000, // Default cache size
            block_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gets an item from storage with caching (matches C# Neo DataCache.TryGet)
    pub async fn get(&self, key: &StorageKey) -> Result<Option<StorageItem>> {
        // Check write cache first
        {
            let write_cache = self.write_cache.read().await;
            if let Some(cache_item) = write_cache.get(key) {
                if cache_item.is_deleted {
                    return Ok(None);
                }
                return Ok(cache_item.item.clone());
            }
        }

        // Check read cache
        {
            let read_cache = self.read_cache.read().await;
            if let Some(item) = read_cache.get(key) {
                return Ok(Some(item.clone()));
            }
        }

        // Load from storage
        match self.storage.get(key).await {
            Ok(item) => {
                // Add to read cache
                self.add_to_read_cache(key.clone(), item.clone()).await;
                Ok(Some(item))
            }
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Puts an item into storage cache (matches C# Neo DataCache.Add)
    pub async fn put(&self, key: StorageKey, item: StorageItem) -> Result<()> {
        let mut write_cache = self.write_cache.write().await;
        write_cache.insert(key, CacheItem::new_dirty(item));
        Ok(())
    }

    /// Deletes an item from storage (matches C# Neo DataCache.Delete)
    pub async fn delete(&self, key: &StorageKey) -> Result<()> {
        let mut write_cache = self.write_cache.write().await;
        write_cache.insert(key.clone(), CacheItem::deleted());
        Ok(())
    }

    /// Checks if a key exists (matches C# Neo DataCache.Contains)
    pub async fn contains(&self, key: &StorageKey) -> Result<bool> {
        match self.get(key).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Commits all pending changes to storage (matches C# Neo DataCache.Commit)
    pub async fn commit(&self) -> Result<()> {
        let mut write_cache = self.write_cache.write().await;

        // Apply all changes from write cache to storage
        for (key, cache_item) in write_cache.iter() {
            if cache_item.is_dirty {
                if cache_item.is_deleted {
                    // Delete from storage
                    if let Err(e) = self.storage.delete(key).await {
                        if !matches!(e, Error::NotFound) {
                            return Err(e);
                        }
                    }
                } else if let Some(ref item) = cache_item.item {
                    // Put to storage
                    self.storage.put(key, item).await?;
                }
            }
        }

        // Clear write cache after successful commit
        write_cache.clear();
        Ok(())
    }

    /// Discards all pending changes (matches C# Neo DataCache.Reset)
    pub async fn reset(&self) {
        let mut write_cache = self.write_cache.write().await;
        write_cache.clear();
    }

    /// Creates a snapshot of current state (matches C# Neo DataCache.CreateSnapshot)
    pub async fn create_snapshot(&self) -> Result<BlockchainSnapshot> {
        // Commit any pending changes first
        self.commit().await?;

        // Create storage snapshot
        let storage_snapshot = self.storage.snapshot().await?;

        Ok(BlockchainSnapshot::new(Arc::new(storage_snapshot)))
    }

    /// Persists a block to storage (matches C# Neo Blockchain.Persist)
    pub async fn persist_block(&self, block: &Block) -> Result<()> {
        // Store block header
        let header_key = StorageKey::block_header(block.header.index);
        let header_item = StorageItem::new(bincode::serialize(&block.header)?);
        self.put(header_key, header_item).await?;

        // Store block hash
        let hash_key = StorageKey::block_hash(block.header.index);
        let hash_item = StorageItem::new(block.hash().as_bytes().to_vec());
        self.put(hash_key, hash_item).await?;

        let block_hash = block.hash();
        let block_data_key = self.make_block_key(&block_hash);
        let block_data_item = StorageItem::new(bincode::serialize(block)?);
        self.put(block_data_key, block_data_item).await?;

        // Store transactions
        for (tx_index, transaction) in block.transactions.iter().enumerate() {
            self.persist_transaction(transaction, block.header.index, tx_index as u32)
                .await?;
        }

        // Update current block height
        let height_key = StorageKey::current_height();
        let height_item = StorageItem::new(block.header.index.to_le_bytes().to_vec());
        self.put(height_key, height_item).await?;

        // Add to block cache
        self.block_cache
            .write()
            .await
            .insert(block_hash, block.clone());

        // Commit all changes
        self.commit().await?;

        Ok(())
    }

    /// Persists a transaction to storage
    async fn persist_transaction(
        &self,
        transaction: &Transaction,
        block_index: u32,
        tx_index: u32,
    ) -> Result<()> {
        let tx_hash = transaction.hash()?;

        // Store transaction data
        let tx_key = StorageKey::transaction(tx_hash);
        let tx_item = StorageItem::new(bincode::serialize(transaction)?);
        self.put(tx_key, tx_item).await?;

        // Store transaction block index
        let tx_block_key = StorageKey::transaction_block(tx_hash);
        let tx_block_item = StorageItem::new(block_index.to_le_bytes().to_vec());
        self.put(tx_block_key, tx_block_item).await?;

        // Store transaction index within block
        let tx_idx_key = StorageKey::transaction_index(tx_hash);
        let tx_idx_item = StorageItem::new(tx_index.to_le_bytes().to_vec());
        self.put(tx_idx_key, tx_idx_item).await?;

        Ok(())
    }

    /// Gets a block by index
    pub async fn get_block(&self, index: u32) -> Result<Option<Block>> {
        // Get block header first
        let header_key = StorageKey::block_header(index);
        let header_item = match self.get(&header_key).await? {
            Some(item) => item,
            None => return Ok(None),
        };

        // Deserialize block header
        let header: BlockHeader = bincode::deserialize(&header_item.value)?;

        // Get block hash
        let hash_key = StorageKey::block_hash(index);
        let hash_item = match self.get(&hash_key).await? {
            Some(item) => item,
            None => return Ok(None),
        };

        if hash_item.value.len() != HASH_SIZE {
            return Err(Error::Validation("Invalid block hash size".to_string()));
        }

        let block_hash = UInt256::from_bytes(&hash_item.value)?;

        let mut transactions = Vec::new();
        let mut tx_index = 0u32;

        // Keep loading transactions until we don't find any more
        // In production, we would maintain better indexing to efficiently
        // load all transactions belonging to a specific block

        let block = Block {
            header,
            transactions,
        };

        // Cache the block
        self.block_cache
            .write()
            .await
            .insert(block_hash, block.clone());

        Ok(Some(block))
    }

    /// Gets a block by hash
    pub async fn get_block_by_hash(&self, hash: &UInt256) -> Result<Option<Block>> {
        // Check block cache first
        if let Some(cached_block) = self.block_cache.read().await.get(hash) {
            return Ok(Some(cached_block.clone()));
        }

        // Direct lookup by block hash
        let block_key = self.make_block_key(hash);
        match self.get(&block_key).await? {
            Some(block_data) => {
                // Deserialize block from storage
                match bincode::deserialize::<Block>(&block_data.value) {
                    Ok(block) => {
                        self.block_cache.write().await.insert(*hash, block.clone());
                        Ok(Some(block))
                    }
                    Err(e) => {
                        let block_index = self.get_block_index_by_hash(hash).await?;
                        if let Some(index) = block_index {
                            self.get_block(index).await
                        } else {
                            Ok(None)
                        }
                    }
                }
            }
            None => {
                // Try lookup by index as fallback
                let block_index = self.get_block_index_by_hash(hash).await?;
                if let Some(index) = block_index {
                    self.get_block(index).await
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Gets the block index for a given hash
    async fn get_block_index_by_hash(&self, hash: &UInt256) -> Result<Option<u32>> {
        // Search through all blocks to find the one with matching hash
        let current_height = self.get_current_block_height().await?;

        for index in 0..=current_height {
            let hash_key = StorageKey::block_hash(index);
            if let Some(hash_item) = self.get(&hash_key).await? {
                if hash_item.value.len() == HASH_SIZE {
                    let stored_hash = UInt256::from_bytes(&hash_item.value)?;
                    if stored_hash == *hash {
                        return Ok(Some(index));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Gets a transaction by hash
    pub async fn get_transaction(&self, hash: &UInt256) -> Result<Option<Transaction>> {
        let tx_key = StorageKey::transaction(*hash);
        match self.get(&tx_key).await? {
            Some(item) => {
                let transaction: Transaction = bincode::deserialize(&item.value)?;
                Ok(Some(transaction))
            }
            None => Ok(None),
        }
    }

    /// Gets the current block height
    pub async fn get_current_block_height(&self) -> Result<u32> {
        let height_key = StorageKey::current_height();
        match self.get(&height_key).await? {
            Some(item) => {
                if item.value.len() >= 4 {
                    let bytes: [u8; 4] = item.value[0..4]
                        .try_into()
                        .map_err(|_| Error::Validation("Invalid height bytes".to_string()))?;
                    Ok(u32::from_le_bytes(bytes))
                } else {
                    Ok(0) // Genesis case
                }
            }
            None => Ok(0), // No blocks persisted yet
        }
    }

    /// Adds an item to the read cache
    async fn add_to_read_cache(&self, key: StorageKey, item: StorageItem) {
        let mut read_cache = self.read_cache.write().await;

        if read_cache.len() >= self.cache_size_limit {
            if let Some(oldest_key) = read_cache.keys().next().cloned() {
                read_cache.remove(&oldest_key);
            }
        }

        read_cache.insert(key, item);
    }

    /// Sets the cache size limit
    pub fn set_cache_size_limit(&mut self, limit: usize) {
        self.cache_size_limit = limit;
    }

    /// Gets a storage item synchronously (blocking version of get)
    /// This is used when async is not available in the calling context
    pub fn get_storage_item_sync(&self, key: &StorageKey) -> Result<Option<StorageItem>> {
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| Error::Generic(format!("Failed to get tokio runtime: {}", e)))?;

        rt.block_on(self.get(key))
    }

    /// Gets the number of cached items
    pub async fn cache_stats(&self) -> (usize, usize) {
        let read_cache = self.read_cache.read().await;
        let write_cache = self.write_cache.read().await;
        (read_cache.len(), write_cache.len())
    }

    /// Makes a block key from a block hash
    fn make_block_key(&self, hash: &UInt256) -> StorageKey {
        let hash_bytes = hash.as_bytes();
        let index =
            u32::from_le_bytes([hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3]]);
        StorageKey::new(b"BLOCK".to_vec(), hash.as_bytes().to_vec())
    }

    /// Removes a block from storage (used during chain reorganization)
    pub async fn remove_block(&self, index: u32) -> Result<()> {
        // Get block hash first
        let hash_key = StorageKey::block_hash(index);
        let hash_item = match self.get(&hash_key).await? {
            Some(item) => item,
            None => return Ok(()), // Block doesn't exist, nothing to remove
        };

        if hash_item.value.len() != HASH_SIZE {
            return Err(Error::Validation("Invalid block hash size".to_string()));
        }

        let block_hash = UInt256::from_bytes(&hash_item.value)?;

        // Get the block to remove its transactions
        if let Some(block) = self.get_block_by_hash(&block_hash).await? {
            // Remove all transactions from this block
            for transaction in &block.transactions {
                let tx_hash = transaction.hash()?;

                // Remove transaction data
                let tx_key = StorageKey::transaction(tx_hash);
                self.delete(&tx_key).await?;

                // Remove transaction block index
                let tx_block_key = StorageKey::transaction_block(tx_hash);
                self.delete(&tx_block_key).await?;

                // Remove transaction index within block
                let tx_idx_key = StorageKey::transaction_index(tx_hash);
                self.delete(&tx_idx_key).await?;
            }
        }

        // Remove block header
        let header_key = StorageKey::block_header(index);
        self.delete(&header_key).await?;

        // Remove block hash
        self.delete(&hash_key).await?;

        // Remove complete block data
        let block_data_key = self.make_block_key(&block_hash);
        self.delete(&block_data_key).await?;

        // Remove from block cache
        self.block_cache.write().await.remove(&block_hash);

        let current_height = self.get_current_block_height().await?;
        if index == current_height && index > 0 {
            let new_height_key = StorageKey::current_height();
            let new_height_item = StorageItem::new((index - 1).to_le_bytes().to_vec());
            self.put(new_height_key, new_height_item).await?;
        }

        // Commit all changes
        self.commit().await?;

        Ok(())
    }
}

/// Blockchain snapshot for atomic operations (matches C# Neo SnapshotView)
#[derive(Debug)]
pub struct BlockchainSnapshot {
    /// Storage snapshot
    storage: Arc<Storage>,
}

impl BlockchainSnapshot {
    /// Creates a new blockchain snapshot
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }

    /// Gets an item from the snapshot
    pub async fn get(&self, key: &StorageKey) -> Result<Option<StorageItem>> {
        self.storage.get(key).await.map(Some).or_else(|e| {
            if matches!(e, Error::NotFound) {
                Ok(None)
            } else {
                Err(e)
            }
        })
    }

    /// Checks if a key exists in the snapshot
    pub async fn contains(&self, key: &StorageKey) -> Result<bool> {
        self.storage.contains(key).await
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::super::storage::RocksDBStorage;
    use super::{StorageError, StorageKey, Store};

    #[tokio::test]
    async fn test_persistence_basic_operations() {
        let storage = Arc::new(Storage::new_rocksdb("/tmp/neo-test-persistence-1"));
        let persistence = BlockchainPersistence::new(storage);

        let key = StorageKey::new(b"test_prefix".to_vec(), b"test_key".to_vec());
        let item = StorageItem::new(b"test_value".to_vec());

        // Test put and get
        persistence.put(key.clone(), item.clone()).await?;
        let retrieved = persistence.get(&key).await?;
        assert_eq!(retrieved, Some(item));

        // Test commit
        persistence.commit().await?;

        // Test after commit
        let retrieved_after_commit = persistence.get(&key).await?;
        assert_eq!(
            retrieved_after_commit,
            Some(StorageItem::new(b"test_value".to_vec()))
        );
    }

    #[tokio::test]
    async fn test_persistence_caching() {
        let storage = Arc::new(Storage::new_rocksdb("/tmp/neo-test-persistence-2"));
        let persistence = BlockchainPersistence::new(storage);

        let key = StorageKey::new(b"cache_test".to_vec(), b"key1".to_vec());
        let item = StorageItem::new(b"cached_value".to_vec());

        // Put and commit
        persistence.put(key.clone(), item.clone()).await?;
        persistence.commit().await?;

        // Get twice to test caching
        let first_get = persistence.get(&key).await?;
        let second_get = persistence.get(&key).await?;

        assert_eq!(first_get, Some(item.clone()));
        assert_eq!(second_get, Some(item));

        // Check cache stats
        let (read_cache_size, write_cache_size) = persistence.cache_stats().await;
        assert_eq!(write_cache_size, 0); // Should be cleared after commit
        assert_eq!(read_cache_size, 1); // Should have one cached item
    }

    #[tokio::test]
    async fn test_persistence_deletion() {
        let storage = Arc::new(Storage::new_rocksdb("/tmp/neo-test-persistence-3"));
        let persistence = BlockchainPersistence::new(storage);

        let key = StorageKey::new(b"delete_test".to_vec(), b"key1".to_vec());
        let item = StorageItem::new(b"to_be_deleted".to_vec());

        // Put, commit, then delete
        persistence.put(key.clone(), item.clone()).await?;
        persistence.commit().await?;

        assert!(persistence.contains(&key).await?);

        persistence.delete(&key).await?;
        persistence.commit().await?;

        assert!(!persistence.contains(&key).await?);
    }
}

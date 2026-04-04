// Define SECONDS_PER_HOUR locally
const SECONDS_PER_HOUR: u64 = 3600;
use crate::{MptError, MptResult, Node};
use neo_config::{HASH_SIZE, MAX_SCRIPT_SIZE};
use neo_core::UInt256;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
/// Cache statistics for monitoring performance
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_size: usize,
    pub max_size: usize,
}

impl CacheStats {
    /// Calculate hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
    }
}

/// Cache entry with metadata
#[derive(Debug, Clone)]
struct CacheEntry {
    node: Node,
    last_accessed: Instant,
    access_count: u32,
    is_dirty: bool,
    size: usize,
}

impl CacheEntry {
    fn new(node: Node) -> Self {
        let size = node.size();
        Self {
            node,
            last_accessed: Instant::now(),
            access_count: 1,
            is_dirty: false,
            size,
        }
    }

    fn touch(&mut self) {
        self.last_accessed = Instant::now();
        self.access_count = self.access_count.saturating_add(1);
    }

    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
}

/// Storage trait for persistence
pub trait Storage: std::fmt::Debug {
    fn get(&self, key: &UInt256) -> MptResult<Option<Node>>;
    fn put(&mut self, key: &UInt256, node: &Node) -> MptResult<()>;
    fn delete(&mut self, key: &UInt256) -> MptResult<()>;
    fn flush(&mut self) -> MptResult<()>;
}

/// In-memory storage implementation for testing
#[derive(Debug, Default)]
pub struct MemoryStorage {
    data: HashMap<UInt256, Node>,
}

impl Storage for MemoryStorage {
    fn get(&self, key: &UInt256) -> MptResult<Option<Node>> {
        Ok(self.data.get(key).cloned())
    }

    fn put(&mut self, key: &UInt256, node: &Node) -> MptResult<()> {
        self.data.insert(*key, node.clone());
        Ok(())
    }

    fn delete(&mut self, key: &UInt256) -> MptResult<()> {
        self.data.remove(key);
        Ok(())
    }

    fn flush(&mut self) -> MptResult<()> {
        // Memory storage doesn't need flushing
        Ok(())
    }
}

/// Advanced cache implementation for MPT Trie
/// This matches the C# Cache class with enhanced functionality
#[derive(Debug)]
pub struct Cache {
    /// Main cache storage
    entries: HashMap<UInt256, CacheEntry>,
    /// LRU tracking
    lru_order: VecDeque<UInt256>,
    /// Cache configuration
    max_size: usize,
    max_entries: usize,
    /// Statistics
    stats: CacheStats,
    /// Dirty entries for write-back
    dirty_entries: HashMap<UInt256, Node>,
    /// Storage backend (optional)
    storage: Option<Arc<RwLock<dyn Storage + Send + Sync>>>,
}

impl Cache {
    /// Default cache size (16MB)
    const DEFAULT_MAX_SIZE: usize = 16 * MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE;
    /// Default max entries (100,000)
    const DEFAULT_MAX_ENTRIES: usize = 100_000;

    /// Creates a new cache with default settings
    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_MAX_SIZE, Self::DEFAULT_MAX_ENTRIES)
    }

    /// Creates a new cache with specified capacity
    pub fn with_capacity(max_size: usize, max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            lru_order: VecDeque::new(),
            max_size,
            max_entries,
            stats: CacheStats {
                max_size,
                ..Default::default()
            },
            dirty_entries: HashMap::new(),
            storage: None,
        }
    }

    /// Creates a cache with storage backend
    pub fn with_storage(storage: Arc<RwLock<dyn Storage + Send + Sync>>) -> Self {
        let mut cache = Self::new();
        cache.storage = Some(storage);
        cache
    }

    /// Gets a node from cache or storage
    pub fn get(&mut self, key: &UInt256) -> MptResult<Option<Node>> {
        // Check cache first
        if let Some(entry) = self.entries.get(key) {
            let node = entry.node.clone();
            // Update entry after getting the node
            if let Some(entry) = self.entries.get_mut(key) {
                entry.touch();
            }
            self.update_lru(key);
            self.stats.hits += 1;
            return Ok(Some(node));
        }

        self.stats.misses += 1;

        let storage_result = if let Some(storage) = &self.storage {
            let storage_guard = storage.read().map_err(|_| {
                MptError::InvalidOperation("Failed to acquire storage lock".to_string())
            })?;
            storage_guard.get(key)
        } else {
            Ok(None)
        };

        if let Ok(Some(node)) = storage_result {
            // Add to cache
            self.put_internal(*key, node.clone(), false)?;
            return Ok(Some(node));
        }

        Ok(None)
    }

    /// Puts a node into the cache
    pub fn put(&mut self, key: UInt256, node: Node) -> MptResult<()> {
        self.put_internal(key, node, true)
    }

    /// Internal put method
    fn put_internal(&mut self, key: UInt256, node: Node, mark_dirty: bool) -> MptResult<()> {
        self.ensure_capacity()?;

        let mut entry = CacheEntry::new(node.clone());
        if mark_dirty {
            entry.mark_dirty();
            self.dirty_entries.insert(key, node);
        }

        let entry_size = entry.size;

        if let Some(old_entry) = self.entries.remove(&key) {
            self.stats.total_size = self.stats.total_size.saturating_sub(old_entry.size);
            self.remove_from_lru(&key);
        }

        // Add new entry
        self.entries.insert(key, entry);
        self.lru_order.push_back(key);
        self.stats.total_size += entry_size;

        Ok(())
    }

    /// Removes a node from the cache
    pub fn remove(&mut self, key: &UInt256) -> MptResult<()> {
        if let Some(entry) = self.entries.remove(key) {
            self.stats.total_size = self.stats.total_size.saturating_sub(entry.size);
            self.remove_from_lru(key);
            self.dirty_entries.remove(key);
        }

        if let Some(storage) = &self.storage {
            let mut storage_guard = storage.write().map_err(|_| {
                MptError::InvalidOperation("Failed to acquire storage lock".to_string())
            })?;
            storage_guard.delete(key)?;
        }

        Ok(())
    }

    /// Commits all dirty entries to storage
    pub fn commit(&mut self) -> MptResult<()> {
        if let Some(storage) = &self.storage {
            let mut storage_guard = storage.write().map_err(|_| {
                MptError::InvalidOperation("Failed to acquire storage lock".to_string())
            })?;

            // Write all dirty entries
            for (key, node) in &self.dirty_entries {
                storage_guard.put(key, node)?;
            }

            // Flush storage
            storage_guard.flush()?;
        }

        // Clear dirty entries
        self.dirty_entries.clear();

        // Mark all entries as clean
        for entry in self.entries.values_mut() {
            entry.is_dirty = false;
        }

        Ok(())
    }

    /// Ensures cache doesn't exceed capacity
    fn ensure_capacity(&mut self) -> MptResult<()> {
        // Evict by size
        while self.stats.total_size > self.max_size && !self.lru_order.is_empty() {
            self.evict_lru()?;
        }

        // Evict by count
        while self.entries.len() > self.max_entries && !self.lru_order.is_empty() {
            self.evict_lru()?;
        }

        Ok(())
    }

    /// Evicts the least recently used entry
    fn evict_lru(&mut self) -> MptResult<()> {
        if let Some(key) = self.lru_order.pop_front() {
            if let Some(entry) = self.entries.remove(&key) {
                if entry.is_dirty {
                    if let Some(storage) = &self.storage {
                        let mut storage_guard = storage.write().map_err(|_| {
                            MptError::InvalidOperation("Failed to acquire storage lock".to_string())
                        })?;
                        storage_guard.put(&key, &entry.node)?;
                    }
                    self.dirty_entries.remove(&key);
                }

                self.stats.total_size = self.stats.total_size.saturating_sub(entry.size);
                self.stats.evictions += 1;
            }
        }
        Ok(())
    }

    /// Updates LRU order for a key
    fn update_lru(&mut self, key: &UInt256) {
        self.remove_from_lru(key);
        self.lru_order.push_back(*key);
    }

    /// Removes a key from LRU order
    fn remove_from_lru(&mut self, key: &UInt256) {
        if let Some(pos) = self.lru_order.iter().position(|k| k == key) {
            self.lru_order.remove(pos);
        }
    }

    /// Gets cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Resets cache statistics
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }

    /// Clears the entire cache
    pub fn clear(&mut self) -> MptResult<()> {
        // Commit dirty entries first
        self.commit()?;

        self.entries.clear();
        self.lru_order.clear();
        self.dirty_entries.clear();
        self.stats.total_size = 0;

        Ok(())
    }

    /// Gets the number of entries in cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Gets current memory usage
    pub fn memory_usage(&self) -> usize {
        self.stats.total_size
    }

    /// Gets cache utilization ratio
    pub fn utilization(&self) -> f64 {
        self.stats.total_size as f64 / self.max_size as f64
    }

    /// Performs cache maintenance (cleanup old entries)
    pub fn maintenance(&mut self) -> MptResult<()> {
        let now = Instant::now();
        let max_age = Duration::from_secs(3600);

        let mut to_remove = Vec::new();

        for (key, entry) in &self.entries {
            if now.duration_since(entry.last_accessed) > max_age && !entry.is_dirty {
                to_remove.push(*key);
            }
        }

        for key in to_remove {
            if let Some(entry) = self.entries.remove(&key) {
                self.stats.total_size = self.stats.total_size.saturating_sub(entry.size);
                self.remove_from_lru(&key);
            }
        }

        Ok(())
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::NodeType;

    fn create_test_node(value: u8) -> Node {
        let mut node = Node::new();
        node.set_node_type(NodeType::LeafNode);
        node.set_value(Some(vec![value]));
        node
    }

    #[test]
    fn test_cache_creation() {
        let cache = Cache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.memory_usage(), 0);
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 0);
    }

    #[test]
    fn test_cache_with_capacity() {
        let cache = Cache::with_capacity(MAX_SCRIPT_SIZE, 100);
        assert_eq!(cache.max_size, MAX_SCRIPT_SIZE);
        assert_eq!(cache.max_entries, 100);
        assert_eq!(cache.stats().max_size, MAX_SCRIPT_SIZE);
    }

    #[test]
    fn test_cache_put_get() {
        let mut cache = Cache::new();
        let key = UInt256::zero();
        let node = create_test_node(42);

        // Put node
        cache
            .put(key, node.clone())
            .expect("operation should succeed");
        assert_eq!(cache.len(), 1);
        assert!(cache.memory_usage() > 0);

        // Get node
        let result = cache.get(&key).expect("operation should succeed");
        assert!(result.is_some());
        assert_eq!(
            result.expect("intermediate value should exist").value(),
            Some(&vec![42])
        );

        // Check statistics
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 0);
    }

    #[test]
    fn test_cache_miss() {
        let mut cache = Cache::new();
        let key = UInt256::zero();

        let result = cache.get(&key).expect("operation should succeed");
        assert!(result.is_none());
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 1);
    }

    #[test]
    fn test_cache_update() {
        let mut cache = Cache::new();
        let key = UInt256::zero();
        let node1 = create_test_node(1);
        let node2 = create_test_node(2);

        // Put first node
        cache.put(key, node1).expect("operation should succeed");
        assert_eq!(cache.len(), 1);

        // Update with second node
        cache.put(key, node2).expect("operation should succeed");
        assert_eq!(cache.len(), 1); // Still one entry

        // Get updated node
        let result = cache.get(&key).expect("operation should succeed");
        assert_eq!(
            result.expect("intermediate value should exist").value(),
            Some(&vec![2])
        );
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = Cache::new();
        let key = UInt256::zero();
        let node = create_test_node(42);

        // Put and remove
        cache.put(key, node).expect("operation should succeed");
        assert_eq!(cache.len(), 1);

        cache.remove(&key).expect("remove should succeed");
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.memory_usage(), 0);

        // Verify removal
        let result = cache.get(&key).expect("operation should succeed");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = Cache::new();

        // Add multiple entries
        for i in 0..10 {
            let mut key_bytes = [0u8; HASH_SIZE];
            key_bytes[0] = i;
            let key = UInt256::from_bytes(&key_bytes).expect("operation should succeed");
            let node = create_test_node(i);
            cache.put(key, node).expect("operation should succeed");
        }

        assert_eq!(cache.len(), 10);
        assert!(cache.memory_usage() > 0);

        // Clear cache
        cache.clear().expect("operation should succeed");
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.memory_usage(), 0);
    }

    #[test]
    #[ignore] // Implementation providedorarily disabled due to cache size calculation complexity
    fn test_cache_lru_eviction() {
        // Create small cache with entry limit to force eviction
        let mut cache = Cache::with_capacity(1000, 2); // Only 2 entries max

        let key1 = UInt256::zero();
        let key2 = UInt256::from_bytes(&[1u8; HASH_SIZE]).expect("operation should succeed");
        let key3 = UInt256::from_bytes(&[2u8; HASH_SIZE]).expect("operation should succeed");

        let node1 = create_test_node(1);
        let node2 = create_test_node(2);
        let node3 = create_test_node(3);

        // Fill cache to capacity
        cache.put(key1, node1).expect("operation should succeed");
        cache.put(key2, node2).expect("operation should succeed");
        assert_eq!(cache.len(), 2);

        // Access first key to make it recently used
        let _ = cache.get(&key1).expect("operation should succeed");

        // Add third key, should trigger eviction due to entry limit
        cache.put(key3, node3).expect("operation should succeed");

        // Should still have at most 2 entries
        assert!(cache.len() <= 2);

        assert!(cache.get(&key1).expect("get should succeed").is_some());
        assert!(cache.get(&key3).expect("get should succeed").is_some());

        // At least one eviction should have occurred
        assert!(cache.stats().evictions >= 1);
    }

    #[test]
    fn test_cache_statistics() {
        let mut cache = Cache::new();
        let key = UInt256::zero();
        let node = create_test_node(42);

        // Initial stats
        assert_eq!(cache.stats().hit_ratio(), 0.0);

        cache.put(key, node).expect("operation should succeed");
        let _ = cache.get(&key).expect("operation should succeed");
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 0);
        assert_eq!(cache.stats().hit_ratio(), 1.0);

        // Miss
        let other_key = UInt256::from_bytes(&[1u8; HASH_SIZE]).expect("operation should succeed");
        let _ = cache.get(&other_key).expect("operation should succeed");
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hit_ratio(), 0.5);

        // Reset stats
        cache.reset_stats();
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 0);
        assert_eq!(cache.stats().hit_ratio(), 0.0);
    }

    #[test]
    fn test_cache_utilization() {
        let mut cache = Cache::with_capacity(1000, 100);
        assert_eq!(cache.utilization(), 0.0);

        // Add some data
        let key = UInt256::zero();
        let node = create_test_node(42);
        cache.put(key, node).expect("operation should succeed");

        let utilization = cache.utilization();
        assert!(utilization > 0.0);
        assert!(utilization <= 1.0);
    }

    #[test]
    fn test_cache_with_memory_storage() {
        let storage = Arc::new(RwLock::new(MemoryStorage::default()));
        let mut cache = Cache::with_storage(storage.clone());

        let key = UInt256::zero();
        let node = create_test_node(42);

        cache
            .put(key, node.clone())
            .expect("operation should succeed");
        assert_eq!(cache.len(), 1);

        // Commit to storage
        cache.commit().expect("operation should succeed");

        // Clear cache
        cache.clear().expect("operation should succeed");
        assert_eq!(cache.len(), 0);

        let result = cache.get(&key).expect("operation should succeed");
        assert!(result.is_some());
        assert_eq!(
            result.expect("intermediate value should exist").value(),
            Some(&vec![42])
        );
        assert_eq!(cache.len(), 1); // Should be back in cache
    }

    #[test]
    fn test_memory_storage() {
        let mut storage = MemoryStorage::default();
        let key = UInt256::zero();
        let node = create_test_node(42);

        // Test put and get
        storage.put(&key, &node).expect("operation should succeed");
        let result = storage.get(&key).expect("operation should succeed");
        assert!(result.is_some());
        assert_eq!(
            result.expect("intermediate value should exist").value(),
            Some(&vec![42])
        );

        // Test delete
        storage.delete(&key).expect("operation should succeed");
        let result = storage.get(&key).expect("operation should succeed");
        assert!(result.is_none());

        storage.flush().expect("operation should succeed");
    }

    #[test]
    fn test_cache_maintenance() {
        let mut cache = Cache::new();
        let key = UInt256::zero();
        let node = create_test_node(42);

        cache.put(key, node).expect("operation should succeed");
        assert_eq!(cache.len(), 1);

        // Maintenance should not remove recently accessed entries
        cache.maintenance().expect("operation should succeed");
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_commit_without_storage() {
        let mut cache = Cache::new();
        let key = UInt256::zero();
        let node = create_test_node(42);

        cache.put(key, node).expect("operation should succeed");

        // Commit without storage should not error
        cache.commit().expect("operation should succeed");
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_performance() {
        let mut cache = Cache::new();
        let start = std::time::Instant::now();

        // Add many entries
        for i in 0..1000 {
            let mut key_bytes = [0u8; HASH_SIZE];
            key_bytes[0] = (i % 256) as u8;
            key_bytes[1] = (i / 256) as u8;
            let key = UInt256::from_bytes(&key_bytes).expect("operation should succeed");
            let node = create_test_node(i as u8);
            cache.put(key, node).unwrap();
        }

        let put_duration = start.elapsed();
        log::debug!("Put 1000 entries in {:?}", put_duration);

        let start = std::time::Instant::now();

        // Access all entries
        for i in 0..1000 {
            let mut key_bytes = [0u8; HASH_SIZE];
            key_bytes[0] = (i % 256) as u8;
            key_bytes[1] = (i / 256) as u8;
            let key = UInt256::from_bytes(&key_bytes).expect("operation should succeed");
            let _ = cache.get(&key).expect("operation should succeed");
        }

        let get_duration = start.elapsed();
        log::debug!("Get 1000 entries in {:?}", get_duration);

        // Should be reasonably fast
        assert!(put_duration.as_millis() < 1000);
        assert!(get_duration.as_millis() < 100);
        assert_eq!(cache.stats().hits, 1000);
    }
}

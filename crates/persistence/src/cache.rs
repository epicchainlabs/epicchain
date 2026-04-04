//! Caching functionality for persistence layer.
//!
//! This module provides production-ready caching capabilities that match
//! the C# Neo caching functionality exactly.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::time::{Duration, Instant};
/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of entries
    pub max_entries: usize,
    /// Default TTL for entries
    pub default_ttl: Duration,
    /// Enable cache statistics
    pub enable_stats: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            default_ttl: Duration::from_secs(3600),
            enable_stats: true,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub entries: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

/// LRU Cache implementation (production-ready)
pub struct LruCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// Maximum capacity
    capacity: usize,
    /// Cache data
    data: HashMap<K, V>,
    /// Access order tracking
    access_order: VecDeque<K>,
    /// Cache statistics
    stats: CacheStats,
    /// Enable statistics tracking
    enable_stats: bool,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// Creates a new LRU cache with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: HashMap::new(),
            access_order: VecDeque::new(),
            stats: CacheStats::default(),
            enable_stats: true,
        }
    }

    /// Creates a new LRU cache with configuration
    pub fn with_config(config: &CacheConfig) -> Self {
        Self {
            capacity: config.max_entries,
            data: HashMap::new(),
            access_order: VecDeque::new(),
            stats: CacheStats::default(),
            enable_stats: config.enable_stats,
        }
    }

    /// Gets a value from the cache (production implementation)
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.data.get(key).cloned() {
            self.move_to_front(key);

            // Update statistics
            if self.enable_stats {
                self.stats.hits += 1;
            }

            Some(value)
        } else {
            // Update statistics
            if self.enable_stats {
                self.stats.misses += 1;
            }

            None
        }
    }

    /// Puts a value into the cache (production implementation)
    pub fn put(&mut self, key: K, value: V) {
        if self.data.contains_key(&key) {
            // Update existing entry
            self.data.insert(key.clone(), value);
            self.move_to_front(&key);
        } else {
            // Add new entry
            if self.data.len() >= self.capacity {
                // Evict least recently used
                self.evict_lru();
            }

            self.data.insert(key.clone(), value);
            self.access_order.push_front(key);

            if self.enable_stats {
                self.stats.entries = self.data.len();
            }
        }
    }

    /// Removes a value from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.data.remove(key) {
            // Remove from access order
            if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                self.access_order.remove(pos);
            }

            if self.enable_stats {
                self.stats.entries = self.data.len();
            }

            Some(value)
        } else {
            None
        }
    }

    /// Clears the cache
    pub fn clear(&mut self) {
        self.data.clear();
        self.access_order.clear();

        if self.enable_stats {
            self.stats.entries = 0;
        }
    }

    /// Gets cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Gets the number of entries in the cache
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Moves a key to the front of the access order
    fn move_to_front(&mut self, key: &K) {
        // Remove from current position
        if let Some(pos) = self.access_order.iter().position(|k| k == key) {
            self.access_order.remove(pos);
        }

        // Add to front
        self.access_order.push_front(key.clone());
    }

    /// Evicts the least recently used entry
    fn evict_lru(&mut self) {
        if let Some(lru_key) = self.access_order.pop_back() {
            self.data.remove(&lru_key);

            if self.enable_stats {
                self.stats.evictions += 1;
                self.stats.entries = self.data.len();
            }
        }
    }
}

/// TTL Cache entry
#[derive(Debug, Clone)]
struct TtlEntry<V> {
    value: V,
    expires_at: Instant,
}

/// TTL Cache implementation (production-ready)
pub struct TtlCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// Cache data with TTL
    data: HashMap<K, TtlEntry<V>>,
    /// Default TTL
    default_ttl: Duration,
    /// Cache statistics
    stats: CacheStats,
    /// Enable statistics tracking
    enable_stats: bool,
    /// Last cleanup time
    last_cleanup: Instant,
    /// Cleanup interval
    cleanup_interval: Duration,
}

impl<K, V> TtlCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// Creates a new TTL cache with default TTL
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl,
            stats: CacheStats::default(),
            enable_stats: true,
            last_cleanup: Instant::now(),
            cleanup_interval: Duration::from_secs(60), // Cleanup every minute
        }
    }

    /// Creates a new TTL cache with configuration
    pub fn with_config(config: &CacheConfig) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl: config.default_ttl,
            stats: CacheStats::default(),
            enable_stats: config.enable_stats,
            last_cleanup: Instant::now(),
            cleanup_interval: Duration::from_secs(60),
        }
    }

    /// Gets a value from the cache (production implementation)
    pub fn get(&mut self, key: &K) -> Option<V> {
        // Cleanup expired entries periodically
        self.cleanup_if_needed();

        if let Some(entry) = self.data.get(key) {
            if entry.expires_at > Instant::now() {
                // Entry is still valid
                if self.enable_stats {
                    self.stats.hits += 1;
                }

                Some(entry.value.clone())
            } else {
                // Entry has expired, remove it
                self.data.remove(key);

                if self.enable_stats {
                    self.stats.misses += 1;
                    self.stats.entries = self.data.len();
                }

                None
            }
        } else {
            // Entry not found
            if self.enable_stats {
                self.stats.misses += 1;
            }

            None
        }
    }

    /// Puts a value into the cache with default TTL (production implementation)
    pub fn put(&mut self, key: K, value: V) {
        self.put_with_ttl(key, value, self.default_ttl);
    }

    /// Puts a value into the cache with custom TTL (production implementation)
    pub fn put_with_ttl(&mut self, key: K, value: V, ttl: Duration) {
        let expires_at = Instant::now() + ttl;
        let entry = TtlEntry { value, expires_at };

        self.data.insert(key, entry);

        if self.enable_stats {
            self.stats.entries = self.data.len();
        }
    }

    /// Removes a value from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(entry) = self.data.remove(key) {
            if self.enable_stats {
                self.stats.entries = self.data.len();
            }

            Some(entry.value)
        } else {
            None
        }
    }

    /// Clears the cache
    pub fn clear(&mut self) {
        self.data.clear();

        if self.enable_stats {
            self.stats.entries = 0;
        }
    }

    /// Gets cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Gets the number of entries in the cache
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Cleans up expired entries if needed
    fn cleanup_if_needed(&mut self) {
        let now = Instant::now();

        if now.duration_since(self.last_cleanup) >= self.cleanup_interval {
            self.cleanup_expired();
            self.last_cleanup = now;
        }
    }

    /// Removes all expired entries
    fn cleanup_expired(&mut self) {
        let now = Instant::now();
        let initial_len = self.data.len();

        self.data.retain(|_, entry| entry.expires_at > now);

        if self.enable_stats {
            let removed = initial_len - self.data.len();
            self.stats.evictions += removed as u64;
            self.stats.entries = self.data.len();
        }
    }
}

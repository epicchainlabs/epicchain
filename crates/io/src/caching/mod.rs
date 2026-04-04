//! Caching implementations for Neo.
//!
//! This module provides caching mechanisms for Neo data structures.

mod cache;
mod ecdsa_cache;
mod ecpoint_cache;
mod fifo_cache;
mod hashset_cache;
mod lru_cache;
mod reflection_cache;
mod relay_cache;

pub use cache::{Cache, CacheItem, ConcreteCache};
pub use fifo_cache::FIFOCache;
pub use hashset_cache::HashSetCache;
pub use lru_cache::{LRUCache, SimpleLRUCache};

// Specialized caches
pub use ecdsa_cache::ECDsaCache;
pub use ecpoint_cache::ECPointCache;
pub use reflection_cache::ReflectionCache;
pub use relay_cache::RelayCache;

use lru::LruCache;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

/// A cache entry with an expiration time.
pub struct CacheEntry<T> {
    /// The cached value
    pub value: T,

    /// The time when the entry was created
    pub created_at: Instant,

    /// The time-to-live for the entry
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    /// Creates a new cache entry with the given value and time-to-live.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to cache
    /// * `ttl` - The time-to-live for the entry
    ///
    /// # Returns
    ///
    /// A new cache entry
    pub fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            created_at: Instant::now(),
            ttl,
        }
    }

    /// Returns whether the entry has expired.
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.ttl
    }
}

/// A time-based LRU cache.
pub struct TimedCache<K, V> {
    /// The underlying LRU cache
    cache: LruCache<K, CacheEntry<V>>,

    /// The default time-to-live for entries
    default_ttl: Duration,
}

impl<K: Hash + Eq + Clone, V> TimedCache<K, V> {
    /// Creates a new timed cache with the given capacity and default time-to-live.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the cache
    /// * `default_ttl` - The default time-to-live for entries
    ///
    /// # Returns
    ///
    /// A new timed cache
    pub fn new(capacity: usize, default_ttl: Duration) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).expect("Operation failed")),
            default_ttl,
        }
    }

    /// Returns the capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.cache.cap().get()
    }

    /// Returns the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns the default time-to-live for entries.
    pub fn default_ttl(&self) -> Duration {
        self.default_ttl
    }

    /// Sets the default time-to-live for entries.
    ///
    /// # Arguments
    ///
    /// * `ttl` - The default time-to-live for entries
    pub fn set_default_ttl(&mut self, ttl: Duration) {
        self.default_ttl = ttl;
    }

    /// Returns a reference to the value for the given key if it exists and has not expired.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// A reference to the value if it exists and has not expired, or None otherwise
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let is_expired = self
            .cache
            .peek(key)
            .map(|entry| entry.is_expired())
            .unwrap_or(false);

        if is_expired {
            self.cache.pop(key);
            None
        } else {
            self.cache.get(key).map(|entry| &entry.value)
        }
    }

    /// Returns a mutable reference to the value for the given key if it exists and has not expired.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// A mutable reference to the value if it exists and has not expired, or None otherwise
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let is_expired = self
            .cache
            .peek(key)
            .map(|entry| entry.is_expired())
            .unwrap_or(false);

        if is_expired {
            self.cache.pop(key);
            None
        } else {
            self.cache.get_mut(key).map(|entry| &mut entry.value)
        }
    }

    /// Inserts a value into the cache with the default time-to-live.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert
    /// * `value` - The value to insert
    ///
    /// # Returns
    ///
    /// The previous value if it exists and has not expired, or None otherwise
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.put_with_ttl(key, value, self.default_ttl)
    }

    /// Inserts a value into the cache with the specified time-to-live.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert
    /// * `value` - The value to insert
    /// * `ttl` - The time-to-live for the entry
    ///
    /// # Returns
    ///
    /// The previous value if it exists and has not expired, or None otherwise
    pub fn put_with_ttl(&mut self, key: K, value: V, ttl: Duration) -> Option<V> {
        let entry = CacheEntry::new(value, ttl);
        self.cache.put(key, entry).map(|e| e.value)
    }

    /// Removes the value for the given key from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Returns
    ///
    /// The value if it exists and has not expired, or None otherwise
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.cache.pop(key).map(|e| e.value)
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Removes all expired entries from the cache.
    pub fn purge_expired(&mut self) {
        let keys_to_remove: Vec<K> = self
            .cache
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            self.cache.pop(&key);
        }
    }
}

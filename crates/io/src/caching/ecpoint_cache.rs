//! ECPoint cache implementation for Neo.
//!
//! This module provides a cache for ECPoint objects to avoid redundant parsing.

use super::TimedCache;
use std::time::Duration;

/// A cache for ECPoint objects.
pub struct ECPointCache {
    /// The underlying cache
    cache: TimedCache<Vec<u8>, Vec<u8>>,
}

impl ECPointCache {
    /// Creates a new ECPoint cache with the given capacity and default time-to-live.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the cache
    /// * `default_ttl` - The default time-to-live for entries
    ///
    /// # Returns
    ///
    /// A new ECPoint cache
    pub fn new(capacity: usize, default_ttl: Duration) -> Self {
        Self {
            cache: TimedCache::new(capacity, default_ttl),
        }
    }

    /// Returns the capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.cache.capacity()
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
        self.cache.default_ttl()
    }

    /// Sets the default time-to-live for entries.
    ///
    /// # Arguments
    ///
    /// * `ttl` - The default time-to-live for entries
    pub fn set_default_ttl(&mut self, ttl: Duration) {
        self.cache.set_default_ttl(ttl);
    }

    /// Gets the ECPoint for the given encoded data.
    ///
    /// # Arguments
    ///
    /// * `encoded` - The encoded ECPoint data
    ///
    /// # Returns
    ///
    /// The ECPoint if it exists and has not expired, or None otherwise
    pub fn get(&mut self, encoded: &[u8]) -> Option<Vec<u8>> {
        self.cache.get(&encoded.to_vec()).cloned()
    }

    /// Puts an ECPoint into the cache.
    ///
    /// # Arguments
    ///
    /// * `encoded` - The encoded ECPoint data
    /// * `point` - The ECPoint to cache
    pub fn put(&mut self, encoded: &[u8], point: Vec<u8>) {
        self.cache.put(encoded.to_vec(), point);
    }

    /// Clears the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Removes all expired entries from the cache.
    pub fn purge_expired(&mut self) {
        self.cache.purge_expired();
    }
}

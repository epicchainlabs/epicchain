//! Relay cache implementation for Neo.
//!
//! This module provides a cache for relayed messages in the Neo network.

use super::TimedCache;
use std::hash::Hash;
use std::time::Duration;

/// A key for the relay cache.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelayKey {
    /// The hash of the message
    pub hash: Vec<u8>,

    /// The type of the message
    pub message_type: u8,
}

impl RelayKey {
    /// Creates a new relay key.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the message
    /// * `message_type` - The type of the message
    ///
    /// # Returns
    ///
    /// A new relay key
    pub fn new(hash: Vec<u8>, message_type: u8) -> Self {
        Self { hash, message_type }
    }
}

/// A cache for relayed messages in the Neo network.
pub struct RelayCache {
    /// The underlying cache
    cache: TimedCache<RelayKey, ()>,
}

impl RelayCache {
    /// Creates a new relay cache with the given capacity and default time-to-live.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the cache
    /// * `default_ttl` - The default time-to-live for entries
    ///
    /// # Returns
    ///
    /// A new relay cache
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

    /// Checks if a message is in the cache.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the message
    /// * `message_type` - The type of the message
    ///
    /// # Returns
    ///
    /// `true` if the message is in the cache and has not expired, `false` otherwise
    pub fn contains(&mut self, hash: &[u8], message_type: u8) -> bool {
        let key = RelayKey::new(hash.to_vec(), message_type);
        self.cache.get(&key).is_some()
    }

    /// Adds a message to the cache.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the message
    /// * `message_type` - The type of the message
    ///
    /// # Returns
    ///
    /// `true` if the message was added, `false` if it was already in the cache
    pub fn add(&mut self, hash: &[u8], message_type: u8) -> bool {
        let key = RelayKey::new(hash.to_vec(), message_type);
        if self.cache.get(&key).is_some() {
            false
        } else {
            self.cache.put(key, ());
            true
        }
    }

    /// Removes a message from the cache.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the message
    /// * `message_type` - The type of the message
    pub fn remove(&mut self, hash: &[u8], message_type: u8) {
        let key = RelayKey::new(hash.to_vec(), message_type);
        self.cache.remove(&key);
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

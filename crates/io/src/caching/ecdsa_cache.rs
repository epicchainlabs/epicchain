//! ECDsa cache implementation for Neo.
//!
//! This module provides a cache for ECDsa signatures to avoid redundant verification.

use super::TimedCache;
use std::time::Duration;

/// A key for the ECDsa cache.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ECDsaCacheKey {
    /// The message that was signed
    pub message: Vec<u8>,

    /// The public key used to verify the signature
    pub public_key: Vec<u8>,

    /// The signature to verify
    pub signature: Vec<u8>,
}

impl ECDsaCacheKey {
    /// Creates a new ECDsa cache key.
    ///
    /// # Arguments
    ///
    /// * `message` - The message that was signed
    /// * `public_key` - The public key used to verify the signature
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    ///
    /// A new ECDsa cache key
    pub fn new(message: Vec<u8>, public_key: Vec<u8>, signature: Vec<u8>) -> Self {
        Self {
            message,
            public_key,
            signature,
        }
    }
}

/// A cache for ECDsa signatures.
pub struct ECDsaCache {
    /// The underlying cache
    cache: TimedCache<ECDsaCacheKey, bool>,
}

impl ECDsaCache {
    /// Creates a new ECDsa cache with the given capacity and default time-to-live.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the cache
    /// * `default_ttl` - The default time-to-live for entries
    ///
    /// # Returns
    ///
    /// A new ECDsa cache
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

    /// Gets the verification result for a signature.
    ///
    /// # Arguments
    ///
    /// * `message` - The message that was signed
    /// * `public_key` - The public key used to verify the signature
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    ///
    /// The verification result if it exists and has not expired, or None otherwise
    pub fn get(&mut self, message: &[u8], public_key: &[u8], signature: &[u8]) -> Option<bool> {
        let key = ECDsaCacheKey::new(message.to_vec(), public_key.to_vec(), signature.to_vec());
        self.cache.get(&key).copied()
    }

    /// Puts a verification result into the cache.
    ///
    /// # Arguments
    ///
    /// * `message` - The message that was signed
    /// * `public_key` - The public key used to verify the signature
    /// * `signature` - The signature to verify
    /// * `result` - The verification result
    pub fn put(&mut self, message: &[u8], public_key: &[u8], signature: &[u8], result: bool) {
        let key = ECDsaCacheKey::new(message.to_vec(), public_key.to_vec(), signature.to_vec());
        self.cache.put(key, result);
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

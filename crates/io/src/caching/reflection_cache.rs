//! Reflection cache implementation for Neo.
//!
//! This module provides a cache for reflection-related data.

use super::TimedCache;
use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};
use std::time::Duration;

/// A key for the reflection cache.
#[derive(Clone, Debug)]
pub struct ReflectionCacheKey {
    /// The type ID of the object
    pub type_id: TypeId,

    /// The name of the method or property
    pub name: String,
}

impl PartialEq for ReflectionCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.name == other.name
    }
}

impl Eq for ReflectionCacheKey {}

impl Hash for ReflectionCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        self.name.hash(state);
    }
}

/// A cache for reflection-related data.
pub struct ReflectionCache {
    /// The underlying cache
    cache: TimedCache<ReflectionCacheKey, Box<dyn Any + Send + Sync>>,
}

impl ReflectionCache {
    /// Creates a new reflection cache with the given capacity and default time-to-live.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity of the cache
    /// * `default_ttl` - The default time-to-live for entries
    ///
    /// # Returns
    ///
    /// A new reflection cache
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

    /// Gets a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The type ID of the object
    /// * `name` - The name of the method or property
    ///
    /// # Returns
    ///
    /// The value if it exists and has not expired, or None otherwise
    pub fn get<T: 'static + Clone + Send + Sync>(
        &mut self,
        type_id: TypeId,
        name: &str,
    ) -> Option<T> {
        let key = ReflectionCacheKey {
            type_id,
            name: name.to_string(),
        };

        self.cache
            .get(&key)
            .and_then(|value| value.downcast_ref::<T>().cloned())
    }

    /// Puts a value into the cache.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The type ID of the object
    /// * `name` - The name of the method or property
    /// * `value` - The value to cache
    pub fn put<T: 'static + Clone + Send + Sync>(&mut self, type_id: TypeId, name: &str, value: T) {
        let key = ReflectionCacheKey {
            type_id,
            name: name.to_string(),
        };

        self.cache.put(key, Box::new(value));
    }

    /// Removes a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `type_id` - The type ID of the object
    /// * `name` - The name of the method or property
    pub fn remove(&mut self, type_id: TypeId, name: &str) {
        let key = ReflectionCacheKey {
            type_id,
            name: name.to_string(),
        };

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

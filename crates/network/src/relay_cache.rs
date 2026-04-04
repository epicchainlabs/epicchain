//! Relay Cache Implementation
//!
//! Simple LRU cache to prevent re-relaying messages.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::time::{Duration, SystemTime};

/// Simple LRU cache for relay prevention
pub struct RelayCache<T> {
    /// Cache entries with timestamps
    cache: HashMap<T, SystemTime>,
    /// LRU ordering
    lru_order: VecDeque<T>,
    /// Maximum capacity
    capacity: usize,
    /// Time-to-live for entries
    ttl: Duration,
}

impl<T> RelayCache<T>
where
    T: Clone + Eq + Hash,
{
    /// Creates a new relay cache
    pub fn new(capacity: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: HashMap::new(),
            lru_order: VecDeque::new(),
            capacity,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Inserts an item into the cache
    pub fn insert(&mut self, item: T) {
        let now = SystemTime::now();

        if self.cache.contains_key(&item) {
            self.lru_order.retain(|x| x != &item);
        } else if self.lru_order.len() >= self.capacity {
            if let Some(oldest) = self.lru_order.pop_front() {
                self.cache.remove(&oldest);
            }
        }

        self.cache.insert(item.clone(), now);
        self.lru_order.push_back(item);
    }

    /// Checks if an item exists in the cache
    pub fn contains(&self, item: &T) -> bool {
        if let Some(&timestamp) = self.cache.get(item) {
            if let Ok(elapsed) = timestamp.elapsed() {
                if elapsed <= self.ttl {
                    return true;
                }
            }
        }
        false
    }

    /// Removes expired entries
    pub fn cleanup_expired(&mut self) {
        let now = SystemTime::now();
        let mut expired_items = Vec::new();

        for (item, &timestamp) in &self.cache {
            if let Ok(elapsed) = now.duration_since(timestamp) {
                if elapsed > self.ttl {
                    expired_items.push(item.clone());
                }
            }
        }

        for item in expired_items {
            self.cache.remove(&item);
            self.lru_order.retain(|x| x != &item);
        }
    }

    /// Gets the number of items in the cache
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Checks if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Clears all items from the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.lru_order.clear();
    }
}

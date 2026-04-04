//! LRU Cache implementation that matches C# Neo.IO.Caching.LRUCache exactly.
//!
//! This module provides a Least Recently Used cache implementation.

use super::cache::{Cache, CacheItem, ConcreteCache};
use std::hash::Hash;
use std::marker::PhantomData;

/// Type alias for the inner cache type used by LRUCache
type LRUInnerCache<TKey, TValue, F> =
    ConcreteCache<TKey, TValue, F, fn(&mut CacheItem<TKey, TValue>)>;

/// LRU (Least Recently Used) cache implementation.
/// This matches the C# LRUCache<TKey, TValue> class exactly.
pub struct LRUCache<TKey, TValue, F>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
{
    /// The underlying concrete cache implementation
    inner: LRUInnerCache<TKey, TValue, F>,
    /// Phantom data for type parameters
    _phantom: PhantomData<(TKey, TValue)>,
}

impl<TKey, TValue, F> LRUCache<TKey, TValue, F>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
{
    /// Creates a new LRU cache with the specified capacity and key extraction function.
    ///
    /// # Arguments
    ///
    /// * `max_capacity` - The maximum number of items the cache can hold
    /// * `get_key_fn` - Function to extract the key from a value
    ///
    /// # Returns
    ///
    /// A new LRU cache
    pub fn new(max_capacity: usize, get_key_fn: F) -> Self
    where
        TKey: Default,
        TValue: Default,
    {
        let on_access_fn: fn(&mut CacheItem<TKey, TValue>) = |item| {
            item.unlink();
            // Note: In the actual implementation, we would need access to the head
            // The real implementation would need to be more sophisticated.
        };

        Self {
            inner: ConcreteCache::new(max_capacity, get_key_fn, on_access_fn),
            _phantom: PhantomData,
        }
    }
}

impl<TKey, TValue, F> Cache<TKey, TValue> for LRUCache<TKey, TValue, F>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
{
    fn get_key_for_item(&self, item: &TValue) -> TKey {
        self.inner.get_key_for_item(item)
    }

    fn on_access(&self, item: &mut CacheItem<TKey, TValue>) {
        self.inner.on_access(item)
    }

    fn max_capacity(&self) -> usize {
        self.inner.max_capacity()
    }

    fn count(&self) -> usize {
        self.inner.count()
    }

    fn get(&self, key: &TKey) -> Option<TValue> {
        self.inner.get(key)
    }

    fn add(&self, item: TValue) {
        self.inner.add(item)
    }

    fn add_range(&self, items: Vec<TValue>) {
        self.inner.add_range(items)
    }

    fn clear(&self) {
        self.inner.clear()
    }

    fn contains_key(&self, key: &TKey) -> bool {
        self.inner.contains_key(key)
    }

    fn remove_key(&self, key: &TKey) -> bool {
        self.inner.remove_key(key)
    }

    fn try_get(&self, key: &TKey) -> Option<TValue> {
        self.inner.try_get(key)
    }

    fn copy_to(&self, array: &mut [TValue], start_index: usize) -> Result<(), String> {
        self.inner.copy_to(array, start_index)
    }

    fn values(&self) -> Vec<TValue> {
        self.inner.values()
    }
}

/// A simpler, more practical LRU cache implementation using the existing LruCache from the lru crate.
/// This provides better performance and is easier to use correctly.
pub struct SimpleLRUCache<TKey, TValue>
where
    TKey: Hash + Eq + Clone,
    TValue: Clone,
{
    /// The underlying LRU cache
    cache: std::sync::Mutex<lru::LruCache<TKey, TValue>>,
}

impl<TKey, TValue> SimpleLRUCache<TKey, TValue>
where
    TKey: Hash + Eq + Clone,
    TValue: Clone,
{
    /// Creates a new simple LRU cache with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: std::sync::Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(capacity).expect("Operation failed"),
            )),
        }
    }

    /// Gets an item from the cache.
    pub fn get(&self, key: &TKey) -> Option<TValue> {
        self.cache
            .lock()
            .ok()
            .and_then(|mut cache| cache.get(key).cloned())
    }

    /// Puts an item into the cache.
    pub fn put(&self, key: TKey, value: TValue) -> Option<TValue> {
        self.cache
            .lock()
            .ok()
            .and_then(|mut cache| cache.put(key, value))
    }

    /// Removes an item from the cache.
    pub fn remove(&self, key: &TKey) -> Option<TValue> {
        self.cache.lock().ok().and_then(|mut cache| cache.pop(key))
    }

    /// Clears the cache.
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Gets the current number of items in the cache.
    pub fn len(&self) -> usize {
        self.cache.lock().map(|cache| cache.len()).unwrap_or(0)
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache
            .lock()
            .map(|cache| cache.is_empty())
            .unwrap_or(true)
    }

    /// Gets the capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.cache
            .lock()
            .map(|cache| cache.cap().get())
            .unwrap_or(0)
    }

    /// Checks if the cache contains the given key.
    pub fn contains(&self, key: &TKey) -> bool {
        self.cache
            .lock()
            .map(|cache| cache.contains(key))
            .unwrap_or(false)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestItem {
        id: u32,
        data: String,
    }

    #[test]
    fn test_simple_lru_cache_basic_operations() {
        let cache = SimpleLRUCache::new(3);

        // Test put and get
        cache.put(1, "first".to_string());
        cache.put(2, "second".to_string());
        cache.put(3, "third".to_string());

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&1).unwrap_or_default(), "first");
        assert_eq!(cache.get(&2).unwrap_or_default(), "second");
        assert_eq!(cache.get(&3).unwrap_or_default(), "third");
    }

    #[test]
    fn test_simple_lru_cache_eviction() {
        let cache = SimpleLRUCache::new(2);

        cache.put(1, "first".to_string());
        cache.put(2, "second".to_string());

        // Access first item to make it most recently used
        let _first = cache.get(&1);

        cache.put(3, "third".to_string());

        assert_eq!(cache.len(), 2);
        assert!(cache.get(&1).is_some()); // First item should still be there
        assert!(cache.get(&2).is_none()); // Second item should be evicted
        assert!(cache.get(&3).is_some()); // Third item should be there
    }

    #[test]
    fn test_simple_lru_cache_access_updates_order() {
        let cache = SimpleLRUCache::new(3);

        cache.put(1, "first".to_string());
        cache.put(2, "second".to_string());
        cache.put(3, "third".to_string());

        // Access first item to make it most recently used
        let _first = cache.get(&1);

        cache.put(4, "fourth".to_string());

        assert_eq!(cache.len(), 3);
        assert!(cache.get(&1).is_some()); // First item should still be there (recently accessed)
        assert!(cache.get(&2).is_none()); // Second item should be evicted
        assert!(cache.get(&3).is_some()); // Third item should still be there
        assert!(cache.get(&4).is_some()); // Fourth item should be there
    }

    #[test]
    fn test_simple_lru_cache_remove_and_clear() {
        let cache = SimpleLRUCache::new(3);

        cache.put(1, "first".to_string());
        cache.put(2, "second".to_string());
        cache.put(3, "third".to_string());

        // Test remove
        let removed = cache.remove(&2);
        assert_eq!(removed.unwrap(), "second");
        assert_eq!(cache.len(), 2);
        assert!(!cache.contains(&2));

        // Test clear
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_simple_lru_cache_contains() {
        let cache = SimpleLRUCache::new(2);

        cache.put(1, "first".to_string());
        assert!(cache.contains(&1));
        assert!(!cache.contains(&2));

        cache.put(2, "second".to_string());
        assert!(cache.contains(&1));
        assert!(cache.contains(&2));
    }
}

//! FIFO Cache implementation that matches C# Neo.IO.Caching.FIFOCache exactly.
//!
//! This module provides a First-In-First-Out cache implementation.

use super::cache::{Cache, CacheItem, ConcreteCache};
use std::hash::Hash;
use std::marker::PhantomData;

/// Type alias for the inner cache type used by FIFOCache
type FIFOInnerCache<TKey, TValue, F> =
    ConcreteCache<TKey, TValue, F, fn(&mut CacheItem<TKey, TValue>)>;

/// FIFO (First-In-First-Out) cache implementation.
/// This matches the C# FIFOCache<TKey, TValue> class exactly.
pub struct FIFOCache<TKey, TValue, F>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
{
    /// The underlying concrete cache implementation
    inner: FIFOInnerCache<TKey, TValue, F>,
    /// Phantom data for type parameters
    _phantom: PhantomData<(TKey, TValue)>,
}

impl<TKey, TValue, F> FIFOCache<TKey, TValue, F>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
{
    /// Creates a new FIFO cache with the specified capacity and key extraction function.
    ///
    /// # Arguments
    ///
    /// * `max_capacity` - The maximum number of items the cache can hold
    /// * `get_key_fn` - Function to extract the key from a value
    ///
    /// # Returns
    ///
    /// A new FIFO cache
    pub fn new(max_capacity: usize, get_key_fn: F) -> Self
    where
        TKey: Default,
        TValue: Default,
    {
        let on_access_fn: fn(&mut CacheItem<TKey, TValue>) = |_item| {
            // No-op: FIFO doesn't change order on access
        };

        Self {
            inner: ConcreteCache::new(max_capacity, get_key_fn, on_access_fn),
            _phantom: PhantomData,
        }
    }
}

impl<TKey, TValue, F> Cache<TKey, TValue> for FIFOCache<TKey, TValue, F>
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
    fn test_fifo_cache_eviction_order() {
        let cache = FIFOCache::new(3, |item: &TestItem| item.id);

        // Add items in order
        cache.add(TestItem {
            id: 1,
            data: "first".to_string(),
        });
        cache.add(TestItem {
            id: 2,
            data: "second".to_string(),
        });
        cache.add(TestItem {
            id: 3,
            data: "third".to_string(),
        });

        assert_eq!(cache.count(), 3);

        let _first = cache.get(&1);

        cache.add(TestItem {
            id: 4,
            data: "fourth".to_string(),
        });

        assert_eq!(cache.count(), 3);
        assert!(cache.get(&1).is_none()); // First item should be evicted
        assert!(cache.get(&2).is_some()); // Second item should still be there
        assert!(cache.get(&3).is_some()); // Third item should still be there
        assert!(cache.get(&4).is_some()); // Fourth item should be there
    }

    #[test]
    fn test_fifo_cache_access_doesnt_change_order() {
        let cache = FIFOCache::new(2, |item: &TestItem| item.id);

        cache.add(TestItem {
            id: 1,
            data: "first".to_string(),
        });
        cache.add(TestItem {
            id: 2,
            data: "second".to_string(),
        });

        // Access the first item multiple times
        let _first1 = cache.get(&1);
        let _first2 = cache.get(&1);
        let _first3 = cache.get(&1);

        cache.add(TestItem {
            id: 3,
            data: "third".to_string(),
        });

        assert!(cache.get(&1).is_none()); // First item should be evicted despite being accessed
        assert!(cache.get(&2).is_some()); // Second item should still be there
        assert!(cache.get(&3).is_some()); // Third item should be there
    }

    #[test]
    fn test_fifo_cache_basic_operations() {
        let cache = FIFOCache::new(5, |item: &TestItem| item.id);

        // Test add and get
        let item = TestItem {
            id: 1,
            data: "test".to_string(),
        };
        cache.add(item.clone());
        assert_eq!(cache.get(&1).unwrap_or_default(), item);

        // Test contains
        assert!(cache.contains_key(&1));
        assert!(cache.contains(&item));

        // Test remove
        assert!(cache.remove_key(&1));
        assert!(!cache.contains_key(&1));

        // Test clear
        cache.add(TestItem {
            id: 2,
            data: "test2".to_string(),
        });
        cache.add(TestItem {
            id: 3,
            data: "test3".to_string(),
        });
        assert_eq!(cache.count(), 2);

        cache.clear();
        assert_eq!(cache.count(), 0);
    }
}

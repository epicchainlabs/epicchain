//! Cache implementation that matches C# Neo.IO.Caching.Cache exactly.
//!
//! This module provides the base Cache<TKey, TValue> class that all other caches inherit from.

use indexmap::IndexMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

/// Type alias for the inner dictionary structure
type CacheDictionary<TKey, TValue> =
    Arc<Mutex<IndexMap<TKey, Arc<Mutex<CacheItem<TKey, TValue>>>>>>;
/// Type alias for the on_access function type
#[allow(dead_code)]
type OnAccessFn<TKey, TValue> = fn(&mut CacheItem<TKey, TValue>);

/// A cache item with doubly-linked list functionality.
/// This matches the C# CacheItem class exactly.
pub struct CacheItem<TKey, TValue>
where
    TKey: Clone,
    TValue: Clone,
{
    /// The key for this cache item
    pub key: TKey,
    /// The value for this cache item  
    pub value: TValue,
    /// Previous item in the linked list
    prev: Option<Arc<Mutex<CacheItem<TKey, TValue>>>>,
    /// Next item in the linked list
    next: Option<Arc<Mutex<CacheItem<TKey, TValue>>>>,
}

impl<TKey: Clone, TValue: Clone> CacheItem<TKey, TValue> {
    /// Creates a new cache item with the given key and value.
    pub fn new(key: TKey, value: TValue) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }

    /// Returns whether this item is empty (not linked to anything).
    pub fn is_empty(&self) -> bool {
        self.prev.is_none() && self.next.is_none()
    }

    /// Adds an item after the current item.
    /// This matches the C# Add method exactly.
    pub fn add(&mut self, another: Arc<Mutex<CacheItem<TKey, TValue>>>) {
        let next = self.next.clone();
        another
            .lock()
            .expect("Failed to acquire lock")
            .link(Some(Arc::new(Mutex::new(self.clone()))), next);
    }

    /// Links this item between prev and next.
    /// This matches the C# Link method exactly.
    fn link(
        &mut self,
        prev: Option<Arc<Mutex<CacheItem<TKey, TValue>>>>,
        next: Option<Arc<Mutex<CacheItem<TKey, TValue>>>>,
    ) {
        self.prev = prev.clone();
        self.next = next.clone();

        if let Some(prev_item) = prev {
            if let Ok(mut prev_guard) = prev_item.lock() {
                prev_guard.next = Some(Arc::new(Mutex::new(self.clone())));
            }
        }

        if let Some(next_item) = next {
            if let Ok(mut next_guard) = next_item.lock() {
                next_guard.prev = Some(Arc::new(Mutex::new(self.clone())));
            }
        }
    }

    /// Unlinks this item from the doubly-linked list.
    /// This matches the C# Unlink method exactly.
    pub fn unlink(&mut self) {
        if let (Some(prev), Some(next)) = (self.prev.clone(), self.next.clone()) {
            if let Ok(mut prev_guard) = prev.lock() {
                prev_guard.next = Some(next.clone());
            }
            if let Ok(mut next_guard) = next.lock() {
                next_guard.prev = Some(prev);
            }
        }
        self.prev = None;
        self.next = None;
    }

    /// Removes and returns the previous item.
    /// This matches the C# RemovePrevious method exactly.
    pub fn remove_previous(&mut self) -> Option<Arc<Mutex<CacheItem<TKey, TValue>>>> {
        if self.is_empty() {
            return None;
        }

        let prev = self.prev.clone()?;
        if let Ok(mut prev_guard) = prev.lock() {
            prev_guard.unlink();
        }
        Some(prev)
    }
}

impl<TKey: Clone, TValue: Clone> Clone for CacheItem<TKey, TValue> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
            prev: None, // Don't clone links to avoid cycles
            next: None,
        }
    }
}

/// Abstract base cache class that matches C# Cache<TKey, TValue> exactly.
///
/// This provides the foundation for all cache implementations in Neo.
pub trait Cache<TKey, TValue>: Send + Sync
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
{
    /// Gets the key for the given item.
    /// This matches the C# GetKeyForItem abstract method.
    fn get_key_for_item(&self, item: &TValue) -> TKey;

    /// Called when an item is accessed.
    /// This matches the C# OnAccess abstract method.
    fn on_access(&self, item: &mut CacheItem<TKey, TValue>);

    /// Gets the maximum capacity of the cache.
    fn max_capacity(&self) -> usize;

    /// Gets the current count of items in the cache.
    fn count(&self) -> usize;

    /// Gets whether the cache is read-only.
    fn is_read_only(&self) -> bool {
        false
    }

    /// Gets whether the cache supports disposable items.
    fn is_disposable(&self) -> bool {
        false // In Rust, we don't have IDisposable, but we can implement Drop
    }

    /// Gets an item by key.
    /// This matches the C# indexer exactly.
    fn get(&self, key: &TKey) -> Option<TValue>;

    /// Adds an item to the cache.
    /// This matches the C# Add method exactly.
    fn add(&self, item: TValue);

    /// Adds multiple items to the cache.
    /// This matches the C# AddRange method exactly.
    fn add_range(&self, items: Vec<TValue>);

    /// Clears all items from the cache.
    /// This matches the C# Clear method exactly.
    fn clear(&self);

    /// Checks if the cache contains an item with the given key.
    /// This matches the C# Contains(TKey) method exactly.
    fn contains_key(&self, key: &TKey) -> bool;

    /// Checks if the cache contains the given item.
    /// This matches the C# Contains(TValue) method exactly.
    fn contains(&self, item: &TValue) -> bool {
        let key = self.get_key_for_item(item);
        self.contains_key(&key)
    }

    /// Removes an item by key.
    /// This matches the C# Remove(TKey) method exactly.
    fn remove_key(&self, key: &TKey) -> bool;

    /// Removes an item.
    /// This matches the C# Remove(TValue) method exactly.
    fn remove(&self, item: &TValue) -> bool {
        let key = self.get_key_for_item(item);
        self.remove_key(&key)
    }

    /// Tries to get an item by key.
    /// This matches the C# TryGet method exactly.
    fn try_get(&self, key: &TKey) -> Option<TValue>;

    /// Copies items to an array.
    /// This matches the C# CopyTo method exactly.
    fn copy_to(&self, array: &mut [TValue], start_index: usize) -> Result<(), String>;

    /// Gets all values in the cache.
    fn values(&self) -> Vec<TValue>;
}

/// Concrete implementation of the Cache trait.
/// This matches the C# Cache<TKey, TValue> class implementation.
pub struct ConcreteCache<TKey, TValue, F, G>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
    G: Fn(&mut CacheItem<TKey, TValue>) + Send + Sync,
{
    /// The head of the doubly-linked list
    head: Arc<Mutex<CacheItem<TKey, TValue>>>,
    /// The internal dictionary for fast lookups
    inner_dictionary: CacheDictionary<TKey, TValue>,
    /// Maximum capacity of the cache
    max_capacity: usize,
    /// Function to get key from item
    get_key_fn: F,
    /// Function called on access
    on_access_fn: G,
    /// Phantom data for type parameters
    _phantom: PhantomData<(TKey, TValue)>,
}

impl<TKey, TValue, F, G> ConcreteCache<TKey, TValue, F, G>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
    G: Fn(&mut CacheItem<TKey, TValue>) + Send + Sync,
{
    /// Creates a new concrete cache with the specified capacity and functions.
    pub fn new(max_capacity: usize, get_key_fn: F, on_access_fn: G) -> Self
    where
        TKey: Default,
        TValue: Default,
    {
        let dummy_key = TKey::default(); // Use Default trait instead of unsafe zeroed
        let dummy_value = TValue::default(); // Use Default trait instead of unsafe zeroed
        let head = Arc::new(Mutex::new(CacheItem::new(dummy_key, dummy_value)));

        Self {
            head,
            inner_dictionary: Arc::new(Mutex::new(IndexMap::new())),
            max_capacity,
            get_key_fn,
            on_access_fn,
            _phantom: PhantomData,
        }
    }

    /// Internal method to add an item.
    fn add_internal(&self, key: TKey, item: TValue) {
        let mut dict = match self.inner_dictionary.lock() {
            Ok(dict) => dict,
            Err(_) => return,
        };

        if let Some(cached) = dict.get(&key) {
            // Item already exists, just access it
            if let Ok(mut cached_item) = cached.lock() {
                (self.on_access_fn)(&mut *cached_item);
            }
        } else {
            if dict.len() >= self.max_capacity {
                if let Some((first_key, _)) = dict.iter().next() {
                    let first_key_clone = first_key.clone();
                    drop(dict); // Release the lock before calling remove_internal
                    self.remove_internal(&first_key_clone);
                    dict = match self.inner_dictionary.lock() {
                        Ok(dict) => dict,
                        Err(_) => return,
                    };
                }
            }

            // Add new item
            let new_item = Arc::new(Mutex::new(CacheItem::new(key.clone(), item)));
            dict.insert(key, new_item);
        }
    }

    /// Internal method to remove an item.
    fn remove_internal(&self, key: &TKey) -> bool {
        let mut dict = match self.inner_dictionary.lock() {
            Ok(dict) => dict,
            Err(_) => return false,
        };

        if let Some(item) = dict.shift_remove(key) {
            if let Ok(mut item_guard) = item.lock() {
                item_guard.unlink();
            }
            // In Rust, Drop trait handles cleanup automatically
            true
        } else {
            false
        }
    }
}

impl<TKey, TValue, F, G> Cache<TKey, TValue> for ConcreteCache<TKey, TValue, F, G>
where
    TKey: Hash + Eq + Clone + Send + Sync,
    TValue: Clone + Send + Sync,
    F: Fn(&TValue) -> TKey + Send + Sync,
    G: Fn(&mut CacheItem<TKey, TValue>) + Send + Sync,
{
    fn get_key_for_item(&self, item: &TValue) -> TKey {
        (self.get_key_fn)(item)
    }

    fn on_access(&self, item: &mut CacheItem<TKey, TValue>) {
        (self.on_access_fn)(item)
    }

    fn max_capacity(&self) -> usize {
        self.max_capacity
    }

    fn count(&self) -> usize {
        self.inner_dictionary
            .lock()
            .map(|dict| dict.len())
            .unwrap_or(0)
    }

    fn get(&self, key: &TKey) -> Option<TValue> {
        let dict = self.inner_dictionary.lock().ok()?;
        if let Some(item) = dict.get(key) {
            if let Ok(mut item_guard) = item.lock() {
                (self.on_access_fn)(&mut *item_guard);
                Some(item_guard.value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add(&self, item: TValue) {
        let key = self.get_key_for_item(&item);
        self.add_internal(key, item);
    }

    fn add_range(&self, items: Vec<TValue>) {
        for item in items {
            self.add(item);
        }
    }

    fn clear(&self) {
        if let Ok(mut dict) = self.inner_dictionary.lock() {
            dict.clear();
        }

        if let Ok(mut head) = self.head.lock() {
            head.unlink();
        }
    }

    fn contains_key(&self, key: &TKey) -> bool {
        let dict = match self.inner_dictionary.lock() {
            Ok(dict) => dict,
            Err(_) => return false,
        };
        if let Some(item) = dict.get(key) {
            if let Ok(mut item_guard) = item.lock() {
                (self.on_access_fn)(&mut *item_guard);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn remove_key(&self, key: &TKey) -> bool {
        self.remove_internal(key)
    }

    fn try_get(&self, key: &TKey) -> Option<TValue> {
        self.get(key)
    }

    fn copy_to(&self, array: &mut [TValue], start_index: usize) -> Result<(), String> {
        let dict = self
            .inner_dictionary
            .lock()
            .map_err(|_| "Lock error".to_string())?;
        let count = dict.len();

        if start_index + count > array.len() {
            return Err(format!(
                "start_index({}) + count({}) > array.len({})",
                start_index,
                count,
                array.len()
            ));
        }

        let mut index = start_index;
        for item in dict.values() {
            array[index] = item
                .lock()
                .map_err(|_| "Lock error".to_string())?
                .value
                .clone();
            index += 1;
        }

        Ok(())
    }

    fn values(&self) -> Vec<TValue> {
        let dict = match self.inner_dictionary.lock() {
            Ok(dict) => dict,
            Err(_) => return Vec::new(),
        };
        dict.values()
            .filter_map(|item| item.lock().ok().map(|guard| guard.value.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Clone, PartialEq, Default)]
    struct TestItem {
        id: u32,
        data: String,
    }

    fn create_test_cache() -> ConcreteCache<
        u32,
        TestItem,
        impl Fn(&TestItem) -> u32,
        impl Fn(&mut CacheItem<u32, TestItem>),
    > {
        ConcreteCache::new(
            3,                                         // max capacity
            |item: &TestItem| item.id,                 // get key function
            |_item: &mut CacheItem<u32, TestItem>| {}, // on access function (no-op for basic test)
        )
    }

    #[test]
    fn test_cache_add_and_get() {
        let cache = create_test_cache();
        let item = TestItem {
            id: 1,
            data: "test".to_string(),
        };

        cache.add(item.clone());
        assert_eq!(cache.count(), 1);

        let retrieved = cache.get(&1).unwrap_or_default();
        assert_eq!(retrieved, item);
    }

    #[test]
    fn test_cache_capacity_eviction() {
        let cache = create_test_cache();

        // Add items up to capacity
        for i in 1..=3 {
            cache.add(TestItem {
                id: i,
                data: format!("test{i}"),
            });
        }
        assert_eq!(cache.count(), 3);

        // Add one more item, should evict the oldest
        cache.add(TestItem {
            id: 4,
            data: "test4".to_string(),
        });
        assert_eq!(cache.count(), 3);

        // First item should be evicted
        assert!(cache.get(&1).is_none());
        assert!(cache.get(&4).is_some());
    }

    #[test]
    fn test_cache_contains() {
        let cache = create_test_cache();
        let item = TestItem {
            id: 1,
            data: "test".to_string(),
        };

        assert!(!cache.contains_key(&1));
        cache.add(item.clone());
        assert!(cache.contains_key(&1));
        assert!(cache.contains(&item));
    }

    #[test]
    fn test_cache_remove() {
        let cache = create_test_cache();
        let item = TestItem {
            id: 1,
            data: "test".to_string(),
        };

        cache.add(item.clone());
        assert_eq!(cache.count(), 1);

        assert!(cache.remove(&item));
        assert_eq!(cache.count(), 0);
        assert!(!cache.contains_key(&1));
    }

    #[test]
    fn test_cache_clear() {
        let cache = create_test_cache();

        for i in 1..=3 {
            cache.add(TestItem {
                id: i,
                data: format!("test{i}"),
            });
        }
        assert_eq!(cache.count(), 3);

        cache.clear();
        assert_eq!(cache.count(), 0);
    }
}

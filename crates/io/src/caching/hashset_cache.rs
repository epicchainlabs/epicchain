//! HashSet Cache implementation that matches C# Neo.IO.Caching.HashSetCache exactly.
//!
//! This module provides a cache that stores unique items using a hash set.

use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// HashSet cache implementation that matches C# HashSetCache<T> exactly.
///
/// This cache stores unique items and provides fast lookup and insertion.
/// It automatically manages capacity by removing items when the limit is reached.
pub struct HashSetCache<T>
where
    T: Hash + Eq + Clone + Send + Sync,
{
    /// The internal hash set for storing items
    items: Arc<Mutex<HashSet<T>>>,
    /// Maximum capacity of the cache
    max_capacity: usize,
}

impl<T> HashSetCache<T>
where
    T: Hash + Eq + Clone + Send + Sync,
{
    /// Creates a new HashSet cache with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `max_capacity` - The maximum number of items the cache can hold
    ///
    /// # Returns
    ///
    /// A new HashSet cache
    pub fn new(max_capacity: usize) -> Self {
        Self {
            items: Arc::new(Mutex::new(HashSet::new())),
            max_capacity,
        }
    }

    /// Gets the maximum capacity of the cache.
    pub fn max_capacity(&self) -> usize {
        self.max_capacity
    }

    /// Gets the current count of items in the cache.
    pub fn count(&self) -> usize {
        self.items.lock().map(|items| items.len()).unwrap_or(0)
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.items
            .lock()
            .map(|items| items.is_empty())
            .unwrap_or(true)
    }

    /// Adds an item to the cache.
    /// If the cache is at capacity, this may remove an existing item.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add
    ///
    /// # Returns
    ///
    /// True if the item was added (wasn't already present), false otherwise
    pub fn add(&self, item: T) -> bool {
        let mut items = match self.items.lock() {
            Ok(items) => items,
            Err(_) => return false,
        };

        if items.contains(&item) {
            return false;
        }

        if items.len() >= self.max_capacity {
            if let Some(to_remove) = items.iter().next().cloned() {
                items.remove(&to_remove);
            }
        }

        items.insert(item)
    }

    /// Checks if the cache contains the specified item.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to check for
    ///
    /// # Returns
    ///
    /// True if the item is in the cache, false otherwise
    pub fn contains(&self, item: &T) -> bool {
        self.items
            .lock()
            .map(|items| items.contains(item))
            .unwrap_or(false)
    }

    /// Removes an item from the cache.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to remove
    ///
    /// # Returns
    ///
    /// True if the item was removed, false if it wasn't present
    pub fn remove(&self, item: &T) -> bool {
        self.items
            .lock()
            .map(|mut items| items.remove(item))
            .unwrap_or(false)
    }

    /// Clears all items from the cache.
    pub fn clear(&self) {
        if let Ok(mut items) = self.items.lock() {
            items.clear();
        }
    }

    /// Gets all items in the cache as a vector.
    ///
    /// # Returns
    ///
    /// A vector containing all items in the cache
    pub fn to_vec(&self) -> Vec<T> {
        self.items
            .lock()
            .map(|items| items.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Adds multiple items to the cache.
    ///
    /// # Arguments
    ///
    /// * `items` - The items to add
    pub fn add_range(&self, items: Vec<T>) {
        for item in items {
            self.add(item);
        }
    }

    /// Copies items to the provided slice starting at the specified index.
    ///
    /// # Arguments
    ///
    /// * `array` - The array to copy items to
    /// * `start_index` - The starting index in the array
    ///
    /// # Returns
    ///
    /// Result indicating success or error message
    pub fn copy_to(&self, array: &mut [T], start_index: usize) -> Result<(), String> {
        let items = self.items.lock().map_err(|_| "Lock error".to_string())?;
        let count = items.len();

        if start_index + count > array.len() {
            return Err(format!(
                "start_index({}) + count({}) > array.len({})",
                start_index,
                count,
                array.len()
            ));
        }

        let mut index = start_index;
        for item in items.iter() {
            array[index] = item.clone();
            index += 1;
        }

        Ok(())
    }

    /// Creates an iterator over the items in the cache.
    /// Note: This creates a snapshot of the current items.
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.to_vec().into_iter()
    }
}

impl<T> Clone for HashSetCache<T>
where
    T: Hash + Eq + Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        let items = self
            .items
            .lock()
            .map(|items| items.clone())
            .unwrap_or_default();
        Self {
            items: Arc::new(Mutex::new(items)),
            max_capacity: self.max_capacity,
        }
    }
}

impl<T> std::iter::FromIterator<T> for HashSetCache<T>
where
    T: Hash + Eq + Clone + Send + Sync,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<T> = iter.into_iter().collect();
        let capacity = items.len().max(16); // Default minimum capacity
        let cache = Self::new(capacity);
        cache.add_range(items);
        cache
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_hashset_cache_basic_operations() {
        let cache = HashSetCache::new(3);

        // Test add and contains
        assert!(cache.add(1));
        assert!(cache.add(2));
        assert!(cache.add(3));
        assert_eq!(cache.count(), 3);

        assert!(cache.contains(&1));
        assert!(cache.contains(&2));
        assert!(cache.contains(&3));
        assert!(!cache.contains(&4));
    }

    #[test]
    fn test_hashset_cache_duplicate_add() {
        let cache = HashSetCache::new(3);

        assert!(cache.add(1)); // First add should succeed
        assert!(!cache.add(1)); // Duplicate add should fail
        assert_eq!(cache.count(), 1);
    }

    #[test]
    fn test_hashset_cache_capacity_management() {
        let cache = HashSetCache::new(2);

        // Add items up to capacity
        assert!(cache.add(1));
        assert!(cache.add(2));
        assert_eq!(cache.count(), 2);

        // Add one more item, should evict one existing item
        assert!(cache.add(3));
        assert_eq!(cache.count(), 2);

        // One of the original items should be gone
        let remaining_count = [1, 2].iter().filter(|&&x| cache.contains(&x)).count();
        assert_eq!(remaining_count, 1);
        assert!(cache.contains(&3)); // New item should be present
    }

    #[test]
    fn test_hashset_cache_remove() {
        let cache = HashSetCache::new(3);

        cache.add(1);
        cache.add(2);
        cache.add(3);

        assert!(cache.remove(&2));
        assert_eq!(cache.count(), 2);
        assert!(!cache.contains(&2));
        assert!(cache.contains(&1));
        assert!(cache.contains(&3));

        // Try to remove non-existent item
        assert!(!cache.remove(&4));
        assert_eq!(cache.count(), 2);
    }

    #[test]
    fn test_hashset_cache_clear() {
        let cache = HashSetCache::new(3);

        cache.add(1);
        cache.add(2);
        cache.add(3);
        assert_eq!(cache.count(), 3);

        cache.clear();
        assert_eq!(cache.count(), 0);
        assert!(cache.is_empty());
        assert!(!cache.contains(&1));
        assert!(!cache.contains(&2));
        assert!(!cache.contains(&3));
    }

    #[test]
    fn test_hashset_cache_add_range() {
        let cache = HashSetCache::new(5);

        cache.add_range(vec![1, 2, 3, 2, 4]); // Note: 2 is duplicated

        // Should have 4 unique items
        assert_eq!(cache.count(), 4);
        assert!(cache.contains(&1));
        assert!(cache.contains(&2));
        assert!(cache.contains(&3));
        assert!(cache.contains(&4));
    }

    #[test]
    fn test_hashset_cache_to_vec() {
        let cache = HashSetCache::new(3);

        cache.add(1);
        cache.add(2);
        cache.add(3);

        let items = cache.to_vec();
        assert_eq!(items.len(), 3);

        assert!(items.contains(&1));
        assert!(items.contains(&2));
        assert!(items.contains(&3));
    }

    #[test]
    fn test_hashset_cache_copy_to() {
        let cache = HashSetCache::new(3);

        cache.add(1);
        cache.add(2);

        let mut array = [0; 5];
        cache.copy_to(&mut array, 1).unwrap();

        // Check that items were copied starting at index 1
        assert_eq!(array[0], 0); // Should be unchanged
        let copied_items = &array[1..3];
        assert!(copied_items.contains(&1));
        assert!(copied_items.contains(&2));
        assert_eq!(array[3], 0); // Should be unchanged
        assert_eq!(array[4], 0); // Should be unchanged
    }

    #[test]
    fn test_hashset_cache_copy_to_error() {
        let cache = HashSetCache::new(3);

        cache.add(1);
        cache.add(2);
        cache.add(3);

        let mut array = [0; 3];
        let result = cache.copy_to(&mut array, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_hashset_cache_from_iter() {
        let cache: HashSetCache<i32> = [1, 2, 3, 2, 4].iter().cloned().collect();

        assert_eq!(cache.count(), 4); // Should have 4 unique items
        assert!(cache.contains(&1));
        assert!(cache.contains(&2));
        assert!(cache.contains(&3));
        assert!(cache.contains(&4));
    }

    #[test]
    fn test_hashset_cache_clone() {
        let cache1 = HashSetCache::new(3);
        cache1.add(1);
        cache1.add(2);

        let cache2 = cache1.clone();

        // Both caches should have the same items
        assert_eq!(cache1.count(), cache2.count());
        assert!(cache2.contains(&1));
        assert!(cache2.contains(&2));

        // But they should be independent
        cache1.add(3);
        assert!(cache1.contains(&3));
        assert!(!cache2.contains(&3));
    }
}

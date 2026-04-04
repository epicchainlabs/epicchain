//! Collection utilities and extensions

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Extension trait for Vec operations
pub trait VecExt<T> {
    /// Remove duplicates while preserving order
    fn dedup_preserve_order(&mut self)
    where
        T: Clone + Eq + Hash;

    /// Split vector into chunks of specified size
    fn chunks_exact_vec(&self, chunk_size: usize) -> Vec<Vec<T>>
    where
        T: Clone;

    /// Find the index of the maximum element
    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd;

    /// Find the index of the minimum element
    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd;

    /// Check if vector is sorted
    fn is_sorted(&self) -> bool
    where
        T: PartialOrd;

    /// Get the last n elements
    fn last_n(&self, n: usize) -> &[T];

    /// Get the first n elements
    fn first_n(&self, n: usize) -> &[T];
}

impl<T> VecExt<T> for Vec<T> {
    fn dedup_preserve_order(&mut self)
    where
        T: Clone + Eq + Hash,
    {
        let mut seen = HashSet::new();
        self.retain(|item| seen.insert(item.clone()));
    }

    fn chunks_exact_vec(&self, chunk_size: usize) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        if chunk_size == 0 {
            return vec![];
        }

        self.chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn argmax(&self) -> Option<usize>
    where
        T: PartialOrd,
    {
        if self.is_empty() {
            return None;
        }

        let mut max_idx = 0;
        for (i, item) in self.iter().enumerate().skip(1) {
            if item > &self[max_idx] {
                max_idx = i;
            }
        }
        Some(max_idx)
    }

    fn argmin(&self) -> Option<usize>
    where
        T: PartialOrd,
    {
        if self.is_empty() {
            return None;
        }

        let mut min_idx = 0;
        for (i, item) in self.iter().enumerate().skip(1) {
            if item < &self[min_idx] {
                min_idx = i;
            }
        }
        Some(min_idx)
    }

    fn is_sorted(&self) -> bool
    where
        T: PartialOrd,
    {
        self.windows(2).all(|w| w[0] <= w[1])
    }

    fn last_n(&self, n: usize) -> &[T] {
        let start = self.len().saturating_sub(n);
        &self[start..]
    }

    fn first_n(&self, n: usize) -> &[T] {
        let end = n.min(self.len());
        &self[..end]
    }
}

/// Extension trait for HashMap operations
pub trait HashMapExt<K, V> {
    /// Get value or insert default
    fn get_or_insert_default(&mut self, key: K) -> &mut V
    where
        K: Clone + Eq + Hash,
        V: Default;

    /// Merge another HashMap into this one
    fn merge(&mut self, other: HashMap<K, V>)
    where
        K: Eq + Hash;

    /// Get multiple values by keys
    fn get_many(&self, keys: &[K]) -> Vec<Option<&V>>
    where
        K: Eq + Hash;

    /// Filter by predicate on values
    fn filter_values<F>(&self, predicate: F) -> HashMap<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
        F: Fn(&V) -> bool;
}

impl<K, V> HashMapExt<K, V> for HashMap<K, V> {
    fn get_or_insert_default(&mut self, key: K) -> &mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        self.entry(key).or_insert_with(V::default)
    }

    fn merge(&mut self, other: HashMap<K, V>)
    where
        K: Eq + Hash,
    {
        for (key, value) in other {
            self.insert(key, value);
        }
    }

    fn get_many(&self, keys: &[K]) -> Vec<Option<&V>>
    where
        K: Eq + Hash,
    {
        keys.iter().map(|key| self.get(key)).collect()
    }

    fn filter_values<F>(&self, predicate: F) -> HashMap<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
        F: Fn(&V) -> bool,
    {
        self.iter()
            .filter(|(_, v)| predicate(v))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

/// Extension trait for HashSet operations
pub trait HashSetExt<T> {
    /// Check if this set is a subset of another
    fn is_subset_of(&self, other: &HashSet<T>) -> bool
    where
        T: Eq + Hash;

    /// Check if this set is a superset of another
    fn is_superset_of(&self, other: &HashSet<T>) -> bool
    where
        T: Eq + Hash;

    /// Get the intersection with another set
    fn intersection_with(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash;

    /// Get the union with another set
    fn union_with(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash;

    /// Get the difference with another set
    fn difference_with(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash;
}

impl<T> HashSetExt<T> for HashSet<T> {
    fn is_subset_of(&self, other: &HashSet<T>) -> bool
    where
        T: Eq + Hash,
    {
        self.iter().all(|item| other.contains(item))
    }

    fn is_superset_of(&self, other: &HashSet<T>) -> bool
    where
        T: Eq + Hash,
    {
        other.is_subset_of(self)
    }

    fn intersection_with(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash,
    {
        self.intersection(other).cloned().collect()
    }

    fn union_with(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash,
    {
        self.union(other).cloned().collect()
    }

    fn difference_with(&self, other: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash,
    {
        self.difference(other).cloned().collect()
    }
}

/// Utility functions for collections
pub struct Collections;

impl Collections {
    /// Create a HashMap from key-value pairs
    pub fn hashmap_from_pairs<K, V>(pairs: Vec<(K, V)>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        pairs.into_iter().collect()
    }

    /// Create a HashSet from values
    pub fn hashset_from_values<T>(values: Vec<T>) -> HashSet<T>
    where
        T: Eq + Hash,
    {
        values.into_iter().collect()
    }

    /// Group vector elements by a key function
    pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
    where
        K: Eq + Hash,
        F: Fn(&T) -> K,
    {
        let mut groups: HashMap<K, Vec<T>> = HashMap::new();

        for item in items {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }

        groups
    }

    /// Count occurrences of each element
    pub fn count_occurrences<T>(items: &[T]) -> HashMap<T, usize>
    where
        T: Clone + Eq + Hash,
    {
        let mut counts = HashMap::new();

        for item in items {
            *counts.entry(item.clone()).or_insert(0) += 1;
        }

        counts
    }

    /// Find the most common element
    pub fn most_common<T>(items: &[T]) -> Option<T>
    where
        T: Clone + Eq + Hash,
    {
        let counts = Self::count_occurrences(items);
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(item, _)| item)
    }

    /// Partition vector into two based on predicate
    pub fn partition<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
    where
        F: Fn(&T) -> bool,
    {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();

        for item in items {
            if predicate(&item) {
                true_items.push(item);
            } else {
                false_items.push(item);
            }
        }

        (true_items, false_items)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_extensions() {
        let mut vec = vec![1, 2, 2, 3, 1, 4];
        vec.dedup_preserve_order();
        assert_eq!(vec, vec![1, 2, 3, 4]);

        let vec = vec![1, 2, 3, 4, 5, 6];
        let chunks = vec.chunks_exact_vec(2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);

        let vec = vec![3, 1, 4, 1, 5];
        assert_eq!(vec.argmax(), Some(4));
        assert_eq!(vec.argmin(), Some(1));

        let vec = vec![1, 2, 3, 4, 5];
        assert!(vec.is_sorted());

        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.last_n(3), &[3, 4, 5]);
        assert_eq!(vec.first_n(3), &[1, 2, 3]);
    }

    #[test]
    fn test_hashmap_extensions() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let value = map.get_or_insert_default("c");
        *value = 3;
        assert_eq!(map.get("c"), Some(&3));

        let mut other = HashMap::new();
        other.insert("d", 4);
        map.merge(other);
        assert_eq!(map.get("d"), Some(&4));

        let keys = vec!["a", "b", "x"];
        let values = map.get_many(&keys);
        assert_eq!(values, vec![Some(&1), Some(&2), None]);

        let filtered = map.filter_values(|&v| v > 2);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_hashset_extensions() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

        assert!(!set1.is_subset_of(&set2));
        assert!(!set1.is_superset_of(&set2));

        let intersection = set1.intersection_with(&set2);
        assert_eq!(intersection, [2, 3].iter().cloned().collect());

        let union = set1.union_with(&set2);
        assert_eq!(union, [1, 2, 3, 4].iter().cloned().collect());

        let difference = set1.difference_with(&set2);
        assert_eq!(difference, [1].iter().cloned().collect());
    }

    #[test]
    fn test_collections_utilities() {
        let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
        let map = Collections::hashmap_from_pairs(pairs);
        assert_eq!(map.get("a"), Some(&1));

        let values = vec![1, 2, 3, 2, 1];
        let counts = Collections::count_occurrences(&values);
        assert_eq!(counts.get(&1), Some(&2));
        assert_eq!(counts.get(&2), Some(&2));
        assert_eq!(counts.get(&3), Some(&1));

        let most_common = Collections::most_common(&values);
        assert!(most_common == Some(1) || most_common == Some(2)); // Both have count 2

        let items = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds) = Collections::partition(items, |&x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
}

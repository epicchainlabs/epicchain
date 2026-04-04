use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

/// An ordered dictionary that maintains insertion order
/// This matches the C# OrderedDictionary implementation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderedDictionary<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    keys: Vec<K>,
    map: HashMap<K, V>,
}

impl<K, V> OrderedDictionary<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// Creates a new empty OrderedDictionary
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// Creates a new OrderedDictionary with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: Vec::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Inserts a key-value pair into the dictionary
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if !self.map.contains_key(&key) {
            self.keys.push(key.clone());
        }
        self.map.insert(key, value)
    }

    /// Gets a value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /// Gets a mutable reference to a value by key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    /// Removes a key-value pair from the dictionary
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            self.keys.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }

    /// Checks if the dictionary contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Returns the number of key-value pairs
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Checks if the dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clears all key-value pairs
    pub fn clear(&mut self) {
        self.keys.clear();
        self.map.clear();
    }

    /// Returns an iterator over the keys in insertion order
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }

    /// Returns an iterator over the values in insertion order
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.keys.iter().filter_map(move |k| self.map.get(k))
    }

    /// Returns an iterator over key-value pairs in insertion order
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys
            .iter()
            .filter_map(move |k| self.map.get(k).map(|v| (k, v)))
    }
}

impl<K, V> Default for OrderedDictionary<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> std::ops::Index<&K> for OrderedDictionary<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        &self.map[key]
    }
}

impl<K, V> std::ops::IndexMut<&K> for OrderedDictionary<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        self.map.get_mut(key).expect("Key not found")
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_dictionary_basic() {
        let mut dict = OrderedDictionary::new();

        dict.insert("first", 1);
        dict.insert("second", 2);
        dict.insert("third", 3);

        assert_eq!(dict.len(), 3);
        assert_eq!(dict.get(&"first"), Some(&1));
        assert_eq!(dict.get(&"second"), Some(&2));
        assert_eq!(dict.get(&"third"), Some(&3));

        // Check insertion order is maintained
        let keys: Vec<_> = dict.keys().collect();
        assert_eq!(keys, vec![&"first", &"second", &"third"]);
    }

    #[test]
    fn test_ordered_dictionary_remove() {
        let mut dict = OrderedDictionary::new();

        dict.insert("a", 1);
        dict.insert("b", 2);
        dict.insert("c", 3);

        assert_eq!(dict.remove(&"b"), Some(2));
        assert_eq!(dict.len(), 2);

        let keys: Vec<_> = dict.keys().collect();
        assert_eq!(keys, vec![&"a", &"c"]);
    }
}

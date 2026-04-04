//! Wildcard container implementation for Neo smart contract manifests.
//!
//! This module provides the WildcardContainer type which can either hold
//! specific values or represent a wildcard (all values).

use serde::{Deserialize, Serialize};
use std::fmt;

/// A container that supports wildcard functionality.
///
/// This matches the C# WildcardContainer<T> implementation exactly.
/// It can either hold specific values or represent a wildcard (all values).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WildcardContainer<T> {
    /// Wildcard - represents all possible values
    #[serde(with = "wildcard_serde")]
    Wildcard,
    /// Specific values
    Values(Vec<T>),
}

impl<T> WildcardContainer<T> {
    /// Creates a new WildcardContainer with specific values.
    /// This matches C# WildcardContainer<T>.Create(params T[] data).
    pub fn create(data: Vec<T>) -> Self {
        Self::Values(data)
    }

    /// Creates a new WildcardContainer with wildcard.
    /// This matches C# WildcardContainer<T>.CreateWildcard().
    pub fn create_wildcard() -> Self {
        Self::Wildcard
    }

    /// Checks if this container is a wildcard.
    /// This matches C# WildcardContainer<T>.IsWildcard property.
    pub fn is_wildcard(&self) -> bool {
        matches!(self, Self::Wildcard)
    }

    /// Gets the count of elements.
    /// This matches C# WildcardContainer<T>.Count property.
    pub fn count(&self) -> usize {
        match self {
            Self::Wildcard => 0,
            Self::Values(values) => values.len(),
        }
    }

    /// Gets the length of elements (alias for count).
    pub fn len(&self) -> usize {
        self.count()
    }

    /// Checks if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets an element by index.
    /// This matches C# WildcardContainer<T>[int index] indexer.
    pub fn get(&self, index: usize) -> Option<&T> {
        match self {
            Self::Wildcard => None,
            Self::Values(values) => values.get(index),
        }
    }

    /// Gets all values as a slice.
    /// Returns empty slice for wildcard.
    pub fn values(&self) -> &[T] {
        match self {
            Self::Wildcard => &[],
            Self::Values(values) => values,
        }
    }

    /// Checks if the container contains a specific value.
    /// For wildcard containers, this always returns true.
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            Self::Wildcard => true, // Wildcard contains everything
            Self::Values(values) => values.contains(value),
        }
    }

    /// Iterates over the values.
    /// For wildcard containers, this returns an empty iterator.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values().iter()
    }
}

impl<T> Default for WildcardContainer<T> {
    fn default() -> Self {
        Self::create_wildcard()
    }
}

impl<T> fmt::Display for WildcardContainer<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wildcard => write!(f, "*"),
            Self::Values(values) => {
                write!(f, "[")?;
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl<T> IntoIterator for WildcardContainer<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Wildcard => Vec::new().into_iter(),
            Self::Values(values) => values.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a WildcardContainer<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

mod wildcard_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("*")
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "*" {
            Ok(())
        } else {
            Err(serde::de::Error::custom("Expected '*' for wildcard"))
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_wildcard_container_create() {
        let container = WildcardContainer::create(vec!["test1".to_string(), "test2".to_string()]);
        assert!(!container.is_wildcard());
        assert_eq!(container.count(), 2);
        assert_eq!(container.get(0), Some(&"test1".to_string()));
        assert_eq!(container.get(1), Some(&"test2".to_string()));
        assert_eq!(container.get(2), None);
    }

    #[test]
    fn test_wildcard_container_wildcard() {
        let container: WildcardContainer<String> = WildcardContainer::create_wildcard();
        assert!(container.is_wildcard());
        assert_eq!(container.count(), 0);
        assert_eq!(container.get(0), None);
    }

    #[test]
    fn test_wildcard_container_contains() {
        let container = WildcardContainer::create(vec!["test1".to_string(), "test2".to_string()]);
        assert!(container.contains(&"test1".to_string()));
        assert!(container.contains(&"test2".to_string()));
        assert!(!container.contains(&"test3".to_string()));

        let wildcard: WildcardContainer<String> = WildcardContainer::create_wildcard();
        assert!(wildcard.contains(&"anything".to_string()));
    }

    #[test]
    fn test_wildcard_container_iteration() {
        let container = WildcardContainer::create(vec!["test1".to_string(), "test2".to_string()]);
        let values: Vec<&String> = container.iter().collect();
        assert_eq!(values, vec![&"test1".to_string(), &"test2".to_string()]);

        let wildcard: WildcardContainer<String> = WildcardContainer::create_wildcard();
        let values: Vec<&String> = wildcard.iter().collect();
        assert!(values.is_empty());
    }

    #[test]
    fn test_wildcard_container_display() {
        let container = WildcardContainer::create(vec!["test1".to_string(), "test2".to_string()]);
        assert_eq!(container.to_string(), "[test1, test2]");

        let wildcard: WildcardContainer<String> = WildcardContainer::create_wildcard();
        assert_eq!(wildcard.to_string(), "*");
    }

    #[test]
    fn test_wildcard_container_serialization() {
        // Test wildcard serialization
        let wildcard: WildcardContainer<String> = WildcardContainer::create_wildcard();
        let json = serde_json::to_string(&wildcard).unwrap();
        assert_eq!(json, "\"*\"");

        // Test values serialization
        let container = WildcardContainer::create(vec!["test1".to_string(), "test2".to_string()]);
        let json = serde_json::to_string(&container).unwrap();
        assert_eq!(json, "[\"test1\",\"test2\"]");
    }

    #[test]
    fn test_wildcard_container_deserialization() {
        // Test wildcard deserialization
        let json = "\"*\"";
        let container: WildcardContainer<String> =
            serde_json::from_str(json).expect("Failed to parse from string");
        assert!(container.is_wildcard());

        // Test values deserialization
        let json = "[\"test1\",\"test2\"]";
        let container: WildcardContainer<String> =
            serde_json::from_str(json).expect("Failed to parse from string");
        assert!(!container.is_wildcard());
        assert_eq!(container.count(), 2);
        assert_eq!(container.get(0), Some(&"test1".to_string()));
        assert_eq!(container.get(1), Some(&"test2".to_string()));
    }
}

use crate::error::{JsonError, JsonResult};
use crate::JToken;
use std::ops::{Index, IndexMut};

/// Represents a JSON array
/// This matches the C# JArray class
#[derive(Debug, Clone, PartialEq)]
pub struct JArray {
    items: Vec<Option<JToken>>,
}

impl JArray {
    /// Creates a new empty JSON array
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates a new JSON array from a vector of tokens
    pub fn from_vec(items: Vec<Option<JToken>>) -> Self {
        Self { items }
    }

    /// Creates a new JSON array from an iterator of tokens
    /// Note: Consider using the FromIterator trait implementation instead
    pub fn from_iter_explicit<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<JToken>>,
    {
        Self {
            items: iter.into_iter().collect(),
        }
    }

    /// Gets the number of items in the array
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the array is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets the item at the specified index
    pub fn get(&self, index: usize) -> Option<&JToken> {
        self.items.get(index).and_then(|v| v.as_ref())
    }

    /// Gets a mutable reference to the item at the specified index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Option<JToken>> {
        self.items.get_mut(index)
    }

    /// Sets the item at the specified index
    pub fn set(&mut self, index: usize, value: Option<JToken>) -> JsonResult<()> {
        if index < self.items.len() {
            self.items[index] = value;
            Ok(())
        } else {
            Err(JsonError::InvalidOperation(format!(
                "Index {index} out of bounds"
            )))
        }
    }

    /// Adds an item to the end of the array
    pub fn add(&mut self, item: Option<JToken>) {
        self.items.push(item);
    }

    /// Inserts an item at the specified index
    pub fn insert(&mut self, index: usize, item: Option<JToken>) -> JsonResult<()> {
        if index <= self.items.len() {
            self.items.insert(index, item);
            Ok(())
        } else {
            Err(JsonError::InvalidOperation(format!(
                "Index {index} out of bounds"
            )))
        }
    }

    /// Removes the item at the specified index
    pub fn remove(&mut self, index: usize) -> JsonResult<Option<JToken>> {
        if index < self.items.len() {
            Ok(self.items.remove(index))
        } else {
            Err(JsonError::InvalidOperation(format!(
                "Index {index} out of bounds"
            )))
        }
    }

    /// Removes the first occurrence of the specified item
    pub fn remove_item(&mut self, item: &JToken) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x.as_ref() == Some(item)) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    /// Clears all items from the array
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Checks if the array contains the specified item
    pub fn contains(&self, item: &JToken) -> bool {
        self.items.iter().any(|x| x.as_ref() == Some(item))
    }

    /// Gets an iterator over the items in the array
    pub fn iter(&self) -> impl Iterator<Item = Option<&JToken>> {
        self.items.iter().map(|item| item.as_ref())
    }

    /// Gets a mutable iterator over the items in the array
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<JToken>> {
        self.items.iter_mut()
    }

    /// Converts the array to a Vec<Option<JToken>>
    pub fn to_vec(&self) -> Vec<Option<JToken>> {
        self.items.clone()
    }

    /// Gets the underlying vector (for internal use)
    pub fn items(&self) -> &Vec<Option<JToken>> {
        &self.items
    }

    /// Gets a mutable reference to the underlying vector (for internal use)
    pub fn items_mut(&mut self) -> &mut Vec<Option<JToken>> {
        &mut self.items
    }

    /// Converts the JArray to a JToken::Array
    pub fn to_jtoken(self) -> JToken {
        JToken::Array(self.items)
    }

    /// Creates a JArray from a JToken::Array
    pub fn from_jtoken(token: JToken) -> JsonResult<Self> {
        match token {
            JToken::Array(items) => Ok(Self { items }),
            _ => Err(JsonError::InvalidOperation(
                "Token is not an array".to_string(),
            )),
        }
    }
}

impl Default for JArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for JArray {
    type Output = Option<JToken>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl IndexMut<usize> for JArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl From<Vec<Option<JToken>>> for JArray {
    fn from(items: Vec<Option<JToken>>) -> Self {
        Self::from_vec(items)
    }
}

impl From<JArray> for Vec<Option<JToken>> {
    fn from(array: JArray) -> Self {
        array.items
    }
}

impl IntoIterator for JArray {
    type Item = Option<JToken>;
    type IntoIter = std::vec::IntoIter<Option<JToken>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a JArray {
    type Item = Option<&'a JToken>;
    type IntoIter = std::iter::Map<
        std::slice::Iter<'a, Option<JToken>>,
        fn(&'a Option<JToken>) -> Option<&'a JToken>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().map(|item| item.as_ref())
    }
}

impl<'a> IntoIterator for &'a mut JArray {
    type Item = &'a mut Option<JToken>;
    type IntoIter = std::slice::IterMut<'a, Option<JToken>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl std::iter::FromIterator<Option<JToken>> for JArray {
    fn from_iter<I: IntoIterator<Item = Option<JToken>>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jarray_new() {
        let array = JArray::new();
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_jarray_add() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("test".to_string())));
        array.add(Some(JToken::Number(42.0)));
        array.add(None);

        assert_eq!(array.len(), 3);
        assert_eq!(array.get(0), Some(&JToken::String("test".to_string())));
        assert_eq!(array.get(1), Some(&JToken::Number(42.0)));
        assert_eq!(array.get(2), None);
    }

    #[test]
    fn test_jarray_insert() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("first".to_string())));
        array.add(Some(JToken::String("third".to_string())));

        array
            .insert(1, Some(JToken::String("second".to_string())))
            .unwrap();

        assert_eq!(array.len(), 3);
        assert_eq!(array.get(0), Some(&JToken::String("first".to_string())));
        assert_eq!(array.get(1), Some(&JToken::String("second".to_string())));
        assert_eq!(array.get(2), Some(&JToken::String("third".to_string())));
    }

    #[test]
    fn test_jarray_remove() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("first".to_string())));
        array.add(Some(JToken::String("second".to_string())));
        array.add(Some(JToken::String("third".to_string())));

        let removed = array.remove(1).unwrap();
        assert_eq!(removed, Some(JToken::String("second".to_string())));
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(1), Some(&JToken::String("third".to_string())));
    }

    #[test]
    fn test_jarray_indexing() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("test".to_string())));

        assert_eq!(array[0], Some(JToken::String("test".to_string())));
        array[0] = Some(JToken::Number(123.0));
        assert_eq!(array[0], Some(JToken::Number(123.0)));
    }

    #[test]
    fn test_jarray_contains() {
        let mut array = JArray::new();
        let token = JToken::String("test".to_string());
        array.add(Some(token.clone()));

        assert!(array.contains(&token));
        assert!(!array.contains(&JToken::Number(42.0)));
    }

    #[test]
    fn test_jarray_iteration() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("first".to_string())));
        array.add(Some(JToken::String("second".to_string())));

        let items: Vec<_> = array.iter().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], Some(&JToken::String("first".to_string())));
        assert_eq!(items[1], Some(&JToken::String("second".to_string())));
    }

    #[test]
    fn test_jarray_from_vec() {
        let vec = vec![
            Some(JToken::String("test".to_string())),
            Some(JToken::Number(42.0)),
            None,
        ];
        let array = JArray::from_vec(vec.clone());

        assert_eq!(array.len(), 3);
        assert_eq!(array.to_vec(), vec);
    }

    #[test]
    fn test_jarray_jtoken_conversion() {
        let mut array = JArray::new();
        array.add(Some(JToken::String("test".to_string())));

        let token = array.clone().to_jtoken();
        let token_clone = token.clone(); // Clone before pattern matching
        match token {
            JToken::Array(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], Some(JToken::String("test".to_string())));
            }
            _ => panic!("Expected JToken::Array"),
        }

        let array2 = JArray::from_jtoken(token_clone).expect("operation should succeed");
        assert_eq!(array, array2);
    }
}

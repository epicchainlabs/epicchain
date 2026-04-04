//! Array stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Array stack item implementation used in the Neo VM.

use crate::reference_counter::ReferenceCounter;
use crate::stack_item::stack_item_type::StackItemType;
use crate::stack_item::StackItem;
use num_traits::ToPrimitive;
use std::sync::Arc;

/// Represents an array of stack items in the VM.
#[derive(Debug, Clone)]
pub struct Array {
    /// The items in the array.
    items: Vec<StackItem>,
    /// The reference counter for the VM.
    reference_id: Option<usize>,
}

impl Array {
    /// Creates a new array with the specified items.
    pub fn new(items: Vec<StackItem>, reference_counter: Option<Arc<ReferenceCounter>>) -> Self {
        let mut reference_id = None;

        if let Some(rc) = &reference_counter {
            reference_id = Some(rc.add_reference());
        }

        Self {
            items,
            reference_id,
        }
    }

    /// Gets the items in the array.
    pub fn items(&self) -> &[StackItem] {
        &self.items
    }

    /// Gets a mutable reference to the items in the array.
    pub fn items_mut(&mut self) -> &mut Vec<StackItem> {
        &mut self.items
    }

    /// Gets the item at the specified index.
    pub fn get(&self, index: usize) -> Option<&StackItem> {
        self.items.get(index)
    }

    /// Sets the item at the specified index.
    pub fn set(&mut self, index: usize, item: StackItem) {
        if index < self.items.len() {
            self.items[index] = item;
        }
    }

    /// Adds an item to the end of the array.
    pub fn push(&mut self, item: StackItem) {
        self.items.push(item);
    }

    /// Removes and returns the last item in the array.
    pub fn pop(&mut self) -> Option<StackItem> {
        self.items.pop()
    }

    /// Gets the number of items in the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Removes all items from the array.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Creates a deep copy of the array.
    pub fn deep_copy(&self, reference_counter: Option<Arc<ReferenceCounter>>) -> Self {
        let items = self.items.iter().map(|item| item.deep_clone()).collect();
        Self::new(items, reference_counter)
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Array
    }

    /// Inserts an item at the specified index.
    pub fn insert(&mut self, index: usize, item: StackItem) {
        if index <= self.items.len() {
            self.items.insert(index, item);
        }
    }

    /// Removes the item at the specified index.
    pub fn remove(&mut self, index: usize) -> Option<StackItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Gets an iterator over the items.
    pub fn iter(&self) -> std::slice::Iter<StackItem> {
        self.items.iter()
    }

    /// Gets a mutable iterator over the items.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<StackItem> {
        self.items.iter_mut()
    }

    /// Gets a reference to the items vector.
    pub fn items_ref(&self) -> &Vec<StackItem> {
        &self.items
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        // Reference cleanup is handled by the ReferenceCounter automatically
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let array = Array::new(items.clone(), None);

        assert_eq!(array.len(), 3);
        assert_eq!(array.items(), &items);
        assert_eq!(array.stack_item_type(), StackItemType::Array);
    }

    #[test]
    fn test_array_get() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let array = Array::new(items, None);

        assert_eq!(array.get(0).unwrap().as_int().unwrap().to_i32().unwrap(), 1);
        assert_eq!(array.get(1).unwrap().as_int().unwrap().to_i32().unwrap(), 2);
        assert_eq!(array.get(2).unwrap().as_int().unwrap().to_i32().unwrap(), 3);
        assert!(array.get(3).is_none());
        Ok(())
    }

    #[test]
    fn test_array_set() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let mut array = Array::new(items, None);

        array.set(1, StackItem::from_int(42));

        assert_eq!(array.get(0).unwrap().as_int().unwrap().to_i32().unwrap(), 1);
        assert_eq!(
            array.get(1).unwrap().as_int().unwrap().to_i32().unwrap(),
            42
        );
        assert_eq!(array.get(2).unwrap().as_int().unwrap().to_i32().unwrap(), 3);

        // Test setting out of bounds - should not panic, just do nothing
        array.set(3, StackItem::from_int(4));
        assert_eq!(array.len(), 3); // Length should remain the same
        Ok(())
    }

    #[test]
    fn test_array_push_pop() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![StackItem::from_int(1), StackItem::from_int(2)];

        let mut array = Array::new(items, None);

        array.push(StackItem::from_int(3));

        assert_eq!(array.len(), 3);
        assert_eq!(array.get(2).unwrap().as_int().unwrap().to_i32().unwrap(), 3);

        let popped = array.pop().unwrap();

        assert_eq!(array.len(), 2);
        assert_eq!(
            popped
                .as_int()
                .expect("intermediate value should exist")
                .to_i32()
                .unwrap(),
            3
        );
        Ok(())
    }

    #[test]
    fn test_array_clear() {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let mut array = Array::new(items, None);

        array.clear();

        assert_eq!(array.len(), 0);
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_deep_copy() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_array(vec![StackItem::from_int(3), StackItem::from_int(4)]),
        ];

        let array = Array::new(items, None);
        let copied = array.deep_copy(None);

        assert_eq!(copied.len(), array.len());
        assert_eq!(
            copied.get(0).unwrap().as_int().unwrap(),
            array.get(0).unwrap().as_int().unwrap()
        );
        assert_eq!(
            copied.get(0).unwrap().as_int().unwrap(),
            array.get(0).unwrap().as_int().unwrap()
        );

        // Check that the nested array was deep copied
        let nested_original = array.get(2).unwrap().as_array().unwrap();
        let nested_copied = copied.get(2).unwrap().as_array().unwrap();

        assert_eq!(nested_copied.len(), nested_original.len());
        assert_eq!(
            nested_copied[0].as_int().unwrap(),
            nested_original[0].as_int().unwrap()
        );
        assert_eq!(
            nested_copied[1].as_int().unwrap(),
            nested_original[1].as_int().unwrap()
        );
        Ok(())
    }
}

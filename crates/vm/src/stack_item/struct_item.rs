//! Struct stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Struct stack item implementation used in the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::reference_counter::ReferenceCounter;
use crate::stack_item::stack_item_type::StackItemType;
use crate::stack_item::StackItem;
use num_traits::ToPrimitive;
use std::sync::Arc;

/// Represents a struct of stack items in the VM.
#[derive(Debug, Clone)]
pub struct Struct {
    /// The items in the struct.
    items: Vec<StackItem>,
    /// The reference ID for the VM.
    reference_id: Option<usize>,
}

impl Struct {
    /// Creates a new struct with the specified items and reference counter.
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

    /// Gets the items in the struct.
    pub fn items(&self) -> &[StackItem] {
        &self.items
    }

    /// Gets a mutable reference to the items in the struct.
    pub fn items_mut(&mut self) -> &mut Vec<StackItem> {
        &mut self.items
    }

    /// Gets the item at the specified index.
    pub fn get(&self, index: usize) -> VmResult<&StackItem> {
        self.items
            .get(index)
            .ok_or_else(|| VmError::invalid_operation_msg(format!("Index out of range: {index}")))
    }

    /// Sets the item at the specified index.
    pub fn set(&mut self, index: usize, item: StackItem) -> VmResult<()> {
        if index >= self.items.len() {
            return Err(VmError::invalid_operation_msg(format!(
                "Index out of range: {index}"
            )));
        }

        self.items[index] = item;
        Ok(())
    }

    /// Adds an item to the end of the struct.
    pub fn push(&mut self, item: StackItem) {
        self.items.push(item);
    }

    /// Removes and returns the last item in the struct.
    pub fn pop(&mut self) -> VmResult<StackItem> {
        self.items
            .pop()
            .ok_or_else(|| VmError::invalid_operation_msg("Struct is empty"))
    }

    /// Gets the number of items in the struct.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the struct is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Removes all items from the struct.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Creates a deep copy of the struct.
    pub fn deep_copy(&self, reference_counter: Option<Arc<ReferenceCounter>>) -> Self {
        let items = self.items.iter().map(|item| item.deep_clone()).collect();
        Self::new(items, reference_counter)
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Struct
    }

    /// Converts the struct to a boolean.
    pub fn to_boolean(&self) -> bool {
        !self.items.is_empty()
    }
}

impl Drop for Struct {
    fn drop(&mut self) {
        // Reference cleanup is handled by the ReferenceCounter automatically
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_creation() {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let struct_item = Struct::new(items.clone(), None);

        assert_eq!(struct_item.len(), 3);
        assert_eq!(struct_item.items(), &items);
        assert_eq!(struct_item.stack_item_type(), StackItemType::Struct);
    }

    #[test]
    fn test_struct_get() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let struct_item = Struct::new(items, None);

        assert_eq!(struct_item.get(0)?.as_int().unwrap().to_i32().unwrap(), 1);
        assert_eq!(struct_item.get(1)?.as_int().unwrap().to_i32().unwrap(), 2);
        assert_eq!(struct_item.get(2)?.as_int().unwrap().to_i32().unwrap(), 3);
        assert!(struct_item.get(3).is_err());
        Ok(())
    }

    #[test]
    fn test_struct_set() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let mut struct_item = Struct::new(items, None);

        struct_item.set(1, StackItem::from_int(42))?;

        assert_eq!(struct_item.get(0)?.as_int().unwrap().to_i32().unwrap(), 1);
        assert_eq!(struct_item.get(1)?.as_int().unwrap().to_i32().unwrap(), 42);
        assert_eq!(struct_item.get(2)?.as_int().unwrap().to_i32().unwrap(), 3);
        assert!(struct_item.set(3, StackItem::from_int(4)).is_err());
        Ok(())
    }

    #[test]
    fn test_struct_push_pop() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![StackItem::from_int(1), StackItem::from_int(2)];

        let mut struct_item = Struct::new(items, None);

        struct_item.push(StackItem::from_int(3));

        assert_eq!(struct_item.len(), 3);
        assert_eq!(struct_item.get(2)?.as_int().unwrap().to_i32().unwrap(), 3);

        let popped = struct_item.pop().unwrap();

        assert_eq!(struct_item.len(), 2);
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
    fn test_struct_clear() {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let mut struct_item = Struct::new(items, None);

        struct_item.clear();

        assert_eq!(struct_item.len(), 0);
        assert!(struct_item.is_empty());
    }

    #[test]
    fn test_struct_deep_copy() -> Result<(), Box<dyn std::error::Error>> {
        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_array(vec![StackItem::from_int(3), StackItem::from_int(4)]),
        ];

        let struct_item = Struct::new(items, None);
        let copied = struct_item.deep_copy(None);

        assert_eq!(copied.len(), struct_item.len());
        assert_eq!(
            copied.get(0)?.as_int().unwrap(),
            struct_item.get(0)?.as_int().unwrap()
        );
        assert_eq!(
            copied.get(0).unwrap().as_int().unwrap(),
            struct_item.get(0).unwrap().as_int().unwrap()
        );

        // Check that the nested array was deep copied
        let nested_original = struct_item.get(2)?.as_array().unwrap();
        let nested_copied = copied.get(2)?.as_array().unwrap();

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

    #[test]
    fn test_struct_to_boolean() {
        let empty_struct = Struct::new(Vec::new(), None);
        assert_eq!(empty_struct.to_boolean(), false);

        let items = vec![StackItem::from_int(1)];
        let non_empty_struct = Struct::new(items, None);
        assert_eq!(non_empty_struct.to_boolean(), true);
    }
}

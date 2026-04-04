//! Map stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Map stack item implementation used in the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::reference_counter::ReferenceCounter;
use crate::stack_item::stack_item_type::StackItemType;
use crate::stack_item::StackItem;
use num_traits::ToPrimitive;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Represents a map of stack items in the VM.
#[derive(Debug, Clone)]
pub struct Map {
    /// The items in the map.
    items: BTreeMap<StackItem, StackItem>,
    /// The reference ID for the VM.
    reference_id: Option<usize>,
}

impl Map {
    /// Creates a new map with the specified items and reference counter.
    pub fn new(
        items: BTreeMap<StackItem, StackItem>,
        reference_counter: Option<Arc<ReferenceCounter>>,
    ) -> Self {
        let mut reference_id = None;

        if let Some(rc) = &reference_counter {
            reference_id = Some(rc.add_reference());
        }

        Self {
            items,
            reference_id,
        }
    }

    /// Gets the items in the map.
    pub fn items(&self) -> &BTreeMap<StackItem, StackItem> {
        &self.items
    }

    /// Gets a mutable reference to the items in the map.
    pub fn items_mut(&mut self) -> &mut BTreeMap<StackItem, StackItem> {
        &mut self.items
    }

    /// Gets the value for the specified key.
    pub fn get(&self, key: &StackItem) -> VmResult<&StackItem> {
        self.items
            .get(key)
            .ok_or_else(|| VmError::invalid_operation_msg(format!("Key not found: {key:?}")))
    }

    /// Sets the value for the specified key.
    pub fn set(&mut self, key: StackItem, value: StackItem) -> VmResult<()> {
        self.items.insert(key, value);
        Ok(())
    }

    /// Removes the value for the specified key.
    pub fn remove(&mut self, key: &StackItem) -> VmResult<StackItem> {
        let value = self
            .items
            .remove(key)
            .ok_or_else(|| VmError::invalid_operation_msg(format!("Key not found: {key:?}")))?;
        Ok(value)
    }

    /// Gets the number of items in the map.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Removes all items from the map.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Creates a deep copy of the map.
    pub fn deep_copy(&self, reference_counter: Option<Arc<ReferenceCounter>>) -> Self {
        let mut items = BTreeMap::new();
        for (k, v) in &self.items {
            items.insert(k.deep_clone(), v.deep_clone());
        }
        Self::new(items, reference_counter)
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Map
    }

    /// Converts the map to a boolean.
    pub fn to_boolean(&self) -> bool {
        !self.items.is_empty()
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        // Reference cleanup is handled by the ReferenceCounter automatically
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_map_creation() {
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));
        items.insert(StackItem::from_int(2), StackItem::from_int(20));

        let map = Map::new(items.clone(), None);

        assert_eq!(map.len(), 2);
        assert_eq!(map.items(), &items);
        assert_eq!(map.stack_item_type(), StackItemType::Map);
    }

    #[test]
    fn test_map_get() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));
        items.insert(StackItem::from_int(2), StackItem::from_int(20));

        let map = Map::new(items, None);

        assert_eq!(
            map.get(&StackItem::from_int(1))?
                .as_int()
                .unwrap()
                .to_i32()
                .unwrap(),
            10
        );
        assert_eq!(
            map.get(&StackItem::from_int(2))?
                .as_int()
                .unwrap()
                .to_i32()
                .unwrap(),
            20
        );
        assert!(map.get(&StackItem::from_int(3)).is_err());
        Ok(())
    }

    #[test]
    fn test_map_set() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));

        let mut map = Map::new(items, None);

        // Update existing key
        map.set(StackItem::from_int(1), StackItem::from_int(100))?;
        assert_eq!(
            map.get(&StackItem::from_int(1))?
                .as_int()
                .unwrap()
                .to_i32()
                .unwrap(),
            100
        );

        // Add new key
        map.set(StackItem::from_int(2), StackItem::from_int(20))?;
        assert_eq!(
            map.get(&StackItem::from_int(2))?
                .as_int()
                .unwrap()
                .to_i32()
                .unwrap(),
            20
        );

        assert_eq!(map.len(), 2);
        Ok(())
    }

    #[test]
    fn test_map_remove() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));
        items.insert(StackItem::from_int(2), StackItem::from_int(20));

        let mut map = Map::new(items, None);

        let removed = map.remove(&StackItem::from_int(1))?;
        assert_eq!(
            removed
                .as_int()
                .expect("intermediate value should exist")
                .to_i32()
                .unwrap(),
            10
        );
        assert_eq!(map.len(), 1);
        assert!(map.get(&StackItem::from_int(1)).is_err());

        assert!(map.remove(&StackItem::from_int(3)).is_err());
        Ok(())
    }

    #[test]
    fn test_map_clear() {
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));
        items.insert(StackItem::from_int(2), StackItem::from_int(20));

        let mut map = Map::new(items, None);

        map.clear();

        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_deep_copy() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));
        items.insert(
            StackItem::from_int(2),
            StackItem::from_array(vec![StackItem::from_int(20), StackItem::from_int(30)]),
        );

        let map = Map::new(items, None);
        let copied = map.deep_copy(None);

        assert_eq!(copied.len(), map.len());

        // Check that the nested array was deep copied
        let nested_original = map.get(&StackItem::from_int(2))?.as_array().unwrap();
        let nested_copied = copied.get(&StackItem::from_int(2))?.as_array().unwrap();

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
    fn test_map_to_boolean() {
        // Test empty map
        let empty_map = Map::new(BTreeMap::new(), None);
        assert_eq!(empty_map.to_boolean(), false);

        // Test non-empty map
        let mut items = BTreeMap::new();
        items.insert(StackItem::from_int(1), StackItem::from_int(10));
        let non_empty_map = Map::new(items, None);
        assert_eq!(non_empty_map.to_boolean(), true);
    }
}

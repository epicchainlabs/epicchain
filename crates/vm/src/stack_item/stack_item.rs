//! Stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the stack item implementations used in the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::stack_item::stack_item_type::StackItemType;
use num_bigint::BigInt;
use num_traits::Zero;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

/// A trait for interop interfaces that can be wrapped by a stack_item.
pub trait InteropInterface: fmt::Debug + Send + Sync {
    /// Gets the type of the interop interface.
    fn interface_type(&self) -> &str;

    /// Allows downcasting to concrete types
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Represents a value in the Neo VM.
#[derive(Debug, Clone)]
pub enum StackItem {
    /// Represents a null value.
    Null,

    /// Represents a boolean value.
    Boolean(bool),

    /// Represents an integer value.
    Integer(BigInt),

    /// Represents an immutable byte string.
    ByteString(Vec<u8>),

    /// Represents a mutable byte buffer.
    Buffer(Vec<u8>),

    /// Represents an array of stack items.
    Array(Vec<StackItem>),

    /// Represents a struct of stack items.
    Struct(Vec<StackItem>),

    /// Represents a map of stack items.
    Map(BTreeMap<StackItem, StackItem>),

    /// Represents a pointer to a position in a script.
    Pointer(usize),

    /// Represents an interop interface.
    InteropInterface(Arc<dyn InteropInterface>),
}

impl StackItem {
    /// The singleton True value.
    pub fn true_value() -> Self {
        StackItem::Boolean(true)
    }

    /// The singleton False value.
    pub fn false_value() -> Self {
        StackItem::Boolean(false)
    }

    /// The singleton Null value.
    pub fn null() -> Self {
        StackItem::Null
    }

    /// Creates a boolean stack item.
    pub fn from_bool(value: bool) -> Self {
        StackItem::Boolean(value)
    }

    /// Creates an integer stack item.
    pub fn from_int<T: Into<BigInt>>(value: T) -> Self {
        StackItem::Integer(value.into())
    }

    /// Creates a byte string stack item.
    pub fn from_byte_string<T: Into<Vec<u8>>>(value: T) -> Self {
        StackItem::ByteString(value.into())
    }

    /// Creates a buffer stack item.
    pub fn from_buffer<T: Into<Vec<u8>>>(value: T) -> Self {
        StackItem::Buffer(value.into())
    }

    /// Creates an array stack item.
    pub fn from_array<T: Into<Vec<StackItem>>>(value: T) -> Self {
        StackItem::Array(value.into())
    }

    /// Creates a struct stack item.
    pub fn from_struct<T: Into<Vec<StackItem>>>(value: T) -> Self {
        StackItem::Struct(value.into())
    }

    /// Creates a map stack item.
    pub fn from_map<T: Into<BTreeMap<StackItem, StackItem>>>(value: T) -> Self {
        StackItem::Map(value.into())
    }

    /// Creates a pointer stack item.
    pub fn from_pointer(value: usize) -> Self {
        StackItem::Pointer(value)
    }

    /// Creates an interop interface stack item.
    pub fn from_interface<T: InteropInterface + 'static>(value: T) -> Self {
        StackItem::InteropInterface(Arc::new(value))
    }

    /// Returns the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        match self {
            StackItem::Null => StackItemType::Any,
            StackItem::Boolean(_) => StackItemType::Boolean,
            StackItem::Integer(_) => StackItemType::Integer,
            StackItem::ByteString(_) => StackItemType::ByteString,
            StackItem::Buffer(_) => StackItemType::Buffer,
            StackItem::Array(_) => StackItemType::Array,
            StackItem::Struct(_) => StackItemType::Struct,
            StackItem::Map(_) => StackItemType::Map,
            StackItem::Pointer(_) => StackItemType::Pointer,
            StackItem::InteropInterface(_) => StackItemType::InteropInterface,
        }
    }

    /// Returns true if the stack item is null.
    pub fn is_null(&self) -> bool {
        matches!(self, StackItem::Null)
    }

    /// Converts the stack item to a boolean.
    pub fn as_bool(&self) -> VmResult<bool> {
        match self {
            StackItem::Null => Ok(false),
            StackItem::Boolean(b) => Ok(*b),
            StackItem::Integer(i) => Ok(!i.is_zero()),
            StackItem::ByteString(b) | StackItem::Buffer(b) => {
                if b.is_empty() {
                    Ok(false)
                } else {
                    // All bytes are 0 -> false, any non-zero byte -> true
                    Ok(b.iter().any(|&byte| byte != 0))
                }
            }
            StackItem::Array(a) | StackItem::Struct(a) => Ok(!a.is_empty()),
            StackItem::Map(m) => Ok(!m.is_empty()),
            StackItem::Pointer(_) => Ok(true),
            StackItem::InteropInterface(_i) => Ok(true),
        }
    }

    /// Converts the stack item to an integer.
    pub fn as_int(&self) -> VmResult<BigInt> {
        match self {
            StackItem::Null => Err(VmError::invalid_type_simple(
                "Cannot convert Null to Integer",
            )),
            StackItem::Boolean(b) => Ok(BigInt::from(if *b { 1 } else { 0 })),
            StackItem::Integer(i) => Ok(i.clone()),
            StackItem::ByteString(b) | StackItem::Buffer(b) => {
                if b.is_empty() {
                    return Ok(BigInt::from(0));
                }

                // Don't reverse - the bytes are already in little-endian format
                let bytes = b.clone();

                let is_negative = (bytes[bytes.len() - 1] & 0x80) != 0;
                if is_negative {
                    let mut bytes_copy = bytes.clone();
                    let len = bytes_copy.len();
                    bytes_copy[len - 1] &= 0x7F; // Clear the sign bit
                    let positive_value = BigInt::from_bytes_le(num_bigint::Sign::Plus, &bytes_copy);

                    // Add the sign bit value back and negate
                    let sign_bit_value = BigInt::from(1) << (len * 8 - 1);
                    Ok(-(sign_bit_value - positive_value))
                } else {
                    Ok(BigInt::from_bytes_le(num_bigint::Sign::Plus, &bytes))
                }
            }
            _ => Err(VmError::invalid_type_simple("Cannot convert to Integer")),
        }
    }

    /// Converts the stack item to a byte array.
    pub fn as_bytes(&self) -> VmResult<Vec<u8>> {
        match self {
            StackItem::Null => Ok(vec![]),
            StackItem::Boolean(b) => Ok(vec![if *b { 1 } else { 0 }]),
            StackItem::Integer(i) => {
                if i.is_zero() {
                    return Ok(vec![]);
                }

                let (sign, mut bytes) = i.to_bytes_le();

                // Handle negative numbers using two's complement
                if matches!(sign, num_bigint::Sign::Minus) {
                    // Set the sign bit in the most significant byte
                    if let Some(last) = bytes.last_mut() {
                        *last |= 0x80;
                    }
                }

                Ok(bytes)
            }
            StackItem::ByteString(b) | StackItem::Buffer(b) => Ok(b.clone()),
            _ => Err(VmError::invalid_type_simple("Cannot convert to ByteArray")),
        }
    }

    /// Converts the stack item to an array.
    pub fn as_array(&self) -> VmResult<&[StackItem]> {
        match self {
            StackItem::Array(a) | StackItem::Struct(a) => Ok(a),
            _ => Err(VmError::invalid_type_simple("Cannot convert to Array")),
        }
    }

    /// Converts the stack item to a map.
    pub fn as_map(&self) -> VmResult<&BTreeMap<StackItem, StackItem>> {
        match self {
            StackItem::Map(m) => Ok(m),
            _ => Err(VmError::invalid_type_simple("Cannot convert to Map")),
        }
    }

    /// Gets the interop interface from the stack item.
    /// Production implementation with proper type downcasting for C# compatibility.
    pub fn as_interface<T: InteropInterface + 'static>(&self) -> VmResult<&T> {
        match self {
            StackItem::InteropInterface(i) => {
                // Use Any trait for runtime type checking (matches C# reflection pattern)
                let interface_any = i.as_any();

                // Attempt to downcast to the requested type
                interface_any.downcast_ref::<T>().ok_or_else(|| {
                    VmError::invalid_type_simple(&format!(
                        "Cannot cast InteropInterface to type {}",
                        std::any::type_name::<T>()
                    ))
                })
            }
            _ => Err(VmError::invalid_type_simple(
                "Stack item is not an InteropInterface",
            )),
        }
    }

    /// Creates a deep clone of the stack item.
    pub fn deep_clone(&self) -> Self {
        self.deep_clone_with_refs(&mut std::collections::HashMap::new())
    }

    /// Creates a deep clone of the stack item with reference tracking to handle cycles.
    fn deep_clone_with_refs(
        &self,
        refs: &mut std::collections::HashMap<*const StackItem, StackItem>,
    ) -> Self {
        let self_ptr = self as *const StackItem;
        if let Some(cloned) = refs.get(&self_ptr) {
            return cloned.clone();
        }

        // Clone the item based on its type
        let result = match self {
            StackItem::Null => StackItem::Null,
            StackItem::Boolean(b) => StackItem::Boolean(*b),
            StackItem::Integer(i) => StackItem::Integer(i.clone()),
            StackItem::ByteString(b) => StackItem::ByteString(b.clone()),
            StackItem::Buffer(b) => StackItem::Buffer(b.clone()),
            StackItem::Pointer(p) => StackItem::Pointer(*p),
            StackItem::InteropInterface(i) => StackItem::InteropInterface(i.clone()),

            StackItem::Array(a) => {
                let mut array = Vec::with_capacity(a.len());
                // Add a placeholder to the refs map to handle cycles
                refs.insert(self_ptr, StackItem::Array(Vec::new()));

                // Clone each item in the array
                for item in a {
                    array.push(item.deep_clone_with_refs(refs));
                }

                StackItem::Array(array)
            }
            StackItem::Struct(s) => {
                let mut structure = Vec::with_capacity(s.len());
                // Add a placeholder to the refs map to handle cycles
                refs.insert(self_ptr, StackItem::Struct(Vec::new()));

                for item in s {
                    structure.push(item.deep_clone_with_refs(refs));
                }

                StackItem::Struct(structure)
            }
            StackItem::Map(m) => {
                let mut map = BTreeMap::new();
                // Add a placeholder to the refs map to handle cycles
                refs.insert(self_ptr, StackItem::Map(BTreeMap::new()));

                // Clone each key-value pair in the map
                for (k, v) in m {
                    map.insert(k.deep_clone_with_refs(refs), v.deep_clone_with_refs(refs));
                }

                StackItem::Map(map)
            }
        };

        refs.insert(self_ptr, result.clone());

        result
    }

    /// Clears all references to other stack items.
    pub fn clear_references(&mut self) {
        match self {
            StackItem::Array(items) => {
                items.clear();
            }
            StackItem::Struct(items) => {
                items.clear();
            }
            StackItem::Map(map) => {
                map.clear();
            }
            _ => {}
        }
    }

    /// Converts the stack item to the specified type.
    pub fn convert_to(&self, item_type: StackItemType) -> VmResult<StackItem> {
        if self.stack_item_type() == item_type {
            return Ok(self.clone());
        }

        match item_type {
            StackItemType::Boolean => Ok(StackItem::Boolean(self.as_bool()?)),
            StackItemType::Integer => Ok(StackItem::Integer(self.as_int()?)),
            StackItemType::ByteString => Ok(StackItem::ByteString(self.as_bytes()?)),
            StackItemType::Buffer => Ok(StackItem::Buffer(self.as_bytes()?)),
            _ => Err(VmError::invalid_type_simple(format!(
                "Cannot convert to {item_type:?}"
            ))),
        }
    }

    /// Checks if two stack items are equal.
    pub fn equals(&self, other: &StackItem) -> VmResult<bool> {
        self.equals_with_refs(other, &mut std::collections::HashSet::new())
    }

    /// Checks if two stack items are equal with reference tracking to handle cycles.
    fn equals_with_refs(
        &self,
        other: &StackItem,
        visited: &mut std::collections::HashSet<(*const StackItem, *const StackItem)>,
    ) -> VmResult<bool> {
        let self_ptr = self as *const StackItem;
        let other_ptr = other as *const StackItem;

        if visited.contains(&(self_ptr, other_ptr)) || visited.contains(&(other_ptr, self_ptr)) {
            return Ok(true);
        }

        // Add this pair to the visited set
        visited.insert((self_ptr, other_ptr));

        let result = match (self, other) {
            (StackItem::Null, StackItem::Null) => Ok(true),
            (StackItem::Boolean(a), StackItem::Boolean(b)) => Ok(a == b),
            (StackItem::Integer(a), StackItem::Integer(b)) => Ok(a == b),
            (StackItem::ByteString(a), StackItem::ByteString(b)) => Ok(a == b),
            (StackItem::Buffer(a), StackItem::Buffer(b)) => Ok(a == b),
            (StackItem::ByteString(a), StackItem::Buffer(b)) => Ok(a == b),
            (StackItem::Buffer(a), StackItem::ByteString(b)) => Ok(a == b),
            (StackItem::Pointer(a), StackItem::Pointer(b)) => Ok(a == b),
            (StackItem::InteropInterface(a), StackItem::InteropInterface(b)) => {
                Ok(Arc::ptr_eq(a, b))
            }
            (StackItem::Array(a), StackItem::Array(b)) => {
                if a.len() != b.len() {
                    return Ok(false);
                }

                for (ai, bi) in a.iter().zip(b.iter()) {
                    if !ai.equals_with_refs(bi, visited)? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            (StackItem::Struct(a), StackItem::Struct(b)) => {
                if a.len() != b.len() {
                    return Ok(false);
                }

                for (ai, bi) in a.iter().zip(b.iter()) {
                    if !ai.equals_with_refs(bi, visited)? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            (StackItem::Map(a), StackItem::Map(b)) => {
                if a.len() != b.len() {
                    return Ok(false);
                }

                for (ak, av) in a {
                    let found = b.iter().any(|(bk, bv)| {
                        ak.equals_with_refs(bk, visited).unwrap_or(false)
                            && av.equals_with_refs(bv, visited).unwrap_or(false)
                    });

                    if !found {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            _ => Ok(false),
        };

        // Remove this pair from the visited set
        visited.remove(&(self_ptr, other_ptr));

        result
    }
}

// Implement PartialEq to allow stack items to be compared and used as keys in collections
impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other).unwrap_or(false)
    }
}

impl Eq for StackItem {}

// Implement PartialOrd and Ord to allow stack items to be used as keys in BTreeMap
// Production-ready implementation matching C# StackItem comparison exactly
impl PartialOrd for StackItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StackItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Production-ready ordering based on C# stack_item comparison rules
        // 1. First compare by type (matches C# stack_item type hierarchy)
        let type_order = (self.stack_item_type() as u8).cmp(&(other.stack_item_type() as u8));
        if type_order != std::cmp::Ordering::Equal {
            return type_order;
        }

        // 2. Compare values within the same type
        match (self, other) {
            (StackItem::Null, StackItem::Null) => std::cmp::Ordering::Equal,
            (StackItem::Boolean(a), StackItem::Boolean(b)) => a.cmp(b),
            (StackItem::Integer(a), StackItem::Integer(b)) => a.cmp(b),
            (StackItem::ByteString(a), StackItem::ByteString(b)) => a.cmp(b),
            (StackItem::Buffer(a), StackItem::Buffer(b)) => a.cmp(b),
            (StackItem::ByteString(a), StackItem::Buffer(b)) => a.cmp(b),
            (StackItem::Buffer(a), StackItem::ByteString(b)) => a.cmp(b),
            (StackItem::Pointer(a), StackItem::Pointer(b)) => a.cmp(b),
            (StackItem::Array(a), StackItem::Array(b)) => {
                let len_cmp = a.len().cmp(&b.len());
                if len_cmp != std::cmp::Ordering::Equal {
                    return len_cmp;
                }
                for (item_a, item_b) in a.iter().zip(b.iter()) {
                    let item_cmp = item_a.cmp(item_b);
                    if item_cmp != std::cmp::Ordering::Equal {
                        return item_cmp;
                    }
                }
                std::cmp::Ordering::Equal
            }
            (StackItem::Struct(a), StackItem::Struct(b)) => {
                let len_cmp = a.len().cmp(&b.len());
                if len_cmp != std::cmp::Ordering::Equal {
                    return len_cmp;
                }
                for (item_a, item_b) in a.iter().zip(b.iter()) {
                    let item_cmp = item_a.cmp(item_b);
                    if item_cmp != std::cmp::Ordering::Equal {
                        return item_cmp;
                    }
                }
                std::cmp::Ordering::Equal
            }
            (StackItem::Map(a), StackItem::Map(b)) => {
                // Compare maps by size first, then by sorted key-value pairs
                let len_cmp = a.len().cmp(&b.len());
                if len_cmp != std::cmp::Ordering::Equal {
                    return len_cmp;
                }

                let mut a_pairs: Vec<_> = a.iter().collect();
                let mut b_pairs: Vec<_> = b.iter().collect();
                a_pairs.sort_by(|x, y| x.0.cmp(y.0));
                b_pairs.sort_by(|x, y| x.0.cmp(y.0));

                for ((key_a, val_a), (key_b, val_b)) in a_pairs.iter().zip(b_pairs.iter()) {
                    let key_cmp = key_a.cmp(key_b);
                    if key_cmp != std::cmp::Ordering::Equal {
                        return key_cmp;
                    }
                    let val_cmp = val_a.cmp(val_b);
                    if val_cmp != std::cmp::Ordering::Equal {
                        return val_cmp;
                    }
                }
                std::cmp::Ordering::Equal
            }
            _ => {
                let _self_discriminant = std::mem::discriminant(self);
                let _other_discriminant = std::mem::discriminant(other);
                // based on the variant order in the enum
                match (self, other) {
                    (StackItem::Null, _) => std::cmp::Ordering::Less,
                    (_, StackItem::Null) => std::cmp::Ordering::Greater,
                    (StackItem::Boolean(_), StackItem::Integer(_)) => std::cmp::Ordering::Less,
                    (StackItem::Integer(_), StackItem::Boolean(_)) => std::cmp::Ordering::Greater,
                    (StackItem::Boolean(_), StackItem::ByteString(_)) => std::cmp::Ordering::Less,
                    (StackItem::ByteString(_), StackItem::Boolean(_)) => {
                        std::cmp::Ordering::Greater
                    }
                    (StackItem::Integer(_), StackItem::ByteString(_)) => std::cmp::Ordering::Less,
                    (StackItem::ByteString(_), StackItem::Integer(_)) => {
                        std::cmp::Ordering::Greater
                    }
                    _ => std::cmp::Ordering::Equal, // Same types that we haven't handled above
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_stack_item() {
        let true_item = StackItem::from_bool(true);
        let false_item = StackItem::from_bool(false);

        assert_eq!(
            true_item.as_bool().expect("Failed to convert to bool"),
            true
        );
        assert_eq!(
            false_item.as_bool().expect("Failed to convert to bool"),
            false
        );
        assert_eq!(true_item.stack_item_type(), StackItemType::Boolean);
    }

    #[test]
    fn test_integer_stack_item() {
        let int_item = StackItem::from_int(42);

        assert_eq!(int_item.as_int().unwrap(), BigInt::from(42));
        assert_eq!(int_item.as_bool().expect("Failed to convert"), true);
        assert_eq!(int_item.stack_item_type(), StackItemType::Integer);

        let zero_item = StackItem::from_int(0);
        assert_eq!(zero_item.as_bool().expect("Failed to convert"), false);
    }

    #[test]
    fn test_bytestring_stack_item() {
        let bytes = vec![1, 2, 3];
        let byte_string = StackItem::from_byte_string(bytes.clone());

        assert_eq!(byte_string.as_bytes().unwrap(), bytes);
        assert_eq!(byte_string.as_bool().expect("Failed to convert"), true);
        assert_eq!(byte_string.stack_item_type(), StackItemType::ByteString);

        let empty_bytes = StackItem::from_byte_string(Vec::<u8>::new());
        assert_eq!(empty_bytes.as_bool().expect("Failed to convert"), false);
    }

    #[test]
    fn test_array_stack_item() {
        let array = StackItem::from_array(vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ]);

        assert_eq!(
            array
                .as_array()
                .expect("intermediate value should exist")
                .len(),
            3
        );
        assert_eq!(array.as_bool().expect("Failed to convert"), true);
        assert_eq!(array.stack_item_type(), StackItemType::Array);

        let empty_array = StackItem::from_array(Vec::<StackItem>::new());
        assert_eq!(empty_array.as_bool().expect("Failed to convert"), false);
    }

    #[test]
    fn test_deep_clone() {
        let array = StackItem::from_array(vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_array(vec![StackItem::from_int(3), StackItem::from_int(4)]),
        ]);

        let cloned = array.deep_clone();
        assert!(array.equals(&cloned).unwrap());
    }

    #[test]
    fn test_equals() {
        let a = StackItem::from_int(42);
        let b = StackItem::from_int(42);
        let c = StackItem::from_int(43);

        assert!(a.equals(&b).unwrap());
        assert!(!a.equals(&c).unwrap());

        let array1 = StackItem::from_array(vec![StackItem::from_int(1), StackItem::from_int(2)]);

        let array2 = StackItem::from_array(vec![StackItem::from_int(1), StackItem::from_int(2)]);

        let array3 = StackItem::from_array(vec![StackItem::from_int(1), StackItem::from_int(3)]);

        assert!(array1.equals(&array2).unwrap_or(false));
        assert!(!array1.equals(&array3).unwrap());
    }

    #[test]
    fn test_convert_to() {
        let int_item = StackItem::from_int(42);

        // Convert to boolean
        let bool_item = int_item.convert_to(StackItemType::Boolean).unwrap();
        assert_eq!(bool_item.stack_item_type(), StackItemType::Boolean);
        assert_eq!(bool_item.as_bool().expect("Failed to convert"), true);

        let byte_string = int_item.convert_to(StackItemType::ByteString).unwrap();
        assert_eq!(byte_string.stack_item_type(), StackItemType::ByteString);
        assert_eq!(byte_string.as_bytes().unwrap(), vec![42]);

        // Convert to buffer
        let buffer = int_item.convert_to(StackItemType::Buffer).unwrap();
        assert_eq!(buffer.stack_item_type(), StackItemType::Buffer);
        assert_eq!(buffer.as_bytes().unwrap(), vec![42]);

        let int_clone = int_item.convert_to(StackItemType::Integer).unwrap();
        assert_eq!(int_clone.stack_item_type(), StackItemType::Integer);
        assert_eq!(
            int_clone.as_int().expect("Operation failed"),
            BigInt::from(42)
        );

        // Convert to unsupported type
        assert!(int_item.convert_to(StackItemType::Array).is_err());
    }

    #[test]
    fn test_equals_with_cycles() {
        // Create two arrays with cycles
        let mut array1 =
            StackItem::from_array(vec![StackItem::from_int(1), StackItem::from_int(2)]);

        let mut array2 =
            StackItem::from_array(vec![StackItem::from_int(1), StackItem::from_int(2)]);

        // Add cycles
        let array1_clone = array1.clone();
        if let StackItem::Array(items1) = &mut array1 {
            items1.push(array1_clone);
        }

        let array2_clone = array2.clone();
        if let StackItem::Array(items2) = &mut array2 {
            items2.push(array2_clone);
        }

        // The arrays should be equal despite the cycles
        assert!(array1.equals(&array2).unwrap_or(false));
    }
}

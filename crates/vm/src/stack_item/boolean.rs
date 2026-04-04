//! Boolean stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Boolean stack item implementation used in the Neo VM.

use crate::stack_item::stack_item_type::StackItemType;
use num_bigint::BigInt;

/// Represents a boolean value in the VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    /// The boolean value.
    value: bool,
}

impl Boolean {
    /// The singleton True value.
    pub fn true_value() -> Self {
        Self { value: true }
    }

    /// The singleton False value.
    pub fn false_value() -> Self {
        Self { value: false }
    }

    /// Creates a new boolean with the specified value.
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    /// Gets the boolean value.
    pub fn value(&self) -> bool {
        self.value
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Boolean
    }

    /// Converts the boolean to an integer.
    pub fn to_integer(&self) -> BigInt {
        if self.value {
            BigInt::from(1)
        } else {
            BigInt::from(0)
        }
    }

    /// Converts the boolean to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        if self.value {
            vec![1]
        } else {
            vec![0]
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_creation() {
        let true_bool = Boolean::new(true);
        let false_bool = Boolean::new(false);

        assert_eq!(true_bool.value(), true);
        assert_eq!(false_bool.value(), false);
        assert_eq!(true_bool.stack_item_type(), StackItemType::Boolean);
        assert_eq!(false_bool.stack_item_type(), StackItemType::Boolean);
    }

    #[test]
    fn test_boolean_to_integer() {
        let true_bool = Boolean::new(true);
        let false_bool = Boolean::new(false);

        assert_eq!(true_bool.to_integer(), BigInt::from(1));
        assert_eq!(false_bool.to_integer(), BigInt::from(0));
    }

    #[test]
    fn test_boolean_to_bytes() {
        let true_bool = Boolean::new(true);
        let false_bool = Boolean::new(false);

        assert_eq!(true_bool.to_bytes(), vec![1]);
        assert_eq!(false_bool.to_bytes(), vec![0]);
    }

    #[test]
    fn test_boolean_singleton() {
        let true_bool = Boolean::true_value();
        let false_bool = Boolean::false_value();

        assert_eq!(true_bool.value(), true);
        assert_eq!(false_bool.value(), false);
    }
}

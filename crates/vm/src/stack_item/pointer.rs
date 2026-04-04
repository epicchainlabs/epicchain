//! Pointer stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Pointer stack item implementation used in the Neo VM.

use crate::stack_item::stack_item_type::StackItemType;
use num_bigint::BigInt;

/// Represents a pointer to a position in a script in the VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pointer {
    /// The position in the script.
    position: usize,
}

impl Pointer {
    /// Creates a new pointer with the specified position.
    pub fn new(position: usize) -> Self {
        Self { position }
    }

    /// Gets the position in the script.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Pointer
    }

    /// Converts the pointer to a boolean.
    pub fn to_boolean(&self) -> bool {
        true
    }

    /// Converts the pointer to an integer.
    pub fn to_integer(&self) -> BigInt {
        BigInt::from(self.position)
    }

    /// Creates a deep copy of the pointer.
    pub fn deep_copy(&self) -> Self {
        Self::new(self.position)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_creation() {
        let pointer = Pointer::new(42);

        assert_eq!(pointer.position(), 42);
        assert_eq!(pointer.stack_item_type(), StackItemType::Pointer);
    }

    #[test]
    fn test_pointer_to_boolean() {
        let pointer = Pointer::new(42);

        assert_eq!(pointer.to_boolean(), true);
    }

    #[test]
    fn test_pointer_to_integer() {
        let pointer = Pointer::new(42);

        assert_eq!(pointer.to_integer(), BigInt::from(42));
    }

    #[test]
    fn test_pointer_deep_copy() {
        let pointer = Pointer::new(42);
        let copied = pointer.deep_copy();

        assert_eq!(copied.position(), pointer.position());
    }
}

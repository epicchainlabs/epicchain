//! Null stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Null stack item implementation used in the Neo VM.

use crate::stack_item::stack_item_type::StackItemType;

/// Represents a null value in the VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Null;

impl Null {
    /// Creates a new null value.
    pub fn new() -> Self {
        Self
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Any
    }

    /// Converts the null to a boolean.
    pub fn to_boolean(&self) -> bool {
        false
    }

    /// Converts the null to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Creates a deep copy of the null.
    pub fn deep_copy(&self) -> Self {
        Self
    }
}

impl Default for Null {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_null_creation() {
        let null = Null::new();

        assert_eq!(null.stack_item_type(), StackItemType::Any);
    }

    #[test]
    fn test_null_to_boolean() {
        let null = Null::new();

        assert_eq!(null.to_boolean(), false);
    }

    #[test]
    fn test_null_to_bytes() {
        let null = Null::new();

        assert_eq!(null.to_bytes(), Vec::<u8>::new());
    }

    #[test]
    fn test_null_deep_copy() {
        let null = Null::new();
        let copied = null.deep_copy();

        assert_eq!(copied, null);
    }
}

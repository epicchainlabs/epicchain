//! Integer stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Integer stack item implementation used in the Neo VM.

use crate::stack_item::stack_item_type::StackItemType;
use num_bigint::BigInt;
use num_traits::Zero;

/// Represents an integer value in the VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    /// The integer value.
    value: BigInt,
}

impl Integer {
    /// Creates a new integer with the specified value.
    pub fn new<T: Into<BigInt>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Gets the integer value.
    pub fn value(&self) -> &BigInt {
        &self.value
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Integer
    }

    /// Converts the integer to a boolean.
    pub fn to_boolean(&self) -> bool {
        !self.value.is_zero()
    }

    /// Converts the integer to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Convert BigInt to little-endian byte array
        let (sign, mut bytes) = self.value.to_bytes_le();

        // Handle negative numbers
        if matches!(sign, num_bigint::Sign::Minus) {
            if let Some(last) = bytes.last_mut() {
                *last |= 0x80;
            }
        }

        bytes
    }

    /// Creates a deep copy of the integer.
    pub fn deep_copy(&self) -> Self {
        Self::new(self.value.clone())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_creation() {
        let int = Integer::new(42);

        assert_eq!(int.value(), &BigInt::from(42));
        assert_eq!(int.stack_item_type(), StackItemType::Integer);
    }

    #[test]
    fn test_integer_to_boolean() {
        let zero = Integer::new(0);
        let nonzero = Integer::new(42);
        let negative = Integer::new(-1);

        assert_eq!(zero.to_boolean(), false);
        assert_eq!(nonzero.to_boolean(), true);
        assert_eq!(negative.to_boolean(), true);
    }

    #[test]
    fn test_integer_to_bytes() {
        // Test zero
        let zero = Integer::new(0);
        assert_eq!(zero.to_bytes(), vec![0]);

        // Test small positive number
        let small = Integer::new(1);
        assert_eq!(small.to_bytes(), vec![1]);

        // Test larger positive number
        let larger = Integer::new(0xABCD);
        assert_eq!(larger.to_bytes(), vec![0xCD, 0xAB]);

        // Test negative number
        let negative = Integer::new(-1);
        let bytes = negative.to_bytes();
        assert!(bytes.len() > 0);
        assert_eq!(bytes[bytes.len() - 1] & 0x80, 0x80); // Check sign bit
    }

    #[test]
    fn test_integer_deep_copy() {
        let int = Integer::new(42);
        let copied = int.deep_copy();

        assert_eq!(copied.value(), int.value());
    }

    #[test]
    fn test_integer_from_various_types() {
        // Test from i32
        let from_i32 = Integer::new(42_i32);
        assert_eq!(from_i32.value(), &BigInt::from(42));

        // Test from i64
        let from_i64 = Integer::new(42_i64);
        assert_eq!(from_i64.value(), &BigInt::from(42));

        // Test from u32
        let from_u32 = Integer::new(42_u32);
        assert_eq!(from_u32.value(), &BigInt::from(42));

        // Test from u64
        let from_u64 = Integer::new(42_u64);
        assert_eq!(from_u64.value(), &BigInt::from(42));

        // Test from BigInt
        let from_bigint = Integer::new(BigInt::from(42));
        assert_eq!(from_bigint.value(), &BigInt::from(42));
    }
}

//! ByteString stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the ByteString stack item implementation used in the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::stack_item::stack_item_type::StackItemType;
use num_bigint::BigInt;

/// Represents an immutable byte string in the VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteString {
    /// The byte string data.
    data: Vec<u8>,
}

impl ByteString {
    /// Creates a new byte string with the specified data.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Creates a new byte string from a string.
    pub fn from_string(s: &str) -> Self {
        Self {
            data: s.as_bytes().to_vec(),
        }
    }

    /// Gets the byte string data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::ByteString
    }

    /// Gets the length of the byte string.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the byte string is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets the byte at the specified index.
    pub fn get(&self, index: usize) -> VmResult<u8> {
        self.data
            .get(index)
            .copied()
            .ok_or_else(|| VmError::invalid_operation_msg(format!("Index out of range: {index}")))
    }

    /// Converts the byte string to an integer.
    ///
    /// This matches the C# Neo implementation exactly:
    /// - Uses little-endian byte order (no reversal needed)
    /// - Handles negative numbers using .NET BigInteger format (not two's complement)
    pub fn to_integer(&self) -> VmResult<BigInt> {
        if self.data.is_empty() {
            return Ok(BigInt::from(0));
        }

        // The C# BigInteger constructor interprets the bytes as signed little-endian
        // but uses a special format where the sign bit indicates negativity
        let bytes = &self.data;

        let is_negative = (bytes[bytes.len() - 1] & 0x80) != 0;

        if is_negative {
            // The magnitude is stored in the bytes with the sign bit cleared
            let mut magnitude_bytes = bytes.to_vec();
            let len = magnitude_bytes.len();
            magnitude_bytes[len - 1] &= 0x7F; // Clear the sign bit

            // Create the positive magnitude
            let magnitude = BigInt::from_bytes_le(num_bigint::Sign::Plus, &magnitude_bytes);

            // Return the negative value
            Ok(-magnitude)
        } else {
            Ok(BigInt::from_bytes_le(num_bigint::Sign::Plus, bytes))
        }
    }

    /// Converts the byte string to a boolean.
    pub fn to_boolean(&self) -> bool {
        if self.data.is_empty() {
            return false;
        }

        // All bytes are 0 -> false, any non-zero byte -> true
        self.data.iter().any(|&byte| byte != 0)
    }

    /// Converts the byte string to a UTF-8 string if possible.
    pub fn to_string(&self) -> VmResult<String> {
        String::from_utf8(self.data.clone())
            .map_err(|e| VmError::invalid_operation_msg(format!("Invalid UTF-8 sequence: {e}")))
    }

    /// Creates a deep copy of the byte string.
    pub fn deep_copy(&self) -> Self {
        Self::new(self.data.clone())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_string_creation() {
        let data = vec![1, 2, 3];
        let byte_string = ByteString::new(data.clone());

        assert_eq!(byte_string.data(), &data);
        assert_eq!(byte_string.len(), 3);
        assert_eq!(byte_string.stack_item_type(), StackItemType::ByteString);
    }

    #[test]
    fn test_byte_string_from_string() {
        let s = "Hello, world!";
        let byte_string = ByteString::from_string(s);

        assert_eq!(byte_string.data(), s.as_bytes());
        assert_eq!(byte_string.len(), s.len());
    }

    #[test]
    fn test_byte_string_get() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec![1, 2, 3];
        let byte_string = ByteString::new(data);

        assert_eq!(byte_string.data().first().ok_or("Empty collection")?, &1);
        assert_eq!(byte_string.get(1)?, 2);
        assert_eq!(byte_string.get(2)?, 3);
        assert!(byte_string.get(3).is_err());
        Ok(())
    }

    #[test]
    fn test_byte_string_to_integer() -> Result<(), Box<dyn std::error::Error>> {
        let empty_byte_string = ByteString::new(vec![]);
        assert_eq!(empty_byte_string.to_integer()?, BigInt::from(0));

        // Test positive number
        let positive_byte_string = ByteString::new(vec![1, 0, 0, 0]);
        assert_eq!(positive_byte_string.to_integer()?, BigInt::from(1));

        // Test larger positive number
        let larger_byte_string = ByteString::new(vec![0xCD, 0xAB, 0, 0]);
        assert_eq!(larger_byte_string.to_integer()?, BigInt::from(0xABCD));

        // Test negative number
        let negative_byte_string = ByteString::new(vec![1, 0, 0, 0x80]);
        assert_eq!(negative_byte_string.to_integer()?, BigInt::from(-1));
        Ok(())
    }

    #[test]
    fn test_byte_string_to_boolean() {
        let empty_byte_string = ByteString::new(vec![]);
        assert_eq!(empty_byte_string.to_boolean(), false);

        let zero_byte_string = ByteString::new(vec![0, 0, 0]);
        assert_eq!(zero_byte_string.to_boolean(), false);

        let nonzero_byte_string = ByteString::new(vec![0, 1, 0]);
        assert_eq!(nonzero_byte_string.to_boolean(), true);
    }

    #[test]
    fn test_byte_string_to_string() {
        // Test valid UTF-8
        let hello_byte_string = ByteString::from_string("Hello, world!");
        assert_eq!(
            hello_byte_string.to_string().expect("valid utf8"),
            "Hello, world!"
        );

        // Test invalid UTF-8
        let invalid_utf8 = ByteString::new(vec![0xFF, 0xFF]);
        assert!(invalid_utf8.to_string().is_err());
    }

    #[test]
    fn test_byte_string_deep_copy() {
        let data = vec![1, 2, 3];
        let byte_string = ByteString::new(data.clone());
        let copied = byte_string.deep_copy();

        assert_eq!(copied.data(), byte_string.data());
    }
}

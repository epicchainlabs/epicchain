//! Buffer stack item implementation for the Neo Virtual Machine.
//!
//! This module provides the Buffer stack item implementation used in the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::stack_item::stack_item_type::StackItemType;
use num_bigint::BigInt;

/// Represents a mutable byte buffer in the VM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    /// The buffer data.
    data: Vec<u8>,
}

impl Buffer {
    /// Creates a new buffer with the specified data.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Gets the buffer data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Gets a mutable reference to the buffer data.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Gets the type of the stack item.
    pub fn stack_item_type(&self) -> StackItemType {
        StackItemType::Buffer
    }

    /// Gets the length of the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the buffer is empty.
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

    /// Sets the byte at the specified index.
    pub fn set(&mut self, index: usize, value: u8) -> VmResult<()> {
        if index >= self.data.len() {
            return Err(VmError::invalid_operation_msg(format!(
                "Index out of range: {index}"
            )));
        }

        self.data[index] = value;
        Ok(())
    }

    /// Converts the buffer to an integer.
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

    /// Converts the buffer to a boolean.
    pub fn to_boolean(&self) -> bool {
        if self.data.is_empty() {
            return false;
        }

        // All bytes are 0 -> false, any non-zero byte -> true
        self.data.iter().any(|&byte| byte != 0)
    }

    /// Creates a deep copy of the buffer.
    pub fn deep_copy(&self) -> Self {
        Self::new(self.data.clone())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let data = vec![1, 2, 3];
        let buffer = Buffer::new(data.clone());

        assert_eq!(buffer.data(), &data);
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.stack_item_type(), StackItemType::Buffer);
    }

    #[test]
    fn test_buffer_get_set() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec![1, 2, 3];
        let mut buffer = Buffer::new(data);

        assert_eq!(*buffer.data.first().ok_or("Empty collection")?, 1);
        assert_eq!(buffer.get(1)?, 2);
        assert_eq!(buffer.get(2)?, 3);
        assert!(buffer.get(3).is_err());

        buffer.set(1, 42).unwrap();

        assert_eq!(*buffer.data.first().ok_or("Empty collection")?, 1);
        assert_eq!(buffer.get(1)?, 42);
        assert_eq!(buffer.get(2)?, 3);
        assert!(buffer.set(3, 4).is_err());
        Ok(())
    }

    #[test]
    fn test_buffer_to_integer() -> Result<(), Box<dyn std::error::Error>> {
        // Test empty buffer
        let empty_buffer = Buffer::new(vec![]);
        assert_eq!(empty_buffer.to_integer()?, BigInt::from(0));

        // Test positive number
        let positive_buffer = Buffer::new(vec![1, 0, 0, 0]);
        assert_eq!(positive_buffer.to_integer()?, BigInt::from(1));

        // Test larger positive number
        let larger_buffer = Buffer::new(vec![0xCD, 0xAB, 0, 0]);
        assert_eq!(larger_buffer.to_integer()?, BigInt::from(0xABCD));

        // Test negative number
        let negative_buffer = Buffer::new(vec![1, 0, 0, 0x80]);
        assert_eq!(negative_buffer.to_integer()?, BigInt::from(-1));
        Ok(())
    }

    #[test]
    fn test_buffer_to_boolean() {
        // Test empty buffer
        let empty_buffer = Buffer::new(vec![]);
        assert_eq!(empty_buffer.to_boolean(), false);

        // Test buffer with all zeros
        let zero_buffer = Buffer::new(vec![0, 0, 0]);
        assert_eq!(zero_buffer.to_boolean(), false);

        // Test buffer with non-zero byte
        let nonzero_buffer = Buffer::new(vec![0, 1, 0]);
        assert_eq!(nonzero_buffer.to_boolean(), true);
    }

    #[test]
    fn test_buffer_deep_copy() {
        let data = vec![1, 2, 3];
        let buffer = Buffer::new(data.clone());
        let copied = buffer.deep_copy();

        assert_eq!(copied.data(), buffer.data());

        // Ensure it's a deep copy
        let mut copied_data = copied.data().to_vec();
        copied_data[0] = 42;

        assert_ne!(copied_data, buffer.data());
    }
}

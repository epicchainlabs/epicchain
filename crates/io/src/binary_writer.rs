//! Binary writer implementation for Neo.
//!
//! This module provides a binary writer for serializing Neo data structures.

use crate::{Result, Serializable};
use bytes::{BufMut, BytesMut};
use std::io::Write;

/// A writer for serializing Neo data structures to binary data.
pub struct BinaryWriter {
    /// The buffer being written to
    buffer: BytesMut,
}

impl BinaryWriter {
    /// Creates a new binary writer.
    ///
    /// # Returns
    ///
    /// A new binary writer
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }

    /// Creates a new binary writer with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The initial capacity of the buffer
    ///
    /// # Returns
    ///
    /// A new binary writer with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: BytesMut::with_capacity(capacity),
        }
    }

    /// Returns the current position in the buffer.
    pub fn position(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the length of the buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Writes a single byte to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_byte(&mut self, value: u8) -> Result<usize> {
        self.buffer.put_u8(value);
        Ok(1)
    }

    /// Writes an unsigned byte to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The unsigned byte to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_u8(&mut self, value: u8) -> Result<usize> {
        self.write_byte(value)
    }

    /// Writes a boolean value to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The boolean value to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_bool(&mut self, value: bool) -> Result<usize> {
        self.write_byte(if value { 1 } else { 0 })
    }

    /// Writes a signed byte to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The signed byte to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_i8(&mut self, value: i8) -> Result<usize> {
        self.write_byte(value as u8)
    }

    /// Writes an unsigned 16-bit integer to the buffer in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `value` - The unsigned 16-bit integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_u16(&mut self, value: u16) -> Result<usize> {
        self.buffer.put_u16_le(value);
        Ok(2)
    }

    /// Writes a signed 16-bit integer to the buffer in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `value` - The signed 16-bit integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_i16(&mut self, value: i16) -> Result<usize> {
        self.buffer.put_i16_le(value);
        Ok(2)
    }

    /// Writes an unsigned HASH_SIZE-bit integer to the buffer in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `value` - The unsigned HASH_SIZE-bit integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_u32(&mut self, value: u32) -> Result<usize> {
        self.buffer.put_u32_le(value);
        Ok(4)
    }

    /// Writes a signed HASH_SIZE-bit integer to the buffer in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `value` - The signed HASH_SIZE-bit integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_i32(&mut self, value: i32) -> Result<usize> {
        self.buffer.put_i32_le(value);
        Ok(4)
    }

    /// Writes an unsigned 64-bit integer to the buffer in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `value` - The unsigned 64-bit integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_u64(&mut self, value: u64) -> Result<usize> {
        self.buffer.put_u64_le(value);
        Ok(8)
    }

    /// Writes a signed 64-bit integer to the buffer in little-endian format.
    ///
    /// # Arguments
    ///
    /// * `value` - The signed 64-bit integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_i64(&mut self, value: i64) -> Result<usize> {
        self.buffer.put_i64_le(value);
        Ok(8)
    }

    /// Writes a variable-length integer to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The variable-length integer to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_var_int(&mut self, value: u64) -> Result<usize> {
        if value < 0xfd {
            self.write_byte(value as u8)
        } else if value <= 0xffff {
            self.write_byte(0xfd)?;
            self.write_u16(value as u16)?;
            Ok(3)
        } else if value <= 0xffffffff {
            self.write_byte(0xfe)?;
            self.write_u32(value as u32)?;
            Ok(5)
        } else {
            self.write_byte(0xff)?;
            self.write_u64(value)?;
            Ok(9)
        }
    }

    /// Writes a variable-length byte array to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The variable-length byte array to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_var_bytes(&mut self, value: &[u8]) -> Result<usize> {
        let length = self.write_var_int(value.len() as u64)?;
        self.write_bytes(value)?;
        Ok(length + value.len())
    }

    /// Writes a fixed-length byte array to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The fixed-length byte array to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_bytes(&mut self, value: &[u8]) -> Result<usize> {
        self.buffer.put_slice(value);
        Ok(value.len())
    }

    /// Writes a variable-length string to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The variable-length string to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_var_string(&mut self, value: &str) -> Result<usize> {
        self.write_var_bytes(value.as_bytes())
    }

    /// Writes a serializable object to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The serializable object to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_serializable<T: Serializable>(&mut self, value: &T) -> Result<usize> {
        let start = self.position();
        value.serialize(self)?;
        Ok(self.position() - start)
    }

    /// Writes a variable-length array of serializable objects to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The variable-length array of serializable objects to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_serializable_list<T: Serializable>(&mut self, value: &[T]) -> Result<usize> {
        let start = self.position();
        self.write_var_int(value.len() as u64)?;

        for item in value {
            self.write_serializable(item)?;
        }

        Ok(self.position() - start)
    }

    /// Writes a fixed-length array of serializable objects to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The fixed-length array of serializable objects to write
    ///
    /// # Returns
    ///
    /// The number of bytes written
    pub fn write_serializable_fixed<T: Serializable>(&mut self, value: &[T]) -> Result<usize> {
        let start = self.position();

        for item in value {
            self.write_serializable(item)?;
        }

        Ok(self.position() - start)
    }

    /// Returns the buffer as a byte vector.
    ///
    /// # Returns
    ///
    /// The buffer as a byte vector
    pub fn to_bytes(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Reserves capacity for at least `additional` more bytes.
    ///
    /// # Arguments
    ///
    /// * `additional` - The number of additional bytes to reserve
    pub fn reserve(&mut self, additional: usize) {
        self.buffer.reserve(additional);
    }
}

impl Default for BinaryWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for BinaryWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_bytes(buf)
            .map_err(|e| std::io::Error::other(e.to_string()))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

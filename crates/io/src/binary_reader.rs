//! Binary reader implementation for Neo.
//!
//! This module provides a binary reader for deserializing Neo data structures.

use crate::{Error, Result, Serializable};
use bytes::Bytes;

/// A reader for deserializing Neo data structures from binary data.
pub struct BinaryReader {
    /// The data being read
    data: Bytes,

    /// The current position in the data
    position: usize,
}

impl BinaryReader {
    /// Creates a new binary reader from the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to read from
    ///
    /// # Returns
    ///
    /// A new binary reader
    pub fn new(data: impl Into<Bytes>) -> Self {
        Self {
            data: data.into(),
            position: 0,
        }
    }

    /// Returns the current position in the data.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns the length of the data.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns whether the end of the data has been reached.
    pub fn is_eof(&self) -> bool {
        self.position >= self.data.len()
    }

    /// Returns the remaining data.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    /// Reads a single byte from the data.
    ///
    /// # Returns
    ///
    /// The byte read or an error if the end of the data has been reached
    pub fn read_byte(&mut self) -> Result<u8> {
        if self.is_eof() {
            return Err(Error::EndOfStream.into());
        }

        let byte = self.data[self.position];
        self.position += 1;

        Ok(byte)
    }

    /// Reads an unsigned byte from the data.
    ///
    /// # Returns
    ///
    /// The unsigned byte read or an error if the end of the data has been reached
    pub fn read_u8(&mut self) -> Result<u8> {
        self.read_byte()
    }

    /// Reads a boolean value from the data.
    ///
    /// # Returns
    ///
    /// The boolean value read or an error if the end of the data has been reached
    pub fn read_bool(&mut self) -> Result<bool> {
        let byte = self.read_byte()?;
        Ok(byte != 0)
    }

    /// Reads a signed byte from the data.
    ///
    /// # Returns
    ///
    /// The signed byte read or an error if the end of the data has been reached
    pub fn read_i8(&mut self) -> Result<i8> {
        let byte = self.read_byte()?;
        Ok(byte as i8)
    }

    /// Reads an unsigned 16-bit integer from the data in little-endian format.
    ///
    /// # Returns
    ///
    /// The unsigned 16-bit integer read or an error if the end of the data has been reached
    pub fn read_u16(&mut self) -> Result<u16> {
        if self.remaining() < 2 {
            return Err(Error::EndOfStream.into());
        }

        let mut buf = [0u8; 2];
        buf.copy_from_slice(&self.data[self.position..self.position + 2]);
        self.position += 2;

        Ok(u16::from_le_bytes(buf))
    }

    /// Reads a signed 16-bit integer from the data in little-endian format.
    ///
    /// # Returns
    ///
    /// The signed 16-bit integer read or an error if the end of the data has been reached
    pub fn read_i16(&mut self) -> Result<i16> {
        if self.remaining() < 2 {
            return Err(Error::EndOfStream.into());
        }

        let mut buf = [0u8; 2];
        buf.copy_from_slice(&self.data[self.position..self.position + 2]);
        self.position += 2;

        Ok(i16::from_le_bytes(buf))
    }

    /// Reads an unsigned HASH_SIZE-bit integer from the data in little-endian format.
    ///
    /// # Returns
    ///
    /// The unsigned HASH_SIZE-bit integer read or an error if the end of the data has been reached
    pub fn read_u32(&mut self) -> Result<u32> {
        if self.remaining() < 4 {
            return Err(Error::EndOfStream.into());
        }

        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.data[self.position..self.position + 4]);
        self.position += 4;

        Ok(u32::from_le_bytes(buf))
    }

    /// Reads a signed HASH_SIZE-bit integer from the data in little-endian format.
    ///
    /// # Returns
    ///
    /// The signed HASH_SIZE-bit integer read or an error if the end of the data has been reached
    pub fn read_i32(&mut self) -> Result<i32> {
        if self.remaining() < 4 {
            return Err(Error::EndOfStream.into());
        }

        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.data[self.position..self.position + 4]);
        self.position += 4;

        Ok(i32::from_le_bytes(buf))
    }

    /// Reads an unsigned 64-bit integer from the data in little-endian format.
    ///
    /// # Returns
    ///
    /// The unsigned 64-bit integer read or an error if the end of the data has been reached
    pub fn read_u64(&mut self) -> Result<u64> {
        if self.remaining() < 8 {
            return Err(Error::EndOfStream.into());
        }

        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.data[self.position..self.position + 8]);
        self.position += 8;

        Ok(u64::from_le_bytes(buf))
    }

    /// Reads a signed 64-bit integer from the data in little-endian format.
    ///
    /// # Returns
    ///
    /// The signed 64-bit integer read or an error if the end of the data has been reached
    pub fn read_i64(&mut self) -> Result<i64> {
        if self.remaining() < 8 {
            return Err(Error::EndOfStream.into());
        }

        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.data[self.position..self.position + 8]);
        self.position += 8;

        Ok(i64::from_le_bytes(buf))
    }

    /// Reads a variable-length integer from the data.
    ///
    /// # Returns
    ///
    /// The variable-length integer read or an error if the end of the data has been reached
    pub fn read_var_int(&mut self) -> Result<u64> {
        let first = self.read_byte()? as u64;

        match first {
            0xfd => self.read_u16().map(|v| v as u64),
            0xfe => self.read_u32().map(|v| v as u64),
            0xff => self.read_u64(),
            _ => Ok(first),
        }
    }

    /// Reads a variable-length byte array from the data.
    ///
    /// # Returns
    ///
    /// The variable-length byte array read or an error if the end of the data has been reached
    pub fn read_var_bytes(&mut self) -> Result<Vec<u8>> {
        let length = self.read_var_int()? as usize;
        self.read_bytes(length)
    }

    /// Reads a fixed-length byte array from the data.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the byte array to read
    ///
    /// # Returns
    ///
    /// The fixed-length byte array read or an error if the end of the data has been reached
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        if self.remaining() < length {
            return Err(Error::EndOfStream.into());
        }

        let bytes = self.data.slice(self.position..self.position + length);
        self.position += length;

        Ok(bytes.to_vec())
    }

    /// Reads a variable-length string from the data.
    ///
    /// # Returns
    ///
    /// The variable-length string read or an error if the end of the data has been reached
    pub fn read_var_string(&mut self) -> Result<String> {
        let bytes = self.read_var_bytes()?;
        String::from_utf8(bytes)
            .map_err(|e| Error::Deserialization(format!("Invalid UTF-8 string: {e}")).into())
    }

    /// Reads a serializable object from the data.
    ///
    /// # Returns
    ///
    /// The serializable object read or an error if the end of the data has been reached
    pub fn read_serializable<T: Serializable>(&mut self) -> Result<T> {
        let remaining_data = &self.data[self.position..];
        let mut memory_reader = crate::MemoryReader::new(remaining_data);
        let result = T::deserialize(&mut memory_reader)?;

        // Update position based on how much the MemoryReader consumed
        self.position += memory_reader.position();

        Ok(result)
    }

    /// Reads a variable-length array of serializable objects from the data.
    ///
    /// # Returns
    ///
    /// The variable-length array of serializable objects read or an error if the end of the data has been reached
    pub fn read_serializable_list<T: Serializable>(&mut self) -> Result<Vec<T>> {
        let count = self.read_var_int()? as usize;
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            result.push(self.read_serializable()?);
        }

        Ok(result)
    }

    /// Reads a fixed-length array of serializable objects from the data.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of objects to read
    ///
    /// # Returns
    ///
    /// The fixed-length array of serializable objects read or an error if the end of the data has been reached
    pub fn read_serializable_fixed<T: Serializable>(&mut self, count: usize) -> Result<Vec<T>> {
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            result.push(self.read_serializable()?);
        }

        Ok(result)
    }

    /// Seeks to the specified position in the data.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to seek to
    ///
    /// # Returns
    ///
    /// An error if the position is out of bounds
    pub fn seek(&mut self, position: usize) -> Result<()> {
        if position > self.data.len() {
            return Err(
                Error::InvalidOperation(format!("Position {position} is out of bounds")).into(),
            );
        }

        self.position = position;
        Ok(())
    }

    /// Skips the specified number of bytes in the data.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of bytes to skip
    ///
    /// # Returns
    ///
    /// An error if the end of the data has been reached
    pub fn skip(&mut self, count: usize) -> Result<()> {
        if self.remaining() < count {
            return Err(Error::EndOfStream.into());
        }

        self.position += count;
        Ok(())
    }
}

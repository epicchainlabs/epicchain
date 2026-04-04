// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Memory reader implementation that matches C# MemoryReader exactly.

use crate::error::{IoError, IoResult};
use std::convert::TryInto;

/// A reader for reading data from memory that matches C# MemoryReader behavior exactly.
pub struct MemoryReader {
    memory: Vec<u8>,
    span: Vec<u8>,
    pos: usize,
}

impl MemoryReader {
    /// Creates a new MemoryReader from the given data.
    pub fn new(data: &[u8]) -> Self {
        Self {
            memory: data.to_vec(),
            span: data.to_vec(),
            pos: 0,
        }
    }

    /// Gets the current position in the reader.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Gets the length of the data.
    pub fn len(&self) -> usize {
        self.span.len()
    }

    /// Returns whether the data is empty.
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Sets the position in the reader.
    pub fn set_position(&mut self, position: usize) -> IoResult<()> {
        if position > self.span.len() {
            return Err(IoError::invalid_operation(
                "set_position",
                &format!("Position {position} is out of bounds"),
            ));
        }
        self.pos = position;
        Ok(())
    }

    /// Ensures that there are enough bytes remaining to read the specified amount.
    fn ensure_position(&self, move_amount: usize) -> IoResult<()> {
        if self.pos + move_amount > self.span.len() {
            return Err(IoError::end_of_stream(move_amount, "memory reader"));
        }
        Ok(())
    }

    /// Peeks at the next byte without advancing the position.
    pub fn peek(&self) -> IoResult<u8> {
        self.ensure_position(1)?;
        Ok(self.span[self.pos])
    }

    /// Reads a boolean value.
    pub fn read_boolean(&mut self) -> IoResult<bool> {
        match self.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(IoError::format_exception(
                "read_boolean",
                "invalid boolean value",
            )),
        }
    }

    /// Reads a signed byte.
    pub fn read_sbyte(&mut self) -> IoResult<i8> {
        self.ensure_position(1)?;
        let b = self.span[self.pos];
        self.pos += 1;
        Ok(b as i8)
    }

    /// Reads an unsigned byte.
    pub fn read_byte(&mut self) -> IoResult<u8> {
        self.ensure_position(1)?;
        let result = self.span[self.pos];
        self.pos += 1;
        Ok(result)
    }

    /// Reads a 16-bit signed integer in little-endian format.
    pub fn read_int16(&mut self) -> IoResult<i16> {
        self.ensure_position(2)?;
        let bytes = &self.span[self.pos..self.pos + 2];
        let result = i16::from_le_bytes(bytes.try_into()?);
        self.pos += 2;
        Ok(result)
    }

    /// Reads a 16-bit signed integer in big-endian format.
    pub fn read_int16_big_endian(&mut self) -> IoResult<i16> {
        self.ensure_position(2)?;
        let bytes = &self.span[self.pos..self.pos + 2];
        let result = i16::from_be_bytes(bytes.try_into()?);
        self.pos += 2;
        Ok(result)
    }

    /// Reads a 16-bit unsigned integer in little-endian format.
    pub fn read_uint16(&mut self) -> IoResult<u16> {
        self.ensure_position(2)?;
        let bytes = &self.span[self.pos..self.pos + 2];
        let result = u16::from_le_bytes(bytes.try_into()?);
        self.pos += 2;
        Ok(result)
    }

    /// Reads a 16-bit unsigned integer in big-endian format.
    pub fn read_uint16_big_endian(&mut self) -> IoResult<u16> {
        self.ensure_position(2)?;
        let bytes = &self.span[self.pos..self.pos + 2];
        let result = u16::from_be_bytes(bytes.try_into()?);
        self.pos += 2;
        Ok(result)
    }

    /// Reads a HASH_SIZE-bit signed integer in little-endian format.
    pub fn read_int32(&mut self) -> IoResult<i32> {
        self.ensure_position(4)?;
        let bytes = &self.span[self.pos..self.pos + 4];
        let result = i32::from_le_bytes(bytes.try_into()?);
        self.pos += 4;
        Ok(result)
    }

    /// Reads a HASH_SIZE-bit signed integer in big-endian format.
    pub fn read_int32_big_endian(&mut self) -> IoResult<i32> {
        self.ensure_position(4)?;
        let bytes = &self.span[self.pos..self.pos + 4];
        let result = i32::from_be_bytes(bytes.try_into()?);
        self.pos += 4;
        Ok(result)
    }

    /// Reads a HASH_SIZE-bit unsigned integer in little-endian format.
    pub fn read_uint32(&mut self) -> IoResult<u32> {
        self.ensure_position(4)?;
        let bytes = &self.span[self.pos..self.pos + 4];
        let result = u32::from_le_bytes(bytes.try_into()?);
        self.pos += 4;
        Ok(result)
    }

    /// Reads a HASH_SIZE-bit unsigned integer in little-endian format (alias for compatibility).
    pub fn read_u32(&mut self) -> IoResult<u32> {
        self.read_uint32()
    }

    /// Reads a HASH_SIZE-bit unsigned integer in big-endian format.
    pub fn read_uint32_big_endian(&mut self) -> IoResult<u32> {
        self.ensure_position(4)?;
        let bytes = &self.span[self.pos..self.pos + 4];
        let result = u32::from_be_bytes(bytes.try_into()?);
        self.pos += 4;
        Ok(result)
    }

    /// Reads a 64-bit signed integer in little-endian format.
    pub fn read_int64(&mut self) -> IoResult<i64> {
        self.ensure_position(8)?;
        let bytes = &self.span[self.pos..self.pos + 8];
        let result = i64::from_le_bytes(bytes.try_into()?);
        self.pos += 8;
        Ok(result)
    }

    /// Reads a 64-bit signed integer in big-endian format.
    pub fn read_int64_big_endian(&mut self) -> IoResult<i64> {
        self.ensure_position(8)?;
        let bytes = &self.span[self.pos..self.pos + 8];
        let result = i64::from_be_bytes(bytes.try_into()?);
        self.pos += 8;
        Ok(result)
    }

    /// Reads a 64-bit unsigned integer in little-endian format.
    pub fn read_uint64(&mut self) -> IoResult<u64> {
        self.ensure_position(8)?;
        let bytes = &self.span[self.pos..self.pos + 8];
        let result = u64::from_le_bytes(bytes.try_into()?);
        self.pos += 8;
        Ok(result)
    }

    /// Reads a 64-bit unsigned integer in little-endian format (alias for compatibility).
    pub fn read_u64(&mut self) -> IoResult<u64> {
        self.read_uint64()
    }

    /// Reads a 64-bit unsigned integer in big-endian format.
    pub fn read_uint64_big_endian(&mut self) -> IoResult<u64> {
        self.ensure_position(8)?;
        let bytes = &self.span[self.pos..self.pos + 8];
        let result = u64::from_be_bytes(bytes.try_into()?);
        self.pos += 8;
        Ok(result)
    }

    /// Reads a variable-length integer.
    pub fn read_var_int(&mut self, max: u64) -> IoResult<u64> {
        let b = self.read_byte()?;
        let value = match b {
            0xfd => self.read_uint16()? as u64,
            0xfe => self.read_u32()? as u64,
            0xff => self.read_uint64()?,
            _ => b as u64,
        };
        if value > max {
            return Err(IoError::format_exception(
                "read_var_int",
                "value out of range",
            ));
        }
        Ok(value)
    }

    /// Reads a fixed-length string.
    pub fn read_fixed_string(&mut self, length: usize) -> IoResult<String> {
        self.ensure_position(length)?;
        let end = self.pos + length;
        let mut i = self.pos;

        // Find the null terminator
        while i < end && self.span[i] != 0 {
            i += 1;
        }

        let data = &self.span[self.pos..i];

        // Check that remaining bytes are all zero
        for j in i..end {
            if self.span[j] != 0 {
                return Err(IoError::format_exception(
                    "read_fixed_string",
                    "invalid null terminator",
                ));
            }
        }

        self.pos = end;

        String::from_utf8(data.to_vec())
            .map_err(|_| IoError::encoding("utf8", "invalid utf8 string"))
    }

    /// Reads a variable-length string.
    pub fn read_var_string(&mut self, max: usize) -> IoResult<String> {
        let length = self.read_var_int(max as u64)? as usize;
        self.ensure_position(length)?;
        let data = &self.span[self.pos..self.pos + length];
        self.pos += length;

        String::from_utf8(data.to_vec())
            .map_err(|_| IoError::encoding("utf8", "invalid utf8 string"))
    }

    /// Reads a memory slice of the specified count.
    pub fn read_memory(&mut self, count: usize) -> IoResult<Vec<u8>> {
        self.ensure_position(count)?;
        let result = self.memory[self.pos..self.pos + count].to_vec();
        self.pos += count;
        Ok(result)
    }

    /// Reads a variable-length memory slice.
    pub fn read_var_memory(&mut self, max: usize) -> IoResult<Vec<u8>> {
        let count = self.read_var_int(max as u64)? as usize;
        self.read_memory(count)
    }

    /// Reads all remaining bytes.
    pub fn read_to_end(&mut self) -> IoResult<Vec<u8>> {
        let result = self.memory[self.pos..].to_vec();
        self.pos = self.memory.len();
        Ok(result)
    }

    /// Reads bytes into the provided buffer.
    pub fn read_bytes(&mut self, count: usize) -> IoResult<Vec<u8>> {
        self.ensure_position(count)?;
        let result = self.span[self.pos..self.pos + count].to_vec();
        self.pos += count;
        Ok(result)
    }

    /// Reads exactly the number of bytes to fill the provided buffer.
    /// Matches standard library Read::read_exact behavior.
    pub fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        self.ensure_position(buf.len())?;
        buf.copy_from_slice(&self.span[self.pos..self.pos + buf.len()]);
        self.pos += buf.len();
        Ok(())
    }

    /// Reads a variable-length byte array.
    pub fn read_var_bytes(&mut self, max: usize) -> IoResult<Vec<u8>> {
        let length = self.read_var_int(max as u64)? as usize;
        self.read_bytes(length)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_read_byte() {
        let data = vec![0x42];
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_byte().unwrap(), 0x42);
    }

    #[test]
    fn test_read_boolean() {
        let data = vec![0x00, 0x01, 0x02];
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_boolean().unwrap(), false);
        assert_eq!(reader.read_boolean().unwrap(), true);
        assert!(reader.read_boolean().is_err()); // Invalid boolean value
    }

    #[test]
    fn test_read_u32() {
        let data = vec![0x78, 0x56, 0x34, 0x12]; // Little-endian 0x12345678
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_uint32().unwrap(), 0x12345678);
    }

    #[test]
    fn test_read_u64() {
        let data = vec![0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00]; // Little-endian
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_uint64().unwrap(), 0x12345678);
    }

    #[test]
    fn test_read_var_int() {
        // Test single byte
        let data = vec![0x42];
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_var_int(u64::MAX).unwrap(), 0x42);

        // Test 2-byte value
        let data = vec![0xfd, 0x34, 0x12];
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_var_int(u64::MAX).unwrap(), 0x1234);

        // Test 4-byte value
        let data = vec![0xfe, 0x78, 0x56, 0x34, 0x12]; // 0x12345678
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_var_int(u64::MAX).unwrap(), 0x12345678);

        // Test 8-byte value
        let data = vec![0xff, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00];
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_var_int(u64::MAX).unwrap(), 0x12345678);
    }

    #[test]
    fn test_read_var_string() {
        let data = vec![0x05, b'h', b'e', b'l', b'l', b'o']; // Length 5, "hello"
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.read_var_string(1000).unwrap(), "hello");
    }

    #[test]
    fn test_position() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = MemoryReader::new(&data);
        assert_eq!(reader.position(), 0);
        reader.read_byte().unwrap();
        assert_eq!(reader.position(), 1);
        reader.read_byte().unwrap();
        assert_eq!(reader.position(), 2);
    }

    #[test]
    fn test_peek() {
        let data = vec![0x42, 0x43];
        let reader = MemoryReader::new(&data);
        assert_eq!(reader.peek().unwrap(), 0x42);
        assert_eq!(reader.position(), 0); // Position should not change
    }

    #[test]
    fn test_ensure_position_error() {
        let data = vec![0x01];
        let mut reader = MemoryReader::new(&data);
        reader.read_byte().unwrap(); // Consume the only byte
        assert!(reader.read_byte().is_err()); // Should fail
    }
}

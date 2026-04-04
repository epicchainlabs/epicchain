//! Mock implementation of the epicchain-io module for testing purposes.

/// A generic result type for IO operations.
pub type Result<T> = std::result::Result<T, Error>;

/// A generic error type for IO operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IOError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("End of stream")]
    EndOfStream,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// A simple memory reader implementation.
pub struct MemoryReader {
    data: Vec<u8>,
    position: usize,
}

impl MemoryReader {
    /// Creates a new memory reader.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    /// Gets the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Sets the current position.
    pub fn set_position(&mut self, position: usize) -> VmResult<()> {
        if position > self.data.len() {
            return Err(VmError::invalid_operation_msg("Position out of range"));
        }
        self.position = position;
        Ok(())
    }

    /// Gets a byte at a specific position.
    pub fn get_byte(&self, position: usize) -> VmResult<u8> {
        if position >= self.data.len() {
            return Err(VmError::EndOfStream);
        }
        Ok(self.data[position])
    }

    /// Gets the current byte without advancing the position.
    pub fn peek_byte(&self) -> VmResult<u8> {
        self.get_byte(self.position)
    }

    /// Reads a byte and advances the position.
    pub fn read_byte(&mut self) -> VmResult<u8> {
        let byte = self.peek_byte()?;
        self.position += 1;
        Ok(byte)
    }

    /// Gets a range of bytes.
    pub fn range(&self, start: usize, end: usize) -> VmResult<Vec<u8>> {
        if start >= self.data.len() || end > self.data.len() || start > end {
            return Err(VmError::invalid_operation_msg("Range out of bounds"));
        }
        Ok(self.data[start..end].to_vec())
    }

    /// Reads a range of bytes and advances the position.
    pub fn read_bytes(&mut self, count: usize) -> VmResult<Vec<u8>> {
        let new_position = self.position + count;
        if new_position > self.data.len() {
            return Err(VmError::EndOfStream);
        }
        let bytes = self.data[self.position..new_position].to_vec();
        self.position = new_position;
        Ok(bytes)
    }

    /// Reads a u16 in little-endian format.
    pub fn read_u16(&mut self) -> VmResult<u16> {
        let bytes = self.read_bytes(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Reads a u32 in little-endian format.
    pub fn read_u32(&mut self) -> VmResult<u32> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Reads a u64 in little-endian format.
    pub fn read_u64(&mut self) -> VmResult<u64> {
        let bytes = self.read_bytes(8)?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Reads a i16 in little-endian format.
    pub fn read_i16(&mut self) -> VmResult<i16> {
        let bytes = self.read_bytes(2)?;
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Reads a i32 in little-endian format.
    pub fn read_i32(&mut self) -> VmResult<i32> {
        let bytes = self.read_bytes(4)?;
        Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Reads a i64 in little-endian format.
    pub fn read_i64(&mut self) -> VmResult<i64> {
        let bytes = self.read_bytes(8)?;
        Ok(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Returns the length of the data.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

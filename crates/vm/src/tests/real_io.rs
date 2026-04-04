//! Real I/O operations for VM testing.

use std::io::{Read, Write};

/// A real I/O implementation for testing VM operations.
pub struct RealIO {
    buffer: Vec<u8>,
    position: usize,
}

impl RealIO {
    /// Create a new RealIO instance.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Create a new RealIO instance with initial data.
    pub fn with_data(data: Vec<u8>) -> Self {
        Self {
            buffer: data,
            position: 0,
        }
    }

    /// Get the current buffer contents.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Reset the position to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Clear the buffer and reset position.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
    }
}

impl Default for RealIO {
    fn default() -> Self {
        Self::new()
    }
}

impl Read for RealIO {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let available = self.buffer.len().saturating_sub(self.position);
        let to_read = buf.len().min(available);

        if to_read == 0 {
            return Ok(0);
        }

        buf[..to_read].copy_from_slice(&self.buffer[self.position..self.position + to_read]);
        self.position += to_read;

        Ok(to_read)
    }
}

impl Write for RealIO {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // No-op for in-memory buffer
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    #[test]
    fn test_real_io_write_read() {
        let mut io = RealIO::new();

        // Write some data
        let data = b"Hello, World!";
        let written = io.write(data).unwrap();
        assert_eq!(written, data.len());

        // Reset position and read it back
        io.reset();
        let mut buffer = [0u8; 20];
        let read = io.read(&mut buffer).unwrap();

        assert_eq!(read, data.len());
        assert_eq!(&buffer[..read], data);
    }

    #[test]
    fn test_real_io_with_initial_data() {
        let initial_data = b"Initial data".to_vec();
        let mut io = RealIO::with_data(initial_data.clone());

        let mut buffer = [0u8; 20];
        let read = io.read(&mut buffer).unwrap();

        assert_eq!(read, initial_data.len());
        assert_eq!(&buffer[..read], &initial_data);
    }

    #[test]
    fn test_real_io_clear() {
        let mut io = RealIO::with_data(b"Some data".to_vec());
        assert!(!io.buffer().is_empty());

        io.clear();
        assert!(io.buffer().is_empty());

        let mut buffer = [0u8; 10];
        let read = io.read(&mut buffer).unwrap();
        assert_eq!(read, 0);
    }
}

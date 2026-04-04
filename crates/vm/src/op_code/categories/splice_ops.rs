//! Splice operation OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to string and buffer manipulation,
//! including concatenation, substring operations, and memory copying.

/// Splice operation OpCodes.
///
/// These opcodes manipulate strings and buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpliceOpCode {
    /// Creates a new buffer with the specified size.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    NEWBUFFER = 0x52,

    /// Copies a range of bytes from one buffer to another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 5 items
    /// ```
    MEMCPY = 0x53,

    /// Concatenates two strings or buffers.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    CAT = 0x54,

    /// Returns a substring of a string or a segment of a buffer.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 3 items
    /// ```
    SUBSTR = 0x55,

    /// Returns the left part of a string or buffer.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    LEFT = 0x56,

    /// Returns the right part of a string or buffer.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    RIGHT = 0x57,
}

impl SpliceOpCode {
    /// Checks if this operation creates a new buffer/string.
    pub fn creates_new(&self) -> bool {
        matches!(
            self,
            Self::NEWBUFFER | Self::CAT | Self::SUBSTR | Self::LEFT | Self::RIGHT
        )
    }

    /// Checks if this operation modifies existing data.
    pub fn modifies_existing(&self) -> bool {
        matches!(self, Self::MEMCPY)
    }

    /// Checks if this operation extracts a portion of data.
    pub fn is_extraction(&self) -> bool {
        matches!(self, Self::SUBSTR | Self::LEFT | Self::RIGHT)
    }
}

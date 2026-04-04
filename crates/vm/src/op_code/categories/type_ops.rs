//! Type operation OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to type operations,
//! including type conversion, type checking, and verification.

/// Type operation OpCodes.
///
/// These opcodes handle type conversion and type checking operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TypeOpCode {
    /// Converts a value to the specified type.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    CONVERT = 0xD1,

    /// Checks if a value is of the specified type.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    ISTYPE = 0xD2,

    /// Checks if a value is null.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    ISNULL = 0xD3,

    /// Verifies that a value meets certain criteria.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    VERIFY = 0xD4,
}

impl TypeOpCode {
    /// Checks if this operation performs type conversion.
    pub fn is_conversion(&self) -> bool {
        matches!(self, Self::CONVERT)
    }

    /// Checks if this operation performs type checking.
    pub fn is_type_check(&self) -> bool {
        matches!(self, Self::ISTYPE | Self::ISNULL)
    }

    /// Checks if this operation performs verification.
    pub fn is_verification(&self) -> bool {
        matches!(self, Self::VERIFY)
    }
}

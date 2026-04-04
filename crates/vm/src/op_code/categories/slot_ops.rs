//! Slot operation OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to slot operations,
//! including local variables, static fields, and arguments.

/// Slot operation OpCodes.
///
/// These opcodes manage local variables, static fields, and function arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SlotOpCode {
    /// Initializes the static field list and local variable list.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    INITSLOT = 0x4B,

    /// Loads a static field onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD = 0x4C,

    /// Stores a value to a static field.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    STSFLD = 0x4D,

    /// Loads a local variable onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDLOC = 0x4E,

    /// Stores a value to a local variable.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    STLOC = 0x4F,

    /// Loads an argument onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDARG = 0x50,

    /// Stores a value to an argument.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    STARG = 0x51,
}

impl SlotOpCode {
    /// Checks if this is a load operation.
    pub fn is_load(&self) -> bool {
        matches!(self, Self::LDSFLD | Self::LDLOC | Self::LDARG)
    }

    /// Checks if this is a store operation.
    pub fn is_store(&self) -> bool {
        matches!(self, Self::STSFLD | Self::STLOC | Self::STARG)
    }

    /// Checks if this operates on static fields.
    pub fn is_static_field(&self) -> bool {
        matches!(self, Self::LDSFLD | Self::STSFLD)
    }

    /// Checks if this operates on local variables.
    pub fn is_local_variable(&self) -> bool {
        matches!(self, Self::LDLOC | Self::STLOC)
    }

    /// Checks if this operates on arguments.
    pub fn is_argument(&self) -> bool {
        matches!(self, Self::LDARG | Self::STARG)
    }
}

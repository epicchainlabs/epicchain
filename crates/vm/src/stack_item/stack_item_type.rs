//! Stack item type definitions for the Neo Virtual Machine.
//!
//! This module defines the types of items that can be stored on the VM stack.

/// Represents the types in the VM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackItemType {
    /// Represents any type.
    Any = 0x00,

    /// Represents a code pointer.
    Pointer = 0x10,

    /// Represents the boolean (true or false) type.
    Boolean = 0x20,

    /// Represents an integer.
    Integer = 0x21,

    /// Represents an immutable memory block.
    ByteString = 0x28,

    /// Represents a memory block that can be used for reading and writing.
    Buffer = 0x30,

    /// Represents an array or a complex object.
    Array = 0x40,

    /// Represents a structure.
    Struct = 0x41,

    /// Represents an ordered collection of key-value pairs.
    Map = 0x48,

    /// Represents an interface used to interoperate with the outside of the VM.
    InteropInterface = 0x60,
}

impl StackItemType {
    /// Converts a byte to a StackItemType.
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x00 => Some(Self::Any),
            0x10 => Some(Self::Pointer),
            0x20 => Some(Self::Boolean),
            0x21 => Some(Self::Integer),
            0x28 => Some(Self::ByteString),
            0x30 => Some(Self::Buffer),
            0x40 => Some(Self::Array),
            0x41 => Some(Self::Struct),
            0x48 => Some(Self::Map),
            0x60 => Some(Self::InteropInterface),
            _ => None,
        }
    }

    /// Converts a StackItemType to a byte.
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

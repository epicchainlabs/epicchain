//! Constant-related OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to pushing constants onto the stack,
//! including integers, booleans, null values, and data.

/// Constant-related OpCodes.
///
/// These opcodes are used to push various constant values onto the execution stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConstantOpCode {
    /// Pushes a 1-byte signed integer onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHINT8 = 0x00,

    /// Pushes a 2-byte signed integer onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHINT16 = 0x01,

    /// Pushes a 4-byte signed integer onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHINT32 = 0x02,

    /// Pushes an 8-byte signed integer onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHINT64 = 0x03,

    /// Pushes a 16-byte signed integer onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHINT128 = 0x04,

    /// Pushes a HASH_SIZE-byte signed integer onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHINT256 = 0x05,

    /// Pushes the boolean value `true` onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHT = 0x08,

    /// Pushes the boolean value `false` onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHF = 0x09,

    /// Converts the 4-byte offset to a Pointer, and pushes it onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHA = 0x0A,

    /// Pushes the value `null` onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHNULL = 0x0B,

    /// The next byte contains the number of bytes to be pushed onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHDATA1 = 0x0C,

    /// The next two bytes contain the number of bytes to be pushed onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHDATA2 = 0x0D,

    /// The next four bytes contain the number of bytes to be pushed onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHDATA4 = 0x0E,

    /// Pushes the number -1 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSHM1 = 0x0F,

    /// Pushes the number 0 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH0 = 0x10,

    /// Pushes the number 1 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH1 = 0x11,

    /// Pushes the number 2 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH2 = 0x12,

    /// Pushes the number 3 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH3 = 0x13,

    /// Pushes the number 4 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH4 = 0x14,

    /// Pushes the number 5 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH5 = 0x15,

    /// Pushes the number 6 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH6 = 0x16,

    /// Pushes the number 7 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH7 = 0x17,

    /// Pushes the number 8 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH8 = 0x18,

    /// Pushes the number 9 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH9 = 0x19,

    /// Pushes the number 10 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH10 = 0x1A,

    /// Pushes the number 11 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH11 = 0x1B,

    /// Pushes the number 12 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH12 = 0x1C,

    /// Pushes the number 13 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH13 = 0x1D,

    /// Pushes the number 14 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH14 = 0x1E,

    /// Pushes the number SECONDS_PER_BLOCK onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH15 = 0x1F,

    /// Pushes the number 16 onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PUSH16 = 0x20,
}

impl ConstantOpCode {
    /// Checks if this is a push integer opcode.
    pub fn is_push_int(&self) -> bool {
        matches!(
            self,
            Self::PUSHINT8
                | Self::PUSHINT16
                | Self::PUSHINT32
                | Self::PUSHINT64
                | Self::PUSHINT128
                | Self::PUSHINT256
        )
    }

    /// Checks if this is a push data opcode.
    pub fn is_push_data(&self) -> bool {
        matches!(self, Self::PUSHDATA1 | Self::PUSHDATA2 | Self::PUSHDATA4)
    }

    /// Checks if this is a push number opcode (0-16, -1).
    pub fn is_push_number(&self) -> bool {
        matches!(
            self,
            Self::PUSHM1
                | Self::PUSH0
                | Self::PUSH1
                | Self::PUSH2
                | Self::PUSH3
                | Self::PUSH4
                | Self::PUSH5
                | Self::PUSH6
                | Self::PUSH7
                | Self::PUSH8
                | Self::PUSH9
                | Self::PUSH10
                | Self::PUSH11
                | Self::PUSH12
                | Self::PUSH13
                | Self::PUSH14
                | Self::PUSH15
                | Self::PUSH16
        )
    }

    /// Gets the numeric value for push number opcodes.
    pub fn get_push_number_value(&self) -> Option<i32> {
        match self {
            Self::PUSHM1 => Some(-1),
            Self::PUSH0 => Some(0),
            Self::PUSH1 => Some(1),
            Self::PUSH2 => Some(2),
            Self::PUSH3 => Some(3),
            Self::PUSH4 => Some(4),
            Self::PUSH5 => Some(5),
            Self::PUSH6 => Some(6),
            Self::PUSH7 => Some(7),
            Self::PUSH8 => Some(8),
            Self::PUSH9 => Some(9),
            Self::PUSH10 => Some(10),
            Self::PUSH11 => Some(11),
            Self::PUSH12 => Some(12),
            Self::PUSH13 => Some(13),
            Self::PUSH14 => Some(14),
            Self::PUSH15 => Some(15),
            Self::PUSH16 => Some(16),
            _ => None,
        }
    }
}

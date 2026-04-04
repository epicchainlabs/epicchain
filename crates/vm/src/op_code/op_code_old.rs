//! Implementation of the OpCode enum for the Neo Virtual Machine.

use super::operand_size::OperandSize;
use neo_config::{HASH_SIZE};

/// Represents the opcode of an instruction in the Neo Virtual Machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Constants
    PUSHINT8 = 0x00,
    PUSHINT16 = 0x01,
    PUSHINT32 = 0x02,
    PUSHINT64 = 0x03,
    PUSHINT128 = 0x04,
    PUSHINT256 = 0x05,

    PUSHT = 0x08,
    PUSHF = 0x09,
    PUSHA = 0x0A,
    PUSHNULL = 0x0B,
    PUSHDATA1 = 0x0C,
    PUSHDATA2 = 0x0D,
    PUSHDATA4 = 0x0E,

    PUSHM1 = 0x0F,
    PUSH0 = 0x10,
    PUSH1 = 0x11,
    PUSH2 = 0x12,
    PUSH3 = 0x13,
    PUSH4 = 0x14,
    PUSH5 = 0x15,
    PUSH6 = 0x16,
    PUSH7 = 0x17,
    PUSH8 = 0x18,
    PUSH9 = 0x19,
    PUSH10 = 0x1A,
    PUSH11 = 0x1B,
    PUSH12 = 0x1C,
    PUSH13 = 0x1D,
    PUSH14 = 0x1E,
    PUSH15 = 0x1F,
    PUSH16 = 0x20,

    // Flow control
    NOP = 0x21,
    JMP = 0x22,
    JmpL = 0x23,
    JMPIF = 0x24,
    JmpifL = 0x25,

    /// Transfers control to a target instruction if the value is false, a null reference, or zero. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    JMPIFNOT = 0x26,

    /// Transfers control to a target instruction if the value is false, a null reference, or zero. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    JmpifnotL = 0x27,

    /// Transfers control to a target instruction if two values are equal. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JMPEQ = 0x28,

    /// Transfers control to a target instruction if two values are equal. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JmpeqL = 0x29,

    /// Transfers control to a target instruction when two values are not equal. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JMPNE = 0x2A,

    /// Transfers control to a target instruction when two values are not equal. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JmpneL = 0x2B,

    /// Transfers control to a target instruction if the first value is greater than the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JMPGT = 0x2C,

    /// Transfers control to a target instruction if the first value is greater than the second value. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JmpgtL = 0x2D,

    /// Transfers control to a target instruction if the first value is greater than or equal to the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JMPGE = 0x2E,

    /// Transfers control to a target instruction if the first value is greater than or equal to the second value. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JmpgeL = 0x2F,

    /// Transfers control to a target instruction if the first value is less than the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JMPLT = 0x30,

    /// Transfers control to a target instruction if the first value is less than the second value. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JmpltL = 0x31,

    /// Transfers control to a target instruction if the first value is less than or equal to the second value. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JMPLE = 0x32,

    /// Transfers control to a target instruction if the first value is less than or equal to the second value. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    JmpleL = 0x33,

    /// Calls the function at the target address. The target instruction is represented as a 1-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    CALL = 0x34,

    /// Calls the function at the target address. The target instruction is represented as a 4-byte signed offset from the beginning of the current instruction.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    CallL = 0x35,

    /// Calls the function at the target address. The target instruction is represented as a value on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    CALLA = 0x36,

    /// Calls the function which is described by the token.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    CALLT = 0x37,

    /// Unconditionally terminates the execution with failure.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    ABORT = 0x38,

    /// Throws an exception if the condition is not met.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    ASSERT = 0x39,

    /// Throws an exception.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    THROW = 0x3A,

    /// Begins a try block.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    TRY = 0x3B,

    /// Begins a try block with long offsets.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    TryL = 0x3C,

    /// Ends a try block.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    ENDTRY = 0x3D,

    /// Ends a try block with long offset.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    EndtryL = 0x3E,

    /// Ends a finally block.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    ENDFINALLY = 0x3F,

    /// Returns from the current function.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    RET = 0x40,

    /// Calls a system function.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    SYSCALL = 0x41,

    // Stack operations

    /// Puts the number of stack items onto the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    DEPTH = 0x43,

    /// Removes the top stack item.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    DROP = 0x45,

    /// Removes the second-to-top stack item.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    NIP = 0x46,

    /// The item n back in the main stack is removed.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: n+1 items
    /// ```
    XDROP = 0x48,

    /// Clear the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: all items
    /// ```
    CLEAR = 0x49,

    /// Duplicates the item at the top of the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    DUP = 0x4A,

    /// Copies the second-to-top stack item to the top.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    OVER = 0x4B,

    /// The item n back in the stack is copied to the top.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    PICK = 0x4D,

    /// The item at the top of the stack is copied and inserted before the second-to-top item.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    TUCK = 0x4E,

    /// The top two items on the stack are swapped.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    SWAP = 0x50,

    /// The top three items on the stack are rotated to the left.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    ROT = 0x51,

    /// The item n back in the stack is moved to the top.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    ROLL = 0x52,

    /// Reverse the order of the top 3 items on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    REVERSE3 = 0x53,

    /// Reverse the order of the top 4 items on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 0 items
    /// ```
    REVERSE4 = 0x54,

    /// Pop the number N on the stack, and reverse the order of the top N items on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    REVERSEN = 0x55,

    // Slot operations

    /// Initializes the static field list for the current execution context.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    INITSSLOT = 0x56,

    /// Initializes the argument and local variable list for the current execution context.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 2 items
    /// ```
    INITSLOT = 0x57,

    /// Loads the static field at index 0 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD0 = 0x58,

    /// Loads the static field at index 1 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD1 = 0x59,

    /// Loads the static field at index 2 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD2 = 0x5A,

    /// Loads the static field at index 3 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD3 = 0x5B,

    /// Loads the static field at index 4 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD4 = 0x5C,

    /// Loads the static field at index 5 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD5 = 0x5D,

    /// Loads the static field at index 6 onto the evaluation stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD6 = 0x5E,

    /// Loads the static field at a specified index onto the evaluation stack. The index is represented as a 1-byte unsigned integer.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    LDSFLD = 0x5F,

    // Splice operations

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

    // Bitwise operations

    /// Performs a bitwise inversion.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    INVERT = 0x58,

    /// Performs a bitwise AND operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    AND = 0x59,

    /// Performs a bitwise OR operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    OR = 0x5A,

    /// Performs a bitwise XOR operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    XOR = 0x5B,

    /// Returns 1 if the inputs are exactly equal, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    EQUAL = 0x5C,

    /// Returns 1 if the inputs are not equal, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    NOTEQUAL = 0x5D,

    // Numeric operations

    /// Increments a numeric value by 1.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    INC = 0x5E,

    /// Decrements a numeric value by 1.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    DEC = 0x5F,

    /// Returns the sign of a numeric value: 1 if positive, 0 if zero, -1 if negative.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    SIGN = 0x60,

    /// Negates a numeric value.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    NEGATE = 0x61,

    /// Returns the absolute value of a numeric value.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    ABS = 0x62,

    /// Adds two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    ADD = 0x63,

    /// Subtracts one numeric value from another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    SUB = 0x64,

    /// Multiplies two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MUL = 0x65,

    /// Divides one numeric value by another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    DIV = 0x66,

    /// Returns the remainder after division.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MOD = 0x67,

    /// Raises one numeric value to the power of another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    POW = 0x68,

    /// Returns the square root of a numeric value.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    SQRT = 0x69,

    /// Performs a left shift operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    SHL = 0x6A,

    /// Performs a right shift operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    SHR = 0x6B,

    /// Returns the smaller of two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MIN = 0x6C,

    /// Returns the larger of two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MAX = 0x6D,

    /// Returns 1 if x is within the specified range (left-inclusive), 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 3 items
    /// ```
    WITHIN = 0x6E,

    // Compound-type operations

    /// Creates a new array with the specified length.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    NEWARRAY = 0xC0,

    /// Creates a new array with the specified length and type.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    NewarrayT = 0xC1,

    /// Creates a new struct with the specified length.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    NEWSTRUCT = 0xC2,

    /// Creates a new map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    NEWMAP = 0xC3,

    /// Appends an item to an array or struct.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    APPEND = 0xC4,

    /// Reverses the order of elements in an array or struct.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    REVERSE = 0xC5,

    /// Removes an item from an array, struct, or map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    REMOVE = 0xC6,

    /// Returns 1 if the key exists in the collection, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    HASKEY = 0xC7,

    /// Returns the keys of a map as an array.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    KEYS = 0xC8,

    /// Returns the values of a map as an array.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    VALUES = 0xC9,

    /// Converts a key-value pair to a map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + 2*n items
    /// ```
    PACKMAP = 0xCA,

    /// Converts a sequence of items to a struct.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n items
    /// ```
    PACKSTRUCT = 0xCB,

    /// Converts a sequence of items to an array.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item + n items
    /// ```
    PACK = 0xCC,

    /// Unpacks an array or struct to the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item + n items
    /// Pop: 1 item
    /// ```
    UNPACK = 0xCD,

    /// Gets an item from an array, struct, or map.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    PICKITEM = 0xCE,

    /// Sets an item in an array, struct, or map.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 3 items
    /// ```
    SETITEM = 0xCF,

    /// Returns the length of an array, struct, map, string, or buffer.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    SIZE = 0xD0,

    // Type operations

    /// Converts a value to a specific type.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    CONVERT = 0xD1,

    /// Returns 1 if the item can be converted to the specified type, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    ISTYPE = 0xD2,

    /// Returns 1 if the item is null, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    ISNULL = 0xD3,

    /// Verifies a signature.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 3 items
    /// ```
    VERIFY = 0xD4,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(byte: u8) -> VmResult<Self, Self::Error> {
        Self::from_byte(byte).ok_or(())
    }
}

impl OpCode {
    /// Attempts to convert a byte to an OpCode.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to convert
    ///
    /// # Returns
    ///
    /// An Option containing the OpCode if the byte is a valid OpCode, None otherwise
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::PUSHINT8),
            0x01 => Some(Self::PUSHINT16),
            0x02 => Some(Self::PUSHINT32),
            0x03 => Some(Self::PUSHINT64),
            0x04 => Some(Self::PUSHINT128),
            0x05 => Some(Self::PUSHINT256),
            0x08 => Some(Self::PUSHT),
            0x09 => Some(Self::PUSHF),
            0x0A => Some(Self::PUSHA),
            0x0B => Some(Self::PUSHNULL),
            0x0C => Some(Self::PUSHDATA1),
            0x0D => Some(Self::PUSHDATA2),
            0x0E => Some(Self::PUSHDATA4),
            0x0F => Some(Self::PUSHM1),
            0x10 => Some(Self::PUSH0),
            0x11 => Some(Self::PUSH1),
            0x12 => Some(Self::PUSH2),
            0x13 => Some(Self::PUSH3),
            0x14 => Some(Self::PUSH4),
            0x15 => Some(Self::PUSH5),
            0x16 => Some(Self::PUSH6),
            0x17 => Some(Self::PUSH7),
            0x18 => Some(Self::PUSH8),
            0x19 => Some(Self::PUSH9),
            0x1A => Some(Self::PUSH10),
            0x1B => Some(Self::PUSH11),
            0x1C => Some(Self::PUSH12),
            0x1D => Some(Self::PUSH13),
            0x1E => Some(Self::PUSH14),
            0x1F => Some(Self::PUSH15),
            0x20 => Some(Self::PUSH16),
            0x21 => Some(Self::NOP),
            0x22 => Some(Self::JMP),
            0x23 => Some(Self::JmpL),
            0x24 => Some(Self::JMPIF),
            0x25 => Some(Self::JmpifL),
            0x26 => Some(Self::JMPIFNOT),
            0x27 => Some(Self::JmpifnotL),
            0x28 => Some(Self::JMPEQ),
            0x29 => Some(Self::JmpeqL),
            0x2A => Some(Self::JMPNE),
            0x2B => Some(Self::JmpneL),
            0x2C => Some(Self::JMPGT),
            0x2D => Some(Self::JmpgtL),
            0x2E => Some(Self::JMPGE),
            0x2F => Some(Self::JmpgeL),
            0x30 => Some(Self::JMPLT),
            0x31 => Some(Self::JmpltL),
            0x32 => Some(Self::JMPLE),
            0x33 => Some(Self::JmpleL),
            0x34 => Some(Self::CALL),
            0x35 => Some(Self::CallL),
            0x36 => Some(Self::CALLA),
            0x37 => Some(Self::CALLT),
            0x38 => Some(Self::ABORT),
            0x39 => Some(Self::ASSERT),
            0x3A => Some(Self::THROW),
            0x3B => Some(Self::TRY),
            0x3C => Some(Self::TryL),
            0x3D => Some(Self::ENDTRY),
            0x3E => Some(Self::EndtryL),
            0x3F => Some(Self::ENDFINALLY),
            0x40 => Some(Self::RET),
            0x41 => Some(Self::SYSCALL),
            0x43 => Some(Self::DEPTH),
            0x45 => Some(Self::DROP),
            0x46 => Some(Self::NIP),
            0x48 => Some(Self::XDROP),
            0x49 => Some(Self::CLEAR),
            0x4A => Some(Self::DUP),
            0x4B => Some(Self::OVER),
            0x4D => Some(Self::PICK),
            0x4E => Some(Self::TUCK),
            0x50 => Some(Self::SWAP),
            0x51 => Some(Self::ROT),
            0x52 => Some(Self::ROLL),
            0x53 => Some(Self::REVERSE3),
            0x54 => Some(Self::REVERSE4),
            0x55 => Some(Self::REVERSEN),
            0x4B => Some(Self::INITSLOT),
            0x4C => Some(Self::LDSFLD),
            0x4D => Some(Self::STSFLD),
            0x4E => Some(Self::LDLOC),
            0x4F => Some(Self::STLOC),
            0x50 => Some(Self::LDARG),
            0x51 => Some(Self::STARG),
            0x52 => Some(Self::NEWBUFFER),
            0x53 => Some(Self::MEMCPY),
            0x54 => Some(Self::CAT),
            0x55 => Some(Self::SUBSTR),
            0x56 => Some(Self::LEFT),
            0x57 => Some(Self::RIGHT),
            0x58 => Some(Self::INVERT),
            0x59 => Some(Self::AND),
            0x5A => Some(Self::OR),
            0x5B => Some(Self::XOR),
            0x5C => Some(Self::EQUAL),
            0x5D => Some(Self::NOTEQUAL),
            0x5E => Some(Self::INC),
            0x5F => Some(Self::DEC),
            0x60 => Some(Self::SIGN),
            0x61 => Some(Self::NEGATE),
            0x62 => Some(Self::ABS),
            0x63 => Some(Self::ADD),
            0x64 => Some(Self::SUB),
            0x65 => Some(Self::MUL),
            0x66 => Some(Self::DIV),
            0x67 => Some(Self::MOD),
            0x68 => Some(Self::POW),
            0x69 => Some(Self::SQRT),
            0x6A => Some(Self::SHL),
            0x6B => Some(Self::SHR),
            0x6C => Some(Self::MIN),
            0x6D => Some(Self::MAX),
            0x6E => Some(Self::WITHIN),
            0xC0 => Some(Self::NEWARRAY),
            0xC1 => Some(Self::NewarrayT),
            0xC2 => Some(Self::NEWSTRUCT),
            0xC3 => Some(Self::NEWMAP),
            0xC4 => Some(Self::APPEND),
            0xC5 => Some(Self::REVERSE),
            0xC6 => Some(Self::REMOVE),
            0xC7 => Some(Self::HASKEY),
            0xC8 => Some(Self::KEYS),
            0xC9 => Some(Self::VALUES),
            0xCA => Some(Self::PACKMAP),
            0xCB => Some(Self::PACKSTRUCT),
            0xCC => Some(Self::PACK),
            0xCD => Some(Self::UNPACK),
            0xCE => Some(Self::PICKITEM),
            0xCF => Some(Self::SETITEM),
            0xD0 => Some(Self::SIZE),
            0xD1 => Some(Self::CONVERT),
            0xD2 => Some(Self::ISTYPE),
            0xD3 => Some(Self::ISNULL),
            0xD4 => Some(Self::VERIFY),
            0xF0 => Some(Self::TryL),
            0xF1 => Some(Self::EndtryL),
            _ => None,
        }
    }

    /// Returns an iterator over all opcodes.
    ///
    /// # Returns
    ///
    /// An iterator over all opcodes
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..=Self::VERIFY as u8).filter_map(Self::from_byte)
    }

    /// Gets the operand size information for this OpCode.
    ///
    /// # Returns
    ///
    /// The operand size information
    pub fn operand_size_info(&self) -> OperandSize {
        match self {
            Self::PUSHINT8 => OperandSize::fixed(1),
            Self::PUSHINT16 => OperandSize::fixed(2),
            Self::PUSHINT32 => OperandSize::fixed(4),
            Self::PUSHINT64 => OperandSize::fixed(8),
            Self::PUSHINT128 => OperandSize::fixed(16),
            Self::PUSHINT256 => OperandSize::fixed(HASH_SIZE),
            Self::PUSHA => OperandSize::fixed(4),
            Self::PUSHDATA1 => OperandSize::prefix(1),
            Self::PUSHDATA2 => OperandSize::prefix(2),
            Self::PUSHDATA4 => OperandSize::prefix(4),
            Self::JMP => OperandSize::fixed(1),
            Self::JmpL => OperandSize::fixed(4),
            Self::JMPIF => OperandSize::fixed(1),
            Self::JmpifL => OperandSize::fixed(4),
            Self::JMPIFNOT => OperandSize::fixed(1),
            Self::JmpifnotL => OperandSize::fixed(4),
            Self::JMPEQ => OperandSize::fixed(1),
            Self::JmpeqL => OperandSize::fixed(4),
            Self::JMPNE => OperandSize::fixed(1),
            Self::JmpneL => OperandSize::fixed(4),
            Self::JMPGT => OperandSize::fixed(1),
            Self::JmpgtL => OperandSize::fixed(4),
            Self::JMPGE => OperandSize::fixed(1),
            Self::JmpgeL => OperandSize::fixed(4),
            Self::JMPLT => OperandSize::fixed(1),
            Self::JmpltL => OperandSize::fixed(4),
            Self::JMPLE => OperandSize::fixed(1),
            Self::JmpleL => OperandSize::fixed(4),
            Self::CALL => OperandSize::fixed(1),
            Self::CallL => OperandSize::fixed(4),
            Self::CALLA => OperandSize::fixed(0),
            Self::ABORT => OperandSize::fixed(0),
            Self::ASSERT => OperandSize::fixed(0),
            Self::THROW => OperandSize::fixed(0),
            Self::TRY => OperandSize::fixed(4), // 2 + 2 bytes for catch and finally offsets
            Self::ENDTRY => OperandSize::fixed(2),
            Self::ENDFINALLY => OperandSize::fixed(0),
            Self::RET => OperandSize::fixed(0),
            Self::SYSCALL => OperandSize::fixed(4),
            Self::DUP => OperandSize::fixed(0),
            Self::SWAP => OperandSize::fixed(0),
            Self::OVER => OperandSize::fixed(0),
            Self::ROT => OperandSize::fixed(0),
            Self::TUCK => OperandSize::fixed(0),
            Self::DEPTH => OperandSize::fixed(0),
            Self::DROP => OperandSize::fixed(0),
            Self::NIP => OperandSize::fixed(0),
            Self::XDROP => OperandSize::fixed(0),
            Self::CLEAR => OperandSize::fixed(0),
            Self::PICK => OperandSize::fixed(0),
            Self::INITSLOT => OperandSize::fixed(2), // 1 + 1 bytes for local and argument counts
            Self::LDSFLD => OperandSize::fixed(1),
            Self::STSFLD => OperandSize::fixed(1),
            Self::LDLOC => OperandSize::fixed(1),
            Self::STLOC => OperandSize::fixed(1),
            Self::LDARG => OperandSize::fixed(1),
            Self::STARG => OperandSize::fixed(1),
            Self::NEWBUFFER => OperandSize::fixed(0),
            Self::MEMCPY => OperandSize::fixed(0),
            Self::CAT => OperandSize::fixed(0),
            Self::SUBSTR => OperandSize::fixed(0),
            Self::LEFT => OperandSize::fixed(0),
            Self::RIGHT => OperandSize::fixed(0),
            Self::INVERT => OperandSize::fixed(0),
            Self::AND => OperandSize::fixed(0),
            Self::OR => OperandSize::fixed(0),
            Self::XOR => OperandSize::fixed(0),
            Self::EQUAL => OperandSize::fixed(0),
            Self::NOTEQUAL => OperandSize::fixed(0),
            Self::INC => OperandSize::fixed(0),
            Self::DEC => OperandSize::fixed(0),
            Self::SIGN => OperandSize::fixed(0),
            Self::NEGATE => OperandSize::fixed(0),
            Self::ABS => OperandSize::fixed(0),
            Self::ADD => OperandSize::fixed(0),
            Self::SUB => OperandSize::fixed(0),
            Self::MUL => OperandSize::fixed(0),
            Self::DIV => OperandSize::fixed(0),
            Self::MOD => OperandSize::fixed(0),
            Self::POW => OperandSize::fixed(0),
            Self::SQRT => OperandSize::fixed(0),
            Self::SHL => OperandSize::fixed(0),
            Self::SHR => OperandSize::fixed(0),
            Self::MIN => OperandSize::fixed(0),
            Self::MAX => OperandSize::fixed(0),
            Self::WITHIN => OperandSize::fixed(0),
            Self::NEWARRAY => OperandSize::fixed(0),
            Self::NewarrayT => OperandSize::fixed(1),
            Self::NEWSTRUCT => OperandSize::fixed(0),
            Self::NEWMAP => OperandSize::fixed(0),
            Self::APPEND => OperandSize::fixed(0),
            Self::REVERSE => OperandSize::fixed(0),
            Self::REMOVE => OperandSize::fixed(0),
            Self::HASKEY => OperandSize::fixed(0),
            Self::KEYS => OperandSize::fixed(0),
            Self::VALUES => OperandSize::fixed(0),
            Self::PACKMAP => OperandSize::fixed(0),
            Self::PACKSTRUCT => OperandSize::fixed(0),
            Self::PACK => OperandSize::fixed(0),
            Self::UNPACK => OperandSize::fixed(0),
            Self::PICKITEM => OperandSize::fixed(0),
            Self::SETITEM => OperandSize::fixed(0),
            Self::SIZE => OperandSize::fixed(0),
            Self::CONVERT => OperandSize::fixed(1),
            Self::ISTYPE => OperandSize::fixed(1),
            Self::ISNULL => OperandSize::fixed(0),
            _ => OperandSize::fixed(0),
        }
    }

    /// Gets the name of this OpCode as a string.
    ///
    /// # Returns
    ///
    /// The name of the OpCode
    pub fn name(&self) -> &'static str {
        match self {
            Self::PUSHINT8 => "PUSHINT8",
            Self::PUSHINT16 => "PUSHINT16",
            Self::PUSHINT32 => "PUSHINT32",
            Self::PUSHINT64 => "PUSHINT64",
            Self::PUSHINT128 => "PUSHINT128",
            Self::PUSHINT256 => "PUSHINT256",
            Self::PUSHT => "PUSHT",
            Self::PUSHF => "PUSHF",
            Self::PUSHA => "PUSHA",
            Self::PUSHNULL => "PUSHNULL",
            Self::PUSHDATA1 => "PUSHDATA1",
            Self::PUSHDATA2 => "PUSHDATA2",
            Self::PUSHDATA4 => "PUSHDATA4",
            Self::PUSHM1 => "PUSHM1",
            Self::PUSH0 => "PUSH0",
            Self::PUSH1 => "PUSH1",
            Self::PUSH2 => "PUSH2",
            Self::PUSH3 => "PUSH3",
            Self::PUSH4 => "PUSH4",
            Self::PUSH5 => "PUSH5",
            Self::PUSH6 => "PUSH6",
            Self::PUSH7 => "PUSH7",
            Self::PUSH8 => "PUSH8",
            Self::PUSH9 => "PUSH9",
            Self::PUSH10 => "PUSH10",
            Self::PUSH11 => "PUSH11",
            Self::PUSH12 => "PUSH12",
            Self::PUSH13 => "PUSH13",
            Self::PUSH14 => "PUSH14",
            Self::PUSH15 => "PUSH15",
            Self::PUSH16 => "PUSH16",
            Self::NOP => "NOP",
            Self::JMP => "JMP",
            Self::JmpL => "JmpL",
            Self::JMPIF => "JMPIF",
            Self::JmpifL => "JmpifL",
            Self::JMPIFNOT => "JMPIFNOT",
            Self::JmpifnotL => "JmpifnotL",
            Self::JMPEQ => "JMPEQ",
            Self::JmpeqL => "JmpeqL",
            Self::JMPNE => "JMPNE",
            Self::JmpneL => "JmpneL",
            Self::JMPGT => "JMPGT",
            Self::JmpgtL => "JmpgtL",
            Self::JMPGE => "JMPGE",
            Self::JmpgeL => "JmpgeL",
            Self::JMPLT => "JMPLT",
            Self::JmpltL => "JmpltL",
            Self::JMPLE => "JMPLE",
            Self::JmpleL => "JmpleL",
            Self::CallL => "CallL",
            Self::TryL => "TryL",
            Self::EndtryL => "EndtryL",
            Self::NewarrayT => "NewarrayT",
            _ => "UNKNOWN",
        }
    }

    /// Checks if this OpCode is a branch instruction.
    ///
    /// # Returns
    ///
    /// true if this is a branch instruction, false otherwise
    pub fn is_branch(&self) -> bool {
        matches!(
            self,
            Self::JMP
                | Self::JmpL
                | Self::JMPIF
                | Self::JmpifL
                | Self::JMPIFNOT
                | Self::JmpifnotL
                | Self::JMPEQ
                | Self::JmpeqL
                | Self::JMPNE
        )
    }

    /// Gets the number of stack items pushed by this instruction.
    ///
    /// # Returns
    ///
    /// The number of stack items pushed
    pub fn stack_items_pushed(&self) -> i32 {
        match self {
            Self::PUSHINT8
            | Self::PUSHINT16
            | Self::PUSHINT32
            | Self::PUSHINT64
            | Self::PUSHINT128
            | Self::PUSHINT256
            | Self::PUSHT
            | Self::PUSHF
            | Self::PUSHA
            | Self::PUSHNULL
            | Self::PUSHDATA1
            | Self::PUSHDATA2
            | Self::PUSHDATA4
            | Self::PUSHM1
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
            | Self::PUSH16 => 1,
            _ => 0,
        }
    }

    /// Gets the number of stack items popped by this instruction.
    ///
    /// # Returns
    ///
    /// The number of stack items popped
    pub fn stack_items_popped(&self) -> i32 {
        match self {
            Self::JMPIF | Self::JmpifL | Self::JMPIFNOT | Self::JmpifnotL => 1,
            Self::JMPEQ | Self::JmpeqL | Self::JMPNE => 2,
            _ => 0,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_from_byte() {
        assert_eq!(OpCode::from_byte(0x00), Some(OpCode::PUSHINT8));
        assert_eq!(OpCode::from_byte(0x10), Some(OpCode::PUSH0));
        assert_eq!(OpCode::from_byte(0x21), Some(OpCode::NOP));
        assert_eq!(OpCode::from_byte(0x63), Some(OpCode::ADD));
        assert_eq!(OpCode::from_byte(0x3D), Some(OpCode::RET));
        // This would need to be updated when all opcodes are implemented
        assert_eq!(OpCode::from_byte(0xFF), None);
    }

    #[test]
    fn test_operand_size() {
        assert_eq!(OpCode::PUSHINT8.operand_size_info().size(), 1);
        assert_eq!(OpCode::PUSHINT16.operand_size_info().size(), 2);
        assert_eq!(OpCode::PUSHINT32.operand_size_info().size(), 4);
        assert_eq!(OpCode::PUSHINT64.operand_size_info().size(), 8);
        assert_eq!(OpCode::PUSHDATA1.operand_size_info().size_prefix(), 1);
        assert_eq!(OpCode::PUSHDATA2.operand_size_info().size_prefix(), 2);
        assert_eq!(OpCode::PUSHDATA4.operand_size_info().size_prefix(), 4);
        assert_eq!(OpCode::NOP.operand_size_info().size(), 0);
    }

    #[test]
    fn test_name() {
        assert_eq!(OpCode::PUSHINT8.name(), "PUSHINT8");
        assert_eq!(OpCode::JMP.name(), "JMP");
        assert_eq!(OpCode::NOP.name(), "NOP");
    }

    #[test]
    fn test_is_branch() {
        assert!(OpCode::JMP.is_branch());
        assert!(OpCode::JMPIF.is_branch());
        assert!(!OpCode::NOP.is_branch());
        assert!(!OpCode::PUSH0.is_branch());
    }
}
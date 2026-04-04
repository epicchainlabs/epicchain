//! Script builder module for the Neo Virtual Machine.
//!
//! This module provides a way to programmatically construct scripts for the Neo VM.

use crate::error::VmResult;
use crate::op_code::OpCode;
use crate::script::Script;

/// Helps construct VM scripts programmatically.
pub struct ScriptBuilder {
    /// The script being built
    script: Vec<u8>,
}

impl ScriptBuilder {
    /// Creates a new script builder.
    pub fn new() -> Self {
        Self { script: Vec::new() }
    }

    /// Emits a single byte to the script.
    pub fn emit(&mut self, op: u8) -> &mut Self {
        self.script.push(op);
        self
    }

    /// Emits an opcode to the script.
    pub fn emit_opcode(&mut self, op: OpCode) -> &mut Self {
        self.script.push(op as u8);
        self
    }

    /// Emits raw bytes to the script.
    pub fn emit_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.script.extend_from_slice(bytes);
        self
    }

    /// Emits a push operation with the given data.
    pub fn emit_push(&mut self, data: &[u8]) -> &mut Self {
        let len = data.len();

        if len <= 0x75 {
            self.emit(len as u8);
            self.script.extend_from_slice(data);
        } else if len <= 0xFF {
            // PUSHDATA1
            self.emit_opcode(OpCode::PUSHDATA1);
            self.emit(len as u8);
            self.script.extend_from_slice(data);
        } else if len <= 0xFFFF {
            // PUSHDATA2
            self.emit_opcode(OpCode::PUSHDATA2);
            self.emit((len & 0xFF) as u8);
            self.emit((len >> 8) as u8);
            self.script.extend_from_slice(data);
        } else {
            // PUSHDATA4
            self.emit_opcode(OpCode::PUSHDATA4);
            self.emit((len & 0xFF) as u8);
            self.emit(((len >> 8) & 0xFF) as u8);
            self.emit(((len >> 16) & 0xFF) as u8);
            self.emit(((len >> 24) & 0xFF) as u8);
            self.script.extend_from_slice(data);
        }

        self
    }

    /// Emits a push operation for an integer.
    pub fn emit_push_int(&mut self, value: i64) -> &mut Self {
        if value == -1 {
            return self.emit_opcode(OpCode::PUSHM1);
        }
        if (0..=16).contains(&value) {
            let opcode_value = OpCode::PUSH0 as u8 + (value as u8);
            self.emit(opcode_value);
            return self;
        }

        let mut bytes = Vec::new();
        let mut v = value;

        // Convert to little-endian byte representation
        while v != 0 && v != -1 {
            bytes.push((v & 0xFF) as u8);
            v >>= 8;
        }

        // Handle sign bit
        if v == -1 && (bytes.last().unwrap_or(&0) & 0x80) == 0 {
            bytes.push(0xFF);
        } else if v == 0
            && !bytes.is_empty()
            && (bytes.last().expect("collection should not be empty") & 0x80) != 0
        {
            bytes.push(0x00);
        }

        self.emit_push(&bytes)
    }

    /// Emits a push operation for a boolean.
    pub fn emit_push_bool(&mut self, value: bool) -> &mut Self {
        if value {
            self.emit_opcode(OpCode::PUSH1)
        } else {
            self.emit_opcode(OpCode::PUSH0)
        }
    }

    /// Emits a push operation for a byte array.
    pub fn emit_push_byte_array(&mut self, data: &[u8]) -> &mut Self {
        self.emit_push(data)
    }

    /// Emits a push operation for a string.
    pub fn emit_push_string(&mut self, value: &str) -> &mut Self {
        self.emit_push(value.as_bytes())
    }

    /// Emits a push operation for raw bytes.
    pub fn emit_push_bytes(&mut self, data: &[u8]) -> &mut Self {
        self.emit_push(data)
    }

    /// Emits a push operation for a stack item.
    pub fn emit_push_stack_item(
        &mut self,
        item: crate::stack_item::StackItem,
    ) -> VmResult<&mut Self> {
        use crate::stack_item::StackItem;

        match item {
            StackItem::Null => {
                self.emit_opcode(OpCode::PUSHNULL);
            }
            StackItem::Boolean(b) => {
                self.emit_push_bool(b);
            }
            StackItem::Integer(i) => {
                use num_traits::ToPrimitive;
                if let Some(value) = i.to_i64() {
                    self.emit_push_int(value);
                } else {
                    let bytes = i.to_bytes_le().1;
                    self.emit_push(&bytes);
                }
            }
            StackItem::ByteString(bytes) | StackItem::Buffer(bytes) => {
                self.emit_push(&bytes);
            }
            StackItem::Array(items) | StackItem::Struct(items) => {
                // Push items in reverse order, then pack
                let items_len = items.len();
                for item in items.into_iter().rev() {
                    self.emit_push_stack_item(item)?;
                }
                self.emit_push_int(items_len as i64);
                self.emit_pack();
            }
            StackItem::Map(map) => {
                // 1. Emit map size (production map format)
                self.emit_push_int(map.len() as i64);

                // 2. Create new map instruction (production map creation)
                self.emit_opcode(OpCode::NEWMAP);

                // 3. Emit key-value pairs (production map population)
                for (key, value) in map {
                    self.emit_opcode(OpCode::DUP);

                    // Push key and value onto stack
                    self.emit_push_stack_item(key)?;
                    self.emit_push_stack_item(value)?;

                    // Set the key-value pair in the map
                    self.emit_opcode(OpCode::SETITEM);
                }
            }
            StackItem::Pointer(addr) => {
                self.emit_push_int(addr as i64);
            }
            StackItem::InteropInterface(_) => {
                // InteropInterface cannot be serialized to script
                return Err(crate::VmError::invalid_operation_msg(
                    "Cannot serialize InteropInterface to script".to_string(),
                ));
            }
        }

        Ok(self)
    }

    /// Emits a jump operation.
    pub fn emit_jump(&mut self, op: OpCode, offset: i16) -> VmResult<&mut Self> {
        if op != OpCode::JMP && op != OpCode::JMPIF && op != OpCode::JMPIFNOT && op != OpCode::CALL
        {
            return Err(crate::error::VmError::InvalidOperation {
                operation: "emit_jump".to_string(),
                reason: format!("Invalid jump operation: {:?}", op),
            });
        }

        self.emit_opcode(op);

        // Emit offset as little-endian
        self.emit((offset & 0xFF) as u8);
        self.emit(((offset >> 8) & 0xFF) as u8);

        Ok(self)
    }

    /// Emits a call operation.
    pub fn emit_call(&mut self, offset: i16) -> VmResult<&mut Self> {
        self.emit_jump(OpCode::CALL, offset)
    }

    /// Emits a syscall operation.
    pub fn emit_syscall(&mut self, api: &str) -> VmResult<&mut Self> {
        let api_bytes = api.as_bytes();
        let api_bytes_len = api_bytes.len();

        if api_bytes_len > 252 {
            return Err(crate::error::VmError::InvalidOperation {
                operation: "emit_syscall".to_string(),
                reason: format!("Syscall API too long: {} bytes (max 252)", api_bytes_len),
            });
        }

        self.emit_opcode(OpCode::SYSCALL);
        self.emit(api_bytes_len as u8);
        self.script.extend_from_slice(api_bytes);

        Ok(self)
    }

    /// Emits an append operation.
    pub fn emit_append(&mut self) -> &mut Self {
        self.emit_opcode(OpCode::APPEND)
    }

    /// Emits a pack operation.
    pub fn emit_pack(&mut self) -> &mut Self {
        self.emit_opcode(OpCode::PACK)
    }

    /// Converts the builder to a script.
    pub fn to_script(&self) -> Script {
        Script::new_relaxed(self.script.clone())
    }

    /// Converts the builder to a byte array.
    pub fn to_array(&self) -> Vec<u8> {
        self.script.clone()
    }
}

impl Default for ScriptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_opcode() {
        let mut builder = ScriptBuilder::new();
        builder.emit_opcode(OpCode::PUSH1);
        builder.emit_opcode(OpCode::PUSH2);
        builder.emit_opcode(OpCode::ADD);

        let script = builder.to_array();
        assert_eq!(
            script,
            vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8, OpCode::ADD as u8]
        );
    }

    #[test]
    fn test_emit_push_int() {
        let mut builder = ScriptBuilder::new();

        // Special cases
        builder.emit_push_int(-1);
        builder.emit_push_int(0);
        builder.emit_push_int(10);

        // Larger integers
        builder.emit_push_int(100);
        builder.emit_push_int(-100);

        let script = builder.to_array();

        // Check special cases
        assert_eq!(script[0], OpCode::PUSHM1 as u8);
        assert_eq!(script[1], OpCode::PUSH0 as u8);
        assert_eq!(script[2], OpCode::PUSH10 as u8);

        assert!(script.len() > 5);
    }

    #[test]
    fn test_emit_push_bool() {
        let mut builder = ScriptBuilder::new();
        builder.emit_push_bool(true);
        builder.emit_push_bool(false);

        let script = builder.to_array();
        assert_eq!(script, vec![OpCode::PUSH1 as u8, OpCode::PUSH0 as u8]);
    }

    #[test]
    fn test_emit_push_byte_array() {
        let mut builder = ScriptBuilder::new();

        // Small array
        let small_array = [1, 2, 3];
        builder.emit_push_byte_array(&small_array);

        let medium_array = [0; 200];
        builder.emit_push_byte_array(&medium_array);

        let large_array = [0; 65000];
        builder.emit_push_byte_array(&large_array);

        let script = builder.to_array();

        assert_eq!(script[0], 3); // Length as opcode
        assert_eq!(&script[1..4], &[1, 2, 3]);

        assert_eq!(script[4], OpCode::PUSHDATA1 as u8);
        assert_eq!(script[5], 200); // Length as single byte

        let large_array_offset = 4 + 2 + 200;
        assert_eq!(script[large_array_offset], OpCode::PUSHDATA2 as u8);
        assert_eq!(script[large_array_offset + 1], (65000 & 0xFF) as u8);
        assert_eq!(script[large_array_offset + 2], ((65000 >> 8) & 0xFF) as u8);
    }

    #[test]
    fn test_emit_jump() {
        let mut builder = ScriptBuilder::new();
        builder.emit_jump(OpCode::JMP, 10);

        let script = builder.to_array();
        assert_eq!(script, vec![OpCode::JMP as u8, 10, 0]);
    }

    #[test]
    fn test_emit_syscall() {
        let mut builder = ScriptBuilder::new();
        let api_name = "System.Runtime.Log";
        builder.emit_syscall(api_name);

        let script = builder.to_array();

        // Check opcode
        assert_eq!(script[0], OpCode::SYSCALL as u8);

        // Check length - "System.Runtime.Log" is 18 characters
        assert_eq!(script[1], 18);

        let api_bytes = &script[2..20]; // ADDRESS_SIZE = 20
        assert_eq!(api_bytes, b"System.Runtime.Log");
    }

    #[test]
    fn test_to_script() {
        let mut builder = ScriptBuilder::new();
        builder.emit_opcode(OpCode::PUSH1);
        builder.emit_opcode(OpCode::RET);

        let script = builder.to_script();

        assert_eq!(script.len(), 2);
    }
}

//! Script module for the Neo Virtual Machine.
//!
//! This module provides script representation and parsing functionality.

use crate::error::VmError;
use crate::error::VmResult;
use crate::instruction::Instruction;
use crate::op_code::OpCode;
use neo_io::MemoryReader;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

/// Represents a script in the Neo VM.
#[derive(Debug, Clone)]
pub struct Script {
    /// The script data
    script: Vec<u8>,

    /// Cached instructions (wrapped in Arc<Mutex> for safe mutable access)
    instructions: Arc<Mutex<HashMap<usize, Instruction>>>,

    /// Whether strict mode is enabled
    strict_mode: bool,

    /// Cached hash code (wrapped in Arc<Mutex> for safe mutable access)
    hash_code: Arc<Mutex<Option<u64>>>,
}

/// Iterator over the instructions in a script.
pub struct InstructionIterator<'a> {
    script: &'a Script,
    position: usize,
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = VmResult<(usize, Instruction)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.script.len() {
            return None;
        }

        match self.script.get_instruction(self.position) {
            Ok(instruction) => {
                let current_position = self.position;
                self.position += instruction.size();
                Some(Ok((current_position, instruction)))
            }
            Err(error) => Some(Err(error)),
        }
    }
}

impl Script {
    /// Creates a new script with optional validation and strict mode.
    pub fn new(script: Vec<u8>, strict_mode: bool) -> VmResult<Self> {
        let mut s = Self {
            script,
            instructions: Arc::new(Mutex::new(HashMap::new())),
            strict_mode: false, // Start with false to allow parsing
            hash_code: Arc::new(Mutex::new(None)),
        };

        if strict_mode {
            s.populate_instruction_cache()?;
            s.strict_mode = true;
            s.validate_strict()?;
        } else {
            s.validate()?;
        }

        Ok(s)
    }

    /// Creates a new script with default settings (non-strict mode).
    /// This provides backward compatibility for code expecting Script::new(script).
    pub fn from(script: Vec<u8>) -> VmResult<Self> {
        Self::new(script, false)
    }

    /// Creates a new script without validation - backward compatibility with C# API
    /// This matches the C# Script(byte[] script) constructor exactly
    pub fn new_from_bytes(script: Vec<u8>) -> Self {
        Self {
            script,
            instructions: Arc::new(Mutex::new(HashMap::new())),
            strict_mode: false,
            hash_code: Arc::new(Mutex::new(None)),
        }
    }

    /// Creates a new script without validation.
    pub fn new_relaxed(script: Vec<u8>) -> Self {
        Self {
            script,
            instructions: Arc::new(Mutex::new(HashMap::new())),
            strict_mode: false,
            hash_code: Arc::new(Mutex::new(None)),
        }
    }

    /// Populates the instruction cache by parsing all instructions
    fn populate_instruction_cache(&mut self) -> VmResult<()> {
        let mut position = 0;
        let mut instructions = HashMap::new();

        while position < self.script.len() {
            let mut reader = MemoryReader::new(&self.script);
            reader.set_position(position)?;
            let instruction = Instruction::parse_from_neo_io_reader(&mut reader)?;

            instructions.insert(position, instruction.clone());
            position += instruction.size();
        }

        *self
            .instructions
            .lock()
            .map_err(|_| VmError::invalid_operation_msg("Lock poisoned"))? = instructions;
        Ok(())
    }

    /// Validates the script.
    pub fn validate(&self) -> VmResult<()> {
        let mut reader = MemoryReader::new(&self.script);

        while reader.position() < reader.len() {
            let instruction = Instruction::parse_from_neo_io_reader(&mut reader)?;

            let _opcode = instruction.opcode();

            if instruction.pointer() + instruction.size() > self.script.len() {
                return Err(VmError::invalid_instruction_msg(format!(
                    "Instruction at position {} exceeds script bounds",
                    instruction.pointer()
                )));
            }
        }

        Ok(())
    }

    /// Validates the script in strict mode.
    pub fn validate_strict(&self) -> VmResult<()> {
        let instructions = self
            .instructions
            .lock()
            .map_err(|_| VmError::invalid_operation_msg("Lock poisoned"))?;

        // Validate jump targets
        for (&ip, instruction) in instructions.iter() {
            match instruction.opcode() {
                OpCode::JMP
                | OpCode::JMPIF
                | OpCode::JMPIFNOT
                | OpCode::JMPEQ
                | OpCode::JMPNE
                | OpCode::JMPGT
                | OpCode::JMPGE
                | OpCode::JMPLT
                | OpCode::JMPLE
                | OpCode::CALL
                | OpCode::ENDTRY => {
                    let offset = instruction.operand_as::<i8>()?;
                    // Jump offsets are relative to the next instruction
                    let next_ip = ip + instruction.size();
                    let target = (next_ip as i32 + offset as i32) as usize;
                    if !instructions.contains_key(&target) {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid jump target at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }
                }
                OpCode::PUSHA
                | OpCode::JmpL
                | OpCode::JmpifL
                | OpCode::JmpifnotL
                | OpCode::JmpeqL
                | OpCode::JmpneL
                | OpCode::JmpgtL
                | OpCode::JmpgeL
                | OpCode::JmpltL
                | OpCode::JmpleL
                | OpCode::CallL
                | OpCode::EndtryL => {
                    let offset = instruction.operand_as::<i32>()?;
                    // Jump offsets are relative to the next instruction
                    let next_ip = ip + instruction.size();
                    let target = (next_ip as i32 + offset) as usize;
                    if !instructions.contains_key(&target) {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid jump target at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }
                }
                OpCode::TRY => {
                    let catch_offset = instruction.operand_as::<i8>()?;
                    let finally_offset = instruction.operand_as::<i8>()?;

                    // Jump offsets are relative to the next instruction
                    let next_ip = ip + instruction.size();
                    let catch_target = (next_ip as i32 + catch_offset as i32) as usize;
                    let finally_target = (next_ip as i32 + finally_offset as i32) as usize;

                    if !instructions.contains_key(&catch_target) {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid catch target at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }

                    if !instructions.contains_key(&finally_target) {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid finally target at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }
                }
                OpCode::TryL => {
                    let catch_offset = instruction.operand_as::<i32>()?;
                    let finally_offset = instruction.operand_as::<i32>()?;

                    // Jump offsets are relative to the next instruction
                    let next_ip = ip + instruction.size();
                    let catch_target = (next_ip as i32 + catch_offset) as usize;
                    let finally_target = (next_ip as i32 + finally_offset) as usize;

                    if !instructions.contains_key(&catch_target) {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid catch target at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }

                    if !instructions.contains_key(&finally_target) {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid finally target at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }
                }
                OpCode::NewarrayT | OpCode::ISTYPE | OpCode::CONVERT => {
                    let type_byte = instruction.operand_as::<u8>()?;
                    if let Some(item_type) =
                        crate::stack_item::stack_item_type::StackItemType::from_byte(type_byte)
                    {
                        if instruction.opcode() != OpCode::NewarrayT
                            && item_type == crate::stack_item::stack_item_type::StackItemType::Any
                        {
                            return Err(VmError::invalid_script_msg(format!(
                                "Invalid type at position {}: {:?}",
                                ip,
                                instruction.opcode()
                            )));
                        }
                    } else {
                        return Err(VmError::invalid_script_msg(format!(
                            "Invalid type at position {}: {:?}",
                            ip,
                            instruction.opcode()
                        )));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Gets the instruction at the specified position.
    pub fn get_instruction(&self, position: usize) -> VmResult<Instruction> {
        if position >= self.script.len() {
            return Err(VmError::invalid_operation_msg(format!(
                "Position {position} is beyond script bounds"
            )));
        }

        {
            let instructions = self
                .instructions
                .lock()
                .map_err(|_| VmError::invalid_operation_msg("Lock poisoned"))?;
            if let Some(instruction) = instructions.get(&position) {
                return Ok(instruction.clone());
            }
        }

        if self.strict_mode {
            return Err(VmError::invalid_operation_msg(format!(
                "Position {position} not found with strict mode"
            )));
        }

        // Parse the instruction
        let mut reader = MemoryReader::new(&self.script);
        reader.set_position(position)?;
        let instruction = Instruction::parse_from_neo_io_reader(&mut reader)?;

        // Cache the instruction
        {
            let mut instructions = self
                .instructions
                .lock()
                .map_err(|_| VmError::invalid_operation_msg("Lock poisoned"))?;
            instructions.insert(position, instruction.clone());
        }

        Ok(instruction)
    }

    /// Gets a byte at the specified position.
    pub fn get_byte(&self, position: usize) -> VmResult<u8> {
        if position >= self.script.len() {
            return Err(VmError::invalid_operation_msg(format!(
                "Position {position} is beyond script bounds"
            )));
        }

        Ok(self.script[position])
    }

    /// Gets a range of bytes from the script.
    pub fn range(&self, start: usize, end: usize) -> VmResult<Vec<u8>> {
        if start >= self.script.len() || end > self.script.len() || start > end {
            return Err(VmError::invalid_operation_msg(format!(
                "Range {start}..{end} is invalid"
            )));
        }

        Ok(self.script[start..end].to_vec())
    }

    /// Returns the script as a byte array.
    pub fn to_array(&self) -> Vec<u8> {
        self.script.clone()
    }

    /// Returns the script as a byte slice.
    /// This matches the C# implementation's ToArray() behavior exactly.
    pub fn as_bytes(&self) -> &[u8] {
        &self.script
    }

    /// Returns the length of the script.
    pub fn len(&self) -> usize {
        self.script.len()
    }

    /// Returns the length of the script - C# API compatibility
    /// This matches the C# Script.Length property exactly
    pub fn length(&self) -> usize {
        self.script.len()
    }

    /// Returns true if the script is empty.
    pub fn is_empty(&self) -> bool {
        self.script.is_empty()
    }

    /// Returns an iterator over the instructions in the script.
    ///
    /// # Returns
    ///
    /// An iterator over the instructions in the script
    pub fn instructions(&self) -> InstructionIterator {
        InstructionIterator {
            script: self,
            position: 0,
        }
    }

    /// Calculates the offset for a jump instruction.
    ///
    /// # Arguments
    ///
    /// * `next_position` - The position after the jump instruction (where offset is relative to)
    /// * `offset` - The jump offset from the next instruction
    ///
    /// # Returns
    ///
    /// The absolute position after the jump
    pub fn get_jump_offset(&self, next_position: usize, offset: i32) -> VmResult<usize> {
        let new_position = next_position as i32 + offset;

        if new_position < 0 || new_position >= self.script.len() as i32 {
            return Err(VmError::invalid_script_msg("Jump offset out of bounds"));
        }

        Ok(new_position as usize)
    }

    /// Calculates the hash of the script.
    ///
    /// # Returns
    ///
    /// The hash of the script as a byte array
    pub fn hash(&self) -> Vec<u8> {
        {
            let hash_code = self.hash_code.lock().expect("Lock poisoned");
            if let Some(hash) = *hash_code {
                return hash.to_le_bytes().to_vec();
            }
        }

        // Calculate the hash
        let mut hasher = DefaultHasher::new();
        self.script.hash(&mut hasher);
        let hash = hasher.finish();

        // Cache the hash
        {
            let mut hash_code = self.hash_code.lock().expect("Lock poisoned");
            *hash_code = Some(hash);
        }

        // Convert the hash to a byte array
        hash.to_le_bytes().to_vec()
    }

    /// Gets the hash code of the script.
    pub fn hash_code(&self) -> u64 {
        {
            let hash_code = self.hash_code.lock().expect("Lock poisoned");
            if let Some(hash) = *hash_code {
                return hash;
            }
        }

        // Calculate the hash
        let mut hasher = DefaultHasher::new();
        self.script.hash(&mut hasher);
        let hash = hasher.finish();

        // Cache the hash
        {
            let mut hash_code = self.hash_code.lock().expect("Lock poisoned");
            *hash_code = Some(hash);
        }

        hash
    }

    /// Calculates the jump target for a jump instruction.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The jump instruction
    ///
    /// # Returns
    ///
    /// The absolute position of the jump target
    pub fn get_jump_target(&self, instruction: &Instruction) -> VmResult<usize> {
        let opcode = instruction.opcode();
        let position = instruction.pointer();
        let next_position = position + instruction.size();

        match opcode {
            OpCode::JMP | OpCode::JMPIF | OpCode::JMPIFNOT | OpCode::CALL => {
                // 1-byte offset
                let offset = instruction.operand_as::<i8>()?;
                self.get_jump_offset(next_position, offset as i32)
            }
            OpCode::JmpL | OpCode::JmpifL | OpCode::JmpifnotL | OpCode::CallL => {
                // 4-byte offset
                let offset = instruction.operand_as::<i32>()?;
                self.get_jump_offset(next_position, offset)
            }
            OpCode::JMPEQ
            | OpCode::JMPNE
            | OpCode::JMPGT
            | OpCode::JMPGE
            | OpCode::JMPLT
            | OpCode::JMPLE => {
                // 1-byte offset
                let offset = instruction.operand_as::<i8>()?;
                self.get_jump_offset(next_position, offset as i32)
            }
            OpCode::JmpeqL
            | OpCode::JmpneL
            | OpCode::JmpgtL
            | OpCode::JmpgeL
            | OpCode::JmpltL
            | OpCode::JmpleL => {
                // 4-byte offset
                let offset = instruction.operand_as::<i32>()?;
                self.get_jump_offset(next_position, offset)
            }
            _ => Err(VmError::invalid_instruction_msg(format!(
                "Not a jump instruction: {opcode:?}"
            ))),
        }
    }

    /// Calculates the try-catch-finally offsets for a TRY instruction.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The TRY instruction
    ///
    /// # Returns
    ///
    /// A tuple of (catch_offset, finally_offset) as absolute positions
    pub fn get_try_offsets(&self, instruction: &Instruction) -> VmResult<(usize, usize)> {
        let opcode = instruction.opcode();
        let position = instruction.pointer();
        let next_position = position + instruction.size();

        if opcode != OpCode::TRY {
            return Err(VmError::invalid_instruction_msg(format!(
                "Not a TRY instruction: {opcode:?}"
            )));
        }

        // Get the catch and finally offsets
        let catch_offset = instruction.operand_as::<i16>()?;
        let finally_offset = instruction.operand_as::<i16>()?;

        // Calculate the absolute positions
        let catch_position = self.get_jump_offset(next_position, catch_offset as i32)?;
        let finally_position = self.get_jump_offset(next_position, finally_offset as i32)?;

        Ok((catch_position, finally_position))
    }

    /// Gets the next instruction position after the given position.
    ///
    /// # Arguments
    ///
    /// * `position` - The current instruction position
    ///
    /// # Returns
    ///
    /// A tuple of (instruction, next_position)
    pub fn get_next_instruction(&self, position: usize) -> VmResult<(Instruction, usize)> {
        let instruction = self.get_instruction(position)?;
        let next_position = position + instruction.size();
        Ok((instruction, next_position))
    }
}

impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        &self.script
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::OpCode;

    #[test]
    fn test_script_creation_and_validation() {
        // Create a valid script: PUSH1, PUSH2, ADD, RET
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];

        // Create without validation
        let script = Script::new_relaxed(script_bytes.clone());
        assert_eq!(script.len(), 4);

        // Create with validation
        let script = Script::new(script_bytes.clone(), true).unwrap();
        assert_eq!(script.len(), 4);

        // Get an instruction
        let instr = script.get_instruction(0).unwrap();
        assert_eq!(instr.opcode(), OpCode::PUSH1);

        let instr = script.get_instruction(1).unwrap();
        assert_eq!(instr.opcode(), OpCode::PUSH2);

        // Get a byte
        assert_eq!(script.get_byte(0).unwrap(), OpCode::PUSH1 as u8);
        assert_eq!(script.get_byte(1).unwrap(), OpCode::PUSH2 as u8);

        // Get a range
        assert_eq!(
            script.range(0, 2).expect("Operation failed"),
            vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8]
        );

        // Test hash code
        let hash = script.hash_code();
        assert_ne!(hash, 0);

        // Test hash
        let hash_bytes = script.hash();
        assert_eq!(hash_bytes.len(), 8);
    }

    #[test]
    fn test_script_validation_with_invalid_jump() {
        // Create an invalid script: JMP with a target beyond the script bounds
        let script_bytes = vec![
            OpCode::JMP as u8,
            0xFF, // Jump to a position well beyond the script
        ];

        // Create without validation should succeed
        let script = Script::new_relaxed(script_bytes.clone());
        assert_eq!(script.len(), 2);

        // Create with validation should fail
        let result = Script::new(script_bytes.clone(), true);
        assert!(result.is_err());
    }

    #[test]
    fn test_script_instruction_caching() {
        // Create a valid script: PUSH1, PUSH2, ADD, RET
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];

        // Create without validation
        let script = Script::new_relaxed(script_bytes.clone());

        // Get an instruction
        let instr1 = script.get_instruction(0).unwrap();
        assert_eq!(instr1.opcode(), OpCode::PUSH1);

        let instr2 = script.get_instruction(0).unwrap();
        assert_eq!(instr2.opcode(), OpCode::PUSH1);

        // The instructions should be the same object
        assert_eq!(instr1.pointer(), instr2.pointer());
        assert_eq!(instr1.opcode(), instr2.opcode());
    }

    #[test]
    fn test_script_with_valid_jumps() {
        // Create a valid script with jumps
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::JMP as u8,
            0x00, // 1: Jump to position 3 (offset 0 from next instruction at 3)
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];

        // Create with validation
        let script = Script::new(script_bytes.clone(), true).unwrap();

        // Get the jump target
        let jump_instr = script.get_instruction(1).unwrap();
        let target = script.get_jump_target(&jump_instr).unwrap();

        // The target should be position 3
        assert_eq!(target, 3);

        // The instruction at the target should be PUSH2
        let target_instr = script.get_instruction(target).unwrap();
        assert_eq!(target_instr.opcode(), OpCode::PUSH2);
    }
}

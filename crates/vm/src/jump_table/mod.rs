//! Jump table module for the Neo Virtual Machine.
//!
//! This module provides the jump table implementation used in the Neo VM.

pub mod bitwise;
pub mod compound;
pub mod control;
pub mod crypto;
pub mod numeric;
pub mod push;
pub mod slot;
pub mod splice;
pub mod stack;
pub mod types;

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::op_code::OpCode;

/// A handler for a VM instruction.
pub type InstructionHandler = fn(&mut ExecutionEngine, &Instruction) -> VmResult<()>;

/// Represents a jump table for the VM.
#[derive(Clone)]
pub struct JumpTable {
    /// The handlers for each opcode.
    /// Uses a fixed-size array of 256 entries (one for each possible byte value)
    /// exactly matching the C# implementation which uses DelAction[] Table = new DelAction[byte.MaxValue]
    handlers: [Option<InstructionHandler>; 256],
}

impl Default for JumpTable {
    fn default() -> Self {
        Self::new()
    }
}

use std::sync::OnceLock;

/// The default jump table.
static DEFAULT: OnceLock<JumpTable> = OnceLock::new();

impl JumpTable {
    /// Creates a new jump table.
    pub fn new() -> Self {
        let mut jump_table = Self {
            handlers: [None; 256],
        };

        // Register default handlers
        jump_table.register_default_handlers();

        jump_table
    }

    /// Gets the default jump table.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        // Use OnceLock for safe one-time initialization
        DEFAULT.get_or_init(|| Self::new()).clone()
    }

    /// Registers a handler for an opcode.
    pub fn register(&mut self, opcode: OpCode, handler: InstructionHandler) {
        self.set_handler(opcode, handler);
    }

    /// Gets the handler for an opcode.
    pub fn get(&self, opcode: OpCode) -> Option<InstructionHandler> {
        self.get_handler(opcode)
    }

    /// Gets the handler for an opcode.
    /// This matches the C# implementation's indexer get accessor.
    pub fn get_handler(&self, opcode: OpCode) -> Option<InstructionHandler> {
        self.handlers[opcode as usize]
    }

    /// Sets the handler for an opcode.
    /// This matches the C# implementation's indexer set accessor.
    pub fn set_handler(&mut self, opcode: OpCode, handler: InstructionHandler) {
        self.handlers[opcode as usize] = Some(handler);
    }

    /// Sets the handler for an opcode.
    /// Alias for set_handler for convenience.
    pub fn set(&mut self, opcode: OpCode, handler: InstructionHandler) {
        self.set_handler(opcode, handler);
    }

    /// Executes an instruction.
    pub fn execute(&self, engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
        if let Some(handler) = self.get_handler(instruction.opcode()) {
            handler(engine, instruction)
        } else {
            self.invalid_opcode(engine, instruction)
        }
    }

    /// Handles an invalid opcode.
    pub fn invalid_opcode(
        &self,
        _engine: &mut ExecutionEngine,
        instruction: &Instruction,
    ) -> VmResult<()> {
        Err(VmError::unsupported_operation_msg(format!(
            "Unsupported opcode: {:?}",
            instruction.opcode()
        )))
    }

    /// Executes a throw operation.
    pub fn execute_throw(&self, engine: &mut ExecutionEngine, message: &str) -> VmResult<()> {
        let exception = crate::stack_item::StackItem::from_byte_string(message.as_bytes().to_vec());

        // Set the uncaught exception
        engine.set_uncaught_exception(Some(exception));

        if !engine.handle_exception() {
            // No exception handler found, set VM state to FAULT
            engine.set_state(crate::execution_engine::VMState::FAULT);
        }

        Ok(())
    }

    /// Registers the default handlers for all opcodes.
    fn register_default_handlers(&mut self) {
        // Register bitwise handlers
        bitwise::register_handlers(self);

        // Register compound handlers
        compound::register_handlers(self);

        // Register control handlers
        control::register_handlers(self);

        // Register crypto handlers
        crypto::register_handlers(self);

        // Register numeric handlers
        numeric::register_handlers(self);

        // Register push handlers
        push::register_handlers(self);

        // Register slot handlers
        slot::register_handlers(self);

        // Register splice handlers
        splice::register_handlers(self);

        // Register stack handlers
        stack::register_handlers(self);

        // Register types handlers
        types::register_handlers(self);
    }
}

impl std::ops::Index<OpCode> for JumpTable {
    type Output = InstructionHandler;

    fn index(&self, opcode: OpCode) -> &Self::Output {
        self.handlers[opcode as usize]
            .as_ref()
            .expect("Unsupported opcode")
    }
}

impl std::ops::IndexMut<OpCode> for JumpTable {
    fn index_mut(&mut self, opcode: OpCode) -> &mut Self::Output {
        // We need to ensure the handler exists first
        if self.handlers[opcode as usize].is_none() {
            self.handlers[opcode as usize] = Some(
                |_engine: &mut ExecutionEngine, instruction: &Instruction| -> VmResult<()> {
                    Err(VmError::unsupported_operation_msg(format!(
                        "Unsupported opcode: {:?}",
                        instruction.opcode()
                    )))
                },
            );
        }

        // Now we can safely get a mutable reference
        self.handlers[opcode as usize]
            .as_mut()
            .expect("Unsupported opcode")
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_table_creation() {
        let jump_table = JumpTable::new();

        // Check that all opcodes have handlers
        for opcode in OpCode::iter() {
            assert!(
                jump_table.get(opcode).is_some(),
                "No handler for opcode: {:?}",
                opcode
            );
        }
    }

    #[test]
    fn test_jump_table_register() -> Result<(), Box<dyn std::error::Error>> {
        let mut jump_table = JumpTable::new();

        // Define a custom handler
        fn custom_handler(
            _engine: &mut ExecutionEngine,
            _instruction: &Instruction,
        ) -> VmResult<()> {
            Ok(())
        }

        // Register the custom handler
        jump_table.register(OpCode::NOP, custom_handler);

        // Check that the handler was registered
        assert_eq!(
            jump_table.get(OpCode::NOP).ok_or("Index out of bounds")? as usize,
            custom_handler as usize
        );
        Ok(())
    }

    #[test]
    fn test_jump_table_index() -> Result<(), Box<dyn std::error::Error>> {
        let mut jump_table = JumpTable::new();

        // Define a custom handler
        fn custom_handler(
            _engine: &mut ExecutionEngine,
            _instruction: &Instruction,
        ) -> VmResult<()> {
            Ok(())
        }

        // Set the handler using the index operator
        jump_table[OpCode::NOP] = custom_handler;

        // Check that the handler was set
        assert_eq!(
            jump_table.get(OpCode::NOP).ok_or("Index out of bounds")? as usize,
            custom_handler as usize
        );
        Ok(())
    }

    #[test]
    fn test_jump_table_default() {
        // Get the default jump table
        let jump_table = JumpTable::default();

        // Check that all opcodes have handlers
        for opcode in OpCode::iter() {
            assert!(
                jump_table.get(opcode).is_some(),
                "No handler for opcode: {:?}",
                opcode
            );
        }
    }

    #[test]
    fn test_jump_table_invalid_opcode() {
        let jump_table = JumpTable::new();

        // Create a mock engine and instruction
        let mut engine = ExecutionEngine::new(None);
        let instruction = Instruction {
            pointer: 0,
            opcode: OpCode::NOP,
            operand: vec![],
        };

        let mut jump_table = jump_table.clone();
        jump_table.handlers[OpCode::NOP as usize] = None;

        // Execute the instruction
        let result = jump_table.execute(&mut engine, &instruction);

        assert!(result.is_err());
    }
}

//! Execution engine module for the Neo Virtual Machine.
//!
//! This module provides the execution engine implementation for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::evaluation_stack::EvaluationStack;
use crate::execution_context::ExecutionContext;
use crate::gas_calculator::{GasCalculator, GasError};
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::reference_counter::ReferenceCounter;
use crate::script::Script;
use crate::stack_item::StackItem;
use neo_config::{HASH_SIZE, MAX_SCRIPT_SIZE};

/// The VM state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VMState {
    /// The VM is not running.
    NONE = 0,

    /// The VM has halted normally.
    HALT = 1,

    /// The VM has encountered an error.
    FAULT = 2,

    /// The VM is in a debug break state.
    BREAK = 4,
}

/// Restrictions on the VM.
#[derive(Debug, Clone)]
pub struct ExecutionEngineLimits {
    /// The maximum number of items allowed on a stack.
    pub max_stack_size: usize,

    /// The maximum size of an item in bytes.
    pub max_item_size: usize,

    /// The maximum number of frames allowed on the invocation stack.
    pub max_invocation_stack_size: usize,

    /// Whether to catch engine exceptions.
    pub catch_engine_exceptions: bool,
}

impl ExecutionEngineLimits {
    /// The default execution engine limits.
    pub const DEFAULT: Self = Self {
        max_stack_size: 2048,
        max_item_size: MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE,
        max_invocation_stack_size: MAX_SCRIPT_SIZE,
        catch_engine_exceptions: true,
    };
}

impl Default for ExecutionEngineLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The execution engine for the Neo VM.
pub struct ExecutionEngine {
    /// The current state of the VM
    state: VMState,

    /// Flag indicating if the engine is in the middle of a jump
    pub is_jumping: bool,

    /// The jump table used to execute instructions
    jump_table: JumpTable,

    /// Restrictions on the VM
    limits: ExecutionEngineLimits,

    /// Used for reference counting of objects in the VM
    reference_counter: ReferenceCounter,

    /// Gas calculator for execution cost tracking (matches C# ApplicationEngine)
    gas_calculator: GasCalculator,

    /// The invocation stack of the VM
    invocation_stack: Vec<ExecutionContext>,

    /// The stack to store the return values
    result_stack: EvaluationStack,

    /// The VM object representing the uncaught exception
    uncaught_exception: Option<StackItem>,
}

impl ExecutionEngine {
    /// Creates a new execution engine with the specified jump table.
    pub fn new(jump_table: Option<JumpTable>) -> Self {
        let reference_counter = ReferenceCounter::new();
        Self::new_with_limits(
            jump_table,
            reference_counter,
            ExecutionEngineLimits::default(),
        )
    }

    /// Creates a new execution engine with the specified reference counter and limits.
    pub fn new_with_limits(
        jump_table: Option<JumpTable>,
        reference_counter: ReferenceCounter,
        limits: ExecutionEngineLimits,
    ) -> Self {
        Self {
            state: VMState::BREAK,
            is_jumping: false,
            jump_table: jump_table.unwrap_or_else(JumpTable::default),
            limits,
            reference_counter: reference_counter.clone(),
            gas_calculator: GasCalculator::new(1_000_000_000, 30), // Default gas limit and fee factor
            invocation_stack: Vec::new(),
            result_stack: EvaluationStack::new(reference_counter),
            uncaught_exception: None,
        }
    }

    /// Returns the current state of the VM.
    pub fn state(&self) -> VMState {
        self.state
    }

    /// Sets the state of the VM.
    pub fn set_state(&mut self, state: VMState) {
        if self.state != state {
            self.state = state;
            self.on_state_changed();
        }
    }

    /// Called when the VM state changes.
    fn on_state_changed(&mut self) {}

    /// Called when an exception causes the VM to enter the FAULT state.
    fn on_fault(&mut self, err: VmError) {
        // Set the fault exception
        let message = err.to_string();
        let exception = StackItem::from_byte_string(message.as_bytes().to_vec());
        self.uncaught_exception = Some(exception);

        // Set the state to FAULT
        self.set_state(VMState::FAULT);
    }

    /// Returns the reference counter.
    pub fn reference_counter(&self) -> &ReferenceCounter {
        &self.reference_counter
    }

    /// Returns the invocation stack.
    pub fn invocation_stack(&self) -> &[ExecutionContext] {
        &self.invocation_stack
    }

    /// Returns the current context, if any.
    pub fn current_context(&self) -> Option<&ExecutionContext> {
        self.invocation_stack.last()
    }

    /// Returns the current context (mutable), if any.
    pub fn current_context_mut(&mut self) -> Option<&mut ExecutionContext> {
        self.invocation_stack.last_mut()
    }

    /// Returns the entry context, if any.
    pub fn entry_context(&self) -> Option<&ExecutionContext> {
        self.invocation_stack.first()
    }

    /// Returns the result stack.
    pub fn result_stack(&self) -> &EvaluationStack {
        &self.result_stack
    }

    /// Returns the result stack (mutable).
    pub fn result_stack_mut(&mut self) -> &mut EvaluationStack {
        &mut self.result_stack
    }

    /// Returns the uncaught exception, if any.
    pub fn uncaught_exception(&self) -> Option<&StackItem> {
        self.uncaught_exception.as_ref()
    }

    /// Sets the uncaught exception.
    pub fn set_uncaught_exception(&mut self, exception: Option<StackItem>) {
        self.uncaught_exception = exception;
    }

    /// Gets the uncaught exception (matches C# UncaughtException property exactly).
    pub fn get_uncaught_exception(&self) -> Option<&StackItem> {
        self.uncaught_exception.as_ref()
    }

    /// Handles an exception by setting it as uncaught and transitioning to FAULT state.
    /// Returns true if the exception was handled, false otherwise.
    /// This matches C# exception handling behavior exactly.
    pub fn handle_exception(&mut self) -> bool {
        if self.uncaught_exception.is_some() {
            self.set_state(VMState::FAULT);
            true
        } else {
            false
        }
    }

    /// Gets the interop service for this engine.
    /// Returns None for base ExecutionEngine (ApplicationEngine overrides this).
    /// This matches C# ExecutionEngine.InteropService behavior exactly.
    pub fn interop_service(&self) -> Option<&dyn crate::interop_service::InteropServiceTrait> {
        None
    }

    /// Returns the jump table.
    pub fn jump_table(&self) -> &JumpTable {
        &self.jump_table
    }

    /// Returns the jump table (mutable).
    pub fn jump_table_mut(&mut self) -> &mut JumpTable {
        &mut self.jump_table
    }

    /// Sets the jump table.
    pub fn set_jump_table(&mut self, jump_table: JumpTable) {
        self.jump_table = jump_table;
    }

    /// Starts execution of the VM.
    pub fn execute(&mut self) -> VMState {
        if self.state == VMState::BREAK {
            self.set_state(VMState::NONE);
        }

        // Execute until HALT or FAULT
        while self.state != VMState::HALT && self.state != VMState::FAULT {
            if let Err(err) = self.execute_next() {
                self.on_fault(err);
            }
        }

        self.state
    }

    /// Executes the next instruction.
    pub fn execute_next(&mut self) -> VmResult<()> {
        if self.state == VMState::HALT || self.state == VMState::FAULT {
            return Ok(());
        }

        self.is_jumping = false;

        // Get the current context
        let context = self
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

        if context.instruction_pointer() >= context.script().len() {
            // Perform implicit RET when reaching end of script
            // Get return value count from the current context
            let rvcount = context.rvcount();

            if rvcount != 0 {
                let eval_stack_len = context.evaluation_stack().len();

                // Collect items to transfer
                let mut items = Vec::new();

                if rvcount == -1 {
                    // Return all items
                    for i in 0..eval_stack_len {
                        if let Ok(item) = context.evaluation_stack().peek(i as isize) {
                            items.push(item.clone());
                        }
                    }
                } else if rvcount > 0 {
                    // Return specific number of items
                    let count = (rvcount as usize).min(eval_stack_len);
                    for i in 0..count {
                        if let Ok(item) = context.evaluation_stack().peek(i as isize) {
                            items.push(item.clone());
                        }
                    }
                }

                // Push to result stack in reverse order
                items.reverse();
                for item in items {
                    self.result_stack.push(item);
                }
            }

            // Remove the current context
            let context_index = self.invocation_stack.len() - 1;
            self.remove_context(context_index)?;

            // If no more contexts, halt
            if self.invocation_stack.is_empty() {
                self.set_state(VMState::HALT);
            }

            return Ok(());
        }

        // Get the current instruction
        let instruction = context.current_instruction()?;

        // Get the instruction handler
        let handler = self
            .jump_table
            .get_handler(instruction.opcode())
            .ok_or_else(|| {
                VmError::invalid_operation_msg(format!(
                    "No handler for opcode: {:?}",
                    instruction.opcode()
                ))
            })?;

        // Execute the instruction
        handler(self, &instruction)?;

        if !self.is_jumping {
            if let Some(context) = self.current_context_mut() {
                let next_position = context.instruction_pointer() + instruction.size();
                context.set_instruction_pointer(next_position);
            }
        }

        Ok(())
    }

    /// Executes the next instruction - C# API compatibility
    /// This matches the C# ExecutionEngine.ExecuteNextInstruction() method exactly
    pub fn execute_next_instruction(&mut self) -> VmResult<()> {
        self.execute_next()
    }

    /// Executes the next instruction in step mode (for debugging/testing).
    /// This matches C# ExecuteNext behavior for step-by-step execution.
    pub fn step_next(&mut self) -> VMState {
        if self.invocation_stack.is_empty() {
            self.set_state(VMState::HALT);
            return self.state;
        }

        // Try to execute the next instruction
        match self.execute_next_internal() {
            Ok(_) => {
                // unless we're already in HALT or FAULT state
                if self.state != VMState::HALT && self.state != VMState::FAULT {
                    self.set_state(VMState::BREAK);
                }
                self.state
            }
            Err(err) => {
                self.on_fault(err);
                self.state
            }
        }
    }

    /// Internal implementation of execute_next.
    fn execute_next_internal(&mut self) -> VmResult<()> {
        // Get the current context
        let context = self
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

        // Get the current instruction
        let instruction = match context.current_instruction() {
            Ok(instruction) => instruction,
            Err(err) => {
                let error_msg = format!("{err:?}");
                if error_msg.contains("Instruction pointer is out of range") {
                    Instruction::ret()
                } else {
                    // Instruction parsing error - this should cause a FAULT
                    return Err(err);
                }
            }
        };

        self.pre_execute_instruction(&instruction)?;

        // Execute the instruction
        // We need to avoid borrowing conflicts by extracting the jump table temporarily
        // But we must preserve the custom handlers that were set up
        let jump_table = std::mem::take(&mut self.jump_table);
        let result = jump_table.execute(self, &instruction);

        if let Err(err) = result {
            if self.limits.catch_engine_exceptions {
                // Execute the throw operation
                let throw_result = jump_table.execute_throw(self, &err.to_string());
                self.jump_table = jump_table;
                throw_result?;
            } else {
                self.jump_table = jump_table;
                return Err(err);
            }
        } else {
            self.jump_table = jump_table;
        }

        self.post_execute_instruction(&instruction)?;

        if !self.is_jumping {
            if let Some(context) = self.current_context_mut() {
                let _ = context.move_next(); // Ignore errors here
            }
        }
        self.is_jumping = false;

        Ok(())
    }

    /// Called before executing an instruction.
    fn pre_execute_instruction(&mut self, instruction: &Instruction) -> VmResult<()> {
        // Consume gas for instruction execution (matches C# ApplicationEngine exactly)
        if let Err(gas_error) = self.gas_calculator.consume_gas(instruction.opcode()) {
            return Err(VmError::invalid_operation_msg(format!(
                "Gas limit exceeded: {}",
                gas_error
            )));
        }

        // Record instruction execution in metrics
        if let Ok(metrics) = std::env::var("NEO_VM_METRICS") {
            if metrics == "1" {
                crate::metrics::global_metrics().record_instruction();
            }
        }
        Ok(())
    }

    /// Called after executing an instruction.
    fn post_execute_instruction(&mut self, _instruction: &Instruction) -> VmResult<()> {
        // Update stack depth metrics
        if let Ok(metrics) = std::env::var("NEO_VM_METRICS") {
            if metrics == "1" {
                let depth = if let Some(context) = self.current_context() {
                    context.evaluation_stack().len()
                } else {
                    0
                };
                crate::metrics::global_metrics().update_stack_depth(depth);
            }
        }
        Ok(())
    }

    /// Loads a context into the invocation stack.
    pub fn load_context(&mut self, context: ExecutionContext) -> VmResult<()> {
        if self.invocation_stack.len() >= self.limits.max_invocation_stack_size {
            return Err(VmError::invalid_operation_msg(format!(
                "MaxInvocationStackSize exceed: {}",
                self.invocation_stack.len()
            )));
        }

        // Push the context onto the invocation stack
        self.invocation_stack.push(context);

        Ok(())
    }

    /// Unloads a context from the invocation stack.
    pub fn unload_context(&mut self, context: &mut ExecutionContext) -> VmResult<()> {
        // Update current context
        if self.invocation_stack.is_empty() {
            // No more contexts
        } else {
            // Get the new current context
        }

        if let Some(static_fields) = context.static_fields_mut() {
            let current_static_fields = self
                .current_context()
                .and_then(|ctx| ctx.static_fields())
                .map(|sf| sf as *const _);

            if current_static_fields.is_none()
                || !std::ptr::eq(
                    current_static_fields.expect("Operation failed"),
                    static_fields,
                )
            {
                static_fields.clear_references();
            }
        }

        if let Some(local_variables) = context.local_variables_mut() {
            local_variables.clear_references();
        }

        if let Some(arguments) = context.arguments_mut() {
            arguments.clear_references();
        }

        Ok(())
    }

    /// Removes a context from the invocation stack.
    pub fn remove_context(&mut self, context_index: usize) -> VmResult<ExecutionContext> {
        // Get the context
        if context_index >= self.invocation_stack.len() {
            return Err(VmError::invalid_operation_msg("Context index out of range"));
        }

        // Remove the context
        let mut context = self.invocation_stack.remove(context_index);

        if self.invocation_stack.is_empty() {
            self.set_state(VMState::HALT);
        }

        // Unload the context
        self.unload_context(&mut context)?;

        self.reference_counter.check_zero_referred();

        Ok(context)
    }

    /// Creates a new context with the specified script.
    pub fn create_context(
        &self,
        script: Script,
        rvcount: i32,
        initial_position: usize,
    ) -> ExecutionContext {
        let mut context = ExecutionContext::new(script, rvcount, &self.reference_counter);
        context.set_instruction_pointer(initial_position);
        context
    }

    /// Loads a script and creates a new context.
    pub fn load_script(
        &mut self,
        script: Script,
        rvcount: i32,
        initial_position: usize,
    ) -> VmResult<ExecutionContext> {
        let context = self.create_context(script, rvcount, initial_position);
        self.load_context(context)?;

        // Return a reference to the loaded context
        self.current_context()
            .ok_or_else(|| VmError::InvalidOperation {
                operation: "load_script_and_return_context".to_string(),
                reason: "No current execution context after loading".to_string(),
            })
            .map(|ctx| ctx.clone())
    }

    /// Returns the item at the specified index from the top of the current stack without removing it.
    pub fn peek(&self, index: isize) -> VmResult<&StackItem> {
        let context = self
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        context.evaluation_stack().peek(index)
    }

    /// Removes and returns the item at the top of the current stack.
    pub fn pop(&mut self) -> VmResult<StackItem> {
        let context = self
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        context.evaluation_stack_mut().pop()
    }

    /// Pushes an item onto the top of the current stack.
    pub fn push(&mut self, item: StackItem) -> VmResult<()> {
        let context = self
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        context.evaluation_stack_mut().push(item);
        Ok(())
    }

    /// Adds gas consumed (integrated with gas calculator)
    /// ApplicationEngine overrides this with additional gas tracking
    pub fn add_gas_consumed(&mut self, gas: i64) -> VmResult<()> {
        if let Err(gas_error) = self.gas_calculator.add_gas(gas) {
            return Err(VmError::invalid_operation_msg(format!(
                "Gas limit exceeded: {}",
                gas_error
            )));
        }
        Ok(())
    }

    /// Gets gas consumed (integrated with gas calculator)
    /// ApplicationEngine overrides this with additional gas tracking
    pub fn gas_consumed(&self) -> i64 {
        self.gas_calculator.gas_consumed()
    }

    /// Gets gas limit (integrated with gas calculator)
    /// ApplicationEngine overrides this with actual gas limit
    pub fn gas_limit(&self) -> i64 {
        self.gas_calculator.gas_limit()
    }

    /// Gets current script hash (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual script hash tracking
    pub fn current_script_hash(&self) -> Option<&[u8]> {
        // Base implementation returns None
        None
    }

    /// Gets script container (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual script container
    pub fn get_script_container(&self) -> Option<&dyn std::any::Any> {
        // Base implementation returns None
        None
    }

    /// Gets the script container hash for signature verification.
    /// Returns the hash of the current transaction or block being executed.
    pub fn get_script_container_hash(&self) -> Vec<u8> {
        // Base implementation returns empty hash
        // ApplicationEngine overrides this with actual container hash
        vec![0u8; HASH_SIZE]
    }

    /// Attempts to cast this ExecutionEngine to an ApplicationEngine (immutable)
    /// Returns None for base ExecutionEngine, Some for ApplicationEngine
    pub fn as_application_engine(&self) -> Option<&crate::application_engine::ApplicationEngine> {
        // Base ExecutionEngine cannot be cast to ApplicationEngine
        None
    }

    /// Attempts to cast this ExecutionEngine to an ApplicationEngine (mutable)
    /// Returns None for base ExecutionEngine, Some for ApplicationEngine
    pub fn as_application_engine_mut(
        &mut self,
    ) -> Option<&mut crate::application_engine::ApplicationEngine> {
        // Base ExecutionEngine cannot be cast to ApplicationEngine
        None
    }

    /// Gets the trigger type for this execution (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual trigger type
    pub fn get_trigger_type(&self) -> u8 {
        0x40
    }

    /// Emits a runtime log event (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual event emission
    pub fn emit_runtime_log_event(&mut self, _message: &str) -> VmResult<()> {
        // Base implementation does nothing
        Ok(())
    }

    /// Adds an execution log (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual log tracking
    pub fn add_execution_log(&mut self, _message: String) -> VmResult<()> {
        // Base implementation does nothing
        Ok(())
    }

    /// Gets transaction hash (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual transaction access
    pub fn get_transaction_hash(&self) -> Option<Vec<u8>> {
        // Base implementation returns None
        None
    }

    /// Gets transaction by hash (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual blockchain access
    pub fn get_transaction_by_hash(
        &self,
        _hash: &[u8],
    ) -> Option<crate::jump_table::control::Transaction> {
        // Base implementation returns None
        None
    }

    /// Gets current block hash (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual blockchain access
    pub fn get_current_block_hash(&self) -> Option<Vec<u8>> {
        // Base implementation returns None
        None
    }

    /// Gets block by hash (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual blockchain access
    pub fn get_block_by_hash(&self, _hash: &[u8]) -> Option<crate::jump_table::control::Block> {
        // Base implementation returns None
        None
    }

    /// Gets storage item (stub implementation for base ExecutionEngine)
    /// ApplicationEngine overrides this with actual storage access
    pub fn get_storage_item(&self, _key: &[u8]) -> Option<Vec<u8>> {
        // Base implementation returns None
        None
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        // Clear the invocation stack
        self.invocation_stack.clear();
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::op_code::OpCode;

    #[test]
    fn test_execution_engine_creation() {
        let engine = ExecutionEngine::new(None);
        assert_eq!(engine.state(), VMState::BREAK);
        assert!(engine.invocation_stack().is_empty());
        assert!(engine.result_stack().is_empty());
        assert!(engine.uncaught_exception().is_none());
    }

    #[test]
    fn test_load_script() {
        let mut engine = ExecutionEngine::new(None);

        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];
        let script = Script::new_relaxed(script_bytes);

        let context = engine
            .load_script(script, -1, 0)
            .expect("VM operation should succeed");

        assert_eq!(engine.invocation_stack().len(), 1);
        assert_eq!(context.instruction_pointer(), 0);
        assert_eq!(context.rvcount(), -1);
    }

    #[test]
    fn test_set_state() {
        let mut engine = ExecutionEngine::new(None);
        assert_eq!(engine.state(), VMState::BREAK);

        engine.set_state(VMState::NONE);
        assert_eq!(engine.state(), VMState::NONE);

        engine.set_state(VMState::HALT);
        assert_eq!(engine.state(), VMState::HALT);

        engine.set_state(VMState::FAULT);
        assert_eq!(engine.state(), VMState::FAULT);
    }

    #[test]
    fn test_jump_table_methods() {
        let mut engine = ExecutionEngine::new(None);

        // Test jump_table getter
        let _jump_table = engine.jump_table();

        // Test jump_table_mut getter
        let _jump_table_mut = engine.jump_table_mut();

        // Test set_jump_table
        let new_jump_table = JumpTable::new();
        engine.set_jump_table(new_jump_table);
    }

    #[test]
    fn test_stack_operations() {
        let mut engine = ExecutionEngine::new(None);

        // Create a script with a few instructions
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];
        let script = Script::new_relaxed(script_bytes);

        // Load the script
        engine
            .load_script(script, -1, 0)
            .expect("VM operation should succeed");

        // Push some items onto the stack
        engine
            .push(StackItem::from_int(1))
            .expect("VM operation should succeed");
        engine
            .push(StackItem::from_int(2))
            .expect("VM operation should succeed");
        engine
            .push(StackItem::from_int(3))
            .expect("VM operation should succeed");

        // Peek at the items
        assert_eq!(
            engine
                .peek(0)
                .expect("intermediate value should exist")
                .as_int()
                .expect("VM operation should succeed"),
            num_bigint::BigInt::from(3)
        );
        assert_eq!(
            engine
                .peek(1)
                .expect("intermediate value should exist")
                .as_int()
                .expect("VM operation should succeed"),
            num_bigint::BigInt::from(2)
        );
        assert_eq!(
            engine
                .peek(2)
                .expect("intermediate value should exist")
                .as_int()
                .expect("VM operation should succeed"),
            num_bigint::BigInt::from(1)
        );

        // Pop an item
        let item = engine.pop().unwrap();
        assert_eq!(
            item.as_int().expect("Operation failed"),
            num_bigint::BigInt::from(3)
        );

        // Peek again
        assert_eq!(
            engine
                .peek(0)
                .expect("intermediate value should exist")
                .as_int()
                .expect("Operation failed"),
            num_bigint::BigInt::from(2)
        );
        assert_eq!(
            engine
                .peek(1)
                .expect("intermediate value should exist")
                .as_int()
                .expect("Operation failed"),
            num_bigint::BigInt::from(1)
        );
    }

    #[test]
    fn test_unload_context() {
        let mut engine = ExecutionEngine::new(None);

        // Create a script with a few instructions
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];
        let script = Script::new_relaxed(script_bytes);

        // Load the script
        let context = engine
            .load_script(script, -1, 0)
            .expect("VM operation should succeed");

        // Push some items onto the stack
        engine
            .push(StackItem::from_int(1))
            .expect("VM operation should succeed");
        engine
            .push(StackItem::from_int(2))
            .expect("VM operation should succeed");

        // Remove the context
        let mut context = engine
            .remove_context(0)
            .expect("VM operation should succeed");

        // Check that the invocation stack is empty
        assert!(engine.invocation_stack().is_empty());

        // Check that the VM state is HALT
        assert_eq!(engine.state(), VMState::HALT);
    }
}

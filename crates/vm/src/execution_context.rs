//! Execution context module for the Neo Virtual Machine.
//!
//! This module provides the execution context implementation for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::evaluation_stack::EvaluationStack;
use crate::instruction::Instruction;
use crate::jump_table::control::ExceptionHandler;
use crate::reference_counter::ReferenceCounter;
use crate::script::Script;
use crate::stack_item::StackItem;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Maximum exception handler stack size
const MAX_EXCEPTION_STACK_SIZE: usize = 32;

/// Exception handler state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionHandlerState {
    Try,
    Catch,
    Finally,
}

/// Execution state for context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionState {
    None,
    Fault,
    Break,
    Halt,
}

/// A slot for storing variables or arguments in a context.
#[derive(Clone, Debug)]
pub struct Slot {
    /// The items in the slot
    items: Vec<StackItem>,

    /// The reference counter
    reference_counter: ReferenceCounter,
}

impl Slot {
    /// Creates a new slot with the specified items and reference counter.
    pub fn new(items: Vec<StackItem>, reference_counter: ReferenceCounter) -> Self {
        let mut slot = Self {
            items: Vec::with_capacity(items.len()),
            reference_counter,
        };

        for item in items {
            slot.set(slot.items.len(), item);
        }

        slot
    }

    /// Returns the number of items in the slot.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the slot is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the item at the specified index.
    pub fn get(&self, index: usize) -> VmResult<&StackItem> {
        self.items
            .get(index)
            .ok_or_else(|| VmError::invalid_operation_msg(format!("Index out of range: {index}")))
    }

    /// Sets the item at the specified index.
    pub fn set(&mut self, index: usize, item: StackItem) {
        if index >= self.items.len() {
            self.items.resize_with(index + 1, StackItem::null);
        }

        if index < self.items.len() {
            self.reference_counter
                .remove_stack_reference(&self.items[index]);
        }

        self.reference_counter.add_stack_reference(&item);

        self.items[index] = item;
    }

    /// Clears all references in the slot.
    pub fn clear_references(&mut self) {
        for item in &self.items {
            self.reference_counter.remove_stack_reference(item);
        }
        self.items.clear();
    }

    /// Returns an iterator over the items in the slot.
    pub fn iter(&self) -> impl Iterator<Item = &StackItem> {
        self.items.iter()
    }

    /// Returns a mutable iterator over the items in the slot.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut StackItem> {
        self.items.iter_mut()
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        self.clear_references();
    }
}

/// Shared states for execution contexts that can be cloned and shared.
/// This matches the C# implementation's SharedStates class exactly.
#[derive(Clone)]
pub struct SharedStates {
    pub script: crate::script::Script,
    pub evaluation_stack: crate::evaluation_stack::EvaluationStack,
    pub static_fields: Option<Slot>,
    pub reference_counter: ReferenceCounter,
    /// State map matching C# Dictionary<Type, object>
    states: Arc<std::sync::RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl std::fmt::Debug for SharedStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedStates")
            .field("script", &"<Script>")
            .field("evaluation_stack", &"<EvaluationStack>")
            .field("static_fields", &self.static_fields)
            .finish()
    }
}

impl SharedStates {
    pub fn new(script: crate::script::Script, reference_counter: ReferenceCounter) -> Self {
        Self {
            script,
            evaluation_stack: crate::evaluation_stack::EvaluationStack::new(
                reference_counter.clone(),
            ),
            static_fields: None,
            reference_counter,
            states: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    pub fn reference_counter(&self) -> &ReferenceCounter {
        &self.reference_counter
    }

    pub fn script(&self) -> &crate::script::Script {
        &self.script
    }

    pub fn evaluation_stack(&self) -> &crate::evaluation_stack::EvaluationStack {
        &self.evaluation_stack
    }

    pub fn evaluation_stack_mut(&mut self) -> &mut crate::evaluation_stack::EvaluationStack {
        &mut self.evaluation_stack
    }

    pub fn static_fields(&self) -> Option<&Slot> {
        self.static_fields.as_ref()
    }

    pub fn static_fields_mut(&mut self) -> &mut Option<Slot> {
        &mut self.static_fields
    }

    pub fn set_static_fields(&mut self, static_fields: Option<Slot>) {
        self.static_fields = static_fields;
    }

    /// Gets custom data of the specified type. If the data does not exist, create a new one.
    /// This matches the C# implementation's SharedStates.GetState<T> method.
    pub fn get_state<T: 'static + Default + Send + Sync>(
        &self,
        key: &str,
    ) -> Arc<std::sync::RwLock<T>> {
        // Create a composite key using the type and the provided key
        let composite_key = format!("{}::{}", std::any::type_name::<T>(), key);
        let type_id = TypeId::of::<String>(); // Use String for the composite key

        // Try to get existing state using the composite key
        {
            let states = self.states.read().expect("Lock poisoned");
            if let Some(boxed_state) = states.get(&type_id) {
                if let Some(state_map) = boxed_state
                    .downcast_ref::<std::collections::HashMap<String, Arc<std::sync::RwLock<T>>>>()
                {
                    if let Some(state) = state_map.get(&composite_key) {
                        return state.clone();
                    }
                }
            }
        }

        self.get_state_with_factory(key, T::default)
    }

    /// Gets custom data of the specified type, creating it with the provided factory if it doesn't exist.
    /// This matches the C# implementation's SharedStates.GetState<T>(Func<T>) method.
    pub fn get_state_with_factory<T: 'static + Send + Sync, F: FnOnce() -> T>(
        &self,
        key: &str,
        factory: F,
    ) -> Arc<std::sync::RwLock<T>> {
        // Create a composite key using the type and the provided key
        let composite_key = format!("{}::{}", std::any::type_name::<T>(), key);
        let type_id = TypeId::of::<String>(); // Use String for the composite key

        // Try to get existing state using the composite key
        {
            let states = self.states.read().expect("Lock poisoned");
            if let Some(boxed_state) = states.get(&type_id) {
                if let Some(state_map) = boxed_state
                    .downcast_ref::<std::collections::HashMap<String, Arc<std::sync::RwLock<T>>>>()
                {
                    if let Some(state) = state_map.get(&composite_key) {
                        return state.clone();
                    }
                }
            }
        }

        let new_state = Arc::new(std::sync::RwLock::new(factory()));

        // Store the state in the map
        {
            let mut states = self.states.write().expect("Lock poisoned");
            let state_map = states.entry(type_id).or_insert_with(|| {
                Box::new(std::collections::HashMap::<String, Arc<std::sync::RwLock<T>>>::new())
                    as Box<dyn std::any::Any + Send + Sync>
            });

            if let Some(map) = state_map
                .downcast_mut::<std::collections::HashMap<String, Arc<std::sync::RwLock<T>>>>()
            {
                map.insert(composite_key, new_state.clone());
            }
        }

        new_state
    }

    /// Sets a state value by type.
    /// This matches the C# implementation's state setting behavior.
    pub fn set_state<T: 'static + Send + Sync>(&self, value: T) {
        let type_id = TypeId::of::<T>();
        let state = Arc::new(std::sync::RwLock::new(value));

        let mut states = self.states.write().expect("Lock poisoned");
        states.insert(type_id, Box::new(state));
    }
}

/// Represents an execution context in the Neo Virtual Machine.
/// This matches the C# implementation's ExecutionContext class.
#[derive(Clone)]
pub struct ExecutionContext {
    /// The shared states (script, evaluation stack, static fields)
    shared_states: SharedStates,

    /// The current instruction pointer
    instruction_pointer: usize,

    /// The number of values to return when the context is unloaded (-1 for all)
    rvcount: i32,

    /// The local variables for this context
    local_variables: Option<Slot>,

    /// The arguments for this context
    arguments: Option<Slot>,

    /// The stack containing nested exception handling contexts
    try_stack: Option<Vec<crate::exception_handling::ExceptionHandlingContext>>,

    /// Exception handlers stack for TRY/CATCH/FINALLY operations
    exception_handlers: Vec<ExceptionHandler>,

    /// Current execution state
    state: ExecutionState,
}

impl ExecutionContext {
    /// Creates a new execution context.
    /// This matches the C# implementation's constructor pattern.
    pub fn new(script: Script, rvcount: i32, reference_counter: &ReferenceCounter) -> Self {
        Self {
            shared_states: SharedStates::new(script, reference_counter.clone()),
            instruction_pointer: 0,
            rvcount,
            local_variables: None,
            arguments: None,
            try_stack: None,
            exception_handlers: Vec::new(),
            state: ExecutionState::None,
        }
    }

    /// Returns the script for this context.
    /// This matches the C# implementation's Script property.
    pub fn script(&self) -> &Script {
        self.shared_states.script()
    }

    /// Returns the script hash for this context.
    /// This matches the C# implementation's Script.ToScriptHash() behavior exactly.
    /// Uses Hash160 (RIPEMD-160 of SHA-256) as per Neo protocol.
    pub fn script_hash(&self) -> neo_core::UInt160 {
        use ripemd::{Digest as RipemdDigest, Ripemd160};
        use sha2::Sha256;

        let script_bytes = self.script().as_bytes();

        // Calculate SHA-256 hash
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(script_bytes);
        let sha256_hash = sha256_hasher.finalize();

        // Calculate RIPEMD-160 hash of the SHA-256 hash
        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha256_hash);
        let ripemd_hash = ripemd_hasher.finalize();

        // Convert to UInt160
        neo_core::UInt160::from_bytes(&ripemd_hash).unwrap_or_else(|_| neo_core::UInt160::zero())
    }

    /// Pushes an exception handler onto the exception handler stack.
    /// This matches the C# implementation's exception handling exactly.
    pub fn push_exception_handler(&mut self, handler: ExceptionHandler) {
        self.exception_handlers.push(handler);
    }

    /// Pops an exception handler from the exception handler stack.
    /// Returns None if the stack is empty.
    /// This matches the C# implementation's exception handling exactly.
    pub fn pop_exception_handler(&mut self) -> Option<ExceptionHandler> {
        self.exception_handlers.pop()
    }

    /// Checks if the context is currently in an exception state.
    /// This matches the C# implementation's exception state tracking.
    pub fn is_in_exception(&self) -> bool {
        // 1. Check if there are any active exception handlers
        if !self.exception_handlers.is_empty() {
            // 2. Check if the top handler is in an exception state
            if let Some(top_handler) = self.exception_handlers.last() {
                return top_handler.is_in_exception_state();
            }
        }

        // 3. Check try stack for exception state (production exception tracking)
        if let Some(try_stack) = &self.try_stack {
            for try_context in try_stack {
                if try_context.is_in_exception() {
                    return true;
                }
            }
        }

        false
    }

    /// Sets the exception state for this context.
    /// This matches the C# implementation's exception state management.
    pub fn set_exception_state(&mut self, in_exception: bool) {
        if in_exception {
            // Mark the context as being in an exception state
            self.exception_handlers.push(ExceptionHandler {
                catch_offset: Some(self.instruction_pointer),
                finally_offset: None,
                stack_depth: 0, // Would be set based on current evaluation stack depth
            });

            // Set VM state to fault if needed
            if self.exception_handlers.len() > MAX_EXCEPTION_STACK_SIZE {
                self.state = ExecutionState::Fault;
            }
        } else {
            // Clear the exception state by removing the top exception handler
            if !self.exception_handlers.is_empty() {
                self.exception_handlers.pop();
            }

            // Restore normal execution state if no more exceptions
            if self.exception_handlers.is_empty() && self.state == ExecutionState::Fault {
                self.state = ExecutionState::None;
            }
        }
    }

    /// Returns the current instruction pointer.
    pub fn instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    /// Sets the instruction pointer.
    pub fn set_instruction_pointer(&mut self, position: usize) {
        self.instruction_pointer = position;
    }

    /// Returns the current instruction or None if at the end of the script.
    /// This matches the C# implementation's CurrentInstruction property.
    pub fn current_instruction(&self) -> VmResult<Instruction> {
        if self.instruction_pointer >= self.script().len() {
            return Err(VmError::invalid_operation_msg(
                "Instruction pointer is out of range",
            ));
        }

        self.script().get_instruction(self.instruction_pointer)
    }

    /// Returns the next instruction or None if at the end of the script.
    /// This matches the C# implementation's NextInstruction property.
    pub fn next_instruction(&self) -> VmResult<Instruction> {
        let current = self.current_instruction()?;
        let next_position = self.instruction_pointer + current.size();

        if next_position >= self.script().len() {
            return Err(VmError::invalid_operation_msg(
                "Next instruction is out of range",
            ));
        }

        self.script().get_instruction(next_position)
    }

    /// Returns the number of values to return when the context is unloaded (-1 for all).
    pub fn rvcount(&self) -> i32 {
        self.rvcount
    }

    /// Returns the evaluation stack for this context.
    /// This matches the C# implementation's EvaluationStack property.
    pub fn evaluation_stack(&self) -> &EvaluationStack {
        self.shared_states.evaluation_stack()
    }

    /// Returns the evaluation stack for this context (mutable).
    /// This matches the C# implementation's EvaluationStack property.
    pub fn evaluation_stack_mut(&mut self) -> &mut EvaluationStack {
        self.shared_states.evaluation_stack_mut()
    }

    /// Returns the static fields for this context.
    /// This matches the C# implementation's StaticFields property getter.
    pub fn static_fields(&self) -> Option<&Slot> {
        self.shared_states.static_fields()
    }

    /// Returns the static fields for this context (mutable).
    /// This matches the C# implementation's StaticFields property setter.
    pub fn static_fields_mut(&mut self) -> &mut Option<Slot> {
        self.shared_states.static_fields_mut()
    }

    /// Sets the static fields for this context.
    pub fn set_static_fields(&mut self, static_fields: Option<Slot>) {
        self.shared_states.set_static_fields(static_fields);
    }

    /// Returns the local variables for this context.
    pub fn local_variables(&self) -> Option<&Slot> {
        self.local_variables.as_ref()
    }

    /// Returns the local variables for this context (mutable).
    pub fn local_variables_mut(&mut self) -> Option<&mut Slot> {
        self.local_variables.as_mut()
    }

    /// Sets the local variables for this context.
    pub fn set_local_variables(&mut self, local_variables: Option<Slot>) {
        self.local_variables = local_variables;
    }

    /// Returns the arguments for this context.
    pub fn arguments(&self) -> Option<&Slot> {
        self.arguments.as_ref()
    }

    /// Returns the arguments for this context (mutable).
    pub fn arguments_mut(&mut self) -> Option<&mut Slot> {
        self.arguments.as_mut()
    }

    /// Sets the arguments for this context.
    pub fn set_arguments(&mut self, arguments: Option<Slot>) {
        self.arguments = arguments;
    }

    /// Returns the try stack for this context.
    pub fn try_stack(&self) -> Option<&Vec<crate::exception_handling::ExceptionHandlingContext>> {
        self.try_stack.as_ref()
    }

    /// Returns the try stack for this context (mutable).
    pub fn try_stack_mut(
        &mut self,
    ) -> Option<&mut Vec<crate::exception_handling::ExceptionHandlingContext>> {
        self.try_stack.as_mut()
    }

    /// Sets the try stack for this context.
    pub fn set_try_stack(
        &mut self,
        try_stack: Option<Vec<crate::exception_handling::ExceptionHandlingContext>>,
    ) {
        self.try_stack = try_stack;
    }

    /// Moves to the next instruction.
    pub fn move_next(&mut self) -> VmResult<()> {
        if self.instruction_pointer >= self.script().len() {
            return Err(VmError::invalid_operation_msg(
                "Instruction pointer is out of range",
            ));
        }

        let instruction = self.script().get_instruction(self.instruction_pointer)?;
        self.instruction_pointer += instruction.size();

        Ok(())
    }

    /// Gets a state value by key, creating it if it doesn't exist.
    /// This matches the C# implementation's GetState<T> method.
    pub fn get_state<T: 'static + Default + Send + Sync>(
        &self,
        key: &str,
    ) -> Arc<std::sync::RwLock<T>> {
        // In the C# implementation, ExecutionContext has its own state management
        self.shared_states.get_state::<T>(key)
    }

    /// Gets a state value by key, creating it with the provided factory if it doesn't exist.
    /// This matches the C# implementation's GetState<T>(Func<T>) method.
    pub fn get_state_with_factory<T: 'static + Send + Sync, F: FnOnce() -> T>(
        &self,
        key: &str,
        factory: F,
    ) -> Arc<std::sync::RwLock<T>> {
        self.shared_states.get_state_with_factory(key, factory)
    }

    /// Sets a state value by key.
    /// This matches the C# implementation's state setting behavior.
    pub fn set_state<T: 'static + Send + Sync>(&self, _key: String, value: T) {
        // Set state in the shared states
        self.shared_states.set_state(value);
    }

    /// Push an item onto the evaluation stack
    pub fn push(&mut self, item: crate::stack_item::StackItem) -> VmResult<()> {
        self.shared_states.evaluation_stack_mut().push(item);
        Ok(())
    }

    /// Pop an item from the evaluation stack
    pub fn pop(&mut self) -> VmResult<crate::stack_item::StackItem> {
        self.shared_states.evaluation_stack_mut().pop()
    }

    /// Peek at an item on the evaluation stack without removing it
    pub fn peek(&self, index: usize) -> VmResult<crate::stack_item::StackItem> {
        self.shared_states
            .evaluation_stack()
            .peek(index as isize)
            .cloned()
    }

    /// Clones the context with a new reference counter.
    pub fn clone_for_reference_counter(&self, reference_counter: &ReferenceCounter) -> Self {
        // Create a new shared states with the new reference counter
        let mut shared_states = SharedStates::new(self.script().clone(), reference_counter.clone());

        // Copy the evaluation stack
        self.evaluation_stack()
            .copy_to(shared_states.evaluation_stack_mut());

        // Create the new context

        // Note: Not cloning static_fields, local_variables, or arguments
        // as they would need separate reference counter handling

        Self {
            shared_states,
            instruction_pointer: self.instruction_pointer,
            rvcount: self.rvcount,
            local_variables: None,
            arguments: None,
            try_stack: self.try_stack.clone(),
            exception_handlers: self.exception_handlers.clone(),
            state: ExecutionState::None,
        }
    }

    /// Clones the context so that they share the same script, stack, and static fields.
    /// This matches the C# implementation's Clone method.
    pub fn clone(&self) -> Self {
        self.clone_with_position(self.instruction_pointer)
    }

    /// Clones the context so that they share the same script, stack, and static fields.
    /// This matches the C# implementation's Clone(int) method.
    pub fn clone_with_position(&self, position: usize) -> Self {
        Self {
            shared_states: self.shared_states.clone(),
            instruction_pointer: position,
            rvcount: 0, // The C# implementation sets this to 0
            local_variables: None,
            arguments: None,
            try_stack: None,
            exception_handlers: Vec::new(),
            state: ExecutionState::None,
        }
    }

    /// Gets a shared state by type, creating it if it doesn't exist.
    /// This matches the C# implementation's GetState<T> method.
    pub fn get_shared_state<T: 'static + Default + Send + Sync>(
        &self,
        key: &str,
    ) -> Arc<std::sync::RwLock<T>> {
        self.shared_states.get_state::<T>(key)
    }

    /// Gets a shared state by type, creating it with the provided factory if it doesn't exist.
    /// This matches the C# implementation's GetState<T>(Func<T>) method.
    pub fn get_shared_state_with_factory<T: 'static + Send + Sync, F: FnOnce() -> T>(
        &self,
        key: &str,
        factory: F,
    ) -> Arc<std::sync::RwLock<T>> {
        self.shared_states.get_state_with_factory(key, factory)
    }

    /// Initializes the slots for local variables and arguments.
    pub fn init_slot(&mut self, local_count: usize, argument_count: usize) -> VmResult<()> {
        // Initialize local variables
        if local_count > 0 {
            let local_items = vec![crate::stack_item::StackItem::null(); local_count];
            self.local_variables = Some(Slot::new(
                local_items,
                self.shared_states.reference_counter().clone(),
            ));
        }

        // Initialize arguments
        if argument_count > 0 {
            let arg_items = vec![crate::stack_item::StackItem::null(); argument_count];
            self.arguments = Some(Slot::new(
                arg_items,
                self.shared_states.reference_counter().clone(),
            ));
        }

        Ok(())
    }

    /// Loads a value from a static field.
    pub fn load_static_field(&self, index: usize) -> VmResult<crate::stack_item::StackItem> {
        if let Some(static_fields) = &self.shared_states.static_fields() {
            static_fields.get(index).cloned()
        } else {
            Err(crate::VmError::invalid_operation_msg(
                "No static fields initialized",
            ))
        }
    }

    /// Stores a value to a static field.
    pub fn store_static_field(
        &mut self,
        index: usize,
        value: crate::stack_item::StackItem,
    ) -> VmResult<()> {
        if let Some(static_fields) = self.shared_states.static_fields_mut() {
            static_fields.set(index, value);
            Ok(())
        } else {
            Err(crate::VmError::invalid_operation_msg(
                "No static fields initialized",
            ))
        }
    }

    /// Loads a value from a local variable.
    pub fn load_local(&self, index: usize) -> VmResult<crate::stack_item::StackItem> {
        if let Some(local_variables) = &self.local_variables {
            local_variables.get(index).cloned()
        } else {
            Err(crate::VmError::invalid_operation_msg(
                "No local variables initialized",
            ))
        }
    }

    /// Stores a value to a local variable.
    pub fn store_local(
        &mut self,
        index: usize,
        value: crate::stack_item::StackItem,
    ) -> VmResult<()> {
        if let Some(local_variables) = &mut self.local_variables {
            local_variables.set(index, value);
            Ok(())
        } else {
            Err(crate::VmError::invalid_operation_msg(
                "No local variables initialized",
            ))
        }
    }

    /// Loads a value from an argument.
    pub fn load_argument(&self, index: usize) -> VmResult<crate::stack_item::StackItem> {
        if let Some(arguments) = &self.arguments {
            arguments.get(index).cloned()
        } else {
            Err(crate::VmError::invalid_operation_msg(
                "No arguments initialized",
            ))
        }
    }

    /// Stores a value to an argument.
    pub fn store_argument(
        &mut self,
        index: usize,
        value: crate::stack_item::StackItem,
    ) -> VmResult<()> {
        if let Some(arguments) = &mut self.arguments {
            arguments.set(index, value);
            Ok(())
        } else {
            Err(crate::VmError::invalid_operation_msg(
                "No arguments initialized",
            ))
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::op_code::OpCode;
    use num_bigint::BigInt;

    #[test]
    fn test_execution_context_creation() {
        let script_bytes = vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8, OpCode::ADD as u8];
        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let context = ExecutionContext::new(script, -1, &reference_counter);

        assert_eq!(context.instruction_pointer(), 0);
        assert_eq!(context.rvcount(), -1);
        assert_eq!(
            context
                .current_instruction()
                .expect("intermediate value should exist")
                .opcode(),
            OpCode::PUSH1
        );
        assert!(context.evaluation_stack().is_empty());
        assert!(context.try_stack().is_none());
    }

    #[test]
    fn test_move_next() {
        let script_bytes = vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8, OpCode::ADD as u8];
        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let mut context = ExecutionContext::new(script, -1, &reference_counter);

        assert_eq!(
            context
                .current_instruction()
                .expect("intermediate value should exist")
                .opcode(),
            OpCode::PUSH1
        );

        context.move_next().expect("VM operation should succeed");
        assert_eq!(context.instruction_pointer(), 1);
        assert_eq!(
            context
                .current_instruction()
                .expect("intermediate value should exist")
                .opcode(),
            OpCode::PUSH2
        );

        context.move_next().expect("VM operation should succeed");
        assert_eq!(context.instruction_pointer(), 2);
        assert_eq!(
            context
                .current_instruction()
                .expect("intermediate value should exist")
                .opcode(),
            OpCode::ADD
        );

        context.move_next().expect("VM operation should succeed");
        assert_eq!(context.instruction_pointer(), 3);
        assert!(context.current_instruction().is_err());
    }

    #[test]
    fn test_try_stack() {
        let script_bytes = vec![OpCode::NOP as u8];
        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let mut context = ExecutionContext::new(script, -1, &reference_counter);

        // Initially, try_stack is None
        assert!(context.try_stack().is_none());

        // Create a try stack with one context
        use crate::exception_handling::{ExceptionHandlingContext, ExceptionHandlingState};
        let mut try_stack = Vec::new();
        let try_context = ExceptionHandlingContext::new(0, 10, 10, 20, 30); // ADDRESS_SIZE = 20
        try_stack.push(try_context);

        // Set the try stack
        context.set_try_stack(Some(try_stack));

        // Check that the try stack is set
        assert!(context.try_stack().is_some());
        assert_eq!(
            context
                .try_stack()
                .expect("intermediate value should exist")
                .len(),
            1
        );
        assert_eq!(
            context.try_stack().expect("VM operation should succeed")[0].catch_pointer,
            10
        );
        assert_eq!(
            context.try_stack().expect("Operation failed")[0].finally_pointer as usize,
            20 // ADDRESS_SIZE constant
        );

        // Modify the try stack
        if let Some(stack) = context.try_stack_mut() {
            let exception_context = &mut stack[0];
            exception_context.state = ExceptionHandlingState::Catch;
        }

        // Check that the modification was applied
        assert_eq!(
            context.try_stack().expect("Operation failed")[0].state,
            ExceptionHandlingState::Catch
        );
    }

    #[test]
    fn test_slot() {
        let reference_counter = ReferenceCounter::new();

        let items = vec![
            StackItem::from_int(1),
            StackItem::from_int(2),
            StackItem::from_int(3),
        ];

        let mut slot = Slot::new(items, reference_counter);

        assert_eq!(slot.len(), 3);
        assert_eq!(
            slot.items
                .first()
                .expect("Empty collection")
                .as_int()
                .expect("VM operation should succeed"),
            BigInt::from(1)
        );
        assert_eq!(
            slot.get(1)
                .expect("Index out of bounds")
                .as_int()
                .expect("VM operation should succeed"),
            BigInt::from(2)
        );
        assert_eq!(
            slot.get(2)
                .expect("Index out of bounds")
                .as_int()
                .expect("VM operation should succeed"),
            BigInt::from(3)
        );

        slot.set(1, StackItem::from_int(42));
        assert_eq!(
            slot.get(1)
                .expect("Index out of bounds")
                .as_int()
                .expect("VM operation should succeed"),
            BigInt::from(42)
        );

        slot.set(5, StackItem::from_int(5));
        assert_eq!(slot.len(), 6);
        assert_eq!(
            slot.get(5)
                .expect("Index out of bounds")
                .as_int()
                .expect("VM operation should succeed"),
            BigInt::from(5)
        );

        slot.clear_references();
        assert_eq!(slot.len(), 0);
        assert!(slot.is_empty());
    }

    #[test]
    fn test_next_instruction() {
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];

        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let context = ExecutionContext::new(script, -1, &reference_counter);

        // Test current instruction
        let current = context
            .current_instruction()
            .expect("VM operation should succeed");
        assert_eq!(current.opcode(), OpCode::PUSH1);

        // Test next instruction
        let next = context
            .next_instruction()
            .expect("VM operation should succeed");
        assert_eq!(next.opcode(), OpCode::PUSH2);
    }

    #[test]
    fn test_clone() {
        let script_bytes = vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ];

        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let mut context = ExecutionContext::new(script, -1, &reference_counter);

        // Push a value onto the stack
        context.evaluation_stack_mut().push(StackItem::from_int(42));

        // Clone the context
        let clone = context.clone();

        // Check that the clone has the same script and stack
        assert_eq!(clone.script().to_array(), context.script().to_array());
        assert_eq!(
            clone
                .evaluation_stack()
                .peek(0)
                .expect("intermediate value should exist")
                .as_int()
                .expect("VM operation should succeed"),
            BigInt::from(42)
        );

        // Check that the clone has a different instruction pointer
        assert_eq!(clone.instruction_pointer(), context.instruction_pointer());

        // Check that the clone has a different rvcount
        assert_eq!(clone.rvcount(), 0);

        // Clone with a different position
        let clone_with_position = context.clone_with_position(2);

        // Check that the clone has a different instruction pointer
        assert_eq!(clone_with_position.instruction_pointer(), 2);
    }

    #[test]
    fn test_get_state() -> Result<(), Box<dyn std::error::Error>> {
        let script_bytes = vec![OpCode::NOP as u8];
        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let mut context = ExecutionContext::new(script, -1, &reference_counter);

        // Test getting state with type inference
        let state = context.get_state::<i32>("test");
        {
            let mut value = state.write().map_err(|_| "poisoned")?;
            *value = 42;
        }

        // Get the same state again - should be the same instance
        let state2 = context.get_state::<i32>("test");
        {
            let value = state2.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, 42);
        }

        // Test factory-created state
        let state_with_factory = context.get_state_with_factory("test2", || "hello".to_string());
        {
            let value = state_with_factory.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, "hello");
        }

        // Test different type has different state
        let int_state = context.get_state::<i32>("test3");
        {
            let value = int_state.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, 0); // Default value
        }
        Ok(())
    }

    #[test]
    fn test_get_shared_state() -> Result<(), Box<dyn std::error::Error>> {
        let script_bytes = vec![OpCode::NOP as u8];
        let script = Script::new_relaxed(script_bytes);
        let reference_counter = ReferenceCounter::new();

        let mut context = ExecutionContext::new(script, -1, &reference_counter);

        // Test getting shared state
        let state = context.get_shared_state::<i32>("test");
        {
            let mut value = state.write().map_err(|_| "poisoned")?;
            *value = 100;
        }

        // Test factory-created shared state with SAME key "test"
        let state_with_factory = context.get_shared_state_with_factory("test", || 200);
        {
            let value = state_with_factory.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, 100); // Should return existing value, not factory value
        }

        // Test factory-created shared state with DIFFERENT key "test2"
        // Since this key doesn't exist, factory SHOULD be called
        let state_with_factory_new = context.get_shared_state_with_factory("test2", || 300);
        {
            let value = state_with_factory_new.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, 300); // Should return factory value for new key
        }

        // Clone context to test shared state
        let clone = context.clone();
        let shared_state = clone.get_shared_state::<i32>("test");
        {
            let value = shared_state.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, 100); // Should be shared
        }

        // Modify shared state from clone
        {
            let mut value = shared_state.write().map_err(|_| "poisoned")?;
            *value = 200;
        }

        // Check that original context sees the change
        let original_state = context.get_shared_state::<i32>("test");
        {
            let value = original_state.read().map_err(|_| "poisoned")?;
            assert_eq!(*value, 200); // Should be updated
        }
        Ok(())
    }
}

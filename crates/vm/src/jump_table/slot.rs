//! Slot operations for the Neo Virtual Machine.
//!
//! This module provides the slot operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;

/// Registers the slot operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::INITSSLOT, init_static_slot);
    jump_table.register(OpCode::INITSLOT, init_slot);

    // Static field operations
    jump_table.register(OpCode::LDSFLD0, load_static_field_0);
    jump_table.register(OpCode::LDSFLD1, load_static_field_1);
    jump_table.register(OpCode::LDSFLD2, load_static_field_2);
    jump_table.register(OpCode::LDSFLD3, load_static_field_3);
    jump_table.register(OpCode::LDSFLD4, load_static_field_4);
    jump_table.register(OpCode::LDSFLD5, load_static_field_5);
    jump_table.register(OpCode::LDSFLD6, load_static_field_6);
    jump_table.register(OpCode::LDSFLD, load_static_field);
    jump_table.register(OpCode::STSFLD0, store_static_field_0);
    jump_table.register(OpCode::STSFLD1, store_static_field_1);
    jump_table.register(OpCode::STSFLD2, store_static_field_2);
    jump_table.register(OpCode::STSFLD3, store_static_field_3);
    jump_table.register(OpCode::STSFLD4, store_static_field_4);
    jump_table.register(OpCode::STSFLD5, store_static_field_5);
    jump_table.register(OpCode::STSFLD6, store_static_field_6);
    jump_table.register(OpCode::STSFLD, store_static_field);

    // Local variable operations
    jump_table.register(OpCode::LDLOC0, load_local_0);
    jump_table.register(OpCode::LDLOC1, load_local_1);
    jump_table.register(OpCode::LDLOC2, load_local_2);
    jump_table.register(OpCode::LDLOC3, load_local_3);
    jump_table.register(OpCode::LDLOC4, load_local_4);
    jump_table.register(OpCode::LDLOC5, load_local_5);
    jump_table.register(OpCode::LDLOC6, load_local_6);
    jump_table.register(OpCode::LDLOC, load_local);
    jump_table.register(OpCode::STLOC0, store_local_0);
    jump_table.register(OpCode::STLOC1, store_local_1);
    jump_table.register(OpCode::STLOC2, store_local_2);
    jump_table.register(OpCode::STLOC3, store_local_3);
    jump_table.register(OpCode::STLOC4, store_local_4);
    jump_table.register(OpCode::STLOC5, store_local_5);
    jump_table.register(OpCode::STLOC6, store_local_6);
    jump_table.register(OpCode::STLOC, store_local);

    // Argument operations
    jump_table.register(OpCode::LDARG0, load_argument_0);
    jump_table.register(OpCode::LDARG1, load_argument_1);
    jump_table.register(OpCode::LDARG2, load_argument_2);
    jump_table.register(OpCode::LDARG3, load_argument_3);
    jump_table.register(OpCode::LDARG4, load_argument_4);
    jump_table.register(OpCode::LDARG5, load_argument_5);
    jump_table.register(OpCode::LDARG6, load_argument_6);
    jump_table.register(OpCode::LDARG, load_argument);
    jump_table.register(OpCode::STARG0, store_argument_0);
    jump_table.register(OpCode::STARG1, store_argument_1);
    jump_table.register(OpCode::STARG2, store_argument_2);
    jump_table.register(OpCode::STARG3, store_argument_3);
    jump_table.register(OpCode::STARG4, store_argument_4);
    jump_table.register(OpCode::STARG5, store_argument_5);
    jump_table.register(OpCode::STARG6, store_argument_6);
    jump_table.register(OpCode::STARG, store_argument);
}

/// Implements the INITSLOT operation.
fn init_slot(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    if context.local_variables().is_some() || context.arguments().is_some() {
        return Err(VmError::invalid_operation_msg(
            "INITSLOT cannot be executed twice",
        ));
    }

    // Get the local and argument counts from the instruction
    let local_count = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing local count"))?
        as usize;
    let argument_count = instruction
        .operand()
        .get(1)
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing argument count"))?
        as usize;

    // Check that at least one count is greater than 0
    if local_count == 0 && argument_count == 0 {
        return Err(VmError::invalid_operation_msg(
            "The operand is invalid for OpCode.INITSLOT",
        ));
    }

    if local_count > 0 {
        let local_items = vec![crate::stack_item::StackItem::null(); local_count];
        let reference_counter = context.evaluation_stack().reference_counter().clone();
        let local_slot = crate::execution_context::Slot::new(local_items, reference_counter);
        context.set_local_variables(Some(local_slot));
    }

    if argument_count > 0 {
        let mut arg_items = Vec::with_capacity(argument_count);
        for _ in 0..argument_count {
            let value = context.pop()?;
            arg_items.push(value);
        }

        let reference_counter = context.evaluation_stack().reference_counter().clone();
        let arg_slot = crate::execution_context::Slot::new(arg_items, reference_counter);
        context.set_arguments(Some(arg_slot));
    }

    Ok(())
}

/// Implements the LDSFLD operation.
fn load_static_field(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the index from the instruction
    let index = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing index"))? as usize;

    // Load the static field
    let value = context.load_static_field(index)?;

    // Push the value onto the stack
    context.push(value)?;

    Ok(())
}

/// Implements the STSFLD operation.
fn store_static_field(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the index from the instruction
    let index = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing index"))? as usize;

    // Pop the value from the stack
    let value = context.pop()?;

    // Store the static field
    context.store_static_field(index, value)?;

    Ok(())
}

/// Implements the LDLOC operation.
fn load_local(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the index from the instruction
    let index = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing index"))? as usize;

    // Load the local variable
    let value = context.load_local(index)?;

    // Push the value onto the stack
    context.push(value)?;

    Ok(())
}

/// Implements the STLOC operation.
fn store_local(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the index from the instruction
    let index = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing index"))? as usize;

    // Pop the value from the stack
    let value = context.pop()?;

    // Store the local variable
    context.store_local(index, value)?;

    Ok(())
}

/// Implements the LDARG operation.
fn load_argument(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the index from the instruction
    let index = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing index"))? as usize;

    // Load the argument
    let value = context.load_argument(index)?;

    // Push the value onto the stack
    context.push(value)?;

    Ok(())
}

/// Implements the STARG operation.
fn store_argument(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the index from the instruction
    let index = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing index"))? as usize;

    // Pop the value from the stack
    let value = context.pop()?;

    // Store the argument
    context.store_argument(index, value)?;

    Ok(())
}

/// Implements the INITSSLOT operation.
fn init_static_slot(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the static field count from the instruction
    let static_count = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing static count"))?
        as usize;

    if context.static_fields().is_some() {
        return Err(VmError::invalid_operation_msg(
            "INITSSLOT cannot be executed twice",
        ));
    }

    // Create a new slot with the specified count, filled with null values
    if static_count > 0 {
        let static_items = vec![crate::stack_item::StackItem::null(); static_count];
        let reference_counter = context.evaluation_stack().reference_counter().clone();
        let static_slot = crate::execution_context::Slot::new(static_items, reference_counter);
        context.set_static_fields(Some(static_slot));
    }

    Ok(())
}

/// Implements the LDSFLD0 operation.
fn load_static_field_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(0)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDSFLD1 operation.
fn load_static_field_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(1)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDSFLD2 operation.
fn load_static_field_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(2)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDSFLD3 operation.
fn load_static_field_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(3)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDSFLD4 operation.
fn load_static_field_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(4)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDSFLD5 operation.
fn load_static_field_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(5)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDSFLD6 operation.
fn load_static_field_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_static_field(6)?;
    context.push(value)?;
    Ok(())
}

/// Implements the STSFLD0 operation.
fn store_static_field_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(0, value)?;
    Ok(())
}

/// Implements the STSFLD1 operation.
fn store_static_field_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(1, value)?;
    Ok(())
}

/// Implements the STSFLD2 operation.
fn store_static_field_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(2, value)?;
    Ok(())
}

/// Implements the STSFLD3 operation.
fn store_static_field_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(3, value)?;
    Ok(())
}

/// Implements the STSFLD4 operation.
fn store_static_field_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(4, value)?;
    Ok(())
}

/// Implements the STSFLD5 operation.
fn store_static_field_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(5, value)?;
    Ok(())
}

/// Implements the STSFLD6 operation.
fn store_static_field_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_static_field(6, value)?;
    Ok(())
}

/// Implements the LDLOC0 operation.
fn load_local_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(0)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDLOC1 operation.
fn load_local_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(1)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDLOC2 operation.
fn load_local_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(2)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDLOC3 operation.
fn load_local_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(3)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDLOC4 operation.
fn load_local_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(4)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDLOC5 operation.
fn load_local_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(5)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDLOC6 operation.
fn load_local_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_local(6)?;
    context.push(value)?;
    Ok(())
}

/// Implements the STLOC0 operation.
fn store_local_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(0, value)?;
    Ok(())
}

/// Implements the STLOC1 operation.
fn store_local_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(1, value)?;
    Ok(())
}

/// Implements the STLOC2 operation.
fn store_local_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(2, value)?;
    Ok(())
}

/// Implements the STLOC3 operation.
fn store_local_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(3, value)?;
    Ok(())
}

/// Implements the STLOC4 operation.
fn store_local_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(4, value)?;
    Ok(())
}

/// Implements the STLOC5 operation.
fn store_local_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(5, value)?;
    Ok(())
}

/// Implements the STLOC6 operation.
fn store_local_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_local(6, value)?;
    Ok(())
}

/// Implements the LDARG0 operation.
fn load_argument_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(0)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDARG1 operation.
fn load_argument_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(1)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDARG2 operation.
fn load_argument_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(2)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDARG3 operation.
fn load_argument_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(3)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDARG4 operation.
fn load_argument_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(4)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDARG5 operation.
fn load_argument_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(5)?;
    context.push(value)?;
    Ok(())
}

/// Implements the LDARG6 operation.
fn load_argument_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.load_argument(6)?;
    context.push(value)?;
    Ok(())
}

/// Implements the STARG0 operation.
fn store_argument_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(0, value)?;
    Ok(())
}

/// Implements the STARG1 operation.
fn store_argument_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(1, value)?;
    Ok(())
}

/// Implements the STARG2 operation.
fn store_argument_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(2, value)?;
    Ok(())
}

/// Implements the STARG3 operation.
fn store_argument_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(3, value)?;
    Ok(())
}

/// Implements the STARG4 operation.
fn store_argument_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(4, value)?;
    Ok(())
}

/// Implements the STARG5 operation.
fn store_argument_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(5, value)?;
    Ok(())
}

/// Implements the STARG6 operation.
fn store_argument_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
    let value = context.pop()?;
    context.store_argument(6, value)?;
    Ok(())
}

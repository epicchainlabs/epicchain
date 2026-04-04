//! Stack operations for the Neo Virtual Machine.
//!
//! This module provides the stack operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;
use crate::stack_item::StackItem;
use num_bigint::Sign;
use num_traits::ToPrimitive;

/// Registers the stack operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::DUP, dup);
    jump_table.register(OpCode::SWAP, swap);
    jump_table.register(OpCode::TUCK, tuck);
    jump_table.register(OpCode::OVER, over);
    // TOALTSTACK and FROMALTSTACK removed - not in C# Neo
    jump_table.register(OpCode::PICK, pick);
    jump_table.register(OpCode::ROT, rot);
    jump_table.register(OpCode::DEPTH, depth);
    jump_table.register(OpCode::DROP, drop);
    jump_table.register(OpCode::NIP, nip);
    jump_table.register(OpCode::XDROP, xdrop);
    jump_table.register(OpCode::CLEAR, clear);
    jump_table.register(OpCode::ROLL, roll);
    jump_table.register(OpCode::REVERSE3, reverse3);
    jump_table.register(OpCode::REVERSE4, reverse4);
    jump_table.register(OpCode::REVERSEN, reversen);
}

/// Implements the DUP operation.
fn dup(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Peek the top item on the stack and push a copy
    let item = engine.peek(0)?.clone();
    engine.push(item)?;
    Ok(())
}

/// Implements the SWAP operation.
fn swap(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Pop the top two items from the stack
    let b = engine.pop()?;
    let a = engine.pop()?;

    // Push them back in reverse order
    engine.push(b)?;
    engine.push(a)?;

    Ok(())
}

/// Implements the TUCK operation.
fn tuck(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    if context.evaluation_stack().len() < 2 {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Pop the top two items from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Push them back in the order: b, a, b
    context.push(b.clone())?;
    context.push(a)?;
    context.push(b)?;

    Ok(())
}

/// Implements the OVER operation.
fn over(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    if context.evaluation_stack().len() < 2 {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Peek the second item on the stack
    let item = context.peek(1)?;

    // Push a copy of the item onto the stack
    context.push(item)?;

    Ok(())
}

/// Implements the ROT operation.
fn rot(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    if context.evaluation_stack().len() < 3 {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Pop the top three items from the stack
    let c = context.pop()?;
    let b = context.pop()?;
    let a = context.pop()?;

    // Push them back in the order: b, c, a
    context.push(b)?;
    context.push(c)?;
    context.push(a)?;

    Ok(())
}

/// Implements the DEPTH operation.
fn depth(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the stack depth
    let depth = context.evaluation_stack().len();

    // Push the depth onto the stack
    context.push(StackItem::from_int(depth))?;

    Ok(())
}

/// Implements the DROP operation.
fn drop(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the top item from the stack
    context.pop()?;

    Ok(())
}

/// Implements the NIP operation.
fn nip(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    if context.evaluation_stack().len() < 2 {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Pop the top two items from the stack
    let b = context.pop()?;
    let _a = context.pop()?;

    // Push the top item back onto the stack
    context.push(b)?;

    Ok(())
}

/// Implements the XDROP operation.
fn xdrop(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the index from the stack
    let n = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid index"))?;

    if context.evaluation_stack().len() <= n {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Remove the item at the specified index
    let mut items = Vec::new();
    for _i in 0..n {
        items.push(context.pop()?);
    }

    // Pop the item to be removed
    context.pop()?;

    // Push the items back onto the stack in reverse order
    for item in items.into_iter().rev() {
        context.push(item)?;
    }

    Ok(())
}

/// Implements the CLEAR operation.
fn clear(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Clear the stack
    context.evaluation_stack_mut().clear();

    Ok(())
}

/// Implements the PICK operation.
fn pick(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the index from the stack
    let n = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid index"))?;

    if context.evaluation_stack().len() <= n {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Peek the item at the specified index
    let item = context.peek(n)?;

    // Push a copy of the item onto the stack
    context.push(item)?;

    Ok(())
}

/// Implements the ROLL operation.
fn roll(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the index from the stack
    let n = context.pop()?.as_int()?;

    if n.sign() == Sign::Minus {
        return Err(VmError::invalid_operation_msg(format!(
            "The negative value {n} is invalid for OpCode.ROLL"
        )));
    }

    let n = n
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid index"))?;

    if n == 0 {
        return Ok(());
    }

    if context.evaluation_stack().len() <= n {
        return Err(VmError::stack_underflow_msg(0, 0));
    }

    // Remove the item at the specified index and push it to the top
    let mut items = Vec::new();
    for _ in 0..n {
        items.push(context.pop()?);
    }

    // Pop the item to be moved to the top
    let item_to_roll = context.pop()?;

    // Push the items back onto the stack in reverse order
    for item in items.into_iter().rev() {
        context.push(item)?;
    }

    // Push the rolled item to the top
    context.push(item_to_roll)?;

    Ok(())
}

/// Implements the REVERSE3 operation.
fn reverse3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    context.evaluation_stack_mut().reverse(3)?;

    Ok(())
}

/// Implements the REVERSE4 operation.
fn reverse4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    context.evaluation_stack_mut().reverse(4)?;

    Ok(())
}

/// Implements the REVERSEN operation.
fn reversen(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the count from the stack
    let n = context.pop()?.as_int()?;

    if n.sign() == Sign::Minus {
        return Err(VmError::invalid_operation_msg(format!(
            "Reverse count out of range: {n}"
        )));
    }

    let n = n
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid count"))?;

    context.evaluation_stack_mut().reverse(n)?;

    Ok(())
}

// TOALTSTACK and FROMALTSTACK removed - not in C# Neo
// These opcodes (0x4C and 0x4F) are not present in the C# implementation

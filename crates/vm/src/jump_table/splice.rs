//! Splice operations for the Neo Virtual Machine.
//!
//! This module provides the splice operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;
use crate::stack_item::StackItem;
use num_traits::ToPrimitive;

/// Registers the splice operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::NEWBUFFER, new_buffer);
    jump_table.register(OpCode::MEMCPY, memcpy);
    jump_table.register(OpCode::CAT, cat);
    jump_table.register(OpCode::SUBSTR, substr);
    jump_table.register(OpCode::LEFT, left);
    jump_table.register(OpCode::RIGHT, right);
}

/// Implements the NEWBUFFER operation.
fn new_buffer(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the size from the stack
    let size = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid buffer size"))?;

    // Create a new buffer
    let buffer = StackItem::from_buffer(vec![0; size]);

    // Push the buffer onto the stack
    context.push(buffer)?;

    Ok(())
}

/// Implements the MEMCPY operation.
fn memcpy(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid count"))?;
    let src_offset = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid source offset"))?;
    let dst_offset = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid destination offset"))?;
    let src = context.pop()?;
    let dst = context.pop()?;

    // Get the source and destination data
    let src_data = match src {
        StackItem::ByteString(data) | StackItem::Buffer(data) => data,
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected ByteString or Buffer for source",
            ));
        }
    };

    // Check bounds
    if src_offset + count > src_data.len() {
        return Err(VmError::invalid_operation_msg(format!(
            "Source out of bounds: {} + {} > {}",
            src_offset,
            count,
            src_data.len()
        )));
    }

    // Get the destination data
    match dst {
        StackItem::Buffer(mut data) => {
            // Check bounds
            if dst_offset + count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Destination out of bounds: {} + {} > {}",
                    dst_offset,
                    count,
                    data.len()
                )));
            }

            // Copy the data
            data[dst_offset..(dst_offset + count)]
                .copy_from_slice(&src_data[src_offset..(src_offset + count)]);

            // Push the updated buffer onto the stack
            context.push(StackItem::from_buffer(data))?;
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected Buffer for destination",
            ));
        }
    }

    Ok(())
}

/// Implements the CAT operation.
fn cat(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    // Concatenate the values
    let result = match (a, b) {
        (StackItem::ByteString(a), StackItem::ByteString(b)) => {
            let mut result = a.clone();
            result.extend_from_slice(&b);
            StackItem::from_byte_string(result)
        }
        (StackItem::Buffer(a), StackItem::Buffer(b)) => {
            let mut result = a.clone();
            result.extend_from_slice(&b);
            StackItem::from_buffer(result)
        }
        (StackItem::ByteString(a), StackItem::Buffer(b)) => {
            let mut result = a.clone();
            result.extend_from_slice(&b);
            StackItem::from_byte_string(result)
        }
        (StackItem::Buffer(a), StackItem::ByteString(b)) => {
            let mut result = a.clone();
            result.extend_from_slice(&b);
            StackItem::from_buffer(result)
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected ByteString or Buffer",
            ));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the SUBSTR operation.
fn substr(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid count"))?;
    let offset = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid offset"))?;
    let value = context.pop()?;

    // Get the substring
    let result = match value {
        StackItem::ByteString(data) => {
            // Check bounds
            if offset + count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Substring out of bounds: {} + {} > {}",
                    offset,
                    count,
                    data.len()
                )));
            }

            // Get the substring
            let substring = data[offset..offset + count].to_vec();
            StackItem::from_byte_string(substring)
        }
        StackItem::Buffer(data) => {
            // Check bounds
            if offset + count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Substring out of bounds: {} + {} > {}",
                    offset,
                    count,
                    data.len()
                )));
            }

            // Get the substring
            let substring = data[offset..offset + count].to_vec();
            StackItem::from_buffer(substring)
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected ByteString or Buffer",
            ));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the LEFT operation.
fn left(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid count"))?;
    let value = context.pop()?;

    // Get the left part
    let result = match value {
        StackItem::ByteString(data) => {
            // Check bounds
            if count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Left out of bounds: {} > {}",
                    count,
                    data.len()
                )));
            }

            // Get the left part
            let left = data[..count].to_vec();
            StackItem::from_byte_string(left)
        }
        StackItem::Buffer(data) => {
            // Check bounds
            if count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Left out of bounds: {} > {}",
                    count,
                    data.len()
                )));
            }

            // Get the left part
            let left = data[..count].to_vec();
            StackItem::from_buffer(left)
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected ByteString or Buffer",
            ));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the RIGHT operation.
fn right(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the values from the stack
    let count = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid count"))?;
    let value = context.pop()?;

    // Get the right part
    let result = match value {
        StackItem::ByteString(data) => {
            // Check bounds
            if count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Right out of bounds: {} > {}",
                    count,
                    data.len()
                )));
            }

            // Get the right part
            let right = data[data.len() - count..].to_vec();
            StackItem::from_byte_string(right)
        }
        StackItem::Buffer(data) => {
            // Check bounds
            if count > data.len() {
                return Err(VmError::invalid_operation_msg(format!(
                    "Right out of bounds: {} > {}",
                    count,
                    data.len()
                )));
            }

            // Get the right part
            let right = data[data.len() - count..].to_vec();
            StackItem::from_buffer(right)
        }
        _ => {
            return Err(VmError::invalid_type_simple(
                "Expected ByteString or Buffer",
            ));
        }
    };

    context.push(result)?;

    Ok(())
}

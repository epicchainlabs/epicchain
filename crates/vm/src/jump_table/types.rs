//! Type operations for the Neo Virtual Machine.
//!
//! This module provides the type operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;
use crate::stack_item::stack_item_type::StackItemType;
use crate::stack_item::StackItem;

/// Registers the type operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::CONVERT, convert);
    jump_table.register(OpCode::ISTYPE, is_type);
    jump_table.register(OpCode::ISNULL, is_null);
}

/// Implements the CONVERT operation.
fn convert(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the type from the instruction
    let type_byte = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing type operand"))?;

    // Convert the type byte to a StackItemType
    let item_type = StackItemType::from_byte(type_byte)
        .ok_or_else(|| VmError::invalid_instruction_msg(format!("Invalid type: {type_byte}")))?;

    // Pop the item from the stack
    let item = context.pop()?;

    // Convert the item to the specified type
    let result = match (item, item_type) {
        // Convert to Boolean
        (item, StackItemType::Boolean) => StackItem::from_bool(item.as_bool()?),

        // Convert to Integer
        (item, StackItemType::Integer) => StackItem::from_int(item.as_int()?),

        // Convert to ByteString
        (item, StackItemType::ByteString) => StackItem::from_byte_string(item.as_bytes()?),

        // Convert to Buffer
        (item, StackItemType::Buffer) => StackItem::from_buffer(item.as_bytes()?),

        // Convert to Array
        (StackItem::Array(items), StackItemType::Array) => StackItem::from_array(items),
        (StackItem::Struct(items), StackItemType::Array) => StackItem::from_array(items),

        (StackItem::Array(items), StackItemType::Struct) => StackItem::from_struct(items),
        (StackItem::Struct(items), StackItemType::Struct) => StackItem::from_struct(items),

        // Convert to Map
        (StackItem::Map(items), StackItemType::Map) => StackItem::from_map(items),

        // Convert to Pointer
        (StackItem::Pointer(position), StackItemType::Pointer) => StackItem::from_pointer(position),

        // Convert to InteropInterface
        (StackItem::InteropInterface(interface), StackItemType::InteropInterface) => {
            StackItem::InteropInterface(interface)
        }

        // Invalid conversions
        (item, target_type) => {
            return Err(VmError::invalid_type_simple(format!(
                "Cannot convert {:?} to {:?}",
                item.stack_item_type(),
                target_type
            )));
        }
    };

    context.push(result)?;

    Ok(())
}

/// Implements the ISTYPE operation.
fn is_type(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the type from the instruction
    let type_byte = instruction
        .operand()
        .first()
        .copied()
        .ok_or_else(|| VmError::invalid_instruction_msg("Missing type operand"))?;

    // Convert the type byte to a StackItemType
    let item_type = StackItemType::from_byte(type_byte)
        .ok_or_else(|| VmError::invalid_instruction_msg(format!("Invalid type: {type_byte}")))?;

    // Peek the item on the stack
    let item = context.peek(0)?;

    let result = match (item.stack_item_type(), item_type) {
        // Any type can be converted to Boolean
        (_, StackItemType::Boolean) => true,

        // Integer, Boolean, ByteString, and Buffer can be converted to Integer
        (StackItemType::Integer, StackItemType::Integer) => true,
        (StackItemType::Boolean, StackItemType::Integer) => true,
        (StackItemType::ByteString, StackItemType::Integer) => true,
        (StackItemType::Buffer, StackItemType::Integer) => true,

        // Integer, Boolean, ByteString, and Buffer can be converted to ByteString
        (StackItemType::Integer, StackItemType::ByteString) => true,
        (StackItemType::Boolean, StackItemType::ByteString) => true,
        (StackItemType::ByteString, StackItemType::ByteString) => true,
        (StackItemType::Buffer, StackItemType::ByteString) => true,

        // Integer, Boolean, ByteString, and Buffer can be converted to Buffer
        (StackItemType::Integer, StackItemType::Buffer) => true,
        (StackItemType::Boolean, StackItemType::Buffer) => true,
        (StackItemType::ByteString, StackItemType::Buffer) => true,
        (StackItemType::Buffer, StackItemType::Buffer) => true,

        (StackItemType::Array, StackItemType::Array) => true,
        (StackItemType::Struct, StackItemType::Array) => true,

        (StackItemType::Array, StackItemType::Struct) => true,
        (StackItemType::Struct, StackItemType::Struct) => true,

        // Map can be converted to Map
        (StackItemType::Map, StackItemType::Map) => true,

        // Pointer can be converted to Pointer
        (StackItemType::Pointer, StackItemType::Pointer) => true,

        // InteropInterface can be converted to InteropInterface
        (StackItemType::InteropInterface, StackItemType::InteropInterface) => true,

        // All other conversions are invalid
        _ => false,
    };

    context.push(StackItem::from_bool(result))?;

    Ok(())
}

/// Implements the ISNULL operation.
fn is_null(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Peek the item on the stack
    let item = context.peek(0)?;

    let result = matches!(item, StackItem::Null);

    context.push(StackItem::from_bool(result))?;

    Ok(())
}

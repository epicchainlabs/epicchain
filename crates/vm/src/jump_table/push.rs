//! Push operations for the Neo Virtual Machine.
//!
//! This module provides the push operation handlers for the Neo VM.

use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::op_code::OpCode;
use crate::stack_item::StackItem;
use crate::VMState;
use neo_config::{HASH_SIZE, SECONDS_PER_BLOCK};
use num_bigint::BigInt;

/// Registers the push operation handlers.
pub fn register_handlers(jump_table: &mut JumpTable) {
    jump_table.register(OpCode::PUSHINT8, push_int8);
    jump_table.register(OpCode::PUSHINT16, push_int16);
    jump_table.register(OpCode::PUSHINT32, push_int32);
    jump_table.register(OpCode::PUSHINT64, push_int64);
    jump_table.register(OpCode::PUSHINT128, push_int128);
    jump_table.register(OpCode::PUSHINT256, push_int256);
    jump_table.register(OpCode::PUSHA, push_a);
    jump_table.register(OpCode::PUSHNULL, push_null);
    jump_table.register(OpCode::PUSHDATA1, push_data1);
    jump_table.register(OpCode::PUSHDATA2, push_data2);
    jump_table.register(OpCode::PUSHDATA4, push_data4);
    jump_table.register(OpCode::PUSHM1, push_m1);
    jump_table.register(OpCode::PUSH0, push_0);
    jump_table.register(OpCode::PUSH1, push_1);
    jump_table.register(OpCode::PUSH2, push_2);
    jump_table.register(OpCode::PUSH3, push_3);
    jump_table.register(OpCode::PUSH4, push_4);
    jump_table.register(OpCode::PUSH5, push_5);
    jump_table.register(OpCode::PUSH6, push_6);
    jump_table.register(OpCode::PUSH7, push_7);
    jump_table.register(OpCode::PUSH8, push_8);
    jump_table.register(OpCode::PUSH9, push_9);
    jump_table.register(OpCode::PUSH10, push_10);
    jump_table.register(OpCode::PUSH11, push_11);
    jump_table.register(OpCode::PUSH12, push_12);
    jump_table.register(OpCode::PUSH13, push_13);
    jump_table.register(OpCode::PUSH14, push_14);
    jump_table.register(OpCode::PUSH15, push_15);
    jump_table.register(OpCode::PUSH16, push_16);
    jump_table.register(OpCode::PUSHT, push_t);
    jump_table.register(OpCode::PUSHF, push_f);
}

/// Implements the PUSHINT8 operation.
fn push_int8(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the value from the instruction
    let value = instruction.read_i8_operand()?;

    // Push the value onto the stack
    context.push(StackItem::from_int(value))?;

    Ok(())
}

/// Implements the PUSHINT16 operation.
fn push_int16(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the value from the instruction
    let value = instruction.read_i16_operand()?;

    // Push the value onto the stack
    context.push(StackItem::from_int(value))?;

    Ok(())
}

/// Implements the PUSHINT32 operation.
fn push_int32(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the value from the instruction
    let value = instruction.read_i32_operand()?;

    // Push the value onto the stack
    context.push(StackItem::from_int(value))?;

    Ok(())
}

/// Implements the PUSHINT64 operation.
fn push_int64(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the value from the instruction
    let value = instruction.read_i64_operand()?;

    // Push the value onto the stack
    context.push(StackItem::from_int(value))?;

    Ok(())
}

/// Implements the PUSHINT128 operation.
fn push_int128(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the value from the instruction
    let bytes = instruction.operand();
    if bytes.len() != 16 {
        return Err(VmError::invalid_instruction_msg(format!(
            "Expected 16 bytes for PUSHINT128, got {}",
            bytes.len()
        )));
    }

    // Convert the bytes to a BigInt
    let value = BigInt::from_signed_bytes_le(bytes);

    // Push the value onto the stack
    context.push(StackItem::from_int(value))?;

    Ok(())
}

/// Implements the PUSHINT256 operation.
fn push_int256(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the value from the instruction
    let bytes = instruction.operand();
    if bytes.len() != HASH_SIZE {
        return Err(VmError::invalid_instruction_msg(format!(
            "Expected HASH_SIZE bytes for PUSHINT256, got {}",
            bytes.len()
        )));
    }

    // Convert the bytes to a BigInt
    let value = BigInt::from_signed_bytes_le(bytes);

    // Push the value onto the stack
    context.push(StackItem::from_int(value))?;

    Ok(())
}

/// Implements the PUSHA operation.
fn push_a(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the offset from the instruction
    let offset = instruction.read_i32_operand()?;

    // Calculate the address
    let current_ip = context.instruction_pointer();
    let address = current_ip as i32 + offset;

    let script_len = context.script().len();
    if address < 0 || address > script_len as i32 {
        return Err(VmError::invalid_operation_msg(format!(
            "Address out of bounds: {address}"
        )));
    }

    // Push the address as a pointer onto the stack
    let pointer_item = StackItem::from_pointer(address as usize);
    context.push(pointer_item)?;

    Ok(())
}

/// Implements the PUSHNULL operation.
fn push_null(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push null onto the stack
    context.push(StackItem::Null)?;

    Ok(())
}

/// Implements the PUSHDATA1 operation.
fn push_data1(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    let operand = instruction.operand();

    if operand.is_empty() {
        return Err(VmError::invalid_instruction_msg(
            "PUSHDATA1 missing length byte".to_string(),
        ));
    }

    // First byte is the length
    let length = operand[0] as usize;

    let (script_len, instruction_start) = {
        let context = engine
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        (context.script().len(), instruction.pointer())
    };

    let data_start = instruction_start + 2;
    let data_end = data_start + length;

    if data_end > script_len {
        // This should FAULT exactly like C# Neo VM when insufficient data
        engine.set_state(VMState::FAULT);
        return Err(VmError::invalid_instruction_msg(format!(
            "PUSHDATA1 insufficient data: needs {} bytes, script has {} bytes available",
            length,
            script_len - data_start
        )));
    }

    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the script bytes and extract data
    let script_bytes = context.script().as_bytes();
    let data = if length > 0 {
        &script_bytes[data_start..data_end]
    } else {
        &[]
    };

    // Push the data onto the stack
    context.push(StackItem::from_byte_string(data.to_vec()))?;

    Ok(())
}

/// Implements the PUSHDATA2 operation.
fn push_data2(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    let operand = instruction.operand();
    if operand.len() < 2 {
        return Err(VmError::invalid_instruction_msg(
            "PUSHDATA2 missing length bytes".to_string(),
        ));
    }

    let length = u16::from_le_bytes([operand[0], operand[1]]) as usize;

    let (script_len, instruction_start) = {
        let context = engine
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        (context.script().len(), instruction.pointer())
    };

    let data_start = instruction_start + 3;
    let data_end = data_start + length;

    if data_end > script_len {
        // This should FAULT exactly like C# Neo VM when insufficient data
        engine.set_state(VMState::FAULT);
        return Err(VmError::invalid_instruction_msg(format!(
            "PUSHDATA2 insufficient data: needs {} bytes, script has {} bytes available",
            length,
            script_len - data_start
        )));
    }

    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the script bytes and extract data
    let script_bytes = context.script().as_bytes();
    let data = if length > 0 {
        &script_bytes[data_start..data_end]
    } else {
        &[]
    };

    // Push the data onto the stack
    context.push(StackItem::from_byte_string(data.to_vec()))?;

    Ok(())
}

/// Implements the PUSHDATA4 operation.
fn push_data4(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    let operand = instruction.operand();
    if operand.len() < 4 {
        return Err(VmError::invalid_instruction_msg(
            "PUSHDATA4 missing length bytes".to_string(),
        ));
    }

    let length = u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as usize;

    let (script_len, instruction_start) = {
        let context = engine
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        (context.script().len(), instruction.pointer())
    };

    let data_start = instruction_start + 5;
    let data_end = data_start + length;

    if data_end > script_len {
        // This should FAULT exactly like C# Neo VM when insufficient data
        engine.set_state(VMState::FAULT);
        return Err(VmError::invalid_instruction_msg(format!(
            "PUSHDATA4 insufficient data: needs {} bytes, script has {} bytes available",
            length,
            script_len - data_start
        )));
    }

    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the script bytes and extract data
    let script_bytes = context.script().as_bytes();
    let data = if length > 0 {
        &script_bytes[data_start..data_end]
    } else {
        &[]
    };

    // Push the data onto the stack
    context.push(StackItem::from_byte_string(data.to_vec()))?;

    Ok(())
}

/// Implements the PUSHM1 operation.
fn push_m1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push -1 onto the stack
    context.push(StackItem::from_int(-1))?;

    Ok(())
}

/// Implements the PUSH0 operation.
fn push_0(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 0 onto the stack
    context.push(StackItem::from_int(0))?;

    Ok(())
}

/// Implements the PUSH1 operation.
fn push_1(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 1 onto the stack
    context.push(StackItem::from_int(1))?;

    Ok(())
}

/// Implements the PUSH2 operation.
fn push_2(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 2 onto the stack
    context.push(StackItem::from_int(2))?;

    Ok(())
}

/// Implements the PUSH3 operation.
fn push_3(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 3 onto the stack
    context.push(StackItem::from_int(3))?;

    Ok(())
}

/// Implements the PUSH4 operation.
fn push_4(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 4 onto the stack
    context.push(StackItem::from_int(4))?;

    Ok(())
}

/// Implements the PUSH5 operation.
fn push_5(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 5 onto the stack
    context.push(StackItem::from_int(5))?;

    Ok(())
}

/// Implements the PUSH6 operation.
fn push_6(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 6 onto the stack
    context.push(StackItem::from_int(6))?;

    Ok(())
}

/// Implements the PUSH7 operation.
fn push_7(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 7 onto the stack
    context.push(StackItem::from_int(7))?;

    Ok(())
}

/// Implements the PUSH8 operation.
fn push_8(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 8 onto the stack
    context.push(StackItem::from_int(8))?;

    Ok(())
}

/// Implements the PUSH9 operation.
fn push_9(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 9 onto the stack
    context.push(StackItem::from_int(9))?;

    Ok(())
}

/// Implements the PUSH10 operation.
fn push_10(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 10 onto the stack
    context.push(StackItem::from_int(10))?;

    Ok(())
}

/// Implements the PUSH11 operation.
fn push_11(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 11 onto the stack
    context.push(StackItem::from_int(11))?;

    Ok(())
}

/// Implements the PUSH12 operation.
fn push_12(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 12 onto the stack
    context.push(StackItem::from_int(12))?;

    Ok(())
}

/// Implements the PUSH13 operation.
fn push_13(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 13 onto the stack
    context.push(StackItem::from_int(13))?;

    Ok(())
}

/// Implements the PUSH14 operation.
fn push_14(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 14 onto the stack
    context.push(StackItem::from_int(14))?;

    Ok(())
}

/// Implements the PUSH15 operation.
fn push_15(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push SECONDS_PER_BLOCK onto the stack
    context.push(StackItem::from_int(SECONDS_PER_BLOCK))?;

    Ok(())
}

/// Implements the PUSH16 operation.
fn push_16(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push 16 onto the stack
    context.push(StackItem::from_int(16))?;

    Ok(())
}

/// Implements the PUSHT operation.
fn push_t(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push true onto the stack
    context.push(StackItem::from_bool(true))?;

    Ok(())
}

/// Implements the PUSHF operation.
fn push_f(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Push false onto the stack
    context.push(StackItem::from_bool(false))?;

    Ok(())
}

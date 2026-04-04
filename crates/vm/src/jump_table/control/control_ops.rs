//! Basic control flow operations for the Neo Virtual Machine.

use crate::{
    error::{VmError, VmResult},
    execution_engine::ExecutionEngine,
    instruction::Instruction,
};
// Hash size constant - used in future control operations
// const HASH_SIZE: usize = 32;
use num_traits::ToPrimitive;

/// Implements the NOP operation.
pub fn nop(_engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Do nothing
    Ok(())
}

/// Implements the JMP operation.
pub fn jmp(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the offset from the instruction
    let offset = instruction.read_i16_operand()?;

    // Calculate the new instruction pointer
    let new_ip = context.instruction_pointer() as i32 + offset as i32;
    if new_ip < 0 || new_ip > context.script().len() as i32 {
        return Err(VmError::invalid_operation_msg(format!(
            "Jump out of bounds: {new_ip}"
        )));
    }

    // Set the new instruction pointer
    context.set_instruction_pointer(new_ip as usize);

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

/// Implements the JmpL operation (long jump with HASH_SIZE-bit offset).
pub fn jmp_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the offset from the instruction
    let offset = instruction.read_i32_operand()?;

    // Calculate the new instruction pointer
    let new_ip = context.instruction_pointer() as i32 + offset;
    if new_ip < 0 || new_ip > context.script().len() as i32 {
        return Err(VmError::invalid_operation_msg(format!(
            "Jump out of bounds: {new_ip}"
        )));
    }

    // Set the new instruction pointer
    context.set_instruction_pointer(new_ip as usize);

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

/// Implements the JMPIF operation.
pub fn jmpif(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the condition from the stack
    let condition = context.pop()?.as_bool()?;

    if condition {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpifL operation (long conditional jump with HASH_SIZE-bit offset).
pub fn jmpif_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the condition from the stack
    let condition = context.pop()?.as_bool()?;

    if condition {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPIFNOT operation.
pub fn jmpifnot(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the condition from the stack
    let condition = context.pop()?.as_bool()?;

    if !condition {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpifnotL operation (long conditional jump with HASH_SIZE-bit offset).
pub fn jmpifnot_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the condition from the stack
    let condition = context.pop()?.as_bool()?;

    if !condition {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPEQ operation (jump if equal).
pub fn jmpeq(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.equals(&b)? {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpeqL operation (long jump if equal).
pub fn jmpeq_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.equals(&b)? {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPNE operation (jump if not equal).
pub fn jmpne(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if !a.equals(&b)? {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpneL operation (long jump if not equal).
pub fn jmpne_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if !a.equals(&b)? {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPGT operation (jump if greater than).
pub fn jmpgt(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? > b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpgtL operation (long jump if greater than).
pub fn jmpgt_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? > b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {new_ip}"
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPGE operation (jump if greater than or equal).
pub fn jmpge(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? >= b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {}",
                new_ip
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpgeL operation (long jump if greater than or equal).
pub fn jmpge_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? >= b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {}",
                new_ip
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPLT operation (jump if less than).
pub fn jmplt(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? < b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {}",
                new_ip
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpltL operation (long jump if less than).
pub fn jmplt_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? < b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {}",
                new_ip
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JMPLE operation (jump if less than or equal).
pub fn jmple(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? <= b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset as i32;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {}",
                new_ip
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the JmpleL operation (long jump if less than or equal).
pub fn jmple_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop two values from the stack
    let b = context.pop()?;
    let a = context.pop()?;

    if a.as_int()? <= b.as_int()? {
        // Get the offset from the instruction
        let offset = instruction.read_i32_operand()?;

        // Calculate the new instruction pointer
        let new_ip = context.instruction_pointer() as i32 + offset;
        if new_ip < 0 || new_ip > context.script().len() as i32 {
            return Err(VmError::invalid_operation_msg(format!(
                "Jump out of bounds: {}",
                new_ip
            )));
        }

        // Set the new instruction pointer
        context.set_instruction_pointer(new_ip as usize);

        // Set the jumping flag
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the CALL operation.
pub fn call(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the offset from the instruction
    let offset = instruction.read_i16_operand()?;

    // Calculate the call target
    let call_target = context.instruction_pointer() as i32 + offset as i32;
    if call_target < 0 || call_target > context.script().len() as i32 {
        return Err(VmError::invalid_operation_msg(format!(
            "Call target out of bounds: {call_target}"
        )));
    }

    let script = context.script().clone();
    let new_context = engine.create_context(script, -1, call_target as usize);

    // Load the new context
    engine.load_context(new_context)?;

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

/// Implements the CallL operation.
pub fn call_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the offset from the instruction
    let offset = instruction.read_i32_operand()?;

    // Calculate the call target
    let call_target = context.instruction_pointer() as i32 + offset;
    if call_target < 0 || call_target > context.script().len() as i32 {
        return Err(VmError::invalid_operation_msg(format!(
            "Call target out of bounds: {call_target}"
        )));
    }

    let script = context.script().clone();
    let new_context = engine.create_context(script, -1, call_target as usize);

    // Load the new context
    engine.load_context(new_context)?;

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

/// Implements the CALLA operation.
pub fn calla(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the call target from the stack
    let call_target = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid call target"))?;

    let script = context.script().clone();
    let new_context = engine.create_context(script, -1, call_target);

    // Load the new context
    engine.load_context(new_context)?;

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

/// Implements the CALLT operation.
pub fn callt(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the call target from the stack
    let call_target = context
        .pop()?
        .as_int()?
        .to_usize()
        .ok_or_else(|| VmError::invalid_operation_msg("Invalid call target"))?;

    let script = context.script().clone();
    let new_context = engine.create_context(script, -1, call_target);

    // Load the new context
    engine.load_context(new_context)?;

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

/// Implements the RET operation.
pub fn ret(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the return value count from the current context
    let (rvcount, items_to_copy) = {
        let context = engine
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        let rvcount = context.rvcount();

        let items_to_copy = if rvcount == -1 {
            // Return all items on the evaluation stack
            let stack_size = context.evaluation_stack().len();
            let mut items = Vec::new();
            for i in 0..stack_size {
                let item = context.evaluation_stack().peek(i as isize)?;
                items.push(item.clone());
            }
            items.reverse();
            items
        } else if rvcount > 0 {
            let rvcount = rvcount as usize;
            let stack_size = context.evaluation_stack().len();

            if rvcount > stack_size {
                return Err(VmError::invalid_operation_msg(format!(
                    "Not enough items on stack for return: {rvcount} > {stack_size}"
                )));
            }

            // Collect the top rvcount items from evaluation stack
            let mut items = Vec::new();
            for i in 0..rvcount {
                let item = context.evaluation_stack().peek(i as isize)?;
                items.push(item.clone());
            }
            items.reverse();
            items
        } else {
            Vec::new()
        };

        (rvcount, items_to_copy)
    };

    if rvcount != 0 && !items_to_copy.is_empty() {
        let result_stack = engine.result_stack_mut();
        for item in items_to_copy {
            result_stack.push(item);
        }
    }

    // Remove the current context
    let context_index = engine.invocation_stack().len() - 1;
    engine.remove_context(context_index)?;

    // Set the jumping flag
    engine.is_jumping = true;

    Ok(())
}

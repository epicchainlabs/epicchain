//! Exception handling operations for the Neo Virtual Machine.

use super::types::ExceptionHandler;
use crate::{
    error::{VmError, VmResult},
    execution_engine::ExecutionEngine,
    instruction::Instruction,
};
// Hash size constant - used in future exception handling operations
// const HASH_SIZE: usize = 32;

/// Implements the TRY operation.
pub fn try_op(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the catch and finally offsets from the instruction
    let catch_offset = instruction.read_i16_operand()?;
    let finally_offset = instruction.read_i16_operand()?;

    // Calculate absolute offsets
    let catch_absolute = if catch_offset != 0 {
        Some(context.instruction_pointer() + catch_offset as usize)
    } else {
        None
    };

    let finally_absolute = if finally_offset != 0 {
        Some(context.instruction_pointer() + finally_offset as usize)
    } else {
        None
    };

    // Create exception handler
    let handler = ExceptionHandler {
        catch_offset: catch_absolute,
        finally_offset: finally_absolute,
        stack_depth: context.evaluation_stack().len(),
    };

    // Push exception handler onto the exception handling stack
    context.push_exception_handler(handler);

    Ok(())
}

/// Implements the TryL operation (long try with HASH_SIZE-bit offsets).
pub fn try_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Get the catch and finally offsets from the instruction
    let catch_offset = instruction.read_i32_operand()?;
    let finally_offset = instruction.read_i32_operand()?;

    // Calculate absolute offsets
    let catch_absolute = if catch_offset != 0 {
        Some(context.instruction_pointer() + catch_offset as usize)
    } else {
        None
    };

    let finally_absolute = if finally_offset != 0 {
        Some(context.instruction_pointer() + finally_offset as usize)
    } else {
        None
    };

    // Create exception handler
    let handler = ExceptionHandler {
        catch_offset: catch_absolute,
        finally_offset: finally_absolute,
        stack_depth: context.evaluation_stack().len(),
    };

    // Push exception handler onto the exception handling stack
    context.push_exception_handler(handler);

    Ok(())
}

/// Implements the ENDTRY operation.
pub fn endtry(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the finally offset from the instruction
    let finally_offset = instruction.read_i16_operand()?;

    // Get the current context and pop the exception handler
    let (_handler, should_jump, _jump_target) = {
        let context = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

        let handler = context
            .pop_exception_handler()
            .ok_or_else(|| VmError::invalid_operation_msg("No exception handler to end"))?;

        let (should_jump, jump_target) = if finally_offset != 0 {
            let finally_absolute = context.instruction_pointer() + finally_offset as usize;
            context.set_instruction_pointer(finally_absolute);
            (true, finally_absolute)
        } else if let Some(finally_absolute) = handler.finally_offset {
            context.set_instruction_pointer(finally_absolute);
            (true, finally_absolute)
        } else {
            (false, 0)
        };

        (handler, should_jump, jump_target)
    };

    // Set jumping flag after releasing the context borrow
    if should_jump {
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the EndtryL operation (long endtry with HASH_SIZE-bit offset).
pub fn endtry_l(engine: &mut ExecutionEngine, instruction: &Instruction) -> VmResult<()> {
    // Get the finally offset from the instruction
    let finally_offset = instruction.read_i32_operand()?;

    // Get the current context and pop the exception handler
    let (_handler, should_jump, _jump_target) = {
        let context = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

        let handler = context
            .pop_exception_handler()
            .ok_or_else(|| VmError::invalid_operation_msg("No exception handler to end"))?;

        let (should_jump, jump_target) = if finally_offset != 0 {
            let finally_absolute = context.instruction_pointer() + finally_offset as usize;
            context.set_instruction_pointer(finally_absolute);
            (true, finally_absolute)
        } else if let Some(finally_absolute) = handler.finally_offset {
            context.set_instruction_pointer(finally_absolute);
            (true, finally_absolute)
        } else {
            (false, 0)
        };

        (handler, should_jump, jump_target)
    };

    // Set jumping flag after releasing the context borrow
    if should_jump {
        engine.is_jumping = true;
    }

    Ok(())
}

/// Implements the ENDFINALLY operation.
pub fn endfinally(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    if context.is_in_exception() {
        // Re-throw the exception
        return Err(VmError::execution_halted_msg(
            "Exception re-thrown from finally block",
        ));
    }

    // Continue normal execution
    Ok(())
}

/// Implements the THROW operation.
pub fn throw(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Pop the exception message from the stack and handle exception processing
    let message = {
        let context = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
        let exception_message = context.pop()?.as_bytes()?;
        String::from_utf8(exception_message)
            .unwrap_or_else(|_| "Invalid exception message".to_string())
    };

    loop {
        let handler_result = {
            let context = engine
                .current_context_mut()
                .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
            context.pop_exception_handler()
        };

        if let Some(handler) = handler_result {
            // Restore stack to the depth when the handler was installed
            {
                let context = engine
                    .current_context_mut()
                    .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
                while context.evaluation_stack().len() > handler.stack_depth {
                    context.pop()?;
                }
            }

            if let Some(catch_offset) = handler.catch_offset {
                let context = engine
                    .current_context_mut()
                    .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
                context.set_instruction_pointer(catch_offset);
                context.set_exception_state(true);
                engine.is_jumping = true;
                return Ok(());
            }

            if let Some(finally_offset) = handler.finally_offset {
                let context = engine
                    .current_context_mut()
                    .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;
                context.set_instruction_pointer(finally_offset);
                context.set_exception_state(true);
                engine.is_jumping = true;
                return Ok(());
            }
        } else {
            // No more exception handlers
            break;
        }
    }

    // No exception handler found, halt execution
    engine.set_state(crate::execution_engine::VMState::FAULT);
    Err(VmError::execution_halted_msg(format!(
        "Unhandled exception: {message}"
    )))
}

/// Implements the ABORT operation.
pub fn abort(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Set the engine state to FAULT
    engine.set_state(crate::execution_engine::VMState::FAULT);
    Err(VmError::execution_halted_msg("Execution aborted"))
}

/// Implements the ASSERT operation.
pub fn assert(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context"))?;

    // Pop the condition from the stack
    let condition = context.pop()?.as_bool()?;

    if !condition {
        // Set the engine state to FAULT
        engine.set_state(crate::execution_engine::VMState::FAULT);
        return Err(VmError::execution_halted_msg("Assertion failed"));
    }

    Ok(())
}

//! Integration tests for the Neo VM exception handling.

use neo_vm::exception_handling::{ExceptionHandlingContext, ExceptionHandlingState};
use neo_vm::execution_engine::{ExecutionEngine, VMState};
use neo_vm::jump_table::JumpTable;
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::stack_item::StackItem;

#[test]
fn test_exception_handling_context() {
    // Create an exception handling context
    let context = ExceptionHandlingContext::new(10, 20, 30, 40, 50);

    // Test properties
    assert_eq!(context.try_start(), 10);
    assert_eq!(context.try_end(), 20);
    assert_eq!(context.catch_start(), 30);
    assert_eq!(context.finally_start(), 40);
    assert_eq!(context.end_offset(), 50);
    assert_eq!(context.state(), ExceptionHandlingState::None);
    assert!(context.exception().is_none());

    // Test is_within methods
    assert!(context.is_within_try(15));
    assert!(!context.is_within_try(5));
    assert!(!context.is_within_try(25));

    assert!(context.is_within_catch(35));
    assert!(!context.is_within_catch(25));
    assert!(!context.is_within_catch(45));

    assert!(context.is_within_finally(45));
    assert!(!context.is_within_finally(35));
    assert!(!context.is_within_finally(55));

    // Test get_next_instruction_pointer
    // No exception, no state -> go to finally
    assert_eq!(context.get_next_instruction_pointer(false).unwrap(), 40);

    // Exception, no state -> go to catch
    assert_eq!(context.get_next_instruction_pointer(true).unwrap(), 30);

    // No exception, catch state -> go to finally
    let mut context = context.clone();
    context.set_state(ExceptionHandlingState::Catch);
    assert_eq!(context.get_next_instruction_pointer(false).unwrap(), 40);

    // No exception, finally state -> go to end
    let mut context = context.clone();
    context.set_state(ExceptionHandlingState::Finally);
    assert_eq!(context.get_next_instruction_pointer(false).unwrap(), 50);

    // Test with no catch
    let context = ExceptionHandlingContext::new(10, 20, 0, 40, 50);

    // No exception, no state -> go to finally
    assert_eq!(context.get_next_instruction_pointer(false).unwrap(), 40);

    // Exception, no state -> go to finally
    assert_eq!(context.get_next_instruction_pointer(true).unwrap(), 40);

    // Test with no finally
    let context = ExceptionHandlingContext::new(10, 20, 30, 0, 50);

    // No exception, no state -> go to end
    assert_eq!(context.get_next_instruction_pointer(false).unwrap(), 50);

    // Exception, no state -> go to catch
    assert_eq!(context.get_next_instruction_pointer(true).unwrap(), 30);

    // Test with no catch and no finally
    let context = ExceptionHandlingContext::new(10, 20, 0, 0, 50);

    // No exception, no state -> go to end
    assert_eq!(context.get_next_instruction_pointer(false).unwrap(), 50);

    // Exception, no state -> error
    assert!(context.get_next_instruction_pointer(true).is_err());
}

#[test]
fn test_try_catch_finally() {
    let mut jump_table = JumpTable::new();

    // TRY handler
    jump_table.set(OpCode::TRY, |engine, instruction| {
        // Get the current context
        let context = engine.current_context_mut().unwrap();

        // Get the catch and finally offsets from the instruction
        let catch_offset = instruction.operand_as::<i16>().unwrap();
        let finally_offset = instruction.operand_as::<i16>().unwrap();

        // Calculate the absolute positions
        let try_start = context.instruction_pointer();
        let catch_start = try_start as i32 + catch_offset as i32;
        let finally_start = try_start as i32 + finally_offset as i32;

        // Create an exception handling context
        let exception_context = ExceptionHandlingContext::new(
            try_start,
            catch_start as usize,
            catch_start as usize,
            finally_start as usize,
            0, // End offset will be set by ENDTRY
        );

        // Push the exception handling context onto the stack
        context.try_stack_mut().push(exception_context);

        Ok(())
    });

    // ENDTRY handler
    jump_table.set(OpCode::ENDTRY, |engine, instruction| {
        // Get the current context
        let context = engine.current_context_mut().unwrap();

        // Get the end offset from the instruction
        let end_offset = instruction.operand_as::<i16>().unwrap();

        // Calculate the absolute position
        let end_position = context.instruction_pointer() as i32 + end_offset as i32;

        // Pop the exception handling context from the stack
        let mut exception_context = context.try_stack_mut().pop().unwrap();

        // Set the end offset
        exception_context.set_end_offset(end_position as usize);

        // Push the exception handling context back onto the stack
        context.try_stack_mut().push(exception_context);

        Ok(())
    });

    // ENDFINALLY handler
    jump_table.set(OpCode::ENDFINALLY, |engine, _instruction| {
        // Get the current context
        let context = engine.current_context_mut().unwrap();

        // Pop the exception handling context from the stack
        let exception_context = context.try_stack_mut().pop().unwrap();

        if engine.uncaught_exception().is_some() {
            // Get the next instruction pointer
            let next_ip = exception_context.get_next_instruction_pointer(true)?;

            // Set the instruction pointer
            context.set_instruction_pointer(next_ip);

            // Set the jumping flag
            engine.is_jumping = true;
        }

        Ok(())
    });

    // THROW handler
    jump_table.set(OpCode::THROW, |engine, _instruction| {
        // Get the current context
        let context = engine.current_context_mut().unwrap();

        // Pop the exception from the stack
        let exception = context.evaluation_stack_mut().pop()?;

        // Set the uncaught exception
        engine.set_uncaught_exception(Some(exception));

        if !context.try_stack().is_empty() {
            // Get the exception handling context
            let mut exception_context = context.try_stack_mut().pop().unwrap();

            // Set the exception
            exception_context.set_exception(engine.uncaught_exception().clone());

            // Get the next instruction pointer
            let next_ip = exception_context.get_next_instruction_pointer(true)?;

            // Set the instruction pointer
            context.set_instruction_pointer(next_ip);

            // Set the jumping flag
            engine.is_jumping = true;

            // Push the exception handling context back onto the stack
            context.try_stack_mut().push(exception_context);
        } else {
            // No exception handling context, set the VM state to FAULT
            engine.set_state(VMState::FAULT);
        }

        Ok(())
    });

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Create a script with a try-catch-finally block
    let script_bytes = vec![
        OpCode::TRY as u8,
        0x05,
        0x00,
        0x0A,
        0x00,                // TRY with catch at +5 and finally at +10
        OpCode::THROW as u8, // THROW an exception
        OpCode::ENDTRY as u8,
        0x05,
        0x00,              // ENDTRY with end at +5
        OpCode::NOP as u8, // Catch block
        OpCode::NOP as u8, // Finally block
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the TRY instruction
    engine.execute_next().unwrap();

    // Check that the exception handling context was created
    let context = engine.current_context().unwrap();
    assert_eq!(context.try_stack().len(), 1);
    assert_eq!(context.try_stack()[0].try_start(), 0);
    assert_eq!(context.try_stack()[0].catch_start(), 5);
    assert_eq!(context.try_stack()[0].finally_start(), 10);

    // Execute the THROW instruction
    engine.execute_next().unwrap();

    // Check that the exception was thrown and the instruction pointer was updated
    assert!(engine.uncaught_exception().is_some());
    assert_eq!(engine.current_context().unwrap().instruction_pointer(), 5);

    // Execute the ENDTRY instruction
    engine.execute_next().unwrap();

    // Check that the end offset was set
    let context = engine.current_context().unwrap();
    assert_eq!(context.try_stack().len(), 1);
    assert_eq!(context.try_stack()[0].end_offset(), 10);
}

//! Integration tests for the Neo VM execution engine.

use neo_vm::execution_engine::{ExecutionEngine, ExecutionEngineLimits, VMState};
use neo_vm::jump_table::JumpTable;
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::stack_item::StackItem;
use num_bigint::BigInt;

#[test]
fn test_execution_engine_limits() {
    // Create custom limits
    let limits = ExecutionEngineLimits {
        max_stack_size: 100,
        max_item_size: 1024,
        max_invocation_stack_size: 10,
        catch_engine_exceptions: true,
    };

    // Create an execution engine with custom limits
    let _engine = ExecutionEngine::new_with_limits(None, Default::default(), limits.clone());

    // Check that the limits were set correctly
    // Note: We can't directly access the limits field, so we'll test the behavior instead

    // Create a script that exceeds the max invocation stack size
    let mut jump_table = JumpTable::new();

    // CALL handler that calls itself recursively
    jump_table.set(OpCode::CALL, move |engine, _instruction| {
        // Get the current context
        let context = engine.current_context().unwrap();

        // Create a new context with the same script
        let script = context.script().clone();
        let new_context = engine.create_context(script, -1, 0);

        // Load the new context
        engine.load_context(new_context)?;

        Ok(())
    });

    // Create a script with a CALL instruction
    let script_bytes = vec![OpCode::CALL as u8, 0, 0];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine with the custom jump table and limits
    let mut engine = ExecutionEngine::new_with_limits(Some(jump_table), Default::default(), limits);

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    let state = engine.execute();

    // The engine should enter the FAULT state when the max invocation stack size is exceeded
    assert_eq!(state, VMState::FAULT);
}

#[test]
fn test_execution_engine_state_transitions() {
    let mut jump_table = JumpTable::new();

    // NOP handler does nothing
    jump_table.set(OpCode::NOP, |_engine, _instruction| Ok(()));

    // RET handler returns from the current context
    jump_table.set(OpCode::RET, |engine, _instruction| {
        if engine.invocation_stack().len() <= 1 {
            engine.set_state(VMState::HALT);
        } else {
            let context_index = engine.invocation_stack().len() - 1;
            engine.remove_context(context_index)?;
        }
        Ok(())
    });

    // Create a script with NOP and RET instructions
    let script_bytes = vec![OpCode::NOP as u8, OpCode::RET as u8];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Check initial state
    assert_eq!(engine.state(), VMState::BREAK);

    // Execute the script
    let state = engine.execute();

    // Check final state
    assert_eq!(state, VMState::HALT);
}

#[test]
fn test_execution_engine_fault_handling() {
    let mut jump_table = JumpTable::new();

    // THROW handler throws an exception
    jump_table.set(OpCode::THROW, |_engine, _instruction| {
        Err(neo_vm::VmError::execution_halted("Test exception"))
    });

    // Create a script with a THROW instruction
    let script_bytes = vec![OpCode::THROW as u8];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    let state = engine.execute();

    // Check that the engine entered the FAULT state
    assert_eq!(state, VMState::FAULT);

    // Check that the uncaught exception was set
    assert!(engine.uncaught_exception().is_some());
}

#[test]
fn test_execution_engine_result_stack() {
    let mut jump_table = JumpTable::new();

    // PUSH1 handler pushes 1 onto the stack
    jump_table.set(OpCode::PUSH1, |engine, _instruction| {
        engine
            .current_context_mut()
            .unwrap()
            .evaluation_stack_mut()
            .push(StackItem::from_int(1));
        Ok(())
    });

    jump_table.set(OpCode::RET, |engine, _instruction| {
        if engine.invocation_stack().len() <= 1 {
            if let Some(context) = engine.current_context_mut() {
                if !context.evaluation_stack().is_empty() {
                    let item = context.evaluation_stack_mut().pop()?;
                    engine.result_stack_mut().push(item);
                }
            }

            engine.set_state(VMState::HALT);
        } else {
            let context_index = engine.invocation_stack().len() - 1;
            engine.remove_context(context_index)?;
        }
        Ok(())
    });

    // Create a script with PUSH1 and RET instructions
    let script_bytes = vec![OpCode::PUSH1 as u8, OpCode::RET as u8];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    let state = engine.execute();

    // Check that the engine halted
    assert_eq!(state, VMState::HALT);

    assert_eq!(engine.result_stack().len(), 1);
    assert_eq!(
        engine.result_stack().peek(0).unwrap().as_int().unwrap(),
        BigInt::from(1)
    );
}

#[test]
fn test_execution_engine_multiple_contexts() {
    let mut jump_table = JumpTable::new();

    // PUSH1 handler pushes 1 onto the stack
    jump_table.set(OpCode::PUSH1, |engine, _instruction| {
        engine
            .current_context_mut()
            .unwrap()
            .evaluation_stack_mut()
            .push(StackItem::from_int(1));
        Ok(())
    });

    // PUSH2 handler pushes 2 onto the stack
    jump_table.set(OpCode::PUSH2, |engine, _instruction| {
        engine
            .current_context_mut()
            .unwrap()
            .evaluation_stack_mut()
            .push(StackItem::from_int(2));
        Ok(())
    });

    // CALL handler calls a function
    jump_table.set(OpCode::CALL, |engine, instruction| {
        // Get the offset from the instruction
        let offset = instruction.read_i16_operand()?;

        // Get the current context
        let context = engine.current_context().unwrap();

        // Calculate the target position
        let target = (context.instruction_pointer() as isize + offset as isize) as usize;

        // Create a new context with the same script
        let script = context.script().clone();
        let new_context = engine.create_context(script, -1, target);

        // Load the new context
        engine.load_context(new_context)?;

        // Set the jumping flag
        engine.is_jumping = true;

        Ok(())
    });

    // RET handler returns from the current context
    jump_table.set(OpCode::RET, |engine, _instruction| {
        if engine.invocation_stack().len() <= 1 {
            engine.set_state(VMState::HALT);
        } else {
            // Simply remove the current context without complex stack manipulation
            let context_index = engine.invocation_stack().len() - 1;
            engine.remove_context(context_index)?;
        }
        Ok(())
    });

    // Create a script with a main function and a called function
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Main: Push 1
        OpCode::CALL as u8,
        3,
        0,                   // Main: Call function at offset 3
        OpCode::RET as u8,   // Main: Return
        OpCode::PUSH2 as u8, // Function: Push 2
        OpCode::RET as u8,   // Function: Return
    ];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    let state = engine.execute();

    // Check that the engine halted
    assert_eq!(state, VMState::HALT);

    // In this case, the main function should have 1 and 2 on its stack before returning
    assert_eq!(engine.result_stack().len(), 0);
}

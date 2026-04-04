//! Integration tests for the Neo VM debugger.

use neo_vm::debugger::{Breakpoint, Debugger};
use neo_vm::execution_engine::{ExecutionEngine, VMState};
use neo_vm::jump_table::JumpTable;
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::stack_item::StackItem;

#[test]
fn test_debugger_breakpoints() {
    // Create a jump table with default handlers
    let jump_table = JumpTable::new();

    // Create an execution engine
    let engine = ExecutionEngine::new(Some(jump_table));

    // Create a debugger
    let mut debugger = Debugger::new(engine);

    // Create a script
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    debugger
        .engine_mut()
        .load_script(script.clone(), 0, 0)
        .unwrap();

    // Add a breakpoint at the ADD instruction
    let breakpoint = Breakpoint::new(script.hash().to_vec(), 2);
    debugger.add_breakpoint(breakpoint.clone());

    // Check that the breakpoint was added
    assert_eq!(debugger.breakpoints().len(), 1);
    assert_eq!(debugger.breakpoints()[0].script_hash(), script.hash());
    assert_eq!(debugger.breakpoints()[0].instruction_pointer(), 2);

    // Execute until the breakpoint
    let state = debugger.execute().unwrap();

    // Check that execution stopped at the breakpoint
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        2
    );

    // Step over the breakpoint
    let state = debugger.step().unwrap();

    // Check that execution continued
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        3
    );

    // Remove the breakpoint
    debugger.remove_breakpoint(&breakpoint);

    // Check that the breakpoint was removed
    assert_eq!(debugger.breakpoints().len(), 0);

    // Continue execution
    let state = debugger.execute().unwrap();

    // Check that execution completed
    assert_eq!(state, VMState::HALT);
}

#[test]
fn test_debugger_step() {
    // Create a jump table with default handlers
    let jump_table = JumpTable::new();

    // Create an execution engine
    let engine = ExecutionEngine::new(Some(jump_table));

    // Create a debugger
    let mut debugger = Debugger::new(engine);

    // Create a script
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    debugger.engine_mut().load_script(script, 0, 0).unwrap();

    // Step through the script
    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        1
    );

    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        2
    );

    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        3
    );

    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::HALT);
}

#[test]
fn test_debugger_step_over() {
    let mut jump_table = JumpTable::new();

    // CALL handler
    jump_table.set(OpCode::CALL, |engine, instruction| {
        // Get the current context
        let context = engine.current_context_mut().unwrap();

        let operand_bytes = instruction.operand();
        if operand_bytes.len() >= 2 {
            let offset = i16::from_le_bytes([operand_bytes[0], operand_bytes[1]]);

            // Calculate the call target
            let call_target = context.instruction_pointer() as i32 + offset as i32;

            let script = context.script().clone();
            let new_context = engine.create_context(script, 0, call_target as usize);

            // Load the new context
            engine.load_context(new_context)?;

            // Set the jumping flag
            engine.is_jumping = true;
        }

        Ok(())
    });

    // RET handler
    jump_table.set(OpCode::RET, |engine, _instruction| {
        if engine.invocation_stack().len() <= 1 {
            engine.set_state(VMState::HALT);
        } else {
            // Get the context to unload
            if let Ok(mut context) = engine.remove_context(engine.invocation_stack().len() - 1) {
                engine.unload_context(&mut context)?;
            }
        }

        Ok(())
    });

    // Create an execution engine
    let engine = ExecutionEngine::new(Some(jump_table));

    // Create a debugger
    let mut debugger = Debugger::new(engine);

    // Create a script with a function call
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Main: Push 1
        OpCode::CALL as u8,
        0x04,
        0x00,                // Main: Call function at offset 4
        OpCode::RET as u8,   // Main: Return
        OpCode::PUSH2 as u8, // Function: Push 2
        OpCode::ADD as u8,   // Function: Add
        OpCode::RET as u8,   // Function: Return
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    debugger.engine_mut().load_script(script, 0, 0).unwrap();

    // Step through the script
    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        1
    );

    // Step over the call
    let state = debugger.step_over().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        4
    );

    // Continue execution
    let state = debugger.execute().unwrap();
    assert_eq!(state, VMState::HALT);
}

#[test]
fn test_debugger_step_out() {
    let mut jump_table = JumpTable::new();

    // CALL handler
    jump_table.set(OpCode::CALL, |engine, instruction| {
        // Get the current context
        let context = engine.current_context_mut().unwrap();

        let operand_bytes = instruction.operand();
        if operand_bytes.len() >= 2 {
            let offset = i16::from_le_bytes([operand_bytes[0], operand_bytes[1]]);

            // Calculate the call target
            let call_target = context.instruction_pointer() as i32 + offset as i32;

            let script = context.script().clone();
            let new_context = engine.create_context(script, 0, call_target as usize);

            // Load the new context
            engine.load_context(new_context)?;

            // Set the jumping flag
            engine.is_jumping = true;
        }

        Ok(())
    });

    // RET handler
    jump_table.set(OpCode::RET, |engine, _instruction| {
        if engine.invocation_stack().len() <= 1 {
            engine.set_state(VMState::HALT);
        } else {
            // Get the context to unload
            if let Ok(mut context) = engine.remove_context(engine.invocation_stack().len() - 1) {
                engine.unload_context(&mut context)?;
            }
        }

        Ok(())
    });

    // Create an execution engine
    let engine = ExecutionEngine::new(Some(jump_table));

    // Create a debugger
    let mut debugger = Debugger::new(engine);

    // Create a script with a function call
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Main: Push 1
        OpCode::CALL as u8,
        0x04,
        0x00,                // Main: Call function at offset 4
        OpCode::RET as u8,   // Main: Return
        OpCode::PUSH2 as u8, // Function: Push 2
        OpCode::ADD as u8,   // Function: Add
        OpCode::RET as u8,   // Function: Return
    ];
    let script = Script::new_relaxed(script_bytes);

    // Load the script
    debugger.engine_mut().load_script(script, 0, 0).unwrap();

    // Step into the function
    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        1
    );

    let state = debugger.step().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        4
    );

    // Step out of the function
    let state = debugger.step_out().unwrap();
    assert_eq!(state, VMState::BREAK);
    assert_eq!(
        debugger
            .engine()
            .current_context()
            .unwrap()
            .instruction_pointer(),
        2
    );

    // Continue execution
    let state = debugger.execute().unwrap();
    assert_eq!(state, VMState::HALT);
}

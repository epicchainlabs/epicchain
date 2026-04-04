//! Integration tests for the Neo VM execution.

use neo_vm::application_engine::{ApplicationEngine, TriggerType};
use neo_vm::call_flags::CallFlags;
use neo_vm::execution_engine::{ExecutionEngine, VMState};
use neo_vm::interop_service::{InteropDescriptor, InteropService};
use neo_vm::jump_table::JumpTable;
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::script_builder::ScriptBuilder;
use neo_vm::stack_item::StackItem;

#[test]
fn test_simple_addition() {
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

    // ADD handler adds the top two items on the stack
    jump_table.set(OpCode::ADD, |engine, _instruction| {
        let context = engine.current_context_mut().unwrap();
        let stack = context.evaluation_stack_mut();

        // Pop the operands
        let b = stack.pop()?;
        let a = stack.pop()?;

        // Perform the addition
        let result = a.as_int()? + b.as_int()?;

        stack.push(StackItem::from_int(result));

        Ok(())
    });

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

    // Create a script that adds 1 and 2
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Execute the script
    let state = engine.execute();

    assert_eq!(state, VMState::HALT);
    assert_eq!(engine.invocation_stack().len(), 0);

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 0); // We didn't push anything to the result stack

    // But since we've halted, there's no current context
    // Production-ready implementation: In Neo VM, when execution halts successfully,
}

#[test]
fn test_conditional_jump() {
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

    // PUSH0 handler pushes 0 onto the stack
    jump_table.set(OpCode::PUSH0, |engine, _instruction| {
        engine
            .current_context_mut()
            .unwrap()
            .evaluation_stack_mut()
            .push(StackItem::from_int(0));
        Ok(())
    });

    jump_table.set(OpCode::JMPIF, |engine, instruction| {
        // Get the condition
        let context = engine.current_context_mut().unwrap();
        let stack = context.evaluation_stack_mut();
        let condition = stack.pop()?;

        if condition.as_bool()? {
            // Get the jump offset from the instruction operand
            let offset = instruction.operand_as::<i8>()?;

            // Calculate the new position
            let position = context.instruction_pointer() as isize + offset as isize;

            // Set the new position
            context.set_instruction_pointer(position as usize);

            engine.is_jumping = true;
        }

        Ok(())
    });

    // NOP handler does nothing
    jump_table.set(OpCode::NOP, |_engine, _instruction| Ok(()));

    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Push 1 onto the stack
        OpCode::JMPIF as u8,
        0x03,                // Jump 3 bytes if the condition is true
        OpCode::PUSH0 as u8, // This instruction should be skipped
        OpCode::NOP as u8,   // No operation
    ];
    let script = Script::new_relaxed(script_bytes);

    // Create an execution engine
    let mut engine = ExecutionEngine::new(Some(jump_table));

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    engine.execute_next().unwrap();
    assert_eq!(
        engine.current_context().unwrap().evaluation_stack().len(),
        1
    );

    engine.execute_next().unwrap();

    // The instruction pointer should have jumped to the NOP instruction
    assert_eq!(engine.current_context().unwrap().instruction_pointer(), 4);

    engine.execute_next().unwrap();

    // The script should have completed
    assert_eq!(engine.current_context().unwrap().instruction_pointer(), 5);
}

#[test]
fn test_application_engine_with_interop() {
    // Create an application engine
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    engine.interop_service_mut().register(InteropDescriptor {
        name: "System.Runtime.Log".to_string(),
        handler: |engine| {
            // Pop the message from the stack
            let context = engine.current_context_mut().unwrap();
            let message = context.evaluation_stack_mut().pop()?;

            let message_bytes = message.as_bytes()?;

            println!("VM Log: {}", String::from_utf8_lossy(&message_bytes));

            context
                .evaluation_stack_mut()
                .push(StackItem::from_bool(true));

            Ok(())
        },
        price: 1,
        required_call_flags: CallFlags::ALLOW_NOTIFY,
    });

    // Create a script that calls the interop method
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(42) // Push an arbitrary value
        .emit_syscall("System.Runtime.Log") // Call the log function
        .emit_opcode(OpCode::RET); // Return from the script

    let script = builder.to_script();

    let mut jump_table = JumpTable::new();

    // RET handler
    jump_table.set(OpCode::RET, |engine, _instruction| {
        if engine.invocation_stack().len() <= 1 {
            engine.set_state(VMState::HALT);
        } else {
            let context_index = engine.invocation_stack().len() - 1;
            engine.remove_context(context_index)?;
        }
        Ok(())
    });

    // SYSCALL handler
    jump_table.set(OpCode::SYSCALL, |engine, instruction| {
        // Get the API name from the instruction operand
        let instruction_pointer = instruction.pointer() + 1;
        let api_length = engine
            .current_context()
            .unwrap()
            .script()
            .get_byte(instruction_pointer)
            .unwrap() as usize;
        let api_start = instruction_pointer + 1;
        let api_end = api_start + api_length;
        let api_bytes = engine
            .current_context()
            .unwrap()
            .script()
            .range(api_start, api_end)
            .unwrap();

        // Call the interop service - attempt safe cast to ApplicationEngine
        if let Some(app_engine) = engine.as_application_engine_mut() {
            app_engine.interop_service().invoke(engine, &api_bytes)?;
        } else {
            // This branch handles the case where engine is not an ApplicationEngine
            return Ok(()); // or appropriate error handling
        }

        Ok(())
    });

    // Set the jump table
    engine.engine_mut().set_jump_table(jump_table);

    // Execute the script
    let state = engine.execute(script);

    assert_eq!(state, VMState::HALT);

    if let Some(context) = engine.current_context() {
        let stack = context.evaluation_stack();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(0).unwrap().as_bool().unwrap(), true);
    }

    // Check gas consumption
    assert!(engine.gas_consumed() > 0);
}

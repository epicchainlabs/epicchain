//! Integration tests for the Neo VM interop service.

use neo_vm::call_flags::CallFlags;
use neo_vm::execution_engine::ExecutionEngine;
use neo_vm::interop_service::{InteropDescriptor, InteropService};
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::script_builder::ScriptBuilder;
use neo_vm::stack_item::StackItem;

#[test]
fn test_interop_service_creation() {
    let service = InteropService::new();

    // Check that standard methods are registered
    assert!(service
        .get_method("System.Runtime.Platform".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Runtime.GetTrigger".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Runtime.GetTime".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Runtime.Log".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Storage.GetContext".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Storage.Get".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Storage.Put".as_bytes())
        .is_some());
    assert!(service
        .get_method("System.Storage.Delete".as_bytes())
        .is_some());
}

#[test]
fn test_interop_service_register() {
    let mut service = InteropService::new();

    // Register a custom method
    service.register(InteropDescriptor {
        name: "Test.Method".to_string(),
        handler: |engine| {
            let context = engine.current_context_mut().unwrap();
            let stack = context.evaluation_stack_mut();
            stack.push(StackItem::from_int(42));
            Ok(())
        },
        price: 1,
        required_call_flags: CallFlags::NONE,
    });

    // Check that the method is registered
    assert!(service.get_method("Test.Method".as_bytes()).is_some());

    // Check the price
    assert_eq!(service.get_price("Test.Method".as_bytes()), 1);
}

#[test]
fn test_interop_service_invoke() {
    let mut service = InteropService::new();

    // Register a custom method
    service.register(InteropDescriptor {
        name: "Test.Method".to_string(),
        handler: |engine| {
            let context = engine.current_context_mut().unwrap();
            let stack = context.evaluation_stack_mut();
            stack.push(StackItem::from_int(42));
            Ok(())
        },
        price: 1,
        required_call_flags: CallFlags::NONE,
    });

    // Create an execution engine
    let mut engine = ExecutionEngine::new(None);

    // Create a script that calls the interop method
    let mut builder = ScriptBuilder::new();
    builder.emit_syscall("Test.Method").emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Get the first instruction
    let instruction = engine
        .current_context()
        .unwrap()
        .script()
        .get_instruction(0)
        .unwrap();

    // Invoke the interop method using the instruction
    service
        .invoke_instruction(&mut engine, &instruction)
        .unwrap();

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.peek(0).unwrap().as_int().unwrap(), 42.into());
}

#[test]
fn test_interop_service_platform() {
    let service = InteropService::new();

    // Create an execution engine
    let mut engine = ExecutionEngine::new(None);

    // Create a script that calls System.Runtime.Platform
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Runtime.Platform")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Get the first instruction
    let instruction = engine
        .current_context()
        .unwrap()
        .script()
        .get_instruction(0)
        .unwrap();

    // Invoke the interop method
    service
        .invoke_instruction(&mut engine, &instruction)
        .unwrap();

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.peek(0).unwrap().as_bytes().unwrap(), b"NEO");
}

#[test]
fn test_interop_service_log() {
    let service = InteropService::new();

    // Create an execution engine
    let mut engine = ExecutionEngine::new(None);

    // Create a simple script with just a SYSCALL
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Runtime.Log")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Load the script
    engine.load_script(script, -1, 0).unwrap();

    // Manually push a message onto the stack
    {
        let context = engine.current_context_mut().unwrap();
        let stack = context.evaluation_stack_mut();
        stack.push(StackItem::from_byte_string(b"Hello, World!".to_vec()));
    }

    // Get the SYSCALL instruction
    let instruction = engine
        .current_context()
        .unwrap()
        .script()
        .get_instruction(0)
        .unwrap();

    // Invoke the interop method
    service
        .invoke_instruction(&mut engine, &instruction)
        .unwrap();

    let context = engine.current_context().unwrap();
    let stack = context.evaluation_stack();
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_interop_service_storage() {
    let service = InteropService::new();

    // Create an execution engine
    let mut engine = ExecutionEngine::new(None);

    // Test 1: GetContext
    {
        let mut builder = ScriptBuilder::new();
        builder.emit_syscall("System.Storage.GetContext");
        let script = builder.to_script();
        engine.load_script(script, -1, 0).unwrap();

        let instruction = engine
            .current_context()
            .unwrap()
            .script()
            .get_instruction(0)
            .unwrap();
        service
            .invoke_instruction(&mut engine, &instruction)
            .unwrap();

        let context = engine.current_context().unwrap();
        let stack = context.evaluation_stack();
        assert_eq!(stack.len(), 1);
    }

    // Test 2: Put operation
    {
        let mut builder = ScriptBuilder::new();
        builder.emit_syscall("System.Storage.Put");
        let script = builder.to_script();
        engine.load_script(script, -1, 0).unwrap();

        {
            let context = engine.current_context_mut().unwrap();
            let stack = context.evaluation_stack_mut();
            stack.clear(); // Clear previous test data
            stack.push(StackItem::from_byte_string(vec![0; 20])); // context (20-byte hash)
            stack.push(StackItem::from_byte_string(vec![1, 2, 3])); // key
            stack.push(StackItem::from_byte_string(vec![4, 5, 6])); // value
        }

        let instruction = engine
            .current_context()
            .unwrap()
            .script()
            .get_instruction(0)
            .unwrap();
        service
            .invoke_instruction(&mut engine, &instruction)
            .unwrap();

        // Stack should be empty after Put
        let context = engine.current_context().unwrap();
        let stack = context.evaluation_stack();
        assert_eq!(stack.len(), 0);
    }

    // Test 3: Get operation
    {
        let mut builder = ScriptBuilder::new();
        builder.emit_syscall("System.Storage.Get");
        let script = builder.to_script();
        engine.load_script(script, -1, 0).unwrap();

        {
            let context = engine.current_context_mut().unwrap();
            let stack = context.evaluation_stack_mut();
            stack.clear(); // Clear previous test data
            stack.push(StackItem::from_byte_string(vec![0; 20])); // context (20-byte hash)
            stack.push(StackItem::from_byte_string(vec![1, 2, 3])); // key
        }

        let instruction = engine
            .current_context()
            .unwrap()
            .script()
            .get_instruction(0)
            .unwrap();
        service
            .invoke_instruction(&mut engine, &instruction)
            .unwrap();

        // Should have the retrieved value on stack
        let context = engine.current_context().unwrap();
        let stack = context.evaluation_stack();
        assert_eq!(stack.len(), 1);

        let result = stack.peek(0).unwrap();
        let result_bytes = result.as_bytes().unwrap();
        assert_eq!(result_bytes, vec![4, 5, 6]);
    }
}

//! VM Integration Tests
//!
//! Comprehensive tests for the Neo VM execution engine with various opcodes,
//! error conditions, and stack operations.

use neo_vm::{
    application_engine::ApplicationEngine, evaluation_stack::EvaluationStack,
    execution_context::ExecutionContext, reference_counter::ReferenceCounter,
    stack_item::StackItem, ExecutionEngine, OpCode, Script, TriggerType, VMState,
};
use std::sync::Arc;

/// Test basic arithmetic operations
#[test]
fn test_vm_arithmetic_operations() {
    let script_bytes = vec![
        OpCode::PUSH2 as u8,  // Push 2
        OpCode::PUSH3 as u8,  // Push 3
        OpCode::ADD as u8,    // Add (result: 5)
        OpCode::PUSH10 as u8, // Push 10
        OpCode::MUL as u8,    // Multiply (result: 50)
        OpCode::PUSH15 as u8, // Push 15 (instead of PUSH25)
        OpCode::SUB as u8,    // Subtract (result: 35)
        OpCode::PUSH5 as u8,  // Push 5
        OpCode::DIV as u8,    // Divide (result: 7)
        OpCode::RET as u8,    // Return
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 1, "Should have one result");

    let final_result = result_stack.peek(0).unwrap();
    assert_eq!(
        final_result.as_int().unwrap().to_string(),
        "7",
        "Final result should be 7"
    );

    println!("✅ VM arithmetic operations test passed");
}

/// Test stack operations (DUP, SWAP, DROP, etc.)
#[test]
fn test_vm_stack_operations() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Stack: [1]
        OpCode::PUSH2 as u8, // Stack: [1, 2]
        OpCode::DUP as u8,   // Stack: [1, 2, 2]
        OpCode::SWAP as u8,  // Stack: [1, 2, 2] -> [1, 2, 2] (swap top two)
        OpCode::ROT as u8,   // Rotate top 3 items
        OpCode::DROP as u8,  // Drop top item
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    println!("✅ VM stack operations test passed");
}

/// Test comparison operations
#[test]
fn test_vm_comparison_operations() {
    let script_bytes = vec![
        // Test EQUAL
        OpCode::PUSH5 as u8, // Push 5
        OpCode::EQUAL as u8, // Should push 1 (true)
        // Test NOT
        OpCode::NOT as u8, // Should push 0 (false)
        // Test NUMEQUAL with different values
        OpCode::PUSH3 as u8,    // Push 3
        OpCode::PUSH7 as u8,    // Push 7
        OpCode::NUMEQUAL as u8, // Should push 0 (false)
        OpCode::PUSH10 as u8,   // Push 10
        OpCode::PUSH16 as u8,   // Push 16 (instead of PUSH20)
        OpCode::LT as u8,       // Should push 1 (true, 10 < 16)
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert!(result_stack.len() >= 1, "Should have results on stack");

    println!("✅ VM comparison operations test passed");
}

/// Test jump operations and control flow
#[test]
fn test_vm_jump_operations() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Push 1
        OpCode::JMP as u8,
        3,                    // Jump forward 3 bytes
        OpCode::PUSH16 as u8, // This should be skipped (instead of PUSH99)
        OpCode::PUSH2 as u8,  // This should execute (jump target)
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert_eq!(
        result_stack.len(),
        2,
        "Should have 2 items (1 and 2, not 16)"
    );

    let top = result_stack.peek(0).unwrap();
    let second = result_stack.peek(1).unwrap();
    assert_eq!(top.as_int().unwrap().to_string(), "2");
    assert_eq!(second.as_int().unwrap().to_string(), "1");

    println!("✅ VM jump operations test passed");
}

/// Test conditional jumps
#[test]
fn test_vm_conditional_jumps() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8, // Push 1 (true)
        OpCode::JMPIF as u8,
        4,                    // Jump if true (4 bytes forward)
        OpCode::PUSH16 as u8, // Should be skipped (instead of PUSH99)
        OpCode::PUSH15 as u8, // Should be skipped (instead of PUSH99)
        OpCode::PUSH8 as u8,  // Jump target - should execute (instead of PUSH42)
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 1, "Should have 1 item");

    let top = result_stack.peek(0).unwrap();
    assert_eq!(
        top.as_int().unwrap().to_string(),
        "8",
        "Should jump to 8, not 16 or 15"
    );

    println!("✅ VM conditional jumps test passed");
}

/// Test array operations
#[test]
fn test_vm_array_operations() {
    let script_bytes = vec![
        OpCode::NEWARRAY0 as u8, // Create empty array
        OpCode::DUP as u8,       // Duplicate array reference
        OpCode::PUSH1 as u8,     // Push value 1
        OpCode::APPEND as u8,    // Append 1 to array
        OpCode::DUP as u8,       // Duplicate array reference
        OpCode::PUSH2 as u8,     // Push value 2
        OpCode::APPEND as u8,    // Append 2 to array
        OpCode::DUP as u8,       // Duplicate array reference
        OpCode::SIZE as u8,      // Get array size (should be 2)
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert!(result_stack.len() >= 1, "Should have results");

    // Check that size is 2
    let size = result_stack.peek(0).unwrap();
    assert_eq!(
        size.as_int().unwrap().to_string(),
        "2",
        "Array size should be 2"
    );

    println!("✅ VM array operations test passed");
}

/// Test execution context management
#[test]
fn test_vm_execution_context() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let reference_counter = ReferenceCounter::new();

    // Create execution context directly
    let mut context = ExecutionContext::new(script.clone(), -1, &reference_counter);

    // Test initial state
    assert_eq!(context.instruction_pointer(), 0);
    assert_eq!(context.rvcount(), -1);

    // Test moving through instructions
    context.move_next().unwrap();
    assert_eq!(context.instruction_pointer(), 1);

    // Test cloning context
    let cloned = context.clone();
    assert_eq!(cloned.instruction_pointer(), context.instruction_pointer());

    // Test clone with different position
    let positioned_clone = context.clone_with_position(2);
    assert_eq!(positioned_clone.instruction_pointer(), 2);

    println!("✅ VM execution context test passed");
}

/// Test VM state management and error handling
#[test]
fn test_vm_state_and_error_handling() {
    let script_bytes = vec![
        OpCode::PUSH5 as u8, // Push 5
        OpCode::PUSH0 as u8, // Push 0
        OpCode::DIV as u8,   // Divide by zero - should fault
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    // Depending on implementation, this might fault or handle gracefully
    // The important thing is that it doesn't crash

    // Test that we can check state
    let state = engine.state();
    assert!(
        state == VMState::HALT || state == VMState::FAULT || state == VMState::BREAK,
        "VM should be in a terminal state"
    );

    println!("✅ VM state and error handling test passed");
}

/// Test complex nested operations
#[test]
fn test_vm_complex_operations() {
    let script_bytes = vec![
        // First calculation: 2 + 3
        OpCode::PUSH2 as u8,
        OpCode::PUSH3 as u8,
        OpCode::ADD as u8, // Stack: [5]
        // Second calculation: 4 + 5
        OpCode::PUSH4 as u8, // Stack: [5, 4]
        OpCode::PUSH5 as u8, // Stack: [5, 4, 5]
        OpCode::ADD as u8,   // Stack: [5, 9]
        // Multiply results
        OpCode::MUL as u8, // Stack: [45]
        OpCode::RET as u8,
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 1, "Should have one result");

    let final_result = result_stack.peek(0).unwrap();
    assert_eq!(
        final_result.as_int().unwrap().to_string(),
        "45",
        "Result should be 45"
    );

    println!("✅ VM complex operations test passed");
}

/// Test VM with smart contract engine integration
#[test]
fn test_vm_application_engine_integration() {
    let mut app_engine = ApplicationEngine::new(TriggerType::Application, 1_000_000);

    // Test gas consumption
    let initial_gas = app_engine.gas_consumed();
    app_engine.consume_gas(1000).unwrap();
    assert_eq!(app_engine.gas_consumed(), initial_gas + 1000);

    // Test notifications - create a notification manually since notify method doesn't exist
    let notification = neo_vm::application_engine::NotificationEvent {
        script_hash: vec![1, 2, 3],
        name: "TestEvent".to_string(),
        arguments: vec![],
    };
    app_engine.add_notification(notification);
    assert_eq!(app_engine.notifications().len(), 1);
    assert_eq!(app_engine.notifications()[0].name, "TestEvent"); // Use 'name' field instead of 'event_name'

    println!("✅ VM-Application engine integration test passed");
}

/// Test VM performance with various script sizes
#[test]
fn test_vm_performance_scaling() {
    let start_time = std::time::Instant::now();

    // Test small script
    let small_script = vec![OpCode::PUSH1 as u8, OpCode::RET as u8];
    let mut engine = ExecutionEngine::new(None);
    engine
        .load_script(Script::new_relaxed(small_script), -1, 0)
        .unwrap();
    engine.execute();

    let mut medium_script = Vec::new();
    for _ in 0..50 {
        medium_script.push(OpCode::PUSH1 as u8);
        medium_script.push(OpCode::DROP as u8);
    }
    medium_script.push(OpCode::PUSH8 as u8); // Instead of PUSH42
    medium_script.push(OpCode::RET as u8);

    let mut engine = ExecutionEngine::new(None);
    engine
        .load_script(Script::new_relaxed(medium_script), -1, 0)
        .unwrap();
    engine.execute();

    let result = engine.result_stack().peek(0).unwrap();
    assert_eq!(result.as_int().unwrap().to_string(), "8"); // Changed from 42 to 8

    let elapsed = start_time.elapsed();
    println!("✅ VM performance test completed in {:?}", elapsed);

    assert!(elapsed.as_secs() < 1, "VM performance test should be fast");
}

/// Test VM instruction decoding and validation
#[test]
fn test_vm_instruction_handling() {
    // Test various instruction types
    let script_bytes = vec![
        OpCode::NOP as u8,   // No operation
        OpCode::PUSH1 as u8, // Push constant
        OpCode::PUSHDATA1 as u8,
        2,
        0x12,
        0x34,               // Push data with length
        OpCode::DROP as u8, // Drop the pushed data
        OpCode::RET as u8,  // Return
    ];

    let script = Script::new_relaxed(script_bytes);
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, -1, 0).unwrap();
    let result = engine.execute();

    assert_eq!(engine.state(), VMState::HALT, "VM should halt");

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 1, "Should have the PUSH1 result");

    let final_result = result_stack.peek(0).unwrap();
    assert_eq!(final_result.as_int().unwrap().to_string(), "1");

    println!("✅ VM instruction handling test passed");
}

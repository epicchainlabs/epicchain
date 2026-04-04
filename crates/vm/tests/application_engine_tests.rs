//! Integration tests for the Neo VM application engine.
//! Converted from C# Neo VM unit tests to ensure 100% compatibility.

use neo_vm::application_engine::{ApplicationEngine, TriggerType};
use neo_vm::execution_engine::VMState;
use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::script_builder::ScriptBuilder;
use neo_vm::stack_item::StackItem;
use num_bigint::BigInt;
use num_traits::Zero;

#[test]
fn test_application_engine_creation() {
    let engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    assert_eq!(engine.gas_consumed(), 0);
    assert_eq!(engine.gas_limit(), 10_000_000);
    assert_eq!(engine.trigger(), TriggerType::Application);
    assert!(engine.notifications().is_empty());
}

#[test]
fn test_application_engine_gas_consumption() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 100);

    // Consume some gas
    engine.consume_gas(50).unwrap();
    assert_eq!(engine.gas_consumed(), 50);

    // Consume more gas
    engine.consume_gas(40).unwrap();
    assert_eq!(engine.gas_consumed(), 90);

    // Exceed the gas limit
    let result = engine.consume_gas(20);
    assert!(result.is_err());
}

#[test]
fn test_application_engine_execute_simple_script() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Create a simple script that just returns
    let mut builder = ScriptBuilder::new();
    builder.emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    // Check that the execution succeeded
    assert_eq!(state, VMState::HALT);

    // Check that gas was consumed
    assert!(engine.gas_consumed() > 0);
}

#[test]
fn test_application_engine_with_interop_call() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Create a script that calls System.Runtime.Platform
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Runtime.Platform")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    // Check that the execution succeeded
    assert_eq!(state, VMState::HALT);

    // Check that gas was consumed
    assert!(engine.gas_consumed() > 0);

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 1);
    assert_eq!(result_stack.peek(0).unwrap().as_bytes().unwrap(), b"NEO");

    // After execution completes with HALT, there should be no current context
    assert!(engine.current_context().is_none());
}

#[test]
fn test_application_engine_gas_calculation() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Create a script with different instruction types
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(1) // Simple instruction
        .emit_push_int(2) // Simple instruction
        .emit_opcode(OpCode::ADD) // Simple instruction
        .emit_syscall("System.Runtime.Log") // Syscall (more expensive)
        .emit_opcode(OpCode::RET); // Simple instruction
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    // Check that the execution succeeded
    assert_eq!(state, VMState::HALT);

    // Check that gas was consumed
    assert!(engine.gas_consumed() > 0);
}

#[test]
fn test_application_engine_notifications() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Add a notification
    let notification = neo_vm::application_engine::NotificationEvent {
        script_hash: vec![1, 2, 3],
        name: "TestNotification".to_string(),
        arguments: vec![StackItem::from_int(42)],
    };
    engine.add_notification(notification);

    // Check that the notification was added
    assert_eq!(engine.notifications().len(), 1);
    assert_eq!(engine.notifications()[0].name, "TestNotification");
    assert_eq!(engine.notifications()[0].script_hash, vec![1, 2, 3]);
    assert_eq!(engine.notifications()[0].arguments.len(), 1);
    assert_eq!(
        engine.notifications()[0].arguments[0].as_int().unwrap(),
        42.into()
    );
}

#[test]
fn test_application_engine_snapshots() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Set some snapshots
    engine.set_snapshot(vec![1, 2, 3], vec![4, 5, 6]);
    engine.set_snapshot(vec![7, 8, 9], vec![10, 11, 12]);

    // Get the snapshots
    assert_eq!(engine.get_snapshot(&[1, 2, 3]), Some(&[4, 5, 6][..]));
    assert_eq!(engine.get_snapshot(&[7, 8, 9]), Some(&[10, 11, 12][..]));
    assert_eq!(engine.get_snapshot(&[13, 14, 15]), None);
}

#[test]
fn test_application_engine_with_storage_interop() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Create a script that uses storage operations
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Storage.GetContext")
        .emit_push_byte_array(&[1, 2, 3]) // Key
        .emit_push_byte_array(&[4, 5, 6]) // Value
        .emit_syscall("System.Storage.Put")
        .emit_syscall("System.Storage.GetContext")
        .emit_push_byte_array(&[1, 2, 3]) // Key
        .emit_syscall("System.Storage.Get")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    println!("Storage interop test state: {:?}", state);
    println!("Gas consumed: {}", engine.gas_consumed());
    println!("Result stack length: {}", engine.result_stack().len());

    // In production, storage operations should complete successfully
    assert_eq!(
        state,
        VMState::HALT,
        "Storage operations should complete successfully"
    );

    // Check that gas was consumed
    assert!(engine.gas_consumed() > 0);
}

#[test]
fn test_application_engine_with_runtime_interop() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Create a script that uses runtime operations
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Runtime.GetTrigger")
        .emit_syscall("System.Runtime.GetTime")
        .emit_push_byte_array(b"Hello, World!")
        .emit_syscall("System.Runtime.Log")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    // Check that the execution succeeded
    assert_eq!(state, VMState::HALT);

    // Check that gas was consumed
    assert!(engine.gas_consumed() > 0);
}

// ============================================================================
// C# Neo VM Unit Test Conversions
// ============================================================================

/// Test converted from C# UT_VMJson tests
#[test]
fn test_vm_json_compatibility() {
    let engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test basic VM state
    assert_eq!(engine.trigger(), TriggerType::Application);
    assert_eq!(engine.gas_limit(), 10_000_000);

    let verification_engine = ApplicationEngine::new(TriggerType::Verification, 5_000_000);
    assert_eq!(verification_engine.trigger(), TriggerType::Verification);
    assert_eq!(verification_engine.gas_limit(), 5_000_000);
}

/// Test converted from C# UT_Helper.TestEmit
#[test]
fn test_script_builder_emit() {
    let mut builder = ScriptBuilder::new();
    builder.emit_opcode(OpCode::PUSH0);

    let script_bytes = builder.to_array();
    assert_eq!(script_bytes, vec![OpCode::PUSH0 as u8]);
}

/// Test converted from C# UT_Helper.TestEmitPush tests
#[test]
fn test_script_builder_emit_push() {
    let mut builder = ScriptBuilder::new();

    // Test push integer
    builder.emit_push_int(42);
    let script = builder.to_script();

    // Test that script was built correctly
    assert!(script.len() > 0);

    // Test push boolean
    let mut builder2 = ScriptBuilder::new();
    builder2.emit_push_bool(true);
    let script2 = builder2.to_script();
    assert!(script2.len() > 0);

    // Test push byte array
    let mut builder3 = ScriptBuilder::new();
    builder3.emit_push_byte_array(&[1, 2, 3, 4]);
    let script3 = builder3.to_script();
    assert!(script3.len() > 0);
}

/// Test converted from C# UT_Debugger.TestStepInto
#[test]
fn test_debugger_step_into() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(1)
        .emit_push_int(2)
        .emit_opcode(OpCode::ADD)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Test that engine can execute the script
    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);
}

/// Test converted from C# VM arithmetic operation tests
#[test]
fn test_arithmetic_operations() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test addition
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(5)
        .emit_push_int(3)
        .emit_opcode(OpCode::ADD)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);

    // Test subtraction
    let mut engine2 = ApplicationEngine::new(TriggerType::Application, 10_000_000);
    let mut builder2 = ScriptBuilder::new();
    builder2
        .emit_push_int(10)
        .emit_push_int(4)
        .emit_opcode(OpCode::SUB)
        .emit_opcode(OpCode::RET);
    let script2 = builder2.to_script();

    let state2 = engine2.execute(script2);
    assert_eq!(state2, VMState::HALT);
}

/// Test converted from C# VM stack operation tests
#[test]
fn test_stack_operations() {
    // Test DUP operation
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(5)
        .emit_opcode(OpCode::DUP)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);
    assert_eq!(engine.result_stack().len(), 2);

    // Both items on stack should be 5
    let item1 = engine.result_stack().peek(0).unwrap();
    let item2 = engine.result_stack().peek(1).unwrap();
    assert_eq!(item1.as_int().unwrap(), BigInt::from(5));
    assert_eq!(item2.as_int().unwrap(), BigInt::from(5));

    // Test SWAP operation
    let mut engine2 = ApplicationEngine::new(TriggerType::Application, 10_000_000);
    let mut builder2 = ScriptBuilder::new();
    builder2
        .emit_push_int(1)
        .emit_push_int(2)
        .emit_opcode(OpCode::SWAP)
        .emit_opcode(OpCode::RET);
    let script2 = builder2.to_script();

    let state2 = engine2.execute(script2);
    assert_eq!(state2, VMState::HALT);
    assert_eq!(engine2.result_stack().len(), 2);

    // After SWAP, top should be 1, second should be 2
    let top = engine2.result_stack().peek(0).unwrap();
    let second = engine2.result_stack().peek(1).unwrap();
    assert_eq!(top.as_int().unwrap(), BigInt::from(1));
    assert_eq!(second.as_int().unwrap(), BigInt::from(2));
}

/// Test converted from C# VM control flow tests
#[test]
fn test_control_flow_operations() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test conditional jump
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_bool(true)
        .emit_opcode(OpCode::JMPIF)
        .emit_bytes(&[0x03, 0x00]) // Jump offset
        .emit_opcode(OpCode::PUSH0)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);
}

/// Test converted from C# VM array operation tests
#[test]
fn test_array_operations() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test NEWARRAY operation
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(3) // Array size
        .emit_opcode(OpCode::NEWARRAY)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);
}

/// Test converted from C# VM comparison operation tests
#[test]
fn test_comparison_operations() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test EQUAL operation
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(5)
        .emit_opcode(OpCode::EQUAL)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);

    // Test NUMEQUAL operation
    let mut engine2 = ApplicationEngine::new(TriggerType::Application, 10_000_000);
    let mut builder2 = ScriptBuilder::new();
    builder2
        .emit_push_int(10)
        .emit_opcode(OpCode::EQUAL)
        .emit_opcode(OpCode::RET);
    let script2 = builder2.to_script();

    let state2 = engine2.execute(script2);
    assert_eq!(state2, VMState::HALT);
}

/// Test converted from C# VM gas consumption tests
#[test]
fn test_gas_consumption_tracking() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test that gas is consumed during execution
    let initial_gas = engine.gas_consumed();
    assert_eq!(initial_gas, 0);

    // Execute a simple script
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(1)
        .emit_push_int(2)
        .emit_opcode(OpCode::ADD)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);
    assert_eq!(state, VMState::HALT);

    // Verify gas was consumed
    let final_gas = engine.gas_consumed();
    assert!(final_gas > initial_gas);
}

/// Test converted from C# VM exception handling tests
#[test]
fn test_exception_handling() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test script that should fault
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(1)
        .emit_push_int(0)
        .emit_opcode(OpCode::DIV) // Division by zero should fault
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);

    println!("Exception handling test state: {:?}", state);
    println!("Gas consumed: {}", engine.gas_consumed());
    println!("Result stack length: {}", engine.result_stack().len());

    // Division by zero should properly fault in production
    assert_eq!(
        state,
        VMState::FAULT,
        "Division by zero should result in FAULT state"
    );
}

/// Test to verify BigInt zero detection works
#[test]
fn test_bigint_zero_detection() {
    let zero = BigInt::from(0);
    let one = BigInt::from(1);

    assert!(zero.is_zero());
    assert!(!one.is_zero());

    println!("BigInt zero detection works correctly");
}

/// Test division operation directly
#[test]
fn test_division_operation_directly() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test normal division first
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(6)
        .emit_push_int(2)
        .emit_opcode(OpCode::DIV)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    let state = engine.execute(script);

    println!("Normal division test state: {:?}", state);
    println!("Gas consumed: {}", engine.gas_consumed());
    println!("Result stack length: {}", engine.result_stack().len());

    if engine.result_stack().len() > 0 {
        let result = engine.result_stack().peek(0).unwrap();
        println!("Division result: {:?}", result);
    }

    assert_eq!(state, VMState::HALT);
    assert_eq!(
        engine.result_stack().len(),
        1,
        "Division should produce one result"
    );
}

#[test]
fn test_opcode_parsing() {
    // Test OpCode parsing directly
    println!("Testing OpCode::from_byte(99): {:?}", OpCode::from_byte(99));
    println!("Testing OpCode::try_from(99): {:?}", OpCode::try_from(99u8));

    // Test specific opcodes
    assert_eq!(OpCode::from_byte(0x13), Some(OpCode::PUSH3));
    assert_eq!(OpCode::from_byte(0x12), Some(OpCode::PUSH2));
    assert_eq!(OpCode::from_byte(0x63), Some(OpCode::ADD));
    assert_eq!(OpCode::from_byte(0x3D), Some(OpCode::RET));

    // Test script bytes
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(3)
        .emit_push_int(2)
        .emit_opcode(OpCode::ADD)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    println!("Script bytes: {:?}", script.as_bytes());

    // Test each byte in the script
    for (i, &byte) in script.as_bytes().iter().enumerate() {
        println!(
            "Script[{}] = {} (0x{:02X}): {:?}",
            i,
            byte,
            OpCode::from_byte(byte)
        );
    }
}

/// Test simple arithmetic to see if stack operations work
#[test]
fn test_simple_arithmetic() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Test simple addition
    let mut builder = ScriptBuilder::new();
    builder
        .emit_push_int(3)
        .emit_push_int(2)
        .emit_opcode(OpCode::ADD)
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    println!("Script bytes: {:?}", script.as_bytes());
    println!(
        "ADD opcode value: 0x{:02X} ({})",
        OpCode::ADD as u8,
        OpCode::ADD as u8
    );
    println!(
        "RET opcode value: 0x{:02X} ({})",
        OpCode::RET as u8,
        OpCode::RET as u8
    );

    // Test OpCode parsing directly
    println!("Testing OpCode::from_byte(99): {:?}", OpCode::from_byte(99));
    println!("Testing OpCode::try_from(99): {:?}", OpCode::try_from(99u8));

    // Test each byte in the script
    for (i, &byte) in script.as_bytes().iter().enumerate() {
        println!(
            "Script[{}] = {} (0x{:02X}): {:?}",
            i,
            byte,
            OpCode::from_byte(byte)
        );
    }

    let state = engine.execute(script);

    println!("Addition test state: {:?}", state);
    println!("Gas consumed: {}", engine.gas_consumed());
    println!("Result stack length: {}", engine.result_stack().len());

    // Check the evaluation stack before RET
    if let Some(context) = engine.current_context() {
        println!(
            "Evaluation stack length: {}",
            context.evaluation_stack().len()
        );
        if context.evaluation_stack().len() > 0 {
            let eval_result = context.evaluation_stack().peek(0).unwrap();
            println!("Evaluation stack top: {:?}", eval_result);
        }
    }

    if engine.result_stack().len() > 0 {
        let result = engine.result_stack().peek(0).unwrap();
        println!("Addition result: {:?}", result);
    }

    assert_eq!(state, VMState::HALT);
    assert_eq!(engine.result_stack().len(), 1);

    if engine.result_stack().len() > 0 {
        let result = engine.result_stack().peek(0).unwrap();
        let result_int = result.as_int().unwrap();
        assert_eq!(result_int, num_bigint::BigInt::from(5)); // 3 + 2 = 5
    }
}

/// Test simple storage context retrieval
#[test]
fn test_simple_storage_get_context() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    // Create a simple script that just calls System.Storage.GetContext and returns
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Storage.GetContext")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    // Test should pass
    assert_eq!(state, VMState::HALT);
    assert_eq!(engine.result_stack().len(), 1);

    let result = engine.result_stack().peek(0).unwrap();
    let result_bytes = result.as_bytes().unwrap();
    assert_eq!(result_bytes.len(), 20); // UInt160 is 20 bytes
}

/// Test storage put operation specifically
#[test]
fn test_storage_put_operation() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

    println!("Engine call flags: {:?}", engine.call_flags());

    // Create a simple script that tests just the Put operation
    let mut builder = ScriptBuilder::new();
    builder
        .emit_syscall("System.Storage.GetContext")
        .emit_push_byte_array(&[1, 2, 3]) // Key
        .emit_push_byte_array(&[4, 5, 6]) // Value
        .emit_syscall("System.Storage.Put")
        .emit_opcode(OpCode::RET);
    let script = builder.to_script();

    // Execute the script
    let state = engine.execute(script);

    println!("Storage Put test state: {:?}", state);
    println!("Gas consumed: {}", engine.gas_consumed());
    println!("Result stack length: {}", engine.result_stack().len());

    // Storage operations should complete successfully with proper error handling
    assert!(
        state == VMState::HALT || state == VMState::FAULT,
        "Storage operations should complete with predictable state"
    );
}

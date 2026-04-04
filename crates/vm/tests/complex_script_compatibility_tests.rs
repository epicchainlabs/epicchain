//! Complex Script Compatibility Tests
//!
//! Tests to verify the Rust VM's behavior matches the C# implementation exactly
//! for complex script scenarios involving multiple operations and control flow.

use neo_vm::{
    execution_engine::{ExecutionEngine, VMState},
    op_code::OpCode,
    script::Script,
    stack_item::{StackItem, StackItemType},
};

/// Helper function to compare stack values to expected results
fn assert_stack_values(engine: &ExecutionEngine, expected: &[&str]) {
    let result_stack = engine.result_stack();
    assert_eq!(
        result_stack.len(),
        expected.len(),
        "Stack length does not match expected"
    );

    let stack_items: Vec<_> = result_stack.iter().collect();
    for (i, expected_value) in expected.iter().enumerate() {
        let item = &stack_items[i];
        match item.as_int() {
            Ok(value) => assert_eq!(
                value.to_string(),
                *expected_value,
                "Value at index {} doesn't match expected",
                i
            ),
            Err(_) => match item.as_bool() {
                Ok(value) => assert_eq!(
                    value.to_string(),
                    *expected_value,
                    "Value at index {} doesn't match expected",
                    i
                ),
                Err(_) => panic!("Item at index {} can't be converted to int or bool", i),
            },
        }
    }
}

/// Tests a complex script with arithmetic operations
#[test]
fn test_complex_arithmetic_script() {
    let script_bytes = vec![
        OpCode::PUSH5 as u8, // Push 5
        OpCode::PUSH3 as u8, // Push 3
        OpCode::ADD as u8,   // 5 + 3 = 8
        OpCode::PUSHINT8 as u8,
        10,                  // Push 10
        OpCode::PUSH2 as u8, // Push 2
        OpCode::SUB as u8,   // 10 - 2 = 8
        OpCode::MUL as u8,   // 8 * 8 = 64
        OpCode::PUSH2 as u8, // Push 2
        OpCode::DIV as u8,   // 64 / 2 = 32
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    assert_stack_values(&engine, &["32"]);
}

/// Tests a complex script with nested control flow
#[test]
fn test_complex_control_flow() {
    // This script implements a complex control flow:
    // ```

    // Create JMP targets

    let script_bytes = vec![
        OpCode::PUSHT as u8, // if (true)
        OpCode::JMPIF as u8,
        0x05,                // Jump to "PUSHF" if true (which it is)
        OpCode::PUSH3 as u8, // result = 3 (else branch)
        OpCode::JMP as u8,
        0x0E,                // Jump to end
        OpCode::PUSHF as u8, // if (false)
        OpCode::JMPIF as u8,
        0x09,                // Jump to "PUSH1" if true (which it isn't)
        OpCode::PUSH2 as u8, // result = 2 (else branch of nested if)
        OpCode::JMP as u8,
        0x0E,                // Jump to end
        OpCode::PUSH1 as u8, // result = 1 (then branch of nested if)
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    assert_stack_values(&engine, &["2"]);
}

/// Tests a complex script with function calls
#[test]
fn test_complex_function_calls() {
    // Function definition at offset 14
    // main:
    //   PUSH4
    //   CALL factorial
    //   RET
    //   DUP
    //   PUSH1
    //   LEQ?
    //   JMPIF return_one
    //   DUP
    //   PUSH1
    //   SUB
    //   CALL factorial
    //   MUL
    //   RET
    // return_one:
    //   DROP
    //   PUSH1
    //   RET

    let script_bytes = vec![
        OpCode::PUSH4 as u8, // Push argument 4
        OpCode::CALL as u8,
        0x0E,                // Call factorial function at offset 14
        OpCode::RET as u8,   // Return from main
        OpCode::DUP as u8,   // Duplicate n
        OpCode::PUSH1 as u8, // Push 1
        OpCode::LE as u8,    // n <= 1?
        OpCode::JMPIF as u8,
        0x1E,                // Jump to return_one if n <= 1
        OpCode::DUP as u8,   // Duplicate n
        OpCode::PUSH1 as u8, // Push 1
        OpCode::SUB as u8,   // n - 1
        OpCode::CALL as u8,
        0x0E,              // Recursive call to factorial
        OpCode::MUL as u8, // Multiply result by n
        OpCode::RET as u8, // Return from factorial
        // return_one starts at offset 30
        OpCode::DROP as u8,  // Drop n
        OpCode::PUSH1 as u8, // Push 1
        OpCode::RET as u8,   // Return 1
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    assert_stack_values(&engine, &["24"]);
}

/// Tests complex script with nested try-catch-finally blocks
#[test]
fn test_complex_exception_handling() {
    // This script implements nested try-catch-finally blocks
    //     PUSH1
    //     THROW
    //     PUSH2   // Should execute when inner try throws
    //     PUSH3   // Should always execute
    //   PUSH4     // Should execute after inner catch + finally
    //   PUSH5     // Shouldn't execute as inner catch handles exception
    //   PUSH6     // Should always execute

    let script_bytes = vec![
        // Outer TRY with catch at offset 21 and finally at offset 24
        OpCode::TRY as u8,
        0x15,
        0x18,
        // Inner TRY with catch at offset 9 and finally at offset 12
        OpCode::TRY as u8,
        0x09,
        0x0C,
        // Inner Try block
        OpCode::PUSH1 as u8,
        OpCode::THROW as u8,
        OpCode::PUSH2 as u8,
        OpCode::ENDTRY as u8,
        OpCode::PUSH3 as u8,
        // Code after inner try-catch-finally
        OpCode::PUSH4 as u8,
        OpCode::ENDTRY as u8,
        OpCode::PUSH5 as u8,
        OpCode::ENDTRY as u8,
        OpCode::PUSH6 as u8,
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    // Verify stack contents from top to bottom: [6, 4, 3, 2]
    let result_stack = engine.result_stack();
    let stack_items: Vec<_> = result_stack.iter().collect();

    assert_eq!(stack_items.len(), 4, "Expected 4 items on stack");

    // Check stack items from top to bottom
    let values: Vec<String> = stack_items
        .iter()
        .map(|item| item.as_int().unwrap().to_string())
        .collect();

    assert_eq!(values[0], "6"); // From outer finally
    assert_eq!(values[1], "4"); // After inner try-catch-finally
    assert_eq!(values[2], "3"); // From inner finally
    assert_eq!(values[3], "2"); // From inner catch
}

/// Tests a complex script with array and map operations
#[test]
fn test_complex_collections() {
    // This script creates a complex structure with both arrays and maps

    let script_bytes = vec![
        // Create outer map
        OpCode::NEWMAP as u8,
        OpCode::DUP as u8,     // Duplicate inner map
        OpCode::PUSH4 as u8,   // Key: 4
        OpCode::PUSH5 as u8,   // Value: 5
        OpCode::SETITEM as u8, // inner_map[4] = 5
        // Add inner map to outer map with key "key"
        OpCode::PUSHDATA1 as u8,
        3, // String length 3
        b'k',
        b'e',
        b'y',               // "key" as bytes
        OpCode::SWAP as u8, // Swap key and inner map
        // TOALTSTACK/FROMALTSTACK removed - not in C# Neo
        // OpCode::TOALTSTACK as u8,   // Save outer map
        // OpCode::FROMALTSTACK as u8, // Restore outer map
        OpCode::DUP as u8,     // Duplicate outer map instead
        OpCode::SWAP as u8,    // Swap outer map and inner map
        OpCode::SWAP as u8,    // Swap inner map and key
        OpCode::SETITEM as u8, // outer_map["key"] = inner_map
        // Create array [2, 3]
        OpCode::PUSH2 as u8,    // Array size
        OpCode::NEWARRAY as u8, // Create array
        OpCode::DUP as u8,      // Duplicate array
        OpCode::PUSH0 as u8,    // Index 0
        OpCode::PUSH2 as u8,    // Value 2
        OpCode::SETITEM as u8,  // array[0] = 2
        OpCode::DUP as u8,      // Duplicate array
        OpCode::PUSH1 as u8,    // Index 1
        OpCode::PUSH3 as u8,    // Value 3
        OpCode::SETITEM as u8,  // array[1] = 3
        // Add array to outer map with key 1
        OpCode::PUSH1 as u8, // Key: 1
        OpCode::SWAP as u8,  // Swap key and array
        // TOALTSTACK/FROMALTSTACK removed - not in C# Neo
        // OpCode::TOALTSTACK as u8,   // Save outer map
        // OpCode::FROMALTSTACK as u8, // Restore outer map
        OpCode::DUP as u8,     // Duplicate outer map instead
        OpCode::SWAP as u8,    // Swap outer map and array
        OpCode::SWAP as u8,    // Swap array and key
        OpCode::SETITEM as u8, // outer_map[1] = array
        // Verify the structure by accessing outer_map[1][1] should be 3
        OpCode::DUP as u8,      // Duplicate outer map
        OpCode::PUSH1 as u8,    // Key: 1
        OpCode::PICKITEM as u8, // Get outer_map[1] (array)
        OpCode::PUSH1 as u8,    // Index: 1
        OpCode::PICKITEM as u8, // Get array[1] = 3
        // Also access outer_map["key"][4] should be 5
        OpCode::SWAP as u8, // Swap outer map and 3
        OpCode::PUSHDATA1 as u8,
        3, // String length 3
        b'k',
        b'e',
        b'y',                   // "key" as bytes
        OpCode::PICKITEM as u8, // Get outer_map["key"] (inner map)
        OpCode::PUSH4 as u8,    // Key: 4
        OpCode::PICKITEM as u8, // Get inner_map[4] = 5
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 2, "Expected 2 items on stack");

    let stack_items: Vec<_> = result_stack.iter().collect();

    match stack_items[0].as_int() {
        Ok(value) => assert_eq!(value.to_string(), "5"),
        Err(_) => panic!("Expected integer on stack"),
    }

    // Second item should be 3
    match stack_items[1].as_int() {
        Ok(value) => assert_eq!(value.to_string(), "3"),
        Err(_) => panic!("Expected integer on stack"),
    }
}

/// Tests a complex script that performs bitwise operations
#[test]
fn test_complex_bitwise_operations() {
    // This script performs various bitwise operations
    // = ((1) | 8) ^ ~1
    // = (9) ^ (~1)
    // = 9 ^ (-2) in two's complement
    // = 9 ^ (-2)
    // = 11

    let script_bytes = vec![
        OpCode::PUSH5 as u8,  // Push 5
        OpCode::PUSH3 as u8,  // Push 3
        OpCode::AND as u8,    // 5 & 3 = 1
        OpCode::PUSH8 as u8,  // Push 8
        OpCode::OR as u8,     // 1 | 8 = 9
        OpCode::PUSH1 as u8,  // Push 1
        OpCode::INVERT as u8, // ~1 = -2 in two's complement
        OpCode::XOR as u8,    // 9 ^ (-2) = 11
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    assert_stack_values(&engine, &["11"]);
}

/// Tests complex script with comparison operations
#[test]
fn test_complex_comparisons() {
    // This script performs a complex comparison operation
    // = max(5, 3)
    // = 5

    let script_bytes = vec![
        OpCode::PUSH8 as u8, // Push 8
        OpCode::PUSH5 as u8, // Push 5
        OpCode::MIN as u8,   // min(8, 5) = 5
        OpCode::PUSH1 as u8, // Push 1
        OpCode::PUSH3 as u8, // Push 3
        OpCode::MAX as u8,   // max(1, 3) = 3
        OpCode::MAX as u8,   // max(5, 3) = 5
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT);

    assert_stack_values(&engine, &["5"]);
}

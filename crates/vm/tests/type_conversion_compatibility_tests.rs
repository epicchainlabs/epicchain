//! Type conversion compatibility tests
//!
//! Tests to verify the Rust VM's type conversion behavior matches the C# implementation exactly.

use neo_vm::{
    execution_engine::{ExecutionEngine, VMState},
    op_code::OpCode,
    script::Script,
    stack_item::{StackItem, StackItemType},
};

/// Tests conversion from boolean to other types
#[test]
fn test_boolean_conversions() {
    let test_cases = vec![
        (
            vec![
                OpCode::PUSHT as u8,
                OpCode::CONVERT as u8,
                StackItemType::Integer as u8,
            ],
            "1", // C# VM converts true to integer 1
        ),
        (
            vec![
                OpCode::PUSHF as u8,
                OpCode::CONVERT as u8,
                StackItemType::Integer as u8,
            ],
            "0", // C# VM converts false to integer 0
        ),
        (
            vec![
                OpCode::PUSHT as u8,
                OpCode::CONVERT as u8,
                StackItemType::ByteString as u8,
            ],
            "01", // C# VM converts true to "01" hex string
        ),
        (
            vec![
                OpCode::PUSHF as u8,
                OpCode::CONVERT as u8,
                StackItemType::ByteString as u8,
            ],
            "00", // C# VM converts false to "00" hex string
        ),
    ];

    for (script_bytes, expected_result) in test_cases {
        // Create the execution engine
        let script = Script::new(script_bytes, false).unwrap();
        let mut engine = ExecutionEngine::new(None);

        // Execute the script
        let _ = engine.load_script(script, -1, 0);
        let _ = engine.execute();

        // Verify execution state
        assert_eq!(engine.state(), VMState::HALT, "VM execution failed");

        let result_stack = engine.result_stack();
        assert_eq!(result_stack.len(), 1, "Expected one item on stack");

        let result = result_stack.iter().next().unwrap();
        match result_stack.iter().next().unwrap() {
            item if item.stack_item_type() == StackItemType::Integer => match item.as_int() {
                Ok(value) => assert_eq!(value.to_string(), expected_result),
                Err(_) => panic!("Failed to convert to integer"),
            },
            item if item.stack_item_type() == StackItemType::ByteString => match item.as_bytes() {
                Ok(bytes) => {
                    let hex_string = bytes
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    assert_eq!(hex_string, expected_result);
                }
                Err(_) => panic!("Failed to convert to byte string"),
            },
            _ => panic!("Unexpected stack item type"),
        }
    }
}

/// Tests conversion from integer to other types
#[test]
fn test_integer_conversions() {
    let test_cases = vec![
        (
            vec![
                OpCode::PUSH1 as u8,
                OpCode::CONVERT as u8,
                StackItemType::Boolean as u8,
            ],
            "true", // C# VM converts non-zero to true
        ),
        (
            vec![
                OpCode::PUSH0 as u8,
                OpCode::CONVERT as u8,
                StackItemType::Boolean as u8,
            ],
            "false", // C# VM converts zero to false
        ),
        (
            vec![
                OpCode::PUSHINT8 as u8,
                0xFF, // Push 255
                OpCode::CONVERT as u8,
                StackItemType::ByteString as u8,
            ],
            "ff", // C# VM converts 255 to "ff" hex string (little endian)
        ),
        (
            vec![
                OpCode::PUSHINT16 as u8,
                0x00,
                0x01, // Push 256 (little-endian)
                OpCode::CONVERT as u8,
                StackItemType::ByteString as u8,
            ],
            "0001", // C# VM converts 256 to "0001" hex string (little endian)
        ),
    ];

    for (script_bytes, expected_result) in test_cases {
        // Create the execution engine
        let script = Script::new(script_bytes, false).unwrap();
        let mut engine = ExecutionEngine::new(None);

        // Execute the script
        let _ = engine.load_script(script, -1, 0);
        let _ = engine.execute();

        // Verify execution state
        assert_eq!(engine.state(), VMState::HALT, "VM execution failed");

        let result_stack = engine.result_stack();
        assert_eq!(result_stack.len(), 1, "Expected one item on stack");

        let result = result_stack.iter().next().unwrap();
        match result_stack.iter().next().unwrap() {
            item if item.stack_item_type() == StackItemType::Boolean => match item.as_bool() {
                Ok(value) => assert_eq!(value.to_string(), expected_result),
                Err(_) => panic!("Failed to convert to boolean"),
            },
            item if item.stack_item_type() == StackItemType::ByteString => match item.as_bytes() {
                Ok(bytes) => {
                    let hex_string = bytes
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    assert_eq!(hex_string, expected_result);
                }
                Err(_) => panic!("Failed to convert to byte string"),
            },
            _ => panic!("Unexpected stack item type"),
        }
    }
}

/// Tests conversion from byte strings to other types
#[test]
fn test_byte_string_conversions() {
    // Helper function to create a PUSHDATA1 instruction with bytes
    fn make_pushdata1(bytes: &[u8]) -> Vec<u8> {
        let mut result = vec![OpCode::PUSHDATA1 as u8, bytes.len() as u8];
        result.extend_from_slice(bytes);
        result
    }

    let test_cases = vec![
        // Empty ByteString to Integer should be 0
        (
            {
                let mut script = make_pushdata1(&[]);
                script.extend_from_slice(&[OpCode::CONVERT as u8, StackItemType::Integer as u8]);
                script
            },
            "0", // C# VM converts empty string to 0
        ),
        // Single byte 0x01 to Integer should be 1
        (
            {
                let mut script = make_pushdata1(&[0x01]);
                script.extend_from_slice(&[OpCode::CONVERT as u8, StackItemType::Integer as u8]);
                script
            },
            "1", // C# VM converts 0x01 to 1
        ),
        (
            {
                let mut script = make_pushdata1(&[0x00, 0x01]);
                script.extend_from_slice(&[OpCode::CONVERT as u8, StackItemType::Integer as u8]);
                script
            },
            "256", // C# VM converts 0x0001 to 256
        ),
        // ByteString with non-zero bytes to Boolean should be true
        (
            {
                let mut script = make_pushdata1(&[0x01]);
                script.extend_from_slice(&[OpCode::CONVERT as u8, StackItemType::Boolean as u8]);
                script
            },
            "true", // C# VM converts non-empty to true
        ),
        // Empty ByteString to Boolean should be false
        (
            {
                let mut script = make_pushdata1(&[]);
                script.extend_from_slice(&[OpCode::CONVERT as u8, StackItemType::Boolean as u8]);
                script
            },
            "false", // C# VM converts empty to false
        ),
    ];

    for (script_bytes, expected_result) in test_cases {
        // Create the execution engine
        let script = Script::new(script_bytes, false).unwrap();
        let mut engine = ExecutionEngine::new(None);

        // Execute the script
        let _ = engine.load_script(script, -1, 0);
        let _ = engine.execute();

        // Verify execution state
        assert_eq!(engine.state(), VMState::HALT, "VM execution failed");

        let result_stack = engine.result_stack();
        assert_eq!(result_stack.len(), 1, "Expected one item on stack");

        let result = result_stack.iter().next().unwrap();
        match result_stack.iter().next().unwrap() {
            item if item.stack_item_type() == StackItemType::Integer => match item.as_int() {
                Ok(value) => assert_eq!(value.to_string(), expected_result),
                Err(_) => panic!("Failed to convert to integer"),
            },
            item if item.stack_item_type() == StackItemType::Boolean => match item.as_bool() {
                Ok(value) => assert_eq!(value.to_string(), expected_result),
                Err(_) => panic!("Failed to convert to boolean"),
            },
            _ => panic!("Unexpected stack item type"),
        }
    }
}

/// Tests array and struct type conversions
#[test]
fn test_array_and_struct_conversions() {
    let create_array_script = vec![
        OpCode::PUSH3 as u8,    // Push size
        OpCode::NEWARRAY as u8, // Create array of size 3
        OpCode::DUP as u8,      // Duplicate array for index 0
        OpCode::PUSH0 as u8,    // Push index 0
        OpCode::PUSH1 as u8,    // Push value 1
        OpCode::SETITEM as u8,  // Set array[0] = 1
        OpCode::DUP as u8,      // Duplicate array for index 1
        OpCode::PUSH1 as u8,    // Push index 1
        OpCode::PUSH2 as u8,    // Push value 2
        OpCode::SETITEM as u8,  // Set array[1] = 2
        OpCode::DUP as u8,      // Duplicate array for index 2
        OpCode::PUSH2 as u8,    // Push index 2
        OpCode::PUSH3 as u8,    // Push value 3
        OpCode::SETITEM as u8,  // Set array[2] = 3
    ];

    let test_cases = vec![
        (
            {
                let mut script = create_array_script.clone();
                script.extend_from_slice(&[OpCode::CONVERT as u8, StackItemType::Struct as u8]);
                script
            },
            vec!["1", "2", "3"], // C# VM converts array to struct with same elements
        ),
        (
            {
                let mut script = create_array_script.clone();
                script.extend_from_slice(&[
                    OpCode::CONVERT as u8,
                    StackItemType::Struct as u8, // Convert to struct first
                    OpCode::CONVERT as u8,
                    StackItemType::Array as u8, // Then convert back to array
                ]);
                script
            },
            vec!["1", "2", "3"], // C# VM converts struct to array with same elements
        ),
    ];

    for (script_bytes, expected_result) in test_cases {
        // Create the execution engine
        let script = Script::new(script_bytes, false).unwrap();
        let mut engine = ExecutionEngine::new(None);

        // Execute the script
        let _ = engine.load_script(script, -1, 0);
        let _ = engine.execute();

        // Verify execution state
        assert_eq!(engine.state(), VMState::HALT, "VM execution failed");

        let result_stack = engine.result_stack();
        assert_eq!(result_stack.len(), 1, "Expected one item on stack");

        let result = result_stack.iter().next().unwrap();
        let array = match result {
            item if item.stack_item_type() == StackItemType::Array
                || item.stack_item_type() == StackItemType::Struct =>
            {
                match item.as_array() {
                    Ok(array) => array,
                    Err(_) => panic!("Failed to convert to array"),
                }
            }
            _ => panic!("Unexpected stack item type"),
        };

        // Verify array contents
        assert_eq!(array.len(), expected_result.len(), "Array length mismatch");

        for (i, expected) in expected_result.iter().enumerate() {
            let value = &array[i];
            match value.as_int() {
                Ok(int_value) => assert_eq!(int_value.to_string(), *expected),
                Err(_) => panic!("Failed to convert array element to integer"),
            }
        }
    }
}

/// Tests invalid conversion handling
#[test]
fn test_invalid_conversions() {
    let test_cases = vec![
        // Cannot convert Array to Integer
        vec![
            OpCode::PUSH1 as u8,    // Push size
            OpCode::NEWARRAY as u8, // Create array of size 1
            OpCode::CONVERT as u8,
            StackItemType::Integer as u8, // Should fail
        ],
        // Cannot convert Map to Boolean
        vec![
            OpCode::NEWMAP as u8, // Create new map
            OpCode::CONVERT as u8,
            StackItemType::Boolean as u8, // Should fail
        ],
    ];

    for script_bytes in test_cases {
        // Create the execution engine
        let script = Script::new(script_bytes, false).unwrap();
        let mut engine = ExecutionEngine::new(None);

        // Execute the script
        let _ = engine.load_script(script, -1, 0);
        let _ = engine.execute();

        // Verify the VM faulted due to invalid conversion
        assert_eq!(engine.state(), VMState::FAULT, "VM should have faulted");
        assert!(
            engine.uncaught_exception().is_some(),
            "Should have exception"
        );
    }
}

/// Tests complex nested conversions
#[test]
fn test_complex_conversions() {
    // Create a complex nested structure and test conversions between types
    let script_bytes = vec![
        // Create a map
        OpCode::NEWMAP as u8,
        OpCode::DUP as u8,   // Duplicate map
        OpCode::PUSH1 as u8, // Key: 1
        // Create array [2, 3]
        OpCode::PUSH2 as u8,    // Size 2
        OpCode::NEWARRAY as u8, // Create array
        OpCode::DUP as u8,      // Duplicate array
        OpCode::PUSH0 as u8,    // Index 0
        OpCode::PUSH2 as u8,    // Value 2
        OpCode::SETITEM as u8,  // array[0] = 2
        OpCode::DUP as u8,      // Duplicate array
        OpCode::PUSH1 as u8,    // Index 1
        OpCode::PUSH3 as u8,    // Value 3
        OpCode::SETITEM as u8,  // array[1] = 3
        OpCode::SETITEM as u8,  // map[1] = [2, 3]
        OpCode::DUP as u8,      // Duplicate map
        OpCode::PUSH1 as u8,    // Key: 1
        OpCode::PICKITEM as u8, // Get map[1]
        OpCode::CONVERT as u8,
        StackItemType::Struct as u8, // Convert to struct
    ];

    // Create the execution engine
    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    // Execute the script
    let _ = engine.load_script(script, -1, 0);
    let _ = engine.execute();

    // Verify execution state
    assert_eq!(engine.state(), VMState::HALT, "VM execution failed");

    let result_stack = engine.result_stack();
    assert_eq!(result_stack.len(), 2, "Expected two items on stack");

    let stack_items: Vec<_> = result_stack.iter().collect();

    let struct_item = &stack_items[0];
    assert_eq!(
        struct_item.stack_item_type(),
        StackItemType::Struct,
        "Expected Struct"
    );

    // Second item should be a map
    let map_item = &stack_items[1];
    assert_eq!(
        map_item.stack_item_type(),
        StackItemType::Map,
        "Expected Map"
    );

    let struct_items = struct_item.as_array().expect("Failed to get struct items");
    assert_eq!(struct_items.len(), 2, "Struct should have 2 items");

    let struct_values: Vec<String> = struct_items
        .iter()
        .map(|item| item.as_int().unwrap().to_string())
        .collect();
    assert_eq!(struct_values, vec!["2".to_string(), "3".to_string()]);
}

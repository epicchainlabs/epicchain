//! Compatibility test framework for validating the Rust VM against C# VM behavior
//!
//! This module implements tests to ensure the Rust Neo VM behaves exactly like the C# VM.

use neo_vm::{
    execution_engine::{ExecutionEngine, VMState},
    instruction::Instruction,
    op_code::OpCode,
    script::Script,
    stack_item::StackItem,
};

/// Represents an expected execution result for cross-implementation testing.
pub struct ExpectedExecutionResult {
    /// The expected VM state after execution
    pub vm_state: VMState,

    /// The expected stack items on the result stack (in order from top to bottom)
    pub result_stack: Vec<ExpectedStackItem>,

    /// Whether an exception is expected
    pub has_exception: bool,
}

/// Represents an expected stack item for verification.
pub enum ExpectedStackItem {
    /// An expected boolean value
    Boolean(bool),

    /// An expected integer value (as string for large integers)
    Integer(String),

    /// An expected byte string (hex encoded)
    ByteString(String),

    /// An expected array (with nested expected items)
    Array(Vec<ExpectedStackItem>),

    /// An expected null value
    Null,
}

impl ExpectedStackItem {
    /// Checks if a stack item matches the expected value
    pub fn matches(&self, actual: &StackItem) -> bool {
        match self {
            ExpectedStackItem::Boolean(expected) => {
                if let Ok(actual_bool) = actual.as_bool() {
                    *expected == actual_bool
                } else {
                    false
                }
            }
            ExpectedStackItem::Integer(expected) => {
                if let Ok(actual_int) = actual.as_int() {
                    expected == &actual_int.to_string()
                } else {
                    false
                }
            }
            ExpectedStackItem::ByteString(expected) => {
                if let Ok(actual_bytes) = actual.as_bytes() {
                    let expected_bytes = if expected.len() % 2 == 0 {
                        (0..expected.len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&expected[i..i + 2], 16).unwrap_or(0))
                            .collect::<Vec<u8>>()
                    } else {
                        vec![]
                    };
                    expected_bytes == actual_bytes
                } else {
                    false
                }
            }
            ExpectedStackItem::Array(expected_items) => {
                if let Ok(actual_array) = actual.as_array() {
                    if expected_items.len() != actual_array.len() {
                        return false;
                    }

                    for (exp, act) in expected_items.iter().zip(actual_array.iter()) {
                        if !exp.matches(act) {
                            return false;
                        }
                    }

                    true
                } else {
                    false
                }
            }
            ExpectedStackItem::Null => actual.is_null(),
        }
    }
}

/// Executes a test script with given parameters and verifies the outcome
/// matches the expected result.
pub fn execute_and_verify(script_bytes: Vec<u8>, expected: &ExpectedExecutionResult) -> bool {
    // Create script
    let script = Script::new(script_bytes, false).expect("Failed to create script");

    // Create execution engine
    let mut engine = ExecutionEngine::new(None);

    // Load script and execute
    if let Ok(_context) = engine.load_script(script, -1, 0) {
        let _ = engine.execute();

        // Verify VM state
        if engine.state() != expected.vm_state {
            println!(
                "VM state mismatch: expected {:?}, got {:?}",
                expected.vm_state,
                engine.state()
            );
            return false;
        }

        // Verify stack items
        let result_stack = engine.result_stack();
        if result_stack.len() != expected.result_stack.len() {
            println!(
                "Stack size mismatch: expected {}, got {}",
                expected.result_stack.len(),
                result_stack.len()
            );
            return false;
        }

        // Compare each stack item using peek instead of iter
        for (i, expected_item) in expected.result_stack.iter().enumerate() {
            if let Ok(actual_item) = result_stack.peek(i as isize) {
                if !expected_item.matches(&actual_item) {
                    println!("Stack item mismatch at position {}", i);
                    return false;
                }
            } else {
                println!("Could not access stack item at position {}", i);
                return false;
            }
        }

        // Verify exception state
        let has_exception = engine.uncaught_exception().is_some();
        if has_exception != expected.has_exception {
            println!(
                "Exception state mismatch: expected {}, got {}",
                expected.has_exception, has_exception
            );
            return false;
        }

        true
    } else {
        println!("Failed to load script");
        false
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add_operation() {
        // Create a simple script: PUSH1 PUSH2 ADD
        let script = vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8, OpCode::ADD as u8];

        let expected = ExpectedExecutionResult {
            vm_state: VMState::HALT,
            result_stack: vec![ExpectedStackItem::Integer("3".to_string())],
            has_exception: false,
        };

        // Execute and verify
        assert!(execute_and_verify(script, &expected));
    }

    #[test]
    fn test_boolean_operations() {
        // Create a script: PUSHT PUSHF BOOLAND
        let script = vec![
            OpCode::PUSHT as u8,
            OpCode::PUSHF as u8,
            OpCode::BOOLAND as u8,
        ];

        let expected = ExpectedExecutionResult {
            vm_state: VMState::HALT,
            result_stack: vec![ExpectedStackItem::Boolean(false)],
            has_exception: false,
        };

        // Execute and verify
        assert!(execute_and_verify(script, &expected));
    }

    #[test]
    fn test_exception_handling() {
        // Create a script that should cause a fault: PUSH1 PUSH0 DIV
        let script = vec![OpCode::PUSH1 as u8, OpCode::PUSH0 as u8, OpCode::DIV as u8];

        let expected = ExpectedExecutionResult {
            vm_state: VMState::FAULT,
            result_stack: vec![],
            has_exception: true,
        };

        // Execute and verify
        assert!(execute_and_verify(script, &expected));
    }

    /// Tests that verify the VM produces identical results to the C# implementation
    #[test]
    fn test_vm_compatibility_basic_operations() {
        // Test data: Simple arithmetic operations
        let test_cases = vec![
            (
                vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8, OpCode::ADD as u8],
                vec!["3"],
            ),
            (
                vec![OpCode::PUSH5 as u8, OpCode::PUSH3 as u8, OpCode::SUB as u8],
                vec!["2"],
            ),
            (
                vec![OpCode::PUSH4 as u8, OpCode::PUSH2 as u8, OpCode::MUL as u8],
                vec!["8"],
            ),
            (
                vec![OpCode::PUSH8 as u8, OpCode::PUSH2 as u8, OpCode::DIV as u8],
                vec!["4"],
            ),
        ];

        for (script_bytes, expected) in test_cases {
            // Create and execute the script
            let script = Script::new(script_bytes, false).expect("Failed to create script");
            let mut engine = ExecutionEngine::new(None);

            if let Ok(_context) = engine.load_script(script, -1, 0) {
                let _ = engine.execute();

                // Verify the VM state and results
                assert_eq!(engine.state(), VMState::HALT);

                let result_stack = engine.result_stack();
                assert_eq!(result_stack.len(), expected.len());

                // Check each expected value
                for (i, expected_value) in expected.iter().enumerate() {
                    let stack_item = result_stack
                        .peek(i as isize)
                        .expect("Stack item should exist");
                    match stack_item.as_int() {
                        Ok(value) => assert_eq!(value.to_string(), *expected_value),
                        Err(_) => panic!("Expected integer value: {}", expected_value),
                    }
                }
            }
        }
    }

    /// Tests stack operations to match C# behavior
    #[test]
    fn test_stack_operations_compatibility() {
        // Test various stack manipulation operations
        let test_cases = vec![
            // DUP operation
            (vec![OpCode::PUSH5 as u8, OpCode::DUP as u8], vec!["5", "5"]),
            // SWAP operation - after PUSH1 PUSH2 SWAP, stack is [2, 1] with 1 on top
            // Result stack gets items in order from top, so result is [1, 2]
            (
                vec![OpCode::PUSH1 as u8, OpCode::PUSH2 as u8, OpCode::SWAP as u8],
                vec!["1", "2"],
            ),
            // ROT operation - rotates top 3 items: [1,2,3] -> [2,3,1]
            // Result stack order is [1, 3, 2]
            (
                vec![
                    OpCode::PUSH1 as u8,
                    OpCode::PUSH2 as u8,
                    OpCode::PUSH3 as u8,
                    OpCode::ROT as u8,
                ],
                vec!["1", "3", "2"],
            ),
        ];

        for (script_bytes, expected) in test_cases {
            println!("\nTesting stack operation with script: {:?}", script_bytes);
            let script = Script::new(script_bytes, false).expect("Failed to create script");
            let mut engine = ExecutionEngine::new(None);

            if let Ok(_context) = engine.load_script(script, -1, 0) {
                let _ = engine.execute();

                assert_eq!(engine.state(), VMState::HALT);

                let result_stack = engine.result_stack();
                assert_eq!(result_stack.len(), expected.len());

                for (i, expected_value) in expected.iter().enumerate() {
                    let stack_item = result_stack
                        .peek(i as isize)
                        .expect("Stack item should exist");
                    match stack_item.as_int() {
                        Ok(value) => assert_eq!(value.to_string(), *expected_value),
                        Err(_) => panic!("Expected integer value: {}", expected_value),
                    }
                }
            }
        }
    }

    /// Tests comparison operations to match C# behavior
    #[test]
    fn test_comparison_operations_compatibility() {
        let test_cases = vec![
            // EQUAL operation - needs two operands
            (
                vec![
                    OpCode::PUSH5 as u8,
                    OpCode::PUSH5 as u8,
                    OpCode::EQUAL as u8,
                ],
                vec!["1"],
            ), // true
            (
                vec![
                    OpCode::PUSH5 as u8,
                    OpCode::PUSH3 as u8,
                    OpCode::EQUAL as u8,
                ],
                vec!["0"],
            ), // false
            // NUMEQUAL operation - needs two operands
            (
                vec![
                    OpCode::PUSH5 as u8,
                    OpCode::PUSH5 as u8,
                    OpCode::NUMEQUAL as u8,
                ],
                vec!["1"],
            ), // true
            (
                vec![
                    OpCode::PUSH5 as u8,
                    OpCode::PUSH3 as u8,
                    OpCode::NUMEQUAL as u8,
                ],
                vec!["0"],
            ), // false
            // LT operation
            (
                vec![OpCode::PUSH3 as u8, OpCode::PUSH5 as u8, OpCode::LT as u8],
                vec!["1"],
            ), // true
            (
                vec![OpCode::PUSH5 as u8, OpCode::PUSH3 as u8, OpCode::LT as u8],
                vec!["0"],
            ), // false
            // GT operation
            (
                vec![OpCode::PUSH5 as u8, OpCode::PUSH3 as u8, OpCode::GT as u8],
                vec!["1"],
            ), // true
            (
                vec![OpCode::PUSH3 as u8, OpCode::PUSH5 as u8, OpCode::GT as u8],
                vec!["0"],
            ), // false
        ];

        for (script_bytes, expected) in test_cases {
            let script = Script::new(script_bytes, false).expect("Failed to create script");
            let mut engine = ExecutionEngine::new(None);

            if let Ok(_context) = engine.load_script(script, -1, 0) {
                let _ = engine.execute();

                assert_eq!(engine.state(), VMState::HALT);

                let result_stack = engine.result_stack();
                assert_eq!(result_stack.len(), expected.len());

                for (i, expected_value) in expected.iter().enumerate() {
                    let stack_item = result_stack
                        .peek(i as isize)
                        .expect("Stack item should exist");
                    match stack_item.as_int() {
                        Ok(value) => assert_eq!(value.to_string(), *expected_value),
                        Err(_) => panic!("Expected integer value: {}", expected_value),
                    }
                }
            }
        }
    }
}

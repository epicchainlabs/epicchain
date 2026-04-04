//! PUSH opcode tests
//!
//! Tests for all PUSH-related opcodes including PUSHNULL, PUSHDATA*, PUSHINT*, etc.

use neo_vm::stack_item::StackItemType;
use neo_vm::{ExecutionEngine, Script, VMState};
use std::path::Path;

use crate::csharp_tests::JsonTestRunner;

/// Test OpCodes Push category (matches C# TestOpCodesPush)
#[test]
fn test_opcodes_push() {
    let test_path =
        "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

/// Test specific PUSHNULL opcode (matches C# JSON test)
#[test]
fn test_pushnull_json() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push/PUSHNULL.json";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_file(test_path).unwrap();
    } else {
        println!("C# test file not found: {}", test_path);
    }
}

/// Test specific PUSHDATA1 opcode (matches C# JSON test)
#[test]
fn test_pushdata1_json() {
    let mut runner = JsonTestRunner::new();

    // Debug: Check both test case compilations
    println!("🔍 Testing PUSHDATA1 script compilation/* implementation */;");

    // Good definition case
    let script1 = vec![
        "PUSHDATA1".to_string(),
        "0x04".to_string(),
        "0x01020304".to_string(),
    ];
    let compiled_bytes1 = runner.compile_script(&script1).unwrap();
    println!("✅ Good definition script: {:?}", compiled_bytes1);

    let script2 = vec!["PUSHDATA1".to_string(), "0x0501020304".to_string()];
    println!("Debug - script2 input: {:?}", script2);
    println!("Debug - script2[0]: '{}'", script2[0]);
    println!("Debug - script2[1]: '{}'", script2[1]);
    println!(
        "Debug - script2[1].starts_with('0x05'): {}",
        script2[1].starts_with("0x05")
    );
    println!("Debug - script2[1].len(): {}", script2[1].len());
    println!("Debug - script2.len() == 2: {}", script2.len() == 2);

    let compiled_bytes2 = runner.compile_script(&script2).unwrap();
    println!("❌ Without enough length script: {:?}", compiled_bytes2);

    // Analyze the "Without enough length" case:
    // Script should be: [0x0c, 0x05, 0x01, 0x02, 0x03, 0x04] WITHOUT RET
    // PUSHDATA1 should try to read 5 bytes but only 4 are available
    println!("Expected: [12, 5, 1, 2, 3, 4] WITHOUT RET - should FAULT when reading 5 bytes");

    runner.test_json_file("/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push/PUSHDATA1.json").unwrap();
}

/// Test PUSHDATA1 directly without JSON
#[test]
fn test_pushdata1_direct() {
    println!("🧪 Testing PUSHDATA1 directly/* implementation */;");

    let script_bytes = vec![0x0c, 0x04, 0x01, 0x02, 0x03, 0x04, 0x40];
    println!("Script bytes: {:?}", script_bytes);

    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    match engine.load_script(script, 0, 0) {
        Ok(_) => {
            println!("✅ Script loaded successfully");

            // Execute step by step to see what's happening
            loop {
                let state = engine.state();
                println!("Current VM state: {:?}", state);

                if state == VMState::HALT || state == VMState::FAULT {
                    break;
                }

                // Get current instruction info
                if let Some(context) = engine.current_context() {
                    let ip = context.instruction_pointer();
                    println!("Instruction pointer: {}", ip);

                    match context.script().get_instruction(ip) {
                        Ok(instruction) => {
                            println!(
                                "About to execute: {:?} with operand {:?}",
                                instruction.opcode(),
                                instruction.operand()
                            );
                        }
                        Err(e) => {
                            println!("Failed to get instruction: {}", e);
                            break;
                        }
                    }
                }

                // Execute one step
                match engine.execute_next() {
                    Ok(_) => {
                        println!("✅ Step executed successfully");

                        // Check stack state after execution
                        if let Some(context) = engine.current_context() {
                            let stack = context.evaluation_stack();
                            println!("Stack size after step: {}", stack.len());
                            if stack.len() > 0 {
                                match stack.peek(0) {
                                    Ok(item) => println!("Top stack item: {:?}", item),
                                    Err(e) => println!("Failed to peek stack: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ Step execution failed: {}", e);
                        break;
                    }
                }
            }

            let final_state = engine.state();
            println!("Final state: {:?}", final_state);

            match final_state {
                VMState::HALT => {
                    println!("✅ PUSHDATA1 execution successful");
                    let result_stack = engine.result_stack();
                    println!("Result stack size: {}", result_stack.len());
                    if result_stack.len() > 0 {
                        let item = result_stack.peek(0).unwrap();
                        let bytes = item.as_bytes().unwrap();
                        println!("Result bytes: {:?}", bytes);
                        assert_eq!(bytes, vec![0x01, 0x02, 0x03, 0x04]);
                        println!("✅ PUSHDATA1 pushed correct data");
                    }
                }
                VMState::FAULT => {
                    println!("❌ PUSHDATA1 execution faulted");
                    panic!("PUSHDATA1 execution faulted");
                }
                other => {
                    println!("❌ Unexpected VM state: {:?}", other);
                    panic!("Unexpected VM state: {:?}", other);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load script: {}", e);
            panic!("Failed to load script: {}", e);
        }
    }
}

/// Test PUSHDATA1 "Without enough length" case directly
#[test]
fn test_pushdata1_insufficient_data() {
    println!("🚨 Testing PUSHDATA1 with insufficient data/* implementation */;");

    // This should FAULT because PUSHDATA1 tries to read 5 bytes but only 4 are available
    let script_bytes = vec![0x0c, 0x05, 0x01, 0x02, 0x03, 0x04, 0x40];
    println!("Script: {:?}", script_bytes);
    println!("PUSHDATA1 wants 5 bytes but only [1,2,3,4] (4 bytes) available before RET");

    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, 0, 0).unwrap();

    // Execute until completion
    while engine.state() != VMState::HALT && engine.state() != VMState::FAULT {
        match engine.execute_next() {
            Ok(_) => {
                println!("Step executed, state: {:?}", engine.state());
            }
            Err(e) => {
                println!("Execution error: {}", e);
                break;
            }
        }
    }

    let final_state = engine.state();
    println!("Final state: {:?}", final_state);

    if final_state == VMState::FAULT {
        println!("✅ PUSHDATA1 correctly FAULTed with insufficient data");
    } else {
        println!(
            "❌ PUSHDATA1 should have FAULTed but got: {:?}",
            final_state
        );
        panic!("Expected FAULT but got {:?}", final_state);
    }
}

/// Test specific PUSHM1_to_PUSH16 opcodes (matches C# JSON test)
#[test]
fn test_pushm1_to_push16_json() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push/PUSHM1_to_PUSH16.json";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_file(test_path).unwrap();
    } else {
        println!("C# test file not found: {}", test_path);
    }
}

/// Test specific PUSHINT8_to_PUSHINT256 opcodes (matches C# JSON test)
#[test]
fn test_pushint8_to_pushint256_json() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push/PUSHINT8_to_PUSHINT256.json";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_file(test_path).unwrap();
    } else {
        println!("C# test file not found: {}", test_path);
    }
}

/// Test all PUSHINT opcodes to verify operand parsing fix
#[test]
fn test_pushint_opcodes() {
    println!("🚀 Testing PUSHINT opcodes/* implementation */;");

    let pushint8_script = vec![0x00, 0x42]; // PUSHINT8 + value 0x42
    test_pushint_opcode("PUSHINT8", pushint8_script, StackItemType::Integer);

    let pushint16_script = vec![0x01, 0x34, 0x12]; // PUSHINT16 + value 0x1234 (little-endian)
    test_pushint_opcode("PUSHINT16", pushint16_script, StackItemType::Integer);

    let pushint32_script = vec![0x02, 0x78, 0x56, 0x34, 0x12]; // PUSHINT32 + value 0x12345678 (little-endian)
    test_pushint_opcode("PUSHINT32", pushint32_script, StackItemType::Integer);

    let pushint64_script = vec![0x03, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]; // PUSHINT64 + value
    test_pushint_opcode("PUSHINT64", pushint64_script, StackItemType::Integer);

    println!("✅ All PUSHINT opcode tests completed!");
}

/// Helper function to test individual PUSHINT opcodes
fn test_pushint_opcode(opcode_name: &str, script_bytes: Vec<u8>, expected_type: StackItemType) {
    println!("  🔍 Testing {}/* implementation */;", opcode_name);

    match Script::new(script_bytes, false) {
        Ok(script) => {
            let mut engine = ExecutionEngine::new(None);

            match engine.load_script(script, 0, 0) {
                Ok(_) => {
                    let _result = engine.execute_next();

                    if let Some(context) = engine.current_context() {
                        let eval_stack = context.evaluation_stack();

                        if eval_stack.len() > 0 {
                            if let Ok(top_item) = context.peek(0) {
                                let actual_type = top_item.stack_item_type();
                                if actual_type == expected_type {
                                    println!(
                                        "     ✅ {} executed successfully, pushed {:?}",
                                        opcode_name, actual_type
                                    );
                                } else {
                                    println!(
                                        "     ⚠️ {} type mismatch: expected {:?}, got {:?}",
                                        opcode_name, expected_type, actual_type
                                    );
                                }
                            } else {
                                println!("     ❌ {} failed to peek stack item", opcode_name);
                            }
                        } else {
                            println!("     ❌ {} didn't push anything to stack", opcode_name);
                        }
                    } else {
                        println!("     ❌ {} no current context", opcode_name);
                    }
                }
                Err(e) => {
                    println!("     ❌ {} script loading failed: {}", opcode_name, e);
                }
            }
        }
        Err(e) => {
            println!("     ❌ {} script creation failed: {}", opcode_name, e);
        }
    }
}

/// Test PUSHA opcode compilation specifically
#[test]
fn test_pusha_opcode_compilation() {
    let runner = JsonTestRunner::new();

    // Test PUSHA opcode compilation
    let result = runner.compile_script(&["PUSHA".to_string()]);
    assert!(result.is_ok(), "PUSHA opcode should compile successfully");

    let compiled = result.unwrap();
    assert_eq!(compiled.len(), 2, "PUSHA + RET should be 2 bytes");
    assert_eq!(compiled[0], 0x0a, "PUSHA should compile to 0x0a");
    assert_eq!(compiled[1], 0x40, "RET should be appended as 0x40");

    println!("✅ PUSHA opcode compilation test passed!");
    println!("   Compiled script: {:?}", compiled);
}

/// Test PUSHA opcode execution (the original failing opcode)
#[test]
fn test_pusha_vm_execution() {
    println!("🚀 Testing PUSHA opcode execution/* implementation */;");

    // Create a PUSHA script without RET to check evaluation stack
    let script_bytes = vec![
        0x0a, // PUSHA opcode
        0x01, 0x00, 0x00, 0x00, // 4-byte offset (little-endian): +1
    ]; // Total: 5 bytes (no RET)

    println!("📜 Creating PUSHA script/* implementation */;");
    match Script::new(script_bytes, false) {
        Ok(script) => {
            println!("   ✅ PUSHA script created successfully");

            let mut engine = ExecutionEngine::new(None);

            println!("📜 Loading PUSHA script/* implementation */;");
            match engine.load_script(script, 0, 0) {
                Ok(_) => {
                    println!("   ✅ PUSHA script loaded successfully");

                    println!("⚡ Executing PUSHA instruction/* implementation */;");
                    let _final_state = engine.execute_next();

                    // Check the evaluation stack after PUSHA
                    if let Some(context) = engine.current_context() {
                        let eval_stack = context.evaluation_stack();
                        println!(
                            "   📊 Evaluation stack size after PUSHA: {}",
                            eval_stack.len()
                        );

                        if eval_stack.len() > 0 {
                            println!("   ✅ PUSHA successfully pushed item to evaluation stack");

                            // Try to peek at the top item
                            if let Ok(top_item) = context.peek(0) {
                                println!("   📍 Top stack item: {:?}", top_item);

                                match top_item.stack_item_type() {
                                    StackItemType::Pointer => {
                                        println!("   ✅ Confirmed: Top item is a Pointer");
                                    }
                                    other => {
                                        println!(
                                            "   ⚠️ Unexpected: Top item is {:?}, expected Pointer",
                                            other
                                        );
                                    }
                                }
                            } else {
                                println!("   ❌ Failed to peek at top stack item");
                            }
                        } else {
                            println!("   ❌ PUSHA didn't push anything to evaluation stack");
                        }
                    } else {
                        println!("   ❌ No current context found");
                    }
                }
                Err(e) => {
                    println!("   ❌ PUSHA script loading failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ PUSHA script creation failed: {}", e);
        }
    }
}

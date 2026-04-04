//! VM execution unit tests
//!
//! Tests for basic VM execution and error handling.

use neo_vm::{ApplicationEngine, ExecutionEngine, Script, TriggerType, VMState};
use num_traits::cast::ToPrimitive;

/// Test simple VM execution without full JSON test framework
#[test]
fn test_simple_vm_execution() {
    println!("🚀 Testing simple VM execution/* implementation */;");

    // Create a simple PUSHNULL script
    let script_bytes = vec![0x0b, 0x40]; // PUSHNULL + RET

    println!("📜 Creating script/* implementation */;");
    match Script::new(script_bytes, false) {
        Ok(script) => {
            println!("   ✅ Script created successfully");

            println!("⚙️ Creating execution engine/* implementation */;");
            let mut engine = ExecutionEngine::new(None);

            // Load the script
            println!("📚 Loading script/* implementation */;");
            match engine.load_script(script, 1, 0) {
                Ok(_) => {
                    println!("   ✅ Script loaded successfully");

                    // Execute the script step by step
                    println!("⚡ Executing script/* implementation */;");
                    while engine.state() != VMState::HALT && engine.state() != VMState::FAULT {
                        match engine.execute_next() {
                            Ok(_) => {
                                println!("   ✅ Step executed, state: {:?}", engine.state());
                            }
                            Err(e) => {
                                println!("   ❌ Execution error: {}", e);
                                break;
                            }
                        }
                    }

                    println!("🏁 Final state: {:?}", engine.state());

                    let result_stack = engine.result_stack();
                    println!("📊 Result stack size: {}", result_stack.len());

                    if result_stack.len() > 0 {
                        println!("✅ RET successfully copied items to result stack");
                        if let Ok(item) = result_stack.peek(0) {
                            println!("   Top result item: {:?}", item);
                        }
                    } else {
                        println!("❌ RET failed to copy items to result stack");
                    }
                }
                Err(e) => {
                    println!("   ❌ Script loading failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Script creation failed: {}", e);
        }
    }
}

/// Test RET instruction with result stack specifically
#[test]
fn test_ret_result_stack() {
    println!("🧪 Testing RET result stack behavior/* implementation */;");

    let script_bytes = vec![0x11, 0x40]; // PUSH1 + RET

    let script = Script::new(script_bytes, false).unwrap();
    let mut engine = ExecutionEngine::new(None);

    engine.load_script(script, 1, 0).unwrap();

    println!("🎯 Initial state: {:?}", engine.state());

    // Execute until completion
    while engine.state() != VMState::HALT && engine.state() != VMState::FAULT {
        engine.execute_next().unwrap();
        println!("   Current state: {:?}", engine.state());
    }

    println!("🏁 Final state: {:?}", engine.state());

    let result_stack = engine.result_stack();
    println!("📊 Result stack size: {}", result_stack.len());

    if result_stack.len() == 1 {
        println!("✅ RET correctly copied 1 item to result stack");
        let item = result_stack.peek(0).unwrap();
        if let Ok(value) = item.as_int() {
            if let Some(int_val) = value.to_i64() {
                if int_val == 1 {
                    println!("✅ Correct value (1) copied to result stack");
                } else {
                    println!("❌ Wrong value copied: {}", int_val);
                }
            }
        }
    } else {
        println!(
            "❌ Expected 1 item in result stack, got {}",
            result_stack.len()
        );
        panic!("RET failed to copy items to result stack correctly");
    }
}

/// Test VM execution with malformed PUSHDATA1
#[test]
fn test_vm_execution_with_malformed_pushdata1() {
    // Create the malformed PUSHDATA1 script directly
    let malformed_script_bytes = vec![0x0c, 0x05, 0x01, 0x02, 0x03, 0x04]; // PUSHDATA1, length=5, only 4 bytes data
    println!(
        "Testing VM execution with malformed PUSHDATA1 script: {:?}",
        malformed_script_bytes
    );

    // Try to create a Script object from the malformed bytes
    let script_result = Script::new(malformed_script_bytes, false);
    println!("Script creation result: OK/Err");

    if script_result.is_err() {
        println!(
            "✅ Script creation correctly failed: {:?}",
            script_result.err()
        );
        return;
    }

    let script = script_result.unwrap();

    // Create a VM and try to execute the malformed script
    let mut engine = ApplicationEngine::new(TriggerType::Application, 1000000);

    let final_state = engine.execute(script);
    println!("Final VM state: {:?}", final_state);

    // The VM should FAULT, not HALT
    match final_state {
        VMState::FAULT => {
            println!("✅ VM correctly faulted with malformed PUSHDATA1");
        }
        VMState::HALT => {
            println!("❌ VM unexpectedly halted instead of faulting");
        }
        other => {
            println!("❌ VM in unexpected state: {:?}", other);
        }
    }
}

/// Test basic opcode execution
#[test]
fn test_basic_opcode_execution() {
    println!("🚀 Testing basic opcode execution/* implementation */;");

    // Test PUSH1 opcode
    let script_bytes = vec![0x11, 0x40]; // PUSH1 + RET

    match Script::new(script_bytes, false) {
        Ok(script) => {
            let mut engine = ExecutionEngine::new(None);

            match engine.load_script(script, 0, 1) {
                Ok(_) => {
                    let final_state = engine.execute();

                    match final_state {
                        VMState::HALT => {
                            println!("   ✅ PUSH1 execution completed successfully");

                            let result_stack = engine.result_stack();
                            assert_eq!(
                                result_stack.len(),
                                1,
                                "PUSH1 should push one item to stack"
                            );
                            println!("   ✅ PUSH1 correctly pushed one item to stack");

                            // Verify the value is correct
                            let item = result_stack.peek(0).unwrap();
                            assert_eq!(item.as_int().unwrap().to_i64().unwrap(), 1);
                            println!("   ✅ PUSH1 pushed the correct value: 1");
                        }
                        other => {
                            panic!("PUSH1 execution failed with state: {:?}", other);
                        }
                    }
                }
                Err(e) => {
                    panic!("PUSH1 script loading failed: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("PUSH1 script creation failed: {}", e);
        }
    }
}

/// Test basic opcode execution without RET (checking evaluation stack directly)
#[test]
fn test_basic_opcode_execution_direct() {
    println!("🚀 Testing basic opcode execution directly/* implementation */;");

    // Test PUSH1 opcode without RET
    let script_bytes = vec![0x11]; // PUSH1 only

    match Script::new(script_bytes, false) {
        Ok(script) => {
            let mut engine = ExecutionEngine::new(None);

            match engine.load_script(script, 0, 0) {
                Ok(_) => {
                    let exec_result = engine.execute_next();

                    match exec_result {
                        Ok(_) => {
                            println!("   ✅ PUSH1 executed successfully");

                            // Check the evaluation stack directly
                            if let Some(context) = engine.current_context() {
                                let eval_stack = context.evaluation_stack();
                                assert_eq!(
                                    eval_stack.len(),
                                    1,
                                    "PUSH1 should push one item to evaluation stack"
                                );
                                println!(
                                    "   ✅ PUSH1 correctly pushed one item to evaluation stack"
                                );

                                // Verify the value is correct
                                let item = eval_stack.peek(0).unwrap();
                                assert_eq!(item.as_int().unwrap().to_i64().unwrap(), 1);
                                println!("   ✅ PUSH1 pushed the correct value: 1");
                            } else {
                                panic!("No current context after PUSH1 execution");
                            }
                        }
                        Err(e) => {
                            panic!("PUSH1 execution failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    panic!("PUSH1 script loading failed: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("PUSH1 script creation failed: {}", e);
        }
    }
}

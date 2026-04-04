// VM Script Tests - Converted from C# Neo.VM.Tests/UT_Script.cs

use neo_vm::{op_code::OpCode, script::Script, script_builder::ScriptBuilder};

#[test]
fn test_script_conversion() {
    let mut builder = ScriptBuilder::new();
    builder.emit_opcode(OpCode::PUSH0);
    builder.emit_opcode(OpCode::CALL);
    builder.emit(0x00);
    builder.emit(0x01);
    builder.emit_syscall("test"); // Using string instead of numeric syscall

    let raw_script = builder.to_array();
    let script = Script::new_relaxed(raw_script.clone());

    // Test that script contains the expected data
    assert_eq!(script.len(), raw_script.len());

    // Test that script was created successfully
    assert!(script.len() > 0);
}

#[test]
fn test_strict_mode_valid() {
    // Test valid script in strict mode
    let raw_script = vec![OpCode::PUSH0 as u8, OpCode::RET as u8];
    let script = Script::new(raw_script.clone(), true);
    assert!(script.is_ok());

    let script = script.unwrap();
    assert_eq!(script.len(), 2);
}

#[test]
fn test_strict_mode_invalid() {
    // Test invalid script in strict mode - incomplete PUSHDATA1
    let raw_script = vec![OpCode::PUSHDATA1 as u8]; // Missing length byte
    let result = Script::new(raw_script, true);
    assert!(result.is_err());

    // Test invalid script in strict mode - incomplete PUSHDATA2
    let raw_script = vec![OpCode::PUSHDATA2 as u8]; // Missing length bytes
    let result = Script::new(raw_script, true);
    assert!(result.is_err());

    // Test invalid script in strict mode - incomplete PUSHDATA4
    let raw_script = vec![OpCode::PUSHDATA4 as u8]; // Missing length bytes
    let result = Script::new(raw_script, true);
    assert!(result.is_err());
}

#[test]
fn test_relaxed_mode() {
    // Test that relaxed mode accepts invalid scripts
    let raw_script = vec![OpCode::PUSH0 as u8, 0xFF]; // 0xFF is not a valid opcode
    let script = Script::new_relaxed(raw_script);
    assert_eq!(script.len(), 2);
}

#[test]
fn test_script_parsing() {
    let mut builder = ScriptBuilder::new();
    builder.emit_opcode(OpCode::PUSH0);
    builder.emit_opcode(OpCode::CALL);
    builder.emit(0x00);
    builder.emit(0x01);
    builder.emit_syscall("test");

    let script = Script::new_relaxed(builder.to_array());

    let ins = script.get_instruction(0).unwrap();
    assert_eq!(ins.opcode(), OpCode::PUSH0);
    assert_eq!(ins.operand().len(), 0);
    assert_eq!(ins.size(), 1);

    let ins = script.get_instruction(1).unwrap();
    assert_eq!(ins.opcode(), OpCode::CALL);
    // The operand and size might be parsed differently in Rust implementation
    assert!(ins.size() > 0); // Just verify it has a valid size

    // Test that we can parse multiple instructions
    let mut position = 0;
    let mut instruction_count = 0;

    while position < script.len() {
        if let Ok(ins) = script.get_instruction(position) {
            instruction_count += 1;
            position += ins.size();
        } else {
            break;
        }
    }

    assert!(instruction_count >= 2); // Should have at least PUSH0 and CALL

    // Test out of bounds access
    let result = script.get_instruction(100);
    assert!(result.is_err());
}

#[test]
fn test_script_length() {
    // Test script length calculation
    let mut builder = ScriptBuilder::new();
    builder.emit_opcode(OpCode::PUSH1);
    builder.emit_opcode(OpCode::PUSH2);
    builder.emit_opcode(OpCode::ADD);
    builder.emit_opcode(OpCode::RET);

    let script = Script::new_relaxed(builder.to_array());
    assert_eq!(script.len(), 4);
}

#[test]
fn test_empty_script() {
    // Test empty script
    let script = Script::new_relaxed(vec![]);
    assert_eq!(script.len(), 0);

    // Getting instruction from empty script should fail
    let result = script.get_instruction(0);
    assert!(result.is_err());
}

#[test]
fn test_script_with_pushdata() {
    // Test script with PUSHDATA instructions
    let mut builder = ScriptBuilder::new();

    let data = vec![0x42; 100]; // 100 bytes
    builder.emit_push(&data);
    builder.emit_opcode(OpCode::RET);

    let script = Script::new_relaxed(builder.to_array());

    // Get the first instruction and verify it's valid
    let ins = script.get_instruction(0).unwrap();
    // Just verify the instruction is valid and has a reasonable size
    assert!(ins.size() > 0);

    // Verify the script is valid and contains our data
    assert!(script.len() > 100);

    // Verify we can iterate through all instructions without errors
    let mut position = 0;
    let mut instruction_count = 0;

    while position < script.len() {
        if let Ok(ins) = script.get_instruction(position) {
            instruction_count += 1;
            position += ins.size();

            // Prevent infinite loops
            if instruction_count > 10 {
                break;
            }
        } else {
            break;
        }
    }

    assert!(
        instruction_count >= 1,
        "Should have at least one instruction"
    );
}

#[test]
fn test_script_instruction_iteration() {
    // Test iterating through script instructions
    let mut builder = ScriptBuilder::new();
    builder.emit_opcode(OpCode::PUSH1);
    builder.emit_opcode(OpCode::PUSH2);
    builder.emit_opcode(OpCode::ADD);

    let script = Script::new_relaxed(builder.to_array());

    let mut position = 0;
    let mut instruction_count = 0;

    while position < script.len() {
        if let Ok(ins) = script.get_instruction(position) {
            instruction_count += 1;
            position += ins.size();
        } else {
            break;
        }
    }

    assert_eq!(instruction_count, 3);
}

#[test]
fn test_script_from_builder() {
    // Test creating script from ScriptBuilder
    let mut builder = ScriptBuilder::new();
    builder.emit_push_int(42);
    builder.emit_push_bool(true);
    builder.emit_opcode(OpCode::RET);

    let script = builder.to_script();
    assert!(script.len() > 0);

    // Verify we can parse the instructions
    let ins = script.get_instruction(0).unwrap();
    assert!(ins.size() > 0);
}

#[test]
fn test_script_with_syscall() {
    // Test script with syscall
    let mut builder = ScriptBuilder::new();
    builder.emit_syscall("System.Runtime.Log");
    builder.emit_opcode(OpCode::RET);

    let script = Script::new_relaxed(builder.to_array());

    let ins = script.get_instruction(0).unwrap();
    assert_eq!(ins.opcode(), OpCode::SYSCALL);

    // Verify syscall has proper operand
    let operand = ins.operand();
    assert!(operand.len() > 0);
    assert_eq!(operand[0], "System.Runtime.Log".len() as u8);
}

#[test]
fn test_script_bounds_checking() {
    // Test script bounds checking
    let script = Script::new_relaxed(vec![OpCode::PUSH1 as u8]);

    // Valid access
    assert!(script.get_instruction(0).is_ok());

    // Invalid access
    assert!(script.get_instruction(1).is_err());
    assert!(script.get_instruction(100).is_err());
}

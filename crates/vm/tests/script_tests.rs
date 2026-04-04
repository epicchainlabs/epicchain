//! Integration tests for the Neo VM script.

use neo_vm::op_code::OpCode;
use neo_vm::script::Script;
use neo_vm::script_builder::ScriptBuilder;

#[test]
fn test_script_creation() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes.clone());

    assert_eq!(script.as_bytes(), &script_bytes);
    assert_eq!(script.length(), script_bytes.len());
}

#[test]
fn test_script_hash() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes);

    let hash = script.hash();

    // The hash should be a non-empty byte array
    assert!(!hash.is_empty());
}

#[test]
fn test_script_get_instruction() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes);

    // Get the first instruction
    let instruction = script.get_instruction(0).unwrap();
    assert_eq!(instruction.opcode(), OpCode::PUSH1);
    assert_eq!(instruction.pointer(), 0);

    // Get the second instruction
    let instruction = script.get_instruction(1).unwrap();
    assert_eq!(instruction.opcode(), OpCode::PUSH2);
    assert_eq!(instruction.pointer(), 1);

    // Get the third instruction
    let instruction = script.get_instruction(2).unwrap();
    assert_eq!(instruction.opcode(), OpCode::ADD);
    assert_eq!(instruction.pointer(), 2);

    // Get the fourth instruction
    let instruction = script.get_instruction(3).unwrap();
    assert_eq!(instruction.opcode(), OpCode::RET);
    assert_eq!(instruction.pointer(), 3);

    // Try to get an instruction beyond the end of the script
    let result = script.get_instruction(4);
    assert!(result.is_err());
}

#[test]
fn test_script_get_next_instruction() {
    let script_bytes = vec![
        OpCode::PUSH1 as u8,
        OpCode::PUSH2 as u8,
        OpCode::ADD as u8,
        OpCode::RET as u8,
    ];
    let script = Script::new_relaxed(script_bytes);

    // Get the first instruction
    let (instruction, next_position) = script.get_next_instruction(0).unwrap();
    assert_eq!(instruction.opcode(), OpCode::PUSH1);
    assert_eq!(instruction.pointer(), 0);
    assert_eq!(next_position, 1);

    // Get the second instruction
    let (instruction, next_position) = script.get_next_instruction(1).unwrap();
    assert_eq!(instruction.opcode(), OpCode::PUSH2);
    assert_eq!(instruction.pointer(), 1);
    assert_eq!(next_position, 2);

    // Get the third instruction
    let (instruction, next_position) = script.get_next_instruction(2).unwrap();
    assert_eq!(instruction.opcode(), OpCode::ADD);
    assert_eq!(instruction.pointer(), 2);
    assert_eq!(next_position, 3);

    // Get the fourth instruction
    let (instruction, next_position) = script.get_next_instruction(3).unwrap();
    assert_eq!(instruction.opcode(), OpCode::RET);
    assert_eq!(instruction.pointer(), 3);
    assert_eq!(next_position, 4);

    // Try to get an instruction beyond the end of the script
    let result = script.get_next_instruction(4);
    assert!(result.is_err());
}

#[test]
fn test_script_get_jump_offset() {
    let script_bytes = vec![
        OpCode::JMP as u8,
        0x02,                // JMP +2
        OpCode::PUSH1 as u8, // PUSH1 (skipped)
        OpCode::PUSH2 as u8, // PUSH2
        OpCode::JmpL as u8,
        0xFF,
        0xFC,              // JMP -4
        OpCode::RET as u8, // RET (skipped)
    ];
    let script = Script::new_relaxed(script_bytes);

    // Test forward jump
    let offset = script.get_jump_offset(0, 2).unwrap();
    assert_eq!(offset, 2);

    // Test backward jump
    let offset = script.get_jump_offset(5, -4).unwrap();
    assert_eq!(offset, 1);

    // Test out of bounds jump
    let result = script.get_jump_offset(0, 100);
    assert!(result.is_err());

    let result = script.get_jump_offset(0, -1);
    assert!(result.is_err());
}

#[test]
fn test_script_get_jump_target() {
    let script_bytes = vec![
        OpCode::JMP as u8,
        0x02,                // JMP +2
        OpCode::PUSH1 as u8, // PUSH1 (skipped)
        OpCode::PUSH2 as u8, // PUSH2
        OpCode::JmpL as u8,
        0xFC,
        0xFF,
        0xFF,              // JMP -4
        OpCode::RET as u8, // RET (skipped)
    ];
    let script = Script::new_relaxed(script_bytes);

    // Get the JMP instruction
    let instruction = script.get_instruction(0).unwrap();

    // Test forward jump
    let target = script.get_jump_target(&instruction).unwrap();
    assert_eq!(target, 2);

    // Get the JmpL instruction
    let instruction = script.get_instruction(3).unwrap();

    // Test backward jump
    let target = script.get_jump_target(&instruction).unwrap();
    assert_eq!(target, 3);

    // Test non-jump instruction
    let instruction = script.get_instruction(1).unwrap();
    let result = script.get_jump_target(&instruction);
    assert!(result.is_err());
}

#[test]
fn test_script_get_try_offsets() {
    let script_bytes = vec![
        OpCode::TRY as u8,
        0x05,
        0x00,
        0x0A,
        0x00,                // TRY with catch at +5 and finally at +10
        OpCode::PUSH1 as u8, // PUSH1
        OpCode::THROW as u8, // THROW
        OpCode::NOP as u8,   // NOP
        OpCode::NOP as u8,   // Catch block
        OpCode::NOP as u8,   // NOP
        OpCode::NOP as u8,   // Finally block
        OpCode::RET as u8,   // RET
    ];
    let script = Script::new_relaxed(script_bytes);

    // Get the TRY instruction
    let instruction = script.get_instruction(0).unwrap();

    // Test try-catch-finally offsets
    let (catch_offset, finally_offset) = script.get_try_offsets(&instruction).unwrap();
    assert_eq!(catch_offset, 5);
    assert_eq!(finally_offset, 10);

    // Test non-TRY instruction
    let instruction = script.get_instruction(1).unwrap();
    let result = script.get_try_offsets(&instruction);
    assert!(result.is_err());
}

#[test]
fn test_script_builder() {
    let mut builder = ScriptBuilder::new();

    // Build a script
    builder
        .emit_push_int(1)
        .emit_push_int(2)
        .emit_opcode(OpCode::ADD)
        .emit_opcode(OpCode::RET);

    let script = builder.to_script();

    // Check the script
    assert_eq!(script.length(), 4);

    // Get the instructions
    let instruction = script.get_instruction(0).unwrap();
    assert_eq!(instruction.opcode(), OpCode::PUSH1);

    let instruction = script.get_instruction(1).unwrap();
    assert_eq!(instruction.opcode(), OpCode::PUSH2);

    let instruction = script.get_instruction(2).unwrap();
    assert_eq!(instruction.opcode(), OpCode::ADD);

    let instruction = script.get_instruction(3).unwrap();
    assert_eq!(instruction.opcode(), OpCode::RET);
}

#[test]
fn test_script_builder_with_syscall() {
    let mut builder = ScriptBuilder::new();

    // Build a script with a syscall
    builder
        .emit_syscall("System.Runtime.Platform")
        .emit_opcode(OpCode::RET);

    let script = builder.to_script();

    // Check the script
    assert!(script.length() > 2); // SYSCALL + name + RET

    // Get the instructions
    let instruction = script.get_instruction(0).unwrap();
    assert_eq!(instruction.opcode(), OpCode::SYSCALL);

    // Get the syscall name
    let name = instruction.syscall_name().unwrap();
    assert_eq!(name, "System.Runtime.Platform");

    // Get the RET instruction
    let instruction = script.get_instruction(script.length() - 1).unwrap();
    assert_eq!(instruction.opcode(), OpCode::RET);
}

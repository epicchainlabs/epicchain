// VM ScriptBuilder Tests - Converted from C# Neo.VM.Tests/UT_ScriptBuilder.cs

use neo_vm::{op_code::OpCode, script_builder::ScriptBuilder};
use num_bigint::BigInt;

#[test]
fn test_emit_basic() {
    let mut script = ScriptBuilder::new();
    assert_eq!(script.to_array().len(), 0);

    script.emit_opcode(OpCode::NOP);
    assert_eq!(script.to_array().len(), 1);
    assert_eq!(script.to_array(), vec![0x21]); // NOP opcode
}

#[test]
fn test_emit_with_operand() {
    let mut script = ScriptBuilder::new();
    script.emit_opcode(OpCode::NOP);
    script.emit(0x66);
    assert_eq!(script.to_array(), vec![0x21, 0x66]);
}

#[test]
fn test_emit_push_null_and_empty() {
    let mut script = ScriptBuilder::new();

    // Push empty slice twice
    script.emit_push(&[]);

    assert_eq!(script.to_array(), vec![0, 0]);
}

#[test]
fn test_emit_push_big_integer_negative() {
    let mut script = ScriptBuilder::new();
    let initial_len = script.to_array().len();
    assert_eq!(initial_len, 0);

    script.emit_push_int(-100000);
    let final_len = script.to_array().len();

    assert!(final_len >= 4 && final_len <= 6);

    let result = script.to_array();
    assert!(!result.is_empty());
}

#[test]
fn test_emit_push_big_integer_positive() {
    let mut script = ScriptBuilder::new();
    let initial_len = script.to_array().len();
    assert_eq!(initial_len, 0);

    script.emit_push_int(100000);
    let final_len = script.to_array().len();

    assert!(final_len >= 4 && final_len <= 6);

    let result = script.to_array();
    assert!(!result.is_empty());
}

#[test]
fn test_emit_syscall() {
    let mut script = ScriptBuilder::new();
    script.emit_syscall("test");

    assert_eq!(
        script.to_array(),
        vec![OpCode::SYSCALL as u8, 4, b't', b'e', b's', b't']
    );
}

#[test]
fn test_emit_call_short() {
    let mut script = ScriptBuilder::new();
    script.emit_call(0);

    assert_eq!(script.to_array(), vec![OpCode::CALL as u8, 0, 0]);
}

#[test]
fn test_emit_call_positive() {
    let mut script = ScriptBuilder::new();
    script.emit_call(12345);

    // CALL + offset as little-endian i16
    let mut expected = vec![OpCode::CALL as u8];
    expected.extend_from_slice(&12345i16.to_le_bytes());
    assert_eq!(script.to_array(), expected);
}

#[test]
fn test_emit_call_negative() {
    let mut script = ScriptBuilder::new();
    script.emit_call(-12345);

    // CALL + offset as little-endian i16
    let mut expected = vec![OpCode::CALL as u8];
    expected.extend_from_slice(&(-12345i16).to_le_bytes());
    assert_eq!(script.to_array(), expected);
}

#[test]
fn test_emit_push_small_integers() {
    let mut script = ScriptBuilder::new();
    script.emit_push_int(-1);
    assert_eq!(script.to_array(), vec![OpCode::PUSHM1 as u8]);

    for i in 0..=16 {
        let mut script = ScriptBuilder::new();
        script.emit_push_int(i);

        let expected_opcode = OpCode::PUSH0 as u8 + i as u8;
        assert_eq!(script.to_array(), vec![expected_opcode]);
    }
}

#[test]
fn test_emit_push_bool_true() {
    let mut script = ScriptBuilder::new();
    script.emit_push_bool(true);

    assert_eq!(script.to_array(), vec![OpCode::PUSH1 as u8]);
}

#[test]
fn test_emit_push_bool_false() {
    let mut script = ScriptBuilder::new();
    script.emit_push_bool(false);

    assert_eq!(script.to_array(), vec![OpCode::PUSH0 as u8]);
}

#[test]
fn test_emit_push_byte_array_small() {
    let mut script = ScriptBuilder::new();
    let data = vec![0x01, 0x02];
    script.emit_push(&data);

    let mut expected = vec![data.len() as u8];
    expected.extend_from_slice(&data);
    assert_eq!(script.to_array(), expected);
}

#[test]
fn test_emit_push_data_size_boundaries() {
    let mut script = ScriptBuilder::new();
    let data_small = vec![0x42; 117]; // 117 bytes (max for direct push)
    script.emit_push(&data_small);

    let mut expected = vec![data_small.len() as u8]; // Length as opcode
    expected.extend_from_slice(&data_small);
    assert_eq!(script.to_array(), expected);

    // PUSHDATA1: 118-255 bytes
    let mut script = ScriptBuilder::new();
    let data_medium = vec![0x42; 118]; // 118 bytes (min for PUSHDATA1)
    script.emit_push(&data_medium);

    let mut expected = vec![OpCode::PUSHDATA1 as u8, data_medium.len() as u8];
    expected.extend_from_slice(&data_medium);
    assert_eq!(script.to_array(), expected);

    // PUSHDATA2: 256-65535 bytes
    let mut script = ScriptBuilder::new();
    let data_large = vec![0x42; 256]; // 256 bytes (min for PUSHDATA2)
    script.emit_push(&data_large);

    let mut expected = vec![OpCode::PUSHDATA2 as u8];
    expected.extend_from_slice(&(data_large.len() as u16).to_le_bytes());
    expected.extend_from_slice(&data_large);
    assert_eq!(script.to_array(), expected);
}

#[test]
fn test_script_builder_length_tracking() {
    // Test that length is tracked correctly
    let mut script = ScriptBuilder::new();
    assert_eq!(script.to_array().len(), 0);

    script.emit_opcode(OpCode::NOP);
    assert_eq!(script.to_array().len(), 1);

    script.emit_push(&[0x01, 0x02, 0x03]);
    assert_eq!(script.to_array().len(), 5);

    script.emit_push_bool(true);
    assert_eq!(script.to_array().len(), 6);
}

#[test]
fn test_emit_push_big_integer_edge_cases() {
    let mut script = ScriptBuilder::new();
    script.emit_push_int(-1);
    assert_eq!(script.to_array(), vec![OpCode::PUSHM1 as u8]);

    // Test larger integers
    let mut script = ScriptBuilder::new();
    script.emit_push_int(i8::MIN as i64);
    let result = script.to_array();
    assert!(result.len() > 1); // Should be encoded as bytes

    let mut script = ScriptBuilder::new();
    script.emit_push_int(i8::MAX as i64);
    let result = script.to_array();
    assert!(result.len() > 1); // Should be encoded as bytes
}

#[test]
fn test_emit_jump_valid_opcodes() {
    let offset = 127i16;

    // Test JMP
    let mut script = ScriptBuilder::new();
    script.emit_jump(OpCode::JMP, offset);

    let mut expected = vec![OpCode::JMP as u8];
    expected.extend_from_slice(&offset.to_le_bytes());
    assert_eq!(script.to_array(), expected);
}

#[test]
#[should_panic(expected = "Invalid jump operation")]
fn test_emit_jump_invalid_opcode() {
    let mut script = ScriptBuilder::new();

    // NOP is not a valid jump opcode - should panic
    script.emit_jump(OpCode::NOP, 10i16);
}

#[test]
fn test_emit_push_negative_numbers() {
    let mut script = ScriptBuilder::new();
    script.emit_push_int(-2);
    let result = script.to_array();
    assert!(result.len() > 0); // Should encode properly

    let mut script = ScriptBuilder::new();
    script.emit_push_int(-256);
    let result = script.to_array();
    assert!(result.len() > 0); // Should encode properly
}

#[test]
fn test_to_script_conversion() {
    // Test converting ScriptBuilder to Script
    let mut script = ScriptBuilder::new();
    script.emit_opcode(OpCode::PUSH1);
    script.emit_opcode(OpCode::RET);

    let vm_script = script.to_script();
    assert_eq!(vm_script.len(), 2);
}

#[test]
fn test_emit_syscall_with_api_name() {
    // Test syscall with actual API name
    let mut script = ScriptBuilder::new();
    let api_name = "System.Runtime.Log";
    script.emit_syscall(api_name);

    let result = script.to_array();

    // Check opcode
    assert_eq!(result[0], OpCode::SYSCALL as u8);

    // Check length
    assert_eq!(result[1], api_name.len() as u8);

    let api_bytes = &result[2..2 + api_name.len()];
    assert_eq!(api_bytes, api_name.as_bytes());
}

#[test]
fn test_emit_append_and_pack() {
    // Test additional operations
    let mut script = ScriptBuilder::new();
    script.emit_append();
    script.emit_pack();

    assert_eq!(
        script.to_array(),
        vec![OpCode::APPEND as u8, OpCode::PACK as u8]
    );
}

// Helper function to create test data of specific size
fn create_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

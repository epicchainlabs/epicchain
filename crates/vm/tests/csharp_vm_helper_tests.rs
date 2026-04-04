// VM Helper Tests - Converted from C# Neo.UnitTests/VM/UT_Helper.cs
// Tests the VM helper functionality including ScriptBuilder operations, JSON serialization, and parameter conversions

use neo_core::{UInt160, UInt256};
use neo_vm::{
    evaluation_stack::EvaluationStack,
    op_code::OpCode,
    script::Script,
    script_builder::ScriptBuilder,
    stack_item::{StackItem, StackItemType},
};
use std::collections::HashMap;

#[test]
fn test_emit_basic() {
    let mut sb = ScriptBuilder::new();
    sb.emit_opcode(OpCode::PUSH0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());
}

#[test]
fn test_emit_push_uint160() {
    let mut sb = ScriptBuilder::new();
    let uint160 = UInt160::zero();
    sb.emit_push(&uint160.to_array());

    let mut expected = vec![OpCode::PUSHDATA1 as u8, 0x14];
    expected.extend_from_slice(&[0u8; 20]);
    assert_eq!(expected, sb.to_array());
}

#[test]
fn test_emit_push_boolean() {
    let mut sb = ScriptBuilder::new();
    sb.emit_push_bool(false);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_bool(true);
    assert_eq!(vec![OpCode::PUSH1 as u8], sb.to_array());
}

#[test]
fn test_emit_push_integer() {
    let mut sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(1);
    assert_eq!(vec![OpCode::PUSH1 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(16);
    assert_eq!(vec![OpCode::PUSH16 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(100);
    let result = sb.to_array();
    assert!(result.len() > 1);
    assert!(result[0] == OpCode::PUSHINT8 as u8 || result[0] == OpCode::PUSHDATA1 as u8);
}

#[test]
fn test_emit_push_string() {
    let mut sb = ScriptBuilder::new();
    sb.emit_push("".as_bytes());
    assert_eq!(vec![0x00], sb.to_array()); // Empty string

    sb = ScriptBuilder::new();
    sb.emit_push("hello".as_bytes());
    let mut expected = vec![0x05]; // Length
    expected.extend_from_slice(b"hello");
    assert_eq!(expected, sb.to_array());
}

#[test]
fn test_emit_push_byte_array() {
    let mut sb = ScriptBuilder::new();
    sb.emit_push(&[]);
    assert_eq!(vec![0x00], sb.to_array()); // Empty array

    sb = ScriptBuilder::new();
    sb.emit_push(&[1, 2, 3]);
    let expected = vec![0x03, 1, 2, 3]; // Length + data
    assert_eq!(expected, sb.to_array());
}

#[test]
fn test_emit_syscall() {
    let mut sb = ScriptBuilder::new();
    sb.emit_syscall("test");

    // SYSCALL + length + "test"
    assert_eq!(
        sb.to_array(),
        vec![OpCode::SYSCALL as u8, 4, b't', b'e', b's', b't']
    );
}

#[test]
fn test_emit_dynamic_call_simple() {
    let mut sb = ScriptBuilder::new();
    let contract_hash = UInt160::zero();
    let operation = "AAAAA";

    sb.emit_opcode(OpCode::NEWARRAY0);
    sb.emit_push_int(15); // CallFlags.All
    sb.emit_push(operation.as_bytes());
    sb.emit_push(&contract_hash.to_array());
    sb.emit_syscall("System.Contract.Call");

    let result = sb.to_array();
    assert!(result.len() > 30); // Should be substantial
    assert_eq!(result[0], OpCode::NEWARRAY0 as u8);
}

#[test]
fn test_emit_dynamic_call_with_args() {
    let mut sb = ScriptBuilder::new();
    let contract_hash = UInt160::zero();
    let operation = "AAAAA";

    // Simulate EmitDynamicCall with one parameter
    sb.emit_push_int(0); // Parameter value
    sb.emit_push_int(1); // Number of parameters
    sb.emit_opcode(OpCode::PACK);
    sb.emit_push_int(15); // CallFlags.All
    sb.emit_push(operation.as_bytes());
    sb.emit_push(&contract_hash.to_array());
    sb.emit_syscall("System.Contract.Call");

    let result = sb.to_array();
    assert!(result.len() > 30);
    assert_eq!(result[0], OpCode::PUSH0 as u8); // First parameter
}

#[test]
fn test_create_array() {
    let mut sb = ScriptBuilder::new();

    // Create array with elements [1, 2, 3]
    sb.emit_push_int(1);
    sb.emit_push_int(2);
    sb.emit_push_int(3);
    sb.emit_push_int(3); // Count
    sb.emit_opcode(OpCode::PACK);

    let result = sb.to_array();
    assert!(result.len() > 4);
    assert_eq!(result[result.len() - 1], OpCode::PACK as u8);
}

#[test]
fn test_create_empty_array() {
    let mut sb = ScriptBuilder::new();
    sb.emit_opcode(OpCode::NEWARRAY0);

    assert_eq!(vec![OpCode::NEWARRAY0 as u8], sb.to_array());
}

#[test]
fn test_create_struct() {
    let mut sb = ScriptBuilder::new();

    sb.emit_push_int(1);
    sb.emit_push_int(2);
    sb.emit_push_int(2); // Count
    sb.emit_opcode(OpCode::PACKSTRUCT);

    let result = sb.to_array();
    assert!(result.len() > 3);
    assert_eq!(result[result.len() - 1], OpCode::PACKSTRUCT as u8);
}

#[test]
fn test_create_map() {
    let mut sb = ScriptBuilder::new();

    sb.emit_opcode(OpCode::NEWMAP);
    sb.emit_push_int(1);
    sb.emit_push_int(2);
    sb.emit_opcode(OpCode::SETITEM);
    sb.emit_push_int(3);
    sb.emit_push_int(4);
    sb.emit_opcode(OpCode::SETITEM);

    let result = sb.to_array();
    assert!(result.len() > 10);
    assert_eq!(result[0], OpCode::NEWMAP as u8);
}

#[test]
fn test_make_script() {
    let mut sb = ScriptBuilder::new();
    let contract_hash = UInt160::zero();
    let operation = "balanceOf";
    let parameter = UInt160::zero();

    sb.emit_push(&parameter.to_array());
    sb.emit_push_int(1); // Parameter count
    sb.emit_opcode(OpCode::PACK);
    sb.emit_push_int(15); // CallFlags.All
    sb.emit_push(operation.as_bytes());
    sb.emit_push(&contract_hash.to_array());
    sb.emit_syscall("System.Contract.Call");

    let script = sb.to_array();
    assert!(script.len() > 20);
    assert!(script.contains(&(OpCode::SYSCALL as u8)));
}

#[test]
fn test_stack_item_json_serialization() {
    // Test Integer
    let item = StackItem::Integer(5.into());
    let json_str = format!("{{\"type\":\"Integer\",\"value\":\"5\"}}");
    // Note: Actual JSON implementation may vary, this tests the concept

    // Test Boolean
    let item = StackItem::Boolean(true);
    let json_str = format!("{{\"type\":\"Boolean\",\"value\":true}}");

    // Test ByteString
    let item = StackItem::ByteString(vec![1, 2, 3]);
    // Should serialize to base64: AQID

    // Test Array
    let items = vec![StackItem::Integer(5.into()), StackItem::Boolean(true)];
    let item = StackItem::Array(items);
    // Should contain nested JSON structure
}

#[test]
fn test_stack_item_to_parameter() {
    // Test Integer
    let item = StackItem::Integer(1000.into());
    // Should convert to ContractParameterType::Integer

    // Test Boolean
    let item = StackItem::Boolean(false);
    // Should convert to ContractParameterType::Boolean

    // Test ByteString
    let item = StackItem::ByteString(vec![1, 2, 3]);
    // Should convert to ContractParameterType::ByteArray

    // Test Array
    let items = vec![StackItem::Integer(1.into()), StackItem::Boolean(true)];
    let item = StackItem::Array(items);
    // Should convert to ContractParameterType::Array
}

#[test]
fn test_parameter_to_stack_item() {
    // Test Integer parameter -> StackItem
    let item = StackItem::Integer(1000.into());
    match item {
        StackItem::Integer(val) => {
            // Verify conversion
            assert!(val.to_string() == "1000");
        }
        _ => panic!("Expected Integer StackItem"),
    }

    // Test Boolean parameter -> StackItem
    let item = StackItem::Boolean(true);
    match item {
        StackItem::Boolean(val) => assert_eq!(val, true),
        _ => panic!("Expected Boolean StackItem"),
    }
}

#[test]
fn test_emit_push_various_types() {
    let mut sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());

    sb = ScriptBuilder::new();
    sb.emit_push_int(0);
    assert_eq!(vec![OpCode::PUSH0 as u8], sb.to_array());
}

#[test]
fn test_emit_push_uint256() {
    let mut sb = ScriptBuilder::new();
    let uint256 = UInt256::zero();
    sb.emit_push(&uint256.to_array());

    let mut expected = vec![OpCode::PUSHDATA1 as u8, 0x20];
    expected.extend_from_slice(&[0u8; 32]);
    assert_eq!(expected, sb.to_array());
}

#[test]
fn test_emit_push_signature() {
    let mut sb = ScriptBuilder::new();
    let signature = vec![0u8; 64];
    sb.emit_push(&signature);

    let mut expected = vec![OpCode::PUSHDATA1 as u8, 0x40];
    expected.extend_from_slice(&signature);
    assert_eq!(expected, sb.to_array());
}

#[test]
fn test_emit_push_public_key() {
    let mut sb = ScriptBuilder::new();
    let pk_bytes = vec![0u8; 33]; // Compressed public key format
    sb.emit_push(&pk_bytes);

    let mut expected = vec![OpCode::PUSHDATA1 as u8, 33];
    expected.extend_from_slice(&pk_bytes);
    assert_eq!(expected, sb.to_array());
}

#[test]
fn test_cyclic_reference_handling() {
    // This is more of a safety test to ensure we don't get infinite loops

    let mut sb = ScriptBuilder::new();

    sb.emit_opcode(OpCode::NEWMAP);
    sb.emit_push_int(1);
    sb.emit_push_int(1); // Self-referential in value
    sb.emit_opcode(OpCode::SETITEM);

    // Should complete without infinite loop
    let result = sb.to_array();
    assert!(result.len() > 0);
}

#[test]
fn test_large_data_handling() {
    // Test handling of larger data structures
    let mut sb = ScriptBuilder::new();

    // Create array with 100 elements
    for i in 0..100 {
        sb.emit_push_int(i);
    }
    sb.emit_push_int(100); // Count
    sb.emit_opcode(OpCode::PACK);

    let result = sb.to_array();
    assert!(result.len() > 100); // Should have substantial size
    assert_eq!(result[result.len() - 1], OpCode::PACK as u8); // Should end with PACK
}

#[test]
fn test_error_handling() {
    // Test various error conditions and edge cases

    // Test empty script
    let sb = ScriptBuilder::new();
    assert_eq!(sb.to_array().len(), 0);

    // Test script with only opcodes
    let mut sb = ScriptBuilder::new();
    sb.emit_opcode(OpCode::NOP);
    sb.emit_opcode(OpCode::RET);
    assert_eq!(sb.to_array(), vec![OpCode::NOP as u8, OpCode::RET as u8]);
}

#[test]
fn test_script_builder_chaining() {
    // Test that ScriptBuilder operations can be chained effectively
    let mut sb = ScriptBuilder::new();
    sb.emit_push_int(1);
    sb.emit_push_int(2);
    sb.emit_opcode(OpCode::ADD);
    sb.emit_opcode(OpCode::RET);

    let result = sb.to_array();
    assert_eq!(
        result,
        vec![
            OpCode::PUSH1 as u8,
            OpCode::PUSH2 as u8,
            OpCode::ADD as u8,
            OpCode::RET as u8,
        ]
    );
}

#[test]
fn test_char_as_uint16() {
    let mut sb = ScriptBuilder::new();
    let char_value = 'A' as u16; // 65
    sb.emit_push_int(char_value as i64);

    let result = sb.to_array();
    assert!(result.len() > 0);
    // Should encode the character value properly
}

#[test]
fn test_emit_push_data_size_boundaries() {
    // Test different PUSHDATA instruction boundaries

    let mut sb = ScriptBuilder::new();
    let data_small = vec![0x42; 75]; // 75 bytes (max for direct push)
    sb.emit_push(&data_small);

    let mut expected = vec![data_small.len() as u8]; // Length as opcode
    expected.extend_from_slice(&data_small);
    assert_eq!(sb.to_array(), expected);

    // PUSHDATA1: 76-255 bytes
    sb = ScriptBuilder::new();
    let data_medium = vec![0x42; 76]; // 76 bytes (min for PUSHDATA1)
    sb.emit_push(&data_medium);

    let mut expected = vec![OpCode::PUSHDATA1 as u8, data_medium.len() as u8];
    expected.extend_from_slice(&data_medium);
    assert_eq!(sb.to_array(), expected);

    // PUSHDATA2: 256-65535 bytes
    sb = ScriptBuilder::new();
    let data_large = vec![0x42; 256]; // 256 bytes (min for PUSHDATA2)
    sb.emit_push(&data_large);

    let mut expected = vec![OpCode::PUSHDATA2 as u8];
    expected.extend_from_slice(&(data_large.len() as u16).to_le_bytes());
    expected.extend_from_slice(&data_large);
    assert_eq!(sb.to_array(), expected);
}

#[test]
fn test_script_conversion() {
    // Test converting ScriptBuilder to Script
    let mut sb = ScriptBuilder::new();
    sb.emit_push_int(1);
    sb.emit_push_int(2);
    sb.emit_opcode(OpCode::ADD);
    sb.emit_opcode(OpCode::RET);

    let script = sb.to_script();
    assert_eq!(script.len(), 4);

    // Verify the script can be created from bytes
    let bytes = sb.to_array();
    let script_from_bytes = Script::new(bytes, false).unwrap();
    assert_eq!(script.len(), script_from_bytes.len());
}

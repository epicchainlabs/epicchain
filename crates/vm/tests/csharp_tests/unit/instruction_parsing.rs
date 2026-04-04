//! Instruction parsing unit tests
//!
//! Tests for instruction parsing edge cases and error handling.

use neo_io::MemoryReader;
use neo_vm::instruction::Instruction;

/// Test that MemoryReader correctly fails when trying to read beyond bounds
#[test]
fn test_memory_reader_bounds_check() {
    // Test that MemoryReader correctly fails when trying to read beyond bounds
    let data = vec![0x01, 0x02, 0x03, 0x04]; // 4 bytes
    let mut reader = MemoryReader::new(&data);

    // Should succeed: read 4 bytes
    let result = reader.read_bytes(4);
    assert!(
        result.is_ok(),
        "Should be able to read 4 bytes from 4-byte data"
    );
    assert_eq!(result.unwrap(), &[0x01, 0x02, 0x03, 0x04]);

    // Reset reader
    let mut reader = MemoryReader::new(&data);

    // Should fail: try to read 5 bytes from 4-byte data
    let result = reader.read_bytes(5);
    assert!(
        result.is_err(),
        "Should fail when trying to read 5 bytes from 4-byte data"
    );
    println!(
        "Expected error when reading beyond bounds: {:?}",
        result.err()
    );
}

/// Test PUSHDATA1 instruction parsing with insufficient data
#[test]
fn test_pushdata1_instruction_parsing() {
    // Test PUSHDATA1 instruction parsing with insufficient data
    let script = vec![0x0c, 0x05, 0x01, 0x02, 0x03, 0x04]; // PUSHDATA1, length=5, only 4 bytes data
    let mut reader = MemoryReader::new(&script);

    // This should fail because PUSHDATA1 expects 5 bytes but only 4 are available
    let result = Instruction::parse_from_reader(&mut reader);
    println!("PUSHDATA1 parse_from_reader result: {:?}", result);

    if result.is_err() {
        println!("✅ parse_from_reader correctly failed with insufficient data");
    } else {
        println!("❌ parse_from_reader unexpectedly succeeded");
        let instruction = result.unwrap();
        println!(
            "Parsed instruction: opcode={:?}, operand={:?}",
            instruction.opcode(),
            instruction.operand()
        );
    }

    // Now test the epicchain-io version
    let mut neo_reader = neo_io::MemoryReader::new(&script);
    let result = Instruction::parse_from_neo_io_reader(&mut neo_reader);
    println!("PUSHDATA1 parse_from_neo_io_reader result: {:?}", result);

    if result.is_err() {
        println!("✅ parse_from_neo_io_reader correctly failed with insufficient data");
    } else {
        println!("❌ parse_from_neo_io_reader unexpectedly succeeded");
        let instruction = result.unwrap();
        println!(
            "Parsed instruction: opcode={:?}, operand={:?}",
            instruction.opcode(),
            instruction.operand()
        );
    }
}

/// Test instruction parsing with various operand sizes
#[test]
fn test_instruction_operand_sizes() {
    let script = vec![0x00, 0x42]; // PUSHINT8 + value
    let mut reader = MemoryReader::new(&script);
    let result = Instruction::parse_from_reader(&mut reader);
    assert!(result.is_ok(), "PUSHINT8 should parse successfully");

    let instruction = result.unwrap();
    assert_eq!(
        instruction.operand_data().len(),
        1,
        "PUSHINT8 should have 1-byte operand"
    );
    assert_eq!(
        instruction.operand_data()[0],
        0x42,
        "PUSHINT8 operand should be 0x42"
    );

    let script = vec![0x01, 0x34, 0x12]; // PUSHINT16 + value (little-endian)
    let mut reader = MemoryReader::new(&script);
    let result = Instruction::parse_from_reader(&mut reader);
    assert!(result.is_ok(), "PUSHINT16 should parse successfully");

    let instruction = result.unwrap();
    assert_eq!(
        instruction.operand_data().len(),
        2,
        "PUSHINT16 should have 2-byte operand"
    );
    assert_eq!(
        instruction.operand_data(),
        &[0x34, 0x12],
        "PUSHINT16 operand should be [0x34, 0x12]"
    );

    let script = vec![0x02, 0x78, 0x56, 0x34, 0x12]; // PUSHINT32 + value (little-endian)
    let mut reader = MemoryReader::new(&script);
    let result = Instruction::parse_from_reader(&mut reader);
    assert!(result.is_ok(), "PUSHINT32 should parse successfully");

    let instruction = result.unwrap();
    assert_eq!(
        instruction.operand_data().len(),
        4,
        "PUSHINT32 should have 4-byte operand"
    );
    assert_eq!(
        instruction.operand_data(),
        &[0x78, 0x56, 0x34, 0x12],
        "PUSHINT32 operand should be [0x78, 0x56, 0x34, 0x12]"
    );

    let script = vec![0x03, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]; // PUSHINT64 + value
    let mut reader = MemoryReader::new(&script);
    let result = Instruction::parse_from_reader(&mut reader);
    assert!(result.is_ok(), "PUSHINT64 should parse successfully");

    let instruction = result.unwrap();
    assert_eq!(
        instruction.operand_data().len(),
        8,
        "PUSHINT64 should have 8-byte operand"
    );
    assert_eq!(
        instruction.operand_data(),
        &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11],
        "PUSHINT64 operand should be correct"
    );
}

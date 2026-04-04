/// Comprehensive VM OpCode compatibility tests
/// These tests ensure that the Rust VM implementation matches the C# Neo VM exactly
///
/// Reference: https://github.com/neo-project/epicchain-vm/blob/master/src/Neo.VM/OpCode.cs
use neo_vm::op_code::OpCode;

#[test]
fn test_opcode_values_match_csharp() {
    // Constants
    assert_eq!(OpCode::PUSHINT8 as u8, 0x00);
    assert_eq!(OpCode::PUSHINT16 as u8, 0x01);
    assert_eq!(OpCode::PUSHINT32 as u8, 0x02);
    assert_eq!(OpCode::PUSHINT64 as u8, 0x03);
    assert_eq!(OpCode::PUSHINT128 as u8, 0x04);
    assert_eq!(OpCode::PUSHINT256 as u8, 0x05);
    assert_eq!(OpCode::PUSHT as u8, 0x08);
    assert_eq!(OpCode::PUSHF as u8, 0x09);
    assert_eq!(OpCode::PUSHA as u8, 0x0A);
    assert_eq!(OpCode::PUSHNULL as u8, 0x0B);
    assert_eq!(OpCode::PUSHDATA1 as u8, 0x0C);
    assert_eq!(OpCode::PUSHDATA2 as u8, 0x0D);
    assert_eq!(OpCode::PUSHDATA4 as u8, 0x0E);
    assert_eq!(OpCode::PUSHM1 as u8, 0x0F);
    assert_eq!(OpCode::PUSH0 as u8, 0x10);
    assert_eq!(OpCode::PUSH1 as u8, 0x11);
    assert_eq!(OpCode::PUSH2 as u8, 0x12);
    assert_eq!(OpCode::PUSH3 as u8, 0x13);
    assert_eq!(OpCode::PUSH4 as u8, 0x14);
    assert_eq!(OpCode::PUSH5 as u8, 0x15);
    assert_eq!(OpCode::PUSH6 as u8, 0x16);
    assert_eq!(OpCode::PUSH7 as u8, 0x17);
    assert_eq!(OpCode::PUSH8 as u8, 0x18);
    assert_eq!(OpCode::PUSH9 as u8, 0x19);
    assert_eq!(OpCode::PUSH10 as u8, 0x1A);
    assert_eq!(OpCode::PUSH11 as u8, 0x1B);
    assert_eq!(OpCode::PUSH12 as u8, 0x1C);
    assert_eq!(OpCode::PUSH13 as u8, 0x1D);
    assert_eq!(OpCode::PUSH14 as u8, 0x1E);
    assert_eq!(OpCode::PUSH15 as u8, 0x1F);
    assert_eq!(OpCode::PUSH16 as u8, 0x20);

    // Flow control
    assert_eq!(OpCode::NOP as u8, 0x21);
    assert_eq!(OpCode::JMP as u8, 0x22);
    assert_eq!(OpCode::JmpL as u8, 0x23);
    assert_eq!(OpCode::JMPIF as u8, 0x24);
    assert_eq!(OpCode::JmpifL as u8, 0x25);
    assert_eq!(OpCode::JMPIFNOT as u8, 0x26);
    assert_eq!(OpCode::JmpifnotL as u8, 0x27);
    assert_eq!(OpCode::JMPEQ as u8, 0x28);
    assert_eq!(OpCode::JmpeqL as u8, 0x29);
    assert_eq!(OpCode::JMPNE as u8, 0x2A);
    assert_eq!(OpCode::JmpneL as u8, 0x2B);
    assert_eq!(OpCode::JMPGT as u8, 0x2C);
    assert_eq!(OpCode::JmpgtL as u8, 0x2D);
    assert_eq!(OpCode::JMPGE as u8, 0x2E);
    assert_eq!(OpCode::JmpgeL as u8, 0x2F);
    assert_eq!(OpCode::JMPLT as u8, 0x30);
    assert_eq!(OpCode::JmpltL as u8, 0x31);
    assert_eq!(OpCode::JMPLE as u8, 0x32);
    assert_eq!(OpCode::JmpleL as u8, 0x33);
    assert_eq!(OpCode::CALL as u8, 0x34);
    assert_eq!(OpCode::CallL as u8, 0x35);
    assert_eq!(OpCode::CALLA as u8, 0x36);
    assert_eq!(OpCode::CALLT as u8, 0x37);
    assert_eq!(OpCode::ABORT as u8, 0x38);
    assert_eq!(OpCode::ASSERT as u8, 0x39);
    assert_eq!(OpCode::THROW as u8, 0x3A);
    assert_eq!(OpCode::TRY as u8, 0x3B);
    assert_eq!(OpCode::TryL as u8, 0x3C);
    assert_eq!(OpCode::ENDTRY as u8, 0x3D);
    assert_eq!(OpCode::EndtryL as u8, 0x3E);
    assert_eq!(OpCode::ENDFINALLY as u8, 0x3F);
    assert_eq!(OpCode::RET as u8, 0x40);
    assert_eq!(OpCode::SYSCALL as u8, 0x41);

    // Stack
    assert_eq!(OpCode::DEPTH as u8, 0x43);
    assert_eq!(OpCode::DROP as u8, 0x45);
    assert_eq!(OpCode::NIP as u8, 0x46);
    assert_eq!(OpCode::XDROP as u8, 0x48);
    assert_eq!(OpCode::CLEAR as u8, 0x49);
    assert_eq!(OpCode::DUP as u8, 0x4A);
    assert_eq!(OpCode::OVER as u8, 0x4B);
    assert_eq!(OpCode::PICK as u8, 0x4D);
    assert_eq!(OpCode::TUCK as u8, 0x4E);
    assert_eq!(OpCode::SWAP as u8, 0x50);
    assert_eq!(OpCode::ROT as u8, 0x51);
    assert_eq!(OpCode::ROLL as u8, 0x52);
    assert_eq!(OpCode::REVERSE3 as u8, 0x53);
    assert_eq!(OpCode::REVERSE4 as u8, 0x54);
    assert_eq!(OpCode::REVERSEN as u8, 0x55);

    // Splice - CRITICAL TEST AREA
    assert_eq!(OpCode::NEWBUFFER as u8, 0x88);
    assert_eq!(OpCode::MEMCPY as u8, 0x89);
    // 0x8A is not used in C# Neo
    assert_eq!(OpCode::CAT as u8, 0x8B);
    assert_eq!(OpCode::SUBSTR as u8, 0x8C);
    assert_eq!(OpCode::LEFT as u8, 0x8D);
    assert_eq!(OpCode::RIGHT as u8, 0x8E);

    // Bitwise
    assert_eq!(OpCode::INVERT as u8, 0x90);
    assert_eq!(OpCode::AND as u8, 0x91);
    assert_eq!(OpCode::OR as u8, 0x92);
    assert_eq!(OpCode::XOR as u8, 0x93);
    assert_eq!(OpCode::EQUAL as u8, 0x97);
    assert_eq!(OpCode::NOTEQUAL as u8, 0x98);

    // Numeric
    assert_eq!(OpCode::SIGN as u8, 0x99);
    assert_eq!(OpCode::ABS as u8, 0x9A);
    assert_eq!(OpCode::NEGATE as u8, 0x9B);
    assert_eq!(OpCode::INC as u8, 0x9C);
    assert_eq!(OpCode::DEC as u8, 0x9D);
    assert_eq!(OpCode::ADD as u8, 0x9E);
    assert_eq!(OpCode::SUB as u8, 0x9F);
    assert_eq!(OpCode::MUL as u8, 0xA0);
    assert_eq!(OpCode::DIV as u8, 0xA1);
    assert_eq!(OpCode::MOD as u8, 0xA2);
    assert_eq!(OpCode::POW as u8, 0xA3);
    assert_eq!(OpCode::SQRT as u8, 0xA4);
    assert_eq!(OpCode::MODMUL as u8, 0xA5);
    assert_eq!(OpCode::MODPOW as u8, 0xA6);
    assert_eq!(OpCode::SHL as u8, 0xA8);
    assert_eq!(OpCode::SHR as u8, 0xA9);
    assert_eq!(OpCode::NOT as u8, 0xAA);
    assert_eq!(OpCode::BOOLAND as u8, 0xAB);
    assert_eq!(OpCode::BOOLOR as u8, 0xAC);
    assert_eq!(OpCode::NZ as u8, 0xB1);
    assert_eq!(OpCode::NUMEQUAL as u8, 0xB3);
    assert_eq!(OpCode::NUMNOTEQUAL as u8, 0xB4);
    assert_eq!(OpCode::LT as u8, 0xB5);
    assert_eq!(OpCode::LE as u8, 0xB6);
    assert_eq!(OpCode::GT as u8, 0xB7);
    assert_eq!(OpCode::GE as u8, 0xB8);
    assert_eq!(OpCode::MIN as u8, 0xB9);
    assert_eq!(OpCode::MAX as u8, 0xBA);
    assert_eq!(OpCode::WITHIN as u8, 0xBB);

    // Compound
    assert_eq!(OpCode::PACKMAP as u8, 0xBE);
    assert_eq!(OpCode::PACKSTRUCT as u8, 0xBF);
    assert_eq!(OpCode::PACK as u8, 0xC0);
    assert_eq!(OpCode::UNPACK as u8, 0xC1);
    assert_eq!(OpCode::NEWARRAY0 as u8, 0xC2);
    assert_eq!(OpCode::NEWARRAY as u8, 0xC3);
    assert_eq!(OpCode::NewarrayT as u8, 0xC4);
    assert_eq!(OpCode::NEWSTRUCT0 as u8, 0xC5);
    assert_eq!(OpCode::NEWSTRUCT as u8, 0xC6);
    assert_eq!(OpCode::NEWMAP as u8, 0xC8);
    assert_eq!(OpCode::SIZE as u8, 0xCA);
    assert_eq!(OpCode::HASKEY as u8, 0xCB);
    assert_eq!(OpCode::KEYS as u8, 0xCC);
    assert_eq!(OpCode::VALUES as u8, 0xCD);
    assert_eq!(OpCode::PICKITEM as u8, 0xCE);
    assert_eq!(OpCode::APPEND as u8, 0xCF);
    assert_eq!(OpCode::SETITEM as u8, 0xD0);
    assert_eq!(OpCode::REVERSEITEMS as u8, 0xD1);
    assert_eq!(OpCode::REMOVE as u8, 0xD2);
    assert_eq!(OpCode::CLEARITEMS as u8, 0xD3);
    assert_eq!(OpCode::POPITEM as u8, 0xD4);

    // Types
    assert_eq!(OpCode::ISNULL as u8, 0xD8);
    assert_eq!(OpCode::ISTYPE as u8, 0xD9);
    assert_eq!(OpCode::CONVERT as u8, 0xDB);

    // Extensions
    assert_eq!(OpCode::ABORTMSG as u8, 0xE0);
    assert_eq!(OpCode::ASSERTMSG as u8, 0xE1);
}

#[test]
fn test_opcode_from_byte_critical_values() {
    // Test the critical splice opcodes that were previously broken
    assert_eq!(OpCode::from_byte(0x88), Some(OpCode::NEWBUFFER));
    assert_eq!(OpCode::from_byte(0x89), Some(OpCode::MEMCPY));
    assert_eq!(OpCode::from_byte(0x8A), None); // Not used in C# Neo
    assert_eq!(OpCode::from_byte(0x8B), Some(OpCode::CAT));
    assert_eq!(OpCode::from_byte(0x8C), Some(OpCode::SUBSTR));
    assert_eq!(OpCode::from_byte(0x8D), Some(OpCode::LEFT));
    assert_eq!(OpCode::from_byte(0x8E), Some(OpCode::RIGHT));

    // Test some other important values
    assert_eq!(OpCode::from_byte(0x00), Some(OpCode::PUSHINT8));
    assert_eq!(OpCode::from_byte(0x41), Some(OpCode::SYSCALL));
    assert_eq!(OpCode::from_byte(0x4C), None); // TOALTSTACK not in C#
    assert_eq!(OpCode::from_byte(0x4F), None); // FROMALTSTACK not in C#
}

#[test]
fn test_no_invalid_opcodes() {
    // Ensure we don't have TOALTSTACK or FROMALTSTACK
    for i in 0u8..=255 {
        if let Some(opcode) = OpCode::from_byte(i) {
            // Make sure we don't have any invalid opcodes
            match i {
                0x4C | 0x4F => panic!("Found invalid opcode at 0x{:02X}: {:?}", i, opcode),
                _ => {} // Valid opcode
            }
        }
    }
}

#[test]
fn test_opcode_roundtrip() {
    // Test that all valid opcodes can be converted to byte and back
    for opcode in OpCode::iter() {
        let byte = opcode as u8;
        let from_byte = OpCode::from_byte(byte);
        assert_eq!(
            from_byte,
            Some(opcode),
            "OpCode {:?} (0x{:02X}) failed roundtrip",
            opcode,
            byte
        );
    }
}

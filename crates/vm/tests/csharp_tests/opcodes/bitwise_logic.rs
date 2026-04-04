//! Bitwise and logic opcode tests
//!
//! Tests for bitwise and logical operations like AND, OR, XOR, etc.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test OpCodes BitwiseLogic category (matches C# TestOpCodesBitwiseLogic)
#[test]
fn test_opcodes_bitwise_logic() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/BitwiseLogic";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

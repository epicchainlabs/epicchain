//! Arithmetic opcode tests
//!
//! Tests for arithmetic operations like ADD, SUB, MUL, DIV, etc.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test OpCodes Arithmetic category (matches C# TestOpCodesArithmetic)
#[test]
fn test_opcodes_arithmetic() {
    let test_path =
        "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Arithmetic";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

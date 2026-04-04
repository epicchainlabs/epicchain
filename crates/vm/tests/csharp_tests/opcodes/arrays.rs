//! Array operation opcode tests
//!
//! Tests for array operations like NEWARRAY, APPEND, etc.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test OpCodes Arrays category (matches C# TestOpCodesArrays)
#[test]
fn test_opcodes_arrays() {
    let test_path =
        "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Arrays";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

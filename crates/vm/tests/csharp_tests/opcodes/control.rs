//! Control flow opcode tests
//!
//! Tests for control flow operations like JMP, CALL, RET, etc.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test OpCodes Control category (matches C# TestOpCodesControl)
#[test]
fn test_opcodes_control() {
    let test_path =
        "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Control";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

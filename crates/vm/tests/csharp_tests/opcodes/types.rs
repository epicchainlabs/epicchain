//! Type operation opcode tests
//!
//! Tests for type operations like ISNULL, ISTYPE, etc.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test OpCodes Types category (matches C# TestOpCodesTypes)
#[test]
fn test_opcodes_types() {
    let test_path =
        "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Types";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

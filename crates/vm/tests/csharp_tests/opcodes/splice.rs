//! Splice operation opcode tests
//!
//! Tests for splice operations like CAT, SUBSTR, etc.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test OpCodes Splice category (matches C# TestOpCodesSplice)
#[test]
fn test_opcodes_splice() {
    let test_path =
        "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Splice";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

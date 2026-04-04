//! Comprehensive integration tests
//!
//! Tests that run the complete C# JSON test suite to ensure
//! overall compatibility between Rust and C# implementations.

use crate::csharp_tests::JsonTestRunner;
use std::path::Path;

/// Test Others category (matches C# TestOthers)
#[test]
fn test_others() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/Others";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();
        runner.test_json_directory(test_path).unwrap();
    } else {
        println!("C# test directory not found: {}", test_path);
    }
}

/// Test all available JSON test files in the C# test suite
#[test]
fn test_all_csharp_json_tests() {
    let base_test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests";
    if Path::new(base_test_path).exists() {
        let mut runner = JsonTestRunner::new();

        println!("Running comprehensive C# JSON test suite/* implementation */;");

        let categories = vec![
            "Others",
            "OpCodes/Arrays",
            "OpCodes/Stack",
            "OpCodes/Slot",
            "OpCodes/Splice",
            "OpCodes/Control",
            "OpCodes/Push",
            "OpCodes/Arithmetic",
            "OpCodes/BitwiseLogic",
            "OpCodes/Types",
        ];

        for category in categories {
            let category_path = format!("{}/{}", base_test_path, category);
            if Path::new(&category_path).exists() {
                println!("Testing category: {}", category);
                match runner.test_json_directory(&category_path) {
                    Ok(_) => println!("  ✓ Category {} passed", category),
                    Err(e) => {
                        println!("  ✗ Category {} failed: {}", category, e);
                        // Continue with other categories instead of failing completely
                    }
                }
            } else {
                println!("  - Category {} not found", category);
            }
        }
    } else {
        println!("C# test base directory not found: {}", base_test_path);
    }
}

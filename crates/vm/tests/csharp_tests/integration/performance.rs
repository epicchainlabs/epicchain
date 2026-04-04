//! Performance and benchmark tests
//!
//! Tests that measure performance and handle edge cases
//! like malformed JSON and error conditions.

use serde_json;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::csharp_tests::{JsonTestRunner, VMUT};

/// Benchmark test to compare performance with C# implementation
#[test]
fn benchmark_json_test_performance() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push/PUSHNULL.json";
    if Path::new(test_path).exists() {
        let mut runner = JsonTestRunner::new();

        let start = Instant::now();
        for _ in 0..100 {
            runner.test_json_file(test_path).unwrap();
        }
        let duration = start.elapsed();

        println!("100 iterations of PUSHNULL test took: {:?}", duration);
        println!("Average per test: {:?}", duration / 100);
    } else {
        println!("C# test file not found for benchmark: {}", test_path);
    }
}

/// Test error handling for malformed JSON tests
#[test]
fn test_malformed_json_handling() {
    let mut runner = JsonTestRunner::new();

    // Test with non-existent file
    let result = runner.test_json_file("/non/existent/path.json");
    assert!(result.is_err(), "Should fail for non-existent file");

    // Test with non-existent directory
    let result = runner.test_json_directory("/non/existent/directory");
    assert!(result.is_err(), "Should fail for non-existent directory");
}

/// Test JSON file parsing without execution
#[test]
fn test_json_file_parsing() {
    let test_path = "/Users/jinghuiliao/git/will/neo-dev/neo-sharp/tests/Neo.VM.Tests/Tests/OpCodes/Push/PUSHNULL.json";

    if std::path::Path::new(test_path).exists() {
        println!("📁 Loading JSON test file: {}", test_path);

        // Try to read and parse the JSON file
        let file_content = fs::read_to_string(test_path).expect("Failed to read test file");
        println!("📄 File content length: {} bytes", file_content.len());

        // Try to parse as VMUT structure
        let vmut_result: Result<VMUT, _> = serde_json::from_str(&file_content);
        match vmut_result {
            Ok(vmut) => {
                println!("✅ JSON parsing successful!");
                println!("   Category: {}", vmut.category);
                println!("   Name: {}", vmut.name);
                println!("   Number of tests: {}", vmut.tests.len());

                // Try to compile the first test script
                if let Some(first_test) = vmut.tests.first() {
                    println!("   First test: {}", first_test.name);
                    println!("   Script: {:?}", first_test.script);

                    let runner = JsonTestRunner::new();
                    let compile_result = runner.compile_script(&first_test.script);
                    match compile_result {
                        Ok(bytecode) => {
                            println!("   ✅ Script compilation successful: {:?}", bytecode);
                        }
                        Err(e) => {
                            println!("   ❌ Script compilation failed: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ JSON parsing failed: {}", e);
                println!(
                    "   Raw content preview: {}",
                    &file_content[..std::cmp::min(200, file_content.len())]
                );
            }
        }
    } else {
        println!("❌ Test file not found: {}", test_path);
    }
}

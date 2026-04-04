//! C# JSON Test Suite for Neo VM
//!
//! This module provides comprehensive testing capabilities for the Neo VM by executing
//! C# JSON test files and verifying that the Rust implementation produces identical results.
//!
//! ## Overview
//!
//! The C# Neo VM test suite contains hundreds of JSON test files that define:
//! - Script bytecode to execute
//! - Step-by-step execution actions
//! - Expected VM state after each step
//! - Expected stack contents and values
//!
//! This module deserializes these JSON files and executes them using the Rust Neo VM,
//! ensuring compatibility between the C# and Rust implementations.
//!
//! ## Test Categories
//!
//! Tests are organized into categories matching the C# test structure:
//! - **OpCodes**: Tests for individual opcodes (Push, Arithmetic, Stack, etc.)
//! - **Others**: General VM behavior tests
//! - **Integration**: Comprehensive test suite execution
//!
//! ## Usage
//!
//! Run specific test categories:
//! ```bash
//! cargo test -p epicchain-vm --test csharp_json_tests opcodes::push
//! cargo test -p epicchain-vm --test csharp_json_tests opcodes::arithmetic
//! ```
//!
//! Run all C# JSON tests:
//! ```bash
//! cargo test -p epicchain-vm --test csharp_json_tests integration::comprehensive
//! ```

mod csharp_tests;

//! C# JSON Test Suite for Neo VM
//!
//! This module contains a comprehensive test suite that executes C# Neo VM JSON test files
//! to ensure compatibility between the Rust and C# implementations.
//!
//! ## Structure
//!
//! - `common`: Shared data structures for JSON deserialization
//! - `runner`: Test execution engine
//! - `opcodes`: Tests organized by opcode categories
//! - `integration`: Comprehensive integration tests
//! - `unit`: Focused unit tests for specific components

pub mod common;
pub mod runner;

pub mod integration;
pub mod opcodes;
pub mod unit;

pub use common::*;
pub use runner::JsonTestRunner;

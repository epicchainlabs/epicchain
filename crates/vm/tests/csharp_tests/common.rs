//! Common data structures and utilities for C# JSON tests
//!
//! This module contains the shared data structures used for deserializing
//! C# Neo VM test JSON files and common utilities used across all test modules.

use serde::{Deserialize, Serialize};

/// VM Unit Test structure (matches C# VMUT class)
#[derive(Debug, Deserialize, Serialize)]
pub struct VMUT {
    pub category: String,
    pub name: String,
    pub tests: Vec<VMUTTest>,
}

/// Individual test case within a VMUT (matches C# VMUTTest class)
#[derive(Debug, Deserialize, Serialize)]
pub struct VMUTTest {
    pub name: String,
    pub script: Vec<String>,
    pub steps: Vec<VMUTStep>,
}

/// Test execution step (matches C# VMUTStep class)
#[derive(Debug, Deserialize, Serialize)]
pub struct VMUTStep {
    #[serde(default)]
    pub name: Option<String>,
    pub actions: Vec<String>,
    pub result: VMUTExecutionEngineState,
}

/// Expected execution engine state (matches C# VMUTExecutionEngineState class)
#[derive(Debug, Deserialize, Serialize)]
pub struct VMUTExecutionEngineState {
    pub state: String,
    #[serde(rename = "invocationStack")]
    pub invocation_stack: Option<Vec<VMUTExecutionContextState>>,
    #[serde(rename = "resultStack")]
    pub result_stack: Option<Vec<VMUTStackItem>>,
}

/// Execution context state (matches C# VMUTExecutionContextState class)
#[derive(Debug, Deserialize, Serialize)]
pub struct VMUTExecutionContextState {
    #[serde(rename = "scriptHash")]
    pub script_hash: Option<String>,
    #[serde(rename = "instructionPointer")]
    pub instruction_pointer: Option<u32>,
    #[serde(rename = "nextInstruction")]
    pub next_instruction: Option<String>,
    #[serde(rename = "evaluationStack")]
    pub evaluation_stack: Option<Vec<VMUTStackItem>>,
}

/// Stack item representation (matches C# VMUTStackItem class)
#[derive(Debug, Deserialize, Serialize)]
pub struct VMUTStackItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub value: Option<serde_json::Value>,
}

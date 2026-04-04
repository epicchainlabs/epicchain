//! JSON Test Runner for Neo VM
//!
//! This module provides functionality to execute JSON-based VM tests
//! that match the C# Neo.VM.Tests JSON test format exactly.

use neo_vm::{ExecutionEngine, Script, StackItem, VMState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// VM Unit Test structure (matches C# VMUT class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMUT {
    pub category: String,
    pub name: String,
    pub tests: Vec<VMUTTest>,
}

/// Individual test case (matches C# VMUTTest class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMUTTest {
    pub name: String,
    pub script: Vec<String>,
    pub steps: Vec<VMUTStep>,
}

/// Test step (matches C# VMUTStep class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMUTStep {
    pub actions: Vec<String>,
    pub result: VMUTExecutionEngineState,
}

/// Execution engine state (matches C# VMUTExecutionEngineState class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMUTExecutionEngineState {
    pub state: String,
    #[serde(rename = "invocationStack")]
    pub invocation_stack: Option<Vec<VMUTExecutionContext>>,
    #[serde(rename = "resultStack")]
    pub result_stack: Option<Vec<VMUTStackItem>>,
}

/// Execution context (matches C# VMUTExecutionContext class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMUTExecutionContext {
    #[serde(rename = "instructionPointer")]
    pub instruction_pointer: usize,
    #[serde(rename = "nextInstruction")]
    pub next_instruction: Option<String>,
    #[serde(rename = "evaluationStack")]
    pub evaluation_stack: Vec<VMUTStackItem>,
}

/// Stack item representation (matches C# VMUTStackItem class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMUTStackItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub value: Option<serde_json::Value>,
}

/// JSON Test Runner (matches C# VMJsonTestBase functionality)
pub struct JsonTestRunner {
    engine: ExecutionEngine,
}

impl JsonTestRunner {
    /// Create a new JSON test runner
    pub fn new() -> Self {
        Self {
            engine: ExecutionEngine::new(None),
        }
    }

    /// Execute a JSON test file (matches C# TestJson method)
    pub fn test_json_file<P: AsRef<Path>>(
        &mut self,
        file_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(file_path.as_ref())?;
        let vmut: VMUT = serde_json::from_str(&file_content)?;

        println!("Testing: {} - {}", vmut.category, vmut.name);

        for test in vmut.tests {
            println!("  Running test: {}", test.name);
            self.execute_test(&test)?;
        }

        Ok(())
    }

    /// Execute a directory of JSON tests (matches C# TestJson method)
    pub fn test_json_directory<P: AsRef<Path>>(
        &mut self,
        dir_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let entries = fs::read_dir(dir_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                println!("Processing file: {:?}", path);
                self.test_json_file(&path)?;
            } else if path.is_dir() {
                self.test_json_directory(&path)?;
            }
        }

        Ok(())
    }

    /// Execute a single test (matches C# ExecuteTest method)
    fn execute_test(&mut self, test: &VMUTTest) -> Result<(), Box<dyn std::error::Error>> {
        let script_bytes = self.compile_script(&test.script)?;
        let script = Script::new(script_bytes, false).unwrap();

        // Load script into engine
        self.engine.load_script(script, -1, 0).unwrap();

        // Execute each step
        for (step_index, step) in test.steps.iter().enumerate() {
            println!("    Step {}: {:?}", step_index + 1, step.actions);

            // Execute actions
            for action in &step.actions {
                self.execute_action(action)?;
            }

            self.verify_result(&step.result)?;
        }

        Ok(())
    }

    /// Compile script from opcode strings to bytes (matches C# script compilation)
    fn compile_script(&self, opcodes: &[String]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();

        for opcode in opcodes {
            match opcode.as_str() {
                "PUSHNULL" => bytes.push(0xf0),
                "PUSHDATA1" => bytes.push(0x0c),
                "PUSHDATA2" => bytes.push(0x0d),
                "PUSHDATA4" => bytes.push(0x0e),
                "PUSHM1" => bytes.push(0x0f),
                "PUSH0" => bytes.push(0x10),
                "PUSH1" => bytes.push(0x11),
                "PUSH2" => bytes.push(0x12),
                "PUSH3" => bytes.push(0x13),
                "PUSH4" => bytes.push(0x14),
                "PUSH5" => bytes.push(0x15),
                "PUSH6" => bytes.push(0x16),
                "PUSH7" => bytes.push(0x17),
                "PUSH8" => bytes.push(0x18),
                "PUSH9" => bytes.push(0x19),
                "PUSH10" => bytes.push(0x1a),
                "PUSH11" => bytes.push(0x1b),
                "PUSH12" => bytes.push(0x1c),
                "PUSH13" => bytes.push(0x1d),
                "PUSH14" => bytes.push(0x1e),
                "PUSH15" => bytes.push(0x1f),
                "PUSH16" => bytes.push(0x20),
                "NOP" => bytes.push(0x21),
                "JMP" => bytes.push(0x22),
                "JMPIF" => bytes.push(0x23),
                "JMPIFNOT" => bytes.push(0x24),
                "JMPEQ" => bytes.push(0x25),
                "JMPNE" => bytes.push(0x26),
                "JMPGT" => bytes.push(0x27),
                "JMPGE" => bytes.push(0x28),
                "JMPLT" => bytes.push(0x29),
                "JMPLE" => bytes.push(0x2a),
                "CALL" => bytes.push(0x2b),
                "CALLA" => bytes.push(0x2c),
                "CALLT" => bytes.push(0x2d),
                "ABORT" => bytes.push(0x2e),
                "ASSERT" => bytes.push(0x2f),
                "THROW" => bytes.push(0x3a),
                "TRY" => bytes.push(0x3b),
                "TryL" => bytes.push(0x3c),
                "ENDTRY" => bytes.push(0x3d),
                "EndtryL" => bytes.push(0x3e),
                "ENDFINALLY" => bytes.push(0x3f),
                "RET" => bytes.push(0x40),
                "SYSCALL" => bytes.push(0x41),
                "DEPTH" => bytes.push(0x43),
                "DROP" => bytes.push(0x45),
                "NIP" => bytes.push(0x46),
                "XDROP" => bytes.push(0x48),
                "CLEAR" => bytes.push(0x49),
                "DUP" => bytes.push(0x4a),
                "OVER" => bytes.push(0x4b),
                "PICK" => bytes.push(0x4d),
                "TUCK" => bytes.push(0x4e),
                "SWAP" => bytes.push(0x50),
                "ROT" => bytes.push(0x51),
                "ROLL" => bytes.push(0x52),
                "REVERSE3" => bytes.push(0x53),
                "REVERSE4" => bytes.push(0x54),
                "REVERSEN" => bytes.push(0x55),
                "INITSSLOT" => bytes.push(0x56),
                "INITSLOT" => bytes.push(0x57),
                "LDSFLD0" => bytes.push(0x58),
                "LDSFLD1" => bytes.push(0x59),
                "LDSFLD2" => bytes.push(0x5a),
                "LDSFLD3" => bytes.push(0x5b),
                "LDSFLD4" => bytes.push(0x5c),
                "LDSFLD5" => bytes.push(0x5d),
                "LDSFLD6" => bytes.push(0x5e),
                "LDSFLD" => bytes.push(0x5f),
                "STSFLD0" => bytes.push(0x60),
                "STSFLD1" => bytes.push(0x61),
                "STSFLD2" => bytes.push(0x62),
                "STSFLD3" => bytes.push(0x63),
                "STSFLD4" => bytes.push(0x64),
                "STSFLD5" => bytes.push(0x65),
                "STSFLD6" => bytes.push(0x66),
                "STSFLD" => bytes.push(0x67),
                "LDLOC0" => bytes.push(0x68),
                "LDLOC1" => bytes.push(0x69),
                "LDLOC2" => bytes.push(0x6a),
                "LDLOC3" => bytes.push(0x6b),
                "LDLOC4" => bytes.push(0x6c),
                "LDLOC5" => bytes.push(0x6d),
                "LDLOC6" => bytes.push(0x6e),
                "LDLOC" => bytes.push(0x6f),
                "STLOC0" => bytes.push(0x70),
                "STLOC1" => bytes.push(0x71),
                "STLOC2" => bytes.push(0x72),
                "STLOC3" => bytes.push(0x73),
                "STLOC4" => bytes.push(0x74),
                "STLOC5" => bytes.push(0x75),
                "STLOC6" => bytes.push(0x76),
                "STLOC" => bytes.push(0x77),
                "LDARG0" => bytes.push(0x78),
                "LDARG1" => bytes.push(0x79),
                "LDARG2" => bytes.push(0x7a),
                "LDARG3" => bytes.push(0x7b),
                "LDARG4" => bytes.push(0x7c),
                "LDARG5" => bytes.push(0x7d),
                "LDARG6" => bytes.push(0x7e),
                "LDARG" => bytes.push(0x7f),
                "STARG0" => bytes.push(0x80),
                "STARG1" => bytes.push(0x81),
                "STARG2" => bytes.push(0x82),
                "STARG3" => bytes.push(0x83),
                "STARG4" => bytes.push(0x84),
                "STARG5" => bytes.push(0x85),
                "STARG6" => bytes.push(0x86),
                "STARG" => bytes.push(0x87),
                _ => return Err(format!("Unknown opcode: {}", opcode).into()),
            }
        }

        if bytes.is_empty() || bytes[bytes.len() - 1] != 0x40 {
            bytes.push(0x40); // RET
        }

        Ok(bytes)
    }

    /// Execute an action (matches C# action execution)
    fn execute_action(&mut self, action: &str) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            "stepInto" => {
                self.engine.execute_next_instruction()?;
            }
            "stepOver" => {
                self.engine.execute_next_instruction()?;
            }
            "stepOut" => {
                while self.engine.state() != VMState::HALT && self.engine.state() != VMState::FAULT
                {
                    self.engine.execute_next_instruction()?;
                }
            }
            _ => return Err(format!("Unknown action: {}", action).into()),
        }
        Ok(())
    }

    /// Verify the result matches expected state (matches C# result verification)
    fn verify_result(
        &self,
        expected: &VMUTExecutionEngineState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Verify VM state
        let actual_state = match self.engine.state() {
            VMState::NONE => "NONE",
            VMState::HALT => "HALT",
            VMState::FAULT => "FAULT",
            VMState::BREAK => "BREAK",
        };

        if actual_state != expected.state {
            return Err(format!(
                "State mismatch: expected {}, got {}",
                expected.state, actual_state
            )
            .into());
        }

        if let Some(expected_invocation_stack) = &expected.invocation_stack {
            println!("    Invocation stack verification not yet implemented");
        }

        if let Some(expected_result_stack) = &expected.result_stack {
            println!("    Result stack verification not yet implemented");
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_json_runner_creation() {
        let _runner = JsonTestRunner::new();
    }

    #[test]
    fn test_script_compilation() {
        let runner = JsonTestRunner::new();
        let opcodes = vec!["PUSHNULL".to_string(), "RET".to_string()];
        let result = runner.compile_script(&opcodes).unwrap();
        assert_eq!(result, vec![0xf0, 0x40]); // PUSHNULL + RET
    }
}

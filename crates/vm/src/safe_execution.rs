//! Safe execution utilities for the VM module
//!
//! This module provides safe alternatives to panic! calls and unsafe operations
//! in the VM execution engine.

use crate::{OpCode, VmError, VmResult};
use std::fmt;

/// Safe assertion for VM execution state
pub struct SafeVmAssertion;

impl SafeVmAssertion {
    /// Assert execution state with proper error handling
    pub fn assert_execution_state<T: fmt::Debug>(
        state: T,
        expected: bool,
        context: &str,
    ) -> VmResult<()> {
        if !expected {
            return Err(VmError::ExecutionHalted {
                reason: format!("Execution failed in {}: state={:?}", context, state),
            });
        }
        Ok(())
    }

    /// Validate jump operation
    pub fn validate_jump_operation(op: OpCode) -> VmResult<()> {
        match op {
            OpCode::JMP | OpCode::JMPIF | OpCode::JMPIFNOT | OpCode::CALL => Ok(()),
            _ => Err(VmError::InvalidOperation {
                operation: format!("Jump operation"),
                reason: format!("Invalid jump operation: {:?}", op),
            }),
        }
    }

    /// Validate syscall API length
    pub fn validate_syscall_api_length(api_bytes_len: usize) -> VmResult<()> {
        if api_bytes_len > 252 {
            return Err(VmError::InvalidOperation {
                operation: "Syscall".to_string(),
                reason: format!("Syscall API is too long: {} bytes (max 252)", api_bytes_len),
            });
        }
        Ok(())
    }
}

/// Safe memory operations for VM
pub struct SafeMemoryOps;

impl SafeMemoryOps {
    /// Safe memory copy with bounds checking
    pub fn safe_copy(src: &[u8], dst: &mut [u8], offset: usize, len: usize) -> VmResult<()> {
        // Validate source bounds
        if src.len() < len {
            return Err(VmError::Overflow {
                operation: format!("Buffer copy: source too small ({} < {})", src.len(), len),
            });
        }

        // Validate destination bounds
        if dst.len() < offset + len {
            return Err(VmError::Overflow {
                operation: format!(
                    "Buffer copy: destination too small ({} < {})",
                    dst.len(),
                    offset + len
                ),
            });
        }

        // Safe copy using standard library
        dst[offset..offset + len].copy_from_slice(&src[..len]);
        Ok(())
    }

    /// Safe stack allocation with size limits
    pub fn safe_stack_alloc(size: usize, max_size: usize) -> VmResult<Vec<u8>> {
        if size > max_size {
            return Err(VmError::StackOverflow { max_size });
        }

        // Use try_reserve to handle allocation failures
        let mut vec = Vec::new();
        vec.try_reserve(size).map_err(|e| VmError::Parse {
            message: format!("Failed to allocate stack: {}", e),
        })?;
        vec.resize(size, 0);
        Ok(vec)
    }
}

/// Safe script builder operations
pub struct SafeScriptBuilder;

impl SafeScriptBuilder {
    /// Build jump instruction safely
    pub fn build_jump_safely(op: OpCode, offset: i32, buffer: &mut Vec<u8>) -> VmResult<()> {
        // Validate operation
        SafeVmAssertion::validate_jump_operation(op)?;

        // Add opcode
        buffer.push(op as u8);

        // Add offset (little-endian)
        buffer.extend_from_slice(&offset.to_le_bytes());

        Ok(())
    }

    /// Build syscall safely
    pub fn build_syscall_safely(api: &str, buffer: &mut Vec<u8>) -> VmResult<()> {
        let api_bytes = api.as_bytes();

        // Validate API length
        SafeVmAssertion::validate_syscall_api_length(api_bytes.len())?;

        // Add syscall opcode
        buffer.push(OpCode::SYSCALL as u8);

        // Add API length
        buffer.push(api_bytes.len() as u8);

        // Add API string
        buffer.extend_from_slice(api_bytes);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_vm_assertion() {
        // Test successful assertion
        let result = SafeVmAssertion::assert_execution_state("success", true, "test context");
        assert!(result.is_ok());

        // Test failed assertion
        let result = SafeVmAssertion::assert_execution_state("failure", false, "test context");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_jump_operation() {
        // Valid operations
        assert!(SafeVmAssertion::validate_jump_operation(OpCode::JMP).is_ok());
        assert!(SafeVmAssertion::validate_jump_operation(OpCode::JMPIF).is_ok());
        assert!(SafeVmAssertion::validate_jump_operation(OpCode::JMPIFNOT).is_ok());
        assert!(SafeVmAssertion::validate_jump_operation(OpCode::CALL).is_ok());

        // Invalid operation
        assert!(SafeVmAssertion::validate_jump_operation(OpCode::NOP).is_err());
    }

    #[test]
    fn test_safe_memory_copy() {
        let src = vec![1, 2, 3, 4, 5];
        let mut dst = vec![0; 10];

        // Successful copy
        let result = SafeMemoryOps::safe_copy(&src, &mut dst, 2, 3);
        assert!(result.is_ok());
        assert_eq!(&dst[2..5], &[1, 2, 3]);

        // Source too small
        let result = SafeMemoryOps::safe_copy(&src, &mut dst, 0, 10);
        assert!(result.is_err());

        // Destination too small
        let result = SafeMemoryOps::safe_copy(&src, &mut dst, 8, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_stack_alloc() {
        // Successful allocation
        let result = SafeMemoryOps::safe_stack_alloc(100, 1000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 100);

        // Allocation too large
        let result = SafeMemoryOps::safe_stack_alloc(2000, 1000);
        assert!(result.is_err());
    }
}

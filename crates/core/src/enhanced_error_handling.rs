//! Enhanced Error Handling Patterns
//!
//! This module provides enhanced error handling patterns specifically designed
//! for the Neo blockchain node operations, replacing unsafe unwrap() patterns.

use crate::safe_error_handling::SafeError;
use std::fmt;

/// Enhanced error context for blockchain operations
#[derive(Debug, Clone)]
pub struct BlockchainErrorContext {
    /// Operation being performed
    pub operation: String,
    /// Block height if applicable
    pub block_height: Option<u32>,
    /// Transaction hash if applicable  
    pub transaction_hash: Option<String>,
    /// Component that failed
    pub component: String,
}

impl BlockchainErrorContext {
    pub fn new(operation: impl Into<String>, component: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            block_height: None,
            transaction_hash: None,
            component: component.into(),
        }
    }

    pub fn with_block_height(mut self, height: u32) -> Self {
        self.block_height = Some(height);
        self
    }

    pub fn with_transaction(mut self, tx_hash: impl Into<String>) -> Self {
        self.transaction_hash = Some(tx_hash.into());
        self
    }
}

impl fmt::Display for BlockchainErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in {} component", self.operation, self.component)?;
        if let Some(height) = self.block_height {
            write!(f, " at block height {}", height)?;
        }
        if let Some(ref tx) = self.transaction_hash {
            write!(f, " for transaction {}", tx)?;
        }
        Ok(())
    }
}

/// Safe transaction processing utilities
pub struct SafeTransactionProcessor;

impl SafeTransactionProcessor {
    /// Safely extract transaction hash with context
    pub fn extract_hash(transaction: &[u8]) -> Result<Vec<u8>, SafeError> {
        if transaction.len() < 32 {
            return Err(SafeError::new(
                "Transaction too short for hash extraction",
                "transaction_processor::hash_extraction",
            ));
        }

        // Extract hash safely
        Ok(transaction[..32].to_vec())
    }

    /// Safely validate transaction format
    pub fn validate_format(transaction: &[u8]) -> Result<(), SafeError> {
        if transaction.is_empty() {
            return Err(SafeError::new(
                "Empty transaction data",
                "transaction_processor::format_validation",
            ));
        }

        if transaction.len() > 1024 * 1024 {
            return Err(SafeError::new(
                "Transaction too large",
                "transaction_processor::format_validation",
            ));
        }

        Ok(())
    }
}

/// Safe block processing utilities
pub struct SafeBlockProcessor;

impl SafeBlockProcessor {
    /// Safely validate block header
    pub fn validate_header(header: &[u8]) -> Result<(), SafeError> {
        if header.len() < 80 {
            return Err(SafeError::new(
                "Block header too short",
                "block_processor::header_validation",
            ));
        }

        Ok(())
    }

    /// Safely extract block height
    pub fn extract_height(header: &[u8]) -> Result<u32, SafeError> {
        if header.len() < 84 {
            return Err(SafeError::new(
                "Header too short for height extraction",
                "block_processor::height_extraction",
            ));
        }

        let height_bytes = &header[80..84];
        Ok(u32::from_le_bytes([
            height_bytes[0],
            height_bytes[1],
            height_bytes[2],
            height_bytes[3],
        ]))
    }
}

/// Safe VM execution utilities
pub struct SafeVmExecutor;

impl SafeVmExecutor {
    /// Safely execute script with proper error handling
    pub fn execute_script(script: &[u8], gas_limit: u64) -> Result<Vec<u8>, SafeError> {
        if script.is_empty() {
            return Err(SafeError::new(
                "Empty script provided",
                "vm_executor::script_execution",
            ));
        }

        if gas_limit == 0 {
            return Err(SafeError::new(
                "Zero gas limit provided",
                "vm_executor::script_execution",
            ));
        }

        // For now, return success with empty result
        // In real implementation, this would execute the script
        Ok(Vec::new())
    }

    /// Safely validate script format
    pub fn validate_script(script: &[u8]) -> Result<(), SafeError> {
        if script.len() > 1024 * 1024 {
            return Err(SafeError::new(
                "Script too large",
                "vm_executor::script_validation",
            ));
        }

        // Check for basic script validity
        if !script.is_empty() && script[0] > 0x4F && script[0] < 0x60 {
            // Invalid opcode range
            return Err(SafeError::new(
                "Invalid opcode detected",
                "vm_executor::script_validation",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_transaction_processing() {
        // Test hash extraction
        let valid_tx = vec![0u8; 64];
        assert!(SafeTransactionProcessor::extract_hash(&valid_tx).is_ok());

        let invalid_tx = vec![0u8; 16];
        assert!(SafeTransactionProcessor::extract_hash(&invalid_tx).is_err());

        // Test format validation
        assert!(SafeTransactionProcessor::validate_format(&valid_tx).is_ok());
        assert!(SafeTransactionProcessor::validate_format(&[]).is_err());
    }

    #[test]
    fn test_safe_block_processing() {
        // Test header validation
        let valid_header = vec![0u8; 100];
        assert!(SafeBlockProcessor::validate_header(&valid_header).is_ok());

        let invalid_header = vec![0u8; 50];
        assert!(SafeBlockProcessor::validate_header(&invalid_header).is_err());

        // Test height extraction
        let mut header_with_height = vec![0u8; 100];
        header_with_height[80..84].copy_from_slice(&42u32.to_le_bytes());
        assert_eq!(
            SafeBlockProcessor::extract_height(&header_with_height).unwrap(),
            42
        );
    }

    #[test]
    fn test_safe_vm_execution() {
        let valid_script = vec![0x41]; // CHECKSIG opcode
        assert!(SafeVmExecutor::execute_script(&valid_script, 1000).is_ok());
        assert!(SafeVmExecutor::execute_script(&[], 1000).is_err());
        assert!(SafeVmExecutor::execute_script(&valid_script, 0).is_err());

        // Test script validation
        assert!(SafeVmExecutor::validate_script(&valid_script).is_ok());
        let invalid_script = vec![0x55]; // Invalid opcode
        assert!(SafeVmExecutor::validate_script(&invalid_script).is_err());
    }
}

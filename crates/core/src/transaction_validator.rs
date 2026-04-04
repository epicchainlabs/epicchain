//! Transaction validation module
//!
//! This module provides comprehensive validation for blockchain transactions,
//! ensuring data integrity, security, and protocol compliance.

use crate::{CoreError, CoreResult, UInt160};
use std::collections::HashSet;

/// Maximum transaction size in bytes (1MB)
const MAX_TRANSACTION_SIZE: usize = 1024 * 1024;

/// Maximum number of attributes per transaction
const MAX_TRANSACTION_ATTRIBUTES: usize = 16;

/// Maximum script length in bytes
const MAX_SCRIPT_LENGTH: usize = 65536;

/// Maximum number of signers per transaction
const MAX_SIGNERS: usize = 16;

/// Transaction validation rules
#[derive(Debug, Clone)]
pub struct TransactionValidator {
    /// Maximum allowed transaction size
    max_size: usize,
    /// Maximum allowed attributes
    max_attributes: usize,
    /// Maximum script length
    max_script_length: usize,
    /// Whether to enforce strict validation
    strict_mode: bool,
    /// Blocked addresses (e.g., sanctioned addresses)
    blocked_addresses: HashSet<UInt160>,
}

impl Default for TransactionValidator {
    fn default() -> Self {
        Self {
            max_size: MAX_TRANSACTION_SIZE,
            max_attributes: MAX_TRANSACTION_ATTRIBUTES,
            max_script_length: MAX_SCRIPT_LENGTH,
            strict_mode: true,
            blocked_addresses: HashSet::new(),
        }
    }
}

impl TransactionValidator {
    /// Create a new transaction validator
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a validator with custom limits
    pub fn with_limits(max_size: usize, max_attributes: usize, max_script_length: usize) -> Self {
        Self {
            max_size,
            max_attributes,
            max_script_length,
            ..Default::default()
        }
    }

    /// Set strict validation mode
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Add a blocked address
    pub fn block_address(&mut self, address: UInt160) {
        self.blocked_addresses.insert(address);
    }

    /// Validate transaction size
    pub fn validate_size(&self, size: usize) -> CoreResult<()> {
        if size > self.max_size {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Transaction size {} exceeds maximum {}",
                    size, self.max_size
                ),
            });
        }

        if self.strict_mode && size == 0 {
            return Err(CoreError::ValidationFailed {
                reason: "Transaction size cannot be zero".to_string(),
            });
        }

        Ok(())
    }

    /// Validate transaction attributes
    pub fn validate_attributes(&self, attributes_count: usize) -> CoreResult<()> {
        if attributes_count > self.max_attributes {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Attribute count {} exceeds maximum {}",
                    attributes_count, self.max_attributes
                ),
            });
        }
        Ok(())
    }

    /// Validate script
    pub fn validate_script(&self, script: &[u8]) -> CoreResult<()> {
        if script.is_empty() {
            return Err(CoreError::ValidationFailed {
                reason: "Script cannot be empty".to_string(),
            });
        }

        if script.len() > self.max_script_length {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Script length {} exceeds maximum {}",
                    script.len(),
                    self.max_script_length
                ),
            });
        }

        // Additional script validation could be added here
        // e.g., checking for valid opcodes, script structure, etc.

        Ok(())
    }

    /// Validate transaction signers
    pub fn validate_signers(&self, signers: &[UInt160]) -> CoreResult<()> {
        if signers.is_empty() {
            return Err(CoreError::ValidationFailed {
                reason: "Transaction must have at least one signer".to_string(),
            });
        }

        if signers.len() > MAX_SIGNERS {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Signer count {} exceeds maximum {}",
                    signers.len(),
                    MAX_SIGNERS
                ),
            });
        }

        // Check for blocked addresses
        for signer in signers {
            if self.blocked_addresses.contains(signer) {
                return Err(CoreError::ValidationFailed {
                    reason: format!("Signer {} is blocked", signer),
                });
            }
        }

        // Check for duplicate signers
        let mut seen = HashSet::new();
        for signer in signers {
            if !seen.insert(signer) {
                return Err(CoreError::ValidationFailed {
                    reason: format!("Duplicate signer: {}", signer),
                });
            }
        }

        Ok(())
    }

    /// Validate transaction fees
    pub fn validate_fees(&self, network_fee: i64, system_fee: i64) -> CoreResult<()> {
        if network_fee < 0 {
            return Err(CoreError::ValidationFailed {
                reason: format!("Network fee cannot be negative: {}", network_fee),
            });
        }

        if system_fee < 0 {
            return Err(CoreError::ValidationFailed {
                reason: format!("System fee cannot be negative: {}", system_fee),
            });
        }

        // Could add maximum fee validation to prevent accidents
        const MAX_FEE: i64 = 100_000_000_000; // 1000 GAS
        if self.strict_mode {
            if network_fee > MAX_FEE {
                return Err(CoreError::ValidationFailed {
                    reason: format!("Network fee {} exceeds maximum", network_fee),
                });
            }

            if system_fee > MAX_FEE {
                return Err(CoreError::ValidationFailed {
                    reason: format!("System fee {} exceeds maximum", system_fee),
                });
            }
        }

        Ok(())
    }

    /// Validate nonce for uniqueness
    pub fn validate_nonce(&self, nonce: u32) -> CoreResult<()> {
        // In strict mode, ensure nonce is not zero (likely uninitialized)
        if self.strict_mode && nonce == 0 {
            return Err(CoreError::ValidationFailed {
                reason: "Nonce should not be zero".to_string(),
            });
        }

        // Additional nonce validation could check against recent transactions
        // to prevent replay attacks

        Ok(())
    }

    /// Validate transaction timestamp
    pub fn validate_timestamp(
        &self,
        valid_until_block: u32,
        current_height: u32,
    ) -> CoreResult<()> {
        if valid_until_block <= current_height {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Transaction expired: valid_until {} <= current height {}",
                    valid_until_block, current_height
                ),
            });
        }

        // Prevent transactions that are valid for too long
        const MAX_VALIDITY_BLOCKS: u32 = 5760; // ~24 hours at 15s blocks
        if self.strict_mode && valid_until_block > current_height + MAX_VALIDITY_BLOCKS {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Transaction validity period too long: {} blocks",
                    valid_until_block - current_height
                ),
            });
        }

        Ok(())
    }
}

/// Input validation for network messages
pub struct InputValidator {
    /// Maximum allowed input size
    max_input_size: usize,
    /// Allow empty inputs
    allow_empty: bool,
}

impl InputValidator {
    /// Create a new input validator
    pub fn new(max_input_size: usize) -> Self {
        Self {
            max_input_size,
            allow_empty: false,
        }
    }

    /// Validate input data
    pub fn validate(&self, input: &[u8]) -> CoreResult<()> {
        if !self.allow_empty && input.is_empty() {
            return Err(CoreError::ValidationFailed {
                reason: "Input cannot be empty".to_string(),
            });
        }

        if input.len() > self.max_input_size {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Input size {} exceeds maximum {}",
                    input.len(),
                    self.max_input_size
                ),
            });
        }

        Ok(())
    }

    /// Validate string input
    pub fn validate_string(&self, input: &str) -> CoreResult<()> {
        self.validate(input.as_bytes())?;

        // Check for valid UTF-8 (already guaranteed by &str)
        // Could add additional checks for control characters, etc.

        Ok(())
    }

    /// Validate hash input
    pub fn validate_hash(&self, hash: &[u8]) -> CoreResult<()> {
        const HASH_SIZE: usize = 32;

        if hash.len() != HASH_SIZE {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Invalid hash size: expected {}, got {}",
                    HASH_SIZE,
                    hash.len()
                ),
            });
        }

        // Check that hash is not all zeros (likely uninitialized)
        if hash.iter().all(|&b| b == 0) {
            return Err(CoreError::ValidationFailed {
                reason: "Hash cannot be all zeros".to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_size_validation() {
        let validator = TransactionValidator::new();

        // Valid size
        assert!(validator.validate_size(1000).is_ok());

        // Too large
        assert!(validator.validate_size(MAX_TRANSACTION_SIZE + 1).is_err());

        // Zero size in strict mode
        assert!(validator.validate_size(0).is_err());
    }

    #[test]
    fn test_signer_validation() {
        let mut validator = TransactionValidator::new();

        // Valid signers
        let signers = vec![UInt160::from([1u8; 20]), UInt160::from([2u8; 20])];
        assert!(validator.validate_signers(&signers).is_ok());

        // Empty signers
        assert!(validator.validate_signers(&[]).is_err());

        // Duplicate signers
        let duplicate_signers = vec![UInt160::from([1u8; 20]), UInt160::from([1u8; 20])];
        assert!(validator.validate_signers(&duplicate_signers).is_err());

        // Blocked address
        let blocked = UInt160::from([3u8; 20]);
        validator.block_address(blocked);
        let signers_with_blocked = vec![UInt160::from([1u8; 20]), blocked];
        assert!(validator.validate_signers(&signers_with_blocked).is_err());
    }

    #[test]
    fn test_fee_validation() {
        let validator = TransactionValidator::new();

        // Valid fees
        assert!(validator.validate_fees(1000, 2000).is_ok());

        // Negative fees
        assert!(validator.validate_fees(-1, 1000).is_err());
        assert!(validator.validate_fees(1000, -1).is_err());

        // Excessive fees in strict mode
        assert!(validator.validate_fees(200_000_000_000, 1000).is_err());
    }

    #[test]
    fn test_input_validation() {
        let validator = InputValidator::new(1024);

        // Valid input
        assert!(validator.validate(b"valid input").is_ok());

        // Empty input
        assert!(validator.validate(b"").is_err());

        // Too large input
        let large_input = vec![0u8; 1025];
        assert!(validator.validate(&large_input).is_err());
    }

    #[test]
    fn test_hash_validation() {
        let validator = InputValidator::new(1024);

        // Valid hash
        let valid_hash = [1u8; 32];
        assert!(validator.validate_hash(&valid_hash).is_ok());

        // Invalid size
        let invalid_hash = [1u8; 31];
        assert!(validator.validate_hash(&invalid_hash).is_err());

        // All zeros
        let zero_hash = [0u8; 32];
        assert!(validator.validate_hash(&zero_hash).is_err());
    }
}

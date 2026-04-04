//! Input validation framework for Neo-RS
//!
//! This module provides comprehensive input validation to prevent security vulnerabilities
//! and ensure data integrity across the system.

use crate::error::CoreError;
use std::fmt::Display;

/// Validation result type
pub type ValidationResult<T> = Result<T, CoreError>;

/// Trait for validatable types
pub trait Validatable {
    /// Validate the instance and return an error if invalid
    fn validate(&self) -> ValidationResult<()>;

    /// Check if the instance is valid
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

/// Input validator with configurable rules
pub struct Validator {
    /// Maximum allowed size for inputs
    pub max_size: usize,
    /// Allow empty inputs
    pub allow_empty: bool,
    /// Allow null bytes in strings
    pub allow_null_bytes: bool,
}

impl Default for Validator {
    fn default() -> Self {
        Self {
            max_size: 1024 * 1024, // 1MB default
            allow_empty: false,
            allow_null_bytes: false,
        }
    }
}

impl Validator {
    /// Create a new validator with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a strict validator for security-critical operations
    pub fn strict() -> Self {
        Self {
            max_size: 64 * 1024, // 64KB
            allow_empty: false,
            allow_null_bytes: false,
        }
    }

    /// Validate a byte array
    pub fn validate_bytes(&self, _data: &[u8]) -> ValidationResult<()> {
        if !self.allow_empty && data.is_empty() {
            return Err(CoreError::ValidationFailed {
                reason: "Empty data not allowed".to_string(),
            });
        }

        if data.len() > self.max_size {
            return Err(CoreError::ValidationFailed {
                reason: format!("Data size {} exceeds maximum {}", data.len(), self.max_size),
            });
        }

        if !self.allow_null_bytes && data.contains(&0) {
            return Err(CoreError::ValidationFailed {
                reason: "Null bytes not allowed".to_string(),
            });
        }

        Ok(())
    }

    /// Validate a string
    pub fn validate_string(&self, _s: &str) -> ValidationResult<()> {
        self.validate_bytes(s.as_bytes())?;

        // Additional string-specific validation
        if s.chars()
            .any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
        {
            return Err(CoreError::ValidationFailed {
                reason: "Control characters not allowed".to_string(),
            });
        }

        Ok(())
    }

    /// Validate a number is within range
    pub fn validate_range<T>(&self, value: T, min: T, _max: T) -> ValidationResult<()>
    where
        T: PartialOrd + Display,
    {
        if value < min || value > max {
            return Err(CoreError::ValidationFailed {
                reason: format!("Value {} out of range [{}, {}]", value, min, max),
            });
        }
        Ok(())
    }

    /// Validate an address (20 bytes)
    pub fn validate_address(&self, _address: &[u8]) -> ValidationResult<()> {
        if address.len() != 20 {
            return Err(CoreError::ValidationFailed {
                reason: format!("Invalid address length: {} (expected 20)", address.len()),
            });
        }
        Ok(())
    }

    /// Validate a hash (32 bytes)
    pub fn validate_hash(&self, _hash: &[u8]) -> ValidationResult<()> {
        if hash.len() != 32 {
            return Err(CoreError::ValidationFailed {
                reason: format!("Invalid hash length: {} (expected 32)", hash.len()),
            });
        }
        Ok(())
    }

    /// Validate a public key (33 or 65 bytes)
    pub fn validate_public_key(&self, _key: &[u8]) -> ValidationResult<()> {
        if key.len() != 33 && key.len() != 65 {
            return Err(CoreError::ValidationFailed {
                reason: format!(
                    "Invalid public key length: {} (expected 33 or 65)",
                    key.len()
                ),
            });
        }
        Ok(())
    }

    /// Validate a signature (64 bytes)
    pub fn validate_signature(&self, _sig: &[u8]) -> ValidationResult<()> {
        if sig.len() != 64 {
            return Err(CoreError::ValidationFailed {
                reason: format!("Invalid signature length: {} (expected 64)", sig.len()),
            });
        }
        Ok(())
    }
}

/// Builder pattern for validators
pub struct ValidatorBuilder {
    validator: Validator,
}

impl Default for ValidatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidatorBuilder {
    /// Create a new validator builder
    pub fn new() -> Self {
        Self {
            validator: Validator::default(),
        }
    }

    /// Set maximum size
    pub fn max_size(mut self, _size: usize) -> Self {
        self.validator.max_size = size;
        self
    }

    /// Allow empty inputs
    pub fn allow_empty(mut self, _allow: bool) -> Self {
        self.validator.allow_empty = allow;
        self
    }

    /// Allow null bytes
    pub fn allow_null_bytes(mut self, _allow: bool) -> Self {
        self.validator.allow_null_bytes = allow;
        self
    }

    /// Build the validator
    pub fn build(self) -> Validator {
        self.validator
    }
}

/// Sanitize input by removing dangerous characters
pub fn sanitize_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
        .collect()
}

/// Truncate input to maximum length
pub fn truncate_string(input: &str, _max_len: usize) -> String {
    if input.len() <= max_len {
        input.to_string()
    } else {
        format!("{}...", &input[..max_len - 3])
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_bytes() {
        let _validator = Validator::new();

        // Valid data
        assert!(validator.validate_bytes(b"hello").is_ok());

        // Empty data
        assert!(validator.validate_bytes(b"").is_err());

        // With null bytes
        assert!(validator.validate_bytes(b"hello\0world").is_err());
    }

    #[test]
    fn test_validator_string() {
        let _validator = Validator::new();

        // Valid string
        assert!(validator.validate_string("hello world").is_ok());

        // With control characters
        assert!(validator.validate_string("hello\x00world").is_err());
    }

    #[test]
    fn test_validator_range() {
        let _validator = Validator::new();

        assert!(validator.validate_range(5, 0, 10).is_ok());
        assert!(validator.validate_range(15, 0, 10).is_err());
        assert!(validator.validate_range(-5, 0, 10).is_err());
    }

    #[test]
    fn test_sanitize_string() {
        assert_eq!(sanitize_string("hello\x00world"), "helloworld");
        assert_eq!(sanitize_string("line1\nline2"), "line1\nline2");
    }
}

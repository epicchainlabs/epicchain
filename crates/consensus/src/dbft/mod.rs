//! Modular dBFT (delegated Byzantine Fault Tolerance) consensus implementation.
//!
//! This module provides a clean, maintainable implementation of the dBFT consensus algorithm
//! split into logical components for better organization and testing.

pub mod config;
pub mod engine;
pub mod message_handler;
pub mod state;

pub use config::DbftConfig;
pub use engine::DbftEngine;
pub use message_handler::{MessageHandleResult, MessageHandler};
use neo_config::{MAX_BLOCK_SIZE, MILLISECONDS_PER_BLOCK};
pub use state::{DbftEvent, DbftState, DbftStats};

/// dBFT module version
pub const VERSION: &str = "1.0.0";

/// dBFT protocol constants
pub mod constants {
    /// Maximum number of validators supported
    pub const MAX_VALIDATORS: usize = 21;

    /// Minimum number of validators required
    pub const MIN_VALIDATORS: usize = 4;

    /// Default block time in milliseconds
    pub const DEFAULT_BLOCK_TIME_MS: u64 = super::MILLISECONDS_PER_BLOCK;

    /// Maximum view number
    pub const MAX_VIEW_NUMBER: u32 = 255;

    /// Maximum concurrent rounds
    pub const MAX_CONCURRENT_ROUNDS: usize = 10;

    /// Default message timeout in milliseconds
    pub const DEFAULT_MESSAGE_TIMEOUT_MS: u64 = 5000;

    /// Maximum message size in bytes
    pub const MAX_MESSAGE_SIZE: usize = super::MAX_BLOCK_SIZE;

    /// Byzantine fault tolerance threshold (2/3 + 1)
    pub fn byzantine_threshold(validator_count: usize) -> usize {
        (validator_count * 2 / 3) + 1
    }

    /// Calculates the required consensus count
    pub fn required_consensus_count(validator_count: usize) -> usize {
        byzantine_threshold(validator_count)
    }
}

/// dBFT utility functions
pub mod utils {
    use super::constants;

    /// Validates a validator count
    pub fn validate_validator_count(count: usize) -> crate::Result<()> {
        if count < constants::MIN_VALIDATORS {
            return Err(crate::Error::InvalidConfig(format!(
                "Validator count {} is below minimum {}",
                count,
                constants::MIN_VALIDATORS
            )));
        }

        if count > constants::MAX_VALIDATORS {
            return Err(crate::Error::InvalidConfig(format!(
                "Validator count {} exceeds maximum {}",
                count,
                constants::MAX_VALIDATORS
            )));
        }

        Ok(())
    }

    /// Calculates the primary validator index for a given view
    pub fn calculate_primary_index(view_number: u32, validator_count: usize) -> usize {
        (view_number as usize) % validator_count
    }

    /// Checks if a validator index is valid
    pub fn is_valid_validator_index(index: usize, validator_count: usize) -> bool {
        index < validator_count
    }

    /// Calculates the next view number
    pub fn next_view_number(current_view: u32) -> u32 {
        if current_view >= constants::MAX_VIEW_NUMBER {
            0 // Wrap around
        } else {
            current_view + 1
        }
    }

    /// Calculates timeout for a given view (exponential backoff)
    pub fn calculate_view_timeout(base_timeout_ms: u64, view_number: u32) -> u64 {
        let multiplier = 1.5_f64.powi(view_number.min(10) as i32); // Cap at view 10
        (base_timeout_ms as f64 * multiplier) as u64
    }
}

/// dBFT error types specific to this module
#[derive(Debug, thiserror::Error)]
pub enum DbftError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Invalid state transition: {from} -> {to}")]
    InvalidStateTransition { from: DbftState, to: DbftState },

    #[error("Message handling error: {0}")]
    MessageHandling(String),

    #[error("Consensus timeout: {timer_type:?}")]
    ConsensusTimeout {
        timer_type: crate::context::TimerType,
    },

    #[error("View change failed: {reason}")]
    ViewChangeFailed { reason: String },

    #[error("Recovery failed: {reason}")]
    RecoveryFailed { reason: String },
}

impl From<DbftError> for crate::Error {
    fn from(err: DbftError) -> Self {
        match err {
            DbftError::InvalidConfig(msg) => crate::Error::InvalidConfig(msg),
            DbftError::InvalidStateTransition { from, to } => {
                crate::Error::InvalidState(format!("Invalid state transition: {} -> {}", from, to))
            }
            DbftError::MessageHandling(msg) => crate::Error::InvalidMessage(msg),
            DbftError::ConsensusTimeout { timer_type } => {
                crate::Error::Timeout(format!("Consensus timeout: {:?}", timer_type))
            }
            DbftError::ViewChangeFailed { reason } => crate::Error::ViewChange(reason),
            DbftError::RecoveryFailed { reason } => crate::Error::Recovery(reason),
        }
    }
}

/// dBFT result type
pub type DbftResult<T> = Result<T, DbftError>;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_constants() {
        assert_eq!(constants::MIN_VALIDATORS, 4);
        assert_eq!(constants::MAX_VALIDATORS, 21);
        assert_eq!(constants::byzantine_threshold(7), 5);
        assert_eq!(constants::required_consensus_count(7), 5);
    }

    #[test]
    fn test_utils() {
        // Test validator count validation
        assert!(utils::validate_validator_count(7).is_ok());
        assert!(utils::validate_validator_count(3).is_err());
        assert!(utils::validate_validator_count(25).is_err());

        // Test primary index calculation
        assert_eq!(utils::calculate_primary_index(0, 7), 0);
        assert_eq!(utils::calculate_primary_index(1, 7), 1);
        assert_eq!(utils::calculate_primary_index(7, 7), 0); // Wrap around

        // Test validator index validation
        assert!(utils::is_valid_validator_index(0, 7));
        assert!(utils::is_valid_validator_index(6, 7));
        assert!(!utils::is_valid_validator_index(7, 7));

        // Test view number calculation
        assert_eq!(utils::next_view_number(0), 1);
        assert_eq!(utils::next_view_number(254), 255);
        assert_eq!(utils::next_view_number(255), 0); // Wrap around

        // Test timeout calculation
        let base_timeout = 1000;
        assert_eq!(utils::calculate_view_timeout(base_timeout, 0), 1000);
        assert!(utils::calculate_view_timeout(base_timeout, 1) > 1000);
        assert!(
            utils::calculate_view_timeout(base_timeout, 2)
                > utils::calculate_view_timeout(base_timeout, 1)
        );
    }

    #[test]
    fn test_error_conversion() {
        let dbft_error = DbftError::InvalidConfig("test".to_string());
        let consensus_error: crate::Error = dbft_error.into();

        match consensus_error {
            crate::Error::InvalidConfig(msg) => assert_eq!(msg, "test"),
            other => assert!(false, "Unexpected error type: {:?}", other),
        }
    }
}

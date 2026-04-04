//! dBFT configuration module.
//!
//! This module contains configuration structures and defaults for the dBFT consensus engine.

use crate::ConsensusConfig;
use neo_config::{
    ADDRESS_SIZE, MAX_BLOCK_SIZE, MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK,
    MILLISECONDS_PER_BLOCK,
};
use neo_core::constants::DEFAULT_TIMEOUT_MS;
use serde::{Deserialize, Serialize};
/// dBFT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbftConfig {
    /// Base consensus configuration
    pub consensus_config: ConsensusConfig,
    /// Enable fast view change
    pub enable_fast_view_change: bool,
    /// Maximum concurrent rounds
    pub max_concurrent_rounds: usize,
    /// Enable message batching
    pub enable_message_batching: bool,
    /// Message batch size
    pub message_batch_size: usize,
    /// Enable signature aggregation
    pub enable_signature_aggregation: bool,
    /// Block time target in milliseconds
    pub block_time_target_ms: u64,
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Timeout multiplier for view changes
    pub view_change_timeout_multiplier: f64,
    /// Enable recovery mode
    pub enable_recovery: bool,
    /// Recovery timeout in milliseconds
    pub recovery_timeout_ms: u64,
}

impl Default for DbftConfig {
    fn default() -> Self {
        Self {
            consensus_config: ConsensusConfig::default(),
            enable_fast_view_change: true,
            max_concurrent_rounds: 3,
            enable_message_batching: true,
            message_batch_size: 10,
            enable_signature_aggregation: false, // Implementation provided yet
            block_time_target_ms: MILLISECONDS_PER_BLOCK, // SECONDS_PER_BLOCK seconds
            max_block_size: MAX_BLOCK_SIZE,
            max_transactions_per_block: MAX_TRANSACTIONS_PER_BLOCK,
            view_change_timeout_multiplier: 2.0,
            enable_recovery: true,
            recovery_timeout_ms: 30000, // 30 seconds
        }
    }
}

impl DbftConfig {
    /// Creates a new dBFT configuration with custom settings
    pub fn new(
        consensus_config: ConsensusConfig,
        enable_fast_view_change: bool,
        max_concurrent_rounds: usize,
    ) -> Self {
        Self {
            consensus_config,
            enable_fast_view_change,
            max_concurrent_rounds,
            ..Default::default()
        }
    }

    /// Creates a configuration optimized for testing
    pub fn for_testing() -> Self {
        Self {
            block_time_target_ms: 1000, // 1 second for faster tests
            max_transactions_per_block: 100,
            view_change_timeout_multiplier: 1.5,
            recovery_timeout_ms: DEFAULT_TIMEOUT_MS, // 5 seconds
            ..Default::default()
        }
    }

    /// Creates a configuration optimized for production
    pub fn for_production() -> Self {
        Self {
            enable_fast_view_change: true,
            max_concurrent_rounds: 5,
            enable_message_batching: true,
            message_batch_size: ADDRESS_SIZE,
            block_time_target_ms: MILLISECONDS_PER_BLOCK, // SECONDS_PER_BLOCK seconds
            max_block_size: MAX_BLOCK_SIZE,
            max_transactions_per_block: MAX_SCRIPT_SIZE,
            view_change_timeout_multiplier: 2.5,
            enable_recovery: true,
            recovery_timeout_ms: 60000, // 60 seconds
            ..Default::default()
        }
    }

    /// Validates the configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Validate base consensus config
        self.consensus_config.validate()?;

        // Validate dBFT-specific settings
        if self.max_concurrent_rounds == 0 {
            return Err(crate::Error::InvalidConfig(
                "max_concurrent_rounds must be greater than 0".to_string(),
            ));
        }

        if self.message_batch_size == 0 {
            return Err(crate::Error::InvalidConfig(
                "message_batch_size must be greater than 0".to_string(),
            ));
        }

        if self.block_time_target_ms == 0 {
            return Err(crate::Error::InvalidConfig(
                "block_time_target_ms must be greater than 0".to_string(),
            ));
        }

        if self.max_block_size == 0 {
            return Err(crate::Error::InvalidConfig(
                "max_block_size must be greater than 0".to_string(),
            ));
        }

        if self.max_transactions_per_block == 0 {
            return Err(crate::Error::InvalidConfig(
                "max_transactions_per_block must be greater than 0".to_string(),
            ));
        }

        if self.view_change_timeout_multiplier <= 1.0 {
            return Err(crate::Error::InvalidConfig(
                "view_change_timeout_multiplier must be greater than 1.0".to_string(),
            ));
        }

        if self.recovery_timeout_ms == 0 {
            return Err(crate::Error::InvalidConfig(
                "recovery_timeout_ms must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Gets the timeout for a specific timer type based on configuration
    pub fn get_timeout_ms(&self, timer_type: crate::context::TimerType) -> u64 {
        match timer_type {
            crate::context::TimerType::PrepareRequest => self.block_time_target_ms,
            crate::context::TimerType::PrepareResponse => self.block_time_target_ms / 2,
            crate::context::TimerType::Commit => self.block_time_target_ms / 4,
            crate::context::TimerType::ViewChange => {
                (self.block_time_target_ms as f64 * self.view_change_timeout_multiplier) as u64
            }
            crate::context::TimerType::Recovery => self.recovery_timeout_ms,
        }
    }

    /// Checks if a feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        match feature {
            "fast_view_change" => self.enable_fast_view_change,
            "message_batching" => self.enable_message_batching,
            "signature_aggregation" => self.enable_signature_aggregation,
            "recovery" => self.enable_recovery,
            _ => false,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ConsensusContext, ConsensusMessage, ConsensusState};

    #[test]
    fn test_default_config() {
        let config = DbftConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.block_time_target_ms, MILLISECONDS_PER_BLOCK);
        assert_eq!(
            config.max_transactions_per_block,
            MAX_TRANSACTIONS_PER_BLOCK
        );
    }

    #[test]
    fn test_testing_config() {
        let config = DbftConfig::for_testing();
        assert!(config.validate().is_ok());
        assert_eq!(config.block_time_target_ms, 1000);
        assert_eq!(config.max_transactions_per_block, 100);
    }

    #[test]
    fn test_production_config() {
        let config = DbftConfig::for_production();
        assert!(config.validate().is_ok());
        assert_eq!(config.max_block_size, MAX_BLOCK_SIZE);
        assert_eq!(config.max_transactions_per_block, MAX_SCRIPT_SIZE);
    }

    #[test]
    fn test_timeout_calculation() {
        let config = DbftConfig::default();
        assert_eq!(
            config.get_timeout_ms(crate::context::TimerType::PrepareRequest),
            MILLISECONDS_PER_BLOCK
        );
        assert_eq!(
            config.get_timeout_ms(crate::context::TimerType::PrepareResponse),
            7500
        );
    }

    #[test]
    fn test_feature_flags() {
        let config = DbftConfig::default();
        assert!(config.is_feature_enabled("fast_view_change"));
        assert!(config.is_feature_enabled("message_batching"));
        assert!(!config.is_feature_enabled("signature_aggregation"));
        assert!(!config.is_feature_enabled("unknown_feature"));
    }
}

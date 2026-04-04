//! Validator management and selection.
//!
//! This module provides comprehensive validator management functionality,
//! including validator selection, rotation, and performance tracking.

use crate::{Error, NodeRole, Result, ViewNumber};
use neo_config::ADDRESS_SIZE;
use neo_core::UInt160;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    /// Minimum stake required to be a validator
    pub min_stake: u64,
    /// Maximum number of validators
    pub max_validators: usize,
    /// Validator rotation interval in blocks
    pub rotation_interval: u32,
    /// Enable validator performance tracking
    pub enable_performance_tracking: bool,
    /// Performance evaluation window in blocks
    pub performance_window: u32,
    /// Minimum performance score to remain validator
    pub min_performance_score: f64,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            min_stake: 1000_00000000, // 1000 NEO
            max_validators: 21,
            rotation_interval: 21600, // ~6 hours at 15s block time
            enable_performance_tracking: true,
            performance_window: 1000,
            min_performance_score: 0.8,
        }
    }
}

/// Validator information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Validator {
    /// Validator public key hash
    pub public_key_hash: UInt160,
    /// Validator public key
    pub public_key: Vec<u8>,
    /// Validator stake amount
    pub stake: u64,
    /// Validator index in the current set
    pub index: u8,
    /// Whether the validator is active
    pub active: bool,
    /// Registration block height
    pub registered_at: u32,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Performance statistics
    pub performance: ValidatorPerformance,
}

impl Validator {
    /// Creates a new validator
    pub fn new(
        public_key_hash: UInt160,
        public_key: Vec<u8>,
        stake: u64,
        index: u8,
        registered_at: u32,
    ) -> Self {
        Self {
            public_key_hash,
            public_key,
            stake,
            index,
            active: true,
            registered_at,
            last_activity: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            performance: ValidatorPerformance::default(),
        }
    }

    /// Updates the validator's last activity
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Checks if the validator is online (active recently)
    pub fn is_online(&self, timeout_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now - self.last_activity <= timeout_seconds
    }

    /// Gets the validator's performance score
    pub fn performance_score(&self) -> f64 {
        self.performance.calculate_score()
    }
}

/// Validator performance statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorPerformance {
    /// Total blocks proposed
    pub blocks_proposed: u32,
    /// Total blocks signed
    pub blocks_signed: u32,
    /// Total consensus rounds participated
    pub rounds_participated: u32,
    /// Total consensus rounds missed
    pub rounds_missed: u32,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Number of view changes initiated
    pub view_changes_initiated: u32,
    /// Uptime percentage
    pub uptime_percentage: f64,
}

impl Default for ValidatorPerformance {
    fn default() -> Self {
        Self {
            blocks_proposed: 0,
            blocks_signed: 0,
            rounds_participated: 0,
            rounds_missed: 0,
            avg_response_time_ms: 0.0,
            view_changes_initiated: 0,
            uptime_percentage: 100.0,
        }
    }
}

impl ValidatorPerformance {
    /// Calculates the overall performance score
    pub fn calculate_score(&self) -> f64 {
        let total_rounds = self.rounds_participated + self.rounds_missed;
        if total_rounds == 0 {
            return 1.0; // New validator gets benefit of doubt
        }

        let participation_rate = self.rounds_participated as f64 / total_rounds as f64;
        let signing_rate = if self.rounds_participated > 0 {
            self.blocks_signed as f64 / self.rounds_participated as f64
        } else {
            0.0
        };

        // Weighted score: 40% participation, 40% signing, ADDRESS_SIZE% uptime
        (participation_rate * 0.4) + (signing_rate * 0.4) + (self.uptime_percentage / 100.0 * 0.2)
    }

    /// Records a block proposal
    pub fn record_block_proposal(&mut self) {
        self.blocks_proposed += 1;
    }

    /// Records a block signature
    pub fn record_block_signature(&mut self) {
        self.blocks_signed += 1;
    }

    /// Records consensus participation
    pub fn record_participation(&mut self) {
        self.rounds_participated += 1;
    }

    /// Records a missed consensus round
    pub fn record_miss(&mut self) {
        self.rounds_missed += 1;
    }

    /// Records a view change initiation
    pub fn record_view_change(&mut self) {
        self.view_changes_initiated += 1;
    }

    /// Updates response time
    pub fn update_response_time(&mut self, response_time_ms: f64) {
        let total_responses = self.rounds_participated as f64;
        if total_responses > 0.0 {
            self.avg_response_time_ms = (self.avg_response_time_ms * (total_responses - 1.0)
                + response_time_ms)
                / total_responses;
        } else {
            self.avg_response_time_ms = response_time_ms;
        }
    }
}

/// Validator set representing the current active validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    /// List of validators
    pub validators: Vec<Validator>,
    /// Current block height
    pub block_height: u32,
    /// Set creation timestamp
    pub created_at: u64,
}

impl ValidatorSet {
    /// Creates a new validator set
    pub fn new(validators: Vec<Validator>, block_height: u32) -> Self {
        Self {
            validators,
            block_height,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Gets the number of validators
    pub fn len(&self) -> usize {
        self.validators.len()
    }

    /// Checks if the set is empty
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    /// Gets a validator by index
    pub fn get_validator(&self, index: usize) -> Option<&Validator> {
        self.validators.get(index)
    }

    /// Gets a validator by public key hash
    pub fn get_validator_by_hash(&self, hash: &UInt160) -> Option<&Validator> {
        self.validators.iter().find(|v| &v.public_key_hash == hash)
    }

    /// Gets the primary validator for a given view
    pub fn get_primary(&self, view: ViewNumber) -> Option<&Validator> {
        if self.validators.is_empty() {
            return None;
        }

        let index = (view.value() as usize) % self.validators.len();
        self.validators.get(index)
    }

    /// Gets all backup validators for a given view
    pub fn get_backups(&self, view: ViewNumber) -> Vec<&Validator> {
        if self.validators.is_empty() {
            return Vec::new();
        }

        let primary_index = (view.value() as usize) % self.validators.len();
        self.validators
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != primary_index)
            .map(|(_, v)| v)
            .collect()
    }

    /// Gets the role of a validator in the current view
    pub fn get_validator_role(&self, validator_hash: &UInt160, view: ViewNumber) -> NodeRole {
        if let Some(primary) = self.get_primary(view) {
            if &primary.public_key_hash == validator_hash {
                return NodeRole::Primary;
            }
        }

        if self.get_validator_by_hash(validator_hash).is_some() {
            NodeRole::Backup
        } else {
            NodeRole::Observer
        }
    }

    /// Checks if a validator is in the set
    pub fn contains_validator(&self, hash: &UInt160) -> bool {
        self.validators.iter().any(|v| &v.public_key_hash == hash)
    }

    /// Gets the required number of signatures for consensus
    pub fn required_signatures(&self) -> usize {
        let f = (self.validators.len() - 1) / 3; // Byzantine fault tolerance
        self.validators.len() - f
    }

    /// Validates the validator set
    pub fn validate(&self) -> Result<()> {
        if self.validators.len() < 4 {
            return Err(Error::InvalidValidator(
                "Validator set must have at least 4 validators".to_string(),
            ));
        }

        if (self.validators.len() - 1) % 3 != 0 {
            return Err(Error::InvalidValidator(
                "Validator count must be 3f+1 where f is the number of Byzantine nodes".to_string(),
            ));
        }

        let mut seen = HashSet::new();
        for validator in &self.validators {
            if !seen.insert(&validator.public_key_hash) {
                return Err(Error::InvalidValidator(
                    "Duplicate validator in set".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// Validator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorStats {
    /// Total number of validators
    pub total_validators: usize,
    /// Number of active validators
    pub active_validators: usize,
    /// Number of online validators
    pub online_validators: usize,
    /// Average performance score
    pub avg_performance_score: f64,
    /// Total stake in the network
    pub total_stake: u64,
    /// Current validator set size
    pub current_set_size: usize,
}

impl Default for ValidatorStats {
    fn default() -> Self {
        Self {
            total_validators: 0,
            active_validators: 0,
            online_validators: 0,
            avg_performance_score: 0.0,
            total_stake: 0,
            current_set_size: 0,
        }
    }
}

/// Validator manager for handling validator operations
pub struct ValidatorManager {
    /// Configuration
    config: ValidatorConfig,
    /// Current validator set
    current_set: Arc<RwLock<Option<ValidatorSet>>>,
    /// All registered validators
    all_validators: Arc<RwLock<HashMap<UInt160, Validator>>>,
    /// Validator statistics
    stats: Arc<RwLock<ValidatorStats>>,
}

impl ValidatorManager {
    /// Creates a new validator manager
    pub fn new(config: ValidatorConfig) -> Self {
        Self {
            config,
            current_set: Arc::new(RwLock::new(None)),
            all_validators: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ValidatorStats::default())),
        }
    }

    /// Registers a new validator
    pub fn register_validator(
        &self,
        public_key_hash: UInt160,
        public_key: Vec<u8>,
        stake: u64,
        block_height: u32,
    ) -> Result<()> {
        if stake < self.config.min_stake {
            return Err(Error::InvalidValidator(format!(
                "Stake {} is below minimum {}",
                stake, self.config.min_stake
            )));
        }

        // Scope the write lock to avoid holding it during update_stats
        let validator_count = {
            let mut all_validators = self.all_validators.write();

            if all_validators.contains_key(&public_key_hash) {
                return Err(Error::InvalidValidator(
                    "Validator already registered".to_string(),
                ));
            }

            let index = all_validators.len() as u8;
            let validator = Validator::new(public_key_hash, public_key, stake, index, block_height);

            all_validators.insert(public_key_hash, validator);
            all_validators.len()
        }; // Write lock is released here

        // Update stats without holding the validators lock
        self.update_stats_safe();

        Ok(())
    }

    /// Updates a validator's stake
    pub fn update_validator_stake(&self, validator_hash: &UInt160, new_stake: u64) -> Result<()> {
        // Scope the write lock
        {
            let mut all_validators = self.all_validators.write();

            if let Some(validator) = all_validators.get_mut(validator_hash) {
                validator.stake = new_stake;
                validator.active = new_stake >= self.config.min_stake;
            } else {
                return Err(Error::InvalidValidator("Validator not found".to_string()));
            }
        } // Write lock is released here

        self.update_stats_safe();
        Ok(())
    }

    /// Sets the current validator set
    pub fn set_validator_set(&self, validator_set: ValidatorSet) -> Result<()> {
        validator_set.validate()?;
        *self.current_set.write() = Some(validator_set);
        self.update_stats_safe();
        Ok(())
    }

    /// Gets the current validator set
    pub fn get_validator_set(&self) -> Option<ValidatorSet> {
        self.current_set.read().clone()
    }

    /// Gets a validator by hash
    pub fn get_validator(&self, hash: &UInt160) -> Option<Validator> {
        self.all_validators.read().get(hash).cloned()
    }

    /// Gets all validators
    pub fn get_all_validators(&self) -> Vec<Validator> {
        self.all_validators.read().values().cloned().collect()
    }

    /// Gets active validators
    pub fn get_active_validators(&self) -> Vec<Validator> {
        self.all_validators
            .read()
            .values()
            .filter(|v| v.active)
            .cloned()
            .collect()
    }

    /// Updates validator performance
    pub fn update_validator_performance<F>(
        &self,
        validator_hash: &UInt160,
        update_fn: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut ValidatorPerformance),
    {
        // Scope the write lock
        {
            let mut all_validators = self.all_validators.write();

            if let Some(validator) = all_validators.get_mut(validator_hash) {
                update_fn(&mut validator.performance);
                validator.update_activity();
            } else {
                return Err(Error::InvalidValidator("Validator not found".to_string()));
            }
        } // Write lock is released here

        Ok(())
    }

    /// Gets validator statistics
    pub fn get_stats(&self) -> ValidatorStats {
        self.stats.read().clone()
    }

    /// Selects validators for the next set based on stake and performance
    pub fn select_next_validator_set(&self, target_size: usize) -> Result<ValidatorSet> {
        let all_validators = self.all_validators.read();

        let mut eligible: Vec<_> = all_validators
            .values()
            .filter(|v| v.active && v.performance_score() >= self.config.min_performance_score)
            .cloned()
            .collect();

        // Release the read lock early
        drop(all_validators);

        if eligible.len() < 4 {
            return Err(Error::InsufficientValidators(format!(
                "Only {} eligible validators, need at least 4",
                eligible.len()
            )));
        }

        eligible.sort_by(|a, b| {
            b.stake.cmp(&a.stake).then_with(|| {
                b.performance_score()
                    .partial_cmp(&a.performance_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });

        // Take top validators up to target size
        let selected_count = target_size
            .min(eligible.len())
            .min(self.config.max_validators);
        let mut selected = eligible
            .into_iter()
            .take(selected_count)
            .collect::<Vec<_>>();

        // Update indices
        for (i, validator) in selected.iter_mut().enumerate() {
            validator.index = i as u8;
        }

        Ok(ValidatorSet::new(selected, 0)) // Block height will be set by caller
    }

    /// Updates internal statistics safely without holding multiple locks
    fn update_stats_safe(&self) {
        let (
            total_validators,
            active_validators,
            online_validators,
            avg_performance_score,
            total_stake,
        ) = {
            let all_validators = self.all_validators.read();

            let total = all_validators.len();
            let active = all_validators.values().filter(|v| v.active).count();
            let online = all_validators
                .values()
                .filter(|v| v.is_online(300)) // 5 minutes
                .count();

            let avg_score = if total > 0 {
                all_validators
                    .values()
                    .map(|v| v.performance_score())
                    .sum::<f64>()
                    / total as f64
            } else {
                0.0
            };

            let stake = all_validators.values().map(|v| v.stake).sum();
            (total, active, online, avg_score, stake)
        }; // Release validators lock

        let current_set_size = {
            self.current_set
                .read()
                .as_ref()
                .map(|s| s.len())
                .unwrap_or(0)
        }; // Release current_set lock

        // Now update stats with a single write lock
        *self.stats.write() = ValidatorStats {
            total_validators,
            active_validators,
            online_validators,
            avg_performance_score,
            total_stake,
            current_set_size,
        };
    }

    /// Updates internal statistics (deprecated, use update_stats_safe)
    fn update_stats(&self) {
        self.update_stats_safe();
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ConsensusContext, ConsensusMessage, ConsensusState};

    #[test]
    fn test_validator_creation() {
        let hash = UInt160::zero();
        let public_key = vec![1, 2, 3, 4];
        let stake = 1000;
        let index = 0;
        let block_height = 100;

        let validator = Validator::new(hash, public_key.clone(), stake, index, block_height);

        assert_eq!(validator.public_key_hash, hash);
        assert_eq!(validator.public_key, public_key);
        assert_eq!(validator.stake, stake);
        assert_eq!(validator.index, index);
        assert!(validator.active);
        assert_eq!(validator.registered_at, block_height);
    }

    #[test]
    fn test_validator_performance() {
        let mut performance = ValidatorPerformance::default();

        // Record some activity
        performance.record_participation();
        performance.record_block_signature();
        performance.record_participation();
        performance.record_miss();

        assert_eq!(performance.rounds_participated, 2);
        assert_eq!(performance.rounds_missed, 1);
        assert_eq!(performance.blocks_signed, 1);

        let score = performance.calculate_score();
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_validator_set() {
        let validators = vec![
            Validator::new(UInt160::zero(), vec![1], 1000, 0, 100),
            Validator::new(
                UInt160::from_bytes(&[1; ADDRESS_SIZE]).unwrap(),
                vec![2],
                2000,
                1,
                100,
            ),
            Validator::new(
                UInt160::from_bytes(&[2; ADDRESS_SIZE]).unwrap(),
                vec![3],
                3000,
                2,
                100,
            ),
            Validator::new(
                UInt160::from_bytes(&[3; ADDRESS_SIZE]).unwrap(),
                vec![4],
                4000,
                3,
                100,
            ),
        ];

        let validator_set = ValidatorSet::new(validators, 100);

        assert_eq!(validator_set.len(), 4);
        assert!(validator_set.validate().is_ok());

        // Test primary selection
        let primary = validator_set.get_primary(ViewNumber::new(0));
        assert!(primary.is_some());
        assert_eq!(primary?.index, 0);

        let primary = validator_set.get_primary(ViewNumber::new(1));
        assert!(primary.is_some());
        assert_eq!(primary?.index, 1);

        // Test backup selection
        let backups = validator_set.get_backups(ViewNumber::new(0));
        assert_eq!(backups.len(), 3);

        // Test required signatures
        assert_eq!(validator_set.required_signatures(), 3); // 4 - (4-1)/3 = 4 - 1 = 3
    }

    #[test]
    fn test_validator_manager() {
        let config = ValidatorConfig::default();
        let manager = ValidatorManager::new(config);

        let hash = UInt160::zero();
        let public_key = vec![1, 2, 3, 4];
        let stake = 1000_00000000; // 1000 NEO

        // Register validator
        manager
            .register_validator(hash, public_key, stake, 100)
            .unwrap();

        // Check validator exists
        let validator = manager.get_validator(&hash);
        assert!(validator.is_some());
        assert_eq!(validator?.stake, stake);

        // Update stake
        manager.update_validator_stake(&hash, stake * 2).unwrap();
        let updated_validator = manager.get_validator(&hash).unwrap();
        assert_eq!(updated_validator.stake, stake * 2);

        // Check stats
        let stats = manager.get_stats();
        assert_eq!(stats.total_validators, 1);
        assert_eq!(stats.active_validators, 1);
    }
}

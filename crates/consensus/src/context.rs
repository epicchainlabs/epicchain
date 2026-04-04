//! Consensus context and state management.
//!
//! This module provides comprehensive consensus context functionality,
//! including state management, round tracking, and timer handling.

use crate::{
    messages::{ChangeView, Commit, PrepareRequest, PrepareResponse, ViewChangeReason},
    BlockIndex, ConsensusConfig, Result, ValidatorSet, ViewNumber,
};
use neo_core::{UInt160, UInt256};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::{interval, Interval};

/// Consensus phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusPhase {
    /// Initial phase - waiting to start
    Initial,
    /// Waiting for prepare request
    WaitingForPrepareRequest,
    /// Waiting for prepare responses
    WaitingForPrepareResponses,
    /// Waiting for commits
    WaitingForCommits,
    /// Block committed
    BlockCommitted,
    /// View changing
    ViewChanging,
    /// Recovery in progress
    Recovery,
}

impl std::fmt::Display for ConsensusPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsensusPhase::Initial => write!(f, "Initial"),
            ConsensusPhase::WaitingForPrepareRequest => write!(f, "Waiting for Prepare Request"),
            ConsensusPhase::WaitingForPrepareResponses => {
                write!(f, "Waiting for Prepare Responses")
            }
            ConsensusPhase::WaitingForCommits => write!(f, "Waiting for Commits"),
            ConsensusPhase::BlockCommitted => write!(f, "Block Committed"),
            ConsensusPhase::ViewChanging => write!(f, "View Changing"),
            ConsensusPhase::Recovery => write!(f, "Recovery"),
        }
    }
}

/// Consensus timer types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerType {
    /// Prepare request timeout
    PrepareRequest,
    /// Prepare response timeout
    PrepareResponse,
    /// Commit timeout
    Commit,
    /// View change timeout
    ViewChange,
    /// Recovery timeout
    Recovery,
}

/// Consensus timer
pub struct ConsensusTimer {
    /// Timer type
    timer_type: TimerType,
    /// Timer interval
    interval: Interval,
    /// Timer duration
    duration: Duration,
    /// Timer start time
    start_time: Instant,
    /// Whether timer is active
    active: bool,
}

impl ConsensusTimer {
    /// Creates a new consensus timer
    pub fn new(timer_type: TimerType, duration: Duration) -> Self {
        let interval = interval(duration);

        Self {
            timer_type,
            interval,
            duration,
            start_time: Instant::now(),
            active: false,
        }
    }

    /// Starts the timer
    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.active = true;
        self.interval.reset();
    }

    /// Stops the timer
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Checks if the timer is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Gets the timer type
    pub fn timer_type(&self) -> TimerType {
        self.timer_type
    }

    /// Gets the elapsed time since start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Waits for the timer to tick
    pub async fn tick(&mut self) {
        if self.active {
            self.interval.tick().await;
        }
    }
}

/// Consensus round information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRound {
    /// Block index for this round
    pub block_index: BlockIndex,
    /// Current view number
    pub view_number: ViewNumber,
    /// Current consensus phase
    pub phase: ConsensusPhase,
    /// Round start timestamp
    pub started_at: u64,
    /// Primary validator for this view
    pub primary_validator: Option<UInt160>,
    /// Prepare request received
    pub prepare_request: Option<PrepareRequest>,
    /// Prepare responses received
    pub prepare_responses: HashMap<u8, PrepareResponse>,
    /// Commits received
    pub commits: HashMap<u8, Commit>,
    /// Change view messages received
    pub change_views: HashMap<u8, ChangeView>,
    /// Whether we have sent our prepare response
    pub prepare_response_sent: bool,
    /// Hash of the prepared block
    pub prepared_block_hash: Option<UInt256>,
    /// Number of transactions in the prepared block
    pub prepared_transaction_count: u32,
    /// Whether we have sent our commit
    pub commit_sent: bool,
    /// Whether we have sent change view
    pub change_view_sent: bool,
}

impl ConsensusRound {
    /// Creates a new consensus round
    pub fn new(block_index: BlockIndex, view_number: ViewNumber) -> Self {
        Self {
            block_index,
            view_number,
            phase: ConsensusPhase::Initial,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            primary_validator: None,
            prepare_request: None,
            prepare_responses: HashMap::new(),
            commits: HashMap::new(),
            change_views: HashMap::new(),
            prepare_response_sent: false,
            prepared_block_hash: None,
            prepared_transaction_count: 0,
            commit_sent: false,
            change_view_sent: false,
        }
    }

    /// Sets the primary validator
    pub fn set_primary_validator(&mut self, validator: UInt160) {
        self.primary_validator = Some(validator);
    }

    /// Sets the prepare request
    pub fn set_prepare_request(&mut self, prepare_request: PrepareRequest) {
        self.prepare_request = Some(prepare_request);
    }

    /// Adds a prepare response
    pub fn add_prepare_response(&mut self, validator_index: u8, response: PrepareResponse) {
        self.prepare_responses.insert(validator_index, response);
    }

    /// Adds a commit
    pub fn add_commit(&mut self, validator_index: u8, commit: Commit) {
        self.commits.insert(validator_index, commit);
    }

    /// Adds a change view
    pub fn add_change_view(&mut self, validator_index: u8, change_view: ChangeView) {
        self.change_views.insert(validator_index, change_view);
    }

    /// Gets the number of prepare responses
    pub fn prepare_response_count(&self) -> usize {
        self.prepare_responses.len()
    }

    /// Gets the number of commits
    pub fn commit_count(&self) -> usize {
        self.commits.len()
    }

    /// Gets the number of change views
    pub fn change_view_count(&self) -> usize {
        self.change_views.len()
    }

    /// Checks if we have enough prepare responses
    pub fn has_enough_prepare_responses(&self, required: usize) -> bool {
        self.prepare_responses
            .values()
            .filter(|r| r.is_accepted())
            .count()
            >= required
    }

    /// Checks if we have enough commits
    pub fn has_enough_commits(&self, required: usize) -> bool {
        self.commits.len() >= required
    }

    /// Checks if we have enough change views
    pub fn has_enough_change_views(&self, required: usize) -> bool {
        self.change_views.len() >= required
    }

    /// Gets the round duration
    pub fn duration(&self) -> Duration {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Duration::from_secs(now.saturating_sub(self.started_at))
    }

    /// Resets the round for a new view
    pub fn reset_for_view(&mut self, new_view: ViewNumber) {
        self.view_number = new_view;
        self.phase = ConsensusPhase::Initial;
        self.started_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.primary_validator = None;
        self.prepare_request = None;
        self.prepare_responses.clear();
        self.commits.clear();
        self.change_views.clear();
        self.prepare_response_sent = false;
        self.commit_sent = false;
        self.change_view_sent = false;
    }
}

/// Consensus context managing the current state
pub struct ConsensusContext {
    /// Configuration
    config: ConsensusConfig,
    /// Current validator set
    validator_set: Arc<RwLock<Option<ValidatorSet>>>,
    /// Current consensus round
    current_round: Arc<RwLock<ConsensusRound>>,
    /// Our validator hash
    my_validator_hash: UInt160,
    /// Consensus timers
    timers: Arc<RwLock<HashMap<TimerType, ConsensusTimer>>>,
    /// Last block hash
    last_block_hash: Arc<RwLock<UInt256>>,
    /// Context statistics
    stats: Arc<RwLock<ContextStats>>,
    /// Mempool service for transaction selection
    mempool: Option<Arc<dyn crate::service::MempoolService>>,
}

/// Context statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStats {
    /// Total rounds participated
    pub rounds_participated: u64,
    /// Total view changes
    pub view_changes: u64,
    /// Total timeouts
    pub timeouts: u64,
    /// Average round duration
    pub avg_round_duration_ms: f64,
    /// Current round start time
    pub current_round_start: u64,
}

impl Default for ContextStats {
    fn default() -> Self {
        Self {
            rounds_participated: 0,
            view_changes: 0,
            timeouts: 0,
            avg_round_duration_ms: 0.0,
            current_round_start: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

impl ConsensusContext {
    /// Creates a new consensus context
    pub fn new(
        config: ConsensusConfig,
        my_validator_hash: UInt160,
        mempool: Option<Arc<dyn crate::service::MempoolService>>,
    ) -> Self {
        let current_round = ConsensusRound::new(BlockIndex::new(0), ViewNumber::new(0));

        // Initialize timers
        let mut timers = HashMap::new();
        timers.insert(
            TimerType::PrepareRequest,
            ConsensusTimer::new(
                TimerType::PrepareRequest,
                Duration::from_millis(config.view_timeout_ms),
            ),
        );
        timers.insert(
            TimerType::PrepareResponse,
            ConsensusTimer::new(
                TimerType::PrepareResponse,
                Duration::from_millis(config.view_timeout_ms),
            ),
        );
        timers.insert(
            TimerType::Commit,
            ConsensusTimer::new(
                TimerType::Commit,
                Duration::from_millis(config.view_timeout_ms),
            ),
        );
        timers.insert(
            TimerType::ViewChange,
            ConsensusTimer::new(
                TimerType::ViewChange,
                Duration::from_millis(config.view_timeout_ms * 2),
            ),
        );
        timers.insert(
            TimerType::Recovery,
            ConsensusTimer::new(
                TimerType::Recovery,
                Duration::from_millis(config.recovery_timeout_ms),
            ),
        );

        Self {
            config,
            validator_set: Arc::new(RwLock::new(None)),
            current_round: Arc::new(RwLock::new(current_round)),
            my_validator_hash,
            timers: Arc::new(RwLock::new(timers)),
            last_block_hash: Arc::new(RwLock::new(UInt256::zero())),
            stats: Arc::new(RwLock::new(ContextStats::default())),
            mempool,
        }
    }

    /// Sets the validator set
    pub fn set_validator_set(&self, validator_set: ValidatorSet) {
        *self.validator_set.write() = Some(validator_set);
    }

    /// Gets the current validator set
    pub fn get_validator_set(&self) -> Option<ValidatorSet> {
        self.validator_set.read().clone()
    }

    /// Starts a new consensus round
    pub fn start_round(&self, block_index: BlockIndex) -> Result<()> {
        let mut round = self.current_round.write();
        *round = ConsensusRound::new(block_index, ViewNumber::new(0));

        // Set primary validator
        if let Some(validator_set) = self.validator_set.read().as_ref() {
            if let Some(primary) = validator_set.get_primary(round.view_number) {
                round.set_primary_validator(primary.public_key_hash);
            }
        }

        round.phase = ConsensusPhase::WaitingForPrepareRequest;

        // Update stats
        let mut stats = self.stats.write();
        stats.rounds_participated += 1;
        stats.current_round_start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Ok(())
    }

    /// Changes to a new view
    pub fn change_view(&self, new_view: ViewNumber, reason: ViewChangeReason) -> Result<()> {
        let mut round = self.current_round.write();
        round.reset_for_view(new_view);

        // Set new primary validator
        if let Some(validator_set) = self.validator_set.read().as_ref() {
            if let Some(primary) = validator_set.get_primary(new_view) {
                round.set_primary_validator(primary.public_key_hash);
            }
        }

        round.phase = ConsensusPhase::WaitingForPrepareRequest;

        // Update stats
        self.stats.write().view_changes += 1;

        // Stop all timers
        self.stop_all_timers();

        Ok(())
    }

    /// Gets the current round
    pub fn get_current_round(&self) -> ConsensusRound {
        self.current_round.read().clone()
    }

    /// Updates the current round
    pub fn update_round<F>(&self, update_fn: F) -> Result<()>
    where
        F: FnOnce(&mut ConsensusRound),
    {
        let mut round = self.current_round.write();
        update_fn(&mut *round);
        Ok(())
    }

    /// Gets our validator index in the current set
    pub fn get_my_validator_index(&self) -> Option<u8> {
        self.validator_set
            .read()
            .as_ref()?
            .get_validator_by_hash(&self.my_validator_hash)?
            .index
            .into()
    }

    /// Gets our validator hash
    pub fn get_my_validator_hash(&self) -> UInt160 {
        self.my_validator_hash
    }

    /// Checks if we are the primary validator
    pub fn am_i_primary(&self) -> bool {
        let round = self.current_round.read();
        round.primary_validator == Some(self.my_validator_hash)
    }

    /// Starts a timer
    pub fn start_timer(&self, timer_type: TimerType) {
        if let Some(timer) = self.timers.write().get_mut(&timer_type) {
            timer.start();
        }
    }

    /// Stops a timer
    pub fn stop_timer(&self, timer_type: TimerType) {
        if let Some(timer) = self.timers.write().get_mut(&timer_type) {
            timer.stop();
        }
    }

    /// Stops all timers
    pub fn stop_all_timers(&self) {
        for timer in self.timers.write().values_mut() {
            timer.stop();
        }
    }

    /// Checks if a timer is active
    pub fn is_timer_active(&self, timer_type: TimerType) -> bool {
        self.timers
            .read()
            .get(&timer_type)
            .map(|t| t.is_active())
            .unwrap_or(false)
    }

    /// Sets the last block hash
    pub fn set_last_block_hash(&self, hash: UInt256) {
        *self.last_block_hash.write() = hash;
    }

    /// Gets the last block hash
    pub fn get_last_block_hash(&self) -> UInt256 {
        *self.last_block_hash.read()
    }

    /// Gets context statistics
    pub fn get_stats(&self) -> ContextStats {
        self.stats.read().clone()
    }

    /// Records a timeout
    pub fn record_timeout(&self, timer_type: TimerType) {
        self.stats.write().timeouts += 1;
    }

    /// Updates round duration statistics
    pub fn update_round_duration(&self, duration_ms: u64) {
        let mut stats = self.stats.write();
        let total_rounds = stats.rounds_participated as f64;

        if total_rounds > 0.0 {
            stats.avg_round_duration_ms = (stats.avg_round_duration_ms * (total_rounds - 1.0)
                + duration_ms as f64)
                / total_rounds;
        } else {
            stats.avg_round_duration_ms = duration_ms as f64;
        }
    }

    /// Gets the required number of signatures for consensus
    pub fn get_required_signatures(&self) -> usize {
        self.config.required_signatures()
    }

    /// Gets the configuration
    pub fn get_config(&self) -> &ConsensusConfig {
        &self.config
    }

    /// Sets the prepared block information
    pub fn set_prepared_block(&self, block_hash: UInt256, transaction_count: u32) {
        let mut round = self.current_round.write();
        round.prepared_block_hash = Some(block_hash);
        round.prepared_transaction_count = transaction_count;
    }

    /// Gets the mempool for transaction selection
    pub fn get_mempool(&self) -> Option<&dyn crate::service::MempoolService> {
        self.mempool.as_ref().map(|m| m.as_ref())
    }

    /// Gets the current blockchain height
    pub fn get_current_height(&self) -> Result<u32> {
        // Get the current round's block index
        let round = self.current_round.read();
        Ok(round.block_index.0)
    }

    /// Gets the previous block hash
    pub fn get_previous_hash(&self) -> Result<UInt256> {
        // Return the last block hash we have stored
        Ok(self.get_last_block_hash())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ConsensusContext, ConsensusMessage, ConsensusState};

    #[test]
    fn test_consensus_phase() {
        let phase = ConsensusPhase::WaitingForPrepareRequest;
        assert_eq!(phase.to_string(), "Waiting for Prepare Request");

        let phase = ConsensusPhase::BlockCommitted;
        assert_eq!(phase.to_string(), "Block Committed");
    }

    #[test]
    fn test_consensus_round() {
        let block_index = BlockIndex::new(100);
        let view_number = ViewNumber::new(1);

        let mut round = ConsensusRound::new(block_index, view_number);
        assert_eq!(round.block_index, block_index);
        assert_eq!(round.view_number, view_number);
        assert_eq!(round.phase, ConsensusPhase::Initial);

        // Test adding responses
        let prep_response = PrepareResponse::accept(UInt256::zero());
        round.add_prepare_response(0, prep_response);
        assert_eq!(round.prepare_response_count(), 1);

        let commit = Commit::new(UInt256::zero(), vec![1, 2, 3]);
        round.add_commit(0, commit);
        assert_eq!(round.commit_count(), 1);

        // Test reset
        round.reset_for_view(ViewNumber::new(2));
        assert_eq!(round.view_number.value(), 2);
        assert_eq!(round.prepare_response_count(), 0);
        assert_eq!(round.commit_count(), 0);
    }

    #[tokio::test]
    async fn test_consensus_timer() {
        let mut timer = ConsensusTimer::new(TimerType::PrepareRequest, Duration::from_millis(100));

        assert!(!timer.is_active());
        assert_eq!(timer.timer_type(), TimerType::PrepareRequest);

        timer.start();
        assert!(timer.is_active());

        timer.stop();
        assert!(!timer.is_active());
    }

    #[tokio::test]
    async fn test_consensus_context() {
        let config = ConsensusConfig::default();
        let my_hash = UInt160::zero();

        let context = ConsensusContext::new(config, my_hash, None);

        // Test starting a round
        context
            .start_round(BlockIndex::new(100))
            .expect("consensus operation should succeed");

        let round = context.get_current_round();
        assert_eq!(round.block_index.value(), 100);
        assert_eq!(round.view_number.value(), 0);

        // Test changing view
        context
            .change_view(ViewNumber::new(1), ViewChangeReason::PrepareRequestTimeout)
            .expect("consensus operation should succeed");

        let round = context.get_current_round();
        assert_eq!(round.view_number.value(), 1);

        // Test stats
        let stats = context.get_stats();
        assert_eq!(stats.rounds_participated, 1);
        assert_eq!(stats.view_changes, 1);
    }
}

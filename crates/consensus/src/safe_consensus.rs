//! Safe consensus operations module
//!
//! This module provides safe alternatives to panic! calls in consensus code,
//! ensuring the consensus mechanism remains stable even under error conditions.

use crate::{Error as ConsensusError, Result as ConsensusResult};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Safe consensus state manager
pub struct SafeConsensusState {
    /// Current view number
    view: u32,
    /// Current height
    height: u32,
    /// Node index in consensus
    node_index: u32,
    /// Total number of consensus nodes
    node_count: u32,
    /// Last error time for rate limiting
    last_error_time: Option<Instant>,
    /// Error count for circuit breaking
    error_count: u32,
}

impl SafeConsensusState {
    /// Create a new safe consensus state
    pub fn new(node_index: u32, node_count: u32) -> ConsensusResult<Self> {
        if node_count == 0 {
            return Err(ConsensusError::InvalidConfig(
                "Node count cannot be zero".to_string(),
            ));
        }

        if node_index >= node_count {
            return Err(ConsensusError::InvalidConfig(format!(
                "Node index {} >= node count {}",
                node_index, node_count
            )));
        }

        Ok(Self {
            view: 0,
            height: 0,
            node_index,
            node_count,
            last_error_time: None,
            error_count: 0,
        })
    }

    /// Safely change view
    pub fn change_view(&mut self, new_view: u32) -> ConsensusResult<()> {
        // Prevent view from going backwards
        if new_view < self.view {
            return Err(ConsensusError::InvalidView(format!(
                "Cannot decrease view from {} to {}",
                self.view, new_view
            )));
        }

        // Prevent excessive view changes (possible attack)
        const MAX_VIEW_JUMP: u32 = 10;
        if new_view > self.view + MAX_VIEW_JUMP {
            return Err(ConsensusError::InvalidView(format!(
                "View jump too large: {} -> {}",
                self.view, new_view
            )));
        }

        self.view = new_view;
        Ok(())
    }

    /// Safely increment height
    pub fn increment_height(&mut self) -> ConsensusResult<()> {
        // Check for height overflow
        if self.height == u32::MAX {
            return Err(ConsensusError::InvalidState(
                "Height overflow detected".to_string(),
            ));
        }

        self.height += 1;
        self.view = 0; // Reset view on new height
        Ok(())
    }

    /// Get primary node index for current view
    pub fn get_primary_index(&self) -> u32 {
        (self.height - self.view) % self.node_count
    }

    /// Check if this node is primary
    pub fn is_primary(&self) -> bool {
        self.get_primary_index() == self.node_index
    }

    /// Record an error with rate limiting
    pub fn record_error(&mut self) -> ConsensusResult<()> {
        let now = Instant::now();

        // Rate limit errors
        if let Some(last_time) = self.last_error_time {
            if now.duration_since(last_time) < Duration::from_millis(100) {
                return Err(ConsensusError::RateLimitExceeded(
                    "Too many errors in short time".to_string(),
                ));
            }
        }

        self.last_error_time = Some(now);
        self.error_count += 1;

        // Circuit breaker pattern
        const MAX_ERRORS: u32 = 10;
        if self.error_count > MAX_ERRORS {
            return Err(ConsensusError::CircuitBreakerOpen(format!(
                "Too many errors: {}",
                self.error_count
            )));
        }

        Ok(())
    }

    /// Reset error count (e.g., after successful operation)
    pub fn reset_errors(&mut self) {
        self.error_count = 0;
        self.last_error_time = None;
    }
}

/// Safe message validator for consensus messages
pub struct SafeMessageValidator {
    /// Maximum message size
    max_message_size: usize,
    /// Validate signatures
    validate_signatures: bool,
}

impl SafeMessageValidator {
    /// Create a new message validator
    pub fn new(max_message_size: usize) -> Self {
        Self {
            max_message_size,
            validate_signatures: true,
        }
    }

    /// Validate consensus message safely
    pub fn validate_message(&self, message: &[u8]) -> ConsensusResult<()> {
        // Check message size
        if message.is_empty() {
            return Err(ConsensusError::InvalidMessage(
                "Empty consensus message".to_string(),
            ));
        }

        if message.len() > self.max_message_size {
            return Err(ConsensusError::InvalidMessage(format!(
                "Message size {} exceeds maximum {}",
                message.len(),
                self.max_message_size
            )));
        }

        // Additional validation could be added here
        // e.g., message type validation, signature verification

        Ok(())
    }

    /// Validate proposal
    pub fn validate_proposal(
        &self,
        proposal_view: u32,
        current_view: u32,
        proposal_height: u32,
        current_height: u32,
    ) -> ConsensusResult<()> {
        // Proposal must be for current or future view
        if proposal_view < current_view {
            return Err(ConsensusError::InvalidProposal(format!(
                "Proposal view {} < current view {}",
                proposal_view, current_view
            )));
        }

        // Proposal must be for current height
        if proposal_height != current_height {
            return Err(ConsensusError::InvalidProposal(format!(
                "Proposal height {} != current height {}",
                proposal_height, current_height
            )));
        }

        Ok(())
    }

    /// Validate vote
    pub fn validate_vote(
        &self,
        vote_view: u32,
        current_view: u32,
        vote_height: u32,
        current_height: u32,
    ) -> ConsensusResult<()> {
        // Vote must be for current view and height
        if vote_view != current_view {
            return Err(ConsensusError::InvalidVote(format!(
                "Vote view {} != current view {}",
                vote_view, current_view
            )));
        }

        if vote_height != current_height {
            return Err(ConsensusError::InvalidVote(format!(
                "Vote height {} != current height {}",
                vote_height, current_height
            )));
        }

        Ok(())
    }
}

/// Safe consensus timeout manager
pub struct SafeTimeoutManager {
    /// Base timeout duration
    base_timeout: Duration,
    /// Maximum timeout duration
    max_timeout: Duration,
    /// Current timeout multiplier
    timeout_multiplier: u32,
}

impl SafeTimeoutManager {
    /// Create a new timeout manager
    pub fn new(base_timeout: Duration, max_timeout: Duration) -> Self {
        Self {
            base_timeout,
            max_timeout,
            timeout_multiplier: 1,
        }
    }

    /// Get current timeout duration
    pub fn get_timeout(&self) -> Duration {
        let timeout = self.base_timeout * self.timeout_multiplier;
        timeout.min(self.max_timeout)
    }

    /// Increase timeout (exponential backoff)
    pub fn increase_timeout(&mut self) {
        const MAX_MULTIPLIER: u32 = 32;
        if self.timeout_multiplier < MAX_MULTIPLIER {
            self.timeout_multiplier *= 2;
        }
    }

    /// Reset timeout to base value
    pub fn reset_timeout(&mut self) {
        self.timeout_multiplier = 1;
    }

    /// Check if timeout has expired
    pub fn is_expired(&self, start_time: Instant) -> bool {
        start_time.elapsed() > self.get_timeout()
    }
}

/// Safe recovery mechanism for consensus failures
pub struct SafeRecoveryManager {
    /// Number of recovery attempts
    recovery_attempts: u32,
    /// Maximum recovery attempts
    max_attempts: u32,
    /// Last recovery time
    last_recovery: Option<Instant>,
}

impl SafeRecoveryManager {
    /// Create a new recovery manager
    pub fn new(max_attempts: u32) -> Self {
        Self {
            recovery_attempts: 0,
            max_attempts,
            last_recovery: None,
        }
    }

    /// Attempt recovery
    pub fn attempt_recovery(&mut self) -> ConsensusResult<()> {
        // Check if we've exceeded max attempts
        if self.recovery_attempts >= self.max_attempts {
            return Err(ConsensusError::RecoveryFailed(format!(
                "Exceeded maximum recovery attempts: {}",
                self.max_attempts
            )));
        }

        // Rate limit recovery attempts
        if let Some(last_time) = self.last_recovery {
            const MIN_RECOVERY_INTERVAL: Duration = Duration::from_secs(5);
            if last_time.elapsed() < MIN_RECOVERY_INTERVAL {
                return Err(ConsensusError::RecoveryTooSoon(
                    "Recovery attempted too soon after last attempt".to_string(),
                ));
            }
        }

        self.recovery_attempts += 1;
        self.last_recovery = Some(Instant::now());
        Ok(())
    }

    /// Reset recovery state after successful consensus
    pub fn reset(&mut self) {
        self.recovery_attempts = 0;
        self.last_recovery = None;
    }

    /// Check if recovery is needed based on error patterns
    pub fn needs_recovery(&self, consecutive_errors: u32) -> bool {
        consecutive_errors >= 3 && self.recovery_attempts < self.max_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_consensus_state() {
        // Valid creation
        let mut state = SafeConsensusState::new(1, 4).unwrap();
        assert_eq!(state.view, 0);
        assert_eq!(state.height, 0);

        // Invalid creation
        assert!(SafeConsensusState::new(0, 0).is_err());
        assert!(SafeConsensusState::new(5, 4).is_err());

        // View change
        assert!(state.change_view(1).is_ok());
        assert!(state.change_view(0).is_err()); // Can't go backwards
        assert!(state.change_view(100).is_err()); // Too large jump
    }

    #[test]
    fn test_message_validator() {
        let validator = SafeMessageValidator::new(1024);

        // Valid message
        assert!(validator.validate_message(b"valid message").is_ok());

        // Empty message
        assert!(validator.validate_message(b"").is_err());

        // Too large message
        let large_message = vec![0u8; 1025];
        assert!(validator.validate_message(&large_message).is_err());
    }

    #[test]
    fn test_timeout_manager() {
        let mut manager = SafeTimeoutManager::new(Duration::from_secs(1), Duration::from_secs(30));

        assert_eq!(manager.get_timeout(), Duration::from_secs(1));

        manager.increase_timeout();
        assert_eq!(manager.get_timeout(), Duration::from_secs(2));

        manager.increase_timeout();
        assert_eq!(manager.get_timeout(), Duration::from_secs(4));

        manager.reset_timeout();
        assert_eq!(manager.get_timeout(), Duration::from_secs(1));
    }

    #[test]
    fn test_recovery_manager() {
        let mut manager = SafeRecoveryManager::new(3);

        // First recovery should succeed
        assert!(manager.attempt_recovery().is_ok());

        // Too soon for another recovery
        assert!(manager.attempt_recovery().is_err());

        // After waiting, recovery should work
        std::thread::sleep(Duration::from_secs(6));
        assert!(manager.attempt_recovery().is_ok());
    }
}

//! dBFT state management module.
//!
//! This module contains state definitions, statistics, and events for the dBFT consensus engine.

use crate::{
    context::TimerType, messages::ViewChangeReason, BlockIndex, ConsensusSignature, ViewNumber,
};
use neo_core::{UInt160, UInt256};
use serde::{Deserialize, Serialize};

/// dBFT engine state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DbftState {
    /// Engine is stopped
    Stopped,
    /// Engine is starting
    Starting,
    /// Engine is running
    Running,
    /// Engine is stopping
    Stopping,
    /// Engine is in recovery mode
    Recovery,
    /// Engine is in view change mode
    ViewChange,
    /// Engine is synchronizing
    Synchronizing,
}

impl std::fmt::Display for DbftState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbftState::Stopped => write!(f, "Stopped"),
            DbftState::Starting => write!(f, "Starting"),
            DbftState::Running => write!(f, "Running"),
            DbftState::Stopping => write!(f, "Stopping"),
            DbftState::Recovery => write!(f, "Recovery"),
            DbftState::ViewChange => write!(f, "ViewChange"),
            DbftState::Synchronizing => write!(f, "Synchronizing"),
        }
    }
}

impl DbftState {
    /// Checks if the engine is in an active state
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Running | Self::Recovery | Self::ViewChange)
    }

    /// Checks if the engine can process messages
    pub fn can_process_messages(&self) -> bool {
        matches!(self, Self::Running | Self::Recovery | Self::ViewChange)
    }

    /// Checks if the engine can start consensus
    pub fn can_start_consensus(&self) -> bool {
        matches!(self, Self::Running)
    }

    /// Gets the next valid state transition
    pub fn next_state(&self, target: DbftState) -> Option<DbftState> {
        match (self, target) {
            (Self::Stopped, Self::Starting) => Some(Self::Starting),
            (Self::Starting, Self::Running) => Some(Self::Running),
            (Self::Running, Self::Stopping) => Some(Self::Stopping),
            (Self::Running, Self::Recovery) => Some(Self::Recovery),
            (Self::Running, Self::ViewChange) => Some(Self::ViewChange),
            (Self::Recovery, Self::Running) => Some(Self::Running),
            (Self::ViewChange, Self::Running) => Some(Self::Running),
            (Self::Stopping, Self::Stopped) => Some(Self::Stopped),
            (Self::Synchronizing, Self::Running) => Some(Self::Running),
            _ => None,
        }
    }
}

/// dBFT engine statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbftStats {
    /// Current state
    pub state: DbftState,
    /// Total blocks produced
    pub blocks_produced: u64,
    /// Total consensus rounds
    pub consensus_rounds: u64,
    /// Total view changes
    pub view_changes: u64,
    /// Total timeouts
    pub timeouts: u64,
    /// Total recovery attempts
    pub recovery_attempts: u64,
    /// Average block time in milliseconds
    pub avg_block_time_ms: f64,
    /// Average consensus time in milliseconds
    pub avg_consensus_time_ms: f64,
    /// Current round start time
    pub current_round_start: u64,
    /// Last block produced time
    pub last_block_time: u64,
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Total invalid messages
    pub invalid_messages: u64,
    /// Current view number
    pub current_view: u32,
    /// Current block index
    pub current_block_index: u32,
}

impl Default for DbftStats {
    fn default() -> Self {
        Self {
            state: DbftState::Stopped,
            blocks_produced: 0,
            consensus_rounds: 0,
            view_changes: 0,
            timeouts: 0,
            recovery_attempts: 0,
            avg_block_time_ms: 0.0,
            avg_consensus_time_ms: 0.0,
            current_round_start: 0,
            last_block_time: 0,
            messages_sent: 0,
            messages_received: 0,
            invalid_messages: 0,
            current_view: 0,
            current_block_index: 0,
        }
    }
}

impl DbftStats {
    /// Updates block production statistics
    pub fn record_block_produced(&mut self, block_time_ms: u64, consensus_time_ms: u64) {
        self.blocks_produced += 1;
        self.last_block_time = block_time_ms;

        // Update averages using exponential moving average
        let alpha = 0.1; // Smoothing factor
        if self.avg_block_time_ms == 0.0 {
            self.avg_block_time_ms = block_time_ms as f64;
            self.avg_consensus_time_ms = consensus_time_ms as f64;
        } else {
            self.avg_block_time_ms =
                alpha * (block_time_ms as f64) + (1.0 - alpha) * self.avg_block_time_ms;
            self.avg_consensus_time_ms =
                alpha * (consensus_time_ms as f64) + (1.0 - alpha) * self.avg_consensus_time_ms;
        }
    }

    /// Records a view change
    pub fn record_view_change(&mut self) {
        self.view_changes += 1;
        self.current_view += 1;
    }

    /// Records a timeout
    pub fn record_timeout(&mut self) {
        self.timeouts += 1;
    }

    /// Records a recovery attempt
    pub fn record_recovery_attempt(&mut self) {
        self.recovery_attempts += 1;
    }

    /// Records a message sent
    pub fn record_message_sent(&mut self) {
        self.messages_sent += 1;
    }

    /// Records a message received
    pub fn record_message_received(&mut self, is_valid: bool) {
        self.messages_received += 1;
        if !is_valid {
            self.invalid_messages += 1;
        }
    }

    /// Gets the success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.consensus_rounds == 0 {
            return 100.0;
        }
        (self.blocks_produced as f64 / self.consensus_rounds as f64) * 100.0
    }

    /// Gets the message validity rate as a percentage
    pub fn message_validity_rate(&self) -> f64 {
        if self.messages_received == 0 {
            return 100.0;
        }
        ((self.messages_received - self.invalid_messages) as f64 / self.messages_received as f64)
            * 100.0
    }
}

/// dBFT engine events
#[derive(Debug, Clone)]
pub enum DbftEvent {
    /// Engine state changed
    StateChanged {
        old_state: DbftState,
        new_state: DbftState,
    },
    /// Block proposal created
    BlockProposed {
        block_index: BlockIndex,
        block_hash: UInt256,
        proposer: UInt160,
        transaction_count: usize,
    },
    /// Block committed
    BlockCommitted {
        block_index: BlockIndex,
        block_hash: UInt256,
        signatures: Vec<ConsensusSignature>,
        consensus_time_ms: u64,
    },
    /// View changed
    ViewChanged {
        block_index: BlockIndex,
        old_view: ViewNumber,
        new_view: ViewNumber,
        reason: ViewChangeReason,
    },
    /// Consensus timeout
    ConsensusTimeout {
        block_index: BlockIndex,
        view: ViewNumber,
        timer_type: TimerType,
    },
    /// Recovery started
    RecoveryStarted {
        block_index: BlockIndex,
        view: ViewNumber,
        reason: String,
    },
    /// Recovery completed
    RecoveryCompleted {
        block_index: BlockIndex,
        view: ViewNumber,
        success: bool,
    },
    /// Message received
    MessageReceived {
        message_type: String,
        validator_index: u8,
        block_index: BlockIndex,
        view: ViewNumber,
    },
    /// Message sent
    MessageSent {
        message_type: String,
        block_index: BlockIndex,
        view: ViewNumber,
    },
    /// Validation error
    ValidationError {
        error: String,
        block_index: BlockIndex,
        view: ViewNumber,
    },
}

impl DbftEvent {
    /// Gets the event type as a string
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::StateChanged { .. } => "StateChanged",
            Self::BlockProposed { .. } => "BlockProposed",
            Self::BlockCommitted { .. } => "BlockCommitted",
            Self::ViewChanged { .. } => "ViewChanged",
            Self::ConsensusTimeout { .. } => "ConsensusTimeout",
            Self::RecoveryStarted { .. } => "RecoveryStarted",
            Self::RecoveryCompleted { .. } => "RecoveryCompleted",
            Self::MessageReceived { .. } => "MessageReceived",
            Self::MessageSent { .. } => "MessageSent",
            Self::ValidationError { .. } => "ValidationError",
        }
    }

    /// Checks if this is an error event
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            Self::ConsensusTimeout { .. } | Self::ValidationError { .. }
        )
    }

    /// Checks if this is a success event
    pub fn is_success(&self) -> bool {
        matches!(
            self,
            Self::BlockCommitted { .. } | Self::RecoveryCompleted { success: true, .. }
        )
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ConsensusContext, ConsensusMessage, ConsensusState};

    #[test]
    fn test_state_transitions() {
        assert_eq!(
            DbftState::Stopped.next_state(DbftState::Starting),
            Some(DbftState::Starting)
        );
        assert_eq!(
            DbftState::Starting.next_state(DbftState::Running),
            Some(DbftState::Running)
        );
        assert_eq!(
            DbftState::Running.next_state(DbftState::Stopped),
            None // Invalid transition
        );
    }

    #[test]
    fn test_state_properties() {
        assert!(DbftState::Running.is_active());
        assert!(DbftState::Running.can_process_messages());
        assert!(DbftState::Running.can_start_consensus());

        assert!(!DbftState::Stopped.is_active());
        assert!(!DbftState::Stopped.can_process_messages());
        assert!(!DbftState::Stopped.can_start_consensus());
    }

    #[test]
    fn test_stats_calculations() {
        let mut stats = DbftStats::default();

        // Test success rate
        stats.consensus_rounds = 10;
        stats.blocks_produced = 8;
        assert_eq!(stats.success_rate(), 80.0);

        // Test message validity rate
        stats.messages_received = 100;
        stats.invalid_messages = 5;
        assert_eq!(stats.message_validity_rate(), 95.0);
    }

    #[test]
    fn test_event_properties() {
        let event = DbftEvent::BlockCommitted {
            block_index: BlockIndex::new(1),
            block_hash: UInt256::zero(),
            signatures: vec![],
            consensus_time_ms: 1000,
        };

        assert_eq!(event.event_type(), "BlockCommitted");
        assert!(event.is_success());
        assert!(!event.is_error());
    }
}

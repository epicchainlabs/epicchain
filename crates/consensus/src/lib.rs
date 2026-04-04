//! Neo Consensus Module
//!
//! This module provides comprehensive consensus functionality for the Neo blockchain,
//! implementing the dBFT (delegated Byzantine Fault Tolerance) consensus algorithm
//! and related consensus mechanisms.
//!
//! ## Components
//!
//! - **dBFT**: Core dBFT consensus algorithm implementation
//! - **Messages**: Consensus message types and handling
//! - **Validators**: Validator management and selection
//! - **Context**: Consensus state and context management
//! - **Proposal**: Block proposal and validation
//! - **Recovery**: View change and recovery mechanisms
//! - **Service**: Main consensus service coordination

pub mod context;
pub mod dbft;
pub mod extensible_wrapper;
pub mod mempool_adapter;
pub mod messages;
pub mod proposal;
pub mod recovery;
pub mod safe_consensus;
pub mod service;
pub mod signature;
pub mod validators;
pub mod view_change_optimization;

// Re-export main types
pub use context::{ConsensusContext, ConsensusPhase, ConsensusRound, ConsensusTimer};
const DEFAULT_TIMEOUT_MS: u64 = 30000;
pub use dbft::{DbftConfig, DbftEngine, DbftState, DbftStats};
use log::{debug, error, info, warn};
pub use messages::{
    ChangeView, Commit, ConsensusMessage, ConsensusMessageType, PrepareRequest, PrepareResponse,
    RecoveryRequest, RecoveryResponse, ViewChangeReason,
};
use neo_config::{MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK, MILLISECONDS_PER_BLOCK};
use neo_core::{UInt160, UInt256};
pub use proposal::{BlockProposal, ProposalConfig, ProposalManager, ProposalStats};
pub use recovery::{RecoveryConfig, RecoveryManager, RecoveryStats};
use serde::{Deserialize, Serialize};
pub use service::{
    ConsensusEvent, ConsensusService, ConsensusServiceConfig, ConsensusStats, LedgerService,
    MempoolService, NetworkService,
};
use sha2::{Digest, Sha256};
pub use signature::{MessageSigner, SignatureProvider};
use std::fmt;
use thiserror::Error;
pub use validators::{Validator, ValidatorConfig, ValidatorManager, ValidatorSet, ValidatorStats};

/// Result type for consensus operations
pub type Result<T> = std::result::Result<T, Error>;

/// Consensus-specific error types
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid consensus message
    #[error("Invalid consensus message: {0}")]
    InvalidMessage(String),

    /// Invalid validator
    #[error("Invalid validator: {0}")]
    InvalidValidator(String),

    /// Invalid block proposal
    #[error("Invalid block proposal: {0}")]
    InvalidProposal(String),

    /// Consensus timeout
    #[error("Consensus timeout: {0}")]
    Timeout(String),

    /// View change error
    #[error("View change error: {0}")]
    ViewChange(String),

    /// Recovery error
    #[error("Recovery error: {0}")]
    Recovery(String),

    /// Signature verification failed
    #[error("Signature verification failed: {0}")]
    SignatureVerification(String),

    /// Insufficient validators
    #[error("Insufficient validators: {0}")]
    InsufficientValidators(String),

    /// Consensus not ready
    #[error("Consensus not ready: {0}")]
    NotReady(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Invalid view
    #[error("Invalid view: {0}")]
    InvalidView(String),

    /// Invalid state
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Invalid vote
    #[error("Invalid vote: {0}")]
    InvalidVote(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Circuit breaker open
    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    /// Recovery failed
    #[error("Recovery failed: {0}")]
    RecoveryFailed(String),

    /// Recovery too soon
    #[error("Recovery too soon: {0}")]
    RecoveryTooSoon(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Ledger error
    #[error("Ledger error: {0}")]
    Ledger(String),

    /// Cryptography error
    #[error("Cryptography error: {0}")]
    Cryptography(String),

    /// Invalid public key
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    /// Invalid block
    #[error("Invalid block: {0}")]
    InvalidBlock(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Core error
    #[error("Core error: {0}")]
    Core(#[from] neo_core::CoreError),

    /// Generic error
    #[error("Consensus error: {0}")]
    Generic(String),

    /// Invalid recovery session
    #[error("Invalid recovery session: {0}")]
    InvalidRecoverySession(String),

    /// Invalid arguments
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    /// Execution halted
    #[error("Execution halted: {0}")]
    ExecutionHalted(String),

    /// VM error
    #[error("VM error: {0}")]
    VmError(String),

    /// Contract not found
    #[error("Contract not found: {0}")]
    ContractNotFound(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Insufficient funds
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Engine error
    #[error("Engine error: {0}")]
    EngineError(String),
}

/// Consensus node role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeRole {
    /// Primary node (block proposer)
    Primary,
    /// Backup node (validator)
    Backup,
    /// Observer node (non-validator)
    Observer,
}

impl fmt::Display for NodeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Primary => write!(f, "Primary"),
            NodeRole::Backup => write!(f, "Backup"),
            NodeRole::Observer => write!(f, "Observer"),
        }
    }
}

/// Consensus view number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ViewNumber(pub u8);

impl ViewNumber {
    /// Creates a new view number
    pub fn new(view: u8) -> Self {
        Self(view)
    }

    /// Gets the view number value
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Increments the view number
    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    /// Gets the next view number
    pub fn next(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

impl Default for ViewNumber {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for ViewNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Consensus block index
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BlockIndex(pub u32);

impl BlockIndex {
    /// Creates a new block index
    pub fn new(index: u32) -> Self {
        Self(index)
    }

    /// Gets the block index value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Increments the block index
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Gets the next block index
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for BlockIndex {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for BlockIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Consensus signature
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConsensusSignature {
    /// Validator public key hash
    pub validator: UInt160,
    /// Signature data
    pub signature: Vec<u8>,
}

impl ConsensusSignature {
    /// Creates a new consensus signature
    pub fn new(validator: UInt160, signature: Vec<u8>) -> Self {
        Self {
            validator,
            signature,
        }
    }

    /// Verifies the signature
    pub fn verify(&self, message: &[u8], public_key: &[u8]) -> Result<bool> {
        if self.signature.is_empty() {
            return Ok(false);
        }

        if public_key.len() != 33 {
            return Err(Error::InvalidPublicKey(
                "Public key must be 33 bytes (compressed)".to_string(),
            ));
        }

        if message.is_empty() {
            return Err(Error::InvalidMessage("Message cannot be empty".to_string()));
        }

        match neo_cryptography::ecdsa::ECDsa::verify_signature_secp256r1(
            message,
            &self.signature,
            public_key,
        ) {
            Ok(is_valid) => {
                if is_valid {
                    log::info!("Consensus signature verification PASSED for validator");
                } else {
                    log::info!("Consensus signature verification FAILED for validator");
                }
                Ok(is_valid)
            }
            Err(e) => {
                log::info!("Error verifying consensus signature: {}", e);
                Ok(false)
            }
        }
    }
}

/// Consensus payload for messages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusPayload {
    /// Validator index
    pub validator_index: u8,
    /// Block index
    pub block_index: BlockIndex,
    /// View number
    pub view_number: ViewNumber,
    /// Timestamp
    pub timestamp: u64,
    /// Message data
    pub data: Vec<u8>,
}

impl ConsensusPayload {
    /// Creates a new consensus payload
    pub fn new(
        validator_index: u8,
        block_index: BlockIndex,
        view_number: ViewNumber,
        data: Vec<u8>,
    ) -> Self {
        Self {
            validator_index,
            block_index,
            view_number,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Operation failed")
                .as_secs(),
            data,
        }
    }

    /// Gets the payload hash
    pub fn hash(&self) -> UInt256 {
        let serialized = bincode::serialize(self).unwrap_or_default();
        let hash = Sha256::digest(&serialized);
        UInt256::from_bytes(&hash).unwrap_or_default()
    }

    /// Serializes the payload to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| Error::Generic(format!("Failed to serialize payload: {}", e)))
    }

    /// Deserializes payload from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
            .map_err(|e| Error::Generic(format!("Failed to deserialize payload: {}", e)))
    }
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Number of validators
    pub validator_count: usize,
    /// Block time in milliseconds
    pub block_time_ms: u64,
    /// View timeout in milliseconds
    pub view_timeout_ms: u64,
    /// Maximum view changes
    pub max_view_changes: u8,
    /// Enable recovery
    pub enable_recovery: bool,
    /// Recovery timeout in milliseconds
    pub recovery_timeout_ms: u64,
    /// Maximum block size
    pub max_block_size: usize,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Enable metrics
    pub enable_metrics: bool,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            validator_count: 7,
            block_time_ms: MILLISECONDS_PER_BLOCK, // SECONDS_PER_BLOCK seconds
            view_timeout_ms: 20000,                // ADDRESS_SIZE seconds
            max_view_changes: 6,
            enable_recovery: true,
            recovery_timeout_ms: DEFAULT_TIMEOUT_MS, // 30 seconds
            max_block_size: MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE,
            max_transactions_per_block: MAX_TRANSACTIONS_PER_BLOCK,
            enable_metrics: true,
        }
    }
}

impl ConsensusConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<()> {
        if self.validator_count < 4 {
            return Err(Error::Configuration(
                "Validator count must be at least 4".to_string(),
            ));
        }

        if self.validator_count % 3 != 1 {
            return Err(Error::Configuration(
                "Validator count must be 3f+1 where f is the number of Byzantine nodes".to_string(),
            ));
        }

        if self.block_time_ms < 1000 {
            return Err(Error::Configuration(
                "Block time must be at least 1 second".to_string(),
            ));
        }

        if self.view_timeout_ms < self.block_time_ms {
            return Err(Error::Configuration(
                "View timeout must be at least as long as block time".to_string(),
            ));
        }

        if self.max_view_changes == 0 {
            return Err(Error::Configuration(
                "Max view changes must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Gets the Byzantine fault tolerance threshold
    pub fn byzantine_threshold(&self) -> usize {
        (self.validator_count - 1) / 3
    }

    /// Gets the required number of signatures for consensus
    pub fn required_signatures(&self) -> usize {
        self.validator_count - self.byzantine_threshold()
    }
}

/// Consensus utilities
pub mod utils {
    use super::{ConsensusConfig, ViewNumber};

    /// Calculates the primary validator index for a view
    pub fn calculate_primary_index(view: ViewNumber, validator_count: usize) -> usize {
        (view.value() as usize) % validator_count
    }

    /// Checks if enough signatures are collected
    pub fn has_enough_signatures(signature_count: usize, config: &ConsensusConfig) -> bool {
        signature_count >= config.required_signatures()
    }

    /// Generates a consensus nonce
    pub fn generate_nonce() -> u64 {
        rand::random()
    }

    /// Calculates message timeout based on view number
    pub fn calculate_timeout(view: ViewNumber, base_timeout_ms: u64) -> u64 {
        base_timeout_ms * (1 << view.value().min(6)) // Exponential backoff, max 64x
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_view_number() {
        let mut view = ViewNumber::new(0);
        assert_eq!(view.value(), 0);

        view.increment();
        assert_eq!(view.value(), 1);

        let next = view.next();
        assert_eq!(next.value(), 2);
        assert_eq!(view.value(), 1); // Original unchanged
    }

    #[test]
    fn test_block_index() {
        let mut index = BlockIndex::new(100);
        assert_eq!(index.value(), 100);

        index.increment();
        assert_eq!(index.value(), 101);

        let next = index.next();
        assert_eq!(next.value(), 102);
        assert_eq!(index.value(), 101); // Original unchanged
    }

    #[test]
    fn test_consensus_config() {
        let config = ConsensusConfig::default();
        assert!(config.validate().is_ok());

        assert_eq!(config.byzantine_threshold(), 2);
        assert_eq!(config.required_signatures(), 5); // 7-2 = 5

        let invalid_config = ConsensusConfig {
            validator_count: 3, // Too few
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_consensus_payload() {
        let payload = ConsensusPayload::new(
            0,
            BlockIndex::new(100),
            ViewNumber::new(1),
            vec![1, 2, 3, 4],
        );

        assert_eq!(payload.validator_index, 0);
        assert_eq!(payload.block_index.value(), 100);
        assert_eq!(payload.view_number.value(), 1);
        assert_eq!(payload.data, vec![1, 2, 3, 4]);

        // Test serialization
        let bytes = payload.to_bytes().unwrap();
        let deserialized = ConsensusPayload::from_bytes(&bytes).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_consensus_signature() {
        let validator = UInt160::zero();
        let signature = vec![1, 2, 3, 4, 5];

        let consensus_sig = ConsensusSignature::new(validator, signature.clone());
        assert_eq!(consensus_sig.validator, validator);
        assert_eq!(consensus_sig.signature, signature);
    }

    #[test]
    fn test_utils() {
        // Test primary index calculation
        assert_eq!(utils::calculate_primary_index(ViewNumber::new(0), 7), 0);
        assert_eq!(utils::calculate_primary_index(ViewNumber::new(1), 7), 1);
        assert_eq!(utils::calculate_primary_index(ViewNumber::new(7), 7), 0);

        // Test signature threshold
        let config = ConsensusConfig::default();
        assert!(!utils::has_enough_signatures(4, &config));
        assert!(utils::has_enough_signatures(5, &config));
        assert!(utils::has_enough_signatures(7, &config));

        // Test timeout calculation
        assert_eq!(utils::calculate_timeout(ViewNumber::new(0), 1000), 1000);
        assert_eq!(utils::calculate_timeout(ViewNumber::new(1), 1000), 2000);
        assert_eq!(utils::calculate_timeout(ViewNumber::new(2), 1000), 4000);
    }

    #[test]
    fn test_node_role() {
        assert_eq!(NodeRole::Primary.to_string(), "Primary");
        assert_eq!(NodeRole::Backup.to_string(), "Backup");
        assert_eq!(NodeRole::Observer.to_string(), "Observer");
    }
}

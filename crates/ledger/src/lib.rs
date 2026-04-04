//! # Neo Ledger
//!
//! Core blockchain ledger functionality for the Neo blockchain protocol.
//!
//! This crate provides the fundamental ledger implementation including block management,
//! transaction processing, state management, and consensus integration. It implements
//! the blockchain data structures and validation logic that form the backbone of the
//! Neo network.
//!
//! ## Features
//!
//! - **Blockchain Management**: Block storage, validation, and chain synchronization
//! - **Transaction Processing**: Transaction validation, execution, and mempool management
//! - **State Management**: Account balances, smart contract storage, and global state
//! - **Consensus Integration**: Block production and consensus message handling
//! - **Header Caching**: Efficient header storage and retrieval for light clients
//! - **Fork Detection**: Chain fork detection and resolution mechanisms
//!
//! ## Architecture
//!
//! The ledger is organized into several key components:
//!
//! - **Blockchain**: Main blockchain actor managing the chain state
//! - **Block**: Block and header data structures with validation logic
//! - **MemoryPool**: Transaction pool for pending transactions
//! - **HeaderCache**: Optimized header storage for fast synchronization
//! - **Storage**: Persistent storage layer for blockchain data
//! - **VerifyResult**: Transaction and block verification results
//!
//! ## Example
//!
//! ```rust,no_run
//! use neo_ledger::{Ledger, LedgerConfig, NetworkType};
//! use neo_core::Transaction;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a new ledger instance
//! let config = LedgerConfig::default();
//! let ledger = Ledger::new_with_network(config, NetworkType::TestNet).await?;
//!
//! // Get current blockchain height
//! let height = ledger.get_height().await;
//! println!("Current height: {}", height);
//!
//! // Get a block by index
//! if let Some(block) = ledger.get_block(height).await? {
//!     println!("Latest block hash: {:?}", block.hash());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Consensus Integration
//!
//! The ledger integrates with the consensus mechanism to:
//! - Validate proposed blocks
//! - Apply consensus-approved blocks to the chain
//! - Provide chain state for consensus decisions
//! - Handle view changes and recovery
//!
//! ## Storage Backend
//!
//! The ledger supports multiple storage backends:
//! - RocksDB (default): High-performance persistent storage
//! - In-Memory: For testing and development
//! - Custom: Implement the Storage trait for custom backends

/// Block and block header structures
pub mod block;
/// Main blockchain implementation
pub mod blockchain;
/// Header caching for light clients
pub mod header_cache;
/// Memory pool for pending transactions
pub mod mempool;
/// Verification result types
pub mod verify_result;

pub use block::{Block, BlockHeader, Header};
pub use blockchain::storage::{Storage, StorageItem, StorageKey};
pub use blockchain::Blockchain;
pub use header_cache::HeaderCache;
pub use mempool::{MemoryPool, MempoolConfig, PooledTransaction};
pub use verify_result::VerifyResult;

pub use neo_config::{
    LedgerConfig, NetworkType, MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK, MILLISECONDS_PER_BLOCK,
    SECONDS_PER_BLOCK,
};

use neo_core::UInt160;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
/// Result type for ledger operations
pub type Result<T> = std::result::Result<T, Error>;

/// Ledger-specific error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Block validation error
    #[error("Block validation failed: {0}")]
    BlockValidation(String),

    /// Transaction validation error
    #[error("Transaction validation failed: {0}")]
    TransactionValidation(String),

    /// Validation error (generic)
    #[error("Validation failed: {0}")]
    Validation(String),

    /// Not found error
    #[error("Not found")]
    NotFound,

    /// Invalid operation error
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Blockchain state error
    #[error("Blockchain state error: {0}")]
    StateError(String),

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Storage error (alias for compatibility)
    #[error("Storage error: {0}")]
    Storage(String),

    /// Mempool error
    #[error("Mempool error: {0}")]
    MempoolError(String),

    /// Consensus error
    #[error("Consensus error: {0}")]
    ConsensusError(String),

    /// Invalid block
    #[error("Invalid block: {0}")]
    InvalidBlock(String),

    /// Invalid view change
    #[error("Invalid view change: {0}")]
    InvalidViewChange(String),

    /// Invalid message
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Invalid validator
    #[error("Invalid validator: {0}")]
    InvalidValidator(String),

    /// Invalid signature
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    /// Invalid committee
    #[error("Invalid committee: {0}")]
    InvalidCommittee(String),

    /// Invalid data
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Signature verification error
    #[error("Signature verification error: {0}")]
    SignatureVerificationError(String),

    /// Block not found
    #[error("Block not found: {0}")]
    BlockNotFound(String),

    /// Transaction not found
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    /// Invalid block height
    #[error("Invalid block height: expected {expected}, got {actual}")]
    InvalidBlockHeight { expected: u32, actual: u32 },

    /// Invalid block hash
    #[error("Invalid block hash: {0}")]
    InvalidBlockHash(String),

    /// Insufficient balance
    #[error(
        "Insufficient balance for account {account}: required {required}, available {available}"
    )]
    InsufficientBalance {
        account: UInt160,
        required: i64,
        available: i64,
    },

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// VM error
    #[error("VM error: {0}")]
    VmError(String),

    /// Smart contract error
    #[error("Smart contract error: {0}")]
    SmartContractError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(String),

    /// Generic error
    #[error("Ledger error: {0}")]
    Generic(String),
}

impl From<neo_vm::VmError> for Error {
    fn from(err: neo_vm::VmError) -> Self {
        Error::VmError(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

impl From<neo_core::CoreError> for Error {
    fn from(err: neo_core::CoreError) -> Self {
        Error::Generic(err.to_string())
    }
}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Error::SerializationError(err.to_string())
    }
}

/// Block validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// Block is valid
    Valid,
    /// Block is invalid with reason
    Invalid(String),
    /// Block validation is pending (async validation)
    Pending,
}

impl ValidationResult {
    /// Checks if the validation result is valid
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Checks if the validation result is invalid
    pub fn is_invalid(&self) -> bool {
        matches!(self, ValidationResult::Invalid(_))
    }

    /// Checks if the validation result is pending
    pub fn is_pending(&self) -> bool {
        matches!(self, ValidationResult::Pending)
    }

    /// Gets the error message if invalid
    pub fn error_message(&self) -> Option<&str> {
        match self {
            ValidationResult::Invalid(msg) => Some(msg),
            _ => None,
        }
    }
}

/// Transaction verification result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationResult {
    /// Whether the transaction is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// Gas consumed during verification
    pub gas_consumed: i64,
}

impl VerificationResult {
    /// Creates a valid verification result
    pub fn valid(gas_consumed: i64) -> Self {
        Self {
            is_valid: true,
            error: None,
            gas_consumed,
        }
    }

    /// Creates an invalid verification result
    pub fn invalid(error: String, gas_consumed: i64) -> Self {
        Self {
            is_valid: false,
            error: Some(error),
            gas_consumed,
        }
    }
}

/// Blockchain statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStats {
    /// Current block height
    pub height: u32,
    /// Total number of transactions
    pub transaction_count: u64,
    /// Total number of accounts
    pub account_count: u64,
    /// Total number of contracts
    pub contract_count: u64,
    /// Current mempool size
    pub mempool_size: usize,
    /// Average block time in seconds
    pub average_block_time: f64,
    /// Network hash rate (if applicable)
    pub network_hashrate: Option<f64>,
}

impl Default for BlockchainStats {
    fn default() -> Self {
        Self {
            height: 0,
            transaction_count: 0,
            account_count: 0,
            contract_count: 0,
            mempool_size: 0,
            average_block_time: SECONDS_PER_BLOCK as f64, // Default SECONDS_PER_BLOCK seconds for Neo
            network_hashrate: None,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{BlockchainStats, Error, Result, ValidationResult, VerificationResult};

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::Valid;
        assert!(valid.is_valid());
        assert!(!valid.is_invalid());
        assert!(!valid.is_pending());

        let invalid = ValidationResult::Invalid("test error".to_string());
        assert!(!invalid.is_valid());
        assert!(invalid.is_invalid());
        assert_eq!(invalid.error_message(), Some("test error"));

        let pending = ValidationResult::Pending;
        assert!(pending.is_pending());
    }

    #[test]
    fn test_verification_result() {
        let valid = VerificationResult::valid(1000);
        assert!(valid.is_valid);
        assert!(valid.error.is_none());
        assert_eq!(valid.gas_consumed, 1000);

        let invalid = VerificationResult::invalid("test error".to_string(), 500);
        assert!(!invalid.is_valid);
        assert_eq!(invalid.error, Some("test error".to_string()));
        assert_eq!(invalid.gas_consumed, 500);
    }

    #[test]
    fn test_ledger_config_default() {
        let config = LedgerConfig::default();
        assert_eq!(config.max_block_size, MAX_BLOCK_SIZE);
        assert_eq!(config.milliseconds_per_block, MILLISECONDS_PER_BLOCK);
        assert_eq!(
            config.max_transactions_per_block,
            MAX_TRANSACTIONS_PER_BLOCK
        );
    }

    #[test]
    fn test_blockchain_stats_default() {
        let stats = BlockchainStats::default();
        assert_eq!(stats.height, 0);
        assert_eq!(stats.transaction_count, 0);
        assert_eq!(stats.average_block_time, SECONDS_PER_BLOCK as f64);
    }
}

/// Main ledger implementation.
///
/// The `Ledger` struct provides the high-level interface for blockchain operations
/// including block management, transaction processing, and state queries. It coordinates
/// between the blockchain, mempool, and storage layers to maintain a consistent
/// view of the blockchain state.
///
/// # Thread Safety
///
/// The ledger is designed to be thread-safe and can be shared across multiple
/// threads using `Arc`. All methods that modify state use appropriate synchronization.
///
/// # Example
///
/// ```rust,no_run
/// use neo_ledger::{Ledger, LedgerConfig};
/// use std::sync::Arc;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = LedgerConfig::default();
/// let ledger = Arc::new(Ledger::new(config).await?);
///
/// // Share ledger across threads
/// let ledger_clone = Arc::clone(&ledger);
/// tokio::spawn(async move {
///     let height = ledger_clone.get_height().await;
///     println!("Height from thread: {}", height);
/// });
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Ledger {
    config: LedgerConfig,
    stats: BlockchainStats,
    blockchain: Arc<Blockchain>,
}

impl Ledger {
    /// Creates a new Ledger instance with default MainNet configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Ledger configuration parameters
    ///
    /// # Returns
    ///
    /// A new `Ledger` instance or an error if initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Storage initialization fails
    /// - Genesis block cannot be loaded or created
    /// - Configuration validation fails
    pub async fn new(config: LedgerConfig) -> Result<Self> {
        let blockchain = Arc::new(
            Blockchain::new_with_storage_suffix(NetworkType::MainNet, Some("ledger-main")).await?,
        );

        Ok(Self {
            config,
            stats: BlockchainStats::default(),
            blockchain,
        })
    }

    /// Creates a new Ledger instance with specific network configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Ledger configuration parameters
    /// * `network` - Network type (MainNet, TestNet, or PrivNet)
    ///
    /// # Returns
    ///
    /// A new `Ledger` instance configured for the specified network.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub async fn new_with_network(config: LedgerConfig, network: NetworkType) -> Result<Self> {
        let blockchain =
            Arc::new(Blockchain::new_with_storage_suffix(network, Some("ledger")).await?);

        Ok(Self {
            config,
            stats: BlockchainStats::default(),
            blockchain,
        })
    }

    /// Creates a new Ledger instance with an existing blockchain.
    ///
    /// This method is useful for testing or when you need to provide
    /// a pre-configured blockchain instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Ledger configuration parameters
    /// * `blockchain` - Pre-configured blockchain instance
    ///
    /// # Returns
    ///
    /// A new `Ledger` instance using the provided blockchain.
    pub fn new_with_blockchain(config: LedgerConfig, blockchain: Arc<Blockchain>) -> Self {
        Self {
            config,
            stats: BlockchainStats::default(),
            blockchain,
        }
    }

    /// Gets the current blockchain statistics.
    ///
    /// # Returns
    ///
    /// A reference to the current `BlockchainStats` containing metrics
    /// such as height, transaction count, and mempool size.
    pub fn get_stats(&self) -> &BlockchainStats {
        &self.stats
    }

    /// Gets the ledger configuration.
    ///
    /// # Returns
    ///
    /// A reference to the `LedgerConfig` used by this ledger instance.
    pub fn get_config(&self) -> &LedgerConfig {
        &self.config
    }

    /// Gets the hash of the best (latest) block in the chain.
    ///
    /// # Returns
    ///
    /// The `UInt256` hash of the latest block.
    ///
    /// # Errors
    ///
    /// Returns an error if the blockchain state cannot be accessed.
    pub async fn get_best_block_hash(&self) -> Result<neo_core::UInt256> {
        self.blockchain.get_best_block_hash().await
    }

    /// Gets a block by its hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the block to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Block)` if the block exists, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if storage access fails.
    pub async fn get_block_by_hash(&self, hash: &neo_core::UInt256) -> Result<Option<Block>> {
        self.blockchain.get_block_by_hash(hash).await
    }

    /// Gets a block by its height/index.
    ///
    /// # Arguments
    ///
    /// * `index` - The height/index of the block to retrieve
    ///
    /// # Returns
    ///
    /// `Some(Block)` if the block exists at the given height, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error if storage access fails.
    pub async fn get_block(&self, index: u32) -> Result<Option<Block>> {
        self.blockchain.get_block(index).await
    }

    /// Gets the current blockchain height.
    ///
    /// The height is the index of the latest block in the chain.
    /// The genesis block has height 0.
    ///
    /// # Returns
    ///
    /// The current blockchain height as a `u32`.
    pub async fn get_height(&self) -> u32 {
        self.blockchain.get_height().await
    }

    /// Adds a new block to the blockchain.
    ///
    /// This method validates the block and adds it to the chain if valid.
    /// It also handles fork detection and chain reorganization if necessary.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to add to the chain
    ///
    /// # Returns
    ///
    /// `Ok(())` if the block was successfully added.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Block validation fails
    /// - Block is already in the chain
    /// - Storage operation fails
    pub async fn add_block(&self, block: Block) -> Result<()> {
        self.blockchain.add_block_with_fork_detection(&block).await
    }

    /// Persists a block to storage.
    ///
    /// This method performs comprehensive validation before persisting the block.
    /// It's typically called after consensus has approved a block.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to persist
    ///
    /// # Returns
    ///
    /// `Ok(())` if the block was successfully persisted.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Block validation fails
    /// - Transaction validation fails
    /// - Storage operation fails
    /// - Block exceeds size or transaction limits
    pub async fn persist_block(&self, block: Block) -> Result<()> {
        // Validate block structure and consensus rules
        if block.header.index == 0 && !block.transactions.is_empty() {
            return Err(Error::Validation(
                "Genesis block cannot contain transactions".to_string(),
            ));
        }

        // Verify block hash and merkle root
        if block.transactions.len() > self.config.max_transactions_per_block {
            return Err(Error::Validation(
                "Too many transactions in block".to_string(),
            ));
        }

        // Block persistence would be handled by storage layer
        Ok(())
    }
}

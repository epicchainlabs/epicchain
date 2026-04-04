// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! # Neo Core
//!
//! Core functionality for the Neo blockchain implementation.
//!
//! This crate provides the fundamental types, traits, and utilities that form
//! the foundation of the Neo blockchain protocol. It includes essential components
//! for blocks, transactions, cryptographic operations, and system management.
//!
//! ## Features
//!
//! - **Type System**: Core types like `UInt160`, `UInt256`, and `BigDecimal`
//! - **Block Structure**: Block and block header implementations
//! - **Transaction System**: Transaction types, attributes, and validation
//! - **Witness System**: Witness, signer, and witness rule implementations
//! - **Error Handling**: Comprehensive error types and result handling
//! - **Monitoring**: System metrics and performance monitoring
//! - **Shutdown Coordination**: Graceful shutdown mechanisms
//!
//! ## Example
//!
//! ```rust,no_run
//! use neo_core::{UInt256, Transaction, Block};
//!
//! // Create a new transaction hash
//! let hash = UInt256::zero();
//!
//! // Work with transactions and blocks
//! // let transaction = Transaction::new();
//! // let block = Block::new();
//! ```
//!
//! ## Architecture
//!
//! The core crate is organized into several key modules:
//!
//! - **Basic Types**: `uint160`, `uint256`, `big_decimal` - Fundamental data types
//! - **Blockchain**: `block`, `transaction`, `witness` - Blockchain primitives
//! - **System**: `neo_system`, `shutdown`, `monitoring` - System management
//! - **Utilities**: `error_handling`, `safe_operations` - Helper functionality

//#![warn(missing_docs)]
//#![warn(rustdoc::missing_crate_level_docs)]

// Module declarations with documentation
/// Advanced performance metrics and monitoring
pub mod advanced_metrics;
/// Big decimal arithmetic implementation
pub mod big_decimal;
/// Block and block header structures
pub mod block;
/// Builder pattern implementations for complex types
pub mod builders;
/// System-wide constants
pub mod constants;
/// Enhanced error handling patterns for blockchain operations
pub mod enhanced_error_handling;
/// Core error types and error handling
pub mod error;
/// Advanced error handling utilities
pub mod error_handling;
/// Error utility functions
pub mod error_utils;
/// Event system for blockchain events
pub mod events;
/// Core extensions and utility traits
pub mod extensions;
/// Hard fork management
pub mod hardfork;
/// System metrics collection
pub mod metrics;
/// Migration helpers for transitioning to safe error handling
pub mod migration_helpers;
/// Monitoring and alerting system
pub mod monitoring;
/// Neo system management
pub mod neo_system;
/// Production monitoring and health checks
pub mod production_monitoring;
/// Enhanced safe error handling system
pub mod safe_error_handling;
/// Safe memory operations for core types
pub mod safe_memory;
/// Safe arithmetic and type operations
pub mod safe_operations;
pub mod safe_result;
/// Graceful shutdown coordination
pub mod shutdown;
/// Transaction signer implementation
pub mod signer;
/// System-wide monitoring and metrics
pub mod system_monitoring;
/// Transaction structures and validation
pub mod transaction;
/// Transaction type definitions
pub mod transaction_type;
/// Transaction validation module
pub mod transaction_validator;
/// 160-bit unsigned integer implementation
pub mod uint160;
/// 256-bit unsigned integer implementation
pub mod uint256;
pub mod unwrap_migration;
/// Witness verification system
pub mod witness;
/// Witness rule evaluation
pub mod witness_rule;
pub mod witness_safe;
/// Witness scope definitions
pub mod witness_scope;

// Re-exports for convenient access
pub use big_decimal::BigDecimal;
pub use block::{Block, BlockHeader};
pub use error::{CoreError, CoreResult, Result};
pub use neo_system::NeoSystem;
pub use shutdown::{
    Shutdown, ShutdownCoordinator, ShutdownError, ShutdownEvent, ShutdownStage, SignalHandler,
};
pub use signer::Signer;
pub use transaction::{
    OracleResponseCode, Transaction, TransactionAttribute, TransactionAttributeType, HEADER_SIZE,
    MAX_TRANSACTION_ATTRIBUTES, MAX_TRANSACTION_SIZE,
};
pub use transaction_type::ContainsTransactionType;
pub use uint160::UInt160;
pub use uint256::UInt256;
pub use witness::Witness;
pub use witness_rule::{WitnessCondition, WitnessConditionType, WitnessRule, WitnessRuleAction};
pub use witness_scope::WitnessScope;

use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

/// Global blockchain singleton instance.
///
/// This provides thread-safe access to the blockchain state and matches
/// the C# implementation's `Blockchain.Singleton` pattern exactly.
///
/// # Thread Safety
///
/// The singleton is protected by a `RwLock` to allow multiple concurrent
/// readers while ensuring exclusive write access.
pub static GLOBAL_BLOCKCHAIN: Lazy<
    Arc<RwLock<Option<transaction::blockchain::BlockchainSingleton>>>,
> = Lazy::new(|| Arc::new(RwLock::new(None)));

/// Global persistence store singleton instance.
///
/// This provides thread-safe access to the underlying storage layer and
/// matches the C# implementation's `Store.Singleton` pattern exactly.
///
/// # Thread Safety
///
/// The store is protected by a `RwLock` and uses dynamic dispatch to
/// support different storage backend implementations.
#[allow(clippy::type_complexity)]
pub static GLOBAL_STORE: Lazy<
    Arc<RwLock<Option<Box<dyn transaction::blockchain::PersistenceStore + Send + Sync>>>>,
> = Lazy::new(|| Arc::new(RwLock::new(None)));

/// Trait for verifiable blockchain objects.
///
/// This trait defines the interface for objects that can be cryptographically
/// verified, such as blocks and transactions. It provides methods for verification,
/// hashing, and type conversion.
///
/// # Implementation Requirements
///
/// Types implementing this trait must provide:
/// - Verification logic
/// - Hash computation
/// - Serialization for signing
/// - Type erasure support via `Any`
pub trait IVerifiable: std::any::Any {
    /// Verifies the cryptographic validity of the object.
    ///
    /// # Returns
    ///
    /// `true` if the object is valid, `false` otherwise.
    fn verify(&self) -> bool;

    /// Computes the hash of the object.
    ///
    /// # Returns
    ///
    /// A `CoreResult` containing the computed hash or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if hash computation fails.
    fn hash(&self) -> CoreResult<UInt256>;

    /// Gets the serialized data used for hash computation.
    ///
    /// This method returns the byte representation that will be hashed
    /// to produce the object's identifier.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the hashable data.
    fn get_hash_data(&self) -> Vec<u8>;

    /// Returns a reference to self as `Any` for downcasting.
    ///
    /// This enables runtime type checking and downcasting to concrete types.
    ///
    /// # Returns
    ///
    /// A reference to self as a trait object.
    fn as_any(&self) -> &dyn std::any::Any;
}

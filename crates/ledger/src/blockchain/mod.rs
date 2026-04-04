//! Blockchain module.
//!
//! This module provides the main blockchain functionality exactly matching C# Neo Blockchain.
//! The module is organized following C# Neo's blockchain structure:
//! - storage: Storage interface and implementation (matches C# Storage classes)
//! - genesis: Genesis block creation and initialization (matches C# Genesis handling)
//! - verification: Block, header, and transaction verification (matches C# verification logic)
//! - persistence: Block persistence and storage management (matches C# persistence layer)
//! - state: Blockchain state management (matches C# state handling)
//! - blockchain: Main Blockchain struct (matches C# Blockchain class)

pub mod advanced_validation;
pub mod blockchain;
pub mod genesis;
pub mod import;
pub mod persistence;
pub mod state;
pub mod storage;
pub mod validation;
pub mod verification;

pub use blockchain::{Blockchain, BlockchainStats};
pub use genesis::GenesisManager;
pub use persistence::{BlockchainPersistence, BlockchainSnapshot};
pub use state::{BlockchainState, PolicySettings};
pub use storage::{RocksDBStorage, Storage, StorageItem, StorageKey, StorageProvider};
pub use verification::{BlockchainVerifier, VerifyResult};

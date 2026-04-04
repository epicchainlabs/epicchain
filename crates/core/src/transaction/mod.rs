// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Transaction module for Neo blockchain transactions.
//!
//! This module provides the core Transaction implementation that matches
//! the C# Neo N3 implementation exactly, broken down into logical components:
//!
//! - `transaction` - Core Transaction struct and basic operations
//! - `attributes` - Transaction attributes (HighPriority, Oracle, etc.)
//! - `validation` - Transaction verification and validation logic
//! - `blockchain` - Blockchain state integration (snapshots, storage)
//! - `vm` - VM integration (ApplicationEngine, VMState)

pub mod attributes;
pub mod blockchain;
pub mod core;
pub mod serialization;
pub mod validation;
pub mod vm;

pub use attributes::{OracleResponseCode, TransactionAttribute, TransactionAttributeType};
pub use blockchain::BlockchainSnapshot;
pub use core::Transaction;
pub use vm::ApplicationEngine;

// Re-export constants
pub use core::{HEADER_SIZE, MAX_TRANSACTION_ATTRIBUTES, MAX_TRANSACTION_SIZE};

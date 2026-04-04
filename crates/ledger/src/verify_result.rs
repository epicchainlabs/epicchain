//! Verification result types.
//!
//! This module provides verification result types that exactly match C# Neo VerifyResult.

use serde::{Deserialize, Serialize};

/// Verification result for blockchain operations (matches C# VerifyResult exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerifyResult {
    /// Indicates that the verification was successful
    Succeed,
    /// Indicates that an inventory with the same hash already exists
    AlreadyExists,
    /// Indicates that an inventory with the same hash already exists in the memory pool
    AlreadyInPool,
    /// Indicates that the memory pool is full and the transaction cannot be verified
    OutOfMemory,
    /// Indicates that the previous block has not been received, so the block cannot be verified
    UnableToVerify,
    /// Indicates that the inventory is invalid
    Invalid,
    /// Indicates that the transaction has an invalid script
    InvalidScript,
    /// Indicates that the transaction has an invalid attribute
    InvalidAttribute,
    /// Indicates that the inventory has an invalid signature
    InvalidSignature,
    /// Indicates that the witness is invalid
    InvalidWitness,
    /// Indicates that the size of the inventory is not allowed
    OverSize,
    /// Indicates that the transaction has expired
    Expired,
    /// Indicates that the transaction failed to verify due to insufficient funds
    InsufficientFunds,
    /// Indicates that the transaction failed to verify because it didn't comply with the policy
    PolicyFail,
    /// Indicates that the transaction failed to verify because it conflicts with transactions
    HasConflicts,
    /// Indicates that the inventory failed to verify due to other reasons
    Unknown,
}

impl Default for VerifyResult {
    fn default() -> Self {
        VerifyResult::Unknown
    }
}

impl From<u8> for VerifyResult {
    fn from(value: u8) -> Self {
        match value {
            0 => VerifyResult::Succeed,
            1 => VerifyResult::AlreadyExists,
            2 => VerifyResult::AlreadyInPool,
            3 => VerifyResult::OutOfMemory,
            4 => VerifyResult::PolicyFail,
            5 => VerifyResult::Invalid,
            6 => VerifyResult::InsufficientFunds,
            7 => VerifyResult::Expired,
            8 => VerifyResult::InvalidAttribute,
            9 => VerifyResult::InvalidScript,
            10 => VerifyResult::InvalidSignature,
            11 => VerifyResult::InvalidWitness,
            12 => VerifyResult::OverSize,
            13 => VerifyResult::UnableToVerify,
            14 => VerifyResult::HasConflicts,
            _ => VerifyResult::Unknown,
        }
    }
}

impl From<VerifyResult> for u8 {
    fn from(result: VerifyResult) -> Self {
        result as u8
    }
}

impl std::fmt::Display for VerifyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyResult::Succeed => write!(f, "Succeed"),
            VerifyResult::AlreadyExists => write!(f, "AlreadyExists"),
            VerifyResult::AlreadyInPool => write!(f, "AlreadyInPool"),
            VerifyResult::OutOfMemory => write!(f, "OutOfMemory"),
            VerifyResult::PolicyFail => write!(f, "PolicyFail"),
            VerifyResult::Invalid => write!(f, "Invalid"),
            VerifyResult::InsufficientFunds => write!(f, "InsufficientFunds"),
            VerifyResult::Expired => write!(f, "Expired"),
            VerifyResult::InvalidAttribute => write!(f, "InvalidAttribute"),
            VerifyResult::InvalidScript => write!(f, "InvalidScript"),
            VerifyResult::InvalidSignature => write!(f, "InvalidSignature"),
            VerifyResult::InvalidWitness => write!(f, "InvalidWitness"),
            VerifyResult::OverSize => write!(f, "OverSize"),
            VerifyResult::UnableToVerify => write!(f, "UnableToVerify"),
            VerifyResult::HasConflicts => write!(f, "HasConflicts"),
            VerifyResult::Unknown => write!(f, "Unknown"),
        }
    }
}

//! Block structure for Neo blockchain
//!
//! This module provides the core Block type that is shared across modules

use crate::{Transaction, UInt160, UInt256, Witness};
use serde::{Deserialize, Serialize};

/// Block header structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block version
    pub version: u32,
    /// Previous block hash
    pub previous_hash: UInt256,
    /// Merkle root of transactions
    pub merkle_root: UInt256,
    /// Block timestamp
    pub timestamp: u64,
    /// Block nonce
    pub nonce: u64,
    /// Block index
    pub index: u32,
    /// Primary index (consensus node that created block)
    pub primary_index: u8,
    /// Next consensus address
    pub next_consensus: UInt160,
    /// Witness for the block
    pub witnesses: Vec<Witness>,
}

impl BlockHeader {
    /// Calculate the hash of the block header
    pub fn hash(&self) -> crate::Result<UInt256> {
        use sha2::{Digest, Sha256};

        let mut buffer = Vec::new();

        // Serialize header fields in the same order as C# Neo implementation
        buffer.extend_from_slice(&self.version.to_le_bytes());
        buffer.extend_from_slice(&self.previous_hash.as_bytes());
        buffer.extend_from_slice(&self.merkle_root.as_bytes());
        buffer.extend_from_slice(&self.timestamp.to_le_bytes());
        buffer.extend_from_slice(&self.nonce.to_le_bytes());
        buffer.extend_from_slice(&self.index.to_le_bytes());
        buffer.push(self.primary_index);
        buffer.extend_from_slice(&self.next_consensus.as_bytes());

        // Calculate SHA256 hash
        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let hash = hasher.finalize();

        // Convert to UInt256
        UInt256::from_bytes(&hash).map_err(|e| crate::CoreError::Serialization {
            message: format!("Hash conversion failed: {e}"),
        })
    }
}

/// Block structure containing header and transactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Transactions in the block
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Calculate the hash of the block (same as header hash)
    pub fn hash(&self) -> crate::Result<UInt256> {
        self.header.hash()
    }

    /// Get the block index
    pub fn index(&self) -> u32 {
        self.header.index
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> u64 {
        self.header.timestamp
    }

    /// Get the previous block hash
    pub fn prev_hash(&self) -> &UInt256 {
        &self.header.previous_hash
    }

    /// Calculate the size of the block in bytes
    pub fn size(&self) -> usize {
        // Rough estimate of block size
        // In a real implementation, this would be the exact serialized size
        std::mem::size_of::<BlockHeader>()
            + self.transactions.len() * std::mem::size_of::<Transaction>()
    }
}

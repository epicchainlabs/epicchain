//! Block builder for constructing blocks.
//!
//! This module implements block building functionality exactly matching C# Neo's block construction.

use super::{header::BlockHeader, Block, MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK};
use crate::{Error, Result};
use neo_config::{ADDRESS_SIZE, HASH_SIZE};
use neo_core::{Transaction, UInt160, UInt256, Witness};
use neo_cryptography::MerkleTree;
use neo_io::BinaryWriter;

/// Block builder for constructing new blocks (matches C# Neo block construction)
#[derive(Debug, Clone)]
pub struct BlockBuilder {
    version: u32,
    previous_hash: UInt256,
    timestamp: u64,
    nonce: u64,
    index: u32,
    primary_index: u8,
    next_consensus: UInt160,
    witnesses: Vec<Witness>,
    transactions: Vec<Transaction>,
}

impl BlockBuilder {
    /// Creates a new block builder
    pub fn new() -> Self {
        Self {
            version: 0,
            previous_hash: UInt256::zero(),
            timestamp: 0,
            nonce: 0,
            index: 0,
            primary_index: 0,
            next_consensus: UInt160::zero(),
            witnesses: Vec::new(),
            transactions: Vec::new(),
        }
    }

    /// Sets the block version
    pub fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Sets the previous block hash
    pub fn previous_hash(mut self, hash: UInt256) -> Self {
        self.previous_hash = hash;
        self
    }

    /// Sets the block timestamp
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the current timestamp
    pub fn current_timestamp(mut self) -> Self {
        self.timestamp = BlockHeader::current_timestamp();
        self
    }

    /// Sets the block nonce
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = nonce;
        self
    }

    /// Sets the block index (height)
    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }

    /// Sets the primary index
    pub fn primary_index(mut self, primary_index: u8) -> Self {
        self.primary_index = primary_index;
        self
    }

    /// Sets the next consensus address
    pub fn next_consensus(mut self, next_consensus: UInt160) -> Self {
        self.next_consensus = next_consensus;
        self
    }

    /// Adds a witness to the block
    pub fn add_witness(mut self, witness: Witness) -> Self {
        self.witnesses.push(witness);
        self
    }

    /// Sets all witnesses
    pub fn witnesses(mut self, witnesses: Vec<Witness>) -> Self {
        self.witnesses = witnesses;
        self
    }

    /// Adds a transaction to the block
    pub fn add_transaction(mut self, transaction: Transaction) -> Result<Self> {
        // Check transaction count limit
        if self.transactions.len() >= MAX_TRANSACTIONS_PER_BLOCK {
            return Err(Error::InvalidOperation("Too many transactions".to_string()));
        }

        self.transactions.push(transaction);
        Ok(self)
    }

    /// Adds multiple transactions to the block
    pub fn add_transactions(mut self, transactions: Vec<Transaction>) -> Result<Self> {
        // Check total transaction count limit
        if self.transactions.len() + transactions.len() > MAX_TRANSACTIONS_PER_BLOCK {
            return Err(Error::InvalidOperation("Too many transactions".to_string()));
        }

        self.transactions.extend(transactions);
        Ok(self)
    }

    /// Sets all transactions
    pub fn transactions(mut self, transactions: Vec<Transaction>) -> Result<Self> {
        if transactions.len() > MAX_TRANSACTIONS_PER_BLOCK {
            return Err(Error::InvalidOperation("Too many transactions".to_string()));
        }

        self.transactions = transactions;
        Ok(self)
    }

    /// Builds the block with calculated merkle root
    pub fn build(self) -> Result<Block> {
        // Calculate merkle root from transactions
        let merkle_root = if self.transactions.is_empty() {
            UInt256::zero()
        } else {
            let tx_hashes: std::result::Result<Vec<UInt256>, _> =
                self.transactions.iter().map(|tx| tx.hash()).collect();

            match tx_hashes {
                Ok(hashes) => {
                    let hash_bytes: Vec<Vec<u8>> =
                        hashes.iter().map(|h| h.as_bytes().to_vec()).collect();

                    match MerkleTree::compute_root(&hash_bytes) {
                        Some(root) => {
                            UInt256::from_bytes(&root).unwrap_or_else(|_| UInt256::zero())
                        }
                        None => UInt256::zero(),
                    }
                }
                Err(_) => UInt256::zero(),
            }
        };

        // Create header
        let mut header = BlockHeader::new(
            self.version,
            self.previous_hash,
            merkle_root,
            self.timestamp,
            self.nonce,
            self.index,
            self.primary_index,
            self.next_consensus,
        );

        // Add witnesses to header
        header.witnesses = self.witnesses;

        // Create block
        let block = Block::new(header, self.transactions);

        // Validate block size
        if block.size() > MAX_BLOCK_SIZE {
            return Err(Error::InvalidOperation(
                "Block size exceeds limit".to_string(),
            ));
        }

        Ok(block)
    }

    /// Builds from a previous block (sets appropriate fields)
    pub fn from_previous(mut self, previous_block: &Block) -> Self {
        self.previous_hash = previous_block.hash();
        self.index = previous_block.index() + 1;
        self.next_consensus = previous_block.header.next_consensus;
        self
    }

    /// Creates a genesis block builder
    pub fn genesis(next_consensus: UInt160) -> Self {
        Self::new()
            .version(0)
            .previous_hash(UInt256::zero())
            .index(0)
            .primary_index(0)
            .next_consensus(next_consensus)
            .current_timestamp()
    }

    /// Gets the current transaction count
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Gets the estimated block size
    pub fn estimated_size(&self) -> usize {
        // Calculate estimated header size
        let header_size = 4 + HASH_SIZE + HASH_SIZE + 8 + 8 + 4 + 1 + ADDRESS_SIZE; // Basic header fields

        // Add witness data size
        let witness_size: usize = self
            .witnesses
            .iter()
            .map(|w| w.invocation_script.len() + w.verification_script.len() + 8) // +8 for length prefixes
            .sum();

        let tx_size: usize = self
            .transactions
            .iter()
            .map(|tx| {
                let mut writer = BinaryWriter::new();
                let _ = <Transaction as neo_io::Serializable>::serialize(tx, &mut writer);
                writer.to_bytes().len()
            })
            .sum();

        header_size + witness_size + tx_size + 16 // +16 for various length prefixes
    }

    /// Checks if adding a transaction would exceed size limits
    pub fn can_add_transaction(&self, transaction: &Transaction) -> bool {
        // Check count limit
        if self.transactions.len() >= MAX_TRANSACTIONS_PER_BLOCK {
            return false;
        }

        // Check size limit
        let tx_size = {
            let mut writer = BinaryWriter::new();
            let _ = <Transaction as neo_io::Serializable>::serialize(transaction, &mut writer);
            writer.to_bytes().len()
        };

        self.estimated_size() + tx_size <= MAX_BLOCK_SIZE
    }
}

impl Default for BlockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_block_builder_creation() {
        let builder = BlockBuilder::new();
        assert_eq!(builder.transaction_count(), 0);
        assert_eq!(builder.version, 0);
        assert_eq!(builder.index, 0);
    }

    #[test]
    fn test_block_builder_chain() {
        let next_consensus = UInt160::from_bytes(&[1; ADDRESS_SIZE])
            .expect("Fixed-size array should be valid UInt160");
        let block = BlockBuilder::new()
            .version(0)
            .index(0)
            .primary_index(0)
            .next_consensus(next_consensus)
            .current_timestamp()
            .build()
            .unwrap();

        assert_eq!(block.index(), 0);
        assert_eq!(block.header.next_consensus, next_consensus);
        assert_eq!(block.transaction_count(), 0);
    }

    #[test]
    fn test_genesis_block_builder() {
        let next_consensus = UInt160::from_bytes(&[1; ADDRESS_SIZE])
            .expect("Fixed-size array should be valid UInt160");
        let block = BlockBuilder::genesis(next_consensus)
            .build()
            .expect("Block builder should succeed with valid inputs");

        assert!(block.is_genesis());
        assert_eq!(block.index(), 0);
        assert_eq!(block.header.previous_hash, UInt256::zero());
        assert_eq!(block.header.next_consensus, next_consensus);
    }

    #[test]
    fn test_from_previous_block() {
        let next_consensus = UInt160::from_bytes(&[1; ADDRESS_SIZE])
            .expect("Fixed-size array should be valid UInt160");
        let genesis = BlockBuilder::genesis(next_consensus)
            .build()
            .expect("Block builder should succeed with valid inputs");

        let block = BlockBuilder::new()
            .from_previous(&genesis)
            .current_timestamp()
            .build()
            .unwrap();

        assert_eq!(block.index(), 1);
        assert_eq!(block.header.previous_hash, genesis.hash());
        assert_eq!(block.header.next_consensus, next_consensus);
    }
}

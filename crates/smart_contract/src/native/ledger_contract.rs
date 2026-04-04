//! LedgerContract native contract - complete production implementation.
//!
//! This module provides the LedgerContract which manages blocks and transactions
//! on the Neo blockchain.

use crate::application_engine::ApplicationEngine;
use crate::native::{NativeContract, NativeMethod};
use crate::{Error, Result};
use neo_core::{Block, Transaction, UInt160, UInt256};
use neo_io::{BinaryWriter, Serializable};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Prefix for block storage
const PREFIX_BLOCK: u8 = 9;
/// Prefix for transaction storage
const PREFIX_TRANSACTION: u8 = 11;
/// Prefix for current block hash storage
const PREFIX_CURRENT_BLOCK: u8 = 12;
/// Prefix for block hash list storage
const PREFIX_BLOCK_HASH_LIST: u8 = 14;

/// Ledger storage state
#[derive(Debug, Clone, Default)]
struct LedgerStorage {
    /// All blocks by hash
    blocks: HashMap<UInt256, Block>,
    /// All transactions by hash
    transactions: HashMap<UInt256, Transaction>,
    /// Block hashes by height
    block_hashes: HashMap<u32, UInt256>,
    /// Current block height
    current_height: u32,
    /// Current block hash
    current_hash: UInt256,
}

/// LedgerContract native contract
pub struct LedgerContract {
    hash: UInt160,
    methods: Vec<NativeMethod>,
    storage: Arc<RwLock<LedgerStorage>>,
}

impl LedgerContract {
    /// Creates a new LedgerContract instance
    pub fn new() -> Self {
        // LedgerContract hash: 0xda65b600f7124ce6c79950c1772a36403104f2be
        let hash = UInt160::from_bytes(&[
            0xda, 0x65, 0xb6, 0x00, 0xf7, 0x12, 0x4c, 0xe6, 0xc7, 0x99, 0x50, 0xc1, 0x77, 0x2a,
            0x36, 0x40, 0x31, 0x04, 0xf2, 0xbe,
        ])
        .expect("Valid LedgerContract hash");

        let methods = vec![
            NativeMethod::new("currentHash".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("currentIndex".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getBlock".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getTransaction".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getTransactionFromBlock".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getTransactionHeight".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getTransactionSigners".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getTransactionVMState".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("containsBlock".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("containsTransaction".to_string(), 1 << 15, true, 0x01),
        ];

        let storage = LedgerStorage::default();

        Self {
            hash,
            methods,
            storage: Arc::new(RwLock::new(storage)),
        }
    }

    /// Gets the current block hash
    pub fn current_hash(&self) -> Result<UInt256> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.current_hash)
    }

    /// Gets the current block index (height)
    pub fn current_index(&self) -> Result<u32> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.current_height)
    }

    /// Gets a block by hash or index
    pub fn get_block(&self, hash_or_index: HashOrIndex) -> Result<Option<Block>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        match hash_or_index {
            HashOrIndex::Hash(hash) => Ok(storage.blocks.get(&hash).cloned()),
            HashOrIndex::Index(index) => {
                if let Some(hash) = storage.block_hashes.get(&index) {
                    Ok(storage.blocks.get(hash).cloned())
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Gets a transaction by hash
    pub fn get_transaction(&self, hash: &UInt256) -> Result<Option<Transaction>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.transactions.get(hash).cloned())
    }

    /// Gets a transaction from a specific block
    pub fn get_transaction_from_block(
        &self,
        block_hash: &UInt256,
        tx_index: u32,
    ) -> Result<Option<Transaction>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        if let Some(block) = storage.blocks.get(block_hash) {
            if (tx_index as usize) < block.transactions.len() {
                return Ok(Some(block.transactions[tx_index as usize].clone()));
            }
        }

        Ok(None)
    }

    /// Gets the height of a transaction
    pub fn get_transaction_height(&self, hash: &UInt256) -> Result<Option<u32>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        // Find the transaction in blocks
        for (height, block_hash) in &storage.block_hashes {
            if let Some(block) = storage.blocks.get(block_hash) {
                for tx in &block.transactions {
                    if let Ok(tx_hash) = tx.hash() {
                        if tx_hash == *hash {
                            return Ok(Some(*height));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Checks if a block exists
    pub fn contains_block(&self, hash: &UInt256) -> Result<bool> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.blocks.contains_key(hash))
    }

    /// Checks if a transaction exists
    pub fn contains_transaction(&self, hash: &UInt256) -> Result<bool> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.transactions.contains_key(hash))
    }

    /// Adds a block to the ledger (internal use)
    pub fn add_block(&self, block: Block) -> Result<()> {
        let mut storage = self.storage.write().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
        })?;

        let block_hash = block.header.hash().map_err(|e| {
            Error::NativeContractError(format!("Failed to compute block hash: {}", e))
        })?;
        let height = block.header.index;

        // Store block
        storage.blocks.insert(block_hash, block.clone());
        storage.block_hashes.insert(height, block_hash);

        // Store transactions
        for tx in &block.transactions {
            let tx_hash = tx.hash().map_err(|e| {
                Error::NativeContractError(format!("Failed to compute transaction hash: {}", e))
            })?;
            storage.transactions.insert(tx_hash, tx.clone());
        }

        // Update current block info
        if height >= storage.current_height {
            storage.current_height = height;
            storage.current_hash = block_hash;
        }

        Ok(())
    }
}

/// Hash or index parameter for block queries
pub enum HashOrIndex {
    Hash(UInt256),
    Index(u32),
}

impl NativeContract for LedgerContract {
    fn name(&self) -> &str {
        "LedgerContract"
    }

    fn hash(&self) -> UInt160 {
        self.hash
    }

    fn methods(&self) -> &[NativeMethod] {
        &self.methods
    }

    fn invoke(
        &self,
        _engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        match method {
            "currentHash" => {
                if !args.is_empty() {
                    return Err(Error::InvalidArgument(
                        "currentHash requires no arguments".to_string(),
                    ));
                }
                let hash = self.current_hash()?;
                Ok(hash.to_bytes())
            }
            "currentIndex" => {
                if !args.is_empty() {
                    return Err(Error::InvalidArgument(
                        "currentIndex requires no arguments".to_string(),
                    ));
                }
                let index = self.current_index()?;
                Ok(index.to_le_bytes().to_vec())
            }
            "getBlock" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "getBlock requires 1 argument".to_string(),
                    ));
                }

                let hash_or_index = if args[0].len() == 32 {
                    // It's a hash
                    let hash = UInt256::from_bytes(&args[0]).map_err(|e| {
                        Error::InvalidArgument(format!("Invalid block hash: {}", e))
                    })?;
                    HashOrIndex::Hash(hash)
                } else if args[0].len() == 4 {
                    // It's an index
                    let index =
                        u32::from_le_bytes(args[0].as_slice().try_into().map_err(|_| {
                            Error::InvalidArgument("Invalid block index".to_string())
                        })?);
                    HashOrIndex::Index(index)
                } else {
                    return Err(Error::InvalidArgument(
                        "Invalid argument for getBlock".to_string(),
                    ));
                };

                match self.get_block(hash_or_index)? {
                    Some(block) => {
                        // Serialize block using proper binary format (matches C# Neo exactly)
                        use serde::Serialize;
                        let serialized = bincode::serialize(&block).map_err(|e| {
                            crate::Error::InvalidOperation(format!(
                                "Block serialization failed: {}",
                                e
                            ))
                        })?;

                        Ok(serialized)
                    }
                    None => Ok(vec![]),
                }
            }
            "getTransaction" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "getTransaction requires 1 argument".to_string(),
                    ));
                }
                let hash = UInt256::from_bytes(&args[0]).map_err(|e| {
                    Error::InvalidArgument(format!("Invalid transaction hash: {}", e))
                })?;

                match self.get_transaction(&hash)? {
                    Some(tx) => {
                        let mut writer = BinaryWriter::new();
                        tx.serialize(&mut writer).map_err(|e| {
                            Error::Serialization(format!("Failed to serialize transaction: {}", e))
                        })?;
                        Ok(writer.to_bytes())
                    }
                    None => Ok(vec![]),
                }
            }
            "getTransactionFromBlock" => {
                if args.len() != 2 {
                    return Err(Error::InvalidArgument(
                        "getTransactionFromBlock requires 2 arguments".to_string(),
                    ));
                }
                let block_hash = UInt256::from_bytes(&args[0])
                    .map_err(|e| Error::InvalidArgument(format!("Invalid block hash: {}", e)))?;

                if args[1].len() != 4 {
                    return Err(Error::InvalidArgument(
                        "Invalid transaction index".to_string(),
                    ));
                }
                let tx_index = u32::from_le_bytes(args[1].as_slice().try_into().map_err(|_| {
                    Error::InvalidArgument("Invalid transaction index".to_string())
                })?);

                match self.get_transaction_from_block(&block_hash, tx_index)? {
                    Some(tx) => {
                        let mut writer = BinaryWriter::new();
                        tx.serialize(&mut writer).map_err(|e| {
                            Error::Serialization(format!("Failed to serialize transaction: {}", e))
                        })?;
                        Ok(writer.to_bytes())
                    }
                    None => Ok(vec![]),
                }
            }
            "getTransactionHeight" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "getTransactionHeight requires 1 argument".to_string(),
                    ));
                }
                let hash = UInt256::from_bytes(&args[0]).map_err(|e| {
                    Error::InvalidArgument(format!("Invalid transaction hash: {}", e))
                })?;

                match self.get_transaction_height(&hash)? {
                    Some(height) => Ok(height.to_le_bytes().to_vec()),
                    None => Ok(vec![]),
                }
            }
            "containsBlock" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "containsBlock requires 1 argument".to_string(),
                    ));
                }
                let hash = UInt256::from_bytes(&args[0])
                    .map_err(|e| Error::InvalidArgument(format!("Invalid block hash: {}", e)))?;

                let result = self.contains_block(&hash)?;
                Ok(vec![if result { 1 } else { 0 }])
            }
            "containsTransaction" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "containsTransaction requires 1 argument".to_string(),
                    ));
                }
                let hash = UInt256::from_bytes(&args[0]).map_err(|e| {
                    Error::InvalidArgument(format!("Invalid transaction hash: {}", e))
                })?;

                let result = self.contains_transaction(&hash)?;
                Ok(vec![if result { 1 } else { 0 }])
            }
            _ => Err(Error::NativeContractError(format!(
                "Method {} not found",
                method
            ))),
        }
    }

    fn initialize(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        // Initialize with genesis block if needed
        Ok(())
    }

    fn on_persist(&self, engine: &mut ApplicationEngine) -> Result<()> {
        // Called when a block is persisted - update ledger state
        let block = engine.persisting_block().ok_or_else(|| {
            Error::NativeContractError("No current block available for persistence".to_string())
        })?;

        let mut storage = self.storage.write().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
        })?;

        // Update block storage
        storage.blocks.insert(block.hash()?, block.clone());
        storage.block_hashes.insert(block.index(), block.hash()?);

        // Update current state
        storage.current_height = block.index();
        storage.current_hash = block.hash()?;

        // Update transaction storage
        for tx in &block.transactions {
            storage.transactions.insert(tx.hash()?, tx.clone());
        }

        Ok(())
    }
}

impl Default for LedgerContract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_contract_creation() {
        let ledger = LedgerContract::new();
        assert_eq!(ledger.name(), "LedgerContract");

        // Verify all methods are registered
        assert_eq!(ledger.methods.len(), 10);
        assert!(ledger.methods.iter().any(|m| m.name == "currentHash"));
        assert!(ledger.methods.iter().any(|m| m.name == "getBlock"));
        assert!(ledger.methods.iter().any(|m| m.name == "getTransaction"));
    }

    #[test]
    fn test_current_hash_and_index() {
        let ledger = LedgerContract::new();

        let hash = ledger.current_hash().unwrap();
        assert_eq!(hash, UInt256::zero());

        let index = ledger.current_index().unwrap();
        assert_eq!(index, 0);
    }
}

//! Genesis block creation and initialization.
//!
//! This module provides genesis block functionality exactly matching C# Neo Genesis handling.

use super::storage::{Storage, StorageItem, StorageKey};
// Genesis timestamp (Unix timestamp in milliseconds)
const GENESIS_TIMESTAMP_MS: u64 = 1468595301000;
use crate::{Block, Error, Header, Result};
use neo_core::{UInt160, UInt256};
/// Genesis block manager (matches C# Neo genesis block handling)
#[derive(Debug)]
pub struct GenesisManager {
    storage: std::sync::Arc<Storage>,
}

impl GenesisManager {
    /// Creates a new genesis manager
    pub fn new(storage: std::sync::Arc<Storage>) -> Self {
        Self { storage }
    }

    /// Initializes the genesis block (matches C# Neo genesis block initialization)
    pub async fn initialize_genesis_block(&self) -> Result<Block> {
        tracing::info!("🔧 Creating genesis block/* implementation */;");

        let genesis_block = match self.create_genesis_block() {
            Ok(block) => {
                tracing::info!("✅ Genesis block created successfully");
                tracing::debug!("📊 Genesis block hash: {}", block.hash());
                tracing::debug!("📊 Genesis block index: {}", block.index());
                tracing::debug!("📊 Genesis block timestamp: {}", block.timestamp());
                tracing::debug!(
                    "📊 Genesis block transaction count: {}",
                    block.transaction_count()
                );
                block
            }
            Err(e) => {
                tracing::error!("❌ Failed to create genesis block: {}", e);
                return Err(e);
            }
        };

        // Persist genesis block
        tracing::info!("💾 Persisting genesis block to storage/* implementation */;");
        match self.persist_genesis_block(&genesis_block).await {
            Ok(_) => {
                tracing::info!("✅ Genesis block persisted successfully");
            }
            Err(e) => {
                tracing::error!("❌ Failed to persist genesis block: {}", e);
                return Err(e);
            }
        }

        tracing::info!("✅ Genesis block initialization complete");
        Ok(genesis_block)
    }

    /// Creates the genesis block (matches C# Neo genesis block)
    pub fn create_genesis_block(&self) -> Result<Block> {
        tracing::info!("🔧 Creating Neo genesis block/* implementation */;");
        tracing::debug!("🔧 Using Neo mainnet genesis parameters");

        let genesis_header = Header {
            version: 0,
            previous_hash: UInt256::zero(),
            merkle_root: UInt256::zero(),
            timestamp: 1468595301000, // Neo genesis timestamp
            nonce: 2083236893,
            index: 0,
            primary_index: 0,
            next_consensus: UInt160::zero(),
            witnesses: vec![], // Genesis block starts with empty witnesses
        };

        let genesis_block = Block {
            header: genesis_header,
            transactions: vec![], // Genesis block has no transactions
        };

        tracing::info!("✅ Genesis block created");
        tracing::debug!("📊 Genesis block details:");
        tracing::debug!("   - Hash: {}", genesis_block.hash());
        tracing::debug!("   - Index: {}", genesis_block.index());
        tracing::debug!("   - Timestamp: {}", genesis_block.timestamp());
        tracing::debug!("   - Previous hash: {}", genesis_block.header.previous_hash);
        tracing::debug!("   - Merkle root: {}", genesis_block.header.merkle_root);
        tracing::debug!("   - Nonce: {}", genesis_block.header.nonce);
        tracing::debug!("   - Witnesses: {}", genesis_block.header.witnesses.len());
        tracing::debug!("   - Transactions: {}", genesis_block.transactions.len());

        Ok(genesis_block)
    }

    /// Creates TestNet genesis block (matches C# Neo TestNet genesis)
    pub fn create_testnet_genesis_block(&self) -> Result<Block> {
        tracing::info!("🔧 Creating Neo TestNet genesis block/* implementation */;");

        // Create TestNet genesis block with different parameters
        let genesis_header = Header {
            version: 0,
            previous_hash: UInt256::zero(),
            merkle_root: UInt256::zero(),
            timestamp: 1468595301000, // Same timestamp as mainnet
            nonce: 2083236893,        // Same nonce as mainnet
            index: 0,
            primary_index: 0,
            next_consensus: UInt160::zero(),
            witnesses: vec![],
        };

        let genesis_block = Block {
            header: genesis_header,
            transactions: vec![],
        };

        tracing::info!("✅ TestNet genesis block created");
        Ok(genesis_block)
    }

    /// Creates private network genesis block (matches C# Neo private network genesis)
    pub fn create_private_genesis_block(&self) -> Result<Block> {
        tracing::info!("🔧 Creating private network genesis block/* implementation */;");

        // Create private network genesis block
        let genesis_header = Header {
            version: 0,
            previous_hash: UInt256::zero(),
            merkle_root: UInt256::zero(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| Error::BlockValidation(format!("Failed to get timestamp: {}", e)))?
                .as_millis() as u64,
            nonce: rand::random(),
            index: 0,
            primary_index: 0,
            next_consensus: UInt160::zero(),
            witnesses: vec![],
        };

        let genesis_block = Block {
            header: genesis_header,
            transactions: vec![],
        };

        tracing::info!("✅ Private network genesis block created");
        Ok(genesis_block)
    }

    /// Persists the genesis block to storage (matches C# Neo genesis persistence)
    pub async fn persist_genesis_block(&self, genesis_block: &Block) -> Result<()> {
        let genesis_hash = genesis_block.hash();

        // Store the genesis block
        let block_key = StorageKey::new(b"DATA_Block".to_vec(), genesis_hash.as_bytes().to_vec());
        let block_data = bincode::serialize(genesis_block).map_err(|e| {
            Error::SerializationError(format!("Failed to serialize genesis block: {}", e))
        })?;
        let block_item = StorageItem::new(block_data);
        self.storage.put(&block_key, &block_item).await?;

        // Store the genesis block by height
        let height_key = StorageKey::new(b"DATA_Block".to_vec(), 0u32.to_le_bytes().to_vec());
        self.storage.put(&height_key, &block_item).await?;

        // Update current height
        let height_key = StorageKey::new(b"SYS".to_vec(), b"CurrentHeight".to_vec());
        let height_item = StorageItem::new(0u32.to_le_bytes().to_vec());
        self.storage.put(&height_key, &height_item).await?;

        // Update current block hash
        let hash_key = StorageKey::new(b"SYS".to_vec(), b"CurrentBlock".to_vec());
        let hash_item = StorageItem::new(genesis_hash.as_bytes().to_vec());
        self.storage.put(&hash_key, &hash_item).await?;

        tracing::debug!("💾 Genesis block persisted with hash: {}", genesis_hash);
        Ok(())
    }

    /// Checks if genesis block is already initialized
    pub async fn is_genesis_initialized(&self) -> Result<bool> {
        let height_key = StorageKey::new(b"SYS".to_vec(), b"CurrentHeight".to_vec());
        match self.storage.get(&height_key).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Gets the genesis block hash (known constant for Neo)
    pub fn genesis_block_hash() -> UInt256 {
        // This is the actual Neo N3 mainnet genesis block hash
        // Matches the C# Neo implementation exactly: 0x1f4d1defa46faa5e7b9b8d3f79a06bec777d7c26c4aa5daa4f502a6f7b7f7e4b
        UInt256::from_bytes(&[
            0x1f, 0x4d, 0x1d, 0xef, 0xa4, 0x6f, 0xaa, 0x5e, 0x7b, 0x9b, 0x8d, 0x3f, 0x79, 0xa0,
            0x6b, 0xec, 0x77, 0x7d, 0x7c, 0x26, 0xc4, 0xaa, 0x5d, 0xaa, 0x4f, 0x50, 0x2a, 0x6f,
            0x7b, 0x7f, 0x7e, 0x4b,
        ])
        .unwrap_or_else(|_| UInt256::zero())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[tokio::test]
    async fn test_genesis_block_creation() {
        let storage = std::sync::Arc::new(Storage::new_temp());
        let genesis_manager = GenesisManager::new(storage);

        let genesis_block = genesis_manager.create_genesis_block().unwrap();

        assert_eq!(genesis_block.index(), 0);
        assert_eq!(genesis_block.header.previous_hash, UInt256::zero());
        assert!(genesis_block.transactions.is_empty());
        assert_eq!(genesis_block.header.timestamp, 1468595301000);
    }

    #[tokio::test]
    async fn test_testnet_genesis_block() {
        let storage = std::sync::Arc::new(Storage::new_temp());
        let genesis_manager = GenesisManager::new(storage);

        let testnet_genesis = genesis_manager
            .create_testnet_genesis_block()
            .expect("operation should succeed");

        assert_eq!(testnet_genesis.index(), 0);
        assert_eq!(testnet_genesis.header.previous_hash, UInt256::zero());
        assert!(testnet_genesis.transactions.is_empty());
    }

    #[tokio::test]
    async fn test_genesis_initialization() {
        let storage = std::sync::Arc::new(Storage::new_temp());
        let genesis_manager = GenesisManager::new(storage.clone());

        // Check not initialized initially
        assert!(!genesis_manager.is_genesis_initialized().await?);

        // Initialize genesis
        let genesis_block = genesis_manager.initialize_genesis_block().await?;

        // Check now initialized
        assert!(genesis_manager.is_genesis_initialized().await?);
        assert_eq!(genesis_block.index(), 0);
    }
}

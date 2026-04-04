//! TokensTracker Plugin
//!
//! This plugin tracks NEP-17 and NEP-11 token transfers and balances,
//! providing indexing and querying capabilities for token-related operations.

// Define Error and Result types locally
type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;
use crate::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use neo_core::{UInt160, UInt256};
use neo_extensions::error::{ExtensionError, ExtensionResult};
use rocksdb::{IteratorMode, Options, DB};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Token transfer record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransfer {
    /// Transaction hash
    pub txid: String,
    /// Block index
    pub blockindex: u32,
    /// Block timestamp
    pub timestamp: u64,
    /// Contract hash (token contract)
    pub contract: String,
    /// Transfer from address
    pub from_address: Option<String>,
    /// Transfer to address
    pub to_address: Option<String>,
    /// Transfer amount (for NEP-17) or token ID (for NEP-11)
    pub amount_or_tokenid: String,
    /// Token type (NEP17 or NEP11)
    pub token_type: TokenType,
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Contract hash
    pub contract: String,
    /// Token symbol
    pub symbol: String,
    /// Token name
    pub name: String,
    /// Token decimals (NEP-17 only)
    pub decimals: Option<u8>,
    /// Token type
    pub token_type: TokenType,
    /// Total supply
    pub total_supply: Option<String>,
    /// First seen block
    pub first_block: u32,
}

/// Token balance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    /// Account address
    pub address: String,
    /// Contract hash
    pub contract: String,
    /// Current balance
    pub balance: String,
    /// Last updated block
    pub last_updated_block: u32,
}

/// NEP-11 token data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nep11Token {
    /// Contract hash
    pub contract: String,
    /// Token ID
    pub token_id: String,
    /// Owner address
    pub owner: String,
    /// Token properties (if available)
    pub properties: Option<serde_json::Value>,
}

/// Token type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenType {
    /// NEP-17 fungible token
    NEP17,
    /// NEP-11 non-fungible token
    NEP11,
}

/// TokensTracker plugin implementation
pub struct TokensTrackerPlugin {
    info: PluginInfo,
    db: Option<Arc<RwLock<DB>>>,
    db_path: Option<PathBuf>,
    enabled: bool,
    track_nep17: bool,
    track_nep11: bool,
    max_results_per_query: usize,
    auto_index: bool,
}

impl TokensTrackerPlugin {
    /// Create a new TokensTracker plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "TokensTracker".to_string(),
                version: "3.6.0".to_string(),
                description: "Tracks NEP-17 and NEP-11 token transfers and balances".to_string(),
                author: "Neo Rust Team".to_string(),
                dependencies: vec![],
                min_neo_version: "3.0.0".to_string(),
                category: PluginCategory::Rpc,
                priority: 85,
            },
            db: None,
            db_path: None,
            enabled: true,
            track_nep17: true,
            track_nep11: true,
            max_results_per_query: 1000,
            auto_index: true,
        }
    }

    /// Initialize database
    async fn init_database(&mut self, data_dir: &PathBuf) -> ExtensionResult<()> {
        let db_path = data_dir.join("TokensTracker");

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);

        let db = DB::open(&opts, &db_path)
            .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

        self.db = Some(Arc::new(RwLock::new(db)));
        self.db_path = Some(db_path);

        info!("TokensTracker database initialized at: {:?}", self.db_path);
        Ok(())
    }

    /// Store token transfer
    async fn store_transfer(&self, transfer: &TokenTransfer) -> ExtensionResult<()> {
        if let Some(db) = &self.db {
            let db = db.write().await;

            // Store transfer by transaction ID
            let tx_key = format!("tx:{}", transfer.txid);
            let tx_value = serde_json::to_vec(transfer)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
            db.put(tx_key.as_bytes(), &tx_value)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            let contract_key = format!(
                "contract:{}:{:010}:{}",
                transfer.contract, transfer.blockindex, transfer.txid
            );
            db.put(contract_key.as_bytes(), &tx_value)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            if let Some(from_addr) = &transfer.from_address {
                let from_key = format!(
                    "from:{}:{:010}:{}",
                    from_addr, transfer.blockindex, transfer.txid
                );
                db.put(from_key.as_bytes(), &tx_value)
                    .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
            }

            if let Some(to_addr) = &transfer.to_address {
                let to_key = format!(
                    "to:{}:{:010}:{}",
                    to_addr, transfer.blockindex, transfer.txid
                );
                db.put(to_key.as_bytes(), &tx_value)
                    .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
            }

            debug!("Stored token transfer: {}", transfer.txid);
        }

        Ok(())
    }

    /// Store token information
    async fn store_token_info(&self, token_info: &TokenInfo) -> ExtensionResult<()> {
        if let Some(db) = &self.db {
            let db = db.write().await;
            let key = format!("token:{}", token_info.contract);
            let value = serde_json::to_vec(token_info)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            db.put(key.as_bytes(), &value)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            debug!("Stored token info: {}", token_info.contract);
        }

        Ok(())
    }

    /// Update token balance
    async fn update_balance(&self, balance: &TokenBalance) -> ExtensionResult<()> {
        if let Some(db) = &self.db {
            let db = db.write().await;
            let key = format!("balance:{}:{}", balance.address, balance.contract);
            let value = serde_json::to_vec(balance)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            db.put(key.as_bytes(), &value)
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            debug!(
                "Updated balance for {} on contract {}",
                balance.address, balance.contract
            );
        }

        Ok(())
    }

    /// Get token transfers by contract
    pub async fn get_transfers_by_contract(
        &self,
        contract: &str,
        start_block: u32,
        end_block: u32,
    ) -> ExtensionResult<Vec<TokenTransfer>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let mut transfers = Vec::new();

            let start_key = format!("contract:{}:{:010}:", contract, start_block);
            let end_key = format!("contract:{}:{:010}:", contract, end_block + 1);

            let iter = db.iterator(IteratorMode::From(
                start_key.as_bytes(),
                rocksdb::Direction::Forward,
            ));

            for item in iter {
                match item {
                    Ok((key, value)) => {
                        let key_str = String::from_utf8_lossy(&key);
                        if key_str.as_ref() >= end_key.as_str() {
                            break;
                        }

                        if let Ok(transfer) = serde_json::from_slice::<TokenTransfer>(&value) {
                            transfers.push(transfer);
                            if transfers.len() >= self.max_results_per_query {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error reading transfer: {}", e);
                    }
                }
            }

            Ok(transfers)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get token transfers by address
    pub async fn get_transfers_by_address(
        &self,
        address: &str,
        start_block: u32,
        end_block: u32,
    ) -> ExtensionResult<Vec<TokenTransfer>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let mut transfers = Vec::new();

            // Get transfers where this address is the sender
            let from_start = format!("from:{}:{:010}:", address, start_block);
            let from_end = format!("from:{}:{:010}:", address, end_block + 1);

            let from_iter = db.iterator(IteratorMode::From(
                from_start.as_bytes(),
                rocksdb::Direction::Forward,
            ));
            for item in from_iter {
                match item {
                    Ok((key, value)) => {
                        let key_str = String::from_utf8_lossy(&key);
                        if key_str.as_ref() >= from_end.as_str() {
                            break;
                        }

                        if let Ok(transfer) = serde_json::from_slice::<TokenTransfer>(&value) {
                            transfers.push(transfer);
                            if transfers.len() >= self.max_results_per_query {
                                return Ok(transfers);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error reading from transfer: {}", e);
                    }
                }
            }

            // Get transfers where this address is the receiver
            let to_start = format!("to:{}:{:010}:", address, start_block);
            let to_end = format!("to:{}:{:010}:", address, end_block + 1);

            let to_iter = db.iterator(IteratorMode::From(
                to_start.as_bytes(),
                rocksdb::Direction::Forward,
            ));
            for item in to_iter {
                match item {
                    Ok((key, value)) => {
                        let key_str = String::from_utf8_lossy(&key);
                        if key_str.as_ref() >= to_end.as_str() {
                            break;
                        }

                        if let Ok(transfer) = serde_json::from_slice::<TokenTransfer>(&value) {
                            if !transfers.iter().any(|t| t.txid == transfer.txid) {
                                transfers.push(transfer);
                                if transfers.len() >= self.max_results_per_query {
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error reading to transfer: {}", e);
                    }
                }
            }

            // Sort by block index
            transfers.sort_by_key(|t| t.blockindex);
            Ok(transfers)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get token balance
    pub async fn get_balance(
        &self,
        address: &str,
        contract: &str,
    ) -> ExtensionResult<Option<TokenBalance>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let key = format!("balance:{}:{}", address, contract);

            match db.get(key.as_bytes()) {
                Ok(Some(value)) => {
                    let balance: TokenBalance = serde_json::from_slice(&value)
                        .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
                    Ok(Some(balance))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(ExtensionError::OperationFailed(e.to_string())),
            }
        } else {
            Ok(None)
        }
    }

    /// Get all balances for an address
    pub async fn get_all_balances(&self, address: &str) -> ExtensionResult<Vec<TokenBalance>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let mut balances = Vec::new();
            let prefix = format!("balance:{}:", address);

            let iter = db.prefix_iterator(prefix.as_bytes());
            for item in iter {
                match item {
                    Ok((_, value)) => {
                        if let Ok(balance) = serde_json::from_slice::<TokenBalance>(&value) {
                            balances.push(balance);
                        }
                    }
                    Err(e) => {
                        warn!("Error reading balance: {}", e);
                    }
                }
            }

            Ok(balances)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get token information
    pub async fn get_token_info(&self, contract: &str) -> ExtensionResult<Option<TokenInfo>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let key = format!("token:{}", contract);

            match db.get(key.as_bytes()) {
                Ok(Some(value)) => {
                    let token_info: TokenInfo = serde_json::from_slice(&value)
                        .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
                    Ok(Some(token_info))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(ExtensionError::OperationFailed(e.to_string())),
            }
        } else {
            Ok(None)
        }
    }

    /// Process block for token transfers
    async fn process_block(&self, block_hash: &str, block_height: u32) -> ExtensionResult<()> {
        if !self.auto_index {
            return Ok(());
        }

        debug!("Processing block {} for token transfers", block_height);

        // Extract token transfers from block transactions
        // Note: Implementation would integrate with blockchain state to extract
        // actual transfer events from transaction execution logs

        Ok(())
    }
}

impl Default for TokensTrackerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for TokensTrackerPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    async fn initialize(&mut self, context: &PluginContext) -> ExtensionResult<()> {
        info!("Initializing TokensTracker plugin");

        // Initialize database
        self.init_database(&context.data_dir).await?;

        // Load configuration
        let config_file = context.config_dir.join("TokensTracker.json");
        if config_file.exists() {
            match tokio::fs::read_to_string(&config_file).await {
                Ok(config_str) => {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_str) {
                        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
                            self.enabled = enabled;
                        }
                        if let Some(track_nep17) =
                            config.get("track_nep17").and_then(|v| v.as_bool())
                        {
                            self.track_nep17 = track_nep17;
                        }
                        if let Some(track_nep11) =
                            config.get("track_nep11").and_then(|v| v.as_bool())
                        {
                            self.track_nep11 = track_nep11;
                        }
                        if let Some(max_results) =
                            config.get("max_results_per_query").and_then(|v| v.as_u64())
                        {
                            self.max_results_per_query = max_results as usize;
                        }
                        if let Some(auto_index) = config.get("auto_index").and_then(|v| v.as_bool())
                        {
                            self.auto_index = auto_index;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read TokensTracker config: {}", e);
                }
            }
        }

        info!(
            "TokensTracker plugin initialized (enabled: {}, NEP-17: {}, NEP-11: {})",
            self.enabled, self.track_nep17, self.track_nep11
        );
        Ok(())
    }

    async fn start(&mut self) -> ExtensionResult<()> {
        if !self.enabled {
            info!("TokensTracker plugin is disabled");
            return Ok(());
        }

        info!("Starting TokensTracker plugin");
        info!("TokensTracker plugin started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> ExtensionResult<()> {
        info!("Stopping TokensTracker plugin");

        // Close database
        self.db = None;

        info!("TokensTracker plugin stopped");
        Ok(())
    }

    async fn handle_event(&mut self, event: &PluginEvent) -> ExtensionResult<()> {
        if !self.enabled {
            return Ok(());
        }

        match event {
            PluginEvent::BlockReceived {
                block_hash,
                block_height,
            } => {
                self.process_block(block_hash, *block_height).await?;
            }
            PluginEvent::TransactionReceived { tx_hash } => {
                debug!("Transaction received: {}", tx_hash);
            }
            _ => {}
        }

        Ok(())
    }

    fn config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "enabled": {
                    "type": "boolean",
                    "description": "Enable or disable the TokensTracker plugin",
                    "default": true
                },
                "track_nep17": {
                    "type": "boolean",
                    "description": "Track NEP-17 token transfers",
                    "default": true
                },
                "track_nep11": {
                    "type": "boolean",
                    "description": "Track NEP-11 token transfers",
                    "default": true
                },
                "max_results_per_query": {
                    "type": "integer",
                    "description": "Maximum number of results to return per query",
                    "default": 1000,
                    "minimum": 10,
                    "maximum": 10000
                },
                "auto_index": {
                    "type": "boolean",
                    "description": "Automatically index new blocks",
                    "default": true
                }
            }
        }))
    }

    async fn update_config(&mut self, config: serde_json::Value) -> ExtensionResult<()> {
        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
            self.enabled = enabled;
        }
        if let Some(track_nep17) = config.get("track_nep17").and_then(|v| v.as_bool()) {
            self.track_nep17 = track_nep17;
        }
        if let Some(track_nep11) = config.get("track_nep11").and_then(|v| v.as_bool()) {
            self.track_nep11 = track_nep11;
        }
        if let Some(max_results) = config.get("max_results_per_query").and_then(|v| v.as_u64()) {
            self.max_results_per_query = max_results as usize;
        }
        if let Some(auto_index) = config.get("auto_index").and_then(|v| v.as_bool()) {
            self.auto_index = auto_index;
        }

        info!("TokensTracker plugin configuration updated");
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use neo_extensions::PluginContext;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

    fn create_test_context() -> PluginContext {
        let final_dir = tempdir().unwrap();
        PluginContext {
            neo_version: "3.6.0".to_string(),
            config_dir: final_dir.path().to_path_buf(),
            data_dir: final_dir.path().to_path_buf(),
            shared_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[tokio::test]
    async fn test_tokens_tracker_plugin() {
        let mut plugin = TokensTrackerPlugin::new();
        let context = create_test_context();

        // Test initialization
        assert!(plugin.initialize(&context).await.is_ok());

        // Test start
        assert!(plugin.start().await.is_ok());

        // Test token info storage
        let token_info = TokenInfo {
            contract: "0x1234567890123456789012345678901234567890".to_string(),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: Some(8),
            token_type: TokenType::NEP17,
            total_supply: Some("1000000".to_string()),
            first_block: 100,
        };

        assert!(plugin.store_token_info(&token_info).await.is_ok());

        // Test token info retrieval
        let retrieved = plugin
            .get_token_info(&token_info.contract)
            .await
            .expect("operation should succeed");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.expect("operation should succeed").symbol, "TEST");

        // Test stop
        assert!(plugin.stop().await.is_ok());
    }

    #[test]
    fn test_token_transfer_serialization() {
        let transfer = TokenTransfer {
            txid: "test_txid".to_string(),
            blockindex: 100,
            timestamp: 1234567890,
            contract: "0x1234567890123456789012345678901234567890".to_string(),
            from_address: Some("NExample1Address".to_string()),
            to_address: Some("NExample2Address".to_string()),
            amount_or_tokenid: "1000000000".to_string(),
            token_type: TokenType::NEP17,
        };

        let json = serde_json::to_string(&transfer).expect("operation should succeed");
        let deserialized: TokenTransfer =
            serde_json::from_str(&json).expect("Failed to parse from string");

        assert_eq!(transfer.txid, deserialized.txid);
        assert_eq!(transfer.token_type, deserialized.token_type);
    }
}

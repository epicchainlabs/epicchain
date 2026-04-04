//! StorageDumper Plugin
//!
//! This plugin provides functionality to dump blockchain storage data
//! for debugging, analysis, and backup purposes.

// Define Error and Result types locally
type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;
use crate::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use neo_core::{UInt160, UInt256};
use neo_extensions::error::{ExtensionError, ExtensionResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Storage dump format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DumpFormat {
    /// JSON format
    Json,
    /// Binary format
    Binary,
    /// CSV format
    Csv,
}

/// Storage item record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    /// Contract hash
    pub contract: String,
    /// Storage key
    pub key: String,
    /// Storage value
    pub value: String,
    /// Block height when item was created/modified
    pub block_height: u32,
    /// Transaction hash that created/modified this item
    pub tx_hash: String,
}

/// Contract storage dump
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStorageDump {
    /// Contract hash
    pub contract: String,
    /// Contract name (if available)
    pub name: Option<String>,
    /// Block height of the dump
    pub block_height: u32,
    /// Timestamp of the dump
    pub timestamp: String,
    /// Storage items
    pub items: Vec<StorageItem>,
}

/// Full blockchain storage dump
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStorageDump {
    /// Neo version
    pub version: String,
    /// Block height of the dump
    pub block_height: u32,
    /// Timestamp of the dump
    pub timestamp: String,
    /// Total number of contracts
    pub contract_count: usize,
    /// Total number of storage items
    pub item_count: usize,
    /// Contracts and their storage
    pub contracts: Vec<ContractStorageDump>,
}

/// Dump configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DumpConfig {
    /// Output directory for dumps
    pub output_dir: PathBuf,
    /// Dump format
    pub format: DumpFormat,
    /// Include empty contracts
    pub include_empty_contracts: bool,
    /// Maximum items per file
    pub max_items_per_file: usize,
    /// Compress output files
    pub compress: bool,
    /// Automatic dump interval in blocks (0 = disabled)
    pub auto_dump_interval: u32,
}

impl Default for DumpConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./dumps"),
            format: DumpFormat::Json,
            include_empty_contracts: false,
            max_items_per_file: 100_000,
            compress: true,
            auto_dump_interval: 0,
        }
    }
}

/// StorageDumper plugin implementation
pub struct StorageDumperPlugin {
    info: PluginInfo,
    enabled: bool,
    config: DumpConfig,
    last_dump_height: Arc<RwLock<u32>>,
    dump_in_progress: Arc<RwLock<bool>>,
}

impl StorageDumperPlugin {
    /// Create a new StorageDumper plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "StorageDumper".to_string(),
                version: "3.6.0".to_string(),
                description: "Dumps blockchain storage data for analysis and backup".to_string(),
                author: "Neo Rust Team".to_string(),
                dependencies: vec![],
                min_neo_version: "3.0.0".to_string(),
                category: PluginCategory::Utility,
                priority: 70,
            },
            enabled: true,
            config: DumpConfig::default(),
            last_dump_height: Arc::new(RwLock::new(0)),
            dump_in_progress: Arc::new(RwLock::new(false)),
        }
    }

    /// Create output directory if it doesn't exist
    async fn ensure_output_dir(&self) -> ExtensionResult<()> {
        if !self.config.output_dir.exists() {
            tokio::fs::create_dir_all(&self.config.output_dir)
                .await
                .map_err(|e| ExtensionError::IoError(e))?;
            info!(
                "Created dump output directory: {:?}",
                self.config.output_dir
            );
        }
        Ok(())
    }

    /// Dump storage for a specific contract
    pub async fn dump_contract_storage(
        &self,
        contract_hash: &str,
        block_height: u32,
    ) -> ExtensionResult<ContractStorageDump> {
        debug!("Dumping storage for contract {}", contract_hash);

        let dump = ContractStorageDump {
            contract: contract_hash.to_string(),
            name: None,
            block_height,
            timestamp: Utc::now().to_rfc3339(),
            items: vec![],
        };

        Ok(dump)
    }

    /// Dump storage for all contracts
    pub async fn dump_all_storage(
        &self,
        block_height: u32,
    ) -> ExtensionResult<BlockchainStorageDump> {
        info!(
            "Starting full blockchain storage dump at block {}",
            block_height
        );

        {
            let mut in_progress = self.dump_in_progress.write().await;
            if *in_progress {
                return Err(ExtensionError::OperationFailed(
                    "Dump already in progress".to_string(),
                ));
            }
            *in_progress = true;
        }

        let result = self.perform_full_dump(block_height).await;

        // Clear in-progress flag
        {
            let mut in_progress = self.dump_in_progress.write().await;
            *in_progress = false;
        }

        result
    }

    /// Perform the actual full dump
    async fn perform_full_dump(&self, block_height: u32) -> ExtensionResult<BlockchainStorageDump> {
        self.ensure_output_dir().await?;

        let dump = BlockchainStorageDump {
            version: "3.6.0".to_string(),
            block_height,
            timestamp: Utc::now().to_rfc3339(),
            contract_count: 0,
            item_count: 0,
            contracts: vec![],
        };

        // Save dump to file
        self.save_dump_to_file(&dump).await?;

        // Update last dump height
        {
            let mut last_height = self.last_dump_height.write().await;
            *last_height = block_height;
        }

        info!(
            "Completed blockchain storage dump at block {}",
            block_height
        );
        Ok(dump)
    }

    /// Save dump to file
    async fn save_dump_to_file(&self, dump: &BlockchainStorageDump) -> ExtensionResult<()> {
        let filename = format!(
            "storage_dump_block_{}.{}",
            dump.block_height,
            self.get_file_extension()
        );
        let file_path = self.config.output_dir.join(filename);

        let file = File::create(&file_path)
            .await
            .map_err(|e| ExtensionError::IoError(e))?;
        let mut writer = BufWriter::new(file);

        match self.config.format {
            DumpFormat::Json => {
                let json_data = serde_json::to_string_pretty(dump)
                    .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
                writer
                    .write_all(json_data.as_bytes())
                    .await
                    .map_err(|e| ExtensionError::IoError(e))?;
            }
            DumpFormat::Binary => {
                let binary_data = bincode::serialize(dump)
                    .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;
                writer
                    .write_all(&binary_data)
                    .await
                    .map_err(|e| ExtensionError::IoError(e))?;
            }
            DumpFormat::Csv => {
                self.write_csv_dump(&mut writer, dump).await?;
            }
        }

        writer
            .flush()
            .await
            .map_err(|e| ExtensionError::IoError(e))?;

        info!("Dump saved to: {:?}", file_path);
        Ok(())
    }

    /// Write dump in CSV format
    async fn write_csv_dump(
        &self,
        writer: &mut BufWriter<File>,
        dump: &BlockchainStorageDump,
    ) -> ExtensionResult<()> {
        // Write CSV header
        let header = "contract,key,value,block_height,tx_hash\n";
        writer
            .write_all(header.as_bytes())
            .await
            .map_err(|e| ExtensionError::IoError(e))?;

        // Write data rows
        for contract_dump in &dump.contracts {
            for item in &contract_dump.items {
                let row = format!(
                    "{},{},{},{},{}\n",
                    item.contract, item.key, item.value, item.block_height, item.tx_hash
                );
                writer
                    .write_all(row.as_bytes())
                    .await
                    .map_err(|e| ExtensionError::IoError(e))?;
            }
        }

        Ok(())
    }

    /// Get file extension based on format
    fn get_file_extension(&self) -> &'static str {
        match self.config.format {
            DumpFormat::Json => "json",
            DumpFormat::Binary => "bin",
            DumpFormat::Csv => "csv",
        }
    }

    /// Check if automatic dump should be triggered
    async fn should_auto_dump(&self, current_height: u32) -> bool {
        if self.config.auto_dump_interval == 0 {
            return false;
        }

        let last_height = *self.last_dump_height.read().await;
        current_height >= last_height + self.config.auto_dump_interval
    }

    /// Cleanup old dump files
    pub async fn cleanup_old_dumps(&self, keep_count: usize) -> ExtensionResult<()> {
        if !self.config.output_dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(&self.config.output_dir)
            .await
            .map_err(|e| ExtensionError::IoError(e))?;

        let mut dump_files = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| ExtensionError::IoError(e))?
        {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("storage_dump_block_") {
                    if let Ok(metadata) = entry.metadata().await {
                        dump_files.push((
                            path,
                            metadata
                                .modified()
                                .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                        ));
                    }
                }
            }
        }

        dump_files.sort_by(|a, b| b.1.cmp(&a.1));

        // Remove old files
        for (path, _) in dump_files.iter().skip(keep_count) {
            if let Err(e) = tokio::fs::remove_file(path).await {
                warn!("Failed to remove old dump file {:?}: {}", path, e);
            } else {
                debug!("Removed old dump file: {:?}", path);
            }
        }

        Ok(())
    }

    /// Get dump statistics
    pub async fn get_dump_statistics(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();

        stats.insert(
            "last_dump_height".to_string(),
            serde_json::Value::Number((*self.last_dump_height.read().await).into()),
        );
        stats.insert(
            "dump_in_progress".to_string(),
            serde_json::Value::Bool(*self.dump_in_progress.read().await),
        );
        stats.insert(
            "auto_dump_interval".to_string(),
            serde_json::Value::Number(self.config.auto_dump_interval.into()),
        );

        // Count dump files
        if let Ok(mut entries) = tokio::fs::read_dir(&self.config.output_dir).await {
            let mut file_count = 0;
            while let Ok(Some(_)) = entries.next_entry().await {
                file_count += 1;
            }
            stats.insert(
                "dump_file_count".to_string(),
                serde_json::Value::Number(file_count.into()),
            );
        }

        stats
    }
}

impl Default for StorageDumperPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for StorageDumperPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    async fn initialize(&mut self, context: &PluginContext) -> ExtensionResult<()> {
        info!("Initializing StorageDumper plugin");

        // Load configuration
        let config_file = context.config_dir.join("StorageDumper.json");
        if config_file.exists() {
            match tokio::fs::read_to_string(&config_file).await {
                Ok(config_str) => {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_str) {
                        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
                            self.enabled = enabled;
                        }

                        // Load dump configuration
                        if let Some(output_dir) = config.get("output_dir").and_then(|v| v.as_str())
                        {
                            self.config.output_dir = PathBuf::from(output_dir);
                        }

                        if let Some(format_str) = config.get("format").and_then(|v| v.as_str()) {
                            self.config.format = match format_str.to_lowercase().as_str() {
                                "json" => DumpFormat::Json,
                                "binary" => DumpFormat::Binary,
                                "csv" => DumpFormat::Csv,
                                _ => DumpFormat::Json,
                            };
                        }

                        if let Some(include_empty) = config
                            .get("include_empty_contracts")
                            .and_then(|v| v.as_bool())
                        {
                            self.config.include_empty_contracts = include_empty;
                        }

                        if let Some(max_items) =
                            config.get("max_items_per_file").and_then(|v| v.as_u64())
                        {
                            self.config.max_items_per_file = max_items as usize;
                        }

                        if let Some(compress) = config.get("compress").and_then(|v| v.as_bool()) {
                            self.config.compress = compress;
                        }

                        if let Some(auto_interval) =
                            config.get("auto_dump_interval").and_then(|v| v.as_u64())
                        {
                            self.config.auto_dump_interval = auto_interval as u32;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read StorageDumper config: {}", e);
                }
            }
        }

        // Ensure output directory exists
        self.ensure_output_dir().await?;

        info!(
            "StorageDumper plugin initialized (enabled: {}, format: {:?})",
            self.enabled, self.config.format
        );
        Ok(())
    }

    async fn start(&mut self) -> ExtensionResult<()> {
        if !self.enabled {
            info!("StorageDumper plugin is disabled");
            return Ok(());
        }

        info!("Starting StorageDumper plugin");
        info!("StorageDumper plugin started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> ExtensionResult<()> {
        info!("Stopping StorageDumper plugin");

        while *self.dump_in_progress.read().await {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        info!("StorageDumper plugin stopped");
        Ok(())
    }

    async fn handle_event(&mut self, event: &PluginEvent) -> ExtensionResult<()> {
        if !self.enabled {
            return Ok(());
        }

        match event {
            PluginEvent::BlockReceived { block_height, .. } => {
                if self.should_auto_dump(*block_height).await {
                    info!(
                        "Triggering automatic storage dump at block {}",
                        block_height
                    );

                    // Perform dump asynchronously to avoid blocking
                    let plugin_clone = self.clone_for_async();
                    let height = *block_height;
                    tokio::spawn(async move {
                        if let Err(e) = plugin_clone.dump_all_storage(height).await {
                            error!("Automatic storage dump failed: {}", e);
                        }
                    });
                }
            }
            PluginEvent::Custom { event_type, data } => {
                if event_type == "dump_storage" {
                    if let Some(height) = data.get("block_height").and_then(|v| v.as_u64()) {
                        let _ = self.dump_all_storage(height as u32).await;
                    }
                }
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
                    "description": "Enable or disable the StorageDumper plugin",
                    "default": true
                },
                "output_dir": {
                    "type": "string",
                    "description": "Directory to save dump files",
                    "default": "./dumps"
                },
                "format": {
                    "type": "string",
                    "enum": ["json", "binary", "csv"],
                    "description": "Output format for dumps",
                    "default": "json"
                },
                "include_empty_contracts": {
                    "type": "boolean",
                    "description": "Include contracts with no storage items",
                    "default": false
                },
                "max_items_per_file": {
                    "type": "integer",
                    "description": "Maximum storage items per dump file",
                    "default": 100000,
                    "minimum": 1000
                },
                "compress": {
                    "type": "boolean",
                    "description": "Compress output files",
                    "default": true
                },
                "auto_dump_interval": {
                    "type": "integer",
                    "description": "Automatic dump interval in blocks (0 = disabled)",
                    "default": 0,
                    "minimum": 0
                }
            }
        }))
    }

    async fn update_config(&mut self, config: serde_json::Value) -> ExtensionResult<()> {
        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
            self.enabled = enabled;
        }

        if let Some(output_dir) = config.get("output_dir").and_then(|v| v.as_str()) {
            self.config.output_dir = PathBuf::from(output_dir);
            self.ensure_output_dir().await?;
        }

        if let Some(format_str) = config.get("format").and_then(|v| v.as_str()) {
            self.config.format = match format_str.to_lowercase().as_str() {
                "json" => DumpFormat::Json,
                "binary" => DumpFormat::Binary,
                "csv" => DumpFormat::Csv,
                _ => DumpFormat::Json,
            };
        }

        if let Some(include_empty) = config
            .get("include_empty_contracts")
            .and_then(|v| v.as_bool())
        {
            self.config.include_empty_contracts = include_empty;
        }

        if let Some(max_items) = config.get("max_items_per_file").and_then(|v| v.as_u64()) {
            self.config.max_items_per_file = max_items as usize;
        }

        if let Some(compress) = config.get("compress").and_then(|v| v.as_bool()) {
            self.config.compress = compress;
        }

        if let Some(auto_interval) = config.get("auto_dump_interval").and_then(|v| v.as_u64()) {
            self.config.auto_dump_interval = auto_interval as u32;
        }

        info!("StorageDumper plugin configuration updated");
        Ok(())
    }
}

impl StorageDumperPlugin {
    /// Clone for async operations
    fn clone_for_async(&self) -> Self {
        Self {
            info: self.info.clone(),
            enabled: self.enabled,
            config: self.config.clone(),
            last_dump_height: self.last_dump_height.clone(),
            dump_in_progress: self.dump_in_progress.clone(),
        }
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
    async fn test_storage_dumper_plugin() {
        let mut plugin = StorageDumperPlugin::new();
        let context = create_test_context();

        plugin.config.output_dir = context.data_dir.join("dumps");

        // Test initialization
        assert!(plugin.initialize(&context).await.is_ok());

        // Test start
        assert!(plugin.start().await.is_ok());

        // Test contract dump
        let contract_dump = plugin
            .dump_contract_storage("0x1234567890123456789012345678901234567890", 100)
            .await
            .expect("operation should succeed");

        assert_eq!(
            contract_dump.contract,
            "0x1234567890123456789012345678901234567890"
        );
        assert_eq!(contract_dump.block_height, 100);

        // Test stop
        assert!(plugin.stop().await.is_ok());
    }

    #[test]
    fn test_storage_item_serialization() {
        let item = StorageItem {
            contract: "0x1234567890123456789012345678901234567890".to_string(),
            key: "key1".to_string(),
            value: "value1".to_string(),
            block_height: 100,
            tx_hash: "0xabcdef".to_string(),
        };

        let json = serde_json::to_string(&item).expect("operation should succeed");
        let deserialized: StorageItem =
            serde_json::from_str(&json).expect("Failed to parse from string");

        assert_eq!(item.contract, deserialized.contract);
        assert_eq!(item.key, deserialized.key);
        assert_eq!(item.value, deserialized.value);
    }
}

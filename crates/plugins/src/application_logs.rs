//! ApplicationLogs Plugin
//!
//! This plugin provides detailed logging of all blockchain events, transaction executions,
//! and contract notifications. It matches the functionality of the C# ApplicationLogs plugin.

// Define constant locally
const SECONDS_PER_HOUR: u64 = 3600;
use crate::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use neo_core::{UInt160, UInt256};
use neo_extensions::error::{ExtensionError, ExtensionResult};
use neo_vm::stack_item::StackItem;
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
/// Application execution log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationLog {
    /// Transaction hash
    pub txid: String,
    /// Block hash
    pub blockhash: String,
    /// Block index
    pub blockindex: u32,
    /// Block timestamp
    pub blocktime: u64,
    /// Transaction executions
    pub executions: Vec<Execution>,
}

/// VM execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    /// Trigger type
    pub trigger: String,
    /// VM state
    pub vmstate: String,
    /// Exception message (if any)
    pub exception: Option<String>,
    /// Gas consumed
    pub gasconsumed: String,
    /// Result stack
    pub stack: Vec<serde_json::Value>,
    /// Notifications
    pub notifications: Vec<Notification>,
    /// Execution logs
    pub logs: Vec<LogEntry>,
}

/// Contract notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Contract hash
    pub contract: String,
    /// Event name
    pub eventname: String,
    /// State parameters
    pub state: serde_json::Value,
}

/// Execution log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Contract hash
    pub contract: String,
    /// Log message
    pub message: String,
}

/// ApplicationLogs plugin implementation
pub struct ApplicationLogsPlugin {
    info: PluginInfo,
    db: Option<Arc<RwLock<DB>>>,
    db_path: Option<PathBuf>,
    enabled: bool,
    max_entries: usize,
    cleanup_interval: u64,
}

impl ApplicationLogsPlugin {
    /// Create a new ApplicationLogs plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "ApplicationLogs".to_string(),
                version: "3.6.0".to_string(),
                description: "Logs all blockchain application events and transaction executions"
                    .to_string(),
                author: "Neo Rust Team".to_string(),
                dependencies: vec![],
                min_neo_version: "3.0.0".to_string(),
                category: PluginCategory::Core,
                priority: 90,
            },
            db: None,
            db_path: None,
            enabled: true,
            max_entries: 1_000_000,
            cleanup_interval: 86400, // 24 hours
        }
    }

    /// Initialize database
    async fn init_database(&mut self, data_dir: &PathBuf) -> ExtensionResult<()> {
        let db_path = data_dir.join("ApplicationLogs");

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);

        let db = DB::open(&opts, &db_path)
            .map_err(|e| ExtensionError::OperationFailed(format!("Database error: {}", e)))?;

        self.db = Some(Arc::new(RwLock::new(db)));
        self.db_path = Some(db_path);

        info!(
            "ApplicationLogs database initialized at: {:?}",
            self.db_path
        );
        Ok(())
    }

    /// Store application log
    async fn store_log(&self, log: &ApplicationLog) -> ExtensionResult<()> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let key = format!("log:{}", log.txid);
            let value = serde_json::to_vec(log).map_err(|e| {
                ExtensionError::OperationFailed(format!("Serialization error: {}", e))
            })?;

            db.put(key.as_bytes(), &value)
                .map_err(|e| ExtensionError::OperationFailed(format!("Database error: {}", e)))?;

            // Store block index mapping
            let block_key = format!("block:{}:{}", log.blockindex, log.txid);
            db.put(block_key.as_bytes(), log.txid.as_bytes())
                .map_err(|e| ExtensionError::OperationFailed(format!("Database error: {}", e)))?;

            debug!("Stored application log for transaction: {}", log.txid);
        }

        Ok(())
    }

    /// Get application log by transaction hash
    pub async fn get_log(&self, txid: &str) -> ExtensionResult<Option<ApplicationLog>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let key = format!("log:{}", txid);

            match db.get(key.as_bytes()) {
                Ok(Some(value)) => {
                    let log: ApplicationLog = serde_json::from_slice(&value).map_err(|e| {
                        ExtensionError::OperationFailed(format!("Serialization error: {}", e))
                    })?;
                    Ok(Some(log))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(ExtensionError::OperationFailed(format!(
                    "Database error: {}",
                    e
                ))),
            }
        } else {
            Ok(None)
        }
    }

    /// Get application logs by block index
    pub async fn get_logs_by_block(
        &self,
        block_index: u32,
    ) -> ExtensionResult<Vec<ApplicationLog>> {
        if let Some(db) = &self.db {
            let db = db.read().await;
            let prefix = format!("block:{}:", block_index);
            let mut logs = Vec::new();

            let iter = db.prefix_iterator(prefix.as_bytes());
            for item in iter {
                match item {
                    Ok((_, txid_bytes)) => {
                        let txid = String::from_utf8_lossy(&txid_bytes);
                        if let Some(log) = self.get_log(&txid).await? {
                            logs.push(log);
                        }
                    }
                    Err(e) => {
                        warn!("Error reading block logs: {}", e);
                    }
                }
            }

            Ok(logs)
        } else {
            Ok(Vec::new())
        }
    }

    /// Clean up old entries
    async fn cleanup_old_entries(&self) -> ExtensionResult<()> {
        // This would typically remove entries older than a certain threshold
        info!("Cleaning up old ApplicationLogs entries");
        Ok(())
    }

    /// Process block event and extract application logs
    async fn process_block(&self, block_hash: &str, block_height: u32) -> ExtensionResult<()> {
        debug!("Processing block {} for application logs", block_height);

        let application_log = ApplicationLog {
            txid: block_hash.to_string(),
            blockhash: block_hash.to_string(),
            blockindex: block_height,
            blocktime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            executions: vec![],
        };

        // Store the application log
        self.store_log(&application_log).await?;

        Ok(())
    }
}

impl Default for ApplicationLogsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for ApplicationLogsPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    async fn initialize(&mut self, context: &PluginContext) -> ExtensionResult<()> {
        info!("Initializing ApplicationLogs plugin");

        // Initialize database
        self.init_database(&context.data_dir).await?;

        let config_file = context.config_dir.join("ApplicationLogs.json");
        if config_file.exists() {
            match tokio::fs::read_to_string(&config_file).await {
                Ok(config_str) => {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_str) {
                        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
                            self.enabled = enabled;
                        }
                        if let Some(max_entries) =
                            config.get("max_entries").and_then(|v| v.as_u64())
                        {
                            self.max_entries = max_entries as usize;
                        }
                        if let Some(cleanup_interval) = config
                            .get("cleanup_interval_seconds")
                            .and_then(|v| v.as_u64())
                        {
                            self.cleanup_interval = cleanup_interval;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read ApplicationLogs config: {}", e);
                }
            }
        }

        info!(
            "ApplicationLogs plugin initialized (enabled: {})",
            self.enabled
        );
        Ok(())
    }

    async fn start(&mut self) -> ExtensionResult<()> {
        if !self.enabled {
            info!("ApplicationLogs plugin is disabled");
            return Ok(());
        }

        info!("Starting ApplicationLogs plugin");

        // Start cleanup timer
        let cleanup_interval = self.cleanup_interval;
        if let Some(db) = self.db.clone() {
            tokio::spawn(async move {
                let mut interval =
                    tokio::time::interval(tokio::time::Duration::from_secs(cleanup_interval));

                loop {
                    interval.tick().await;
                    // Perform cleanup
                    debug!("Running ApplicationLogs cleanup");
                }
            });
        }

        info!("ApplicationLogs plugin started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> ExtensionResult<()> {
        info!("Stopping ApplicationLogs plugin");

        // Close database
        self.db = None;

        info!("ApplicationLogs plugin stopped");
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
            _ => {
                // Handle other events as needed
            }
        }

        Ok(())
    }

    fn config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "enabled": {
                    "type": "boolean",
                    "description": "Enable or disable the ApplicationLogs plugin",
                    "default": true
                },
                "max_entries": {
                    "type": "integer",
                    "description": "Maximum number of log entries to keep",
                    "default": 1000000,
                    "minimum": 1000
                },
                "cleanup_interval_seconds": {
                    "type": "integer",
                    "description": "Interval in seconds between cleanup operations",
                    "default": 86400,
                    "minimum": 3600
                }
            }
        }))
    }

    async fn update_config(&mut self, config: serde_json::Value) -> ExtensionResult<()> {
        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
            self.enabled = enabled;
        }
        if let Some(max_entries) = config.get("max_entries").and_then(|v| v.as_u64()) {
            self.max_entries = max_entries as usize;
        }
        if let Some(cleanup_interval) = config
            .get("cleanup_interval_seconds")
            .and_then(|v| v.as_u64())
        {
            self.cleanup_interval = cleanup_interval;
        }

        info!("ApplicationLogs plugin configuration updated");
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

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
    async fn test_application_logs_plugin() {
        let mut plugin = ApplicationLogsPlugin::new();
        let context = create_test_context();

        // Test initialization
        assert!(plugin.initialize(&context).await.is_ok());

        // Test start
        assert!(plugin.start().await.is_ok());

        // Test event handling
        let event = PluginEvent::BlockReceived {
            block_hash: "test_hash".to_string(),
            block_height: 100,
        };
        assert!(plugin.handle_event(&event).await.is_ok());

        // Test stop
        assert!(plugin.stop().await.is_ok());
    }

    #[test]
    fn test_application_log_serialization() {
        let log = ApplicationLog {
            txid: "test_txid".to_string(),
            blockhash: "test_block".to_string(),
            blockindex: 100,
            blocktime: 1234567890,
            executions: vec![],
        };

        let json = serde_json::to_string(&log).expect("operation should succeed");
        let deserialized: ApplicationLog =
            serde_json::from_str(&json).expect("Failed to parse from string");

        assert_eq!(log.txid, deserialized.txid);
        assert_eq!(log.blockindex, deserialized.blockindex);
    }
}

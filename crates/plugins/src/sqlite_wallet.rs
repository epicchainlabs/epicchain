//! SQLite Wallet Plugin (Mock Implementation)
//!
//! This plugin provides SQLite database backend support for Neo wallets,
//! allowing storage and management of wallet data in SQLite database files.
//! This is a mock implementation to avoid thread safety issues.

// Define Error and Result types locally
type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;
use crate::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};
use async_trait::async_trait;
use neo_core::{UInt160, UInt256};
use neo_extensions::error::{ExtensionError, ExtensionResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// SQLite wallet implementation (mock)
#[derive(Debug)]
pub struct SqliteWallet {
    /// Database file path
    pub db_path: PathBuf,
    /// Mock data storage
    data: Arc<RwLock<HashMap<String, String>>>,
    /// Wallet version
    pub version: String,
}

impl SqliteWallet {
    /// Creates a new SQLite wallet
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            data: Arc::new(RwLock::new(HashMap::new())),
            version: "1.0.0".to_string(),
        }
    }

    /// Opens the SQLite database connection (mock)
    pub async fn open(&self) -> ExtensionResult<()> {
        info!("Mock: Opening SQLite wallet at {:?}", self.db_path);
        Ok(())
    }

    /// Closes the database connection (mock)
    pub async fn close(&self) -> ExtensionResult<()> {
        info!("Mock: Closing SQLite wallet");
        Ok(())
    }
}

/// SQLiteWallet plugin implementation
#[derive(Debug)]
pub struct SqliteWalletPlugin {
    info: PluginInfo,
    enabled: bool,
    wallets: Arc<RwLock<HashMap<String, SqliteWallet>>>,
    default_wallet_path: Option<PathBuf>,
}

impl SqliteWalletPlugin {
    /// Creates a new SQLite wallet plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "SQLiteWallet".to_string(),
                description: "SQLite wallet backend support".to_string(),
                version: "1.0.0".to_string(),
                author: "Neo Rust Team".to_string(),
                category: PluginCategory::Wallet,
                dependencies: vec![],
                min_neo_version: "3.0.0".to_string(),
                priority: 10,
            },
            enabled: false,
            wallets: Arc::new(RwLock::new(HashMap::new())),
            default_wallet_path: None,
        }
    }

    /// Check if wallet exists by ID
    pub async fn has_wallet(&self, wallet_id: &str) -> bool {
        let wallets = self.wallets.read().await;
        wallets.contains_key(wallet_id)
    }

    /// List all open wallets
    pub async fn list_wallets(&self) -> Vec<String> {
        let wallets = self.wallets.read().await;
        wallets.keys().cloned().collect()
    }
}

impl Default for SqliteWalletPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for SqliteWalletPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    async fn initialize(&mut self, _context: &PluginContext) -> ExtensionResult<()> {
        info!("Initializing SQLite wallet plugin");
        self.enabled = true;
        Ok(())
    }

    async fn start(&mut self) -> ExtensionResult<()> {
        info!("Starting SQLite wallet plugin");
        self.enabled = true;
        Ok(())
    }

    async fn stop(&mut self) -> ExtensionResult<()> {
        info!("Stopping SQLite wallet plugin");
        self.enabled = false;
        Ok(())
    }

    async fn handle_event(&mut self, _event: &PluginEvent) -> ExtensionResult<()> {
        // Mock event handling
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    fn create_test_context() -> PluginContext {
        let temp_dir = tempdir().unwrap();
        PluginContext {
            neo_version: "3.6.0".to_string(),
            config_dir: temp_dir.path().to_path_buf(),
            data_dir: temp_dir.path().to_path_buf(),
            shared_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[tokio::test]
    async fn test_plugin_initialization() {
        let mut plugin = SqliteWalletPlugin::new();
        let context = create_test_context();

        // Initial state is disabled
        assert_eq!(plugin.enabled, false);
        plugin
            .initialize(&context)
            .await
            .expect("operation should succeed");
        assert_eq!(plugin.enabled, true);
    }

    #[tokio::test]
    async fn test_wallet_operations() {
        let plugin = SqliteWalletPlugin::new();

        assert!(!plugin.has_wallet("test").await);
        assert_eq!(plugin.list_wallets().await.len(), 0);
    }
}

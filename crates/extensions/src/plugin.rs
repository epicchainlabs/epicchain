//! Plugin system for Neo Extensions
//!
//! This module provides a complete plugin architecture that matches
//! the C# Neo plugin system for extensibility.

use crate::error::{ExtensionError, ExtensionResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Plugin trait that all Neo plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin information
    fn info(&self) -> &PluginInfo;

    /// Initialize the plugin
    async fn initialize(&mut self, context: &PluginContext) -> ExtensionResult<()>;

    /// Start the plugin
    async fn start(&mut self) -> ExtensionResult<()>;

    /// Stop the plugin
    async fn stop(&mut self) -> ExtensionResult<()>;

    /// Handle a plugin event
    async fn handle_event(&mut self, event: &PluginEvent) -> ExtensionResult<()>;

    /// Get plugin configuration schema
    fn config_schema(&self) -> Option<serde_json::Value> {
        None
    }

    /// Update plugin configuration
    async fn update_config(&mut self, _config: serde_json::Value) -> ExtensionResult<()> {
        Ok(())
    }
}

/// Plugin information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin description
    pub description: String,

    /// Plugin author
    pub author: String,

    /// Plugin dependencies
    pub dependencies: Vec<String>,

    /// Minimum Neo version required
    pub min_neo_version: String,

    /// Plugin category
    pub category: PluginCategory,

    /// Plugin priority (higher = loaded first)
    pub priority: i32,
}

/// Plugin categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginCategory {
    /// Core system plugins
    Core,

    /// Network protocol plugins
    Network,

    /// Consensus plugins
    Consensus,

    /// RPC plugins
    Rpc,

    /// Wallet plugins
    Wallet,

    /// Storage plugins
    Storage,

    /// Utility plugins
    Utility,

    /// Custom plugins
    Custom(String),
}

/// Plugin context provided during initialization
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Neo version
    pub neo_version: String,

    /// Plugin configuration directory
    pub config_dir: PathBuf,

    /// Plugin data directory
    pub data_dir: PathBuf,

    /// Shared plugin data
    pub shared_data: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

/// Plugin events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    /// Node started
    NodeStarted,

    /// Node stopping
    NodeStopping,

    /// Block received
    BlockReceived {
        block_hash: String,
        block_height: u32,
    },

    /// Transaction received
    TransactionReceived { tx_hash: String },

    /// Consensus state changed
    ConsensusStateChanged { state: String },

    /// RPC request received
    RpcRequest {
        method: String,
        params: serde_json::Value,
    },

    /// Custom event
    Custom {
        event_type: String,
        data: serde_json::Value,
    },
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    plugin_order: Vec<String>,
    context: PluginContext,
    is_initialized: bool,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(context: PluginContext) -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_order: Vec::new(),
            context,
            is_initialized: false,
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> ExtensionResult<()> {
        let info = plugin.info().clone();

        if self.plugins.contains_key(&info.name) {
            return Err(ExtensionError::PluginAlreadyExists(info.name));
        }

        // Check dependencies
        for dep in &info.dependencies {
            if !self.plugins.contains_key(dep) {
                return Err(ExtensionError::MissingDependency {
                    plugin: info.name,
                    dependency: dep.clone(),
                });
            }
        }

        info!("Registering plugin: {} v{}", info.name, info.version);

        // Insert in priority order
        let insert_pos = self
            .plugin_order
            .iter()
            .position(|name| {
                self.plugins
                    .get(name)
                    .map(|p| p.info().priority < info.priority)
                    .unwrap_or(false)
            })
            .unwrap_or(self.plugin_order.len());

        self.plugin_order.insert(insert_pos, info.name.clone());
        self.plugins.insert(info.name, plugin);

        Ok(())
    }

    /// Initialize all plugins
    pub async fn initialize_all(&mut self) -> ExtensionResult<()> {
        if self.is_initialized {
            return Ok(());
        }

        info!("Initializing {} plugins", self.plugins.len());

        for plugin_name in &self.plugin_order.clone() {
            if let Some(plugin) = self.plugins.get_mut(plugin_name) {
                info!("Initializing plugin: {}", plugin_name);

                match plugin.initialize(&self.context).await {
                    Ok(()) => {
                        debug!("Plugin {} initialized successfully", plugin_name);
                    }
                    Err(e) => {
                        error!("Failed to initialize plugin {}: {}", plugin_name, e);
                        return Err(e);
                    }
                }
            }
        }

        self.is_initialized = true;
        info!("All plugins initialized successfully");
        Ok(())
    }

    /// Start all plugins
    pub async fn start_all(&mut self) -> ExtensionResult<()> {
        if !self.is_initialized {
            return Err(ExtensionError::NotInitialized);
        }

        info!("Starting {} plugins", self.plugins.len());

        for plugin_name in &self.plugin_order.clone() {
            if let Some(plugin) = self.plugins.get_mut(plugin_name) {
                info!("Starting plugin: {}", plugin_name);

                match plugin.start().await {
                    Ok(()) => {
                        debug!("Plugin {} started successfully", plugin_name);
                    }
                    Err(e) => {
                        error!("Failed to start plugin {}: {}", plugin_name, e);
                        return Err(e);
                    }
                }
            }
        }

        info!("All plugins started successfully");
        Ok(())
    }

    /// Stop all plugins
    pub async fn stop_all(&mut self) -> ExtensionResult<()> {
        info!("Stopping {} plugins", self.plugins.len());

        // Stop in reverse order
        for plugin_name in self.plugin_order.iter().rev() {
            if let Some(plugin) = self.plugins.get_mut(plugin_name) {
                info!("Stopping plugin: {}", plugin_name);

                match plugin.stop().await {
                    Ok(()) => {
                        debug!("Plugin {} stopped successfully", plugin_name);
                    }
                    Err(e) => {
                        warn!("Error stopping plugin {}: {}", plugin_name, e);
                    }
                }
            }
        }

        info!("All plugins stopped");
        Ok(())
    }

    /// Broadcast an event to all plugins
    pub async fn broadcast_event(&mut self, event: &PluginEvent) -> ExtensionResult<()> {
        debug!("Broadcasting event: {:?}", event);

        for plugin_name in &self.plugin_order.clone() {
            if let Some(plugin) = self.plugins.get_mut(plugin_name) {
                match plugin.handle_event(event).await {
                    Ok(()) => {
                        debug!("Plugin {} handled event successfully", plugin_name);
                    }
                    Err(e) => {
                        warn!("Plugin {} failed to handle event: {}", plugin_name, e);
                        // Continue with other plugins
                    }
                }
            }
        }

        Ok(())
    }

    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Get mutable plugin by name
    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(name)
    }

    /// List all plugins
    pub fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.plugin_order
            .iter()
            .filter_map(|name| self.plugins.get(name).map(|p| p.info()))
            .collect()
    }

    /// Get plugins by category
    pub fn get_plugins_by_category(&self, category: &PluginCategory) -> Vec<&PluginInfo> {
        self.plugins
            .values()
            .map(|p| p.info())
            .filter(|info| &info.category == category)
            .collect()
    }

    /// Check if plugin exists
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

/// Macro for registering plugins
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        inventory::submit! {
            fn() -> Box<dyn $crate::plugin::Plugin> {
                Box::new(<$plugin_type>::new())
            }
        }
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tempfile::tempdir;

    struct TestPlugin {
        info: PluginInfo,
        initialized: AtomicBool,
        started: AtomicBool,
    }

    impl TestPlugin {
        fn new(name: &str, priority: i32) -> Self {
            Self {
                info: PluginInfo {
                    name: name.to_string(),
                    version: "1.0.0".to_string(),
                    description: "Test plugin".to_string(),
                    author: "Test Author".to_string(),
                    dependencies: vec![],
                    min_neo_version: "3.0.0".to_string(),
                    category: PluginCategory::Utility,
                    priority,
                },
                initialized: AtomicBool::new(false),
                started: AtomicBool::new(false),
            }
        }
    }

    #[async_trait]
    impl Plugin for TestPlugin {
        fn info(&self) -> &PluginInfo {
            &self.info
        }

        async fn initialize(&mut self, _context: &PluginContext) -> ExtensionResult<()> {
            self.initialized.store(true, Ordering::Relaxed);
            Ok(())
        }

        async fn start(&mut self) -> ExtensionResult<()> {
            self.started.store(true, Ordering::Relaxed);
            Ok(())
        }

        async fn stop(&mut self) -> ExtensionResult<()> {
            self.started.store(false, Ordering::Relaxed);
            Ok(())
        }

        async fn handle_event(&mut self, _event: &PluginEvent) -> ExtensionResult<()> {
            Ok(())
        }
    }

    fn create_test_context() -> PluginContext {
        let final_dir = tempdir().expect("Operation failed");
        PluginContext {
            neo_version: "3.6.0".to_string(),
            config_dir: final_dir.path().to_path_buf(),
            data_dir: final_dir.path().to_path_buf(),
            shared_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[tokio::test]
    async fn test_plugin_manager() {
        let context = create_test_context();
        let mut manager = PluginManager::new(context);

        // Register plugins
        let plugin1 = Box::new(TestPlugin::new("plugin1", 10));
        let plugin2 = Box::new(TestPlugin::new("plugin2", 5));

        manager
            .register_plugin(plugin1)
            .expect("operation should succeed");
        manager
            .register_plugin(plugin2)
            .expect("operation should succeed");

        assert_eq!(manager.plugin_count(), 2);
        assert!(manager.has_plugin("plugin1"));
        assert!(manager.has_plugin("plugin2"));

        // Initialize and start
        manager
            .initialize_all()
            .await
            .expect("operation should succeed");
        manager.start_all().await.expect("operation should succeed");

        // Test event broadcasting
        let event = PluginEvent::NodeStarted;
        manager
            .broadcast_event(&event)
            .await
            .expect("operation should succeed");

        // Stop plugins
        manager.stop_all().await.expect("operation should succeed");
    }

    #[test]
    fn test_plugin_info() {
        let info = PluginInfo {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            dependencies: vec!["dep1".to_string()],
            min_neo_version: "3.0.0".to_string(),
            category: PluginCategory::Core,
            priority: 100,
        };

        assert_eq!(info.name, "test");
        assert_eq!(info.category, PluginCategory::Core);
        assert_eq!(info.priority, 100);
    }
}

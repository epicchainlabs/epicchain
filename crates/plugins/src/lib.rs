//! Neo Plugins Collection
//!
//! This crate provides a collection of plugins that extend Neo's functionality,
//! matching the C# Neo implementation plugins.

pub mod application_logs;
pub mod complete_plugin_system;
pub mod console_commands;
pub mod dbft_plugin;
pub mod oracle_service;
pub mod plugin_loader;
pub mod rpc_server;
pub mod sqlite_wallet;
pub mod state_service;
pub mod storage_dumper;
pub mod tokens_tracker;

// Re-export common types
pub use neo_extensions::plugin::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};

/// Plugin collection for easy registration
pub struct PluginCollection;

impl PluginCollection {
    /// Get all available plugins (matches C# Neo plugin collection exactly)
    pub fn all_plugins() -> Vec<Box<dyn Plugin>> {
        vec![
            Box::new(application_logs::ApplicationLogsPlugin::new()),
            Box::new(dbft_plugin::DbftPlugin::new()),
            Box::new(oracle_service::OracleServicePlugin::new()),
            Box::new(rpc_server::RpcServerPlugin::new()),
            Box::new(state_service::StateServicePlugin::new()),
            Box::new(storage_dumper::StorageDumperPlugin::new()),
            Box::new(sqlite_wallet::SqliteWalletPlugin::new()),
            Box::new(tokens_tracker::TokensTrackerPlugin::new()),
        ]
    }

    /// Get core plugins (essential for most deployments)
    pub fn core_plugins() -> Vec<Box<dyn Plugin>> {
        vec![
            Box::new(application_logs::ApplicationLogsPlugin::new()),
            Box::new(dbft_plugin::DbftPlugin::new()),
            Box::new(rpc_server::RpcServerPlugin::new()),
            Box::new(tokens_tracker::TokensTrackerPlugin::new()),
        ]
    }

    /// Get RPC-related plugins
    pub fn rpc_plugins() -> Vec<Box<dyn Plugin>> {
        vec![
            Box::new(application_logs::ApplicationLogsPlugin::new()),
            Box::new(rpc_server::RpcServerPlugin::new()),
            Box::new(state_service::StateServicePlugin::new()),
            Box::new(tokens_tracker::TokensTrackerPlugin::new()),
        ]
    }

    /// Get utility plugins
    pub fn utility_plugins() -> Vec<Box<dyn Plugin>> {
        vec![
            Box::new(storage_dumper::StorageDumperPlugin::new()),
            Box::new(sqlite_wallet::SqliteWalletPlugin::new()),
        ]
    }
}

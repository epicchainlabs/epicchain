//! EpicChain-Rust CLI Library
//!
//! This crate provides the command-line interface for EpicChain blockchain node operations.

use log::{debug, error, info, warn};
use std::fmt;

/// CLI version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// EpicChain version compatibility
pub const NEO_VERSION: &str = "3.6.0";

/// EpicChain VM version compatibility
pub const VM_VERSION: &str = "3.6.0";

/// CLI error types
#[derive(Debug)]
pub enum CliError {
    /// Configuration error
    Config(String),
    /// Wallet error
    Wallet(String),
    /// Node error
    Node(String),
    /// RPC error
    Rpc(String),
    /// Console error
    Console(String),
    /// IO error
    Io(std::io::Error),
    /// General error
    General(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Config(msg) => write!(f, "Configuration error: {}", msg),
            CliError::Wallet(msg) => write!(f, "Wallet error: {}", msg),
            CliError::Node(msg) => write!(f, "Node error: {}", msg),
            CliError::Rpc(msg) => write!(f, "RPC error: {}", msg),
            CliError::Console(msg) => write!(f, "Console error: {}", msg),
            CliError::Io(err) => write!(f, "IO error: {}", err),
            CliError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, CliError>;

/// Display version information
pub fn display_version() {
    log::info!("neo-cli {}", VERSION);
    log::info!("Neo N3 compatibility: {}", NEO_VERSION);
    log::info!("Neo VM compatibility: {}", VM_VERSION);
}

// Define the modules first
pub mod args;
pub mod config;
pub mod console;
pub mod node;
pub mod rpc;
pub mod service;
pub mod wallet;

pub use args::CliArgs;
pub use service::MainService;

/// Get version information as a string (for testing)
pub fn get_version_info() -> String {
    format!(
        "Neo CLI v{}\nNeo Core v{}\nNeo VM v{}",
        VERSION, NEO_VERSION, VM_VERSION
    )
}

use thiserror::Error;

/// CLI-specific error types
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Node error: {0}")]
    Node(String),
    
    #[error("Wallet error: {0}")]
    Wallet(String),
    
    #[error("RPC error: {0}")]
    Rpc(String),
    
    #[error("Console error: {0}")]
    Console(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("Core error: {0}")]
    Core(#[from] neo_core::CoreError),
    
    #[error("Cryptography error: {0}")]
    Cryptography(#[from] neo_cryptography::Error),
    
    #[error("JSON library error: {0}")]
    JsonLib(#[from] neo_json::JsonError),
    
    #[error("MPT Trie error: {0}")]
    MptTrie(#[from] neo_mpt_trie::MptError),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Operation cancelled")]
    Cancelled,
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, CliError>; 
//! Neo Configuration Module
//!
//! This module provides configuration types for the Neo N3 Rust node.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::net::SocketAddr;
use std::str::FromStr;

/// Blockchain timing constants
pub const SECONDS_PER_BLOCK: u64 = 15;

/// Default Neo network ports
pub const DEFAULT_NEO_PORT: &str = "10333";
pub const DEFAULT_RPC_PORT: &str = "10332";
pub const DEFAULT_TESTNET_PORT: &str = "20333";
pub const DEFAULT_TESTNET_RPC_PORT: &str = "20332";
pub const MILLISECONDS_PER_BLOCK: u64 = SECONDS_PER_BLOCK * 1000;

/// Network limits constants
pub const MAX_BLOCK_SIZE: usize = 2_097_152; // 2MB (matches C# Neo)
pub const MAX_TRANSACTION_SIZE: usize = 102_400; // 100KB (matches C# Neo)
pub const MAX_TRANSACTIONS_PER_BLOCK: usize = 512;

/// Maximum number of blocks that can be traced (about 1 year)
pub const MAX_TRACEABLE_BLOCKS: u32 = 2_102_400;
/// Size of a hash (UInt256) in bytes
pub const HASH_SIZE: usize = 32;
/// Size of an address (UInt160) in bytes
pub const ADDRESS_SIZE: usize = 20;
/// Maximum script size in bytes
pub const MAX_SCRIPT_SIZE: usize = 65536; // 64KB
/// Maximum script length (64KB)
pub const MAX_SCRIPT_LENGTH: usize = 65536;
/// Network type for Neo blockchain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum NetworkType {
    #[default]
    MainNet,
    TestNet,
    Private,
}
/// Neo MainNet seed nodes
pub const MAINNET_SEEDS: &[&str] = &[
    // NGD Network nodes - these use standard Neo protocol
    "seed1.ngd.network:10333", // 35.226.217.8
    "seed2.ngd.network:10333", // 34.69.36.148
];

/// Neo TestNet seed nodes  
pub const TESTNET_SEEDS: &[&str] = &[
    "seed1t.neo.org:20333",
    "seed2t.neo.org:20333",
    "seed3t.neo.org:20333",
    "seed4t.neo.org:20333",
    "seed5t.neo.org:20333",
];

/// Neo N3 TestNet seed nodes
pub const N3_TESTNET_SEEDS: &[&str] = &[
    "34.133.235.69:20333",  // seed1t5.neo.org
    "35.192.59.217:20333",  // seed2t5.neo.org
    "35.188.199.101:20333", // seed3t5.neo.org
    "35.238.26.128:20333",  // seed4t5.neo.org
    "34.124.145.177:20333", // seed5t5.neo.org
];

impl NetworkType {
    /// Gets the network magic number
    pub fn magic(&self) -> u32 {
        match self {
            NetworkType::MainNet => 0x334f454e, // "NEO3" in little endian
            NetworkType::TestNet => 0x3254334e, // "N3T2" in little endian
            NetworkType::Private => 0x00000000,
        }
    }

    /// Gets the address version
    pub fn address_version(&self) -> u8 {
        match self {
            NetworkType::MainNet => 0x35,
            NetworkType::TestNet => 0x35,
            NetworkType::Private => 0x35,
        }
    }
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkType::MainNet => write!(f, "MainNet"),
            NetworkType::TestNet => write!(f, "TestNet"),
            NetworkType::Private => write!(f, "Private"),
        }
    }
}

impl FromStr for NetworkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" | "main" => Ok(NetworkType::MainNet),
            "testnet" | "test" => Ok(NetworkType::TestNet),
            "private" | "privnet" => Ok(NetworkType::Private),
            _ => Err(format!("Unknown network type: {s}")),
        }
    }
}

/// Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub consensus_enabled: bool,
    pub network_type: NetworkType,
    pub consensus_config: ConsensusConfig,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            consensus_enabled: false,
            network_type: NetworkType::TestNet,
            consensus_config: ConsensusConfig::default(),
        }
    }
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub enabled: bool,
    pub view_timeout_ms: u64,
    pub block_time_ms: u64,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            view_timeout_ms: 10000,
            block_time_ms: MILLISECONDS_PER_BLOCK,
        }
    }
}

/// Ledger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerConfig {
    pub max_transactions_per_block: usize,
    pub max_block_size: usize,
    pub milliseconds_per_block: u64,
}

impl Default for LedgerConfig {
    fn default() -> Self {
        Self {
            max_transactions_per_block: MAX_TRANSACTIONS_PER_BLOCK,
            max_block_size: MAX_BLOCK_SIZE,
            milliseconds_per_block: MILLISECONDS_PER_BLOCK,
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub port: u16,
    pub max_outbound_connections: usize,
    pub max_inbound_connections: usize,
    pub connection_timeout_secs: u64,
    pub seed_nodes: Vec<SocketAddr>,
    pub user_agent: String,
    pub protocol_version: u32,
    pub websocket_enabled: bool,
    pub websocket_port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 20333, // TestNet port
            max_outbound_connections: 10,
            max_inbound_connections: 40,
            connection_timeout_secs: 30,
            seed_nodes: vec![
                "seed1t.neo.org:20333".parse().ok(),
                "seed2t.neo.org:20333".parse().ok(),
                "seed3t.neo.org:20333".parse().ok(),
                "seed4t.neo.org:20333".parse().ok(),
                "seed5t.neo.org:20333".parse().ok(),
            ]
            .into_iter()
            .flatten()
            .collect(),
            user_agent: "Neo-Rust/0.1.0".to_string(),
            protocol_version: 3,
            websocket_enabled: false,
            websocket_port: 20334,
        }
    }
}

/// RPC server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcServerConfig {
    pub enabled: bool,
    pub port: u16,
    pub bind_address: String,
    pub max_connections: usize,
    pub cors_enabled: bool,
    pub ssl_enabled: bool,
}

impl Default for RpcServerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 20332, // TestNet RPC port
            bind_address: "localhost".to_string(),
            max_connections: 50,
            cors_enabled: true,
            ssl_enabled: false,
        }
    }
}

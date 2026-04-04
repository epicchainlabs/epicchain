//! Node configuration management.
//!
//! This module provides configuration structures and management for the Neo node.

use neo_config::{MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK, MILLISECONDS_PER_BLOCK};
use neo_persistence::storage::StorageConfig;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;

use crate::constants::{defaults, magic, ports, seed_nodes};

/// Complete node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node identification
    pub node: NodeInfo,
    /// Network configuration
    pub network: NetworkConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// RPC server configuration
    pub rpc: RpcConfig,
    /// Consensus configuration
    pub consensus: ConsensusConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Whether this is a TestNet node
    pub is_testnet: bool,
    /// Whether this is a MainNet node  
    pub is_mainnet: bool,
    /// RPC server port (convenience field)
    pub rpc_port: u16,
    /// P2P network port (convenience field)
    pub p2p_port: u16,
    /// Data directory for blockchain data (convenience field)
    pub data_dir: PathBuf,
}

/// Node identification information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node name
    pub name: String,
    /// Node version
    pub version: String,
    /// Node user agent
    pub user_agent: String,
    /// Data directory
    pub data_dir: PathBuf,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Network magic number (mainnet: 0x334f454e, testnet: 0x3554334e)
    pub magic: u32,
    /// P2P listen address
    pub listen_addr: SocketAddr,
    /// Maximum number of peers
    pub max_peers: usize,
    /// Seed nodes for initial connection
    pub seed_nodes: Vec<SocketAddr>,
    /// Enable UPnP for port forwarding
    pub enable_upnp: bool,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Ping interval in seconds
    pub ping_interval: u64,
}

/// RPC server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcConfig {
    /// Enable RPC server
    pub enabled: bool,
    /// RPC listen address
    pub listen_addr: SocketAddr,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Enable CORS
    pub enable_cors: bool,
    /// Allowed origins for CORS
    pub cors_origins: Vec<String>,
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Enable consensus participation
    pub enabled: bool,
    /// Consensus algorithm (dBFT for Neo)
    pub algorithm: String,
    /// Block time in milliseconds
    pub block_time_ms: u64,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Maximum block size in bytes
    pub max_block_size: usize,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, pretty)
    pub format: String,
    /// Log to file
    pub log_to_file: bool,
    /// Log file path
    pub log_file: Option<PathBuf>,
    /// Maximum log file size in MB
    pub max_file_size_mb: usize,
    /// Number of log files to keep
    pub max_files: usize,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node: NodeInfo::default(),
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            rpc: RpcConfig::default(),
            consensus: ConsensusConfig::default(),
            logging: LoggingConfig::default(),
            is_testnet: false,
            is_mainnet: true, // Default to mainnet
            rpc_port: ports::MAINNET_RPC,
            p2p_port: ports::MAINNET_P2P,
            data_dir: PathBuf::from("./data"),
        }
    }
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self {
            name: "neo-rust-node".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            user_agent: format!("neo-rust/{}", env!("CARGO_PKG_VERSION")),
            data_dir: PathBuf::from("./data"),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::mainnet()
    }
}

impl NetworkConfig {
    /// Create mainnet network configuration
    pub fn mainnet() -> Self {
        Self {
            magic: magic::MAINNET,
            listen_addr: format!("localhost:{}", ports::MAINNET_P2P)
                .parse()
                .unwrap_or_else(|_| "0.0.0.0:10333".parse().expect("value should parse")),
            max_peers: defaults::MAX_PEERS,
            seed_nodes: seed_nodes::parse_seed_nodes(seed_nodes::MAINNET),
            enable_upnp: false,
            connection_timeout: defaults::CONNECTION_TIMEOUT,
            ping_interval: defaults::PING_INTERVAL,
        }
    }

    /// Create testnet network configuration
    pub fn testnet() -> Self {
        Self {
            magic: magic::TESTNET,
            listen_addr: format!("localhost:{}", ports::TESTNET_P2P)
                .parse()
                .unwrap_or_else(|_| "0.0.0.0:20333".parse().expect("value should parse")),
            max_peers: defaults::MAX_PEERS,
            seed_nodes: seed_nodes::parse_seed_nodes(seed_nodes::TESTNET),
            enable_upnp: false,
            connection_timeout: defaults::CONNECTION_TIMEOUT,
            ping_interval: defaults::PING_INTERVAL,
        }
    }

    /// Create regtest/private network configuration
    pub fn regtest() -> Self {
        Self {
            magic: magic::REGTEST,
            listen_addr: format!("localhost:{}", ports::REGTEST_P2P)
                .parse()
                .unwrap_or_else(|_| "0.0.0.0:30333".parse().expect("value should parse")),
            max_peers: 10, // Fewer peers for private network
            seed_nodes: seed_nodes::parse_seed_nodes(seed_nodes::REGTEST),
            enable_upnp: false,
            connection_timeout: 10, // Faster timeouts for local testing
            ping_interval: 15,      // 15 seconds ping interval
        }
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self::mainnet()
    }
}

impl RpcConfig {
    /// Create mainnet RPC configuration
    pub fn mainnet() -> Self {
        Self {
            enabled: true,
            listen_addr: format!("localhost:{}", ports::MAINNET_RPC)
                .parse()
                .unwrap_or_else(|_| "0.0.0.0:10332".parse().expect("value should parse")),
            max_connections: defaults::MAX_RPC_CONNECTIONS,
            request_timeout: defaults::RPC_TIMEOUT,
            enable_cors: false,   // More restrictive for mainnet
            cors_origins: vec![], // No CORS origins for mainnet by default
        }
    }

    /// Create testnet RPC configuration  
    pub fn testnet() -> Self {
        Self {
            enabled: true,
            listen_addr: format!("localhost:{}", ports::TESTNET_RPC)
                .parse()
                .unwrap_or_else(|_| "0.0.0.0:20332".parse().expect("value should parse")),
            max_connections: defaults::MAX_RPC_CONNECTIONS,
            request_timeout: defaults::RPC_TIMEOUT,
            enable_cors: true,                   // More permissive for testing
            cors_origins: vec!["*".to_string()], // Allow all origins for testing
        }
    }

    /// Create regtest RPC configuration
    pub fn regtest() -> Self {
        Self {
            enabled: true,
            listen_addr: format!("localhost:{}", ports::REGTEST_RPC)
                .parse()
                .unwrap_or_else(|_| "0.0.0.0:30332".parse().expect("value should parse")),
            max_connections: 50, // Fewer connections for development
            request_timeout: 60, // Longer timeout for debugging
            enable_cors: true,   // More permissive for development
            cors_origins: vec!["*".to_string()], // Allow all origins for development
        }
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for non-consensus nodes
            algorithm: "dBFT".to_string(),
            block_time_ms: MILLISECONDS_PER_BLOCK, // SECONDS_PER_BLOCK seconds
            max_transactions_per_block: MAX_TRANSACTIONS_PER_BLOCK,
            max_block_size: MAX_BLOCK_SIZE,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            log_to_file: true,
            log_file: Some(PathBuf::from("./logs/neo-node.log")),
            max_file_size_mb: 100,
            max_files: 10,
        }
    }
}

impl NodeConfig {
    /// Load configuration from file or create default
    pub fn load_or_create(
        config_path: &str,
        is_testnet: bool,
        is_mainnet: bool,
        rpc_port: u16,
        p2p_port: u16,
        data_dir: PathBuf,
    ) -> anyhow::Result<Self> {
        let config_path = PathBuf::from(config_path);

        let mut config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str::<NodeConfig>(&content)?
        } else {
            // Create default config based on network type
            let default_config = if is_testnet {
                Self::testnet()
            } else if is_mainnet {
                Self::default() // mainnet
            } else {
                Self::regtest()
            };

            // Save default config
            let config_content = toml::to_string_pretty(&default_config)?;
            std::fs::write(&config_path, config_content)?;

            default_config
        };

        // Override with command line parameters
        config.rpc.listen_addr = format!("localhost:{}", rpc_port)
            .parse()
            .unwrap_or_else(|_| "0.0.0.0:10332".parse().expect("value should parse"));
        config.network.listen_addr = format!("localhost:{}", p2p_port)
            .parse()
            .unwrap_or_else(|_| "0.0.0.0:10333".parse().expect("value should parse"));
        config.node.data_dir = data_dir;

        config.is_testnet = is_testnet;
        config.is_mainnet = is_mainnet;

        Ok(config)
    }

    /// Load configuration from file
    pub fn load_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: NodeConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create default configuration for testnet
    pub fn testnet() -> Self {
        let mut config = Self::default();
        config.network = NetworkConfig::testnet();
        config.rpc = RpcConfig::testnet();
        config.is_testnet = true;
        config.is_mainnet = false;
        config.rpc_port = ports::TESTNET_RPC;
        config.p2p_port = ports::TESTNET_P2P;
        config
    }

    /// Create default configuration for regtest
    pub fn regtest() -> Self {
        let mut config = Self::default();
        config.network = NetworkConfig::regtest();
        config.rpc = RpcConfig::regtest();
        config.consensus.enabled = true; // Enable consensus for regtest
        config.consensus.block_time_ms = 1000; // 1 second for testing
        config.rpc_port = ports::REGTEST_RPC;
        config.p2p_port = ports::REGTEST_P2P;
        config
    }

    /// Validate configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate network configuration
        if self.network.max_peers == 0 {
            return Err(anyhow::anyhow!("max_peers must be greater than 0"));
        }

        // Validate RPC configuration
        if self.rpc.enabled && self.rpc.max_connections == 0 {
            return Err(anyhow::anyhow!(
                "RPC max_connections must be greater than 0"
            ));
        }

        // Validate consensus configuration
        if self.consensus.enabled {
            if self.consensus.block_time_ms == 0 {
                return Err(anyhow::anyhow!("block_time_ms must be greater than 0"));
            }
            if self.consensus.max_transactions_per_block == 0 {
                return Err(anyhow::anyhow!(
                    "max_transactions_per_block must be greater than 0"
                ));
            }
        }

        Ok(())
    }
}

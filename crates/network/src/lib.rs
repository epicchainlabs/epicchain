//! # Neo Network Protocol
//!
//! Comprehensive networking functionality for the Neo blockchain protocol.
//!
//! This crate implements the complete Neo network protocol including peer-to-peer
//! communication, message handling, blockchain synchronization, and JSON-RPC services.
//! It provides a robust and scalable networking layer that enables Neo nodes to
//! participate in the blockchain network.
//!
//! ## Features
//!
//! - **P2P Protocol**: Complete Neo P2P protocol implementation with handshaking
//! - **Message Handling**: Type-safe network message serialization and validation
//! - **Peer Management**: Connection management, discovery, and health monitoring
//! - **Blockchain Sync**: Fast block synchronization with fork detection
//! - **Transaction Relay**: Efficient transaction propagation and relay
//! - **JSON-RPC API**: Complete RPC server for external integrations
//! - **Error Recovery**: Comprehensive error handling and connection recovery
//!
//! ## Architecture
//!
//! The network layer is organized into several core components:
//!
//! - **P2P Node**: Main networking interface and connection management
//! - **Peer Manager**: Peer discovery, connection lifecycle, and health monitoring
//! - **Message Protocol**: Network message types and serialization
//! - **Sync Manager**: Blockchain synchronization and fork resolution
//! - **RPC Server**: JSON-RPC API server for external clients
//! - **Transaction Relay**: Transaction propagation and relay cache
//! - **Network Server**: High-level network service coordination
//!
//! ## Example Usage
//!
//! ### Basic Network Node
//!
//! ```rust,no_run
//! use neo_network::{NetworkConfig, P2pNode, NetworkEvent};
//! use tokio::sync::mpsc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create network configuration
//! let config = NetworkConfig::default();
//!
//! // Create event channel
//! let (event_tx, mut event_rx) = mpsc::channel(100);
//!
//! // Start P2P node
//! let mut node = P2pNode::new(config, event_tx).await?;
//! node.start().await?;
//!
//! // Handle network events
//! while let Some(event) = event_rx.recv().await {
//!     match event {
//!         NetworkEvent::PeerConnected { peer_id, address } => {
//!             println!("Peer connected: {} at {}", peer_id, address);
//!         }
//!         NetworkEvent::BlockReceived { block_hash, height } => {
//!             println!("Received block {} at height {}", block_hash, height);
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Custom Message Handling
//!
//! ```rust,no_run
//! use neo_network::{NetworkMessage, MessageCommand};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a version message
//! let message = NetworkMessage::new(
//!     MessageCommand::Version,
//!     vec![], // payload
//! )?;
//!
//! // Validate message
//! if message.validate()? {
//!     println!("Message is valid");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Network Configuration
//!
//! ```rust
//! use neo_network::NetworkConfig;
//!
//! // MainNet configuration
//! let mainnet = NetworkConfig::default();
//!
//! // TestNet configuration
//! let testnet = NetworkConfig::testnet();
//!
//! // Private network configuration
//! let private = NetworkConfig::private();
//! ```
//!
//! ## Network Protocols
//!
//! ### P2P Protocol
//!
//! The P2P protocol implements the Neo network specification:
//! - Version negotiation and capability exchange
//! - Peer discovery through seed nodes and peer exchange
//! - Keep-alive mechanism with ping/pong messages
//! - Inventory-based message propagation
//!
//! ### Message Types
//!
//! Supported network message types:
//! - **version**: Version and capability negotiation
//! - **verack**: Version acknowledgment
//! - **inv**: Inventory announcement
//! - **getdata**: Data request
//! - **block**: Block data
//! - **tx**: Transaction data
//! - **ping/pong**: Keep-alive messages
//!
//! ## Performance Features
//!
//! - **Connection Pooling**: Efficient connection reuse and management
//! - **Message Batching**: Batch message processing for better throughput
//! - **Compression**: Optional message compression for bandwidth optimization
//! - **Rate Limiting**: Protection against spam and DoS attacks
//! - **Caching**: Smart caching for frequently accessed data

//#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod composite_handler;
/// Composite message handler for protocol processing
pub mod compression;
/// DOS protection and rate limiting
pub mod dos_protection;
/// Enhanced DoS protection with adaptive rate limiting
pub mod enhanced_dos_protection;
/// Network error types and result handling
pub mod error;
/// Advanced error handling and recovery strategies
pub mod error_handling;
/// Protocol message handlers
pub mod handlers;
/// Network message types and serialization
pub mod messages;
/// Core P2P protocol implementation
pub mod p2p;
/// High-level P2P node interface
pub mod p2p_node;
/// Peer connection management
pub mod peer_manager;
/// Peer discovery and routing
pub mod peers;
/// Transaction and inventory relay cache
pub mod relay_cache;
/// Network resilience patterns
pub mod resilience;
/// JSON-RPC server implementation
pub mod rpc;
/// Safe P2P networking utilities
pub mod safe_p2p;
/// Network server coordination
pub mod server;
/// Graceful shutdown implementation
pub mod shutdown_impl;
/// Snapshot configuration management
pub mod snapshot_config;
/// Blockchain synchronization
pub mod sync;
/// Transaction relay and propagation
pub mod transaction_relay;

// Constants
const UNKNOWN_PEER_ADDR: &str = "0.0.0.0:0";
const DEFAULT_MAINNET_PORT: &str = "10333";
const DEFAULT_PRIVNET_PORT: &str = "30333";
const DEFAULT_WS_PORT: &str = "10334";
const LOCALHOST: &str = "127.0.0.1";

// Re-export main types
pub use crate::error_handling::{
    ErrorStatistics, NetworkErrorEvent, NetworkErrorHandler, OperationContext, RecoveryStrategy,
};
pub use crate::handlers::TransactionMessageHandler;
pub use crate::messages::{
    InventoryItem, InventoryType, MessageCommand, MessageValidator, Neo3Message, NetworkMessage,
    ProtocolMessage,
};
pub use crate::p2p_node::{
    NodeCapability, NodeEvent, NodeStatistics, NodeStatus, P2pNode, PeerInfo,
};
pub use crate::peer_manager::{ConnectionStats, PeerConnection, PeerEvent, PeerManager, PeerState};
pub use crate::relay_cache::RelayCache;
pub use crate::transaction_relay::{
    RelayStatistics, TransactionRelay, TransactionRelayConfig, TransactionRelayEvent,
};
pub use error::{ErrorSeverity, NetworkError, NetworkResult, Result};

/// Node service flags indicating node capabilities.
///
/// These flags indicate what services a node provides to the network.
/// They are used during the version handshake to advertise node capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
/// Represents an enumeration of values.
pub enum NodeServices {
    /// Full node service (able to serve the network)
    NodeNetwork = 0x01,
    /// Able to serve GetBlocks requests
    NodeGetBlocks = 0x02,
    /// Able to serve GetTransactions requests
    NodeGetTransactions = 0x04,
}

/// Bloom filter update flags for transaction filtering.
///
/// These flags control how bloom filters are updated when transactions
/// are processed, enabling light clients to filter relevant transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Represents an enumeration of values.
pub enum BloomFilterFlags {
    UpdateNone = 0x00,
    UpdateAll = 0x01,
    UpdateP2PUB = 0x02,
}

pub type P2PEvent = NodeEvent;
pub type P2PNode = P2pNode;

// Global sync manager reference for direct peer height updates
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static GLOBAL_SYNC_MANAGER: Lazy<Mutex<Option<std::sync::Arc<sync::SyncManager>>>> =
    Lazy::new(|| Mutex::new(None));

/// Sets the global sync manager reference.
///
/// This function allows setting a global reference to the sync manager
/// for use by other components that need direct access to sync state.
///
/// # Arguments
///
/// * `sync_manager` - Arc reference to the sync manager instance
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from any thread.
/// Sets a value in the internal state.
pub fn set_global_sync_manager(sync_manager: std::sync::Arc<sync::SyncManager>) {
    if let Ok(mut guard) = GLOBAL_SYNC_MANAGER.lock() {
        *guard = Some(sync_manager);
    }
}
pub type RpcServer = crate::rpc::RpcServer;
pub type SyncManager = crate::sync::SyncManager;
pub type SyncEvent = crate::sync::SyncEvent;

// Configuration types are already defined in this file

use neo_config::{DEFAULT_RPC_PORT, DEFAULT_TESTNET_PORT, MAINNET_SEEDS, N3_TESTNET_SEEDS};
use neo_core::{UInt160, UInt256};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;
use thiserror::Error;

/// Default Neo network ports
/// Legacy error type for backward compatibility.
///
/// This type provides compatibility with older code that used the previous
/// error system. New code should use [`NetworkError`] instead.
///
/// **Deprecated**: Use [`NetworkError`] instead for new code.
#[deprecated(since = "0.3.0", note = "Use NetworkError instead")]
pub use LegacyError as Error;

/// Legacy network error types for backward compatibility.
///
/// This enum provides the error types used in previous versions of the
/// network crate. It includes comprehensive error variants for all
/// network-related operations.
#[derive(Error, Debug)]
/// Represents an enumeration of values.
pub enum LegacyError {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Peer error
    #[error("Peer error: {0}")]
    Peer(String),

    /// Synchronization error
    #[error("Synchronization error: {0}")]
    Sync(String),

    /// RPC error
    #[error("RPC error: {0}")]
    Rpc(String),

    /// Timeout error
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Rate limiting error
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Invalid message error
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Invalid header error
    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    /// Network configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Peer already connected error
    #[error("Peer already connected: {0}")]
    PeerAlreadyConnected(String),

    /// Connection limit reached error
    #[error("Connection limit reached")]
    ConnectionLimitReached,

    /// Connection failed error
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Connection timeout error
    #[error("Connection timeout")]
    ConnectionTimeout,

    /// Peer not connected error
    #[error("Peer not connected: {0}")]
    PeerNotConnected(String),

    /// Message send failed error
    #[error("Message send failed: {0}")]
    MessageSendFailed(String),

    /// Handshake failed error
    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),

    /// Handshake timeout error
    #[error("Handshake timeout")]
    HandshakeTimeout,

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Ledger error
    #[error("Ledger error: {0}")]
    Ledger(String),

    /// Generic error
    #[error("Network error: {0}")]
    Generic(String),
}

// Legacy error conversions
impl From<neo_ledger::Error> for LegacyError {
    fn from(err: neo_ledger::Error) -> Self {
        LegacyError::Ledger(err.to_string())
    }
}

impl From<neo_io::Error> for LegacyError {
    fn from(err: neo_io::Error) -> Self {
        LegacyError::Serialization(err.to_string())
    }
}

impl From<neo_core::CoreError> for LegacyError {
    fn from(err: neo_core::CoreError) -> Self {
        LegacyError::Protocol(err.to_string())
    }
}

/// Network protocol version.
///
/// Represents the version of the Neo network protocol. Protocol versions
/// are used during handshaking to ensure compatibility between nodes.
///
/// # Protocol Compatibility
///
/// Nodes with different protocol versions can connect if they share
/// the same major version and the connecting node has a minor version
/// greater than or equal to the target node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Represents a data structure.
pub struct ProtocolVersion {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
}

impl ProtocolVersion {
    /// Creates a new protocol version.
    ///
    /// # Arguments
    ///
    /// * `major` - Major version number
    /// * `minor` - Minor version number  
    /// * `patch` - Patch version number
    ///
    /// # Returns
    ///
    /// A new `ProtocolVersion` instance.
    /// Creates a new instance.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the current Neo protocol version.
    ///
    /// Neo N3 uses protocol version 0 consistently across all implementations.
    /// This is the official Neo N3 protocol version number.
    ///
    /// # Returns
    ///
    /// The current protocol version used by this implementation.
    pub fn current() -> Self {
        // Neo N3 uses protocol version 0 (zero) consistently across all implementations
        // This is different from semantic versioning - it's just a single protocol version number
        Self::from_u32(0) // Official Neo N3 protocol version
    }

    /// Checks if this version is compatible with another protocol version.
    ///
    /// Two protocol versions are compatible if they have the same major
    /// version and this version's minor version is greater than or equal
    /// to the other version's minor version.
    ///
    /// # Arguments
    ///
    /// * `other` - The protocol version to check compatibility with
    ///
    /// # Returns
    ///
    /// `true` if the versions are compatible, `false` otherwise.
    /// Checks a boolean condition.
    pub fn is_compatible(&self, other: &ProtocolVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }

    /// Converts the protocol version to a single u32 value.
    ///
    /// This method packs the version components into a single u32 for
    /// network protocol compatibility.
    ///
    /// # Returns
    ///
    /// A u32 value representing the packed version.
    pub fn as_u32(&self) -> u32 {
        ((self.major & 0xFF) << 24) | ((self.minor & 0xFF) << 16) | (self.patch & 0xFFFF)
    }

    /// Creates a protocol version from a u32 value.
    ///
    /// This method unpacks a u32 value into version components.
    ///
    /// # Arguments
    ///
    /// * `value` - The packed version value
    ///
    /// # Returns
    ///
    /// A new `ProtocolVersion` instance.
    pub fn from_u32(value: u32) -> Self {
        Self {
            major: (value >> 24) & 0xFF,
            minor: (value >> 16) & 0xFF,
            patch: value & 0xFFFF,
        }
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::current()
    }
}

/// Network node information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Represents a data structure.
pub struct NodeInfo {
    /// Node ID
    pub id: UInt160,
    /// Node version
    pub version: ProtocolVersion,
    /// Node user agent
    pub user_agent: String,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Node start height
    pub start_height: u32,
    /// Node timestamp
    pub timestamp: u64,
    /// Node nonce
    pub nonce: u32,
}

impl NodeInfo {
    /// Creates a new node info
    /// Creates a new instance.
    pub fn new(id: UInt160, start_height: u32) -> Self {
        Self {
            id,
            version: ProtocolVersion::current(),
            user_agent: "/Neo:3.0.0/".to_string(),
            capabilities: vec![
                "FullNode".to_string(),
                "TcpServer".to_string(),
                "WsServer".to_string(),
            ],
            start_height,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Operation failed")
                .as_secs(),
            nonce: rand::random(),
        }
    }
}

/// Network statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents a data structure.
pub struct NetworkStats {
    /// Number of connected peers
    pub peer_count: usize,
    /// Number of inbound connections
    pub inbound_connections: usize,
    /// Number of outbound connections
    pub outbound_connections: usize,
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Messages sent per second
    pub messages_sent_per_sec: f64,
    /// Messages received per second
    pub messages_received_per_sec: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Sync status
    pub sync_status: String,
    /// Current block height
    pub current_height: u32,
    /// Best known height
    pub best_known_height: u32,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            peer_count: 0,
            inbound_connections: 0,
            outbound_connections: 0,
            bytes_sent: 0,
            bytes_received: 0,
            messages_sent_per_sec: 0.0,
            messages_received_per_sec: 0.0,
            average_latency_ms: 0.0,
            sync_status: "Disconnected".to_string(),
            current_height: 0,
            best_known_height: 0,
        }
    }
}

/// Network command types for external control
#[derive(Debug, Clone)]
/// Represents an enumeration of values.
pub enum NetworkCommand {
    /// Connect to a specific peer
    ConnectToPeer(SocketAddr),
    /// Disconnect from a specific peer
    DisconnectPeer(SocketAddr),
    /// Send a message to a specific peer
    SendMessage {
        peer: SocketAddr,
        message: NetworkMessage,
    },
    /// Broadcast a message to all peers
    BroadcastMessage(NetworkMessage),
    /// Stop the network service
    Stop,
}

/// Message handler for protocol processing
/// Represents a data structure.
pub struct MessageHandler {
    config: NetworkConfig,
}

impl MessageHandler {
    /// Creates a new instance.
    pub fn new(config: NetworkConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

/// Network event types
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an enumeration of values.
pub enum NetworkEvent {
    /// Peer connected
    PeerConnected {
        peer_id: UInt160,
        address: SocketAddr,
    },
    /// Peer disconnected
    PeerDisconnected { peer_id: UInt160, reason: String },
    /// Message received
    MessageReceived {
        peer_id: UInt160,
        message_type: String,
    },
    /// Block received
    BlockReceived { block_hash: UInt256, height: u32 },
    /// Transaction received
    TransactionReceived { tx_hash: UInt256 },
    /// Sync started
    SyncStarted { target_height: u32 },
    /// Sync completed
    SyncCompleted { final_height: u32 },
    /// Sync failed
    SyncFailed { error: String },
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct NetworkConfig {
    /// Network magic number
    pub magic: u32,
    /// Protocol version
    pub protocol_version: ProtocolVersion,
    /// Node user agent
    pub user_agent: String,
    /// Listen address
    pub listen_address: SocketAddr,
    /// P2P configuration
    pub p2p_config: P2PConfig,
    /// RPC configuration
    pub rpc_config: Option<RpcConfig>,
    /// Maximum number of peers
    pub max_peers: usize,
    /// Maximum outbound connections
    pub max_outbound_connections: usize,
    /// Maximum inbound connections
    pub max_inbound_connections: usize,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Handshake timeout in seconds
    pub handshake_timeout: u64,
    /// Ping interval in seconds
    pub ping_interval: u64,
    /// Enable transaction relay
    pub enable_relay: bool,
    /// Seed nodes for peer discovery
    pub seed_nodes: Vec<SocketAddr>,
    /// P2P port
    pub port: u16,
    /// WebSocket enabled
    pub websocket_enabled: bool,
    /// WebSocket port
    pub websocket_port: u16,
}

/// P2P configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct P2PConfig {
    pub listen_address: SocketAddr,
    pub max_peers: usize,
    pub connection_timeout: std::time::Duration,
    pub handshake_timeout: std::time::Duration,
    pub ping_interval: std::time::Duration,
    pub message_buffer_size: usize,
    pub enable_compression: bool,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            listen_address: format!("{}:{}", LOCALHOST, DEFAULT_MAINNET_PORT)
                .parse()
                .expect("value should parse"),
            max_peers: 100,
            connection_timeout: std::time::Duration::from_secs(30),
            handshake_timeout: std::time::Duration::from_secs(10),
            ping_interval: std::time::Duration::from_secs(30),
            message_buffer_size: 1000,
            enable_compression: false,
        }
    }
}

/// RPC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct RpcConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub port: u16,
    pub max_connections: usize,
    pub http_address: Option<SocketAddr>,
    pub ws_address: Option<SocketAddr>,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: "localhost".to_string(),
            port: 10332,
            max_connections: 100,
            http_address: Some(
                format!("{}:{}", LOCALHOST, DEFAULT_RPC_PORT)
                    .parse()
                    .expect("value should parse"),
            ),
            ws_address: Some(
                format!("{}:{}", LOCALHOST, DEFAULT_WS_PORT)
                    .parse()
                    .expect("value should parse"),
            ),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            magic: 0x334F454E, // Neo N3 mainnet magic (860833102 in decimal)
            protocol_version: ProtocolVersion::current(),
            user_agent: "/Neo:3.0.0/".to_string(),
            listen_address: format!("{}:{}", LOCALHOST, DEFAULT_MAINNET_PORT)
                .parse()
                .expect("value should parse"),
            p2p_config: P2PConfig::default(),
            rpc_config: Some(RpcConfig::default()),
            max_peers: 100,
            max_outbound_connections: 10,
            max_inbound_connections: 40,
            connection_timeout: 30,
            handshake_timeout: 10,
            ping_interval: 30,
            enable_relay: true,
            seed_nodes: MAINNET_SEEDS
                .iter()
                .map(|s| {
                    s.parse()
                        .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse"))
                })
                .collect(),
            port: 10333,
            websocket_enabled: false,
            websocket_port: 10334,
        }
    }
}

impl NetworkConfig {
    /// Gets connection timeout in seconds
    pub fn connection_timeout_secs(&self) -> u64 {
        self.connection_timeout
    }
    /// Creates a testnet configuration
    pub fn testnet() -> Self {
        Self {
            magic: 0x3554334E, // Neo N3 testnet magic (894448462 in decimal)
            listen_address: format!("{}:{}", LOCALHOST, DEFAULT_TESTNET_PORT)
                .parse()
                .expect("value should parse"),
            p2p_config: P2PConfig {
                listen_address: format!("{}:{}", LOCALHOST, DEFAULT_TESTNET_PORT)
                    .parse()
                    .expect("value should parse"),
                max_peers: 100,
                connection_timeout: std::time::Duration::from_secs(30),
                handshake_timeout: std::time::Duration::from_secs(10),
                ping_interval: std::time::Duration::from_secs(30),
                message_buffer_size: 1000,
                enable_compression: false,
            },
            seed_nodes: N3_TESTNET_SEEDS
                .iter()
                .map(|s| {
                    s.parse()
                        .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse"))
                })
                .collect(),
            port: 20333,
            websocket_enabled: false,
            websocket_port: 20334,
            ..Default::default()
        }
    }

    /// Creates a private network configuration
    pub fn private() -> Self {
        Self {
            magic: 0x12345678, // Custom magic for private network
            listen_address: format!("{}:{}", LOCALHOST, DEFAULT_PRIVNET_PORT)
                .parse()
                .expect("value should parse"),
            p2p_config: P2PConfig {
                listen_address: format!("{}:{}", LOCALHOST, DEFAULT_PRIVNET_PORT)
                    .parse()
                    .expect("value should parse"),
                max_peers: 10,
                connection_timeout: std::time::Duration::from_secs(30),
                handshake_timeout: std::time::Duration::from_secs(10),
                ping_interval: std::time::Duration::from_secs(30),
                message_buffer_size: 1000,
                enable_compression: false,
            },
            seed_nodes: vec![], // No seed nodes for private network
            max_peers: 10,
            ..Default::default()
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{NetworkError, NetworkStats, PeerInfo};
    use crate::{NetworkConfig, NodeInfo, ProtocolVersion};
    use neo_core::UInt160;

    #[test]
    fn test_protocol_version() {
        let v1 = ProtocolVersion::new(3, 6, 0);
        let v2 = ProtocolVersion::new(3, 5, 0);
        let v3 = ProtocolVersion::new(2, 6, 0);

        assert!(v1.is_compatible(&v2));
        assert!(!v1.is_compatible(&v3));
        assert_eq!(v1.to_string(), "3.6.0");
    }

    #[test]
    fn test_node_info() {
        let node_id = UInt160::zero();
        let info = NodeInfo::new(node_id, 100);

        assert_eq!(info.id, node_id);
        assert_eq!(info.start_height, 100);
        assert_eq!(info.version, ProtocolVersion::current());
        assert!(!info.capabilities.is_empty());
    }

    #[test]
    fn test_network_config() {
        let config = NetworkConfig::default();
        assert_eq!(config.magic, 0x334f454e);
        assert_eq!(config.max_peers, 100);
        assert!(!config.seed_nodes.is_empty());

        let testnet_config = NetworkConfig::testnet();
        assert_eq!(testnet_config.magic, 0x3554334e);

        let private_config = NetworkConfig::private();
        assert_eq!(private_config.magic, 0x12345678);
        assert!(private_config.seed_nodes.is_empty());
    }

    #[test]
    fn test_network_stats() {
        let stats = NetworkStats::default();
        assert_eq!(stats.peer_count, 0);
        assert_eq!(stats.current_height, 0);
        assert_eq!(stats.sync_status, "Disconnected");
    }
}

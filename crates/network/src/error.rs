//! Error types for the Neo Network crate
//!
//! This module provides comprehensive error handling for network operations,
//! including P2P communication, message handling, peer management, and RPC services.

// Default timeout for network operations (30 seconds)
const DEFAULT_TIMEOUT_MS: u64 = 30000;
use std::net::SocketAddr;
use thiserror::Error;

/// Default socket address for unknown peers
const UNKNOWN_PEER_ADDR: &str = "0.0.0.0:0";
/// Network operation errors
#[derive(Error, Debug, Clone, PartialEq)]
/// Represents an enumeration of values.
pub enum NetworkError {
    /// Connection establishment failed
    #[error("Connection failed to {address}: {reason}")]
    ConnectionFailed { address: SocketAddr, reason: String },

    /// Connection timeout
    #[error("Connection timeout to {address} after {timeout_ms}ms")]
    ConnectionTimeout {
        address: SocketAddr,
        timeout_ms: u64,
    },

    /// Connection limit reached
    #[error("Connection limit reached: {current}/{max}")]
    ConnectionLimitReached { current: usize, max: usize },

    /// Peer already connected
    #[error("Peer already connected: {address}")]
    PeerAlreadyConnected { address: SocketAddr },

    /// Peer not connected
    #[error("Peer not connected: {address}")]
    PeerNotConnected { address: SocketAddr },

    /// Peer is banned
    #[error("Peer is banned: {address}")]
    PeerBanned { address: SocketAddr },

    /// Invalid connection
    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    /// Handshake failed
    #[error("Handshake failed: {0}")]
    HandshakeError(String),

    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),

    /// Protocol violation
    #[error("Protocol violation from {peer}: {violation}")]
    ProtocolViolation { peer: SocketAddr, violation: String },

    /// Invalid protocol version
    #[error("Invalid protocol version: expected {expected}, got {actual}")]
    InvalidProtocolVersion { expected: String, actual: String },

    /// Handshake failed
    #[error("Handshake failed with {peer}: {reason}")]
    HandshakeFailed { peer: SocketAddr, reason: String },

    /// Handshake timeout
    #[error("Handshake timeout with {peer} after {timeout_ms}ms")]
    HandshakeTimeout { peer: SocketAddr, timeout_ms: u64 },

    /// Authentication failed
    #[error("Authentication failed for {peer}: {reason}")]
    AuthenticationFailed { peer: SocketAddr, reason: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded for {peer}: {current_rate} messages/sec > {limit}")]
    RateLimitExceeded {
        peer: SocketAddr,
        current_rate: f64,
        limit: f64,
    },

    /// Message serialization failed
    #[error("Message serialization failed: {message_type}, reason: {reason}")]
    MessageSerialization {
        message_type: String,
        reason: String,
    },

    /// Message deserialization failed
    #[error("Message deserialization failed: {data_size} bytes, reason: {reason}")]
    MessageDeserialization { data_size: usize, reason: String },

    /// Invalid message received
    #[error("Invalid message from {peer}: {message_type}, reason: {reason}")]
    InvalidMessage {
        peer: SocketAddr,
        message_type: String,
        reason: String,
    },

    /// Invalid message header
    #[error("Invalid message header from {peer}: {reason}")]
    InvalidHeader { peer: SocketAddr, reason: String },

    /// Message send failed
    #[error("Message send failed to {peer}: {message_type}, reason: {reason}")]
    MessageSendFailed {
        peer: SocketAddr,
        message_type: String,
        reason: String,
    },

    /// Message too large
    #[error("Message too large: {size} bytes > {max_size} bytes")]
    MessageTooLarge { size: usize, max_size: usize },

    /// Synchronization failed
    #[error("Synchronization failed: {reason}")]
    SyncFailed { reason: String },

    /// Sync timeout
    #[error("Synchronization timeout after {timeout_ms}ms")]
    SyncTimeout { timeout_ms: u64 },

    /// Block validation failed
    #[error("Block validation failed: block {block_hash}, reason: {reason}")]
    BlockValidationFailed { block_hash: String, reason: String },

    /// Transaction validation failed
    #[error("Transaction validation failed: tx {tx_hash}, reason: {reason}")]
    TransactionValidationFailed { tx_hash: String, reason: String },

    /// RPC error
    #[error("RPC error: method {method}, code {code}, message: {message}")]
    Rpc {
        method: String,
        code: i32,
        message: String,
    },

    /// RPC timeout
    #[error("RPC timeout: method {method} after {timeout_ms}ms")]
    RpcTimeout { method: String, timeout_ms: u64 },

    /// Invalid RPC request
    #[error("Invalid RPC request: {reason}")]
    InvalidRpcRequest { reason: String },

    /// RPC method not found
    #[error("RPC method not found: {method}")]
    RpcMethodNotFound { method: String },

    /// I/O operation failed
    #[error("I/O error: {operation}, reason: {reason}")]
    Io { operation: String, reason: String },

    /// JSON processing failed
    #[error("JSON error: {reason}")]
    Json { reason: String },

    /// Network configuration error
    #[error("Configuration error: {parameter}, reason: {reason}")]
    Configuration { parameter: String, reason: String },

    /// DNS resolution failed
    #[error("DNS resolution failed for {hostname}: {reason}")]
    DnsResolution { hostname: String, reason: String },

    /// Address binding failed
    #[error("Address binding failed: {address}, reason: {reason}")]
    AddressBinding { address: SocketAddr, reason: String },

    /// Network unreachable
    #[error("Network unreachable: {address}")]
    NetworkUnreachable { address: SocketAddr },

    /// Connection refused
    #[error("Connection refused by {address}")]
    ConnectionRefused { address: SocketAddr },

    /// Host unreachable
    #[error("Host unreachable: {address}")]
    HostUnreachable { address: SocketAddr },

    /// Buffer overflow
    #[error("Buffer overflow: attempted to write {size} bytes to {capacity} byte buffer")]
    BufferOverflow { size: usize, capacity: usize },

    /// Invalid network magic
    #[error("Invalid network magic: expected {expected:#010x}, got {actual:#010x}")]
    InvalidMagic { expected: u32, actual: u32 },

    /// Blockchain inconsistency
    #[error("Blockchain inconsistency: {reason}")]
    BlockchainInconsistency { reason: String },

    /// Resource exhausted
    #[error("Resource exhausted: {resource}, used: {used}, limit: {limit}")]
    ResourceExhausted {
        resource: String,
        used: u64,
        limit: u64,
    },

    /// Service unavailable
    #[error("Service unavailable: {service}, reason: {reason}")]
    ServiceUnavailable { service: String, reason: String },

    /// Transaction validation error
    #[error("Transaction validation failed: hash {hash}, reason: {reason}")]
    TransactionValidation {
        hash: neo_core::UInt256,
        reason: String,
    },

    /// Message send failed
    #[error("Failed to send message to peer {peer}: {reason}")]
    MessageSend { peer: SocketAddr, reason: String },

    /// Ledger error
    #[error("Ledger error: {reason}")]
    Ledger { reason: String },

    /// Server error
    #[error("Server error: {0}")]
    ServerError(String),

    /// Circuit breaker is open
    #[error("Circuit breaker open: {reason}")]
    CircuitBreakerOpen { reason: String },

    /// Temporary failure that may succeed if retried
    #[error("Temporary failure: {reason}")]
    TemporaryFailure { reason: String },

    /// Operation is queued for execution
    #[error("Operation queued: {reason}")]
    Queued { reason: String },

    /// Generic network error
    #[error("Network error: {reason}")]
    Generic { reason: String },
}

impl NetworkError {
    /// Create a new connection failed error
    pub fn connection_failed<S: Into<String>>(address: SocketAddr, reason: S) -> Self {
        Self::ConnectionFailed {
            address,
            reason: reason.into(),
        }
    }

    /// Create a new connection timeout error
    pub fn connection_timeout(address: SocketAddr, timeout_ms: u64) -> Self {
        Self::ConnectionTimeout {
            address,
            timeout_ms,
        }
    }

    /// Create a new connection limit reached error
    pub fn connection_limit_reached(current: usize, max: usize) -> Self {
        Self::ConnectionLimitReached { current, max }
    }

    /// Create a new peer already connected error
    pub fn peer_already_connected(address: SocketAddr) -> Self {
        Self::PeerAlreadyConnected { address }
    }

    /// Create a new peer not connected error
    pub fn peer_not_connected(address: SocketAddr) -> Self {
        Self::PeerNotConnected { address }
    }

    /// Create a new protocol violation error
    pub fn protocol_violation<S: Into<String>>(peer: SocketAddr, violation: S) -> Self {
        Self::ProtocolViolation {
            peer,
            violation: violation.into(),
        }
    }

    /// Create a new handshake failed error
    pub fn handshake_failed<S: Into<String>>(peer: SocketAddr, reason: S) -> Self {
        Self::HandshakeFailed {
            peer,
            reason: reason.into(),
        }
    }

    /// Create a new authentication failed error
    pub fn authentication_failed<S: Into<String>>(peer: SocketAddr, reason: S) -> Self {
        Self::AuthenticationFailed {
            peer,
            reason: reason.into(),
        }
    }

    /// Create a new rate limit exceeded error
    pub fn rate_limit_exceeded(peer: SocketAddr, current_rate: f64, limit: f64) -> Self {
        Self::RateLimitExceeded {
            peer,
            current_rate,
            limit,
        }
    }

    /// Create a new message serialization error
    pub fn message_serialization<S: Into<String>>(message_type: S, reason: S) -> Self {
        Self::MessageSerialization {
            message_type: message_type.into(),
            reason: reason.into(),
        }
    }

    /// Create a new message deserialization error
    pub fn message_deserialization<S: Into<String>>(data_size: usize, reason: S) -> Self {
        Self::MessageDeserialization {
            data_size,
            reason: reason.into(),
        }
    }

    /// Create a new invalid message error
    pub fn invalid_message<S: Into<String>>(peer: SocketAddr, message_type: S, reason: S) -> Self {
        Self::InvalidMessage {
            peer,
            message_type: message_type.into(),
            reason: reason.into(),
        }
    }

    /// Create a new RPC error
    pub fn rpc<S: Into<String>>(method: S, code: i32, message: S) -> Self {
        Self::Rpc {
            method: method.into(),
            code,
            message: message.into(),
        }
    }

    /// Create a new I/O error
    pub fn io<S: Into<String>>(operation: S, reason: S) -> Self {
        Self::Io {
            operation: operation.into(),
            reason: reason.into(),
        }
    }

    /// Create a new configuration error
    pub fn configuration<S: Into<String>>(parameter: S, reason: S) -> Self {
        Self::Configuration {
            parameter: parameter.into(),
            reason: reason.into(),
        }
    }

    /// Create a new generic error
    pub fn generic<S: Into<String>>(reason: S) -> Self {
        Self::Generic {
            reason: reason.into(),
        }
    }

    /// Check if this error is retryable
    /// Checks a boolean condition.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            NetworkError::ConnectionTimeout { .. }
                | NetworkError::HandshakeTimeout { .. }
                | NetworkError::MessageSendFailed { .. }
                | NetworkError::SyncTimeout { .. }
                | NetworkError::RpcTimeout { .. }
                | NetworkError::Io { .. }
                | NetworkError::DnsResolution { .. }
                | NetworkError::NetworkUnreachable { .. }
                | NetworkError::HostUnreachable { .. }
                | NetworkError::ServiceUnavailable { .. }
                | NetworkError::TemporaryFailure { .. }
        )
    }

    /// Check if this error is a connection-related error
    /// Checks a boolean condition.
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            NetworkError::ConnectionFailed { .. }
                | NetworkError::ConnectionTimeout { .. }
                | NetworkError::ConnectionLimitReached { .. }
                | NetworkError::ConnectionRefused { .. }
                | NetworkError::NetworkUnreachable { .. }
                | NetworkError::HostUnreachable { .. }
        )
    }

    /// Check if this error is a protocol-related error
    /// Checks a boolean condition.
    pub fn is_protocol_error(&self) -> bool {
        matches!(
            self,
            NetworkError::ProtocolViolation { .. }
                | NetworkError::InvalidProtocolVersion { .. }
                | NetworkError::HandshakeFailed { .. }
                | NetworkError::InvalidMessage { .. }
                | NetworkError::InvalidHeader { .. }
                | NetworkError::InvalidMagic { .. }
        )
    }

    /// Check if this error is a user error (vs system error)
    /// Checks a boolean condition.
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            NetworkError::InvalidRpcRequest { .. }
                | NetworkError::RpcMethodNotFound { .. }
                | NetworkError::Configuration { .. }
                | NetworkError::InvalidMagic { .. }
                | NetworkError::MessageTooLarge { .. }
        )
    }

    /// Check if this error should cause a peer to be banned
    pub fn should_ban_peer(&self) -> bool {
        matches!(
            self,
            NetworkError::ProtocolViolation { .. }
                | NetworkError::AuthenticationFailed { .. }
                | NetworkError::RateLimitExceeded { .. }
                | NetworkError::InvalidMessage { .. }
                | NetworkError::InvalidHeader { .. }
        )
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            NetworkError::ConnectionFailed { .. }
            | NetworkError::ConnectionTimeout { .. }
            | NetworkError::PeerNotConnected { .. } => ErrorSeverity::Low,

            NetworkError::HandshakeFailed { .. }
            | NetworkError::MessageSendFailed { .. }
            | NetworkError::SyncFailed { .. }
            | NetworkError::Rpc { .. } => ErrorSeverity::Medium,

            NetworkError::ProtocolViolation { .. }
            | NetworkError::AuthenticationFailed { .. }
            | NetworkError::BlockchainInconsistency { .. }
            | NetworkError::ResourceExhausted { .. } => ErrorSeverity::High,

            NetworkError::Configuration { .. }
            | NetworkError::AddressBinding { .. }
            | NetworkError::ServiceUnavailable { .. } => ErrorSeverity::Critical,

            NetworkError::CircuitBreakerOpen { .. } => ErrorSeverity::High,
            NetworkError::TemporaryFailure { .. } => ErrorSeverity::Low,
            NetworkError::Queued { .. } => ErrorSeverity::Low,

            _ => ErrorSeverity::Medium,
        }
    }

    /// Get error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            NetworkError::ConnectionFailed { .. }
            | NetworkError::ConnectionTimeout { .. }
            | NetworkError::ConnectionLimitReached { .. }
            | NetworkError::PeerAlreadyConnected { .. }
            | NetworkError::PeerNotConnected { .. }
            | NetworkError::ConnectionRefused { .. }
            | NetworkError::InvalidConnection(..) => "connection",

            NetworkError::ProtocolViolation { .. }
            | NetworkError::InvalidProtocolVersion { .. }
            | NetworkError::HandshakeFailed { .. }
            | NetworkError::HandshakeTimeout { .. }
            | NetworkError::InvalidMessage { .. }
            | NetworkError::InvalidHeader { .. }
            | NetworkError::InvalidMagic { .. } => "protocol",

            NetworkError::AuthenticationFailed { .. } => "authentication",

            NetworkError::RateLimitExceeded { .. } => "rate_limit",

            NetworkError::MessageSerialization { .. }
            | NetworkError::MessageDeserialization { .. }
            | NetworkError::MessageSendFailed { .. }
            | NetworkError::MessageTooLarge { .. } => "message",

            NetworkError::SyncFailed { .. }
            | NetworkError::SyncTimeout { .. }
            | NetworkError::BlockValidationFailed { .. }
            | NetworkError::TransactionValidationFailed { .. } => "sync",

            NetworkError::Rpc { .. }
            | NetworkError::RpcTimeout { .. }
            | NetworkError::InvalidRpcRequest { .. }
            | NetworkError::RpcMethodNotFound { .. } => "rpc",

            NetworkError::Io { .. } => "io",
            NetworkError::Json { .. } => "json",
            NetworkError::Configuration { .. } => "configuration",
            NetworkError::DnsResolution { .. } => "dns",
            NetworkError::AddressBinding { .. } => "binding",
            NetworkError::NetworkUnreachable { .. } | NetworkError::HostUnreachable { .. } => {
                "reachability"
            }
            NetworkError::BufferOverflow { .. } => "buffer",
            NetworkError::BlockchainInconsistency { .. } => "blockchain",
            NetworkError::ResourceExhausted { .. } => "resource",
            NetworkError::ServiceUnavailable { .. } => "service",
            NetworkError::Ledger { .. } => "ledger",
            NetworkError::ServerError(_) => "server",
            NetworkError::Generic { .. } => "generic",
            NetworkError::InternalError(..) => "internal",
            NetworkError::PeerBanned { .. } => "peer",
            NetworkError::TransactionValidation { .. } => "transaction",
            NetworkError::MessageSend { .. } => "messaging",
            NetworkError::CircuitBreakerOpen { .. } => "resilience",
            NetworkError::TemporaryFailure { .. } => "resilience",
            NetworkError::Queued { .. } => "resilience",
            NetworkError::HandshakeError(_) => "handshake",
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents an enumeration of values.
pub enum ErrorSeverity {
    /// Low severity - minor issues that don't affect functionality
    Low,
    /// Medium severity - issues that may affect performance or specific features
    Medium,
    /// High severity - serious issues that significantly impact functionality
    High,
    /// Critical severity - issues that prevent normal operation
    Critical,
}

/// Result type for network operations
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;

/// Alias for compatibility with existing code
pub type Result<T> = NetworkResult<T>;

// Standard library error conversions
impl From<std::io::Error> for NetworkError {
    fn from(error: std::io::Error) -> Self {
        NetworkError::io("io_operation", &error.to_string())
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(error: serde_json::Error) -> Self {
        NetworkError::Json {
            reason: error.to_string(),
        }
    }
}

impl From<std::net::AddrParseError> for NetworkError {
    fn from(error: std::net::AddrParseError) -> Self {
        NetworkError::configuration("address", &error.to_string())
    }
}

// Neo-specific error conversions
impl From<neo_core::CoreError> for NetworkError {
    fn from(error: neo_core::CoreError) -> Self {
        NetworkError::generic(error.to_string())
    }
}

impl From<neo_io::IoError> for NetworkError {
    fn from(error: neo_io::IoError) -> Self {
        match error {
            neo_io::IoError::EndOfStream { .. } => {
                NetworkError::message_deserialization(0, "unexpected end of stream")
            }
            neo_io::IoError::InvalidData { value, .. } => {
                NetworkError::message_deserialization(0, &value)
            }
            neo_io::IoError::Serialization { reason, .. } => {
                NetworkError::message_serialization("unknown", &reason)
            }
            neo_io::IoError::Deserialization { reason, .. } => {
                NetworkError::message_deserialization(0, &reason)
            }
            _ => NetworkError::io("neo_io", &error.to_string()),
        }
    }
}

impl From<neo_ledger::Error> for NetworkError {
    fn from(error: neo_ledger::Error) -> Self {
        NetworkError::Ledger {
            reason: error.to_string(),
        }
    }
}

// Backward compatibility with old Error type
impl From<NetworkError> for crate::Error {
    fn from(error: NetworkError) -> Self {
        match error {
            NetworkError::ConnectionFailed { reason, .. } => crate::Error::Connection(reason),
            NetworkError::ProtocolViolation { violation, .. } => crate::Error::Protocol(violation),
            NetworkError::MessageSerialization { reason, .. } => {
                crate::Error::Serialization(reason)
            }
            NetworkError::PeerNotConnected { address } => {
                crate::Error::Peer(format!("Peer not connected: {}", address))
            }
            NetworkError::SyncFailed { reason } => crate::Error::Sync(reason),
            NetworkError::Rpc { message, .. } => crate::Error::Rpc(message),
            NetworkError::ConnectionTimeout { .. } => {
                crate::Error::Timeout("Connection timeout".to_string())
            }
            NetworkError::AuthenticationFailed { reason, .. } => {
                crate::Error::Authentication(reason)
            }
            NetworkError::RateLimitExceeded { .. } => {
                crate::Error::RateLimit("Rate limit exceeded".to_string())
            }
            NetworkError::InvalidMessage { reason, .. } => crate::Error::InvalidMessage(reason),
            NetworkError::InvalidHeader { reason, .. } => crate::Error::InvalidHeader(reason),
            NetworkError::Configuration { reason, .. } => crate::Error::Configuration(reason),
            NetworkError::ConnectionLimitReached { .. } => crate::Error::ConnectionLimitReached,
            NetworkError::ConnectionRefused { address } => {
                crate::Error::ConnectionFailed(format!("Connection refused: {}", address))
            }
            NetworkError::Io { reason, .. } => {
                crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, reason))
            }
            NetworkError::Json { reason } => {
                crate::Error::Generic(format!("JSON error: {}", reason))
            }
            NetworkError::Ledger { reason } => crate::Error::Ledger(reason),
            _ => crate::Error::Generic(error.to_string()),
        }
    }
}

impl From<crate::Error> for NetworkError {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::Connection(msg) => NetworkError::connection_failed(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                &msg,
            ),
            crate::Error::Protocol(msg) => NetworkError::protocol_violation(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                &msg,
            ),
            crate::Error::Serialization(msg) => {
                NetworkError::message_serialization("unknown", &msg)
            }
            crate::Error::Peer(_msg) => NetworkError::peer_not_connected(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
            ),
            crate::Error::Sync(msg) => NetworkError::SyncFailed { reason: msg },
            crate::Error::Rpc(msg) => NetworkError::rpc("unknown", -1, &msg),
            crate::Error::Timeout(_msg) => NetworkError::connection_timeout(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                0,
            ),
            crate::Error::Authentication(msg) => NetworkError::authentication_failed(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                &msg,
            ),
            crate::Error::RateLimit(_msg) => NetworkError::rate_limit_exceeded(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                0.0,
                0.0,
            ),
            crate::Error::InvalidMessage(msg) => NetworkError::invalid_message(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                "unknown",
                &msg,
            ),
            crate::Error::InvalidHeader(msg) => NetworkError::InvalidHeader {
                peer: UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                reason: msg,
            },
            crate::Error::Configuration(msg) => NetworkError::configuration("unknown", &msg),
            crate::Error::ConnectionLimitReached => NetworkError::connection_limit_reached(0, 0),
            crate::Error::ConnectionFailed(msg) => NetworkError::connection_failed(
                UNKNOWN_PEER_ADDR
                    .parse()
                    .unwrap_or_else(|_| UNKNOWN_PEER_ADDR.parse().expect("value should parse")),
                &msg,
            ),
            crate::Error::Io(err) => NetworkError::io("io", &err.to_string()),
            crate::Error::Json(err) => NetworkError::Json {
                reason: err.to_string(),
            },
            crate::Error::Ledger(msg) => NetworkError::Ledger { reason: msg },
            crate::Error::Generic(msg) => NetworkError::generic(msg),
            _ => NetworkError::generic(error.to_string()),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ErrorSeverity, DEFAULT_TIMEOUT_MS};
    use crate::{NetworkError, NetworkResult};
    use std::net::SocketAddr;

    #[test]
    fn test_error_creation() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("valid address");
        let error = NetworkError::connection_failed(addr, "Network unreachable");
        assert!(matches!(error, NetworkError::ConnectionFailed { .. }));
        assert!(error.to_string().contains("127.0.0.1:8080"));
    }

    #[test]
    fn test_error_classification() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("valid address");

        // Test retryable errors
        assert!(NetworkError::connection_timeout(addr, DEFAULT_TIMEOUT_MS).is_retryable());
        assert!(!NetworkError::authentication_failed(addr, "Invalid").is_retryable());

        // Test connection errors
        assert!(NetworkError::connection_failed(addr, "Failed").is_connection_error());
        assert!(!NetworkError::rpc("test", 0, "error").is_connection_error());

        // Test protocol errors
        assert!(NetworkError::protocol_violation(addr, "Invalid").is_protocol_error());
        assert!(!NetworkError::connection_failed(addr, "Failed").is_protocol_error());

        // Test ban-worthy errors
        assert!(NetworkError::protocol_violation(addr, "Spam").should_ban_peer());
        assert!(!NetworkError::connection_timeout(addr, DEFAULT_TIMEOUT_MS).should_ban_peer());
    }

    #[test]
    fn test_error_severity() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("valid address");

        assert_eq!(
            NetworkError::connection_failed(addr, "Failed").severity(),
            ErrorSeverity::Low
        );
        assert_eq!(
            NetworkError::protocol_violation(addr, "Invalid").severity(),
            ErrorSeverity::High
        );
        assert_eq!(
            NetworkError::configuration("param", "Invalid").severity(),
            ErrorSeverity::Critical
        );
    }

    #[test]
    fn test_error_categories() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("valid address");

        assert_eq!(
            NetworkError::connection_failed(addr, "Failed").category(),
            "connection"
        );
        assert_eq!(
            NetworkError::protocol_violation(addr, "Invalid").category(),
            "protocol"
        );
        assert_eq!(NetworkError::rpc("test", 0, "error").category(), "rpc");
    }

    #[test]
    fn test_rate_limit_error() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("valid address");
        let error = NetworkError::rate_limit_exceeded(addr, 100.0, 50.0);
        assert_eq!(
            error.to_string(),
            "Rate limit exceeded for 127.0.0.1:8080: 100 messages/sec > 50"
        );
    }

    #[test]
    fn test_backward_compatibility() {
        let network_error = NetworkError::connection_failed(
            "127.0.0.1:8080".parse().expect("valid address"),
            "test",
        );
        let old_error: crate::Error = network_error.into();
        assert!(matches!(old_error, crate::Error::Connection(_)));

        let old_error = crate::Error::Protocol("test".to_string());
        let network_error: NetworkError = old_error.into();
        assert!(matches!(
            network_error,
            NetworkError::ProtocolViolation { .. }
        ));
    }

    #[test]
    fn test_from_std_errors() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let network_error = NetworkError::from(io_error);
        assert!(matches!(network_error, NetworkError::Io { .. }));

        let addr_error = "invalid_address".parse::<SocketAddr>().unwrap_err();
        let network_error = NetworkError::from(addr_error);
        assert!(matches!(network_error, NetworkError::Configuration { .. }));
    }
}

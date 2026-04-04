//! Safe P2P Network Module
//!
//! This module provides safe wrappers and improvements for P2P networking,
//! replacing unsafe unwrap() calls with proper error handling.

use crate::{NetworkConfig, NetworkError, NetworkResult as Result, P2pNode};
use neo_core::safe_error_handling::{SafeError, SafeExpect, SafeUnwrap};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Safe P2P node builder with validation
/// Represents a data structure.
pub struct SafeP2pNodeBuilder {
    config: NetworkConfig,
    validate_config: bool,
    max_retries: u32,
}

impl SafeP2pNodeBuilder {
    /// Create a new safe P2P node builder
    /// Creates a new instance.
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            validate_config: true,
            max_retries: 3,
        }
    }

    /// Set whether to validate configuration
    pub fn with_validation(mut self, validate: bool) -> Self {
        self.validate_config = validate;
        self
    }

    /// Set maximum connection retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Validate the network configuration
    fn validate_config(&self) -> Result<()> {
        // Validate port range
        if self.config.port > 65535 {
            return Err(NetworkError::Configuration {
                parameter: "port".to_string(),
                reason: format!("Invalid port: {}", self.config.port),
            });
        }

        // Validate peer counts
        if self.config.max_peers == 0 {
            return Err(NetworkError::Configuration {
                parameter: "max_peers".to_string(),
                reason: "must be greater than 0".to_string(),
            });
        }

        // Check that at least some connections are allowed
        if self.config.max_outbound_connections == 0 && self.config.max_inbound_connections == 0 {
            return Err(NetworkError::Configuration {
                parameter: "connections".to_string(),
                reason: "must allow at least one inbound or outbound connection".to_string(),
            });
        }

        // Validate timeouts
        if self.config.handshake_timeout == 0 {
            return Err(NetworkError::Configuration {
                parameter: "handshake_timeout".to_string(),
                reason: "must be greater than 0".to_string(),
            });
        }

        Ok(())
    }

    /// Build the P2P node with safe error handling
    pub async fn build(
        self,
        command_receiver: mpsc::Receiver<crate::NetworkCommand>,
    ) -> Result<Arc<P2pNode>> {
        // Validate configuration if requested
        if self.validate_config {
            self.validate_config()?;
        }

        // Create P2P node with validated configuration
        match P2pNode::new(self.config.clone(), command_receiver) {
            Ok(node) => {
                tracing::info!("P2P node created successfully");
                Ok(Arc::new(node))
            }
            Err(e) => {
                tracing::error!("Failed to create P2P node: {}", e);
                Err(e.into())
            }
        }
    }
}

/// Safe message serialization wrapper
/// Represents a data structure.
pub struct SafeMessageSerializer;

impl SafeMessageSerializer {
    /// Safely serialize a message to JSON
    pub fn to_json<T: serde::Serialize>(value: &T, context: &str) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| NetworkError::message_serialization(context, &format!("{}", e)))
    }

    /// Safely deserialize a message from JSON
    pub fn from_json<T: serde::de::DeserializeOwned>(data: &str, context: &str) -> Result<T> {
        serde_json::from_str(data).map_err(|e| {
            NetworkError::message_deserialization(data.len(), format!("{}: {}", context, e))
        })
    }

    /// Safely serialize to bytes
    pub fn to_bytes<T: serde::Serialize>(value: &T, context: &str) -> Result<Vec<u8>> {
        bincode::serialize(value)
            .map_err(|e| NetworkError::message_serialization(context, &format!("{}", e)))
    }

    /// Safely deserialize from bytes
    pub fn from_bytes<T: serde::de::DeserializeOwned>(data: &[u8], context: &str) -> Result<T> {
        bincode::deserialize(data).map_err(|e| {
            NetworkError::message_deserialization(data.len(), format!("{}: {}", context, e))
        })
    }
}

/// Safe network message validator
/// Represents a data structure.
pub struct MessageValidator {
    max_message_size: usize,
    allow_unknown_commands: bool,
}

impl MessageValidator {
    /// Create a new message validator
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            max_message_size: 10 * 1024 * 1024, // 10MB default
            allow_unknown_commands: false,
        }
    }

    /// Set maximum message size
    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }

    /// Set whether to allow unknown commands
    pub fn allow_unknown(mut self, allow: bool) -> Self {
        self.allow_unknown_commands = allow;
        self
    }

    /// Validate message size
    /// Validates the input or state.
    pub fn validate_size(&self, size: usize, _context: &str) -> Result<()> {
        if size > self.max_message_size {
            return Err(NetworkError::MessageTooLarge {
                size,
                max_size: self.max_message_size,
            });
        }
        Ok(())
    }

    /// Validate message command
    /// Validates the input or state.
    pub fn validate_command(&self, command: &str) -> Result<()> {
        // List of known valid commands
        const VALID_COMMANDS: &[&str] = &[
            "version",
            "verack",
            "getaddr",
            "addr",
            "ping",
            "pong",
            "getblocks",
            "getdata",
            "getblockbyindex",
            "inv",
            "block",
            "consensus",
            "tx",
            "merkleblock",
            "notfound",
            "reject",
            "alert",
            "headers",
            "getheaders",
            "mempool",
        ];

        if !VALID_COMMANDS.contains(&command) && !self.allow_unknown_commands {
            // Return error with null peer address for unknown commands
            return Err(NetworkError::InvalidMessage {
                peer: "0.0.0.0:0".parse().expect("Valid null socket address"),
                message_type: "command".to_string(),
                reason: format!("Unknown command: {}", command),
            });
        }

        Ok(())
    }
}

/// Safe peer connection manager
/// Represents a data structure.
pub struct SafePeerManager {
    max_connection_attempts: u32,
    connection_timeout: tokio::time::Duration,
    retry_delay: tokio::time::Duration,
}

impl SafePeerManager {
    /// Create a new safe peer manager
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            max_connection_attempts: 3,
            connection_timeout: tokio::time::Duration::from_secs(10),
            retry_delay: tokio::time::Duration::from_secs(5),
        }
    }

    /// Safely connect to a peer with retry logic
    pub async fn connect_with_retry(
        &self,
        address: std::net::SocketAddr,
    ) -> Result<tokio::net::TcpStream> {
        let mut last_error = None;

        for attempt in 1..=self.max_connection_attempts {
            tracing::debug!(
                "Attempting connection to {} (attempt {}/{})",
                address,
                attempt,
                self.max_connection_attempts
            );

            match tokio::time::timeout(
                self.connection_timeout,
                tokio::net::TcpStream::connect(address),
            )
            .await
            {
                Ok(Ok(stream)) => {
                    tracing::info!("Successfully connected to {}", address);
                    return Ok(stream);
                }
                Ok(Err(e)) => {
                    tracing::warn!("Connection to {} failed: {}", address, e);
                    last_error = Some(NetworkError::ConnectionFailed {
                        address,
                        reason: e.to_string(),
                    });
                }
                Err(_) => {
                    tracing::warn!("Connection to {} timed out", address);
                    last_error = Some(NetworkError::ConnectionTimeout {
                        address,
                        timeout_ms: self.connection_timeout.as_millis() as u64,
                    });
                }
            }

            // Wait before retry (except on last attempt)
            if attempt < self.max_connection_attempts {
                tokio::time::sleep(self.retry_delay).await;
            }
        }

        Err(
            last_error.unwrap_or_else(|| NetworkError::ConnectionFailed {
                address,
                reason: format!(
                    "Failed to connect after {} attempts",
                    self.max_connection_attempts
                ),
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_p2p_builder_validation() {
        let mut config = NetworkConfig::testnet();
        config.max_peers = 0; // Invalid

        let builder = SafeP2pNodeBuilder::new(config);
        let result = builder.validate_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_serialization() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct TestMessage {
            id: u32,
            data: String,
        }

        let msg = TestMessage {
            id: 42,
            data: "test".to_string(),
        };

        // Test JSON serialization
        let json = SafeMessageSerializer::to_json(&msg, "test message")
            .expect("Serialization should succeed");
        let deserialized: TestMessage = SafeMessageSerializer::from_json(&json, "test message")
            .expect("Deserialization should succeed");
        assert_eq!(msg, deserialized);

        // Test binary serialization
        let bytes = SafeMessageSerializer::to_bytes(&msg, "test message")
            .expect("Serialization should succeed");
        let deserialized: TestMessage = SafeMessageSerializer::from_bytes(&bytes, "test message")
            .expect("Deserialization should succeed");
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_message_validator() {
        let validator = MessageValidator::new().with_max_size(1024);

        // Test size validation
        assert!(validator.validate_size(512, "test").is_ok());
        assert!(validator.validate_size(2048, "test").is_err());

        // Test command validation
        assert!(validator.validate_command("ping").is_ok());
        assert!(validator.validate_command("unknown").is_err());

        // Test with unknown commands allowed
        let validator = MessageValidator::new().allow_unknown(true);
        assert!(validator.validate_command("unknown").is_ok());
    }
}

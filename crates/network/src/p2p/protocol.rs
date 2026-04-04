//! P2P protocol message handlers.
//!
//! This module implements protocol message handling exactly matching C# Neo's RemoteNode message handling.

use crate::{NetworkError, NetworkMessage, NetworkResult};
use crate::{NetworkError as Error, NetworkResult as Result};
use neo_config::DEFAULT_NEO_PORT;
use neo_config::DEFAULT_RPC_PORT;
use neo_config::DEFAULT_TESTNET_PORT;
use neo_config::DEFAULT_TESTNET_RPC_PORT;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use std::net::SocketAddr;

/// Default Neo network ports
/// Message handler trait (matches C# Neo IMessageHandler pattern)
#[async_trait::async_trait]
/// Defines a trait interface.
pub trait MessageHandler: Send + Sync {
    /// Handles a message from a peer
    async fn handle_message(
        &self,
        peer_address: SocketAddr,
        message: &NetworkMessage,
    ) -> NetworkResult<()>;
}

/// Protocol message utilities
/// Represents a data structure.
pub struct ProtocolUtils;

impl ProtocolUtils {
    /// Generates a peer ID from connection data (matches C# Neo peer ID generation exactly)
    pub async fn generate_peer_id(
        address: SocketAddr,
        nonce: u32,
        user_agent: &str,
    ) -> neo_core::UInt160 {
        let mut hasher = Sha256::new();

        // Add address components
        hasher.update(&address.ip().to_string().as_bytes());
        hasher.update(&address.port().to_le_bytes());

        // Add nonce
        hasher.update(&nonce.to_le_bytes());

        // Add user agent
        hasher.update(user_agent.as_bytes());

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        hasher.update(&now.as_secs().to_le_bytes());
        hasher.update(&now.subsec_nanos().to_le_bytes());

        // Add some randomness to ensure uniqueness even in rapid calls
        hasher.update(&rand::random::<u64>().to_le_bytes());

        let sha256_result = hasher.finalize();

        // Apply RIPEMD160 to get final 160-bit hash
        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(&sha256_result);
        let ripemd_result = ripemd_hasher.finalize();

        neo_core::UInt160::from_bytes(&ripemd_result).unwrap_or_else(|_| neo_core::UInt160::zero())
    }

    /// Validates protocol message format (matches C# Neo message validation exactly)
    /// Validates the input or state.
    pub fn validate_message(message: &NetworkMessage) -> NetworkResult<()> {
        // Validate magic number
        if message.header.magic == 0 {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: "Invalid magic number".to_string(),
            });
        }

        // No need to validate since MessageCommand ensures this constraint

        // Validate payload size
        if message.header.length > 0x02000000 {
            // 32MB limit
            return Err(NetworkError::MessageTooLarge {
                size: message.header.length as usize,
                max_size: 0x02000000,
            });
        }

        if message.header.length > 0 {
            let message_bytes =
                message
                    .to_bytes()
                    .map_err(|e| NetworkError::MessageSerialization {
                        message_type: "NetworkMessage".to_string(),
                        reason: format!("Failed to serialize message: {}", e),
                    })?;
            let payload_start = 13; // Header is 13 bytes
            if message_bytes.len() > payload_start {
                let payload = &message_bytes[payload_start..];
                let calculated_checksum = Self::calculate_checksum(payload);
                if calculated_checksum != message.header.checksum {
                    return Err(NetworkError::InvalidHeader {
                        peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                        reason: "Invalid checksum".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Calculates message checksum (matches C# Neo checksum calculation exactly)
    pub fn calculate_checksum(payload: &[u8]) -> u32 {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let hash = hasher.finalize();

        // Take first 4 bytes as checksum
        u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
    }

    /// Checks if a message type is critical for sync (matches C# Neo sync protocol exactly)
    /// Checks a boolean condition.
    pub fn is_sync_message(command: &str) -> bool {
        matches!(
            command,
            "version"
                | "verack"
                | "getblocks"
                | "getdata"
                | "block"
                | "headers"
                | "getheaders"
                | "inv"
        )
    }

    /// Checks if a message type requires authentication (matches C# Neo security model exactly)
    pub fn requires_authentication(command: &str) -> bool {
        matches!(command, "addr" | "getaddr" | "ping" | "pong")
    }

    /// Gets message priority for processing (matches C# Neo message prioritization exactly)
    /// Gets a value from the internal state.
    pub fn get_message_priority(command: &str) -> u8 {
        match command {
            // Highest priority - connection management
            "version" | "verack" => 10,

            // High priority - sync critical
            "ping" | "pong" => 9,
            "getblocks" | "getheaders" => 8,
            "headers" | "block" => 7,

            // Medium priority - inventory
            "inv" | "getdata" => 6,
            "tx" => 5,

            // Lower priority - misc
            "addr" | "getaddr" => 3,

            // Lowest priority - unknown
            _ => 1,
        }
    }

    /// Determines if message should be relayed to other peers (matches C# Neo relay logic exactly)
    pub fn should_relay_message(command: &str) -> bool {
        matches!(command, "inv" | "tx" | "block" | "addr")
    }

    /// Gets expected response message type (matches C# Neo protocol flows exactly)
    /// Gets a value from the internal state.
    pub fn get_expected_response(command: &str) -> Option<&'static str> {
        match command {
            "version" => Some("verack"),
            "ping" => Some("pong"),
            "getblocks" => Some("inv"),
            "getheaders" => Some("headers"),
            "getdata" => Some("block"), // or "tx" depending on inventory type
            "getaddr" => Some("addr"),
            _ => None,
        }
    }
}

/// Default message handler implementation
/// Represents a data structure.
pub struct DefaultMessageHandler;

#[async_trait::async_trait]
impl MessageHandler for DefaultMessageHandler {
    async fn handle_message(
        &self,
        peer_address: SocketAddr,
        message: &NetworkMessage,
    ) -> NetworkResult<()> {
        tracing::debug!(
            "Default handler received message from {}: {}",
            peer_address,
            message.header.command
        );

        // Validate the message first
        ProtocolUtils::validate_message(message)?;

        tracing::trace!(
            "Message details - Magic: 0x{:08x}, Command: {}, Length: {}, Checksum: 0x{:08x}",
            message.header.magic,
            message.header.command,
            message.header.length,
            message.header.checksum
        );

        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_peer_id_generation() {
        let address: SocketAddr = "127.0.0.1:10333".parse().unwrap();
        let nonce = 12345;
        let user_agent = "neo-rs/1.0.0";

        let peer_id1 = ProtocolUtils::generate_peer_id(address, nonce, user_agent).await;
        let peer_id2 = ProtocolUtils::generate_peer_id(address, nonce, user_agent).await;

        // IDs should be different due to timestamp
        assert_ne!(peer_id1, peer_id2);
        assert_ne!(peer_id1, neo_core::UInt160::zero());
    }

    #[test]
    fn test_message_priority() {
        assert_eq!(ProtocolUtils::get_message_priority("version"), 10);
        assert_eq!(ProtocolUtils::get_message_priority("ping"), 9);
        assert_eq!(ProtocolUtils::get_message_priority("getblocks"), 8);
        assert_eq!(ProtocolUtils::get_message_priority("block"), 7);
        assert_eq!(ProtocolUtils::get_message_priority("unknown"), 1);
    }

    #[test]
    fn test_sync_messages() {
        assert!(ProtocolUtils::is_sync_message("version"));
        assert!(ProtocolUtils::is_sync_message("getblocks"));
        assert!(ProtocolUtils::is_sync_message("headers"));
        assert!(!ProtocolUtils::is_sync_message("addr"));
    }

    #[test]
    fn test_relay_messages() {
        assert!(ProtocolUtils::should_relay_message("inv"));
        assert!(ProtocolUtils::should_relay_message("tx"));
        assert!(!ProtocolUtils::should_relay_message("version"));
        assert!(!ProtocolUtils::should_relay_message("ping"));
    }

    #[test]
    fn test_expected_responses() {
        assert_eq!(
            ProtocolUtils::get_expected_response("version"),
            Some("verack")
        );
        assert_eq!(ProtocolUtils::get_expected_response("ping"), Some("pong"));
        assert_eq!(
            ProtocolUtils::get_expected_response("getblocks"),
            Some("inv")
        );
        assert_eq!(ProtocolUtils::get_expected_response("unknown"), None);
    }

    #[test]
    fn test_checksum_calculation() {
        let payload = b"test payload";
        let checksum1 = ProtocolUtils::calculate_checksum(payload);
        let checksum2 = ProtocolUtils::calculate_checksum(payload);

        // Same payload should produce same checksum
        assert_eq!(checksum1, checksum2);

        // Different payload should produce different checksum
        let different_payload = b"different payload";
        let checksum3 = ProtocolUtils::calculate_checksum(different_payload);
        assert_ne!(checksum1, checksum3);
    }
}

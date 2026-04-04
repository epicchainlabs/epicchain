//! Complete network message wrapper.
//!
//! This module provides the NetworkMessage type for Neo 3 protocol.
//! Neo 3 uses a 2-byte header (flags + command) with variable-length payload.

use super::{
    commands::{MessageCommand, MessageFlags},
    header::Neo3Message,
    protocol::ProtocolMessage,
};
use crate::{NetworkError, NetworkResult as Result};
use sha2::{Digest, Sha256};

/// Complete network message (Neo 3 format)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkMessage {
    /// Neo 3 message structure
    pub neo3_message: Neo3Message,
    /// Parsed payload (for convenience)
    pub payload: ProtocolMessage,
    /// Compatibility header (for legacy code)
    pub header: HeaderCompat,
    /// Compatibility checksum field (legacy tests expect direct access)
    pub checksum: u32,
}

impl NetworkMessage {
    /// Creates a new network message (Neo 3 format)
    pub fn new(payload: ProtocolMessage) -> Self {
        Self::new_with_magic(payload, 0x334F454E) // Default to MainNet magic
    }

    /// Creates a new network message with specific magic value
    pub fn new_with_magic(payload: ProtocolMessage, magic: u32) -> Self {
        let serialized_payload = payload.to_bytes().unwrap_or_default();
        let command = payload.command();
        let payload_length = serialized_payload.len() as u32;
        let neo3_message = Neo3Message::new(command, serialized_payload);
        // Calculate checksum for compatibility with legacy tests
        let checksum = if payload_length > 0 {
            let mut hasher = Sha256::new();
            hasher.update(neo3_message.get_payload().unwrap_or_default());
            let hash = hasher.finalize();
            u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
        } else {
            0
        };

        let header = HeaderCompat {
            command,
            magic,
            length: payload_length,
            checksum,
        };

        Self {
            neo3_message,
            payload,
            header,
            checksum,
        }
    }

    /// Serializes the message to bytes (uses Neo3 format for N3 networks)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        // Check if we should use Neo3 format based on magic
        if self.header.magic == 0x334F454E || // Neo N3 MainNet
           self.header.magic == 0x3554334E
        // Neo N3 TestNet
        {
            // Use Neo3 format (flags + command + varlen payload length + payload)
            // This is the direct format that Neo N3 nodes expect
            Ok(self.neo3_message.to_bytes())
        } else {
            // Use legacy format for compatibility
            self.to_legacy_bytes()
        }
    }

    /// Compatibility alias for serialization used by some tests
    pub fn serialize(&self) -> Result<Vec<u8>> {
        self.to_bytes()
    }

    /// Deserializes a message from bytes (supports Neo N3 real protocol format)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // First try the actual Neo 3 format (flags + command + varlen + payload)
        // This should be the primary format for Neo N3 networks
        if let Ok(neo3_message) = Neo3Message::from_bytes(bytes) {
            let payload_bytes = neo3_message.get_payload()?;

            // Deserialize payload based on command
            let payload = ProtocolMessage::from_bytes(&neo3_message.command, &payload_bytes)?;

            // Create compatibility header with default TestNet magic for compatibility
            let header = HeaderCompat {
                command: neo3_message.command,
                magic: 0x3554334E, // Neo N3 TestNet magic (default for testnet)
                length: payload_bytes.len() as u32,
                checksum: 0,
            };

            return Ok(Self {
                neo3_message,
                payload,
                header,
                checksum: 0,
            });
        }

        // Check if this is a legacy format message (24-byte header)
        if bytes.len() >= 24 {
            // Try to parse as legacy format
            let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

            // Check for known Neo magic values
            if magic == 0x74746E41 || // Neo N3 TestNet "N3T5"
               magic == 0x334e4f45 || // Neo N3 MainNet "NEO3"
               magic == 0x334F454E || // Neo N3 MainNet "3OEN" (actual magic used)
               magic == 0x3554334E || // Neo N3 TestNet "4N3T" (correct testnet magic)
               magic == 0x74734e4e || // Neo Legacy TestNet
               magic == 0x00746e41
            // Neo Legacy MainNet
            {
                return Self::from_legacy_bytes(bytes);
            }
        }

        // If we can't parse it, return an error with more debugging info
        Err(NetworkError::ProtocolViolation {
            peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
            violation: format!(
                "Unknown message format. Length: {}, First 8 bytes: {:02X?}",
                bytes.len(),
                &bytes[..std::cmp::min(8, bytes.len())]
            ),
        })
    }

    /// Gets the serialized size of the message
    pub fn serialized_size(&self) -> usize {
        self.neo3_message.serialized_size()
    }

    /// Gets the message command
    pub fn command(&self) -> MessageCommand {
        self.neo3_message.command
    }

    /// Gets the message flags
    pub fn flags(&self) -> MessageFlags {
        self.neo3_message.flags
    }

    /// Deserializes a message from legacy 24-byte header format
    fn from_legacy_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 24 {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!("Legacy message too short: {} bytes", bytes.len()),
            });
        }

        // Parse legacy header
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        // Extract command (12 bytes, null-terminated string)
        let command_bytes = &bytes[4..16];
        let command_str = std::str::from_utf8(command_bytes)
            .map_err(|_| NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: "Invalid command string".to_string(),
            })?
            .trim_end_matches('\0');

        let length = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let checksum = u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);

        // Validate payload length
        if length > 0x02000000 {
            // 32MB limit
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!("Payload too large: {} bytes", length),
            });
        }

        // Extract payload
        let payload_start = 24;
        if bytes.len() < payload_start + length as usize {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!(
                    "Insufficient payload data: expected {} bytes, got {}",
                    length,
                    bytes.len() - payload_start
                ),
            });
        }

        let payload_bytes = bytes[payload_start..payload_start + length as usize].to_vec();

        // Verify checksum if payload exists
        if length > 0 {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&payload_bytes);
            let hash = hasher.finalize();
            let calculated_checksum = u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]);

            if calculated_checksum != checksum {
                return Err(NetworkError::ProtocolViolation {
                    peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                    violation: format!(
                        "Checksum mismatch: expected 0x{:08x}, got 0x{:08x}",
                        checksum, calculated_checksum
                    ),
                });
            }
        }

        // Convert command string to enum
        let command = MessageCommand::from_str(command_str)?;

        // Parse payload based on command
        let payload = ProtocolMessage::from_bytes(&command, &payload_bytes)?;

        // Create Neo3Message for compatibility
        let neo3_message = Neo3Message::new_uncompressed(command, payload_bytes);

        // Create header
        let header = HeaderCompat {
            command,
            magic,
            length,
            checksum,
        };

        Ok(Self {
            neo3_message,
            payload,
            header,
            checksum,
        })
    }

    /// Compatibility alias for deserialization used by some tests
    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        Self::from_bytes(bytes)
    }

    /// Serializes the message to legacy 24-byte header format
    fn to_legacy_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Magic (4 bytes)
        bytes.extend_from_slice(&self.header.magic.to_le_bytes());

        // Command (12 bytes, null-padded)
        let command_str = self.header.command.as_str();
        let mut command_bytes = [0u8; 12];
        let command_len = command_str.len().min(12);
        command_bytes[..command_len].copy_from_slice(&command_str.as_bytes()[..command_len]);
        bytes.extend_from_slice(&command_bytes);

        // Get payload bytes
        let payload_bytes = self.payload.to_bytes()?;
        let length = payload_bytes.len() as u32;

        // Length (4 bytes)
        bytes.extend_from_slice(&length.to_le_bytes());

        // Checksum (4 bytes)
        let checksum = if length > 0 {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&payload_bytes);
            let hash = hasher.finalize();
            u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
        } else {
            0
        };
        bytes.extend_from_slice(&checksum.to_le_bytes());

        // Payload
        bytes.extend_from_slice(&payload_bytes);

        Ok(bytes)
    }

    /// Deserializes a message from standard Neo protocol format (as sent by NGD nodes)
    /// Format: [magic(4)] + [command(12)] + [length(4)] + [checksum(4)] + [payload]
    pub fn from_neo3_real_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 24 {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!("Message header too short: {} bytes", bytes.len()),
            });
        }

        // Parse header
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let command_bytes = &bytes[4..16];
        let length = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]) as usize;
        let checksum = u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);

        // Extract payload
        if bytes.len() < 24 + length {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!(
                    "Insufficient payload data: expected {} bytes, got {}",
                    length,
                    bytes.len() - 24
                ),
            });
        }

        let payload_bytes = if length > 0 {
            bytes[24..24 + length].to_vec()
        } else {
            vec![]
        };

        // Parse command
        let command_str = String::from_utf8_lossy(command_bytes)
            .trim_end_matches('\0')
            .to_string();

        let command = MessageCommand::from_str(&command_str)?;
        let payload = ProtocolMessage::from_bytes(&command, &payload_bytes)?;

        // Create Neo3Message for compatibility
        let neo3_message = Neo3Message::new_uncompressed(command, payload_bytes);

        // Create header
        let header = HeaderCompat {
            command,
            magic,
            length: length as u32,
            checksum: 0,
        };

        Ok(Self {
            neo3_message,
            payload,
            header,
            checksum: 0,
        })
    }
}

/// Compatibility header structure for legacy code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderCompat {
    pub command: MessageCommand,
    pub magic: u32,
    pub length: u32,
    pub checksum: u32,
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::messages::protocol::ProtocolMessage;

    #[test]
    fn test_neo3_network_message_serialization() {
        let payload = ProtocolMessage::Ping { nonce: 12345 };

        let message = NetworkMessage::new(payload.clone());

        assert_eq!(message.command(), MessageCommand::Ping);
        assert_eq!(message.payload, payload);

        // Test serialization roundtrip
        let message_bytes = message.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&message_bytes).unwrap();

        assert_eq!(message.header.command, deserialized.header.command);
        assert_eq!(message.header.length, deserialized.header.length);
        assert_eq!(message.payload, deserialized.payload);
    }

    #[test]
    fn test_neo3_network_message_verack() {
        let payload = ProtocolMessage::Verack;

        let message = NetworkMessage::new(payload);

        assert_eq!(message.header.command, MessageCommand::Verack);
        assert_eq!(message.header.length, 0); // Verack has empty payload

        // Test serialization (Neo3 format: flags + command + varlen length)
        let bytes = message.to_bytes().unwrap();
        assert_eq!(bytes.len(), 3);
    }
}

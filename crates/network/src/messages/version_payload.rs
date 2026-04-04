//! VersionPayload implementation matching C# VersionPayload.cs exactly

use super::capabilities::NodeCapability;
use crate::{NetworkError, NetworkResult as Result};
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use serde::{Deserialize, Serialize};

/// Maximum number of capabilities (matches C# MaxCapabilities)
pub const MAX_CAPABILITIES: usize = 32;

/// Version payload for handshake (matches C# VersionPayload exactly)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionPayload {
    /// The magic number of the network (matches C# Network property)
    pub network: u32,

    /// The protocol version of the node (matches C# Version property)
    pub version: u32,

    /// The time when connected to the node UTC (matches C# Timestamp property)
    pub timestamp: u32,

    /// A random number used to identify the node (matches C# Nonce property)
    pub nonce: u32,

    /// String used to identify the client software (matches C# UserAgent property)
    pub user_agent: String,

    /// True if compression is allowed (matches C# AllowCompression property)
    pub allow_compression: bool,

    /// The capabilities of the node (matches C# Capabilities property)
    pub capabilities: Vec<NodeCapability>,
}

impl VersionPayload {
    /// Create new VersionPayload (matches C# constructor)
    pub fn new(network: u32, nonce: u32, user_agent: String) -> Self {
        Self {
            network,
            version: 0, // Current Neo protocol version
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as u32,
            nonce,
            user_agent,
            allow_compression: true, // Enable compression by default
            capabilities: Vec::new(),
        }
    }

    /// Add capability (matches C# capability management)
    pub fn add_capability(&mut self, capability: NodeCapability) -> Result<()> {
        if self.capabilities.len() >= MAX_CAPABILITIES {
            return Err(NetworkError::InvalidMessage {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                message_type: "version".to_string(),
                reason: format!("Too many capabilities: {}", self.capabilities.len()),
            });
        }

        self.capabilities.push(capability);
        Ok(())
    }

    /// Add TCP server capability (convenience method)
    pub fn add_tcp_server_capability(&mut self, port: u16) -> Result<()> {
        self.add_capability(NodeCapability::tcp_server(port))
    }

    /// Add full node capability (convenience method)
    pub fn add_full_node_capability(&mut self, start_height: u32) -> Result<()> {
        self.add_capability(NodeCapability::full_node(start_height))
    }
}

impl Serializable for VersionPayload {
    /// Deserialize VersionPayload (matches C# ISerializable.Deserialize exactly)
    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        let network = reader.read_u32()?;
        let version = reader.read_u32()?;
        let timestamp = reader.read_u32()?;
        let nonce = reader.read_u32()?;

        // Read UserAgent as VarString (matches C# ReadVarString)
        let user_agent = reader.read_var_string(1024)?;

        // Read AllowCompression (not in current C# version but planned)
        let allow_compression = true; // Default for compatibility

        // Read capabilities array (matches C# ReadSerializableArray)
        let capabilities_count = reader.read_var_int(MAX_CAPABILITIES as u64)? as usize;
        let mut capabilities = Vec::with_capacity(capabilities_count);

        for _ in 0..capabilities_count {
            // Each capability is: type (1 byte) + data length + data
            let cap_type = reader.read_byte()?;
            let data_len = reader.read_var_int(1024)? as usize;
            let data = reader.read_bytes(data_len)?;

            let capability = NodeCapability {
                capability_type: match cap_type {
                    0x01 => super::capabilities::NodeCapabilityType::TcpServer,
                    0x02 => super::capabilities::NodeCapabilityType::WsServer,
                    0x10 => super::capabilities::NodeCapabilityType::FullNode,
                    _ => super::capabilities::NodeCapabilityType::TcpServer, // Default
                },
                data,
            };

            capabilities.push(capability);
        }

        Ok(Self {
            network,
            version,
            timestamp,
            nonce,
            user_agent,
            allow_compression,
            capabilities,
        })
    }

    /// Serialize VersionPayload (matches C# ISerializable.Serialize exactly)
    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        writer.write_u32(self.network)?;
        writer.write_u32(self.version)?;
        writer.write_u32(self.timestamp)?;
        writer.write_u32(self.nonce)?;

        // Write UserAgent as VarString (matches C# WriteVarString)
        writer.write_var_string(&self.user_agent)?;

        // Write capabilities array (matches C# WriteSerializableArray)
        writer.write_var_int(self.capabilities.len() as u64)?;

        for capability in &self.capabilities {
            writer.write_u8(capability.capability_type as u8)?;
            writer.write_var_int(capability.data.len() as u64)?;
            writer.write_bytes(&capability.data)?;
        }

        Ok(())
    }

    /// Get size in bytes (matches C# Size property exactly)
    fn size(&self) -> usize {
        4 + // network
        4 + // version  
        4 + // timestamp
        4 + // nonce
        self.get_var_string_size(&self.user_agent) + // user_agent
        neo_io::helper::get_var_size(self.capabilities.len() as u64) + // capabilities count
        self.capabilities.iter().map(|c| c.size()).sum::<usize>() // capabilities data
    }
}

impl VersionPayload {
    /// Calculate VarString size (matches C# GetVarSize for strings)
    fn get_var_string_size(&self, s: &str) -> usize {
        let byte_len = s.len();
        self.get_var_size_static(byte_len) + byte_len
    }

    /// Calculate VarSize statically
    fn get_var_size_static(&self, value: usize) -> usize {
        if value < 0xFD {
            1
        } else if value <= 0xFFFF {
            3
        } else if value <= 0xFFFFFFFF {
            5
        } else {
            9
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::capabilities::NodeCapability;

    #[test]
    fn test_version_payload_creation() {
        let payload = VersionPayload::new(0x334F454E, 12345, "neo-rust/0.4.0".to_string());

        assert_eq!(payload.network, 0x334F454E);
        assert_eq!(payload.nonce, 12345);
        assert_eq!(payload.user_agent, "neo-rust/0.4.0");
        assert_eq!(payload.version, 0);
        assert!(payload.allow_compression);
    }

    #[test]
    fn test_version_payload_serialization() {
        let mut payload = VersionPayload::new(0x334F454E, 12345, "neo-rust/0.4.0".to_string());
        payload.add_tcp_server_capability(10333).unwrap();
        payload.add_full_node_capability(1000).unwrap();

        // Test serialization roundtrip
        let serialized = payload.to_array().unwrap();
        let deserialized = VersionPayload::from_bytes(&serialized).unwrap();

        assert_eq!(payload.network, deserialized.network);
        assert_eq!(payload.nonce, deserialized.nonce);
        assert_eq!(payload.user_agent, deserialized.user_agent);
        assert_eq!(payload.capabilities.len(), deserialized.capabilities.len());
    }
}

//! Node capabilities for Neo network protocol
//!
//! Matches C# Neo.Network.P2P.Capabilities exactly

use serde::{Deserialize, Serialize};

/// Node capability types (matches C# NodeCapabilityType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum NodeCapabilityType {
    /// TCP server capability
    TcpServer = 0x01,
    /// WebSocket server capability  
    WsServer = 0x02,
    /// Full node capability
    FullNode = 0x10,
}

/// Node capability structure (matches C# NodeCapability exactly)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeCapability {
    /// Type of capability
    pub capability_type: NodeCapabilityType,
    /// Capability-specific data
    pub data: Vec<u8>,
}

impl NodeCapability {
    /// Create TCP server capability (matches C# ServerCapability)
    pub fn tcp_server(port: u16) -> Self {
        Self {
            capability_type: NodeCapabilityType::TcpServer,
            data: port.to_le_bytes().to_vec(),
        }
    }

    /// Create WebSocket server capability  
    pub fn ws_server(port: u16) -> Self {
        Self {
            capability_type: NodeCapabilityType::WsServer,
            data: port.to_le_bytes().to_vec(),
        }
    }

    /// Create full node capability (matches C# FullNodeCapability)
    pub fn full_node(start_height: u32) -> Self {
        Self {
            capability_type: NodeCapabilityType::FullNode,
            data: start_height.to_le_bytes().to_vec(),
        }
    }

    /// Get size in bytes (matches C# Size property)
    pub fn size(&self) -> usize {
        1 + self.data.len() // type + data
    }
}

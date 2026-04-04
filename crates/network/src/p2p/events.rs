//! P2P events and notifications.
//!
//! This module implements P2P events exactly matching C# Neo's NetworkEventArgs and event system.

use crate::{NetworkError, NetworkMessage as Message};
use crate::{NetworkMessage, NodeInfo};
use neo_config::ADDRESS_SIZE;
use neo_config::DEFAULT_NEO_PORT;
use neo_config::DEFAULT_RPC_PORT;
use neo_config::DEFAULT_TESTNET_PORT;
use neo_config::DEFAULT_TESTNET_RPC_PORT;
use neo_core::UInt160;
use std::net::SocketAddr;

/// Default Neo network ports
/// P2P events (matches C# Neo NetworkEventArgs exactly)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum P2PEvent {
    /// Peer connected successfully
    PeerConnected {
        /// Unique peer identifier
        peer_id: UInt160,
        /// Peer network address
        address: SocketAddr,
    },

    /// Peer disconnected
    PeerDisconnected {
        /// Unique peer identifier
        peer_id: UInt160,
        /// Peer network address
        address: SocketAddr,
        /// Disconnection reason
        reason: String,
    },

    /// Message received from peer
    MessageReceived {
        /// Unique peer identifier
        peer_id: UInt160,
        /// The received message
        message: NetworkMessage,
    },

    /// Connection attempt failed
    ConnectionFailed {
        /// Target address that failed to connect
        address: SocketAddr,
        /// Error description
        error: String,
    },

    /// Handshake completed successfully
    HandshakeCompleted {
        /// Unique peer identifier
        peer_id: UInt160,
        /// Peer network address
        address: SocketAddr,
        /// Peer node information
        node_info: NodeInfo,
    },

    /// Peer height reported (CRITICAL for blockchain sync)
    /// This event is essential for proper block synchronization
    PeerHeight {
        /// Peer network address
        address: SocketAddr,
        /// Reported blockchain height
        height: u32,
    },

    /// Peer version information received
    PeerVersion {
        /// Peer network address
        address: SocketAddr,
        /// Protocol version
        version: u32,
        /// User agent string
        user_agent: String,
        /// Starting blockchain height
        start_height: u32,
    },

    /// Network connectivity status changed
    NetworkStatus {
        /// Whether network is connected
        connected: bool,
        /// Number of active peers
        peer_count: usize,
    },

    /// Ping measurement completed
    PingCompleted {
        /// Peer network address
        address: SocketAddr,
        /// Round-trip time in milliseconds
        rtt_ms: u64,
    },
}

impl P2PEvent {
    /// Creates a peer connected event
    pub fn peer_connected(peer_id: UInt160, address: SocketAddr) -> Self {
        Self::PeerConnected { peer_id, address }
    }

    /// Creates a peer disconnected event
    pub fn peer_disconnected(peer_id: UInt160, address: SocketAddr, reason: String) -> Self {
        Self::PeerDisconnected {
            peer_id,
            address,
            reason,
        }
    }

    /// Creates a message received event
    pub fn message_received(peer_id: UInt160, message: NetworkMessage) -> Self {
        Self::MessageReceived { peer_id, message }
    }

    /// Creates a connection failed event
    pub fn connection_failed(address: SocketAddr, error: String) -> Self {
        Self::ConnectionFailed { address, error }
    }

    /// Creates a handshake completed event
    pub fn handshake_completed(peer_id: UInt160, address: SocketAddr, node_info: NodeInfo) -> Self {
        Self::HandshakeCompleted {
            peer_id,
            address,
            node_info,
        }
    }

    /// Creates a peer height event (critical for sync)
    pub fn peer_height(address: SocketAddr, height: u32) -> Self {
        Self::PeerHeight { address, height }
    }

    /// Creates a peer version event
    pub fn peer_version(
        address: SocketAddr,
        version: u32,
        user_agent: String,
        start_height: u32,
    ) -> Self {
        Self::PeerVersion {
            address,
            version,
            user_agent,
            start_height,
        }
    }

    /// Creates a network status event
    pub fn network_status(connected: bool, peer_count: usize) -> Self {
        Self::NetworkStatus {
            connected,
            peer_count,
        }
    }

    /// Creates a ping completed event
    pub fn ping_completed(address: SocketAddr, rtt_ms: u64) -> Self {
        Self::PingCompleted { address, rtt_ms }
    }

    /// Gets the peer address associated with this event, if any
    pub fn peer_address(&self) -> Option<SocketAddr> {
        match self {
            P2PEvent::PeerConnected { address, .. }
            | P2PEvent::PeerDisconnected { address, .. }
            | P2PEvent::ConnectionFailed { address, .. }
            | P2PEvent::HandshakeCompleted { address, .. }
            | P2PEvent::PeerHeight { address, .. }
            | P2PEvent::PeerVersion { address, .. }
            | P2PEvent::PingCompleted { address, .. } => Some(*address),
            P2PEvent::MessageReceived { .. } | P2PEvent::NetworkStatus { .. } => None,
        }
    }

    /// Gets the peer ID associated with this event, if any
    pub fn peer_id(&self) -> Option<UInt160> {
        match self {
            P2PEvent::PeerConnected { peer_id, .. }
            | P2PEvent::PeerDisconnected { peer_id, .. }
            | P2PEvent::MessageReceived { peer_id, .. }
            | P2PEvent::HandshakeCompleted { peer_id, .. } => Some(*peer_id),
            _ => None,
        }
    }

    /// Checks if this is a connection-related event
    pub fn is_connection_event(&self) -> bool {
        matches!(
            self,
            P2PEvent::PeerConnected { .. }
                | P2PEvent::PeerDisconnected { .. }
                | P2PEvent::ConnectionFailed { .. }
                | P2PEvent::HandshakeCompleted { .. }
        )
    }

    /// Checks if this is a message-related event
    pub fn is_message_event(&self) -> bool {
        matches!(self, P2PEvent::MessageReceived { .. })
    }

    /// Checks if this is a sync-related event (critical for blockchain sync)
    pub fn is_sync_event(&self) -> bool {
        matches!(
            self,
            P2PEvent::PeerHeight { .. } | P2PEvent::PeerVersion { .. }
        )
    }

    /// Gets event type as string for logging
    pub fn event_type(&self) -> &'static str {
        match self {
            P2PEvent::PeerConnected { .. } => "PeerConnected",
            P2PEvent::PeerDisconnected { .. } => "PeerDisconnected",
            P2PEvent::MessageReceived { .. } => "MessageReceived",
            P2PEvent::ConnectionFailed { .. } => "ConnectionFailed",
            P2PEvent::HandshakeCompleted { .. } => "HandshakeCompleted",
            P2PEvent::PeerHeight { .. } => "PeerHeight",
            P2PEvent::PeerVersion { .. } => "PeerVersion",
            P2PEvent::NetworkStatus { .. } => "NetworkStatus",
            P2PEvent::PingCompleted { .. } => "PingCompleted",
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use neo_core::UInt160;
    use std::net::SocketAddr;
    const ADDRESS_SIZE: usize = 20;

    #[test]
    fn test_p2p_event_creation() {
        let peer_id = UInt160::from_bytes(&[1; ADDRESS_SIZE]).unwrap();
        let address = "127.0.0.1:10333".parse().unwrap();

        let event = P2PEvent::peer_connected(peer_id, address);

        assert_eq!(event.peer_id(), Some(peer_id));
        assert_eq!(event.peer_address(), Some(address));
        assert!(event.is_connection_event());
        assert_eq!(event.event_type(), "PeerConnected");
    }

    #[test]
    fn test_peer_height_event() {
        let address = "127.0.0.1:10333".parse().unwrap();
        let height = 12345;

        let event = P2PEvent::peer_height(address, height);

        assert_eq!(event.peer_address(), Some(address));
        assert!(event.is_sync_event());
        assert_eq!(event.event_type(), "PeerHeight");

        if let P2PEvent::PeerHeight { height: h, .. } = event {
            assert_eq!(h, height);
        } else {
            panic!("Expected PeerHeight event");
        }
    }

    #[test]
    fn test_event_classification() {
        let peer_id = UInt160::from_bytes(&[1; ADDRESS_SIZE]).unwrap();
        let address = "127.0.0.1:10333".parse().unwrap();

        let conn_event = P2PEvent::peer_connected(peer_id, address);
        let height_event = P2PEvent::peer_height(address, 100);
        let status_event = P2PEvent::network_status(true, 5);

        assert!(conn_event.is_connection_event());
        assert!(!conn_event.is_sync_event());

        assert!(height_event.is_sync_event());
        assert!(!height_event.is_connection_event());

        assert!(!status_event.is_connection_event());
        assert!(!status_event.is_sync_event());
    }
}

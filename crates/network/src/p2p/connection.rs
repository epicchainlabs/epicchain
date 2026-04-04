//! P2P connection management and state.
//!
//! This module implements connection management exactly matching C# Neo's Peer and Connection classes.

use crate::{NetworkError, NetworkMessage, NetworkResult, NodeInfo};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

/// Connection state (matches C# Neo RemoteNode state exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Initial connection establishing
    Connecting,

    /// TCP connection established
    Connected,

    /// Performing protocol handshake
    Handshaking,

    /// Fully connected and ready for communication
    Ready,

    /// Connection being closed
    Disconnecting,

    /// Connection closed
    Disconnected,
}

impl ConnectionState {
    /// Checks if the connection is active (can send/receive messages)
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            ConnectionState::Connected | ConnectionState::Handshaking | ConnectionState::Ready
        )
    }

    /// Checks if the connection is ready for normal operations
    pub fn is_ready(&self) -> bool {
        matches!(self, ConnectionState::Ready)
    }

    /// Checks if the connection is being established
    pub fn is_connecting(&self) -> bool {
        matches!(
            self,
            ConnectionState::Connecting | ConnectionState::Handshaking
        )
    }

    /// Checks if the connection is closed or closing
    pub fn is_closed(&self) -> bool {
        matches!(
            self,
            ConnectionState::Disconnecting | ConnectionState::Disconnected
        )
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Connecting => write!(f, "Connecting"),
            ConnectionState::Connected => write!(f, "Connected"),
            ConnectionState::Handshaking => write!(f, "Handshaking"),
            ConnectionState::Ready => write!(f, "Ready"),
            ConnectionState::Disconnecting => write!(f, "Disconnecting"),
            ConnectionState::Disconnected => write!(f, "Disconnected"),
        }
    }
}

/// Peer connection (matches C# Neo RemoteNode exactly)
#[derive(Debug)]
pub struct PeerConnection {
    /// Connection state
    pub state: ConnectionState,

    /// TCP stream
    pub stream: TcpStream,

    /// Peer address
    pub address: SocketAddr,

    /// Peer node info (available after handshake)
    pub node_info: Option<NodeInfo>,

    /// Message sender channel
    pub message_tx: mpsc::UnboundedSender<NetworkMessage>,

    /// Whether this is an inbound connection
    pub inbound: bool,

    /// Last activity timestamp
    pub last_activity: std::time::Instant,

    /// Connection established timestamp
    pub connected_at: std::time::Instant,
}

impl PeerConnection {
    /// Creates a new peer connection
    pub fn new(stream: TcpStream, address: SocketAddr, inbound: bool) -> Self {
        let (message_tx, _) = mpsc::unbounded_channel();
        let now = std::time::Instant::now();

        Self {
            state: ConnectionState::Connected,
            stream,
            address,
            node_info: None,
            message_tx,
            inbound,
            last_activity: now,
            connected_at: now,
        }
    }

    /// Updates the last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = std::time::Instant::now();
    }

    /// Gets the connection duration
    pub fn connection_duration(&self) -> std::time::Duration {
        std::time::Instant::now().duration_since(self.connected_at)
    }

    /// Gets the time since last activity
    pub fn idle_duration(&self) -> std::time::Duration {
        std::time::Instant::now().duration_since(self.last_activity)
    }

    /// Sets the connection state
    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
        self.update_activity();
    }

    /// Sets the peer node information (after handshake)
    pub fn set_node_info(&mut self, node_info: NodeInfo) {
        self.node_info = Some(node_info);
        self.update_activity();
    }

    /// Sends a message to the peer (matches C# RemoteNode.SendMessage exactly)
    pub async fn send_message(&mut self, message: NetworkMessage) -> NetworkResult<()> {
        if !self.state.is_active() {
            return Err(NetworkError::ConnectionFailed {
                address: self.address,
                reason: "Connection not active".to_string(),
            });
        }

        // Serialize message to bytes
        let bytes = message
            .to_bytes()
            .map_err(|e| NetworkError::MessageSerialization {
                message_type: "NetworkMessage".to_string(),
                reason: format!("Failed to serialize message: {}", e),
            })?;

        tracing::debug!(
            "Sending message to {}: {} bytes, command: {:?}",
            self.address,
            bytes.len(),
            message.header.command
        );
        tracing::debug!(
            "First 24 bytes (header): {:02x?}",
            &bytes[..24.min(bytes.len())]
        );

        // Send bytes over TCP stream
        self.stream
            .write_all(&bytes)
            .await
            .map_err(|e| NetworkError::ConnectionFailed {
                address: self.address,
                reason: format!("Failed to send message: {}", e),
            })?;

        // Update activity timestamp
        self.update_activity();

        Ok(())
    }

    /// Receives a message from the peer (matches C# RemoteNode.ReceiveMessage exactly)
    pub async fn receive_message(&mut self) -> NetworkResult<NetworkMessage> {
        if !self.state.is_active() {
            return Err(NetworkError::ConnectionFailed {
                address: self.address,
                reason: "Connection not active".to_string(),
            });
        }

        let mut header_bytes = [0u8; 24];
        self.stream
            .read_exact(&mut header_bytes)
            .await
            .map_err(|e| NetworkError::ConnectionFailed {
                address: self.address,
                reason: format!("Failed to read header: {}", e),
            })?;

        tracing::debug!("Read header bytes: {:02x?}", &header_bytes);

        let payload_length = u32::from_le_bytes([
            header_bytes[16],
            header_bytes[17],
            header_bytes[18],
            header_bytes[19],
        ]);

        // Validate payload length
        if payload_length > 0x02000000 {
            // 32MB limit
            return Err(NetworkError::ProtocolViolation {
                peer: self.address,
                violation: "Payload too large".to_string(),
            });
        }

        // Read payload
        let mut payload_bytes = vec![0u8; payload_length as usize];
        if payload_length > 0 {
            self.stream
                .read_exact(&mut payload_bytes)
                .await
                .map_err(|e| NetworkError::ConnectionFailed {
                    address: self.address,
                    reason: format!("Failed to read payload: {}", e),
                })?;
        }

        // Combine header and payload
        let mut message_bytes = Vec::with_capacity(24 + payload_length as usize);
        message_bytes.extend_from_slice(&header_bytes);
        message_bytes.extend_from_slice(&payload_bytes);

        // Parse network message
        let message = NetworkMessage::from_bytes(&message_bytes).map_err(|e| {
            NetworkError::ProtocolViolation {
                peer: self.address,
                violation: format!("Failed to parse message: {}", e),
            }
        })?;

        // Update activity timestamp
        self.update_activity();

        Ok(message)
    }

    /// Closes the connection gracefully
    pub async fn close(&mut self) -> NetworkResult<()> {
        self.state = ConnectionState::Disconnecting;

        // Shutdown the TCP stream
        if let Err(e) = self.stream.shutdown().await {
            tracing::warn!("Failed to shutdown connection to {}: {}", self.address, e);
        }

        self.state = ConnectionState::Disconnected;
        Ok(())
    }

    /// Checks if the connection should be considered stale
    pub fn is_stale(&self, timeout: std::time::Duration) -> bool {
        self.idle_duration() > timeout
    }

    /// Gets connection information for debugging
    pub fn connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            address: self.address,
            state: self.state,
            inbound: self.inbound,
            connected_at: self.connected_at,
            last_activity: self.last_activity,
            node_info: self.node_info.clone(),
        }
    }
}

/// Connection information for debugging and monitoring
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Peer address
    pub address: SocketAddr,

    /// Connection state
    pub state: ConnectionState,

    /// Whether this is an inbound connection
    pub inbound: bool,

    /// When the connection was established
    pub connected_at: std::time::Instant,

    /// Last activity timestamp
    pub last_activity: std::time::Instant,

    /// Peer node information (if available)
    pub node_info: Option<NodeInfo>,
}

impl ConnectionInfo {
    /// Gets the connection duration
    pub fn duration(&self) -> std::time::Duration {
        std::time::Instant::now().duration_since(self.connected_at)
    }

    /// Gets the idle duration
    pub fn idle_time(&self) -> std::time::Duration {
        std::time::Instant::now().duration_since(self.last_activity)
    }

    /// Gets the connection direction as string
    pub fn direction(&self) -> &'static str {
        if self.inbound {
            "Inbound"
        } else {
            "Outbound"
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, PeerInfo};
    use std::net::SocketAddr;
    use tokio::net::{TcpListener, TcpStream};

    #[test]
    fn test_connection_state() {
        assert!(ConnectionState::Connected.is_active());
        assert!(ConnectionState::Ready.is_ready());
        assert!(ConnectionState::Connecting.is_connecting());
        assert!(ConnectionState::Disconnected.is_closed());

        assert!(!ConnectionState::Disconnected.is_active());
        assert!(!ConnectionState::Connected.is_ready());
    }

    #[tokio::test]
    async fn test_peer_connection_creation() {
        // Create a test TCP connection
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("operation should succeed");
        let addr = listener.local_addr().expect("operation should succeed");

        let stream = TcpStream::connect(addr)
            .await
            .expect("operation should succeed");
        let peer_addr = stream.peer_addr().expect("operation should succeed");

        let mut connection = PeerConnection::new(stream, peer_addr, false);

        assert_eq!(connection.state, ConnectionState::Connected);
        assert_eq!(connection.address, peer_addr);
        assert!(!connection.inbound);
        assert!(connection.node_info.is_none());
    }

    #[test]
    fn test_connection_state_transitions() {
        let mut state = ConnectionState::Connecting;
        assert!(state.is_connecting());

        state = ConnectionState::Connected;
        assert!(state.is_active());
        assert!(!state.is_ready());

        state = ConnectionState::Ready;
        assert!(state.is_active());
        assert!(state.is_ready());

        state = ConnectionState::Disconnected;
        assert!(state.is_closed());
        assert!(!state.is_active());
    }
}

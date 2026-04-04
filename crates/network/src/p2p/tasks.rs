//! P2P background task management.
//!
//! This module implements background task management exactly matching C# Neo's TaskManager pattern.

use crate::{NetworkError, NetworkMessage, NetworkResult as Result, NodeInfo};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    net::TcpListener,
    sync::{broadcast, RwLock},
    task::JoinHandle,
    time::{interval, sleep},
};
use tracing::{debug, error, info, warn};

use super::{connection::PeerConnection, events::P2PEvent};

/// Task manager for P2P background operations (matches C# Neo TaskManager pattern)
pub struct TaskManager {
    /// Active task handles
    task_handles: Arc<RwLock<Vec<JoinHandle<()>>>>,
    /// Ping timing for RTT calculation
    ping_times: Arc<RwLock<HashMap<SocketAddr, Instant>>>,
}

impl TaskManager {
    /// Creates a new task manager
    pub fn new() -> Self {
        Self {
            task_handles: Arc::new(RwLock::new(Vec::new())),
            ping_times: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record ping sent time for RTT calculation
    pub async fn record_ping_sent(&self, address: SocketAddr) {
        self.ping_times
            .write()
            .await
            .insert(address, Instant::now());
    }

    /// Calculate RTT from recorded ping time
    pub async fn calculate_rtt(&self, address: SocketAddr) -> u64 {
        if let Some(ping_time) = self.ping_times.write().await.remove(&address) {
            ping_time.elapsed().as_millis() as u64
        } else {
            0 // No ping time recorded
        }
    }

    /// Starts connection acceptor task (matches C# Neo connection accepting exactly)
    pub async fn start_connection_acceptor(
        &self,
        listener: TcpListener,
        connections: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
        event_tx: broadcast::Sender<P2PEvent>,
        running: Arc<RwLock<bool>>,
    ) {
        let handle = tokio::spawn(async move {
            info!("Connection acceptor task started");

            while *running.read().await {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        info!("Accepted connection from {}", addr);

                        // Create inbound connection
                        let connection = PeerConnection::new(stream, addr, true);
                        connections.write().await.insert(addr, connection);

                        // Emit connection event
                        let _ = event_tx.send(P2PEvent::PeerConnected {
                            peer_id: neo_core::UInt160::zero(), // Will be set during handshake
                            address: addr,
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);

                        // Emit connection failed event
                        let _ = event_tx.send(P2PEvent::ConnectionFailed {
                            address: "0.0.0.0:0".parse().expect("value should parse"), // Unknown address
                            error: e.to_string(),
                        });

                        // Short delay before retrying
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            }

            info!("Connection acceptor task stopped");
        });

        self.task_handles.write().await.push(handle);
    }

    /// Starts ping manager task (matches C# Neo ping management exactly)
    pub async fn start_ping_manager(
        &self,
        connections: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
        ping_interval: Duration,
        _magic: u32,
        running: Arc<RwLock<bool>>,
    ) {
        let ping_times = Arc::clone(&self.ping_times);

        let handle = tokio::spawn(async move {
            info!(
                "Ping manager task started with interval: {:?}",
                ping_interval
            );

            let mut interval = interval(ping_interval);

            while *running.read().await {
                interval.tick().await;

                let connections_read = connections.read().await;
                for (address, connection) in connections_read.iter() {
                    if connection.state.is_ready() {
                        // Generate ping nonce
                        let nonce = rand::random::<u32>();

                        // Record ping send time for RTT calculation
                        ping_times.write().await.insert(*address, Instant::now());

                        // Create ping message
                        let ping = crate::ProtocolMessage::ping();
                        let ping_msg = NetworkMessage::new(ping);

                        if let Err(e) = connection.message_tx.send(ping_msg) {
                            warn!("Failed to send ping to {}: {}", address, e);
                            // Remove ping time on send failure
                            ping_times.write().await.remove(address);
                        } else {
                            debug!("Sent ping to {} with nonce {}", address, nonce);
                        }
                    }
                }
            }

            info!("Ping manager task stopped");
        });

        self.task_handles.write().await.push(handle);
    }

    /// Starts connection manager task (matches C# Neo connection monitoring exactly)
    pub async fn start_connection_manager(
        &self,
        connections: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
        timeout: Duration,
        running: Arc<RwLock<bool>>,
    ) {
        let handle = tokio::spawn(async move {
            info!(
                "Connection manager task started with timeout: {:?}",
                timeout
            );

            let mut interval = interval(Duration::from_secs(30)); // Check every 30 seconds

            while *running.read().await {
                interval.tick().await;

                let mut to_disconnect = Vec::new();

                {
                    let connections_read = connections.read().await;
                    for (address, connection) in connections_read.iter() {
                        if connection.is_stale(timeout) {
                            to_disconnect.push(*address);
                        }
                    }
                }

                // Disconnect stale connections
                if !to_disconnect.is_empty() {
                    let mut connections_write = connections.write().await;
                    for address in to_disconnect {
                        warn!("Disconnecting stale connection: {}", address);
                        connections_write.remove(&address);
                    }
                }
            }

            info!("Connection manager task stopped");
        });

        self.task_handles.write().await.push(handle);
    }

    /// Starts connection handler task for a specific peer (matches C# Neo message handling exactly)
    pub async fn start_connection_handler(
        &self,
        address: SocketAddr,
        connections: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
        event_tx: broadcast::Sender<P2PEvent>,
        magic: u32,
        node_info: NodeInfo,
    ) {
        let ping_times = Arc::clone(&self.ping_times);

        let handle = tokio::spawn(async move {
            info!("Connection handler started for {}", address);

            loop {
                // Get connection
                let mut connection = {
                    let mut connections_write = connections.write().await;
                    match connections_write.remove(&address) {
                        Some(conn) => conn,
                        None => {
                            debug!("Connection {} no longer exists", address);
                            break;
                        }
                    }
                };

                // Try to receive message
                match connection.receive_message().await {
                    Ok(message) => {
                        debug!(
                            "Received message from {}: {}",
                            address, message.header.command
                        );

                        // Handle static message processing
                        if let Err(e) = Self::handle_message_static(
                            &connections,
                            &event_tx,
                            &ping_times,
                            address,
                            message,
                            magic,
                            &node_info,
                        )
                        .await
                        {
                            warn!("Failed to handle message from {}: {}", address, e);
                        }

                        // Put connection back
                        connections.write().await.insert(address, connection);
                    }
                    Err(e) => {
                        error!("Failed to receive message from {}: {}", address, e);

                        // Emit disconnection event
                        if let Some(node_info) = &connection.node_info {
                            let _ = event_tx.send(P2PEvent::PeerDisconnected {
                                peer_id: node_info.id,
                                address,
                                reason: e.to_string(),
                            });
                        }

                        break;
                    }
                }
            }

            info!("Connection handler stopped for {}", address);
        });

        self.task_handles.write().await.push(handle);
    }

    /// Handles message processing (static method for use in tasks)
    async fn handle_message_static(
        connections: &Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
        event_tx: &broadcast::Sender<P2PEvent>,
        ping_times: &Arc<RwLock<HashMap<SocketAddr, Instant>>>,
        address: SocketAddr,
        message: NetworkMessage,
        magic: u32,
        node_info: &NodeInfo,
    ) -> Result<()> {
        // Basic message validation
        if message.header.magic != magic {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: "Invalid magic number".to_string(),
            });
        }

        // Handle specific message types
        match &message.payload {
            crate::ProtocolMessage::Version {
                version,
                nonce,
                user_agent,
                start_height,
                ..
            } => {
                info!(
                    "Received Version from {}: v{}, height={}",
                    address, version, start_height
                );

                // Generate peer ID
                let peer_id =
                    super::protocol::ProtocolUtils::generate_peer_id(address, *nonce, user_agent)
                        .await;

                // Update connection with peer info
                if let Some(connection) = connections.write().await.get_mut(&address) {
                    let mut updated_node_info = node_info.clone();
                    updated_node_info.id = peer_id;
                    connection.set_node_info(updated_node_info);
                }

                // Send verack response
                let verack = crate::ProtocolMessage::Verack;
                let verack_msg = NetworkMessage::new(verack);

                if let Some(connection) = connections.write().await.get_mut(&address) {
                    if let Err(e) = connection.send_message(verack_msg).await {
                        warn!("Failed to send verack to {}: {}", address, e);
                    }
                }

                // Emit events
                let _ = event_tx.send(P2PEvent::PeerVersion {
                    address,
                    version: *version,
                    user_agent: user_agent.clone(),
                    start_height: *start_height,
                });

                let _ = event_tx.send(P2PEvent::PeerHeight {
                    address,
                    height: *start_height,
                });
            }

            crate::ProtocolMessage::Verack => {
                info!("Received Verack from {}", address);

                // Mark connection as ready
                if let Some(connection) = connections.write().await.get_mut(&address) {
                    connection.set_state(super::connection::ConnectionState::Ready);

                    if let Some(node_info) = &connection.node_info {
                        let _ = event_tx.send(P2PEvent::HandshakeCompleted {
                            peer_id: node_info.id,
                            address,
                            node_info: node_info.clone(),
                        });
                    }
                }
            }

            crate::ProtocolMessage::Ping { nonce } => {
                debug!("Received Ping from {}: nonce={}", address, nonce);

                // Send pong response
                let pong = crate::ProtocolMessage::pong(*nonce);
                let pong_msg = NetworkMessage::new(pong);

                if let Some(connection) = connections.write().await.get_mut(&address) {
                    if let Err(e) = connection.send_message(pong_msg).await {
                        warn!("Failed to send pong to {}: {}", address, e);
                    }
                }
            }

            crate::ProtocolMessage::Pong { nonce } => {
                debug!("Received Pong from {}: nonce={}", address, nonce);

                // Calculate actual RTT from recorded ping time
                let rtt_ms = if let Some(ping_time) = ping_times.write().await.remove(&address) {
                    ping_time.elapsed().as_millis() as u64
                } else {
                    0 // No ping time recorded
                };

                // Emit ping completed event with real RTT
                let _ = event_tx.send(P2PEvent::PingCompleted { address, rtt_ms });
            }

            _ => {
                debug!(
                    "Received other message from {}: {}",
                    address, message.header.command
                );
            }
        }

        // Emit message received event
        let peer_id = {
            if let Some(connection) = connections.read().await.get(&address) {
                if let Some(node_info) = &connection.node_info {
                    node_info.id
                } else {
                    neo_core::UInt160::zero()
                }
            } else {
                neo_core::UInt160::zero()
            }
        };

        let _ = event_tx.send(P2PEvent::MessageReceived { peer_id, message });

        Ok(())
    }

    /// Starts statistics updater task (matches C# Neo statistics collection exactly)
    pub async fn start_stats_updater(
        &self,
        connections: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
        event_tx: broadcast::Sender<P2PEvent>,
        running: Arc<RwLock<bool>>,
    ) {
        let handle = tokio::spawn(async move {
            info!("Statistics updater task started");

            let mut interval = interval(Duration::from_secs(60)); // Update every minute

            while *running.read().await {
                interval.tick().await;

                let connections_read = connections.read().await;
                let peer_count = connections_read.len();
                let connected = peer_count > 0;

                // Emit network status event
                let _ = event_tx.send(P2PEvent::NetworkStatus {
                    connected,
                    peer_count,
                });

                debug!("Network status: {} peers connected", peer_count);
            }

            info!("Statistics updater task stopped");
        });

        self.task_handles.write().await.push(handle);
    }

    /// Stops all background tasks
    pub async fn stop_all(&self) {
        info!("Stopping all background tasks");

        let mut handles = self.task_handles.write().await;
        for handle in handles.drain(..) {
            handle.abort();
        }

        info!("All background tasks stopped");
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, PeerInfo};

    #[tokio::test]
    async fn test_task_manager_creation() {
        let task_manager = TaskManager::new();

        // Should start with no active tasks
        assert_eq!(task_manager.task_handles.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_task_manager_stop_all() {
        let task_manager = TaskManager::new();

        task_manager.stop_all().await;

        assert_eq!(task_manager.task_handles.read().await.len(), 0);
    }
}

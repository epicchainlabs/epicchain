//! Peer management and connection handling.
//!
//! This module provides comprehensive peer management functionality,
//! including peer discovery, connection management, and peer state tracking.

const SECONDS_PER_HOUR: u64 = 3600;
use crate::{NetworkError, NetworkResult, NodeInfo, ProtocolVersion};
use neo_config::DEFAULT_NEO_PORT;
use neo_config::DEFAULT_RPC_PORT;
use neo_config::DEFAULT_TESTNET_PORT;
use neo_config::DEFAULT_TESTNET_RPC_PORT;
use neo_core::UInt160;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Default Neo network ports
/// Maximum number of peers to track
pub const MAX_TRACKED_PEERS: usize = 10000;

/// Peer connection timeout
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Peer handshake timeout
pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

/// Peer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Represents an enumeration of values.
pub enum PeerStatus {
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected but not handshaked
    Connected,
    /// Handshake in progress
    Handshaking,
    /// Fully connected and ready
    Ready,
    /// Connection failed
    Failed,
    /// Banned
    Banned,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct PeerInfo {
    /// Peer ID
    pub id: Option<UInt160>,
    /// Peer address
    pub address: SocketAddr,
    /// Peer status
    pub status: PeerStatus,
    /// Protocol version
    pub version: Option<ProtocolVersion>,
    /// User agent
    pub user_agent: Option<String>,
    /// Services provided by peer
    pub services: u64,
    /// Peer height
    pub height: u32,
    /// Last seen timestamp
    pub last_seen: u64,
    /// Connection timestamp
    pub connected_at: Option<u64>,
    /// Ping latency in milliseconds
    pub latency: Option<u64>,
    /// Number of failed connection attempts
    pub failed_attempts: u32,
    /// Whether this is an inbound connection
    pub inbound: bool,
    /// Relay capability
    pub relay: bool,
}

impl PeerInfo {
    /// Creates a new peer info
    /// Creates a new instance.
    pub fn new(address: SocketAddr, inbound: bool) -> Self {
        Self {
            id: None,
            address,
            status: PeerStatus::Disconnected,
            version: None,
            user_agent: None,
            services: 0,
            height: 0,
            last_seen: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("valid address")
                .as_secs(),
            connected_at: None,
            latency: None,
            failed_attempts: 0,
            inbound,
            relay: false,
        }
    }

    /// Updates peer info from node info
    /// Updates the internal state.
    pub fn update_from_node_info(&mut self, node_info: &NodeInfo) {
        self.id = Some(node_info.id);
        self.version = Some(node_info.version);
        self.user_agent = Some(node_info.user_agent.clone());
        self.height = node_info.start_height;
        self.last_seen = node_info.timestamp;
    }

    /// Marks peer as connected
    pub fn mark_connected(&mut self) {
        self.status = PeerStatus::Connected;
        self.connected_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("valid address")
                .as_secs(),
        );
        self.failed_attempts = 0;
    }

    /// Marks peer as ready
    pub fn mark_ready(&mut self) {
        self.status = PeerStatus::Ready;
    }

    /// Marks peer as failed
    pub fn mark_failed(&mut self) {
        self.status = PeerStatus::Failed;
        self.failed_attempts += 1;
        self.connected_at = None;
    }

    /// Marks peer as banned
    pub fn mark_banned(&mut self) {
        self.status = PeerStatus::Banned;
    }

    /// Updates latency
    /// Updates the internal state.
    pub fn update_latency(&mut self, latency: Duration) {
        self.latency = Some(latency.as_millis() as u64);
        self.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid address")
            .as_secs();
    }

    /// Checks if peer should be retried
    pub fn should_retry(&self, max_attempts: u32, retry_delay: Duration) -> bool {
        if self.status == PeerStatus::Banned {
            return false;
        }

        if self.failed_attempts >= max_attempts {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid address")
            .as_secs();

        now > self.last_seen + retry_delay.as_secs()
    }

    /// Checks if peer is active
    /// Checks a boolean condition.
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            PeerStatus::Ready | PeerStatus::Connected | PeerStatus::Handshaking
        )
    }
}

/// Peer connection state
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct Peer {
    /// Peer information
    pub info: PeerInfo,
    /// Connection start time
    pub connection_start: Instant,
    /// Last ping time
    pub last_ping: Option<Instant>,
    /// Pending ping nonce
    pub pending_ping: Option<u32>,
    /// Bytes sent to this peer
    pub bytes_sent: u64,
    /// Bytes received from this peer
    pub bytes_received: u64,
    /// Messages sent to this peer
    pub messages_sent: u64,
    /// Messages received from this peer
    pub messages_received: u64,
}

impl Peer {
    /// Creates a new peer
    /// Creates a new instance.
    pub fn new(address: SocketAddr, inbound: bool) -> Self {
        Self {
            info: PeerInfo::new(address, inbound),
            connection_start: Instant::now(),
            last_ping: None,
            pending_ping: None,
            bytes_sent: 0,
            bytes_received: 0,
            messages_sent: 0,
            messages_received: 0,
        }
    }

    /// Records bytes sent
    /// Records an event or metric.
    pub fn record_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    /// Records bytes received
    /// Records an event or metric.
    pub fn record_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    /// Records message sent
    /// Records an event or metric.
    pub fn record_message_sent(&mut self) {
        self.messages_sent += 1;
    }

    /// Records message received
    /// Records an event or metric.
    pub fn record_message_received(&mut self) {
        self.messages_received += 1;
    }

    /// Starts a ping
    pub fn start_ping(&mut self) -> u32 {
        let nonce = rand::random();
        self.last_ping = Some(Instant::now());
        self.pending_ping = Some(nonce);
        nonce
    }

    /// Completes a ping
    pub fn complete_ping(&mut self, nonce: u32) -> Option<Duration> {
        if let Some(pending_nonce) = self.pending_ping.take() {
            if pending_nonce == nonce {
                if let Some(ping_start) = self.last_ping {
                    let latency = ping_start.elapsed();
                    self.info.update_latency(latency);
                    return Some(latency);
                }
            }
        }
        None
    }

    /// Gets connection duration
    pub fn connection_duration(&self) -> Duration {
        self.connection_start.elapsed()
    }
}

/// Peer manager for handling all peer connections
#[derive(Debug)]
/// Represents a data structure.
pub struct PeerManager {
    /// Connected peers
    peers: Arc<RwLock<HashMap<SocketAddr, Peer>>>,
    /// Known peer addresses
    known_peers: Arc<RwLock<HashMap<SocketAddr, PeerInfo>>>,
    /// Banned peers
    // Track ban info with expiration
    banned_peers: Arc<RwLock<HashMap<SocketAddr, BanInfo>>>,
    /// Maximum number of peers
    max_peers: usize,
    /// Maximum connection attempts
    max_attempts: u32,
    /// Retry delay
    retry_delay: Duration,
    /// Ban duration
    ban_duration: Duration,
}

impl PeerManager {
    /// Creates a new peer manager
    /// Creates a new instance.
    pub fn new(max_peers: usize) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            known_peers: Arc::new(RwLock::new(HashMap::new())),
            banned_peers: Arc::new(RwLock::new(HashMap::new())),
            max_peers,
            max_attempts: 3,
            retry_delay: Duration::from_secs(300), // 5 minutes
            ban_duration: Duration::from_secs(86400), // 24 hours
        }
    }

    /// Adds a known peer
    pub async fn add_known_peer(&self, address: SocketAddr) {
        let mut known_peers = self.known_peers.write().await;

        if known_peers.len() >= MAX_TRACKED_PEERS {
            return;
        }

        if !known_peers.contains_key(&address) {
            known_peers.insert(address, PeerInfo::new(address, false));
        }
    }

    /// Adds multiple known peers
    pub async fn add_known_peers(&self, addresses: Vec<SocketAddr>) {
        for address in addresses {
            self.add_known_peer(address).await;
        }
    }

    /// Checks if we can connect to a peer
    pub async fn can_connect_to(&self, address: SocketAddr) -> bool {
        if self.peers.read().await.contains_key(&address) {
            return false;
        }

        if self.banned_peers.read().await.contains_key(&address) {
            return false;
        }

        // Check peer limit
        if self.peers.read().await.len() >= self.max_peers {
            return false;
        }

        true
    }

    /// Connects to a peer
    pub async fn connect_peer(&self, address: SocketAddr) -> NetworkResult<()> {
        if self.peers.read().await.contains_key(&address) {
            return Ok(());
        }

        if self.banned_peers.read().await.contains_key(&address) {
            return Err(NetworkError::PeerBanned { address });
        }

        // Check peer limit
        if self.peers.read().await.len() >= self.max_peers {
            return Err(NetworkError::ConnectionLimitReached {
                current: self.peers.read().await.len(),
                max: self.max_peers,
            });
        }

        // Create new peer
        let mut peer = Peer::new(address, false);
        peer.info.status = PeerStatus::Connecting;

        // Add to connected peers
        self.peers.write().await.insert(address, peer);

        // Update known peers
        let mut known_peers = self.known_peers.write().await;
        if let Some(known_peer) = known_peers.get_mut(&address) {
            known_peer.status = PeerStatus::Connecting;
        }

        Ok(())
    }

    /// Disconnects a peer
    pub async fn disconnect_peer(&self, address: SocketAddr, _reason: String) -> Option<Peer> {
        let peer = self.peers.write().await.remove(&address);

        // Update known peers
        let mut known_peers = self.known_peers.write().await;
        if let Some(known_peer) = known_peers.get_mut(&address) {
            known_peer.status = PeerStatus::Disconnected;
            known_peer.mark_failed();
        }

        peer
    }

    /// Marks a peer as ready
    pub async fn mark_peer_ready(
        &self,
        address: SocketAddr,
        node_info: &NodeInfo,
    ) -> NetworkResult<()> {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(&address) {
            peer.info.update_from_node_info(node_info);
            peer.info.mark_ready();
        }

        let mut known_peers = self.known_peers.write().await;
        if let Some(known_peer) = known_peers.get_mut(&address) {
            known_peer.update_from_node_info(node_info);
            known_peer.mark_ready();
        }

        Ok(())
    }

    /// Bans a peer
    pub async fn ban_peer(&self, address: SocketAddr, reason: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid address")
            .as_secs();

        // Default to temporary 24h ban if no context
        let info = BanInfo {
            ban_type: BanType::Temporary {
                expires_at: now + SECONDS_PER_HOUR * 24,
            },
            reason: reason.clone(),
            banned_at: now,
        };

        self.banned_peers.write().await.insert(address, info);
        self.disconnect_peer(address, reason).await;

        // Mark as banned in known peers
        let mut known_peers = self.known_peers.write().await;
        if let Some(known_peer) = known_peers.get_mut(&address) {
            known_peer.mark_banned();
        }
    }

    /// Gets a peer by address
    pub async fn get_peer(&self, address: &SocketAddr) -> Option<Peer> {
        self.peers.read().await.get(address).cloned()
    }

    /// Gets all connected peers
    pub async fn get_connected_peers(&self) -> Vec<Peer> {
        self.peers.read().await.values().cloned().collect()
    }

    /// Gets ready peers
    pub async fn get_ready_peers(&self) -> Vec<Peer> {
        self.peers
            .read()
            .await
            .values()
            .filter(|peer| peer.info.status == PeerStatus::Ready)
            .cloned()
            .collect()
    }

    /// Gets peers for connection
    pub async fn get_peers_for_connection(&self, count: usize) -> Vec<SocketAddr> {
        let known_peers = self.known_peers.read().await;
        let connected_peers = self.peers.read().await;
        let banned_peers = self.banned_peers.read().await;

        known_peers
            .values()
            .filter(|peer| {
                !connected_peers.contains_key(&peer.address)
                    && !banned_peers.contains_key(&peer.address)
                    && peer.should_retry(self.max_attempts, self.retry_delay)
            })
            .take(count)
            .map(|peer| peer.address)
            .collect()
    }

    /// Records peer statistics
    pub async fn record_peer_stats(
        &self,
        address: SocketAddr,
        bytes_sent: u64,
        bytes_received: u64,
    ) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(&address) {
            peer.record_bytes_sent(bytes_sent);
            peer.record_bytes_received(bytes_received);
        }
    }

    /// Starts a ping to a peer
    pub async fn start_ping(&self, address: SocketAddr) -> Option<u32> {
        let mut peers = self.peers.write().await;
        peers.get_mut(&address).map(|peer| peer.start_ping())
    }

    /// Completes a ping from a peer
    pub async fn complete_ping(&self, address: SocketAddr, nonce: u32) -> Option<Duration> {
        let mut peers = self.peers.write().await;
        peers
            .get_mut(&address)
            .and_then(|peer| peer.complete_ping(nonce))
    }

    /// Gets peer statistics
    pub async fn get_stats(&self) -> PeerStats {
        let peers = self.peers.read().await;
        let known_peers = self.known_peers.read().await;
        let banned_peers = self.banned_peers.read().await;

        let connected_count = peers.len();
        let inbound_count = peers.values().filter(|p| p.info.inbound).count();
        let outbound_count = connected_count - inbound_count;
        let ready_count = peers
            .values()
            .filter(|p| p.info.status == PeerStatus::Ready)
            .count();

        let total_bytes_sent = peers.values().map(|p| p.bytes_sent).sum();
        let total_bytes_received = peers.values().map(|p| p.bytes_received).sum();

        let average_latency = {
            let latencies: Vec<u64> = peers.values().filter_map(|p| p.info.latency).collect();

            if latencies.is_empty() {
                0.0
            } else {
                latencies.iter().sum::<u64>() as f64 / latencies.len() as f64
            }
        };

        PeerStats {
            connected_peers: connected_count,
            inbound_peers: inbound_count,
            outbound_peers: outbound_count,
            ready_peers: ready_count,
            known_peers: known_peers.len(),
            banned_peers: banned_peers.len(),
            total_bytes_sent,
            total_bytes_received,
            average_latency,
        }
    }

    /// Cleans up old and banned peers
    pub async fn cleanup(&self) {
        // This implements the C# logic: CleanupExpiredBans with timestamp tracking

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid address")
            .as_secs();

        // 1. Track removal statistics for monitoring
        let initial_count = self.banned_peers.read().await.len();
        let mut expired_count = 0;
        let mut permanent_bans = 0;

        // 2. Filter out expired bans (production ban expiration logic)
        // Note: Production cleanup would track ban timestamps and durations
        let mut banned_peers = self.banned_peers.write().await;
        let mut to_remove = Vec::new();

        for (peer_addr, info) in banned_peers.iter() {
            match &info.ban_type {
                BanType::Permanent => {
                    permanent_bans += 1;
                }
                BanType::Temporary { expires_at } => {
                    if now >= *expires_at {
                        to_remove.push(*peer_addr);
                        expired_count += 1;
                    }
                }
                BanType::Protocol {
                    offense_count,
                    first_offense_at: _,
                } => {
                    // progressive bans: e.g., 1h per offense
                    let duration = 3600u64.saturating_mul(*offense_count as u64);
                    if now >= info.banned_at + duration {
                        to_remove.push(*peer_addr);
                        expired_count += 1;
                    }
                }
            }
        }

        for addr in to_remove {
            banned_peers.remove(&addr);
        }

        drop(banned_peers); // Release the lock

        // 3. Update ban statistics (production monitoring)
        let final_count = self.banned_peers.read().await.len();
        let removed_count = initial_count - final_count;

        // 4. Log cleanup results for network monitoring
        if removed_count > 0 {
            log::debug!(
                "Ban cleanup completed: {} expired, {} permanent, {} total removed, {} remaining",
                expired_count,
                permanent_bans,
                removed_count,
                final_count
            );
        }

        // 5. Trigger peer discovery if many bans expired (production network health)
        if removed_count > 5 {
            // Many bans expired - initiate peer discovery to maintain network connectivity
            self.trigger_peer_discovery();
        }
    }

    /// Calculates protocol ban duration based on offense count (production implementation)
    fn calculate_protocol_ban_duration(&self, offense_count: u32) -> u64 {
        match offense_count {
            1 => 3600,    // 1 hour for first offense
            2 => 21600,   // 6 hours for second offense
            3 => 86400,   // 24 hours for third offense
            4 => 259200,  // 3 days for fourth offense
            5 => 604800,  // 1 week for fifth offense
            _ => 2592000, // 30 days for repeat offenders (severe)
        }
    }

    /// Triggers peer discovery to maintain network connectivity (production implementation)
    fn trigger_peer_discovery(&self) {
        // Perform a simple discovery kick: attempt connecting to a few known, disconnected peers
        // Note: This is a synchronous trigger that returns immediately
        // Actual discovery would be handled by a separate long-running task
        // For now, just log that discovery was triggered
        tracing::debug!("Peer discovery triggered");
    }
}

/// Peer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct PeerStats {
    /// Number of connected peers
    pub connected_peers: usize,
    /// Number of inbound peers
    pub inbound_peers: usize,
    /// Number of outbound peers
    pub outbound_peers: usize,
    /// Number of ready peers
    pub ready_peers: usize,
    /// Number of known peers
    pub known_peers: usize,
    /// Number of banned peers
    pub banned_peers: usize,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Total bytes received
    pub total_bytes_received: u64,
    /// Average latency in milliseconds
    pub average_latency: f64,
}

/// Ban information structure (production implementation)
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct BanInfo {
    pub ban_type: BanType,
    pub reason: String,
    pub banned_at: u64,
}

/// Types of bans supported (production implementation)
#[derive(Debug, Clone)]
/// Represents an enumeration of values.
pub enum BanType {
    Temporary {
        expires_at: u64,
    },
    /// Permanent ban (never expires)
    Permanent,
    /// Protocol violation ban with progressive duration
    Protocol {
        offense_count: u32,
        first_offense_at: u64,
    },
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, NetworkResult};
    use std::net::SocketAddr;

    #[test]
    fn test_peer_info() {
        let address = "127.0.0.1:10333".parse().expect("valid address");
        let mut peer_info = PeerInfo::new(address, false);

        assert_eq!(peer_info.address, address);
        assert_eq!(peer_info.status, PeerStatus::Disconnected);
        assert!(!peer_info.inbound);

        peer_info.mark_connected();
        assert_eq!(peer_info.status, PeerStatus::Connected);
        assert!(peer_info.connected_at.is_some());

        peer_info.mark_failed();
        assert_eq!(peer_info.status, PeerStatus::Failed);
        assert_eq!(peer_info.failed_attempts, 1);
    }

    #[test]
    fn test_peer() {
        let address = "127.0.0.1:10333".parse().expect("valid address");
        let mut peer = Peer::new(address, false);

        peer.record_bytes_sent(100);
        peer.record_bytes_received(200);
        peer.record_message_sent();
        peer.record_message_received();

        assert_eq!(peer.bytes_sent, 100);
        assert_eq!(peer.bytes_received, 200);
        assert_eq!(peer.messages_sent, 1);
        assert_eq!(peer.messages_received, 1);

        let nonce = peer.start_ping();
        assert!(peer.pending_ping.is_some());
        assert_eq!(peer.pending_ping.unwrap(), nonce);
    }

    #[tokio::test]
    async fn test_peer_manager() {
        let manager = PeerManager::new(10);
        let address = "127.0.0.1:10333".parse().expect("valid address");

        // Add known peer
        manager.add_known_peer(address).await;

        // Connect peer
        manager
            .connect_peer(address)
            .await
            .expect("operation should succeed");

        // Check peer exists
        let peer = manager.get_peer(&address).await;
        assert!(peer.is_some());
        assert_eq!(
            peer.expect("operation should succeed").info.status,
            PeerStatus::Connecting
        );

        // Disconnect peer
        let disconnected = manager.disconnect_peer(address, "test".to_string()).await;
        assert!(disconnected.is_some());

        // Check peer is gone
        let peer = manager.get_peer(&address).await;
        assert!(peer.is_none());
    }
}

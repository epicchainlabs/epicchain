//! Advanced peer management and discovery system
//!
//! This module provides enhanced peer discovery, connection quality monitoring,
//! and adaptive peer management for optimal network performance.

use neo_config::HASH_SIZE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{debug, info, warn};

/// Test peer IP for examples
const TEST_PEER_IP: &str = "test-peer.local";
/// Test network range for examples
const TEST_NETWORK_RANGE: &str = "test-network.local";

/// Default Neo network ports
const DEFAULT_NEO_PORT: &str = "10333";
const DEFAULT_RPC_PORT: &str = "10332";
const DEFAULT_TESTNET_PORT: &str = "20333";
const DEFAULT_TESTNET_RPC_PORT: &str = "20332";

/// Peer quality metrics and connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerQuality {
    /// Socket address of the peer
    pub address: SocketAddr,
    /// Last successful connection timestamp
    pub last_connected: Option<SystemTime>,
    /// Number of successful connections
    pub successful_connections: u32,
    /// Number of failed connection attempts
    pub failed_connections: u32,
    /// Average response time in milliseconds
    pub avg_response_time: Option<u64>,
    /// Peer reliability score (0.0 to 1.0)
    pub reliability_score: f64,
    /// Whether this peer is currently banned
    pub is_banned: bool,
    /// Timestamp when ban expires (if banned)
    pub ban_expires: Option<SystemTime>,
    /// Geographic region hint (for diversity)
    pub region_hint: Option<String>,
    /// Protocol version supported by peer
    pub protocol_version: Option<String>,
    /// Services provided by this peer
    pub services: PeerServices,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerServices {
    /// Supports full node services
    pub full_node: bool,
    /// Supports blockchain pruning
    pub pruned: bool,
    /// Supports witness services
    pub witness: bool,
    /// Supports RPC services
    pub rpc: bool,
}

impl PeerQuality {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            last_connected: None,
            successful_connections: 0,
            failed_connections: 0,
            avg_response_time: None,
            reliability_score: 0.5, // Start neutral
            is_banned: false,
            ban_expires: None,
            region_hint: None,
            protocol_version: None,
            services: PeerServices::default(),
        }
    }

    /// Update peer quality after a successful connection
    pub fn record_successful_connection(&mut self, response_time_ms: Option<u64>) {
        self.last_connected = Some(SystemTime::now());
        self.successful_connections += 1;

        if let Some(response_time) = response_time_ms {
            self.avg_response_time = Some(match self.avg_response_time {
                Some(existing) => (existing + response_time) / 2,
                None => response_time,
            });
        }

        self.update_reliability_score();
    }

    /// Update peer quality after a failed connection
    pub fn record_failed_connection(&mut self) {
        self.failed_connections += 1;
        self.update_reliability_score();
    }

    /// Calculate reliability score based on connection history
    fn update_reliability_score(&mut self) {
        let total_attempts = self.successful_connections + self.failed_connections;
        if total_attempts == 0 {
            self.reliability_score = 0.5;
            return;
        }

        let success_rate = self.successful_connections as f64 / total_attempts as f64;

        let response_penalty = match self.avg_response_time {
            Some(time) if time > 5000 => 0.1, // Penalty for slow responses
            Some(time) if time > 2000 => 0.05,
            _ => 0.0,
        };

        // Factor in recency - older connections are less valuable
        let recency_bonus = match self.last_connected {
            Some(last) => {
                let age = SystemTime::now().duration_since(last).unwrap_or_default();
                if age < Duration::from_secs(300) {
                    0.1
                }
                // 5 minutes
                else if age < Duration::from_secs(3600) {
                    0.05
                } else {
                    0.0
                }
            }
            None => 0.0,
        };

        self.reliability_score = (success_rate - response_penalty + recency_bonus).clamp(0.0, 1.0);
    }

    /// Check if peer is currently banned
    pub fn is_currently_banned(&self) -> bool {
        if !self.is_banned {
            return false;
        }

        match self.ban_expires {
            Some(expires) => SystemTime::now() < expires,
            None => true, // Permanent ban
        }
    }

    /// Ban this peer for a specified duration
    pub fn ban(&mut self, duration: Option<Duration>) {
        self.is_banned = true;
        self.ban_expires = duration.map(|d| SystemTime::now() + d);
    }

    /// Unban this peer
    pub fn unban(&mut self) {
        self.is_banned = false;
        self.ban_expires = None;
    }
}

/// Advanced peer discovery and management system
pub struct PeerManager {
    /// Known peer quality information
    peers: Arc<RwLock<HashMap<SocketAddr, PeerQuality>>>,
    /// Seed nodes for initial discovery
    seed_nodes: Vec<SocketAddr>,
    /// Maximum number of peers to track
    max_tracked_peers: usize,
    /// Banned IP ranges (for security)
    banned_ranges: Arc<RwLock<Vec<(IpAddr, u8)>>>, // IP, prefix length
    /// Network type for peer validation
    network_type: neo_config::NetworkType,
}

impl PeerManager {
    /// Create a new peer manager
    pub fn new(
        seed_nodes: Vec<SocketAddr>,
        network_type: neo_config::NetworkType,
        max_tracked_peers: Option<usize>,
    ) -> Self {
        let mut peers = HashMap::new();

        // Initialize seed nodes with high initial reliability
        for seed in &seed_nodes {
            let mut peer = PeerQuality::new(*seed);
            peer.reliability_score = 0.8; // Seed nodes start with higher trust
            peers.insert(*seed, peer);
        }

        Self {
            peers: Arc::new(RwLock::new(peers)),
            seed_nodes,
            max_tracked_peers: max_tracked_peers.unwrap_or(500),
            banned_ranges: Arc::new(RwLock::new(Vec::new())),
            network_type,
        }
    }

    /// Get the best peers for connection attempts
    pub async fn get_best_peers(&self, count: usize) -> Vec<SocketAddr> {
        let peers = self.peers.read().await;
        let mut peer_list: Vec<_> = peers
            .values()
            .filter(|p| !p.is_currently_banned())
            .collect();

        peer_list.sort_by(|a, b| {
            b.reliability_score
                .partial_cmp(&a.reliability_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Add some diversity - don't just pick the top rated ones
        let mut result = Vec::new();
        let high_quality_count = (count * 2) / 3; // 2/3 high quality peers
        let diverse_count = count - high_quality_count;

        // Add high quality peers
        for peer in peer_list.iter().take(high_quality_count) {
            result.push(peer.address);
        }

        let remaining_peers: Vec<_> = peer_list
            .iter()
            .skip(high_quality_count)
            .take(diverse_count * 2) // Take more than needed for selection
            .collect();

        for peer in remaining_peers.iter().take(diverse_count) {
            if !result.contains(&peer.address) {
                result.push(peer.address);
            }
        }

        result.truncate(count);
        result
    }

    /// Record a successful connection to a peer
    pub async fn record_successful_connection(
        &self,
        address: SocketAddr,
        response_time_ms: Option<u64>,
    ) {
        let mut peers = self.peers.write().await;
        let peer = peers
            .entry(address)
            .or_insert_with(|| PeerQuality::new(address));
        peer.record_successful_connection(response_time_ms);

        debug!(
            "Recorded successful connection to {}, reliability: {:.2}",
            address, peer.reliability_score
        );
    }

    /// Record a failed connection to a peer
    pub async fn record_failed_connection(&self, address: SocketAddr) {
        let mut peers = self.peers.write().await;
        let peer = peers
            .entry(address)
            .or_insert_with(|| PeerQuality::new(address));
        peer.record_failed_connection();

        debug!(
            "Recorded failed connection to {}, reliability: {:.2}",
            address, peer.reliability_score
        );

        // Auto-ban peers with very poor reliability
        if peer.reliability_score < 0.1 && peer.failed_connections >= 5 {
            peer.ban(Some(Duration::from_secs(3600))); // 1 hour ban
            warn!("Auto-banned peer {} due to poor reliability", address);
        }
    }

    /// Add newly discovered peers
    pub async fn add_discovered_peers(&self, new_peers: &[SocketAddr]) {
        let mut peers = self.peers.write().await;
        let mut added_count = 0;

        for &address in new_peers {
            if peers.contains_key(&address) {
                continue;
            }

            if self.is_ip_banned(address.ip()).await {
                continue;
            }

            // Enforce max tracked peers limit
            if peers.len() >= self.max_tracked_peers {
                // Remove lowest quality peer to make room
                if let Some(worst_peer) = peers
                    .values()
                    .min_by(|a, b| {
                        a.reliability_score
                            .partial_cmp(&b.reliability_score)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|p| p.address)
                {
                    peers.remove(&worst_peer);
                }
            }

            peers.insert(address, PeerQuality::new(address));
            added_count += 1;
        }

        if added_count > 0 {
            info!(
                "Added {} new peers to tracking (total: {})",
                added_count,
                peers.len()
            );
        }
    }

    /// Ban a peer for misbehavior
    pub async fn ban_peer(&self, address: SocketAddr, duration: Option<Duration>, reason: &str) {
        let mut peers = self.peers.write().await;
        let peer = peers
            .entry(address)
            .or_insert_with(|| PeerQuality::new(address));
        peer.ban(duration);

        info!("Banned peer {} for {}: {:?}", address, reason, duration);
    }

    /// Ban an IP range
    pub async fn ban_ip_range(&self, ip: IpAddr, prefix_len: u8) {
        let mut banned = self.banned_ranges.write().await;
        banned.push((ip, prefix_len));
        info!("Banned IP range {}/{}", ip, prefix_len);
    }

    /// Check if an IP is banned
    async fn is_ip_banned(&self, ip: IpAddr) -> bool {
        let banned = self.banned_ranges.read().await;
        for (banned_ip, prefix_len) in banned.iter() {
            if ip_in_range(ip, *banned_ip, *prefix_len) {
                return true;
            }
        }
        false
    }

    /// Get peer statistics
    pub async fn get_peer_statistics(&self) -> PeerStatistics {
        let peers = self.peers.read().await;
        let total_peers = peers.len();
        let banned_peers = peers.values().filter(|p| p.is_currently_banned()).count();
        let high_quality_peers = peers.values().filter(|p| p.reliability_score > 0.7).count();
        let connected_recently = peers
            .values()
            .filter(|p| {
                p.last_connected.map_or(false, |t| {
                    SystemTime::now().duration_since(t).unwrap_or_default()
                        < Duration::from_secs(300)
                })
            })
            .count();

        let avg_reliability = if total_peers > 0 {
            peers.values().map(|p| p.reliability_score).sum::<f64>() / total_peers as f64
        } else {
            0.0
        };

        PeerStatistics {
            total_peers,
            banned_peers,
            high_quality_peers,
            connected_recently,
            avg_reliability,
            seed_nodes: self.seed_nodes.len(),
        }
    }

    /// Start background peer management tasks
    pub async fn start_background_tasks(&self) {
        let peers_clone = self.peers.clone();

        // Cleanup task - remove very old, poor quality peers
        tokio::spawn(async move {
            let mut cleanup_interval = interval(Duration::from_secs(300)); // Every 5 minutes

            loop {
                cleanup_interval.tick().await;

                let mut peers = peers_clone.write().await;
                let mut to_remove = Vec::new();

                for (addr, peer) in peers.iter() {
                    // Remove peers that haven't connected in 24 hours and have poor reliability
                    if peer.reliability_score < 0.2
                        && peer.last_connected.map_or(true, |t| {
                            SystemTime::now().duration_since(t).unwrap_or_default()
                                > Duration::from_secs(86400)
                        })
                    {
                        to_remove.push(*addr);
                    }
                }

                for addr in to_remove {
                    peers.remove(&addr);
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct PeerStatistics {
    pub total_peers: usize,
    pub banned_peers: usize,
    pub high_quality_peers: usize,
    pub connected_recently: usize,
    pub avg_reliability: f64,
    pub seed_nodes: usize,
}

/// Check if an IP address is within a given range
fn ip_in_range(ip: IpAddr, range_ip: IpAddr, prefix_len: u8) -> bool {
    match (ip, range_ip) {
        (IpAddr::V4(ip), IpAddr::V4(range_ip)) => {
            let ip_int = u32::from(ip);
            let range_int = u32::from(range_ip);
            let mask = !((1u32 << (32 - prefix_len)) - 1);
            (ip_int & mask) == (range_int & mask)
        }
        (IpAddr::V6(ip), IpAddr::V6(range_ip)) => {
            let ip_int = u128::from(ip);
            let range_int = u128::from(range_ip);
            let mask = !((1u128 << (128 - prefix_len)) - 1);
            (ip_int & mask) == (range_int & mask)
        }
        _ => false, // Different IP versions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::time::Duration;

    #[test]
    fn test_peer_quality_updates() {
        let addr: SocketAddr = "127.0.0.1:10333".parse().unwrap();
        let mut peer = PeerQuality::new(addr);

        // Record successful connections
        peer.record_successful_connection(Some(100));
        assert!(peer.reliability_score > 0.5);

        // Record failures
        peer.record_failed_connection();
        assert!(peer.reliability_score < 0.7);
    }

    #[test]
    fn test_ip_range_checking() {
        let ip: IpAddr = "192.168.1.5".parse().unwrap();
        let range_ip: IpAddr = "192.168.1.0".parse().unwrap();

        assert!(ip_in_range(ip, range_ip, 24));
        // With /28, 192.168.1.5 is within 192.168.1.0/28 (0-15), so use an out-of-range IP
        let ip2: IpAddr = "192.168.1.20".parse().unwrap();
        assert!(!ip_in_range(ip2, range_ip, 28));
    }
}

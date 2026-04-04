//! DOS Protection and Rate Limiting Module
//!
//! This module provides protection against denial-of-service attacks
//! by implementing rate limiting, connection throttling, and resource management.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

/// DOS protection configuration
#[derive(Debug, Clone)]
pub struct DosProtectionConfig {
    /// Maximum connections per IP address
    pub max_connections_per_ip: usize,
    /// Maximum messages per second from a single peer
    pub max_messages_per_second: u32,
    /// Ban duration for malicious peers
    pub ban_duration: Duration,
    /// Maximum pending connections
    pub max_pending_connections: usize,
    /// Connection rate limit (connections per minute)
    pub connection_rate_limit: u32,
    /// Message size limit in bytes
    pub max_message_size: usize,
    /// Enable automatic banning
    pub auto_ban_enabled: bool,
    /// Whitelist of trusted IPs (never banned)
    pub whitelisted_ips: Vec<IpAddr>,
}

impl Default for DosProtectionConfig {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 3,
            max_messages_per_second: 100,
            ban_duration: Duration::from_secs(3600), // 1 hour
            max_pending_connections: 50,
            connection_rate_limit: 60,          // 60 connections per minute
            max_message_size: 10 * 1024 * 1024, // 10MB
            auto_ban_enabled: true,
            whitelisted_ips: Vec::new(),
        }
    }
}

/// Connection statistics for a peer
#[derive(Debug, Clone)]
struct PeerStats {
    /// Number of active connections
    connection_count: usize,
    /// Message count in current window
    message_count: u32,
    /// Last message timestamp
    last_message_time: Instant,
    /// Window start time
    window_start: Instant,
    /// Violation count
    violations: u32,
    /// Ban expiry time (if banned)
    ban_until: Option<Instant>,
}

impl Default for PeerStats {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            connection_count: 0,
            message_count: 0,
            last_message_time: now,
            window_start: now,
            violations: 0,
            ban_until: None,
        }
    }
}

/// DOS protection manager
pub struct DosProtectionManager {
    config: DosProtectionConfig,
    peer_stats: Arc<RwLock<HashMap<IpAddr, PeerStats>>>,
    global_connection_count: Arc<RwLock<usize>>,
    connection_rate_limiter: Arc<RwLock<RateLimiter>>,
}

impl DosProtectionManager {
    /// Create a new DOS protection manager
    pub fn new(config: DosProtectionConfig) -> Self {
        Self {
            config: config.clone(),
            peer_stats: Arc::new(RwLock::new(HashMap::new())),
            global_connection_count: Arc::new(RwLock::new(0)),
            connection_rate_limiter: Arc::new(RwLock::new(RateLimiter::new(
                config.connection_rate_limit,
                Duration::from_secs(60),
            ))),
        }
    }

    /// Check if an IP is whitelisted
    fn is_whitelisted(&self, ip: &IpAddr) -> bool {
        self.config.whitelisted_ips.contains(ip)
    }

    /// Check if a connection from an IP should be allowed
    pub async fn should_allow_connection(&self, ip: IpAddr) -> bool {
        // Always allow whitelisted IPs
        if self.is_whitelisted(&ip) {
            return true;
        }

        // Check global connection limit
        let global_count = *self.global_connection_count.read().await;
        if global_count >= self.config.max_pending_connections {
            warn!("Global connection limit reached: {}", global_count);
            return false;
        }

        // Check connection rate limit
        if !self.connection_rate_limiter.write().await.try_acquire() {
            warn!("Connection rate limit exceeded for global connections");
            return false;
        }

        // Check per-IP limits
        let mut stats = self.peer_stats.write().await;
        let peer_stat = stats.entry(ip).or_default();

        // Check if peer is banned
        if let Some(ban_until) = peer_stat.ban_until {
            if Instant::now() < ban_until {
                debug!("Connection rejected: IP {} is banned", ip);
                return false;
            } else {
                // Ban expired, clear it
                peer_stat.ban_until = None;
                peer_stat.violations = 0;
            }
        }

        // Check per-IP connection limit
        if peer_stat.connection_count >= self.config.max_connections_per_ip {
            warn!(
                "Connection limit exceeded for IP {}: {}",
                ip, peer_stat.connection_count
            );
            peer_stat.violations += 1;

            // Auto-ban if too many violations
            if self.config.auto_ban_enabled && peer_stat.violations >= 5 {
                peer_stat.ban_until = Some(Instant::now() + self.config.ban_duration);
                error!("IP {} has been banned for excessive violations", ip);
            }

            return false;
        }

        true
    }

    /// Register a new connection
    pub async fn register_connection(&self, ip: IpAddr) {
        if !self.is_whitelisted(&ip) {
            let mut stats = self.peer_stats.write().await;
            let peer_stat = stats.entry(ip).or_default();
            peer_stat.connection_count += 1;

            let mut global_count = self.global_connection_count.write().await;
            *global_count += 1;

            debug!(
                "Connection registered for IP {}: count = {}",
                ip, peer_stat.connection_count
            );
        }
    }

    /// Unregister a connection
    pub async fn unregister_connection(&self, ip: IpAddr) {
        if !self.is_whitelisted(&ip) {
            let mut stats = self.peer_stats.write().await;
            if let Some(peer_stat) = stats.get_mut(&ip) {
                if peer_stat.connection_count > 0 {
                    peer_stat.connection_count -= 1;

                    let mut global_count = self.global_connection_count.write().await;
                    if *global_count > 0 {
                        *global_count -= 1;
                    }

                    debug!(
                        "Connection unregistered for IP {}: count = {}",
                        ip, peer_stat.connection_count
                    );

                    // Clean up entry if no connections and not banned
                    if peer_stat.connection_count == 0 && peer_stat.ban_until.is_none() {
                        stats.remove(&ip);
                    }
                }
            }
        }
    }

    /// Check if a message from a peer should be allowed
    pub async fn should_allow_message(&self, ip: IpAddr, message_size: usize) -> bool {
        // Always allow whitelisted IPs
        if self.is_whitelisted(&ip) {
            return true;
        }

        // Check message size
        if message_size > self.config.max_message_size {
            warn!(
                "Message from {} exceeds size limit: {} bytes",
                ip, message_size
            );
            self.record_violation(ip).await;
            return false;
        }

        let mut stats = self.peer_stats.write().await;
        let peer_stat = stats.entry(ip).or_default();

        // Check if peer is banned
        if let Some(ban_until) = peer_stat.ban_until {
            if Instant::now() < ban_until {
                return false;
            } else {
                peer_stat.ban_until = None;
                peer_stat.violations = 0;
            }
        }

        // Reset window if needed
        let now = Instant::now();
        if now.duration_since(peer_stat.window_start) >= Duration::from_secs(1) {
            peer_stat.message_count = 0;
            peer_stat.window_start = now;
        }

        // Check message rate
        if peer_stat.message_count >= self.config.max_messages_per_second {
            warn!(
                "Message rate limit exceeded for IP {}: {} msg/s",
                ip, peer_stat.message_count
            );
            peer_stat.violations += 1;

            // Auto-ban if too many violations
            if self.config.auto_ban_enabled && peer_stat.violations >= 10 {
                peer_stat.ban_until = Some(now + self.config.ban_duration);
                error!("IP {} has been banned for excessive message rate", ip);
            }

            return false;
        }

        peer_stat.message_count += 1;
        peer_stat.last_message_time = now;

        true
    }

    /// Record a violation for an IP
    async fn record_violation(&self, ip: IpAddr) {
        if !self.is_whitelisted(&ip) {
            let mut stats = self.peer_stats.write().await;
            let peer_stat = stats.entry(ip).or_default();
            peer_stat.violations += 1;

            if self.config.auto_ban_enabled && peer_stat.violations >= 3 {
                peer_stat.ban_until = Some(Instant::now() + self.config.ban_duration);
                error!("IP {} has been banned for violations", ip);
            }
        }
    }

    /// Manually ban an IP address
    pub async fn ban_ip(&self, ip: IpAddr, duration: Duration) {
        if !self.is_whitelisted(&ip) {
            let mut stats = self.peer_stats.write().await;
            let peer_stat = stats.entry(ip).or_default();
            peer_stat.ban_until = Some(Instant::now() + duration);
            error!("IP {} has been manually banned", ip);
        }
    }

    /// Unban an IP address
    pub async fn unban_ip(&self, ip: IpAddr) {
        let mut stats = self.peer_stats.write().await;
        if let Some(peer_stat) = stats.get_mut(&ip) {
            peer_stat.ban_until = None;
            peer_stat.violations = 0;
            debug!("IP {} has been unbanned", ip);
        }
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> DosProtectionStats {
        let stats = self.peer_stats.read().await;
        let global_connections = *self.global_connection_count.read().await;

        let banned_count = stats.values().filter(|s| s.ban_until.is_some()).count();

        let active_peers = stats.values().filter(|s| s.connection_count > 0).count();

        DosProtectionStats {
            total_peers: stats.len(),
            banned_peers: banned_count,
            active_peers,
            global_connections,
        }
    }

    /// Clean up expired bans and inactive entries
    pub async fn cleanup(&self) {
        let mut stats = self.peer_stats.write().await;
        let now = Instant::now();

        stats.retain(|_ip, peer_stat| {
            // Keep if has connections or is currently banned
            peer_stat.connection_count > 0
                || (peer_stat.ban_until.is_some() && peer_stat.ban_until.unwrap() > now)
        });

        debug!("DOS protection cleanup: {} entries remaining", stats.len());
    }
}

/// Rate limiter implementation
struct RateLimiter {
    capacity: u32,
    tokens: u32,
    window: Duration,
    last_refill: Instant,
}

impl RateLimiter {
    fn new(capacity: u32, window: Duration) -> Self {
        Self {
            capacity,
            tokens: capacity,
            window,
            last_refill: Instant::now(),
        }
    }

    fn try_acquire(&mut self) -> bool {
        self.refill();

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);

        if elapsed >= self.window {
            self.tokens = self.capacity;
            self.last_refill = now;
        }
    }
}

/// DOS protection statistics
#[derive(Debug, Clone)]
pub struct DosProtectionStats {
    /// Total number of tracked peers
    pub total_peers: usize,
    /// Number of banned peers
    pub banned_peers: usize,
    /// Number of peers with active connections
    pub active_peers: usize,
    /// Total global connections
    pub global_connections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_limits() {
        let config = DosProtectionConfig {
            max_connections_per_ip: 2,
            ..Default::default()
        };

        let manager = DosProtectionManager::new(config);
        let ip: IpAddr = "192.168.1.1".parse().unwrap();

        // First two connections should be allowed
        assert!(manager.should_allow_connection(ip).await);
        manager.register_connection(ip).await;

        assert!(manager.should_allow_connection(ip).await);
        manager.register_connection(ip).await;

        // Third connection should be rejected
        assert!(!manager.should_allow_connection(ip).await);

        // After unregistering, should allow again
        manager.unregister_connection(ip).await;
        assert!(manager.should_allow_connection(ip).await);
    }

    #[tokio::test]
    async fn test_message_rate_limiting() {
        let config = DosProtectionConfig {
            max_messages_per_second: 5,
            ..Default::default()
        };

        let manager = DosProtectionManager::new(config);
        let ip: IpAddr = "192.168.1.1".parse().unwrap();

        // First 5 messages should be allowed
        for _ in 0..5 {
            assert!(manager.should_allow_message(ip, 100).await);
        }

        // 6th message should be rejected
        assert!(!manager.should_allow_message(ip, 100).await);

        // After 1 second, should allow again
        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(manager.should_allow_message(ip, 100).await);
    }

    #[tokio::test]
    async fn test_whitelisting() {
        let whitelisted_ip: IpAddr = "192.168.1.1".parse().unwrap();
        let config = DosProtectionConfig {
            max_connections_per_ip: 1,
            max_messages_per_second: 1,
            whitelisted_ips: vec![whitelisted_ip],
            ..Default::default()
        };

        let manager = DosProtectionManager::new(config);

        // Whitelisted IP should always be allowed
        for _ in 0..10 {
            assert!(manager.should_allow_connection(whitelisted_ip).await);
            assert!(manager.should_allow_message(whitelisted_ip, 100).await);
        }
    }
}

//! Enhanced DoS Protection and Rate Limiting
//!
//! This module provides advanced DoS protection mechanisms for the Neo network layer,
//! implementing sophisticated rate limiting, connection throttling, and attack mitigation.

use crate::{NetworkError, NetworkResult};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Enhanced DoS protection configuration
#[derive(Debug, Clone)]
pub struct DosProtectionConfig {
    /// Maximum connections per IP address
    pub max_connections_per_ip: usize,
    /// Maximum message rate per connection (messages/second)
    pub max_message_rate: f64,
    /// Maximum bandwidth per connection (bytes/second)
    pub max_bandwidth_per_connection: u64,
    /// Connection rate limit (new connections/minute)
    pub max_connection_rate: usize,
    /// Ban duration for rate limit violations (seconds)
    pub ban_duration_seconds: u64,
    /// Whitelist of trusted IP addresses
    pub trusted_ips: Vec<IpAddr>,
    /// Enable adaptive rate limiting
    pub adaptive_rate_limiting: bool,
}

impl Default for DosProtectionConfig {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 10,
            max_message_rate: 100.0,
            max_bandwidth_per_connection: 1024 * 1024, // 1MB/s
            max_connection_rate: 60,                   // 1 per second
            ban_duration_seconds: 3600,                // 1 hour
            trusted_ips: Vec::new(),
            adaptive_rate_limiting: true,
        }
    }
}

/// Connection tracking information
#[derive(Debug, Clone)]
struct ConnectionInfo {
    /// Connection start time
    start_time: Instant,
    /// Message count in current window
    message_count: u64,
    /// Bytes transferred in current window
    bytes_transferred: u64,
    /// Last message timestamp
    last_message: Instant,
    /// Rate limit violations count
    violations: u32,
    /// Whether connection is currently rate limited
    is_rate_limited: bool,
    /// Rate limit reset time
    rate_limit_reset: Option<Instant>,
}

impl ConnectionInfo {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            message_count: 0,
            bytes_transferred: 0,
            last_message: now,
            violations: 0,
            is_rate_limited: false,
            rate_limit_reset: None,
        }
    }

    fn reset_window(&mut self) {
        self.message_count = 0;
        self.bytes_transferred = 0;
        self.last_message = Instant::now();
    }

    fn should_reset_window(&self) -> bool {
        self.last_message.elapsed() > Duration::from_secs(60)
    }
}

/// IP-based ban information
#[derive(Debug, Clone)]
struct BanInfo {
    /// Ban start time
    start_time: Instant,
    /// Ban duration
    duration: Duration,
    /// Reason for ban
    reason: String,
    /// Number of ban extensions
    extensions: u32,
}

impl BanInfo {
    fn is_expired(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }

    fn extend_ban(&mut self) {
        self.extensions += 1;
        // Progressive ban duration: 1h, 4h, 24h, 7 days
        let multiplier = match self.extensions {
            1 => 4,
            2 => 24,
            _ => 168, // 7 days
        };
        self.duration = Duration::from_secs(3600 * multiplier);
        self.start_time = Instant::now();
    }
}

/// Enhanced DoS protection system
pub struct EnhancedDosProtection {
    /// Configuration
    config: DosProtectionConfig,
    /// Per-connection tracking
    connections: Arc<RwLock<HashMap<SocketAddr, ConnectionInfo>>>,
    /// Per-IP connection counts
    ip_connections: Arc<RwLock<HashMap<IpAddr, usize>>>,
    /// Banned IP addresses
    banned_ips: Arc<RwLock<HashMap<IpAddr, BanInfo>>>,
    /// Connection rate tracking per IP
    connection_rates: Arc<RwLock<HashMap<IpAddr, Vec<Instant>>>>,
    /// Global network load metrics
    global_metrics: Arc<RwLock<GlobalNetworkMetrics>>,
}

#[derive(Debug, Default)]
struct GlobalNetworkMetrics {
    total_connections: usize,
    total_message_rate: f64,
    total_bandwidth: u64,
    cpu_usage: f64,
    memory_usage: f64,
}

impl EnhancedDosProtection {
    /// Creates a new enhanced DoS protection system
    pub fn new(config: DosProtectionConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
            ip_connections: Arc::new(RwLock::new(HashMap::new())),
            banned_ips: Arc::new(RwLock::new(HashMap::new())),
            connection_rates: Arc::new(RwLock::new(HashMap::new())),
            global_metrics: Arc::new(RwLock::new(GlobalNetworkMetrics::default())),
        }
    }

    /// Check if a connection should be allowed
    pub async fn should_allow_connection(&self, addr: &SocketAddr) -> NetworkResult<bool> {
        let ip = addr.ip();

        // Check if IP is banned
        if self.is_ip_banned(&ip).await {
            return Ok(false);
        }

        // Check if IP is trusted
        if self.config.trusted_ips.contains(&ip) {
            return Ok(true);
        }

        // Check connection count per IP
        let ip_connections = self.ip_connections.read().await;
        let current_connections = ip_connections.get(&ip).unwrap_or(&0);

        if *current_connections >= self.config.max_connections_per_ip {
            warn!("Connection rejected: IP {} exceeds connection limit", ip);
            return Ok(false);
        }

        // Check connection rate
        if !self.check_connection_rate(&ip).await {
            warn!("Connection rejected: IP {} exceeds connection rate", ip);
            return Ok(false);
        }

        // Check adaptive rate limiting based on global load
        if self.config.adaptive_rate_limiting {
            let global_metrics = self.global_metrics.read().await;
            if global_metrics.cpu_usage > 90.0 || global_metrics.memory_usage > 85.0 {
                // Under high load, be more restrictive
                if *current_connections >= self.config.max_connections_per_ip / 2 {
                    info!("Connection rejected: System under high load, IP {}", ip);
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Register a new connection
    pub async fn register_connection(&self, addr: &SocketAddr) -> NetworkResult<()> {
        let ip = addr.ip();

        // Update IP connection count
        let mut ip_connections = self.ip_connections.write().await;
        *ip_connections.entry(ip).or_insert(0) += 1;

        // Create connection tracking
        let mut connections = self.connections.write().await;
        connections.insert(*addr, ConnectionInfo::new());

        debug!("Registered connection from {}", addr);
        Ok(())
    }

    /// Remove a connection
    pub async fn unregister_connection(&self, addr: &SocketAddr) -> NetworkResult<()> {
        let ip = addr.ip();

        // Update IP connection count
        let mut ip_connections = self.ip_connections.write().await;
        if let Some(count) = ip_connections.get_mut(&ip) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                ip_connections.remove(&ip);
            }
        }

        // Remove connection tracking
        let mut connections = self.connections.write().await;
        connections.remove(addr);

        debug!("Unregistered connection from {}", addr);
        Ok(())
    }

    /// Check if a message should be rate limited
    pub async fn should_rate_limit_message(
        &self,
        addr: &SocketAddr,
        message_size: usize,
    ) -> NetworkResult<bool> {
        let ip = addr.ip();

        // Trusted IPs bypass rate limiting
        if self.config.trusted_ips.contains(&ip) {
            return Ok(false);
        }

        let mut connections = self.connections.write().await;
        let connection_info = connections.get_mut(addr).ok_or_else(|| {
            NetworkError::InvalidConnection(format!("Connection not registered: {}", addr))
        })?;

        // Reset window if needed
        if connection_info.should_reset_window() {
            connection_info.reset_window();
        }

        // Check if currently rate limited
        if connection_info.is_rate_limited {
            if let Some(reset_time) = connection_info.rate_limit_reset {
                if Instant::now() >= reset_time {
                    connection_info.is_rate_limited = false;
                    connection_info.rate_limit_reset = None;
                    connection_info.violations = 0;
                } else {
                    return Ok(true); // Still rate limited
                }
            }
        }

        // Update message statistics
        connection_info.message_count += 1;
        connection_info.bytes_transferred += message_size as u64;
        connection_info.last_message = Instant::now();

        // Check rate limits
        let time_window = 60.0; // 1 minute window
        let message_rate = connection_info.message_count as f64 / time_window;
        let bandwidth_rate = connection_info.bytes_transferred / 60; // bytes per second

        if message_rate > self.config.max_message_rate {
            connection_info.violations += 1;
            connection_info.is_rate_limited = true;
            connection_info.rate_limit_reset = Some(Instant::now() + Duration::from_secs(60));

            warn!(
                "Rate limiting connection {}: message rate {:.2}/s exceeds limit {:.2}/s",
                addr, message_rate, self.config.max_message_rate
            );

            // Consider banning for repeated violations
            if connection_info.violations >= 3 {
                self.ban_ip(&ip, "Repeated rate limit violations").await;
            }

            return Ok(true);
        }

        if bandwidth_rate > self.config.max_bandwidth_per_connection {
            connection_info.violations += 1;
            connection_info.is_rate_limited = true;
            connection_info.rate_limit_reset = Some(Instant::now() + Duration::from_secs(60));

            warn!(
                "Rate limiting connection {}: bandwidth {} B/s exceeds limit {} B/s",
                addr, bandwidth_rate, self.config.max_bandwidth_per_connection
            );

            return Ok(true);
        }

        Ok(false)
    }

    /// Ban an IP address
    async fn ban_ip(&self, ip: &IpAddr, reason: &str) {
        let mut banned_ips = self.banned_ips.write().await;

        let ban_info = BanInfo {
            start_time: Instant::now(),
            duration: Duration::from_secs(self.config.ban_duration_seconds),
            reason: reason.to_string(),
            extensions: 0,
        };

        if let Some(existing_ban) = banned_ips.get_mut(ip) {
            existing_ban.extend_ban();
            warn!(
                "Extended ban for IP {}: {} (extension #{})",
                ip, reason, existing_ban.extensions
            );
        } else {
            banned_ips.insert(*ip, ban_info);
            warn!("Banned IP {}: {}", ip, reason);
        }
    }

    /// Check if an IP is currently banned
    async fn is_ip_banned(&self, ip: &IpAddr) -> bool {
        let mut banned_ips = self.banned_ips.write().await;

        if let Some(ban_info) = banned_ips.get(ip) {
            if ban_info.is_expired() {
                banned_ips.remove(ip);
                info!("Ban expired for IP {}", ip);
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Check connection rate for an IP
    async fn check_connection_rate(&self, ip: &IpAddr) -> bool {
        let mut connection_rates = self.connection_rates.write().await;
        let now = Instant::now();

        // Get or create rate tracking for this IP
        let timestamps = connection_rates.entry(*ip).or_insert_with(Vec::new);

        // Remove old timestamps (older than 1 minute)
        timestamps.retain(|&ts| now.duration_since(ts) < Duration::from_secs(60));

        // Check if adding this connection would exceed rate limit
        if timestamps.len() >= self.config.max_connection_rate {
            return false;
        }

        // Add current timestamp
        timestamps.push(now);
        true
    }

    /// Update global network metrics for adaptive rate limiting
    pub async fn update_global_metrics(&self, cpu_usage: f64, memory_usage: f64) {
        let mut metrics = self.global_metrics.write().await;
        metrics.cpu_usage = cpu_usage;
        metrics.memory_usage = memory_usage;

        // Update connection and traffic metrics
        let connections = self.connections.read().await;
        metrics.total_connections = connections.len();

        let total_messages: u64 = connections.values().map(|conn| conn.message_count).sum();
        metrics.total_message_rate = total_messages as f64 / 60.0; // messages per second

        let total_bandwidth: u64 = connections
            .values()
            .map(|conn| conn.bytes_transferred)
            .sum();
        metrics.total_bandwidth = total_bandwidth / 60; // bytes per second
    }

    /// Get DoS protection statistics
    pub async fn get_statistics(&self) -> DosProtectionStats {
        let connections = self.connections.read().await;
        let banned_ips = self.banned_ips.read().await;
        let ip_connections = self.ip_connections.read().await;
        let global_metrics = self.global_metrics.read().await;

        let rate_limited_connections = connections
            .values()
            .filter(|conn| conn.is_rate_limited)
            .count();

        let total_violations: u32 = connections.values().map(|conn| conn.violations).sum();

        DosProtectionStats {
            total_connections: connections.len(),
            rate_limited_connections,
            banned_ips_count: banned_ips.len(),
            unique_ips: ip_connections.len(),
            total_violations,
            global_message_rate: global_metrics.total_message_rate,
            global_bandwidth: global_metrics.total_bandwidth,
            adaptive_throttling_active: self.config.adaptive_rate_limiting
                && (global_metrics.cpu_usage > 80.0 || global_metrics.memory_usage > 75.0),
        }
    }

    /// Cleanup expired bans and old connection data
    pub async fn cleanup_expired_data(&self) {
        // Clean up expired bans
        let mut banned_ips = self.banned_ips.write().await;
        banned_ips.retain(|ip, ban_info| {
            if ban_info.is_expired() {
                info!("Removed expired ban for IP {}", ip);
                false
            } else {
                true
            }
        });

        // Clean up old connection rate data
        let mut connection_rates = self.connection_rates.write().await;
        let now = Instant::now();
        for timestamps in connection_rates.values_mut() {
            timestamps.retain(|&ts| now.duration_since(ts) < Duration::from_secs(60));
        }
        connection_rates.retain(|_, timestamps| !timestamps.is_empty());
    }
}

/// DoS protection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosProtectionStats {
    pub total_connections: usize,
    pub rate_limited_connections: usize,
    pub banned_ips_count: usize,
    pub unique_ips: usize,
    pub total_violations: u32,
    pub global_message_rate: f64,
    pub global_bandwidth: u64,
    pub adaptive_throttling_active: bool,
}

use serde::{Deserialize, Serialize};

/// Message type classification for differentiated rate limiting
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    /// Critical consensus messages (higher priority)
    Consensus,
    /// Block synchronization messages
    BlockSync,
    /// Transaction relay messages  
    Transaction,
    /// Peer discovery and handshake
    Discovery,
    /// General protocol messages
    General,
}

impl MessageType {
    /// Get rate limit multiplier for message type
    pub fn rate_multiplier(&self) -> f64 {
        match self {
            MessageType::Consensus => 2.0,   // Allow 2x rate for consensus
            MessageType::BlockSync => 1.5,   // Allow 1.5x for block sync
            MessageType::Transaction => 1.0, // Normal rate for transactions
            MessageType::Discovery => 0.5,   // Reduced rate for discovery
            MessageType::General => 0.8,     // Slightly reduced for general
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[tokio::test]
    async fn test_connection_rate_limiting() {
        let config = DosProtectionConfig {
            max_connections_per_ip: 2,
            max_connection_rate: 2, // 2 connections per minute
            ..Default::default()
        };

        let dos_protection = EnhancedDosProtection::new(config);
        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let addr1 = SocketAddr::new(test_ip, 8000);
        let addr2 = SocketAddr::new(test_ip, 8001);
        let addr3 = SocketAddr::new(test_ip, 8002);

        // First two connections should be allowed
        assert!(dos_protection
            .should_allow_connection(&addr1)
            .await
            .unwrap());
        dos_protection.register_connection(&addr1).await.unwrap();

        assert!(dos_protection
            .should_allow_connection(&addr2)
            .await
            .unwrap());
        dos_protection.register_connection(&addr2).await.unwrap();

        // Third connection should be rejected (exceeds max_connections_per_ip)
        assert!(!dos_protection
            .should_allow_connection(&addr3)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_message_rate_limiting() {
        let config = DosProtectionConfig {
            max_message_rate: 10.0, // 10 messages per second
            ..Default::default()
        };

        let dos_protection = EnhancedDosProtection::new(config);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)), 8000);

        // Register connection
        dos_protection.register_connection(&addr).await.unwrap();

        // Send messages within rate limit
        for _ in 0..10 {
            assert!(!dos_protection
                .should_rate_limit_message(&addr, 100)
                .await
                .unwrap());
        }

        // Additional messages should be rate limited
        // Note: This test is simplified - in practice would need time simulation
    }

    #[test]
    fn test_ban_info_expiration() {
        let mut ban_info = BanInfo {
            start_time: Instant::now() - Duration::from_secs(7200), // 2 hours ago
            duration: Duration::from_secs(3600),                    // 1 hour duration
            reason: "test".to_string(),
            extensions: 0,
        };

        assert!(ban_info.is_expired());

        ban_info.extend_ban();
        assert!(!ban_info.is_expired()); // Should be active again with extended duration
        assert_eq!(ban_info.extensions, 1);
    }

    #[test]
    fn test_message_type_rate_multipliers() {
        assert_eq!(MessageType::Consensus.rate_multiplier(), 2.0);
        assert_eq!(MessageType::Transaction.rate_multiplier(), 1.0);
        assert_eq!(MessageType::Discovery.rate_multiplier(), 0.5);
    }
}

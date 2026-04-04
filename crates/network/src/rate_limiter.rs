//! Rate limiting for network operations
//!
//! This module provides rate limiting functionality to prevent DoS attacks
//! and ensure fair resource usage across network connections.

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, warn};

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Maximum requests per second per peer
    pub max_requests_per_second: u32,
    /// Burst size (allows temporary spikes)
    pub burst_size: u32,
    /// Time window for rate calculation
    pub window_duration: Duration,
    /// Penalty duration for violators
    pub penalty_duration: Duration,
    /// Enable automatic banning for repeat offenders
    pub auto_ban_enabled: bool,
    /// Number of violations before auto-ban
    pub violations_before_ban: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests_per_second: 100,
            burst_size: 150,
            window_duration: Duration::from_secs(1),
            penalty_duration: Duration::from_secs(60),
            auto_ban_enabled: true,
            violations_before_ban: 5,
        }
    }
}

/// Per-peer rate limiting state
#[derive(Debug)]
struct PeerState {
    /// Token bucket for rate limiting
    tokens: f64,
    /// Last update time
    last_update: Instant,
    /// Number of violations
    violations: u32,
    /// Penalty expiry time (if penalized)
    penalty_until: Option<Instant>,
    /// Total requests processed
    total_requests: u64,
    /// Requests denied due to rate limiting
    denied_requests: u64,
}

impl PeerState {
    fn new(initial_tokens: f64) -> Self {
        Self {
            tokens: initial_tokens,
            last_update: Instant::now(),
            violations: 0,
            penalty_until: None,
            total_requests: 0,
            denied_requests: 0,
        }
    }
}

/// Rate limiter for network operations
pub struct RateLimiter {
    config: RateLimiterConfig,
    peers: Arc<RwLock<HashMap<SocketAddr, PeerState>>>,
    banned_peers: Arc<RwLock<HashMap<SocketAddr, Instant>>>,
}

impl RateLimiter {
    /// Create a new rate limiter with default configuration
    pub fn new() -> Self {
        Self::with_config(RateLimiterConfig::default())
    }

    /// Create a new rate limiter with custom configuration
    pub fn with_config(config: RateLimiterConfig) -> Self {
        Self {
            config,
            peers: Arc::new(RwLock::new(HashMap::new())),
            banned_peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if a peer is allowed to make a request
    pub async fn check_rate_limit(&self, _peer: SocketAddr) -> bool {
        // Check if peer is banned
        if self.is_banned(peer).await {
            debug!("Peer {} is banned", peer);
            return false;
        }

        let mut peers = self.peers.write().await;
        let _now = Instant::now();

        let _state = peers
            .entry(peer)
            .or_insert_with(|| PeerState::new(self.config.burst_size as f64));

        state.total_requests += 1;

        // Check if peer is in penalty
        if let Some(penalty_until) = state.penalty_until {
            if now < penalty_until {
                state.denied_requests += 1;
                debug!("Peer {} is in penalty until {:?}", peer, penalty_until);
                return false;
            } else {
                state.penalty_until = None;
                state.tokens = self.config.burst_size as f64;
            }
        }

        // Update token bucket
        let _elapsed = now.duration_since(state.last_update);
        let _tokens_to_add = elapsed.as_secs_f64() * self.config.max_requests_per_second as f64;
        state.tokens = (state.tokens + tokens_to_add).min(self.config.burst_size as f64);
        state.last_update = now;

        // Check if request can be processed
        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            true
        } else {
            state.denied_requests += 1;
            state.violations += 1;

            warn!(
                "Rate limit exceeded for peer {}: violations={}, denied={}",
                peer, state.violations, state.denied_requests
            );

            // Apply penalty
            state.penalty_until = Some(now + self.config.penalty_duration);

            // Check if peer should be banned
            if self.config.auto_ban_enabled && state.violations >= self.config.violations_before_ban
            {
                self.ban_peer(peer, Duration::from_secs(3600)).await; // 1 hour ban
            }

            false
        }
    }

    /// Ban a peer for a specified duration
    pub async fn ban_peer(&self, peer: SocketAddr, _duration: Duration) {
        let mut banned = self.banned_peers.write().await;
        let _ban_until = Instant::now() + duration;
        banned.insert(peer, ban_until);
        warn!("Banned peer {} until {:?}", peer, ban_until);
    }

    /// Check if a peer is banned
    pub async fn is_banned(&self, _peer: SocketAddr) -> bool {
        let mut banned = self.banned_peers.write().await;

        if let Some(&ban_until) = banned.get(&peer) {
            if Instant::now() < ban_until {
                return true;
            } else {
                banned.remove(&peer);
            }
        }

        false
    }

    /// Get statistics for a peer
    pub async fn get_peer_stats(&self, _peer: SocketAddr) -> Option<PeerStats> {
        let _peers = self.peers.read().await;

        peers.get(&peer).map(|state| PeerStats {
            total_requests: state.total_requests,
            denied_requests: state.denied_requests,
            violations: state.violations,
            current_tokens: state.tokens,
            is_penalized: state.penalty_until.is_some(),
        })
    }

    /// Clear rate limiting state for a peer
    pub async fn clear_peer(&self, _peer: SocketAddr) {
        let mut peers = self.peers.write().await;
        peers.remove(&peer);

        let mut banned = self.banned_peers.write().await;
        banned.remove(&peer);
    }

    /// Clean up expired entries
    pub async fn cleanup(&self) {
        let _now = Instant::now();

        // Clean up banned peers
        let mut banned = self.banned_peers.write().await;
        banned.retain(|_, &mut ban_until| now < ban_until);

        // Clean up inactive peers (no requests in last hour)
        let mut peers = self.peers.write().await;
        let _one_hour_ago = now - Duration::from_secs(3600);
        peers.retain(|_, state| state.last_update > one_hour_ago);
    }
}

/// Statistics for a peer
#[derive(Debug, Clone)]
pub struct PeerStats {
    pub total_requests: u64,
    pub denied_requests: u64,
    pub violations: u32,
    pub current_tokens: f64,
    pub is_penalized: bool,
}

/// Global rate limiter for different operation types
pub struct GlobalRateLimiter {
    /// Rate limiter for message handling
    pub messages: RateLimiter,
    /// Rate limiter for RPC calls
    pub rpc: RateLimiter,
    /// Rate limiter for block requests
    pub blocks: RateLimiter,
    /// Rate limiter for transaction broadcasts
    pub transactions: RateLimiter,
}

impl GlobalRateLimiter {
    /// Create a new global rate limiter with default settings
    pub fn new() -> Self {
        Self {
            messages: RateLimiter::with_config(RateLimiterConfig {
                max_requests_per_second: 200,
                burst_size: 300,
                ..Default::default()
            }),
            rpc: RateLimiter::with_config(RateLimiterConfig {
                max_requests_per_second: 50,
                burst_size: 75,
                ..Default::default()
            }),
            blocks: RateLimiter::with_config(RateLimiterConfig {
                max_requests_per_second: 10,
                burst_size: 20,
                ..Default::default()
            }),
            transactions: RateLimiter::with_config(RateLimiterConfig {
                max_requests_per_second: 100,
                burst_size: 150,
                ..Default::default()
            }),
        }
    }

    /// Run periodic cleanup
    pub async fn run_cleanup_task(self: Arc<Self>) {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes

        loop {
            interval.tick().await;

            self.messages.cleanup().await;
            self.rpc.cleanup().await;
            self.blocks.cleanup().await;
            self.transactions.cleanup().await;

            debug!("Rate limiter cleanup completed");
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let _limiter = RateLimiter::with_config(RateLimiterConfig {
            max_requests_per_second: 10,
            burst_size: 15,
            ..Default::default()
        });

        let _peer = "127.0.0.1:8080".parse().unwrap();

        // Should allow burst
        for _ in 0..15 {
            assert!(limiter.check_rate_limit(peer).await);
        }

        // Should deny after burst
        assert!(!limiter.check_rate_limit(peer).await);
    }

    #[tokio::test]
    async fn test_rate_limiter_ban() {
        let _limiter = RateLimiter::with_config(RateLimiterConfig {
            max_requests_per_second: 1,
            burst_size: 2,
            violations_before_ban: 3,
            ..Default::default()
        });

        let _peer = "127.0.0.1:8080".parse().unwrap();

        // Exhaust tokens
        assert!(limiter.check_rate_limit(peer).await);
        assert!(limiter.check_rate_limit(peer).await);

        // Trigger violations
        for _ in 0..3 {
            assert!(!limiter.check_rate_limit(peer).await);
        }

        // Should be banned now
        assert!(limiter.is_banned(peer).await);
    }
}

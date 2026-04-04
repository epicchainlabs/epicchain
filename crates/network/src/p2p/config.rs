//! P2P configuration and settings.
//!
//! This module implements P2P configuration exactly matching C# Neo's ProtocolSettings.

use super::{
    CONNECTION_TIMEOUT_SECS, DEFAULT_PORT, HANDSHAKE_TIMEOUT_SECS, MAX_PEERS, MESSAGE_BUFFER_SIZE,
    PING_INTERVAL_SECS,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};

/// P2P configuration (matches C# Neo ProtocolSettings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfig {
    /// Listen address for incoming connections
    pub listen_address: SocketAddr,

    /// Maximum number of peers to maintain
    pub max_peers: usize,

    /// Connection timeout duration
    pub connection_timeout: Duration,

    /// Handshake timeout duration
    pub handshake_timeout: Duration,

    /// Interval between ping messages
    pub ping_interval: Duration,

    /// Message buffer size for channels
    pub message_buffer_size: usize,

    /// Enable message compression
    pub enable_compression: bool,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            listen_address: format!("localhost:{}", DEFAULT_PORT)
                .parse()
                .unwrap_or_else(|_| {
                    format!("0.0.0.0:{}", DEFAULT_PORT)
                        .parse()
                        .expect("value should parse")
                }),
            max_peers: MAX_PEERS,
            connection_timeout: Duration::from_secs(CONNECTION_TIMEOUT_SECS),
            handshake_timeout: Duration::from_secs(HANDSHAKE_TIMEOUT_SECS),
            ping_interval: Duration::from_secs(PING_INTERVAL_SECS),
            message_buffer_size: MESSAGE_BUFFER_SIZE,
            enable_compression: false,
        }
    }
}

impl P2PConfig {
    /// Creates a new P2P configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the listen address
    pub fn with_listen_address(mut self, address: SocketAddr) -> Self {
        self.listen_address = address;
        self
    }

    /// Sets the maximum number of peers
    pub fn with_max_peers(mut self, max_peers: usize) -> Self {
        self.max_peers = max_peers;
        self
    }

    /// Sets the connection timeout
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Sets the handshake timeout
    pub fn with_handshake_timeout(mut self, timeout: Duration) -> Self {
        self.handshake_timeout = timeout;
        self
    }

    /// Sets the ping interval
    pub fn with_ping_interval(mut self, interval: Duration) -> Self {
        self.ping_interval = interval;
        self
    }

    /// Sets the message buffer size
    pub fn with_message_buffer_size(mut self, size: usize) -> Self {
        self.message_buffer_size = size;
        self
    }

    /// Enables or disables message compression
    pub fn with_compression(mut self, enable: bool) -> Self {
        self.enable_compression = enable;
        self
    }

    /// Validates the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_peers == 0 {
            return Err("max_peers must be greater than 0".to_string());
        }

        if self.connection_timeout.as_secs() == 0 {
            return Err("connection_timeout must be greater than 0".to_string());
        }

        if self.handshake_timeout.as_secs() == 0 {
            return Err("handshake_timeout must be greater than 0".to_string());
        }

        if self.message_buffer_size == 0 {
            return Err("message_buffer_size must be greater than 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, PeerInfo};
    use std::time::Duration;

    #[test]
    fn test_p2p_config_default() {
        let config = P2PConfig::default();
        assert_eq!(config.max_peers, MAX_PEERS);
        assert_eq!(
            config.connection_timeout,
            Duration::from_secs(CONNECTION_TIMEOUT_SECS)
        );
        assert_eq!(config.message_buffer_size, MESSAGE_BUFFER_SIZE);
        assert!(!config.enable_compression);
    }

    #[test]
    fn test_p2p_config_builder() {
        let config = P2PConfig::new().with_max_peers(50).with_compression(true);

        assert_eq!(config.max_peers, 50);
        assert!(config.enable_compression);
    }

    #[test]
    fn test_p2p_config_validation() {
        let valid_config = P2PConfig::default();
        assert!(valid_config.validate().is_ok());

        let invalid_config = P2PConfig::default().with_max_peers(0);
        assert!(invalid_config.validate().is_err());
    }
}

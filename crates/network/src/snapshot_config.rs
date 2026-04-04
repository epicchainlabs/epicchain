//! Snapshot configuration for fast blockchain sync
//!
//! This module provides configuration for blockchain snapshots that enable
//! fast initial sync by downloading pre-validated blockchain state.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Snapshot provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotProvider {
    /// Provider name
    pub name: String,
    /// Provider website
    pub website: String,
    /// Trust level (0-100)
    pub trust_level: u8,
    /// Available snapshots
    pub snapshots: Vec<SnapshotInfo>,
}

/// Detailed snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Network (mainnet, testnet, etc.)
    pub network: String,
    /// Block height
    pub height: u32,
    /// Block hash at this height
    pub block_hash: String,
    /// Creation timestamp
    pub created_at: u64,
    /// File size in bytes
    pub size: u64,
    /// Download URL
    pub url: String,
    /// SHA256 checksum
    pub sha256: String,
    /// Compression format (gz, zstd, etc.)
    pub compression: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Snapshot providers
    pub providers: Vec<SnapshotProvider>,
    /// Minimum trust level required
    pub min_trust_level: u8,
    /// Maximum snapshot age in seconds
    pub max_age_seconds: u64,
    /// Preferred compression formats
    pub preferred_compression: Vec<String>,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            providers: vec![],
            min_trust_level: 80,
            max_age_seconds: 7 * 24 * 3600, // 7 days
            preferred_compression: vec!["zstd".to_string(), "gz".to_string()],
        }
    }
}

impl SnapshotConfig {
    /// Load snapshot configuration from file
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    /// Save snapshot configuration to file
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Find best snapshot for a given height and network
    pub fn find_best_snapshot(&self, network: &str, target_height: u32) -> Option<&SnapshotInfo> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.providers
            .iter()
            .filter(|p| p.trust_level >= self.min_trust_level)
            .flat_map(|p| &p.snapshots)
            .filter(|s| {
                s.network == network
                    && s.height <= target_height
                    && (current_time - s.created_at) <= self.max_age_seconds
            })
            .max_by_key(|s| s.height)
    }
}

/// Example snapshot configuration for Neo mainnet
pub fn example_mainnet_config() -> SnapshotConfig {
    SnapshotConfig {
        providers: vec![SnapshotProvider {
            name: "Neo Official".to_string(),
            website: "https://neo.org".to_string(),
            trust_level: 100,
            snapshots: vec![SnapshotInfo {
                network: "mainnet".to_string(),
                height: 15_000_000,
                block_hash: "0xbf8e4d9c8b7a6f5e4d3c2b1a0f9e8d7c6b5a4f3e2d1c0b9a8f7e6d5c4b3a2f1e"
                    .to_string(),
                created_at: (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .saturating_sub(1)),
                size: 50_000_000_000, // 50GB
                url: "https://sync.neo.org/mainnet/snapshot-15000000.tar.zstd".to_string(),
                sha256: "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
                    .to_string(),
                compression: "zstd".to_string(),
                metadata: HashMap::new(),
            }],
        }],
        ..Default::default()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_config_serialization() {
        let config = example_mainnet_config();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: SnapshotConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.providers.len(), deserialized.providers.len());
    }

    #[test]
    fn test_find_best_snapshot() {
        let config = example_mainnet_config();
        let snapshot = config.find_best_snapshot("mainnet", 16_000_000);
        assert!(snapshot.is_some());
        assert_eq!(snapshot.unwrap().height, 15_000_000);
    }

    #[test]
    fn test_snapshot_filtering() {
        let mut config = example_mainnet_config();
        config.max_age_seconds = 0; // No snapshots should be valid
        let snapshot = config.find_best_snapshot("mainnet", 16_000_000);
        assert!(snapshot.is_none());
    }
}

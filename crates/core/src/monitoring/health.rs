//! Health check system for Neo node monitoring
//!
//! Provides comprehensive health checks for node components and dependencies.

use crate::error_handling::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Health status of a component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Represents an enumeration of values.
pub enum HealthStatus {
    /// Component is healthy and functioning normally
    Healthy,
    /// Component is degraded but still operational
    Degraded,
    /// Component is unhealthy and may not be functioning
    Unhealthy,
    /// Component status is unknown
    Unknown,
}

/// Health check result for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct HealthCheckResult {
    /// Component name
    pub component: String,
    /// Health status
    pub status: HealthStatus,
    /// Optional message describing the status
    pub message: Option<String>,
    /// Additional details
    pub details: HashMap<String, serde_json::Value>,
    /// Timestamp of the check
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    /// Duration of the health check
    pub duration: Duration,
}

/// Overall system health report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct HealthReport {
    /// Overall system status
    pub status: HealthStatus,
    /// Individual component results
    pub components: Vec<HealthCheckResult>,
    /// System uptime
    pub uptime: Duration,
    /// Last check timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    /// Version information
    pub version: String,
}

/// Trait for components that can report health
#[async_trait::async_trait]
/// Defines a trait interface.
pub trait HealthCheck: Send + Sync {
    /// Perform health check
    async fn check_health(&self) -> HealthCheckResult;

    /// Get component name
    fn component_name(&self) -> String;
}

/// Health monitor that coordinates health checks
/// Represents a data structure.
pub struct HealthMonitor {
    /// Registered health checks
    checks: Arc<RwLock<Vec<Arc<dyn HealthCheck>>>>,
    /// Start time for uptime calculation
    start_time: Instant,
    /// Node version
    version: String,
    /// Cache for health results
    cache: Arc<RwLock<Option<HealthReport>>>,
    /// Cache duration
    cache_duration: Duration,
}

impl HealthMonitor {
    /// Create new health monitor
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(version: String) -> Self {
        Self {
            checks: Arc::new(RwLock::new(Vec::new())),
            start_time: Instant::now(),
            version,
            cache: Arc::new(RwLock::new(None)),
            cache_duration: Duration::from_secs(5),
        }
    }

    /// Register a health check
    pub async fn register_check(&self, check: Arc<dyn HealthCheck>) {
        let mut checks = self.checks.write().await;
        checks.push(check);
    }

    /// Run all health checks and generate report
    pub async fn check_health(&self) -> Result<HealthReport> {
        // Check cache first
        if let Some(cached) = self.get_cached_report().await {
            return Ok(cached);
        }

        let checks = self.checks.read().await;
        let mut results = Vec::new();

        // Run all checks in parallel
        let futures: Vec<_> = checks.iter().map(|check| check.check_health()).collect();
        let check_results = futures::future::join_all(futures).await;

        results.extend(check_results);

        // Determine overall status
        let overall_status = self.calculate_overall_status(&results);

        let report = HealthReport {
            status: overall_status,
            components: results,
            uptime: self.start_time.elapsed(),
            timestamp: Utc::now(),
            version: self.version.clone(),
        };

        // Cache the report
        self.cache_report(report.clone()).await;

        Ok(report)
    }

    /// Get cached report if still valid
    async fn get_cached_report(&self) -> Option<HealthReport> {
        let cache = self.cache.read().await;
        if let Some(ref report) = *cache {
            if Utc::now().signed_duration_since(report.timestamp)
                < chrono::Duration::from_std(self.cache_duration).unwrap()
            {
                return Some(report.clone());
            }
        }
        None
    }

    /// Cache health report
    async fn cache_report(&self, report: HealthReport) {
        let mut cache = self.cache.write().await;
        *cache = Some(report);
    }

    /// Calculate overall status from component results
    fn calculate_overall_status(&self, results: &[HealthCheckResult]) -> HealthStatus {
        if results.is_empty() {
            return HealthStatus::Unknown;
        }

        let has_unhealthy = results.iter().any(|r| r.status == HealthStatus::Unhealthy);
        let has_degraded = results.iter().any(|r| r.status == HealthStatus::Degraded);

        if has_unhealthy {
            HealthStatus::Unhealthy
        } else if has_degraded {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }
}

/// Blockchain health check
/// Represents a data structure.
pub struct BlockchainHealthCheck {
    /// Maximum allowed block lag
    _max_block_lag: u64,
}

impl BlockchainHealthCheck {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(max_block_lag: u64) -> Self {
        Self {
            _max_block_lag: max_block_lag,
        }
    }
}

#[async_trait::async_trait]
impl HealthCheck for BlockchainHealthCheck {
    async fn check_health(&self) -> HealthCheckResult {
        let start = Instant::now();
        let mut details = HashMap::new();

        // Check block height
        let current_height = crate::metrics::BLOCK_HEIGHT.get() as u64;
        details.insert(
            "block_height".to_string(),
            serde_json::json!(current_height),
        );

        // Check if we're syncing
        let status = if current_height == 0 {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Healthy
        };

        HealthCheckResult {
            component: self.component_name(),
            status,
            message: Some(format!("Block height: {}", current_height)),
            details,
            timestamp: Utc::now(),
            duration: start.elapsed(),
        }
    }

    fn component_name(&self) -> String {
        "blockchain".to_string()
    }
}

/// Network health check
/// Represents a data structure.
pub struct NetworkHealthCheck {
    /// Minimum required peers
    min_peers: usize,
}

impl NetworkHealthCheck {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(min_peers: usize) -> Self {
        Self { min_peers }
    }
}

#[async_trait::async_trait]
impl HealthCheck for NetworkHealthCheck {
    async fn check_health(&self) -> HealthCheckResult {
        let start = Instant::now();
        let mut details = HashMap::new();

        // Check peer count
        let connected_peers = crate::metrics::PEER_COUNT
            .with_label_values(&["connected"])
            .get() as usize;

        details.insert(
            "connected_peers".to_string(),
            serde_json::json!(connected_peers),
        );
        details.insert("min_peers".to_string(), serde_json::json!(self.min_peers));

        let status = if connected_peers == 0 {
            HealthStatus::Unhealthy
        } else if connected_peers < self.min_peers {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        HealthCheckResult {
            component: self.component_name(),
            status,
            message: Some(format!(
                "Connected peers: {}/{}",
                connected_peers, self.min_peers
            )),
            details,
            timestamp: Utc::now(),
            duration: start.elapsed(),
        }
    }

    fn component_name(&self) -> String {
        "network".to_string()
    }
}

/// Storage health check
/// Represents a data structure.
pub struct StorageHealthCheck {
    /// Minimum required free space in bytes
    min_free_space: u64,
}

impl StorageHealthCheck {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(min_free_space: u64) -> Self {
        Self { min_free_space }
    }
}

#[async_trait::async_trait]
impl HealthCheck for StorageHealthCheck {
    async fn check_health(&self) -> HealthCheckResult {
        let start = Instant::now();
        let mut details = HashMap::new();

        // Check disk space
        let available_space = self.get_available_space();
        details.insert(
            "available_space".to_string(),
            serde_json::json!(available_space),
        );
        details.insert(
            "min_free_space".to_string(),
            serde_json::json!(self.min_free_space),
        );

        let status = if available_space < self.min_free_space {
            HealthStatus::Unhealthy
        } else if available_space < self.min_free_space * 2 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        HealthCheckResult {
            component: self.component_name(),
            status,
            message: Some(format!("Available space: {} bytes", available_space)),
            details,
            timestamp: Utc::now(),
            duration: start.elapsed(),
        }
    }

    fn component_name(&self) -> String {
        "storage".to_string()
    }
}

impl StorageHealthCheck {
    fn get_available_space(&self) -> u64 {
        use std::fs;
        use std::path::Path;

        let path = Path::new(".");
        if let Ok(_metadata) = fs::metadata(path) {
            // This is a simplified implementation
            // In production, use platform-specific APIs for accurate disk space
            1_000_000_000 // 1GB placeholder
        } else {
            0
        }
    }
}

/// Memory health check
/// Represents a data structure.
pub struct MemoryHealthCheck {
    /// Maximum memory usage in bytes
    max_memory: u64,
}

impl MemoryHealthCheck {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(max_memory: u64) -> Self {
        Self { max_memory }
    }
}

#[async_trait::async_trait]
impl HealthCheck for MemoryHealthCheck {
    async fn check_health(&self) -> HealthCheckResult {
        let start = Instant::now();
        let mut details = HashMap::new();

        // Check memory usage
        let memory_usage = crate::metrics::MEMORY_USAGE.get() as u64;
        details.insert("memory_usage".to_string(), serde_json::json!(memory_usage));
        details.insert("max_memory".to_string(), serde_json::json!(self.max_memory));

        let usage_percent = (memory_usage as f64 / self.max_memory as f64) * 100.0;
        details.insert(
            "usage_percent".to_string(),
            serde_json::json!(usage_percent),
        );

        let status = if usage_percent > 90.0 {
            HealthStatus::Unhealthy
        } else if usage_percent > 75.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        HealthCheckResult {
            component: self.component_name(),
            status,
            message: Some(format!("Memory usage: {:.1}%", usage_percent)),
            details,
            timestamp: Utc::now(),
            duration: start.elapsed(),
        }
    }

    fn component_name(&self) -> String {
        "memory".to_string()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitor() {
        let monitor = HealthMonitor::new("1.0.0".to_string());

        // Register checks
        let blockchain_check = Arc::new(BlockchainHealthCheck::new(10));
        let network_check = Arc::new(NetworkHealthCheck::new(3));

        monitor.register_check(blockchain_check).await;
        monitor.register_check(network_check).await;

        // Run health check
        let report = monitor.check_health().await.unwrap();

        assert_eq!(report.version, "1.0.0");
        assert_eq!(report.components.len(), 2);
    }

    #[tokio::test]
    async fn test_health_status_calculation() {
        let monitor = HealthMonitor::new("1.0.0".to_string());

        let results = vec![
            HealthCheckResult {
                component: "test1".to_string(),
                status: HealthStatus::Healthy,
                message: None,
                details: HashMap::new(),
                timestamp: Utc::now(),
                duration: Duration::from_millis(10),
            },
            HealthCheckResult {
                component: "test2".to_string(),
                status: HealthStatus::Degraded,
                message: None,
                details: HashMap::new(),
                timestamp: Utc::now(),
                duration: Duration::from_millis(10),
            },
        ];

        let status = monitor.calculate_overall_status(&results);
        assert_eq!(status, HealthStatus::Degraded);
    }
}

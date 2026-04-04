//! Monitoring and observability module for Neo node
//!
//! Provides comprehensive monitoring, health checks, and performance tracking.

pub mod alerting;
pub mod exporters;
pub mod health;
pub mod performance;

pub use health::{
    BlockchainHealthCheck, HealthCheck, HealthCheckResult, HealthMonitor, HealthReport,
    HealthStatus, MemoryHealthCheck, NetworkHealthCheck, StorageHealthCheck,
};

pub use performance::{
    MetricStatistics, PerformanceAlert, PerformanceMetric, PerformanceMonitor, PerformanceSample,
    PerformanceThreshold, Profiler, ThresholdType,
};

pub use exporters::{
    CsvExporter, ExporterFactory, JsonExporter, MetricsExporter, OpenTelemetryExporter,
    PrometheusExporter, StatusReport,
};

pub use alerting::{
    Alert, AlertLevel, AlertManager, AlertRule, AlertStats, AlertThreshold, LogChannel,
    NotificationChannel, ThresholdOperator, WebhookChannel,
};

use crate::error_handling::Result;
use std::sync::Arc;

/// Initialize monitoring system
pub async fn init_monitoring(version: String) -> Result<MonitoringSystem> {
    // Initialize metrics
    crate::metrics::init_metrics()?;

    // Create health monitor
    let health_monitor = Arc::new(HealthMonitor::new(version));

    // Register default health checks
    health_monitor
        .register_check(Arc::new(BlockchainHealthCheck::new(100)))
        .await;
    health_monitor
        .register_check(Arc::new(NetworkHealthCheck::new(3)))
        .await;
    health_monitor
        .register_check(Arc::new(StorageHealthCheck::new(1_000_000_000))) // 1GB
        .await;
    health_monitor
        .register_check(Arc::new(MemoryHealthCheck::new(4_000_000_000))) // 4GB
        .await;

    // Create performance monitor
    let performance_monitor = Arc::new(PerformanceMonitor::new());

    // Register default metrics
    performance_monitor
        .register_metric("block_processing".to_string(), 1000)
        .await;
    performance_monitor
        .register_metric("tx_validation".to_string(), 1000)
        .await;
    performance_monitor
        .register_metric("consensus_round".to_string(), 100)
        .await;
    performance_monitor
        .register_metric("vm_execution".to_string(), 1000)
        .await;
    performance_monitor
        .register_metric("rpc_request".to_string(), 1000)
        .await;

    // Set default thresholds
    performance_monitor
        .set_threshold(PerformanceThreshold {
            metric: "block_processing".to_string(),
            warning: 1.0,  // 1 second
            critical: 5.0, // 5 seconds
            threshold_type: ThresholdType::Max,
        })
        .await;

    performance_monitor
        .set_threshold(PerformanceThreshold {
            metric: "tx_validation".to_string(),
            warning: 0.1,  // 100ms
            critical: 0.5, // 500ms
            threshold_type: ThresholdType::Max,
        })
        .await;

    Ok(MonitoringSystem {
        health_monitor,
        performance_monitor,
    })
}

/// Monitoring system combining health and performance monitoring
pub struct MonitoringSystem {
    /// Health monitor
    pub health_monitor: Arc<HealthMonitor>,
    /// Performance monitor
    pub performance_monitor: Arc<PerformanceMonitor>,
}

impl MonitoringSystem {
    /// Start background monitoring tasks
    pub fn start_background_tasks(&self) {
        let performance = self.performance_monitor.clone();

        // Start system metrics collection
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

            loop {
                interval.tick().await;

                // Update system metrics
                crate::metrics::update_system_metrics();

                // Record memory usage
                let memory = crate::metrics::MEMORY_USAGE.get() as f64;
                let _ = performance.record("memory_usage", memory).await;

                // Record CPU usage
                let cpu = crate::metrics::CPU_USAGE.get();
                let _ = performance.record("cpu_usage", cpu).await;
            }
        });
    }

    /// Get comprehensive status report
    pub async fn get_status(&self) -> Result<StatusReport> {
        let health = self.health_monitor.check_health().await?;
        let performance = self.performance_monitor.get_all_stats().await;
        let metrics = crate::metrics::get_metrics();

        Ok(StatusReport {
            health,
            performance,
            metrics,
        })
    }

    /// Export metrics in specified format
    pub async fn export(&self, format: &str) -> Result<String> {
        let report = self.get_status().await?;

        let exporter = ExporterFactory::create(format).ok_or_else(|| {
            crate::error_handling::NeoError::InvalidInput(format!(
                "Unsupported export format: {}",
                format
            ))
        })?;

        exporter.export(&report)
    }
}

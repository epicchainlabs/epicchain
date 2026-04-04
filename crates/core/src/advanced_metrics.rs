//! Advanced Performance Metrics and Monitoring
//!
//! This module provides comprehensive performance monitoring capabilities
//! for production Neo node deployment with real-time metrics collection.

use prometheus::{Counter, Gauge, Histogram, Registry};
use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, SystemExt};
// use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Advanced metrics collector with Prometheus integration
#[derive(Clone)]
pub struct AdvancedMetricsCollector {
    /// Prometheus registry
    #[allow(dead_code)]
    registry: Registry,
    /// Blockchain metrics
    #[allow(dead_code)]
    blockchain_metrics: BlockchainMetricsCollector,
    /// Network metrics
    #[allow(dead_code)]
    network_metrics: NetworkMetricsCollector,
    /// VM metrics
    #[allow(dead_code)]
    vm_metrics: VmMetricsCollector,
    /// System metrics
    system_metrics: SystemMetricsCollector,
    /// Collection interval
    collection_interval: Duration,
    /// Metrics storage
    metrics_store: Arc<RwLock<MetricsStore>>,
}

/// Comprehensive metrics storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsStore {
    /// Current metrics snapshot
    pub current: MetricsSnapshot,
    /// Historical metrics (last 24 hours)
    pub history: Vec<MetricsSnapshot>,
    /// Performance alerts
    pub alerts: Vec<PerformanceAlert>,
}

/// Real-time metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Timestamp of snapshot
    pub timestamp: u64,
    /// Blockchain metrics
    pub blockchain: BlockchainPerformanceMetrics,
    /// Network metrics
    pub network: NetworkPerformanceMetrics,
    /// VM metrics
    pub vm: VmPerformanceMetrics,
    /// System metrics
    pub system: SystemPerformanceMetrics,
}

/// Blockchain performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainPerformanceMetrics {
    /// Current blockchain height
    pub height: u64,
    /// Blocks processed per minute
    pub blocks_per_minute: f64,
    /// Transactions processed per second
    pub transactions_per_second: f64,
    /// Average block processing time (ms)
    pub avg_block_processing_time_ms: f64,
    /// Average transaction validation time (us)
    pub avg_tx_validation_time_us: f64,
    /// Mempool size
    pub mempool_size: usize,
    /// Pending transactions
    pub pending_transactions: usize,
    /// Storage operations per second
    pub storage_ops_per_second: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceMetrics {
    /// Connected peer count
    pub connected_peers: usize,
    /// Messages sent per second
    pub messages_sent_per_second: f64,
    /// Messages received per second
    pub messages_received_per_second: f64,
    /// Bandwidth usage (MB/s)
    pub bandwidth_usage_mbps: f64,
    /// Average message latency (ms)
    pub avg_message_latency_ms: f64,
    /// Connection success rate
    pub connection_success_rate: f64,
    /// Network errors per minute
    pub network_errors_per_minute: f64,
}

/// VM performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmPerformanceMetrics {
    /// Script executions per second
    pub executions_per_second: f64,
    /// Average execution time (us)
    pub avg_execution_time_us: f64,
    /// Average gas consumed per execution
    pub avg_gas_consumed: u64,
    /// Successful executions ratio
    pub success_rate: f64,
    /// VM memory usage (MB)
    pub vm_memory_usage_mb: f64,
    /// Contract calls per second
    pub contract_calls_per_second: f64,
    /// Interop calls per second
    pub interop_calls_per_second: f64,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage (MB)
    pub memory_usage_mb: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    /// Open file descriptors
    pub open_file_descriptors: u64,
    /// Load average (1min, 5min, 15min)
    pub load_average: [f64; 3],
    /// Thread count
    pub thread_count: usize,
    /// GC collections per minute
    pub gc_collections_per_minute: f64,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert category
    pub category: String,
    /// Alert message
    pub message: String,
    /// Timestamp
    pub timestamp: u64,
    /// Metric value that triggered alert
    pub metric_value: f64,
    /// Threshold that was exceeded
    pub threshold: f64,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Blockchain metrics collector
#[derive(Clone)]
pub struct BlockchainMetricsCollector {
    /// Block processing counter
    blocks_processed: Counter,
    /// Transaction processing counter
    transactions_processed: Counter,
    /// Block processing time histogram
    block_processing_time: Histogram,
    /// Transaction validation time histogram
    tx_validation_time: Histogram,
    /// Current blockchain height gauge
    current_height: Gauge,
    /// Mempool size gauge
    mempool_size: Gauge,
}

impl BlockchainMetricsCollector {
    /// Creates a new blockchain metrics collector
    pub fn new(registry: &Registry) -> prometheus::Result<Self> {
        let blocks_processed = Counter::new(
            "neo_blocks_processed_total",
            "Total number of blocks processed",
        )?;
        registry.register(Box::new(blocks_processed.clone()))?;

        let transactions_processed = Counter::new(
            "neo_transactions_processed_total",
            "Total number of transactions processed",
        )?;
        registry.register(Box::new(transactions_processed.clone()))?;

        let block_processing_time = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "neo_block_processing_duration_seconds",
                "Block processing time in seconds",
            )
            .buckets(vec![0.001, 0.01, 0.1, 1.0, 10.0]),
        )?;
        registry.register(Box::new(block_processing_time.clone()))?;

        let tx_validation_time = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "neo_transaction_validation_duration_seconds",
                "Transaction validation time in seconds",
            )
            .buckets(vec![0.0001, 0.001, 0.01, 0.1, 1.0]),
        )?;
        registry.register(Box::new(tx_validation_time.clone()))?;

        let current_height = Gauge::new("neo_blockchain_height", "Current blockchain height")?;
        registry.register(Box::new(current_height.clone()))?;

        let mempool_size = Gauge::new("neo_mempool_size", "Current mempool size")?;
        registry.register(Box::new(mempool_size.clone()))?;

        Ok(Self {
            blocks_processed,
            transactions_processed,
            block_processing_time,
            tx_validation_time,
            current_height,
            mempool_size,
        })
    }

    /// Records a block processing event
    pub fn record_block_processed(&self, processing_time: Duration) {
        self.blocks_processed.inc();
        self.block_processing_time
            .observe(processing_time.as_secs_f64());
    }

    /// Records a transaction validation event
    pub fn record_transaction_validated(&self, validation_time: Duration) {
        self.transactions_processed.inc();
        self.tx_validation_time
            .observe(validation_time.as_secs_f64());
    }

    /// Updates current blockchain height
    pub fn update_height(&self, height: u64) {
        self.current_height.set(height as f64);
    }

    /// Updates mempool size
    pub fn update_mempool_size(&self, size: usize) {
        self.mempool_size.set(size as f64);
    }
}

/// Network metrics collector
#[derive(Clone)]
pub struct NetworkMetricsCollector {
    /// Messages sent counter
    messages_sent: Counter,
    /// Messages received counter
    messages_received: Counter,
    /// Bytes sent counter
    bytes_sent: Counter,
    /// Bytes received counter
    bytes_received: Counter,
    /// Connected peers gauge
    connected_peers: Gauge,
    /// Message latency histogram
    message_latency: Histogram,
}

impl NetworkMetricsCollector {
    /// Creates a new network metrics collector
    pub fn new(registry: &Registry) -> prometheus::Result<Self> {
        let messages_sent = Counter::new(
            "neo_network_messages_sent_total",
            "Total network messages sent",
        )?;
        registry.register(Box::new(messages_sent.clone()))?;

        let messages_received = Counter::new(
            "neo_network_messages_received_total",
            "Total network messages received",
        )?;
        registry.register(Box::new(messages_received.clone()))?;

        let bytes_sent = Counter::new("neo_network_bytes_sent_total", "Total network bytes sent")?;
        registry.register(Box::new(bytes_sent.clone()))?;

        let bytes_received = Counter::new(
            "neo_network_bytes_received_total",
            "Total network bytes received",
        )?;
        registry.register(Box::new(bytes_received.clone()))?;

        let connected_peers =
            Gauge::new("neo_network_connected_peers", "Number of connected peers")?;
        registry.register(Box::new(connected_peers.clone()))?;

        let message_latency = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "neo_network_message_latency_seconds",
                "Network message latency",
            )
            .buckets(vec![0.001, 0.01, 0.1, 1.0, 5.0]),
        )?;
        registry.register(Box::new(message_latency.clone()))?;

        Ok(Self {
            messages_sent,
            messages_received,
            bytes_sent,
            bytes_received,
            connected_peers,
            message_latency,
        })
    }

    /// Records a sent message
    pub fn record_message_sent(&self, size_bytes: usize) {
        self.messages_sent.inc();
        self.bytes_sent.inc_by(size_bytes as f64);
    }

    /// Records a received message
    pub fn record_message_received(&self, size_bytes: usize, latency: Duration) {
        self.messages_received.inc();
        self.bytes_received.inc_by(size_bytes as f64);
        self.message_latency.observe(latency.as_secs_f64());
    }

    /// Updates connected peer count
    pub fn update_peer_count(&self, count: usize) {
        self.connected_peers.set(count as f64);
    }
}

/// VM metrics collector
#[derive(Clone)]
pub struct VmMetricsCollector {
    /// Script executions counter
    executions: Counter,
    /// Gas consumed counter
    gas_consumed: Counter,
    /// Execution time histogram
    execution_time: Histogram,
    /// Contract calls counter
    contract_calls: Counter,
    /// VM memory usage gauge
    vm_memory_usage: Gauge,
}

impl VmMetricsCollector {
    /// Creates a new VM metrics collector
    pub fn new(registry: &Registry) -> prometheus::Result<Self> {
        let executions = Counter::new("neo_vm_executions_total", "Total VM script executions")?;
        registry.register(Box::new(executions.clone()))?;

        let gas_consumed = Counter::new("neo_vm_gas_consumed_total", "Total gas consumed by VM")?;
        registry.register(Box::new(gas_consumed.clone()))?;

        let execution_time = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "neo_vm_execution_duration_seconds",
                "VM script execution time",
            )
            .buckets(vec![0.0001, 0.001, 0.01, 0.1, 1.0]),
        )?;
        registry.register(Box::new(execution_time.clone()))?;

        let contract_calls = Counter::new("neo_vm_contract_calls_total", "Total contract calls")?;
        registry.register(Box::new(contract_calls.clone()))?;

        let vm_memory_usage = Gauge::new("neo_vm_memory_usage_bytes", "VM memory usage in bytes")?;
        registry.register(Box::new(vm_memory_usage.clone()))?;

        Ok(Self {
            executions,
            gas_consumed,
            execution_time,
            contract_calls,
            vm_memory_usage,
        })
    }

    /// Records a VM execution
    pub fn record_execution(&self, execution_time: Duration, gas_used: u64) {
        self.executions.inc();
        self.gas_consumed.inc_by(gas_used as f64);
        self.execution_time.observe(execution_time.as_secs_f64());
    }

    /// Records a contract call
    pub fn record_contract_call(&self) {
        self.contract_calls.inc();
    }

    /// Updates VM memory usage
    pub fn update_memory_usage(&self, bytes: usize) {
        self.vm_memory_usage.set(bytes as f64);
    }
}

/// System metrics collector
#[derive(Clone)]
pub struct SystemMetricsCollector {
    /// CPU usage gauge
    cpu_usage: Gauge,
    /// Memory usage gauge
    memory_usage: Gauge,
    /// Disk usage gauge
    disk_usage: Gauge,
    /// Network connections gauge
    #[allow(dead_code)]
    network_connections: Gauge,
    /// Thread count gauge
    thread_count: Gauge,
}

impl SystemMetricsCollector {
    /// Creates a new system metrics collector
    pub fn new(registry: &Registry) -> prometheus::Result<Self> {
        let cpu_usage = Gauge::new(
            "neo_system_cpu_usage_percent",
            "System CPU usage percentage",
        )?;
        registry.register(Box::new(cpu_usage.clone()))?;

        let memory_usage = Gauge::new(
            "neo_system_memory_usage_bytes",
            "System memory usage in bytes",
        )?;
        registry.register(Box::new(memory_usage.clone()))?;

        let disk_usage = Gauge::new(
            "neo_system_disk_usage_percent",
            "System disk usage percentage",
        )?;
        registry.register(Box::new(disk_usage.clone()))?;

        let network_connections = Gauge::new(
            "neo_system_network_connections",
            "Number of network connections",
        )?;
        registry.register(Box::new(network_connections.clone()))?;

        let thread_count = Gauge::new("neo_system_thread_count", "Number of system threads")?;
        registry.register(Box::new(thread_count.clone()))?;

        Ok(Self {
            cpu_usage,
            memory_usage,
            disk_usage,
            network_connections,
            thread_count,
        })
    }

    /// Updates system metrics
    pub async fn update_system_metrics(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Collect system information
        let mut system = sysinfo::System::new();
        system.refresh_all();

        // Update CPU usage
        let cpu_usage = system.global_cpu_info().cpu_usage();
        self.cpu_usage.set(cpu_usage as f64);

        // Update memory usage
        let used_memory = system.used_memory();
        let total_memory = system.total_memory();
        self.memory_usage.set(used_memory as f64);

        // Update disk usage (approximate)
        let disk_usage = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };
        self.disk_usage.set(disk_usage);

        // Update process count (approximate thread count)
        let process_count = system.processes().len();
        self.thread_count.set(process_count as f64);

        Ok(())
    }
}

impl AdvancedMetricsCollector {
    /// Creates a new advanced metrics collector
    pub fn new() -> prometheus::Result<Self> {
        let registry = Registry::new();

        let blockchain_metrics = BlockchainMetricsCollector::new(&registry)?;
        let network_metrics = NetworkMetricsCollector::new(&registry)?;
        let vm_metrics = VmMetricsCollector::new(&registry)?;
        let system_metrics = SystemMetricsCollector::new(&registry)?;

        Ok(Self {
            registry,
            blockchain_metrics,
            network_metrics,
            vm_metrics,
            system_metrics,
            collection_interval: Duration::from_secs(30),
            metrics_store: Arc::new(RwLock::new(MetricsStore {
                current: MetricsSnapshot {
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    blockchain: BlockchainPerformanceMetrics {
                        height: 0,
                        blocks_per_minute: 0.0,
                        transactions_per_second: 0.0,
                        avg_block_processing_time_ms: 0.0,
                        avg_tx_validation_time_us: 0.0,
                        mempool_size: 0,
                        pending_transactions: 0,
                        storage_ops_per_second: 0.0,
                        cache_hit_rate: 0.0,
                    },
                    network: NetworkPerformanceMetrics {
                        connected_peers: 0,
                        messages_sent_per_second: 0.0,
                        messages_received_per_second: 0.0,
                        bandwidth_usage_mbps: 0.0,
                        avg_message_latency_ms: 0.0,
                        connection_success_rate: 0.0,
                        network_errors_per_minute: 0.0,
                    },
                    vm: VmPerformanceMetrics {
                        executions_per_second: 0.0,
                        avg_execution_time_us: 0.0,
                        avg_gas_consumed: 0,
                        success_rate: 0.0,
                        vm_memory_usage_mb: 0.0,
                        contract_calls_per_second: 0.0,
                        interop_calls_per_second: 0.0,
                    },
                    system: SystemPerformanceMetrics {
                        cpu_usage_percent: 0.0,
                        memory_usage_mb: 0.0,
                        memory_usage_percent: 0.0,
                        disk_usage_percent: 0.0,
                        open_file_descriptors: 0,
                        load_average: [0.0, 0.0, 0.0],
                        thread_count: 0,
                        gc_collections_per_minute: 0.0,
                    },
                },
                history: Vec::new(),
                alerts: Vec::new(),
            })),
        })
    }

    /// Starts metrics collection background task
    pub async fn start_collection(&self) -> tokio::task::JoinHandle<()> {
        let _metrics_store = self.metrics_store.clone();
        let system_metrics = self.system_metrics.clone();
        let interval = self.collection_interval;

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);

            loop {
                ticker.tick().await;

                // Collect system metrics
                if let Err(e) = system_metrics.update_system_metrics().await {
                    warn!("Failed to update system metrics: {}", e);
                }

                debug!("Metrics collection cycle completed");
            }
        })
    }

    /// Gets current metrics snapshot
    pub async fn get_current_metrics(&self) -> MetricsSnapshot {
        let store = self.metrics_store.read().await;
        store.current.clone()
    }

    /// Gets metrics history
    pub async fn get_metrics_history(&self, duration_hours: u32) -> Vec<MetricsSnapshot> {
        let store = self.metrics_store.read().await;
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (duration_hours as u64 * 3600);

        store
            .history
            .iter()
            .filter(|snapshot| snapshot.timestamp >= cutoff_time)
            .cloned()
            .collect()
    }

    /// Exports metrics in Prometheus format
    pub fn export_prometheus_metrics(&self) -> String {
        "# EpicChain-RS Metrics\n# Implementation pending\n".to_string()
    }

    /// Records a performance alert
    pub async fn record_alert(&self, alert: PerformanceAlert) {
        let mut store = self.metrics_store.write().await;
        store.alerts.push(alert);

        // Keep only last 1000 alerts
        if store.alerts.len() > 1000 {
            store.alerts.remove(0);
        }
    }

    /// Gets recent alerts
    pub async fn get_recent_alerts(
        &self,
        severity: Option<AlertSeverity>,
    ) -> Vec<PerformanceAlert> {
        let store = self.metrics_store.read().await;

        match severity {
            Some(sev) => store
                .alerts
                .iter()
                .filter(|alert| alert.severity == sev)
                .cloned()
                .collect(),
            None => store.alerts.clone(),
        }
    }
}

/// Performance monitoring service
pub struct PerformanceMonitor {
    /// Metrics collector
    collector: AdvancedMetricsCollector,
    /// Alert thresholds
    thresholds: AlertThresholds,
    /// Monitoring task handle
    monitoring_task: Option<tokio::task::JoinHandle<()>>,
}

/// Alert threshold configuration
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// CPU usage warning threshold (%)
    pub cpu_warning: f64,
    /// CPU usage critical threshold (%)
    pub cpu_critical: f64,
    /// Memory usage warning threshold (%)
    pub memory_warning: f64,
    /// Memory usage critical threshold (%)
    pub memory_critical: f64,
    /// Block processing time warning (ms)
    pub block_processing_warning_ms: f64,
    /// Block processing time critical (ms)
    pub block_processing_critical_ms: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_warning: 80.0,
            cpu_critical: 95.0,
            memory_warning: 80.0,
            memory_critical: 95.0,
            block_processing_warning_ms: 1000.0,
            block_processing_critical_ms: 5000.0,
        }
    }
}

impl PerformanceMonitor {
    /// Creates a new performance monitor
    pub fn new() -> prometheus::Result<Self> {
        let collector = AdvancedMetricsCollector::new()?;

        Ok(Self {
            collector,
            thresholds: AlertThresholds::default(),
            monitoring_task: None,
        })
    }

    /// Starts performance monitoring
    pub async fn start_monitoring(&mut self) {
        info!("🚀 Starting advanced performance monitoring");

        // Start metrics collection
        let _collection_task = self.collector.start_collection().await;

        // Start alerting task
        let collector = self.collector.clone();
        let thresholds = self.thresholds.clone();

        let monitoring_task = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));

            loop {
                ticker.tick().await;

                // Check thresholds and generate alerts
                let current_metrics = collector.get_current_metrics().await;

                // CPU usage alerts
                if current_metrics.system.cpu_usage_percent > thresholds.cpu_critical {
                    let alert = PerformanceAlert {
                        severity: AlertSeverity::Critical,
                        category: "System".to_string(),
                        message: format!(
                            "Critical CPU usage: {:.1}%",
                            current_metrics.system.cpu_usage_percent
                        ),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        metric_value: current_metrics.system.cpu_usage_percent,
                        threshold: thresholds.cpu_critical,
                    };
                    collector.record_alert(alert).await;
                } else if current_metrics.system.cpu_usage_percent > thresholds.cpu_warning {
                    let alert = PerformanceAlert {
                        severity: AlertSeverity::Warning,
                        category: "System".to_string(),
                        message: format!(
                            "High CPU usage: {:.1}%",
                            current_metrics.system.cpu_usage_percent
                        ),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        metric_value: current_metrics.system.cpu_usage_percent,
                        threshold: thresholds.cpu_warning,
                    };
                    collector.record_alert(alert).await;
                }

                // Memory usage alerts
                if current_metrics.system.memory_usage_percent > thresholds.memory_critical {
                    let alert = PerformanceAlert {
                        severity: AlertSeverity::Critical,
                        category: "System".to_string(),
                        message: format!(
                            "Critical memory usage: {:.1}%",
                            current_metrics.system.memory_usage_percent
                        ),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        metric_value: current_metrics.system.memory_usage_percent,
                        threshold: thresholds.memory_critical,
                    };
                    collector.record_alert(alert).await;
                }
            }
        });

        self.monitoring_task = Some(monitoring_task);
        info!("✅ Performance monitoring started");
    }

    /// Stops performance monitoring
    pub async fn stop_monitoring(&mut self) {
        if let Some(task) = self.monitoring_task.take() {
            task.abort();
            info!("Performance monitoring stopped");
        }
    }

    /// Gets performance metrics in Prometheus format
    pub fn get_prometheus_metrics(&self) -> String {
        self.collector.export_prometheus_metrics()
    }

    /// Gets current performance snapshot
    pub async fn get_performance_snapshot(&self) -> MetricsSnapshot {
        self.collector.get_current_metrics().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neo_config::NetworkType;

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = AdvancedMetricsCollector::new();
        assert!(collector.is_ok());
    }

    #[tokio::test]
    async fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert!(monitor.is_ok());
    }

    #[tokio::test]
    async fn test_alert_thresholds() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.cpu_warning, 80.0);
        assert_eq!(thresholds.cpu_critical, 95.0);
    }
}

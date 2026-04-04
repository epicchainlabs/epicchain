//! Metrics collection for Neo node monitoring
//!
//! Provides Prometheus-compatible metrics for monitoring node health,
//! performance, and network activity.

use lazy_static::lazy_static;
use prometheus::{
    Counter, Encoder, Gauge, Histogram, HistogramOpts, HistogramVec, IntCounter, IntCounterVec,
    IntGauge, IntGaugeVec, Opts, Registry, TextEncoder,
};
use std::sync::RwLock;

lazy_static! {
    pub static ref REGISTRY: RwLock<Registry> = RwLock::new(Registry::new());

    // Blockchain metrics
    pub static ref BLOCK_HEIGHT: IntGauge = IntGauge::new(
        "neo_block_height", "Current blockchain height"
    ).expect("metric can be created");

    pub static ref BLOCK_PROCESSING_TIME: Histogram = Histogram::with_opts(
        HistogramOpts::new("neo_block_processing_time_seconds", "Time to process a block")
    ).expect("metric can be created");

    pub static ref BLOCKS_PROCESSED: IntCounter = IntCounter::new(
        "neo_blocks_processed_total", "Total number of blocks processed"
    ).expect("metric can be created");

    // Transaction metrics
    pub static ref TX_POOL_SIZE: IntGauge = IntGauge::new(
        "neo_tx_pool_size", "Number of transactions in mempool"
    ).expect("metric can be created");

    pub static ref TX_PROCESSED: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_tx_processed_total", "Total transactions processed"),
        &["type"]
    ).expect("metric can be created");

    pub static ref TX_VALIDATION_TIME: Histogram = Histogram::with_opts(
        HistogramOpts::new("neo_tx_validation_time_seconds", "Transaction validation time")
    ).expect("metric can be created");

    // Network metrics
    pub static ref PEER_COUNT: IntGaugeVec = IntGaugeVec::new(
        Opts::new("neo_peer_count", "Number of connected peers"),
        &["state"]
    ).expect("metric can be created");

    pub static ref MESSAGES_RECEIVED: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_messages_received_total", "Total messages received"),
        &["type"]
    ).expect("metric can be created");

    pub static ref MESSAGES_SENT: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_messages_sent_total", "Total messages sent"),
        &["type"]
    ).expect("metric can be created");

    pub static ref BYTES_RECEIVED: Counter = Counter::new(
        "neo_bytes_received_total", "Total bytes received"
    ).expect("metric can be created");

    pub static ref BYTES_SENT: Counter = Counter::new(
        "neo_bytes_sent_total", "Total bytes sent"
    ).expect("metric can be created");

    // Consensus metrics
    pub static ref CONSENSUS_VIEW: IntGauge = IntGauge::new(
        "neo_consensus_view", "Current consensus view number"
    ).expect("metric can be created");

    pub static ref CONSENSUS_ROUNDS: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_consensus_rounds_total", "Consensus rounds"),
        &["result"]
    ).expect("metric can be created");

    pub static ref CONSENSUS_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("neo_consensus_duration_seconds", "Time to reach consensus")
    ).expect("metric can be created");

    // VM metrics
    pub static ref VM_EXECUTIONS: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_vm_executions_total", "VM executions"),
        &["result"]
    ).expect("metric can be created");

    pub static ref VM_GAS_CONSUMED: Counter = Counter::new(
        "neo_vm_gas_consumed_total", "Total GAS consumed by VM"
    ).expect("metric can be created");

    pub static ref VM_EXECUTION_TIME: Histogram = Histogram::with_opts(
        HistogramOpts::new("neo_vm_execution_time_seconds", "VM execution time")
    ).expect("metric can be created");

    // Storage metrics
    pub static ref STORAGE_SIZE: IntGaugeVec = IntGaugeVec::new(
        Opts::new("neo_storage_size_bytes", "Storage size in bytes"),
        &["type"]
    ).expect("metric can be created");

    pub static ref STORAGE_OPERATIONS: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_storage_operations_total", "Storage operations"),
        &["operation", "result"]
    ).expect("metric can be created");

    // RPC metrics
    pub static ref RPC_REQUESTS: IntCounterVec = IntCounterVec::new(
        Opts::new("neo_rpc_requests_total", "RPC requests"),
        &["method", "status"]
    ).expect("metric can be created");

    pub static ref RPC_REQUEST_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("neo_rpc_request_duration_seconds", "RPC request duration"),
        &["method"]
    ).expect("metric can be created");

    // System metrics
    pub static ref UPTIME: IntGauge = IntGauge::new(
        "neo_uptime_seconds", "Node uptime in seconds"
    ).expect("metric can be created");

    pub static ref MEMORY_USAGE: IntGauge = IntGauge::new(
        "neo_memory_usage_bytes", "Memory usage in bytes"
    ).expect("metric can be created");

    pub static ref CPU_USAGE: Gauge = Gauge::new(
        "neo_cpu_usage_percent", "CPU usage percentage"
    ).expect("metric can be created");
}

/// Initialize all metrics and register them
pub fn init_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let registry = REGISTRY.write().unwrap();

    // Register blockchain metrics
    registry.register(Box::new(BLOCK_HEIGHT.clone()))?;
    registry.register(Box::new(BLOCK_PROCESSING_TIME.clone()))?;
    registry.register(Box::new(BLOCKS_PROCESSED.clone()))?;

    // Register transaction metrics
    registry.register(Box::new(TX_POOL_SIZE.clone()))?;
    registry.register(Box::new(TX_PROCESSED.clone()))?;
    registry.register(Box::new(TX_VALIDATION_TIME.clone()))?;

    // Register network metrics
    registry.register(Box::new(PEER_COUNT.clone()))?;
    registry.register(Box::new(MESSAGES_RECEIVED.clone()))?;
    registry.register(Box::new(MESSAGES_SENT.clone()))?;
    registry.register(Box::new(BYTES_RECEIVED.clone()))?;
    registry.register(Box::new(BYTES_SENT.clone()))?;

    // Register consensus metrics
    registry.register(Box::new(CONSENSUS_VIEW.clone()))?;
    registry.register(Box::new(CONSENSUS_ROUNDS.clone()))?;
    registry.register(Box::new(CONSENSUS_DURATION.clone()))?;

    // Register VM metrics
    registry.register(Box::new(VM_EXECUTIONS.clone()))?;
    registry.register(Box::new(VM_GAS_CONSUMED.clone()))?;
    registry.register(Box::new(VM_EXECUTION_TIME.clone()))?;

    // Register storage metrics
    registry.register(Box::new(STORAGE_SIZE.clone()))?;
    registry.register(Box::new(STORAGE_OPERATIONS.clone()))?;

    // Register RPC metrics
    registry.register(Box::new(RPC_REQUESTS.clone()))?;
    registry.register(Box::new(RPC_REQUEST_DURATION.clone()))?;

    // Register system metrics
    registry.register(Box::new(UPTIME.clone()))?;
    registry.register(Box::new(MEMORY_USAGE.clone()))?;
    registry.register(Box::new(CPU_USAGE.clone()))?;

    Ok(())
}

/// Get metrics in Prometheus text format
/// Gets a value from the internal state.
pub fn get_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.read().unwrap().gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Update blockchain metrics
/// Updates the internal state.
pub fn update_blockchain_metrics(height: u64, processing_time: f64) {
    BLOCK_HEIGHT.set(height as i64);
    BLOCK_PROCESSING_TIME.observe(processing_time);
    BLOCKS_PROCESSED.inc();
}

/// Update transaction metrics
/// Updates the internal state.
pub fn update_tx_metrics(tx_type: &str, validation_time: f64) {
    TX_PROCESSED.with_label_values(&[tx_type]).inc();
    TX_VALIDATION_TIME.observe(validation_time);
}

/// Update network metrics
/// Updates the internal state.
pub fn update_network_metrics(msg_type: &str, is_received: bool, bytes: usize) {
    if is_received {
        MESSAGES_RECEIVED.with_label_values(&[msg_type]).inc();
        BYTES_RECEIVED.inc_by(bytes as f64);
    } else {
        MESSAGES_SENT.with_label_values(&[msg_type]).inc();
        BYTES_SENT.inc_by(bytes as f64);
    }
}

/// Update consensus metrics
/// Updates the internal state.
pub fn update_consensus_metrics(view: u64, duration: f64, success: bool) {
    CONSENSUS_VIEW.set(view as i64);
    CONSENSUS_DURATION.observe(duration);
    let result = if success { "success" } else { "failure" };
    CONSENSUS_ROUNDS.with_label_values(&[result]).inc();
}

/// Update VM metrics
/// Updates the internal state.
pub fn update_vm_metrics(gas_consumed: u64, execution_time: f64, success: bool) {
    VM_GAS_CONSUMED.inc_by(gas_consumed as f64);
    VM_EXECUTION_TIME.observe(execution_time);
    let result = if success { "success" } else { "failure" };
    VM_EXECUTIONS.with_label_values(&[result]).inc();
}

/// Update peer count
/// Updates the internal state.
pub fn update_peer_count(connected: i64, connecting: i64, disconnected: i64) {
    PEER_COUNT.with_label_values(&["connected"]).set(connected);
    PEER_COUNT
        .with_label_values(&["connecting"])
        .set(connecting);
    PEER_COUNT
        .with_label_values(&["disconnected"])
        .set(disconnected);
}

/// Update system metrics
/// Updates the internal state.
pub fn update_system_metrics() {
    use std::process;
    use sysinfo::{CpuExt, Pid, ProcessExt, System, SystemExt};

    let mut system = System::new_all();
    system.refresh_all();

    // Update uptime
    let pid = Pid::from(process::id() as usize);
    if let Some(process) = system.process(pid) {
        UPTIME.set(process.run_time() as i64);
        MEMORY_USAGE.set(process.memory() as i64 * 1024); // Convert KB to bytes
    }

    // Update CPU usage
    system.refresh_cpu();
    let cpu_usage = system.global_cpu_info().cpu_usage();
    CPU_USAGE.set(cpu_usage as f64);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        assert!(init_metrics().is_ok());
    }

    #[test]
    fn test_blockchain_metrics_update() {
        init_metrics().unwrap();
        update_blockchain_metrics(1000, 0.5);
        assert_eq!(BLOCK_HEIGHT.get(), 1000);
        assert_eq!(BLOCKS_PROCESSED.get(), 1);
    }

    #[test]
    fn test_metrics_export() {
        init_metrics().unwrap();
        update_blockchain_metrics(100, 0.1);
        let metrics = get_metrics();
        assert!(metrics.contains("neo_block_height 100"));
    }
}

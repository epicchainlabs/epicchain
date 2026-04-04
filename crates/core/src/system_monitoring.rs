//! System-wide monitoring and metrics collection
//!
//! This module provides comprehensive monitoring for all blockchain components
//! including performance metrics, error tracking, and health monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// System-wide metrics collector
/// Represents a data structure.
/// Represents a data structure.
pub struct SystemMonitor {
    /// Transaction metrics
    pub transactions: TransactionMetrics,
    /// Block metrics
    pub blocks: BlockMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// VM execution metrics
    pub vm: VmMetrics,
    /// Consensus metrics
    pub consensus: ConsensusMetrics,
    /// Storage metrics
    pub storage: StorageMetrics,
    /// Error tracking
    pub errors: ErrorTracker,
    /// Performance tracking
    pub performance: PerformanceTracker,
}

impl SystemMonitor {
    /// Create a new system monitor
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            transactions: TransactionMetrics::new(),
            blocks: BlockMetrics::new(),
            network: NetworkMetrics::new(),
            vm: VmMetrics::new(),
            consensus: ConsensusMetrics::new(),
            storage: StorageMetrics::new(),
            errors: ErrorTracker::new(),
            performance: PerformanceTracker::new(),
        }
    }

    /// Get a snapshot of all metrics
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> SystemMetricsSnapshot {
        SystemMetricsSnapshot {
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            transactions: self.transactions.snapshot(),
            blocks: self.blocks.snapshot(),
            network: self.network.snapshot(),
            vm: self.vm.snapshot(),
            consensus: self.consensus.snapshot(),
            storage: self.storage.snapshot(),
            errors: self.errors.snapshot(),
            performance: self.performance.snapshot(),
        }
    }

    /// Reset all metrics
    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.transactions.reset();
        self.blocks.reset();
        self.network.reset();
        self.vm.reset();
        self.consensus.reset();
        self.storage.reset();
        self.errors.reset();
        self.performance.reset();
    }
}

/// Transaction metrics
/// Represents a data structure.
/// Represents a data structure.
pub struct TransactionMetrics {
    total_count: AtomicU64,
    verified_count: AtomicU64,
    failed_count: AtomicU64,
    total_size_bytes: AtomicU64,
    average_verification_time_us: AtomicU64,
    mempool_size: AtomicUsize,
}

impl TransactionMetrics {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            total_count: AtomicU64::new(0),
            verified_count: AtomicU64::new(0),
            failed_count: AtomicU64::new(0),
            total_size_bytes: AtomicU64::new(0),
            average_verification_time_us: AtomicU64::new(0),
            mempool_size: AtomicUsize::new(0),
        }
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_transaction(&self, size: u64, verification_time: Duration, success: bool) {
        self.total_count.fetch_add(1, Ordering::Relaxed);
        self.total_size_bytes.fetch_add(size, Ordering::Relaxed);

        if success {
            self.verified_count.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_count.fetch_add(1, Ordering::Relaxed);
        }

        // Update average verification time
        let new_time = verification_time.as_micros() as u64;
        let current_avg = self.average_verification_time_us.load(Ordering::Relaxed);
        let count = self.total_count.load(Ordering::Relaxed);
        let new_avg = ((current_avg * (count - 1)) + new_time) / count;
        self.average_verification_time_us
            .store(new_avg, Ordering::Relaxed);
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_mempool_size(&self, size: usize) {
        self.mempool_size.store(size, Ordering::Relaxed);
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> TransactionMetricsSnapshot {
        TransactionMetricsSnapshot {
            total_count: self.total_count.load(Ordering::Relaxed),
            verified_count: self.verified_count.load(Ordering::Relaxed),
            failed_count: self.failed_count.load(Ordering::Relaxed),
            total_size_bytes: self.total_size_bytes.load(Ordering::Relaxed),
            average_verification_time_us: self.average_verification_time_us.load(Ordering::Relaxed),
            mempool_size: self.mempool_size.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.total_count.store(0, Ordering::Relaxed);
        self.verified_count.store(0, Ordering::Relaxed);
        self.failed_count.store(0, Ordering::Relaxed);
        self.total_size_bytes.store(0, Ordering::Relaxed);
        self.average_verification_time_us
            .store(0, Ordering::Relaxed);
    }
}

/// Block metrics
/// Represents a data structure.
/// Represents a data structure.
pub struct BlockMetrics {
    total_count: AtomicU64,
    current_height: AtomicU64,
    average_block_time_ms: AtomicU64,
    average_block_size_bytes: AtomicU64,
    average_tx_per_block: AtomicU64,
    last_block_time: RwLock<Option<std::time::Instant>>,
}

impl BlockMetrics {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            total_count: AtomicU64::new(0),
            current_height: AtomicU64::new(0),
            average_block_time_ms: AtomicU64::new(0),
            average_block_size_bytes: AtomicU64::new(0),
            average_tx_per_block: AtomicU64::new(0),
            last_block_time: RwLock::new(None),
        }
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_block(&self, height: u64, size: u64, tx_count: u64) {
        self.total_count.fetch_add(1, Ordering::Relaxed);
        self.current_height.store(height, Ordering::Relaxed);

        // Calculate block time
        let now = std::time::Instant::now();
        if let Ok(mut last_time) = self.last_block_time.write() {
            if let Some(last) = *last_time {
                let block_time_ms = now.duration_since(last).as_millis() as u64;

                // Update average block time
                let count = self.total_count.load(Ordering::Relaxed);
                let current_avg = self.average_block_time_ms.load(Ordering::Relaxed);
                let new_avg = ((current_avg * (count - 1)) + block_time_ms) / count;
                self.average_block_time_ms.store(new_avg, Ordering::Relaxed);
            }
            *last_time = Some(now);
        }

        // Update averages
        let count = self.total_count.load(Ordering::Relaxed);

        let current_size_avg = self.average_block_size_bytes.load(Ordering::Relaxed);
        let new_size_avg = ((current_size_avg * (count - 1)) + size) / count;
        self.average_block_size_bytes
            .store(new_size_avg, Ordering::Relaxed);

        let current_tx_avg = self.average_tx_per_block.load(Ordering::Relaxed);
        let new_tx_avg = ((current_tx_avg * (count - 1)) + tx_count) / count;
        self.average_tx_per_block
            .store(new_tx_avg, Ordering::Relaxed);
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> BlockMetricsSnapshot {
        BlockMetricsSnapshot {
            total_count: self.total_count.load(Ordering::Relaxed),
            current_height: self.current_height.load(Ordering::Relaxed),
            average_block_time_ms: self.average_block_time_ms.load(Ordering::Relaxed),
            average_block_size_bytes: self.average_block_size_bytes.load(Ordering::Relaxed),
            average_tx_per_block: self.average_tx_per_block.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.total_count.store(0, Ordering::Relaxed);
        self.average_block_time_ms.store(0, Ordering::Relaxed);
        self.average_block_size_bytes.store(0, Ordering::Relaxed);
        self.average_tx_per_block.store(0, Ordering::Relaxed);
        if let Ok(mut guard) = self.last_block_time.write() {
            *guard = None;
        }
    }
}

/// Network metrics
/// Represents a data structure.
/// Represents a data structure.
pub struct NetworkMetrics {
    peer_count: AtomicUsize,
    messages_sent: AtomicU64,
    messages_received: AtomicU64,
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,
    connection_failures: AtomicU64,
    average_latency_ms: AtomicU64,
}

impl NetworkMetrics {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            peer_count: AtomicUsize::new(0),
            messages_sent: AtomicU64::new(0),
            messages_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            connection_failures: AtomicU64::new(0),
            average_latency_ms: AtomicU64::new(0),
        }
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_peer_count(&self, count: usize) {
        self.peer_count.store(count, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_message_sent(&self, size: u64) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
        self.bytes_sent.fetch_add(size, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_message_received(&self, size: u64) {
        self.messages_received.fetch_add(1, Ordering::Relaxed);
        self.bytes_received.fetch_add(size, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_connection_failure(&self) {
        self.connection_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_average_latency(&self, latency_ms: u64) {
        let count = self.messages_sent.load(Ordering::Relaxed)
            + self.messages_received.load(Ordering::Relaxed);
        if count > 0 {
            let current_avg = self.average_latency_ms.load(Ordering::Relaxed);
            let new_avg = ((current_avg * (count - 1)) + latency_ms) / count;
            self.average_latency_ms.store(new_avg, Ordering::Relaxed);
        }
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> NetworkMetricsSnapshot {
        NetworkMetricsSnapshot {
            peer_count: self.peer_count.load(Ordering::Relaxed),
            messages_sent: self.messages_sent.load(Ordering::Relaxed),
            messages_received: self.messages_received.load(Ordering::Relaxed),
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            connection_failures: self.connection_failures.load(Ordering::Relaxed),
            average_latency_ms: self.average_latency_ms.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.messages_sent.store(0, Ordering::Relaxed);
        self.messages_received.store(0, Ordering::Relaxed);
        self.bytes_sent.store(0, Ordering::Relaxed);
        self.bytes_received.store(0, Ordering::Relaxed);
        self.connection_failures.store(0, Ordering::Relaxed);
        self.average_latency_ms.store(0, Ordering::Relaxed);
    }
}

/// VM execution metrics
/// Represents a data structure.
/// Represents a data structure.
pub struct VmMetrics {
    executions: AtomicU64,
    successful_executions: AtomicU64,
    failed_executions: AtomicU64,
    total_gas_consumed: AtomicU64,
    average_execution_time_us: AtomicU64,
    opcodes_executed: AtomicU64,
}

impl VmMetrics {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            executions: AtomicU64::new(0),
            successful_executions: AtomicU64::new(0),
            failed_executions: AtomicU64::new(0),
            total_gas_consumed: AtomicU64::new(0),
            average_execution_time_us: AtomicU64::new(0),
            opcodes_executed: AtomicU64::new(0),
        }
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_execution(&self, gas: u64, time: Duration, opcodes: u64, success: bool) {
        self.executions.fetch_add(1, Ordering::Relaxed);
        self.total_gas_consumed.fetch_add(gas, Ordering::Relaxed);
        self.opcodes_executed.fetch_add(opcodes, Ordering::Relaxed);

        if success {
            self.successful_executions.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_executions.fetch_add(1, Ordering::Relaxed);
        }

        // Update average execution time
        let count = self.executions.load(Ordering::Relaxed);
        let current_avg = self.average_execution_time_us.load(Ordering::Relaxed);
        let new_time = time.as_micros() as u64;
        let new_avg = ((current_avg * (count - 1)) + new_time) / count;
        self.average_execution_time_us
            .store(new_avg, Ordering::Relaxed);
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> VmMetricsSnapshot {
        VmMetricsSnapshot {
            executions: self.executions.load(Ordering::Relaxed),
            successful_executions: self.successful_executions.load(Ordering::Relaxed),
            failed_executions: self.failed_executions.load(Ordering::Relaxed),
            total_gas_consumed: self.total_gas_consumed.load(Ordering::Relaxed),
            average_execution_time_us: self.average_execution_time_us.load(Ordering::Relaxed),
            opcodes_executed: self.opcodes_executed.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.executions.store(0, Ordering::Relaxed);
        self.successful_executions.store(0, Ordering::Relaxed);
        self.failed_executions.store(0, Ordering::Relaxed);
        self.total_gas_consumed.store(0, Ordering::Relaxed);
        self.average_execution_time_us.store(0, Ordering::Relaxed);
        self.opcodes_executed.store(0, Ordering::Relaxed);
    }
}

/// Consensus metrics
/// Represents a data structure.
/// Represents a data structure.
pub struct ConsensusMetrics {
    view_changes: AtomicU64,
    blocks_proposed: AtomicU64,
    blocks_accepted: AtomicU64,
    blocks_rejected: AtomicU64,
    average_consensus_time_ms: AtomicU64,
    timeouts: AtomicU64,
}

impl ConsensusMetrics {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            view_changes: AtomicU64::new(0),
            blocks_proposed: AtomicU64::new(0),
            blocks_accepted: AtomicU64::new(0),
            blocks_rejected: AtomicU64::new(0),
            average_consensus_time_ms: AtomicU64::new(0),
            timeouts: AtomicU64::new(0),
        }
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_view_change(&self) {
        self.view_changes.fetch_add(1, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_block_proposal(&self, accepted: bool, consensus_time: Duration) {
        self.blocks_proposed.fetch_add(1, Ordering::Relaxed);

        if accepted {
            self.blocks_accepted.fetch_add(1, Ordering::Relaxed);
        } else {
            self.blocks_rejected.fetch_add(1, Ordering::Relaxed);
        }

        // Update average consensus time
        let count = self.blocks_proposed.load(Ordering::Relaxed);
        let current_avg = self.average_consensus_time_ms.load(Ordering::Relaxed);
        let new_time = consensus_time.as_millis() as u64;
        let new_avg = ((current_avg * (count - 1)) + new_time) / count;
        self.average_consensus_time_ms
            .store(new_avg, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_timeout(&self) {
        self.timeouts.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> ConsensusMetricsSnapshot {
        ConsensusMetricsSnapshot {
            view_changes: self.view_changes.load(Ordering::Relaxed),
            blocks_proposed: self.blocks_proposed.load(Ordering::Relaxed),
            blocks_accepted: self.blocks_accepted.load(Ordering::Relaxed),
            blocks_rejected: self.blocks_rejected.load(Ordering::Relaxed),
            average_consensus_time_ms: self.average_consensus_time_ms.load(Ordering::Relaxed),
            timeouts: self.timeouts.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.view_changes.store(0, Ordering::Relaxed);
        self.blocks_proposed.store(0, Ordering::Relaxed);
        self.blocks_accepted.store(0, Ordering::Relaxed);
        self.blocks_rejected.store(0, Ordering::Relaxed);
        self.average_consensus_time_ms.store(0, Ordering::Relaxed);
        self.timeouts.store(0, Ordering::Relaxed);
    }
}

/// Storage metrics
/// Represents a data structure.
/// Represents a data structure.
pub struct StorageMetrics {
    reads: AtomicU64,
    writes: AtomicU64,
    deletes: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    disk_usage_bytes: AtomicU64,
    average_read_time_us: AtomicU64,
    average_write_time_us: AtomicU64,
}

impl StorageMetrics {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            reads: AtomicU64::new(0),
            writes: AtomicU64::new(0),
            deletes: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            disk_usage_bytes: AtomicU64::new(0),
            average_read_time_us: AtomicU64::new(0),
            average_write_time_us: AtomicU64::new(0),
        }
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_read(&self, time: Duration, cache_hit: bool) {
        self.reads.fetch_add(1, Ordering::Relaxed);

        if cache_hit {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.cache_misses.fetch_add(1, Ordering::Relaxed);
        }

        // Update average read time
        let count = self.reads.load(Ordering::Relaxed);
        let current_avg = self.average_read_time_us.load(Ordering::Relaxed);
        let new_time = time.as_micros() as u64;
        let new_avg = ((current_avg * (count - 1)) + new_time) / count;
        self.average_read_time_us.store(new_avg, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_write(&self, time: Duration) {
        self.writes.fetch_add(1, Ordering::Relaxed);

        // Update average write time
        let count = self.writes.load(Ordering::Relaxed);
        let current_avg = self.average_write_time_us.load(Ordering::Relaxed);
        let new_time = time.as_micros() as u64;
        let new_avg = ((current_avg * (count - 1)) + new_time) / count;
        self.average_write_time_us.store(new_avg, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_delete(&self) {
        self.deletes.fetch_add(1, Ordering::Relaxed);
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_disk_usage(&self, bytes: u64) {
        self.disk_usage_bytes.store(bytes, Ordering::Relaxed);
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> StorageMetricsSnapshot {
        StorageMetricsSnapshot {
            reads: self.reads.load(Ordering::Relaxed),
            writes: self.writes.load(Ordering::Relaxed),
            deletes: self.deletes.load(Ordering::Relaxed),
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
            disk_usage_bytes: self.disk_usage_bytes.load(Ordering::Relaxed),
            average_read_time_us: self.average_read_time_us.load(Ordering::Relaxed),
            average_write_time_us: self.average_write_time_us.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.reads.store(0, Ordering::Relaxed);
        self.writes.store(0, Ordering::Relaxed);
        self.deletes.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
        self.average_read_time_us.store(0, Ordering::Relaxed);
        self.average_write_time_us.store(0, Ordering::Relaxed);
    }
}

/// Error tracking
/// Represents a data structure.
/// Represents a data structure.
pub struct ErrorTracker {
    errors_by_category: Arc<RwLock<HashMap<String, u64>>>,
    total_errors: AtomicU64,
    critical_errors: AtomicU64,
    warnings: AtomicU64,
}

impl ErrorTracker {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            errors_by_category: Arc::new(RwLock::new(HashMap::new())),
            total_errors: AtomicU64::new(0),
            critical_errors: AtomicU64::new(0),
            warnings: AtomicU64::new(0),
        }
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_error(&self, category: String, is_critical: bool) {
        self.total_errors.fetch_add(1, Ordering::Relaxed);

        if is_critical {
            self.critical_errors.fetch_add(1, Ordering::Relaxed);
        } else {
            self.warnings.fetch_add(1, Ordering::Relaxed);
        }

        if let Ok(mut guard) = self.errors_by_category.write() {
            *guard.entry(category).or_insert(0) += 1;
        }
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> ErrorTrackerSnapshot {
        ErrorTrackerSnapshot {
            errors_by_category: self
                .errors_by_category
                .read()
                .ok()
                .map(|g| g.clone())
                .unwrap_or_default(),
            total_errors: self.total_errors.load(Ordering::Relaxed),
            critical_errors: self.critical_errors.load(Ordering::Relaxed),
            warnings: self.warnings.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        if let Ok(mut guard) = self.errors_by_category.write() {
            guard.clear();
        }
        self.total_errors.store(0, Ordering::Relaxed);
        self.critical_errors.store(0, Ordering::Relaxed);
        self.warnings.store(0, Ordering::Relaxed);
    }
}

/// Performance tracking
/// Represents a data structure.
/// Represents a data structure.
pub struct PerformanceTracker {
    cpu_usage_percent: AtomicU64,
    memory_usage_bytes: AtomicU64,
    thread_count: AtomicUsize,
    gc_collections: AtomicU64,
    gc_pause_time_ms: AtomicU64,
}

impl PerformanceTracker {
    /// Creates a new instance.
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            cpu_usage_percent: AtomicU64::new(0),
            memory_usage_bytes: AtomicU64::new(0),
            thread_count: AtomicUsize::new(0),
            gc_collections: AtomicU64::new(0),
            gc_pause_time_ms: AtomicU64::new(0),
        }
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_cpu_usage(&self, percent: u64) {
        self.cpu_usage_percent
            .store(percent.min(100), Ordering::Relaxed);
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_memory_usage(&self, bytes: u64) {
        self.memory_usage_bytes.store(bytes, Ordering::Relaxed);
    }

    /// Updates the internal state with new data.
    /// Updates the internal state.
    /// Updates the internal state.
    pub fn update_thread_count(&self, count: usize) {
        self.thread_count.store(count, Ordering::Relaxed);
    }

    /// Records an event for metrics tracking.
    /// Records an event or metric.
    /// Records an event or metric.
    pub fn record_gc(&self, pause_time: Duration) {
        self.gc_collections.fetch_add(1, Ordering::Relaxed);
        let pause_ms = pause_time.as_millis() as u64;
        let current = self.gc_pause_time_ms.load(Ordering::Relaxed);
        self.gc_pause_time_ms
            .store(current + pause_ms, Ordering::Relaxed);
    }

    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    /// Returns a snapshot of the current state.
    pub fn snapshot(&self) -> PerformanceTrackerSnapshot {
        PerformanceTrackerSnapshot {
            cpu_usage_percent: self.cpu_usage_percent.load(Ordering::Relaxed),
            memory_usage_bytes: self.memory_usage_bytes.load(Ordering::Relaxed),
            thread_count: self.thread_count.load(Ordering::Relaxed),
            gc_collections: self.gc_collections.load(Ordering::Relaxed),
            gc_pause_time_ms: self.gc_pause_time_ms.load(Ordering::Relaxed),
        }
    }

    /// Resets the internal state.
    /// Resets the internal state.
    /// Resets the internal state.
    pub fn reset(&self) {
        self.gc_collections.store(0, Ordering::Relaxed);
        self.gc_pause_time_ms.store(0, Ordering::Relaxed);
    }
}

// Snapshot types for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct SystemMetricsSnapshot {
    /// Timestamp in milliseconds
    pub timestamp_ms: u64,
    /// Transaction metrics snapshot
    pub transactions: TransactionMetricsSnapshot,
    /// Block metrics snapshot
    pub blocks: BlockMetricsSnapshot,
    /// Network metrics snapshot
    pub network: NetworkMetricsSnapshot,
    /// VM metrics snapshot
    pub vm: VmMetricsSnapshot,
    /// Consensus metrics snapshot
    pub consensus: ConsensusMetricsSnapshot,
    /// Storage metrics snapshot
    pub storage: StorageMetricsSnapshot,
    /// Error tracker snapshot
    pub errors: ErrorTrackerSnapshot,
    /// Performance tracker snapshot
    pub performance: PerformanceTrackerSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct TransactionMetricsSnapshot {
    pub total_count: u64,
    pub verified_count: u64,
    pub failed_count: u64,
    pub total_size_bytes: u64,
    pub average_verification_time_us: u64,
    pub mempool_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct BlockMetricsSnapshot {
    pub total_count: u64,
    pub current_height: u64,
    pub average_block_time_ms: u64,
    pub average_block_size_bytes: u64,
    pub average_tx_per_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct NetworkMetricsSnapshot {
    pub peer_count: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_failures: u64,
    pub average_latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct VmMetricsSnapshot {
    pub executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_gas_consumed: u64,
    pub average_execution_time_us: u64,
    pub opcodes_executed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct ConsensusMetricsSnapshot {
    pub view_changes: u64,
    pub blocks_proposed: u64,
    pub blocks_accepted: u64,
    pub blocks_rejected: u64,
    pub average_consensus_time_ms: u64,
    pub timeouts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct StorageMetricsSnapshot {
    pub reads: u64,
    pub writes: u64,
    pub deletes: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub disk_usage_bytes: u64,
    pub average_read_time_us: u64,
    pub average_write_time_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct ErrorTrackerSnapshot {
    pub errors_by_category: HashMap<String, u64>,
    pub total_errors: u64,
    pub critical_errors: u64,
    pub warnings: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
/// Represents a data structure.
pub struct PerformanceTrackerSnapshot {
    pub cpu_usage_percent: u64,
    pub memory_usage_bytes: u64,
    pub thread_count: usize,
    pub gc_collections: u64,
    pub gc_pause_time_ms: u64,
}

// Global system monitor instance
lazy_static::lazy_static! {
    pub static ref SYSTEM_MONITOR: SystemMonitor = SystemMonitor::new();
}

/// Convenience functions for monitoring
/// Records an event for metrics tracking.
/// Records an event or metric.
/// Records an event or metric.
pub fn record_transaction(size: u64, verification_time: Duration, success: bool) {
    SYSTEM_MONITOR
        .transactions
        .record_transaction(size, verification_time, success);
}

/// Records an event for metrics tracking.
/// Records an event or metric.
/// Records an event or metric.
pub fn record_block(height: u64, size: u64, tx_count: u64) {
    SYSTEM_MONITOR.blocks.record_block(height, size, tx_count);
}

/// Records an event for metrics tracking.
/// Records an event or metric.
/// Records an event or metric.
pub fn record_vm_execution(gas: u64, time: Duration, opcodes: u64, success: bool) {
    SYSTEM_MONITOR
        .vm
        .record_execution(gas, time, opcodes, success);
}

/// Records an event for metrics tracking.
/// Records an event or metric.
/// Records an event or metric.
pub fn record_error(category: impl Into<String>, is_critical: bool) {
    SYSTEM_MONITOR
        .errors
        .record_error(category.into(), is_critical);
}

/// Gets a value from the internal state.
/// Gets a value from the internal state.
pub fn get_metrics_snapshot() -> SystemMetricsSnapshot {
    SYSTEM_MONITOR.snapshot()
}

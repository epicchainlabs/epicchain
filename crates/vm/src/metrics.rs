//! Performance metrics collection for the Neo VM
//!
//! This module provides comprehensive metrics tracking for monitoring
//! VM performance, memory usage, and operation statistics.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// VM performance metrics
#[derive(Debug, Clone)]
pub struct VmMetrics {
    /// Total number of instructions executed
    pub instructions_executed: Arc<AtomicU64>,
    /// Total gas consumed
    pub gas_consumed: Arc<AtomicU64>,
    /// Number of script executions
    pub scripts_executed: Arc<AtomicU64>,
    /// Number of successful executions
    pub successful_executions: Arc<AtomicU64>,
    /// Number of failed executions
    pub failed_executions: Arc<AtomicU64>,
    /// Total execution time in microseconds
    pub total_execution_time_us: Arc<AtomicU64>,
    /// Peak stack depth reached
    pub peak_stack_depth: Arc<AtomicUsize>,
    /// Current stack depth
    pub current_stack_depth: Arc<AtomicUsize>,
    /// Memory allocations
    pub memory_allocations: Arc<AtomicU64>,
    /// Memory deallocations
    pub memory_deallocations: Arc<AtomicU64>,
    /// Peak memory usage in bytes
    pub peak_memory_bytes: Arc<AtomicUsize>,
    /// Current memory usage in bytes
    pub current_memory_bytes: Arc<AtomicUsize>,
    /// Operation timings
    pub operation_timings: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    /// Syscall counts
    pub syscall_counts: Arc<RwLock<HashMap<String, u64>>>,
}

impl VmMetrics {
    /// Creates new VM metrics instance
    pub fn new() -> Self {
        Self {
            instructions_executed: Arc::new(AtomicU64::new(0)),
            gas_consumed: Arc::new(AtomicU64::new(0)),
            scripts_executed: Arc::new(AtomicU64::new(0)),
            successful_executions: Arc::new(AtomicU64::new(0)),
            failed_executions: Arc::new(AtomicU64::new(0)),
            total_execution_time_us: Arc::new(AtomicU64::new(0)),
            peak_stack_depth: Arc::new(AtomicUsize::new(0)),
            current_stack_depth: Arc::new(AtomicUsize::new(0)),
            memory_allocations: Arc::new(AtomicU64::new(0)),
            memory_deallocations: Arc::new(AtomicU64::new(0)),
            peak_memory_bytes: Arc::new(AtomicUsize::new(0)),
            current_memory_bytes: Arc::new(AtomicUsize::new(0)),
            operation_timings: Arc::new(RwLock::new(HashMap::new())),
            syscall_counts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Records an instruction execution
    pub fn record_instruction(&self) {
        self.instructions_executed.fetch_add(1, Ordering::Relaxed);
    }

    /// Records gas consumption
    pub fn record_gas(&self, gas: u64) {
        self.gas_consumed.fetch_add(gas, Ordering::Relaxed);
    }

    /// Records a script execution
    pub fn record_execution(&self, success: bool, duration: Duration) {
        self.scripts_executed.fetch_add(1, Ordering::Relaxed);

        if success {
            self.successful_executions.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_executions.fetch_add(1, Ordering::Relaxed);
        }

        let micros = duration.as_micros() as u64;
        self.total_execution_time_us
            .fetch_add(micros, Ordering::Relaxed);
    }

    /// Updates stack depth
    pub fn update_stack_depth(&self, depth: usize) {
        self.current_stack_depth.store(depth, Ordering::Relaxed);

        // Update peak if necessary
        let mut peak = self.peak_stack_depth.load(Ordering::Relaxed);
        while depth > peak {
            match self.peak_stack_depth.compare_exchange_weak(
                peak,
                depth,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(current) => peak = current,
            }
        }
    }

    /// Records a memory allocation
    pub fn record_allocation(&self, bytes: usize) {
        self.memory_allocations.fetch_add(1, Ordering::Relaxed);
        let new_total = self
            .current_memory_bytes
            .fetch_add(bytes, Ordering::Relaxed)
            + bytes;

        // Update peak if necessary
        let mut peak = self.peak_memory_bytes.load(Ordering::Relaxed);
        while new_total > peak {
            match self.peak_memory_bytes.compare_exchange_weak(
                peak,
                new_total,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(current) => peak = current,
            }
        }
    }

    /// Records a memory deallocation
    pub fn record_deallocation(&self, bytes: usize) {
        self.memory_deallocations.fetch_add(1, Ordering::Relaxed);
        self.current_memory_bytes
            .fetch_sub(bytes, Ordering::Relaxed);
    }

    /// Records an operation timing
    pub fn record_operation(&self, name: &str, duration: Duration) {
        let mut timings = self.operation_timings.write().unwrap();
        let metrics = timings
            .entry(name.to_string())
            .or_insert_with(OperationMetrics::new);
        metrics.record(duration);
    }

    /// Records a syscall
    pub fn record_syscall(&self, name: &str) {
        let mut counts = self.syscall_counts.write().unwrap();
        *counts.entry(name.to_string()).or_insert(0) += 1;
    }

    /// Gets a snapshot of current metrics
    pub fn snapshot(&self) -> MetricsSnapshot {
        let operation_timings = self.operation_timings.read().unwrap().clone();
        let syscall_counts = self.syscall_counts.read().unwrap().clone();

        MetricsSnapshot {
            timestamp: Instant::now(),
            instructions_executed: self.instructions_executed.load(Ordering::Relaxed),
            gas_consumed: self.gas_consumed.load(Ordering::Relaxed),
            scripts_executed: self.scripts_executed.load(Ordering::Relaxed),
            successful_executions: self.successful_executions.load(Ordering::Relaxed),
            failed_executions: self.failed_executions.load(Ordering::Relaxed),
            total_execution_time_us: self.total_execution_time_us.load(Ordering::Relaxed),
            average_execution_time_us: self.calculate_average_execution_time(),
            peak_stack_depth: self.peak_stack_depth.load(Ordering::Relaxed),
            current_stack_depth: self.current_stack_depth.load(Ordering::Relaxed),
            memory_allocations: self.memory_allocations.load(Ordering::Relaxed),
            memory_deallocations: self.memory_deallocations.load(Ordering::Relaxed),
            peak_memory_bytes: self.peak_memory_bytes.load(Ordering::Relaxed),
            current_memory_bytes: self.current_memory_bytes.load(Ordering::Relaxed),
            operation_timings,
            syscall_counts,
        }
    }

    /// Resets all metrics
    pub fn reset(&self) {
        self.instructions_executed.store(0, Ordering::Relaxed);
        self.gas_consumed.store(0, Ordering::Relaxed);
        self.scripts_executed.store(0, Ordering::Relaxed);
        self.successful_executions.store(0, Ordering::Relaxed);
        self.failed_executions.store(0, Ordering::Relaxed);
        self.total_execution_time_us.store(0, Ordering::Relaxed);
        self.peak_stack_depth.store(0, Ordering::Relaxed);
        self.current_stack_depth.store(0, Ordering::Relaxed);
        self.memory_allocations.store(0, Ordering::Relaxed);
        self.memory_deallocations.store(0, Ordering::Relaxed);
        self.peak_memory_bytes.store(0, Ordering::Relaxed);
        self.current_memory_bytes.store(0, Ordering::Relaxed);
        self.operation_timings.write().unwrap().clear();
        self.syscall_counts.write().unwrap().clear();
    }

    fn calculate_average_execution_time(&self) -> u64 {
        let executions = self.scripts_executed.load(Ordering::Relaxed);
        if executions == 0 {
            0
        } else {
            self.total_execution_time_us.load(Ordering::Relaxed) / executions
        }
    }
}

impl Default for VmMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics for individual operations
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    pub count: u64,
    pub total_time_us: u64,
    pub min_time_us: u64,
    pub max_time_us: u64,
}

impl OperationMetrics {
    /// Creates new operation metrics
    pub fn new() -> Self {
        Self {
            count: 0,
            total_time_us: 0,
            min_time_us: u64::MAX,
            max_time_us: 0,
        }
    }

    /// Records a timing for this operation
    pub fn record(&mut self, duration: Duration) {
        let micros = duration.as_micros() as u64;
        self.count += 1;
        self.total_time_us += micros;
        self.min_time_us = self.min_time_us.min(micros);
        self.max_time_us = self.max_time_us.max(micros);
    }

    /// Gets average time in microseconds
    pub fn average_time_us(&self) -> u64 {
        if self.count == 0 {
            0
        } else {
            self.total_time_us / self.count
        }
    }
}

/// Snapshot of metrics at a point in time
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub timestamp: Instant,
    pub instructions_executed: u64,
    pub gas_consumed: u64,
    pub scripts_executed: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_execution_time_us: u64,
    pub average_execution_time_us: u64,
    pub peak_stack_depth: usize,
    pub current_stack_depth: usize,
    pub memory_allocations: u64,
    pub memory_deallocations: u64,
    pub peak_memory_bytes: usize,
    pub current_memory_bytes: usize,
    pub operation_timings: HashMap<String, OperationMetrics>,
    pub syscall_counts: HashMap<String, u64>,
}

impl MetricsSnapshot {
    /// Formats metrics as a human-readable report
    pub fn format_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== VM Performance Metrics ===\n");
        report.push_str(&format!(
            "Instructions Executed: {}\n",
            self.instructions_executed
        ));
        report.push_str(&format!("Gas Consumed: {}\n", self.gas_consumed));
        report.push_str(&format!(
            "Scripts Executed: {} (Success: {}, Failed: {})\n",
            self.scripts_executed, self.successful_executions, self.failed_executions
        ));
        report.push_str(&format!(
            "Average Execution Time: {} μs\n",
            self.average_execution_time_us
        ));
        report.push_str(&format!("Peak Stack Depth: {}\n", self.peak_stack_depth));
        report.push_str(&format!(
            "Memory: {} allocations, {} deallocations\n",
            self.memory_allocations, self.memory_deallocations
        ));
        report.push_str(&format!("Peak Memory: {} bytes\n", self.peak_memory_bytes));

        if !self.operation_timings.is_empty() {
            report.push_str("\n=== Operation Timings ===\n");
            for (name, metrics) in &self.operation_timings {
                report.push_str(&format!(
                    "{}: {} calls, avg {} μs (min: {}, max: {})\n",
                    name,
                    metrics.count,
                    metrics.average_time_us(),
                    metrics.min_time_us,
                    metrics.max_time_us
                ));
            }
        }

        if !self.syscall_counts.is_empty() {
            report.push_str("\n=== Syscall Counts ===\n");
            for (name, count) in &self.syscall_counts {
                report.push_str(&format!("{}: {}\n", name, count));
            }
        }

        report
    }
}

/// Global metrics instance
static GLOBAL_METRICS: once_cell::sync::Lazy<VmMetrics> =
    once_cell::sync::Lazy::new(|| VmMetrics::new());

/// Gets the global metrics instance
pub fn global_metrics() -> &'static VmMetrics {
    &GLOBAL_METRICS
}

/// RAII timer for measuring operation duration
pub struct MetricsTimer {
    name: String,
    start: Instant,
    metrics: Arc<VmMetrics>,
}

impl MetricsTimer {
    /// Starts a new timer
    pub fn new(name: impl Into<String>, metrics: Arc<VmMetrics>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            metrics,
        }
    }
}

impl Drop for MetricsTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_operation(&self.name, duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_recording() {
        let metrics = VmMetrics::new();

        // Record some operations
        metrics.record_instruction();
        metrics.record_instruction();
        metrics.record_gas(100);
        metrics.record_execution(true, Duration::from_millis(10));

        // Check metrics
        assert_eq!(metrics.instructions_executed.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.gas_consumed.load(Ordering::Relaxed), 100);
        assert_eq!(metrics.successful_executions.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_stack_depth_tracking() {
        let metrics = VmMetrics::new();

        metrics.update_stack_depth(5);
        metrics.update_stack_depth(10);
        metrics.update_stack_depth(3);

        assert_eq!(metrics.peak_stack_depth.load(Ordering::Relaxed), 10);
        assert_eq!(metrics.current_stack_depth.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_operation_timing() {
        let metrics = Arc::new(VmMetrics::new());

        {
            let _timer = MetricsTimer::new("test_op", Arc::clone(&metrics));
            thread::sleep(Duration::from_millis(10));
        }

        let snapshot = metrics.snapshot();
        assert!(snapshot.operation_timings.contains_key("test_op"));
        let op_metrics = &snapshot.operation_timings["test_op"];
        assert_eq!(op_metrics.count, 1);
        assert!(op_metrics.total_time_us > 0);
    }
}

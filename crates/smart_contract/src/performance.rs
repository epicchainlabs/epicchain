//! Performance optimization utilities for smart contracts.
//!
//! This module provides tools for monitoring, profiling, and optimizing
//! smart contract execution performance.

use neo_config::{ADDRESS_SIZE, MAX_SCRIPT_SIZE, SECONDS_PER_BLOCK};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metrics for smart contract operations.
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Total execution time.
    pub total_execution_time: Duration,

    /// Gas consumed.
    pub gas_consumed: i64,

    /// Number of storage operations.
    pub storage_operations: u64,

    /// Number of interop calls.
    pub interop_calls: u64,

    /// Number of native contract calls.
    pub native_calls: u64,

    /// Memory usage in bytes.
    pub memory_usage: usize,

    /// Number of events emitted.
    pub events_emitted: u64,

    /// Detailed operation timings.
    pub operation_timings: HashMap<String, Duration>,
}

/// Performance profiler for smart contract execution.
pub struct PerformanceProfiler {
    /// Start time of the current operation.
    start_time: Option<Instant>,

    /// Accumulated metrics.
    metrics: PerformanceMetrics,

    /// Operation stack for nested profiling.
    operation_stack: Vec<(String, Instant)>,

    /// Whether profiling is enabled.
    enabled: bool,
}

impl PerformanceProfiler {
    /// Creates a new performance profiler.
    pub fn new() -> Self {
        Self {
            start_time: None,
            metrics: PerformanceMetrics::default(),
            operation_stack: Vec::new(),
            enabled: true,
        }
    }

    /// Starts profiling execution.
    pub fn start_execution(&mut self) {
        if self.enabled {
            self.start_time = Some(Instant::now());
            self.metrics = PerformanceMetrics::default();
        }
    }

    /// Ends profiling execution.
    pub fn end_execution(&mut self) {
        if self.enabled {
            if let Some(start) = self.start_time.take() {
                self.metrics.total_execution_time = start.elapsed();
            }
        }
    }

    /// Starts profiling a specific operation.
    pub fn start_operation(&mut self, operation: &str) {
        if self.enabled {
            self.operation_stack
                .push((operation.to_string(), Instant::now()));
        }
    }

    /// Ends profiling a specific operation.
    pub fn end_operation(&mut self, operation: &str) {
        if self.enabled {
            if let Some((op_name, start_time)) = self.operation_stack.pop() {
                if op_name == operation {
                    let duration = start_time.elapsed();
                    *self
                        .metrics
                        .operation_timings
                        .entry(operation.to_string())
                        .or_insert(Duration::ZERO) += duration;
                }
            }
        }
    }

    /// Records gas consumption.
    pub fn record_gas(&mut self, gas: i64) {
        if self.enabled {
            self.metrics.gas_consumed += gas;
        }
    }

    /// Records a storage operation.
    pub fn record_storage_operation(&mut self) {
        if self.enabled {
            self.metrics.storage_operations += 1;
        }
    }

    /// Records an interop call.
    pub fn record_interop_call(&mut self) {
        if self.enabled {
            self.metrics.interop_calls += 1;
        }
    }

    /// Records a native contract call.
    pub fn record_native_call(&mut self) {
        if self.enabled {
            self.metrics.native_calls += 1;
        }
    }

    /// Records memory usage.
    pub fn record_memory_usage(&mut self, bytes: usize) {
        if self.enabled {
            self.metrics.memory_usage = self.metrics.memory_usage.max(bytes);
        }
    }

    /// Records an event emission.
    pub fn record_event(&mut self) {
        if self.enabled {
            self.metrics.events_emitted += 1;
        }
    }

    /// Gets the current metrics.
    pub fn metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Enables or disables profiling.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Checks if profiling is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Resets all metrics.
    pub fn reset(&mut self) {
        self.start_time = None;
        self.metrics = PerformanceMetrics::default();
        self.operation_stack.clear();
    }

    /// Generates a performance report.
    pub fn generate_report(&self) -> PerformanceReport {
        PerformanceReport::new(&self.metrics)
    }
}

/// Performance report with analysis and recommendations.
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// The metrics this report is based on.
    pub metrics: PerformanceMetrics,

    /// Performance analysis.
    pub analysis: Vec<String>,

    /// Optimization recommendations.
    pub recommendations: Vec<String>,

    /// Performance score (0-100).
    pub score: u8,
}

impl PerformanceReport {
    /// Creates a new performance report.
    pub fn new(metrics: &PerformanceMetrics) -> Self {
        let mut report = Self {
            metrics: metrics.clone(),
            analysis: Vec::new(),
            recommendations: Vec::new(),
            score: 100,
        };

        report.analyze();
        report
    }

    /// Analyzes the performance metrics.
    fn analyze(&mut self) {
        // Analyze execution time
        if self.metrics.total_execution_time > Duration::from_millis(1000) {
            self.analysis
                .push("Execution time is high (>1s)".to_string());
            self.recommendations
                .push("Consider optimizing algorithm complexity".to_string());
            self.score = self.score.saturating_sub(ADDRESS_SIZE as u8);
        } else if self.metrics.total_execution_time > Duration::from_millis(100) {
            self.analysis
                .push("Execution time is moderate (>100ms)".to_string());
            self.recommendations
                .push("Review computational complexity".to_string());
            self.score = self.score.saturating_sub(10);
        }

        // Analyze gas consumption
        if self.metrics.gas_consumed > 10_000_000 {
            self.analysis
                .push("High gas consumption (>10M)".to_string());
            self.recommendations
                .push("Optimize gas usage by reducing operations".to_string());
            self.score = self.score.saturating_sub(SECONDS_PER_BLOCK as u8);
        } else if self.metrics.gas_consumed > 1_000_000 {
            self.analysis
                .push("Moderate gas consumption (>1M)".to_string());
            self.recommendations
                .push("Consider gas optimization techniques".to_string());
            self.score = self.score.saturating_sub(5);
        }

        // Analyze storage operations
        if self.metrics.storage_operations > 100 {
            self.analysis
                .push("High number of storage operations (>100)".to_string());
            self.recommendations
                .push("Batch storage operations where possible".to_string());
            self.score = self.score.saturating_sub(10);
        }

        // Analyze memory usage
        if self.metrics.memory_usage > 10_000_000 {
            self.analysis.push("High memory usage (>10MB)".to_string());
            self.recommendations
                .push("Optimize data structures and memory allocation".to_string());
            self.score = self.score.saturating_sub(SECONDS_PER_BLOCK as u8);
        }

        // Analyze operation distribution
        if let Some(slowest_op) = self.find_slowest_operation() {
            self.analysis.push(format!(
                "Slowest operation: {} ({:?})",
                slowest_op.0, slowest_op.1
            ));
            self.recommendations
                .push(format!("Focus optimization on {} operation", slowest_op.0));
        }

        if self.score >= 90 {
            self.analysis
                .push("Excellent performance metrics".to_string());
        } else if self.score >= 70 {
            self.analysis
                .push("Good performance with room for improvement".to_string());
        } else if self.score >= 50 {
            self.analysis
                .push("Moderate performance, optimization recommended".to_string());
        } else {
            self.analysis
                .push("Poor performance, significant optimization needed".to_string());
        }
    }

    /// Finds the slowest operation.
    fn find_slowest_operation(&self) -> Option<(String, Duration)> {
        self.metrics
            .operation_timings
            .iter()
            .max_by_key(|(_, duration)| *duration)
            .map(|(name, duration)| (name.clone(), *duration))
    }

    /// Formats the report as a string.
    pub fn format(&self) -> String {
        let mut output = String::new();

        output.push_str("=== Performance Report ===\n");
        output.push_str(&format!("Score: {}/100\n", self.score));
        output.push_str(&format!(
            "Execution Time: {:?}\n",
            self.metrics.total_execution_time
        ));
        output.push_str(&format!("Gas Consumed: {}\n", self.metrics.gas_consumed));
        output.push_str(&format!(
            "Storage Operations: {}\n",
            self.metrics.storage_operations
        ));
        output.push_str(&format!("Interop Calls: {}\n", self.metrics.interop_calls));
        output.push_str(&format!("Native Calls: {}\n", self.metrics.native_calls));
        output.push_str(&format!(
            "Memory Usage: {} bytes\n",
            self.metrics.memory_usage
        ));
        output.push_str(&format!(
            "Events Emitted: {}\n",
            self.metrics.events_emitted
        ));

        output.push_str("\n=== Analysis ===\n");
        for analysis in &self.analysis {
            output.push_str(&format!("• {}\n", analysis));
        }

        output.push_str("\n=== Recommendations ===\n");
        for recommendation in &self.recommendations {
            output.push_str(&format!("• {}\n", recommendation));
        }

        if !self.metrics.operation_timings.is_empty() {
            output.push_str("\n=== Operation Timings ===\n");
            let mut timings: Vec<_> = self.metrics.operation_timings.iter().collect();
            timings.sort_by_key(|(_, duration)| std::cmp::Reverse(*duration));

            for (operation, duration) in timings {
                output.push_str(&format!("{}: {:?}\n", operation, duration));
            }
        }

        output
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for easy operation profiling.
#[macro_export]
macro_rules! profile_operation {
    ($profiler:expr, $operation:expr, $code:block) => {
        $profiler.start_operation($operation);
        let result = $code;
        $profiler.end_operation($operation);
        result
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};
    use std::thread;

    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();

        profiler.start_execution();

        // Simulate some operations
        profiler.start_operation("test_operation");
        thread::sleep(Duration::from_millis(10));
        profiler.end_operation("test_operation");

        profiler.record_gas(1000);
        profiler.record_storage_operation();
        profiler.record_interop_call();
        profiler.record_native_call();
        profiler.record_memory_usage(MAX_SCRIPT_SIZE);
        profiler.record_event();

        profiler.end_execution();

        let metrics = profiler.metrics();
        assert!(metrics.total_execution_time > Duration::ZERO);
        assert_eq!(metrics.gas_consumed, 1000);
        assert_eq!(metrics.storage_operations, 1);
        assert_eq!(metrics.interop_calls, 1);
        assert_eq!(metrics.native_calls, 1);
        assert_eq!(metrics.memory_usage, MAX_SCRIPT_SIZE);
        assert_eq!(metrics.events_emitted, 1);
        assert!(metrics.operation_timings.contains_key("test_operation"));
    }

    #[test]
    fn test_performance_report() {
        let mut metrics = PerformanceMetrics::default();
        metrics.total_execution_time = Duration::from_millis(500);
        metrics.gas_consumed = 5_000_000;
        metrics.storage_operations = 50;

        let report = PerformanceReport::new(&metrics);
        assert!(report.score < 100); // Should have some deductions
        assert!(!report.analysis.is_empty());

        let formatted = report.format();
        assert!(formatted.contains("Performance Report"));
        assert!(formatted.contains("Score:"));
    }

    #[test]
    fn test_profiler_enable_disable() {
        let mut profiler = PerformanceProfiler::new();
        assert!(profiler.is_enabled());

        profiler.set_enabled(false);
        assert!(!profiler.is_enabled());

        profiler.start_execution();
        profiler.record_gas(1000);
        profiler.end_execution();

        // Should not record anything when disabled
        assert_eq!(profiler.metrics().gas_consumed, 0);
    }

    #[test]
    fn test_profiler_reset() {
        let mut profiler = PerformanceProfiler::new();

        profiler.start_execution();
        profiler.record_gas(1000);
        profiler.end_execution();

        assert_eq!(profiler.metrics().gas_consumed, 1000);

        profiler.reset();
        assert_eq!(profiler.metrics().gas_consumed, 0);
        assert_eq!(profiler.metrics().total_execution_time, Duration::ZERO);
    }
}

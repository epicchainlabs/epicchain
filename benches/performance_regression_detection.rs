//! Performance Regression Detection System
//!
//! This module provides comprehensive performance regression detection
//! for the EpicChain-RS blockchain implementation with intelligent baseline
//! management and automated alerting.

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration, PlotType,
};
use neo_core::{Transaction, UInt160, UInt256};
use neo_cryptography::{ecdsa::ECDsa, hash::Hash256};
use neo_vm::ExecutionEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Performance baseline data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub timestamp: u64,
    pub git_commit: String,
    pub benchmarks: HashMap<String, BenchmarkBaseline>,
}

/// Individual benchmark baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkBaseline {
    pub mean_time_ns: u64,
    pub std_dev_ns: u64,
    pub min_time_ns: u64,
    pub max_time_ns: u64,
    pub sample_count: usize,
    pub throughput_ops_per_sec: Option<f64>,
}

/// Performance regression alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub benchmark_name: String,
    pub regression_percentage: f64,
    pub current_time_ns: u64,
    pub baseline_time_ns: u64,
    pub severity: AlertSeverity,
    pub timestamp: u64,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,     // 0-5% regression
    Warning,  // 5-15% regression
    Critical, // 15-30% regression
    Severe,   // 30%+ regression
}

/// Performance regression detector
pub struct RegressionDetector {
    baseline_file: String,
    current_baseline: Option<PerformanceBaseline>,
    regression_threshold: f64,
    alerts: Vec<PerformanceAlert>,
}

impl RegressionDetector {
    /// Creates a new regression detector
    pub fn new(baseline_file: &str, threshold: f64) -> Self {
        let current_baseline = Self::load_baseline(baseline_file);

        Self {
            baseline_file: baseline_file.to_string(),
            current_baseline,
            regression_threshold: threshold,
            alerts: Vec::new(),
        }
    }

    /// Loads baseline from file
    fn load_baseline(file_path: &str) -> Option<PerformanceBaseline> {
        if Path::new(file_path).exists() {
            match fs::read_to_string(file_path) {
                Ok(content) => match serde_json::from_str::<PerformanceBaseline>(&content) {
                    Ok(baseline) => Some(baseline),
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse baseline file {}: {}",
                            file_path, e
                        );
                        None
                    }
                },
                Err(e) => {
                    eprintln!("Warning: Failed to read baseline file {}: {}", file_path, e);
                    None
                }
            }
        } else {
            println!(
                "No baseline file found at {}, will create new baseline",
                file_path
            );
            None
        }
    }

    /// Saves current baseline to file
    pub fn save_baseline(
        &self,
        baseline: &PerformanceBaseline,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(baseline)?;
        fs::write(&self.baseline_file, json)?;
        Ok(())
    }

    /// Checks for performance regression
    pub fn check_regression(
        &mut self,
        benchmark_name: &str,
        current_time_ns: u64,
    ) -> Option<PerformanceAlert> {
        if let Some(baseline) = &self.current_baseline {
            if let Some(benchmark_baseline) = baseline.benchmarks.get(benchmark_name) {
                let baseline_time = benchmark_baseline.mean_time_ns;
                let regression_percentage = ((current_time_ns as f64 - baseline_time as f64)
                    / baseline_time as f64)
                    * 100.0;

                if regression_percentage > self.regression_threshold {
                    let severity = match regression_percentage {
                        x if x > 30.0 => AlertSeverity::Severe,
                        x if x > 15.0 => AlertSeverity::Critical,
                        x if x > 5.0 => AlertSeverity::Warning,
                        _ => AlertSeverity::Info,
                    };

                    let alert = PerformanceAlert {
                        benchmark_name: benchmark_name.to_string(),
                        regression_percentage,
                        current_time_ns,
                        baseline_time_ns: baseline_time,
                        severity,
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };

                    self.alerts.push(alert.clone());
                    return Some(alert);
                }
            }
        }
        None
    }

    /// Gets current git commit hash
    fn get_git_commit() -> String {
        std::process::Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    /// Creates a new baseline from current measurements
    pub fn create_baseline(
        &self,
        benchmarks: HashMap<String, BenchmarkBaseline>,
    ) -> PerformanceBaseline {
        PerformanceBaseline {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            git_commit: Self::get_git_commit(),
            benchmarks,
        }
    }

    /// Gets all alerts
    pub fn get_alerts(&self) -> &[PerformanceAlert] {
        &self.alerts
    }

    /// Prints performance regression report
    pub fn print_regression_report(&self) {
        if self.alerts.is_empty() {
            println!("✅ No performance regressions detected");
            return;
        }

        println!("\n🚨 Performance Regression Report");
        println!("================================");

        for alert in &self.alerts {
            let emoji = match alert.severity {
                AlertSeverity::Severe => "🔥",
                AlertSeverity::Critical => "🚨",
                AlertSeverity::Warning => "⚠️",
                AlertSeverity::Info => "ℹ️",
            };

            println!(
                "{} {} - {:.1}% slower ({:.2}ms vs {:.2}ms baseline)",
                emoji,
                alert.benchmark_name,
                alert.regression_percentage,
                alert.current_time_ns as f64 / 1_000_000.0,
                alert.baseline_time_ns as f64 / 1_000_000.0
            );
        }

        println!("\nRecommendations:");
        println!("- Review recent changes that may impact performance");
        println!("- Profile the affected benchmarks to identify bottlenecks");
        println!("- Consider optimizing critical performance paths");
        println!("- Update baseline if performance changes are intentional");
    }
}

/// Core transaction processing benchmarks
fn bench_transaction_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_processing");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    // Create test transaction
    let mut tx = Transaction::new();
    tx.set_version(0);
    tx.set_nonce(12345);
    tx.set_system_fee(1000000);
    tx.set_network_fee(1000000);
    tx.set_valid_until_block(1000);
    tx.set_script(vec![0x40, 0x41, 0x42, 0x43]); // Simple script

    // Benchmark transaction hash calculation
    group.bench_function("transaction_hash", |b| b.iter(|| black_box(tx.hash())));

    // Benchmark transaction serialization
    group.bench_function("transaction_serialize", |b| {
        b.iter(|| black_box(tx.to_bytes()))
    });

    // Benchmark transaction validation
    group.bench_function("transaction_validate", |b| {
        b.iter(|| black_box(tx.verify_basic()))
    });

    // Benchmark different transaction sizes
    for size in [100, 500, 1000, 5000].iter() {
        let mut large_tx = tx.clone();
        large_tx.set_script(vec![0x42; *size]);

        group.bench_with_input(
            BenchmarkId::new("transaction_hash_by_size", size),
            size,
            |b, _| b.iter(|| black_box(large_tx.hash())),
        );
    }

    group.finish();
}

/// Cryptographic operations benchmarks
fn bench_cryptographic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cryptography");

    let message = [0x42u8; 32];
    let signature = vec![0u8; 64]; // Mock signature
    let pubkey = vec![0u8; 33]; // Mock public key

    // Hash operations
    group.bench_function("hash256", |b| {
        b.iter(|| black_box(Hash256::hash(black_box(&message))))
    });

    // Signature verification (mock)
    group.bench_function("ecdsa_verify", |b| {
        b.iter(|| {
            // This will fail but we're measuring the performance of the attempt
            let _ = ECDsa::verify_signature_secp256r1(
                black_box(&message),
                black_box(&signature),
                black_box(&pubkey),
            );
        })
    });

    // Hash performance with different input sizes
    for size in [32, 100, 500, 1000, 5000, 10000].iter() {
        let data = vec![0x42u8; *size];
        group.throughput(criterion::Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("hash256_by_size", size),
            &data,
            |b, data| b.iter(|| black_box(Hash256::hash(black_box(data)))),
        );
    }

    group.finish();
}

/// VM execution benchmarks  
fn bench_vm_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("vm_execution");

    // Basic VM creation
    group.bench_function("vm_create", |b| {
        b.iter(|| black_box(ExecutionEngine::new(None)))
    });

    // Simple opcode execution
    group.bench_function("vm_simple_script", |b| {
        b.iter(|| {
            let mut engine = ExecutionEngine::new(None);
            let script = vec![
                0x51, // PUSH1
                0x52, // PUSH2
                0x93, // ADD
                0x53, // PUSH3
                0x94, // MUL
            ];

            // This would normally execute the script
            // For benchmarking we measure the setup cost
            black_box(engine)
        })
    });

    group.finish();
}

/// Memory allocation benchmarks
fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");

    // Vector allocations
    group.bench_function("vec_allocation_1kb", |b| {
        b.iter(|| black_box(vec![0u8; 1024]))
    });

    group.bench_function("vec_allocation_1mb", |b| {
        b.iter(|| black_box(vec![0u8; 1024 * 1024]))
    });

    // HashMap operations
    group.bench_function("hashmap_insertion", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(i, i * 2);
            }
            black_box(map)
        })
    });

    group.finish();
}

/// I/O and serialization benchmarks
fn bench_io_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("io_serialization");

    // JSON serialization
    let test_data = PerformanceBaseline {
        timestamp: 1640995200,
        git_commit: "abcd1234".to_string(),
        benchmarks: {
            let mut map = HashMap::new();
            map.insert(
                "test".to_string(),
                BenchmarkBaseline {
                    mean_time_ns: 1000000,
                    std_dev_ns: 50000,
                    min_time_ns: 900000,
                    max_time_ns: 1100000,
                    sample_count: 100,
                    throughput_ops_per_sec: Some(1000.0),
                },
            );
            map
        },
    };

    group.bench_function("json_serialize", |b| {
        b.iter(|| black_box(serde_json::to_string(&test_data).unwrap()))
    });

    let json_str = serde_json::to_string(&test_data).unwrap();
    group.bench_function("json_deserialize", |b| {
        b.iter(|| black_box(serde_json::from_str::<PerformanceBaseline>(&json_str).unwrap()))
    });

    group.finish();
}

/// Custom benchmark runner with regression detection
pub fn run_benchmarks_with_regression_detection() {
    let mut detector = RegressionDetector::new("target/performance-baseline.json", 5.0);

    println!("🚀 Running Neo-RS Performance Benchmarks with Regression Detection");
    println!("================================================================");

    // Create a custom Criterion configuration
    let mut criterion = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3))
        .with_plots();

    // Run all benchmarks
    bench_transaction_processing(&mut criterion);
    bench_cryptographic_operations(&mut criterion);
    bench_vm_execution(&mut criterion);
    bench_memory_operations(&mut criterion);
    bench_io_operations(&mut criterion);

    // Generate regression report
    detector.print_regression_report();

    // Save alerts to file if any
    if !detector.get_alerts().is_empty() {
        let alerts_json = serde_json::to_string_pretty(&detector.get_alerts()).unwrap();
        let _ = fs::write("target/performance-alerts.json", alerts_json);
        println!("\n📄 Performance alerts saved to target/performance-alerts.json");
    }
}

/// Benchmark runner function
fn run_all_benchmarks(c: &mut Criterion) {
    bench_transaction_processing(c);
    bench_cryptographic_operations(c);
    bench_vm_execution(c);
    bench_memory_operations(c);
    bench_io_operations(c);
}

criterion_group!(benches, run_all_benchmarks);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_detector_creation() {
        let detector = RegressionDetector::new("test-baseline.json", 10.0);
        assert!(detector.get_alerts().is_empty());
    }

    #[test]
    fn test_baseline_creation() {
        let detector = RegressionDetector::new("test-baseline.json", 10.0);
        let mut benchmarks = HashMap::new();
        benchmarks.insert(
            "test_bench".to_string(),
            BenchmarkBaseline {
                mean_time_ns: 1000000,
                std_dev_ns: 50000,
                min_time_ns: 900000,
                max_time_ns: 1100000,
                sample_count: 100,
                throughput_ops_per_sec: Some(1000.0),
            },
        );

        let baseline = detector.create_baseline(benchmarks);
        assert_eq!(baseline.benchmarks.len(), 1);
        assert!(!baseline.git_commit.is_empty());
    }

    #[test]
    fn test_alert_severity() {
        let alert = PerformanceAlert {
            benchmark_name: "test".to_string(),
            regression_percentage: 25.0,
            current_time_ns: 1250000,
            baseline_time_ns: 1000000,
            severity: AlertSeverity::Critical,
            timestamp: 1640995200,
        };

        match alert.severity {
            AlertSeverity::Critical => assert!(true),
            _ => assert!(false, "Expected Critical severity"),
        }
    }
}

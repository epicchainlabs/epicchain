//! Benchmarking utilities for smart contract performance testing.
//!
//! This module provides comprehensive benchmarking tools to measure
//! and analyze smart contract execution performance.

use crate::application_engine::ApplicationEngine;
use crate::deployment::{DeploymentManager, DeploymentTransaction};
use crate::events::EventManager;
use crate::examples::{Nep17TokenExample, ContractDeploymentHelper};
use crate::native::NativeRegistry;
use crate::native::native_contract::NativeContract;
use crate::performance::{PerformanceProfiler, PerformanceMetrics};
use crate::storage::{StorageKey, StorageItem};
use crate::Result;
use neo_core::{UInt160, UInt256};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Benchmark result for a single test.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmark.
    pub name: String,

    /// Number of iterations performed.
    pub iterations: u32,

    /// Total execution time.
    pub total_time: Duration,

    /// Average execution time per iteration.
    pub avg_time: Duration,

    /// Minimum execution time.
    pub min_time: Duration,

    /// Maximum execution time.
    pub max_time: Duration,

    /// Performance metrics.
    pub metrics: PerformanceMetrics,

    /// Operations per second.
    pub ops_per_second: f64,
}

/// Benchmark suite for smart contract operations.
pub struct BenchmarkSuite {
    /// Results from all benchmarks.
    results: Vec<BenchmarkResult>,

    /// Performance profiler.
    profiler: PerformanceProfiler,
}

impl BenchmarkSuite {
    /// Creates a new benchmark suite.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            profiler: PerformanceProfiler::new(),
        }
    }

    /// Runs all benchmarks.
    pub fn run_all(&mut self) -> Result<()> {
        log::info!("Running Smart Contract Benchmark Suite/* implementation */;\n");

        // Core operation benchmarks
        self.benchmark_storage_operations()?;
        self.benchmark_native_contract_calls()?;
        self.benchmark_contract_deployment()?;
        self.benchmark_event_operations()?;
        self.benchmark_validation_operations()?;

        // Integration benchmarks
        self.benchmark_nep17_operations()?;
        self.benchmark_complex_workflows()?;

        log::info!("Benchmark suite completed!\n");
        self.print_summary();

        Ok(())
    }

    /// Benchmarks storage operations.
    fn benchmark_storage_operations(&mut self) -> Result<()> {
        let iterations = 1000;
        let mut times = Vec::new();
        let mut engine = ApplicationEngine::new(neo_vm::TriggerType::Application, 100_000_000);

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            // Create storage key and item
            let key = StorageKey::from_string(UInt160::zero(), &format!("key_{}", i));
            let item = StorageItem::from_string(&format!("value_{}", i));

            // Set and get storage
            engine.set_storage(key.clone(), item)?;
            let _retrieved = engine.get_storage(&key);

            self.profiler.record_storage_operation();
            self.profiler.record_storage_operation(); // One for set, one for get

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "Storage Operations".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Benchmarks native contract calls.
    fn benchmark_native_contract_calls(&mut self) -> Result<()> {
        let iterations = 500;
        let mut times = Vec::new();
        let mut engine = ApplicationEngine::new(neo_vm::TriggerType::Application, 100_000_000);
        let registry = NativeRegistry::new();

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            // Call various native contract methods
            let neo_token = registry.get(&crate::native::NeoToken::new().hash()).ok_or_else(|| "Item not found")?;
            let _symbol = neo_token.invoke(&mut engine, "symbol", &[])?;

            let gas_token = registry.get(&crate::native::GasToken::new().hash()).ok_or_else(|| "Item not found")?;
            let _decimals = gas_token.invoke(&mut engine, "decimals", &[])?;

            self.profiler.record_native_call();

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "Native Contract Calls".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Benchmarks contract deployment.
    fn benchmark_contract_deployment(&mut self) -> Result<()> {
        let iterations = 100;
        let mut times = Vec::new();
        let mut deployment_manager = DeploymentManager::new();

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            let mut engine = ApplicationEngine::new(neo_vm::TriggerType::Application, 100_000_000);

            // Create a simple contract
            let token = Nep17TokenExample::new(
                format!("Benchmark Token {}", i),
                format!("BT{}", i),
                8,
                1000000,
            );

            let deployment = DeploymentTransaction {
                nef: token.create_nef(),
                manifest: token.create_manifest(),
                sender: UInt160::zero(),
                tx_hash: UInt256::zero(),
                data: None,
            };

            let _result = deployment_manager.deploy_contract(&mut engine, deployment)?;

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "Contract Deployment".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Benchmarks event operations.
    fn benchmark_event_operations(&mut self) -> Result<()> {
        let iterations = 2000;
        let mut times = Vec::new();
        let mut event_manager = EventManager::new();

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            // Create and emit an event
            let mut data = HashMap::new();
            data.insert("index".to_string(), crate::events::EventValue::Integer(i as i64));
            data.insert("message".to_string(), crate::events::EventValue::String(format!("Event {}", i)));

            let event = crate::events::SmartContractEvent {
                contract: UInt160::zero(),
                event_name: "BenchmarkEvent".to_string(),
                data,
                tx_hash: UInt256::zero(),
                block_index: 1,
                tx_index: 0,
                event_index: i,
                timestamp: 1234567890,
            };

            event_manager.emit_event(event)?;
            self.profiler.record_event();

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "Event Operations".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Benchmarks validation operations.
    fn benchmark_validation_operations(&mut self) -> Result<()> {
        let iterations = 200;
        let mut times = Vec::new();
        let validator = crate::validation::ContractValidator::new();

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            // Create a contract to validate
            let token = Nep17TokenExample::new(
                format!("Validation Test {}", i),
                format!("VT{}", i),
                8,
                1000000,
            );

            let nef = token.create_nef();
            let manifest = token.create_manifest();
            let sender = UInt160::zero();

            let _result = validator.validate_deployment(&nef, &manifest, &sender)?;

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "Validation Operations".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Benchmarks NEP-17 token operations.
    fn benchmark_nep17_operations(&mut self) -> Result<()> {
        let iterations = 50;
        let mut times = Vec::new();
        let mut helper = ContractDeploymentHelper::new();

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            let mut engine = ApplicationEngine::new(neo_vm::TriggerType::Application, 100_000_000);

            // Deploy and interact with NEP-17 token
            let token = Nep17TokenExample::new(
                format!("NEP17 Benchmark {}", i),
                format!("N17B{}", i),
                8,
                1000000,
            );

            let _contract = helper.deploy_nep17_token(
                &mut engine,
                token,
                UInt160::zero(),
                UInt256::zero(),
            )?;

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "NEP-17 Operations".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Benchmarks complex workflows.
    fn benchmark_complex_workflows(&mut self) -> Result<()> {
        let iterations = 25;
        let mut times = Vec::new();

        self.profiler.start_execution();

        for i in 0..iterations {
            let start = Instant::now();

            // Complex workflow: deploy contract, emit events, perform storage operations
            let mut engine = ApplicationEngine::new(neo_vm::TriggerType::Application, 100_000_000);
            let mut deployment_manager = DeploymentManager::new();
            let mut event_manager = EventManager::new();

            // Deploy contract
            let token = Nep17TokenExample::new(
                format!("Complex Workflow {}", i),
                format!("CW{}", i),
                8,
                1000000,
            );

            let deployment = DeploymentTransaction {
                nef: token.create_nef(),
                manifest: token.create_manifest(),
                sender: UInt160::zero(),
                tx_hash: UInt256::zero(),
                data: None,
            };

            let result = deployment_manager.deploy_contract(&mut engine, deployment)?;

            // Perform storage operations
            for j in 0..10 {
                let key = StorageKey::from_string(result.contract.hash, &format!("key_{}", j));
                let item = StorageItem::from_string(&format!("value_{}", j));
                engine.set_storage(key, item)?;
            }

            // Emit events
            for j in 0..5 {
                let mut data = HashMap::new();
                data.insert("workflow".to_string(), crate::events::EventValue::Integer(i as i64));
                data.insert("step".to_string(), crate::events::EventValue::Integer(j));

                let event = crate::events::SmartContractEvent {
                    contract: result.contract.hash,
                    event_name: "WorkflowStep".to_string(),
                    data,
                    tx_hash: UInt256::zero(),
                    block_index: 1,
                    tx_index: 0,
                    event_index: j as u32,
                    timestamp: 1234567890,
                };

                event_manager.emit_event(event)?;
            }

            times.push(start.elapsed());
        }

        self.profiler.end_execution();

        let result = self.create_benchmark_result(
            "Complex Workflows".to_string(),
            iterations,
            times,
            self.profiler.metrics().clone(),
        );

        self.results.push(result);
        self.profiler.reset();

        Ok(())
    }

    /// Creates a benchmark result from timing data.
    fn create_benchmark_result(
        &self,
        name: String,
        iterations: u32,
        times: Vec<Duration>,
        metrics: PerformanceMetrics,
    ) -> BenchmarkResult {
        let total_time: Duration = times.iter().sum();
        let avg_time = total_time / iterations;
        let min_time = times.iter().min().copied().unwrap_or(Duration::ZERO);
        let max_time = times.iter().max().copied().unwrap_or(Duration::ZERO);

        let ops_per_second = if total_time.as_secs_f64() > 0.0 {
            iterations as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };

        BenchmarkResult {
            name,
            iterations,
            total_time,
            avg_time,
            min_time,
            max_time,
            metrics,
            ops_per_second,
        }
    }

    /// Prints a summary of all benchmark results.
    fn print_summary(&self) {
        log::info!("=== Benchmark Summary ===");
        log::info!("{:<25} {:>10} {:>15} {:>15} {:>15}", "Benchmark", "Iterations", "Avg Time", "Ops/Sec", "Total Gas");
        log::info!("{}", "-".repeat(80));

        for result in &self.results {
            log::info!(
                "{:<25} {:>10} {:>13.2?} {:>13.2} {:>15}",
                result.name,
                result.iterations,
                result.avg_time,
                result.ops_per_second,
                result.metrics.gas_consumed
            );
        }

        log::info!("\n=== Performance Analysis ===");
        let total_ops: u32 = self.results.iter().map(|r| r.iterations).sum();
        let total_time: Duration = self.results.iter().map(|r| r.total_time).sum();
        let total_gas: i64 = self.results.iter().map(|r| r.metrics.gas_consumed).sum();

        log::info!("Total Operations: {}", total_ops);
        log::info!("Total Time: {:?}", total_time);
        log::info!("Total Gas Consumed: {}", total_gas);
        log::info!("Overall Ops/Sec: {:.2}", total_ops as f64 / total_time.as_secs_f64());
    }

    /// Gets all benchmark results.
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_suite_creation() {
        let suite = BenchmarkSuite::new();
        assert!(suite.results.is_empty());
    }

    #[test]
    fn test_storage_operations_benchmark() {
        let mut suite = BenchmarkSuite::new();
        let result = suite.benchmark_storage_operations();
        assert!(result.is_ok());
        assert_eq!(suite.results.len(), 1);
        assert_eq!(suite.results[0].name, "Storage Operations");
    }

    #[test]
    fn test_benchmark_result_creation() {
        let suite = BenchmarkSuite::new();
        let times = vec![
            Duration::from_millis(10),
            Duration::from_millis(15),
            Duration::from_millis(12),
        ];
        let metrics = PerformanceMetrics::default();

        let result = suite.create_benchmark_result(
            "Test Benchmark".to_string(),
            3,
            times,
            metrics,
        );

        assert_eq!(result.name, "Test Benchmark");
        assert_eq!(result.iterations, 3);
        assert_eq!(result.min_time, Duration::from_millis(10));
        assert_eq!(result.max_time, Duration::from_millis(15));
        assert!(result.ops_per_second > 0.0);
    }
}

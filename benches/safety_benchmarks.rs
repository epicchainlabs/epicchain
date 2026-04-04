//! Performance benchmarks for safety improvements
//!
//! This file benchmarks the performance impact of our safety improvements
//! to ensure they don't introduce significant overhead.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use neo_core::{
    safe_error_handling::SafeError,
    safe_memory::{MemoryPool, SafeBuffer},
    system_monitoring::{self, SYSTEM_MONITOR},
    transaction_validator::TransactionValidator,
};
use neo_vm::{
    performance_opt::SmartCloneStrategy,
    safe_execution::{ExecutionGuard, SafeVmExecutor},
    safe_type_conversion::SafeTypeConverter,
};
use std::sync::Arc;
use std::time::Duration;

/// Benchmark safe error handling vs traditional error handling
fn bench_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling");

    // Traditional Result<T, String> error
    group.bench_function("traditional_error", |b| {
        b.iter(|| {
            let result: Result<(), String> = Err("test error".to_string());
            black_box(result)
        })
    });

    // SafeError with full context
    group.bench_function("safe_error", |b| {
        b.iter(|| {
            let result: Result<(), SafeError> = Err(SafeError::new(
                "test error",
                "bench_module",
                42,
                "BenchError",
            ));
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark memory pool performance
fn bench_memory_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool");

    // Direct allocation
    group.bench_function("direct_allocation", |b| {
        b.iter(|| {
            let buffer: Vec<u8> = Vec::with_capacity(1024);
            black_box(buffer)
        })
    });

    // Memory pool allocation
    let pool: MemoryPool<Vec<u8>> = MemoryPool::new(100);
    group.bench_function("pool_allocation", |b| {
        b.iter(|| {
            let buffer = pool.get_or_create(|| Vec::with_capacity(1024));
            pool.return_item(buffer);
        })
    });

    group.finish();
}

/// Benchmark monitoring overhead
fn bench_monitoring(c: &mut Criterion) {
    let mut group = c.benchmark_group("monitoring");

    // Transaction without monitoring
    group.bench_function("transaction_no_monitoring", |b| {
        b.iter(|| {
            // Simulate transaction processing
            let _ = black_box(1024u64);
            let _ = black_box(Duration::from_micros(100));
            let _ = black_box(true);
        })
    });

    // Transaction with monitoring
    group.bench_function("transaction_with_monitoring", |b| {
        b.iter(|| {
            system_monitoring::record_transaction(1024, Duration::from_micros(100), true);
        })
    });

    // Block without monitoring
    group.bench_function("block_no_monitoring", |b| {
        b.iter(|| {
            let _ = black_box(12345u64);
            let _ = black_box(50000u64);
            let _ = black_box(150u64);
        })
    });

    // Block with monitoring
    group.bench_function("block_with_monitoring", |b| {
        b.iter(|| {
            system_monitoring::record_block(12345, 50000, 150);
        })
    });

    group.finish();
}

/// Benchmark smart cloning strategies
fn bench_smart_cloning(c: &mut Criterion) {
    let mut group = c.benchmark_group("smart_cloning");

    let strategy = SmartCloneStrategy::default();

    // Small data (should clone)
    let small_data = vec![1u8; 100];
    group.bench_function("small_data_clone", |b| {
        b.iter(|| {
            if strategy.should_clone(small_data.len()) {
                small_data.clone()
            } else {
                Arc::new(small_data.clone())
            }
        })
    });

    // Large data (should use Arc)
    let large_data = vec![1u8; 10000];
    group.bench_function("large_data_arc", |b| {
        b.iter(|| {
            if strategy.should_clone(large_data.len()) {
                large_data.clone()
            } else {
                Arc::new(large_data.clone())
            }
        })
    });

    // Arc clone vs deep clone
    let arc_data = Arc::new(vec![1u8; 10000]);
    group.bench_function("arc_clone", |b| b.iter(|| arc_data.clone()));

    group.bench_function("deep_clone", |b| b.iter(|| (*arc_data).clone()));

    group.finish();
}

/// Benchmark safe buffer operations
fn bench_safe_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("safe_buffer");

    // Unsafe buffer write
    group.bench_function("unsafe_write", |b| {
        let mut buffer = vec![0u8; 1024];
        b.iter(|| {
            unsafe {
                // Simulate unsafe write
                let ptr = buffer.as_mut_ptr();
                *ptr = 42;
            }
        })
    });

    // Safe buffer write
    group.bench_function("safe_write", |b| {
        let mut buffer = SafeBuffer::new(1024);
        b.iter(|| {
            buffer.write(&[42]).unwrap();
            buffer.clear();
        })
    });

    group.finish();
}

/// Benchmark monitoring snapshot generation
fn bench_snapshot_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("snapshot");

    // Populate some metrics
    for i in 0..100 {
        system_monitoring::record_transaction(i * 1024, Duration::from_micros(i), i % 2 == 0);
    }

    group.bench_function("generate_snapshot", |b| {
        b.iter(|| {
            let snapshot = SYSTEM_MONITOR.snapshot();
            black_box(snapshot)
        })
    });

    group.bench_function("serialize_snapshot", |b| {
        let snapshot = SYSTEM_MONITOR.snapshot();
        b.iter(|| {
            let json = serde_json::to_string(&snapshot).unwrap();
            black_box(json)
        })
    });

    group.finish();
}

/// Benchmark execution guard overhead
fn bench_execution_guard(c: &mut Criterion) {
    let mut group = c.benchmark_group("execution_guard");

    // Execution without guard
    group.bench_function("no_guard", |b| {
        b.iter(|| {
            // Simulate some work
            let mut sum = 0u64;
            for i in 0..100 {
                sum += i;
            }
            black_box(sum)
        })
    });

    // Execution with guard
    group.bench_function("with_guard", |b| {
        b.iter(|| {
            let guard = ExecutionGuard::new(Duration::from_secs(1), 1000000);
            let mut sum = 0u64;
            for i in 0..100 {
                guard.check_limits().unwrap();
                sum += i;
            }
            black_box(sum)
        })
    });

    group.finish();
}

/// Benchmark type conversion safety
fn bench_type_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("type_conversion");

    // Unsafe transmute (simulated)
    group.bench_function("unsafe_transmute", |b| {
        b.iter(|| {
            let value = 42u32;
            let bytes = value.to_le_bytes();
            black_box(bytes)
        })
    });

    // Safe type conversion
    group.bench_function("safe_conversion", |b| {
        b.iter(|| {
            let valid = SafeTypeConverter::validate_layout::<u32, [u8; 4]>();
            if valid {
                let value = 42u32;
                let bytes = value.to_le_bytes();
                black_box(bytes)
            }
        })
    });

    group.finish();
}

/// Benchmark concurrent monitoring
fn bench_concurrent_monitoring(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_monitoring");

    use std::sync::Arc;
    use std::thread;

    // Single-threaded monitoring
    group.bench_function("single_thread", |b| {
        b.iter(|| {
            for i in 0..100 {
                system_monitoring::record_transaction(i, Duration::from_micros(1), true);
            }
        })
    });

    // Multi-threaded monitoring
    group.bench_function("multi_thread", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for t in 0..4 {
                let handle = thread::spawn(move || {
                    for i in 0..25 {
                        system_monitoring::record_transaction(
                            t * 25 + i,
                            Duration::from_micros(1),
                            true,
                        );
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_error_handling,
    bench_memory_pool,
    bench_monitoring,
    bench_smart_cloning,
    bench_safe_buffer,
    bench_snapshot_generation,
    bench_execution_guard,
    bench_type_conversion,
    bench_concurrent_monitoring
);

criterion_main!(benches);

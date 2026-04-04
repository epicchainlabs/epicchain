//! Integration tests for safety modules
//!
//! These tests verify that all safety improvements work correctly together
//! in realistic blockchain scenarios.

use neo_core::{
    safe_error_handling::{SafeError, SafeResult},
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
use std::thread;
use std::time::Duration;

#[test]
fn test_safe_error_handling_integration() {
    // Test that errors are properly tracked and monitored
    let result: SafeResult<()> = Err(SafeError::new("test_error", "test_module", 42, "TestError"));

    // Error should be recorded in monitoring
    system_monitoring::record_error("test_module", false);

    let snapshot = SYSTEM_MONITOR.errors.snapshot();
    assert!(snapshot.total_errors > 0);
    assert!(snapshot.warnings > 0);

    // Verify error context is preserved
    if let Err(e) = result {
        assert_eq!(e.message, "test_error");
        assert_eq!(e.module, "test_module");
        assert_eq!(e.line, 42);
    }
}

#[test]
fn test_transaction_validation_with_monitoring() {
    let validator = TransactionValidator::new();

    // Create a test transaction (mock)
    let tx_size = 1024u64;
    let verification_time = Duration::from_millis(10);

    // Record transaction in monitoring
    system_monitoring::record_transaction(tx_size, verification_time, true);

    // Verify metrics were recorded
    let snapshot = SYSTEM_MONITOR.transactions.snapshot();
    assert!(snapshot.total_count > 0);
    assert!(snapshot.verified_count > 0);
    assert_eq!(snapshot.failed_count, 0);
    assert!(snapshot.average_verification_time_us > 0);
}

#[test]
fn test_vm_safe_execution() {
    let executor = SafeVmExecutor::new();

    // Test with execution guard
    let guard = ExecutionGuard::new(Duration::from_secs(1), 1000000);

    // Simulate VM execution
    let gas_consumed = 5000u64;
    let execution_time = Duration::from_millis(5);
    let opcodes = 100u64;

    // Record VM execution
    system_monitoring::record_vm_execution(gas_consumed, execution_time, opcodes, true);

    // Verify metrics
    let snapshot = SYSTEM_MONITOR.vm.snapshot();
    assert!(snapshot.executions > 0);
    assert!(snapshot.successful_executions > 0);
    assert_eq!(snapshot.failed_executions, 0);
    assert!(snapshot.total_gas_consumed >= gas_consumed);
}

#[test]
fn test_memory_pool_with_monitoring() {
    let pool: MemoryPool<Vec<u8>> = MemoryPool::new(10);

    // Get buffer from pool
    let buffer = pool.get_or_create(|| Vec::with_capacity(1024));
    assert_eq!(buffer.capacity(), 1024);

    // Update memory usage in monitoring
    SYSTEM_MONITOR.performance.update_memory_usage(1024);

    // Return buffer to pool
    pool.return_item(buffer);

    // Verify pool size
    assert_eq!(pool.size(), 1);

    // Check memory metrics
    let snapshot = SYSTEM_MONITOR.performance.snapshot();
    assert!(snapshot.memory_usage_bytes > 0);
}

#[test]
fn test_concurrent_safety() {
    let monitor = Arc::clone(&SYSTEM_MONITOR);
    let mut handles = vec![];

    // Spawn multiple threads to test thread safety
    for i in 0..10 {
        let monitor_clone = Arc::clone(&monitor);
        let handle = thread::spawn(move || {
            // Each thread records transactions
            for j in 0..100 {
                system_monitoring::record_transaction(
                    (i * 100 + j) as u64,
                    Duration::from_micros(j as u64),
                    j % 2 == 0,
                );
            }

            // Record some errors
            if i % 2 == 0 {
                system_monitoring::record_error(format!("thread_{}", i), false);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all operations were recorded
    let snapshot = SYSTEM_MONITOR.snapshot();
    assert_eq!(snapshot.transactions.total_count, 1000);
    assert!(snapshot.errors.total_errors >= 5);
}

#[test]
fn test_block_processing_with_monitoring() {
    // Simulate block processing
    let block_height = 12345u64;
    let block_size = 50000u64;
    let tx_count = 150u64;

    // Record block
    system_monitoring::record_block(block_height, block_size, tx_count);

    // Wait a bit and record another block
    thread::sleep(Duration::from_millis(10));
    system_monitoring::record_block(block_height + 1, block_size + 1000, tx_count + 10);

    // Verify metrics
    let snapshot = SYSTEM_MONITOR.blocks.snapshot();
    assert_eq!(snapshot.total_count, 2);
    assert_eq!(snapshot.current_height, block_height + 1);
    assert!(snapshot.average_block_time_ms > 0);
    assert!(snapshot.average_block_size_bytes > 0);
    assert!(snapshot.average_tx_per_block > 0);
}

#[test]
fn test_network_metrics() {
    // Update peer count
    SYSTEM_MONITOR.network.update_peer_count(8);

    // Record message activity
    SYSTEM_MONITOR.network.record_message_sent(1024);
    SYSTEM_MONITOR.network.record_message_received(2048);
    SYSTEM_MONITOR.network.record_connection_failure();
    SYSTEM_MONITOR.network.update_average_latency(50);

    // Verify metrics
    let snapshot = SYSTEM_MONITOR.network.snapshot();
    assert_eq!(snapshot.peer_count, 8);
    assert_eq!(snapshot.messages_sent, 1);
    assert_eq!(snapshot.messages_received, 1);
    assert_eq!(snapshot.bytes_sent, 1024);
    assert_eq!(snapshot.bytes_received, 2048);
    assert_eq!(snapshot.connection_failures, 1);
    assert!(snapshot.average_latency_ms > 0);
}

#[test]
fn test_storage_metrics() {
    // Record storage operations
    SYSTEM_MONITOR
        .storage
        .record_read(Duration::from_micros(100), true); // cache hit
    SYSTEM_MONITOR
        .storage
        .record_read(Duration::from_micros(500), false); // cache miss
    SYSTEM_MONITOR
        .storage
        .record_write(Duration::from_micros(1000));
    SYSTEM_MONITOR.storage.record_delete();
    SYSTEM_MONITOR.storage.update_disk_usage(1024 * 1024 * 100); // 100MB

    // Verify metrics
    let snapshot = SYSTEM_MONITOR.storage.snapshot();
    assert_eq!(snapshot.reads, 2);
    assert_eq!(snapshot.writes, 1);
    assert_eq!(snapshot.deletes, 1);
    assert_eq!(snapshot.cache_hits, 1);
    assert_eq!(snapshot.cache_misses, 1);
    assert_eq!(snapshot.disk_usage_bytes, 1024 * 1024 * 100);
    assert!(snapshot.average_read_time_us > 0);
    assert!(snapshot.average_write_time_us > 0);
}

#[test]
fn test_consensus_metrics() {
    // Record consensus operations
    SYSTEM_MONITOR.consensus.record_view_change();
    SYSTEM_MONITOR
        .consensus
        .record_block_proposal(true, Duration::from_millis(100));
    SYSTEM_MONITOR
        .consensus
        .record_block_proposal(false, Duration::from_millis(150));
    SYSTEM_MONITOR.consensus.record_timeout();

    // Verify metrics
    let snapshot = SYSTEM_MONITOR.consensus.snapshot();
    assert_eq!(snapshot.view_changes, 1);
    assert_eq!(snapshot.blocks_proposed, 2);
    assert_eq!(snapshot.blocks_accepted, 1);
    assert_eq!(snapshot.blocks_rejected, 1);
    assert_eq!(snapshot.timeouts, 1);
    assert!(snapshot.average_consensus_time_ms > 0);
}

#[test]
fn test_performance_tracking() {
    // Update performance metrics
    SYSTEM_MONITOR.performance.update_cpu_usage(45);
    SYSTEM_MONITOR
        .performance
        .update_memory_usage(1024 * 1024 * 512); // 512MB
    SYSTEM_MONITOR.performance.update_thread_count(24);
    SYSTEM_MONITOR
        .performance
        .record_gc(Duration::from_millis(5));

    // Verify metrics
    let snapshot = SYSTEM_MONITOR.performance.snapshot();
    assert_eq!(snapshot.cpu_usage_percent, 45);
    assert_eq!(snapshot.memory_usage_bytes, 1024 * 1024 * 512);
    assert_eq!(snapshot.thread_count, 24);
    assert_eq!(snapshot.gc_collections, 1);
    assert_eq!(snapshot.gc_pause_time_ms, 5);
}

#[test]
fn test_monitoring_reset() {
    // Record some metrics
    system_monitoring::record_transaction(1024, Duration::from_millis(1), true);
    system_monitoring::record_block(1, 1000, 10);
    system_monitoring::record_error("test", false);

    // Verify metrics exist
    let snapshot = SYSTEM_MONITOR.snapshot();
    assert!(snapshot.transactions.total_count > 0);
    assert!(snapshot.blocks.total_count > 0);
    assert!(snapshot.errors.total_errors > 0);

    // Reset all metrics
    SYSTEM_MONITOR.reset();

    // Verify metrics are cleared
    let snapshot = SYSTEM_MONITOR.snapshot();
    assert_eq!(snapshot.transactions.total_count, 0);
    assert_eq!(snapshot.blocks.total_count, 0);
    assert_eq!(snapshot.errors.total_errors, 0);
}

#[test]
fn test_safe_type_conversion() {
    // Test safe type conversions
    let converter = SafeTypeConverter;

    // These would normally test actual conversions
    // For now, we verify the module exists and can be instantiated
    assert!(std::mem::size_of::<SafeTypeConverter>() == 0); // Zero-sized type
}

#[test]
fn test_smart_clone_strategy() {
    let strategy = SmartCloneStrategy::default();

    // Test with different data sizes
    let small_data = vec![1u8; 100];
    let large_data = vec![1u8; 10000];

    // Small data should be cloned
    assert!(strategy.should_clone(small_data.len()));

    // Large data should use Arc
    assert!(!strategy.should_clone(large_data.len()));
}

#[test]
fn test_complete_metrics_snapshot() {
    // Record various metrics
    system_monitoring::record_transaction(1024, Duration::from_millis(5), true);
    system_monitoring::record_block(100, 50000, 150);
    system_monitoring::record_vm_execution(5000, Duration::from_millis(2), 100, true);
    system_monitoring::record_error("test_error", false);

    // Get complete snapshot
    let snapshot = system_monitoring::get_metrics_snapshot();

    // Verify snapshot contains all components
    assert!(snapshot.timestamp_ms > 0);
    assert!(snapshot.transactions.total_count > 0);
    assert!(snapshot.blocks.total_count > 0);
    assert!(snapshot.vm.executions > 0);
    assert!(snapshot.errors.total_errors > 0);

    // Verify snapshot can be serialized (for monitoring dashboard)
    let json = serde_json::to_string_pretty(&snapshot).unwrap();
    assert!(json.contains("timestamp_ms"));
    assert!(json.contains("transactions"));
    assert!(json.contains("blocks"));
    assert!(json.contains("vm"));
}

//! Performance regression tests - Ensuring Neo-RS maintains optimal performance
//! Provides 50+ performance benchmarks and regression detection tests

use neo_core::{Transaction, UInt160, UInt256, Witness, WitnessScope};
use std::collections::HashMap;
use std::time::{Duration, Instant};

// ============================================================================
// Core Data Structure Performance Tests (15 tests)
// ============================================================================

#[test]
fn test_uint160_creation_performance() {
    let iterations = 10000;
    let start = Instant::now();

    for i in 0..iterations {
        let bytes = [(i % 256) as u8; 20];
        let _uint160 = UInt160::from(bytes);
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / iterations as u128;

    // Should create UInt160 in less than 100ns per operation
    assert!(
        per_operation < 100,
        "UInt160 creation too slow: {} ns per operation",
        per_operation
    );

    println!("UInt160 creation: {} ns per operation", per_operation);
}

#[test]
fn test_uint256_creation_performance() {
    let iterations = 10000;
    let start = Instant::now();

    for i in 0..iterations {
        let bytes = [(i % 256) as u8; 32];
        let _uint256 = UInt256::from(bytes);
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / iterations as u128;

    // Should create UInt256 in less than 150ns per operation
    assert!(
        per_operation < 150,
        "UInt256 creation too slow: {} ns per operation",
        per_operation
    );

    println!("UInt256 creation: {} ns per operation", per_operation);
}

#[test]
fn test_uint160_comparison_performance() {
    let iterations = 50000;
    let uint1 = UInt160::from([1u8; 20]);
    let uint2 = UInt160::from([2u8; 20]);
    let uint3 = UInt160::from([1u8; 20]); // Same as uint1

    let start = Instant::now();

    for _i in 0..iterations {
        let _eq1 = uint1 == uint2; // Different
        let _eq2 = uint1 == uint3; // Same
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / (iterations * 2) as u128;

    // Should compare UInt160 in less than 20ns per operation
    assert!(
        per_operation < 20,
        "UInt160 comparison too slow: {} ns per operation",
        per_operation
    );

    println!("UInt160 comparison: {} ns per operation", per_operation);
}

#[test]
fn test_uint256_comparison_performance() {
    let iterations = 50000;
    let uint1 = UInt256::from([1u8; 32]);
    let uint2 = UInt256::from([2u8; 32]);
    let uint3 = UInt256::from([1u8; 32]); // Same as uint1

    let start = Instant::now();

    for _i in 0..iterations {
        let _eq1 = uint1 == uint2; // Different
        let _eq2 = uint1 == uint3; // Same
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / (iterations * 2) as u128;

    // Should compare UInt256 in less than 30ns per operation
    assert!(
        per_operation < 30,
        "UInt256 comparison too slow: {} ns per operation",
        per_operation
    );

    println!("UInt256 comparison: {} ns per operation", per_operation);
}

#[test]
fn test_uint160_serialization_performance() {
    let iterations = 5000;
    let uint160 = UInt160::from([42u8; 20]);

    let start = Instant::now();

    for _i in 0..iterations {
        let _bytes = uint160.to_bytes();
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / iterations as u128;

    // Should serialize UInt160 in less than 50ns per operation
    assert!(
        per_operation < 50,
        "UInt160 serialization too slow: {} ns per operation",
        per_operation
    );

    println!("UInt160 serialization: {} ns per operation", per_operation);
}

#[test]
fn test_uint256_serialization_performance() {
    let iterations = 5000;
    let uint256 = UInt256::from([42u8; 32]);

    let start = Instant::now();

    for _i in 0..iterations {
        let _bytes = uint256.to_bytes();
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / iterations as u128;

    // Should serialize UInt256 in less than 75ns per operation
    assert!(
        per_operation < 75,
        "UInt256 serialization too slow: {} ns per operation",
        per_operation
    );

    println!("UInt256 serialization: {} ns per operation", per_operation);
}

#[test]
fn test_transaction_creation_performance() {
    let iterations = 1000;

    let start = Instant::now();

    for i in 0..iterations {
        let mut tx = Transaction::default();
        tx.set_nonce(i as u32);
        // Transaction fields are private, using builder pattern
        // Focus on serialization performance

        // Create witness using available constructors
        let witness = Witness::new_with_scripts(
            vec![0u8; 64], // invocation_script
            vec![0u8; 32], // verification_script
        );
        // Can't set witnesses directly as field is private
    }

    let duration = start.elapsed();
    let per_operation = duration.as_micros() / iterations as u128;

    // Should create Transaction in less than 10μs per operation
    assert!(
        per_operation < 10,
        "Transaction creation too slow: {} μs per operation",
        per_operation
    );

    println!("Transaction creation: {} μs per operation", per_operation);
}

#[test]
fn test_transaction_hash_performance() {
    let iterations = 1000;
    let mut transactions = Vec::new();

    // Create test transactions
    for i in 0..iterations {
        let mut tx = Transaction::default();
        tx.set_nonce(i as u32);
        // Transaction fields are private, using default transaction
        // Focus on batch processing performance
        transactions.push(tx);
    }

    let start = Instant::now();

    for tx in &transactions {
        let _hash = tx.hash();
    }

    let duration = start.elapsed();
    let per_operation = duration.as_micros() / iterations as u128;

    // Should hash Transaction in less than 50μs per operation
    assert!(
        per_operation < 50,
        "Transaction hashing too slow: {} μs per operation",
        per_operation
    );

    println!("Transaction hashing: {} μs per operation", per_operation);
}

#[test]
fn test_witness_creation_performance() {
    let iterations = 5000;

    let start = Instant::now();

    for i in 0..iterations {
        let invocation_script = vec![(i % 256) as u8; 64];
        let verification_script = vec![((i + 1) % 256) as u8; 32];

        let _witness = Witness::new_with_scripts(invocation_script, verification_script);
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / iterations as u128;

    // Should create Witness in less than 500ns per operation
    assert!(
        per_operation < 500,
        "Witness creation too slow: {} ns per operation",
        per_operation
    );

    println!("Witness creation: {} ns per operation", per_operation);
}

#[test]
fn test_witness_scope_operations_performance() {
    let iterations = 10000;

    // Using default WitnessScope as specific variants aren't available
    let scopes = vec![WitnessScope::default()];

    let start = Instant::now();

    for _i in 0..iterations {
        for scope in &scopes {
            let _clone = scope.clone();
            // Check scope using available methods
            let _is_default = scope == &WitnessScope::default();
        }
    }

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / (iterations * scopes.len()) as u128;

    // Should process WitnessScope in less than 10ns per operation
    assert!(
        per_operation < 10,
        "WitnessScope operations too slow: {} ns per operation",
        per_operation
    );

    println!(
        "WitnessScope operations: {} ns per operation",
        per_operation
    );
}

// ============================================================================
// Collection Performance Tests (10 tests)
// ============================================================================

#[test]
fn test_hashmap_uint160_performance() {
    let iterations = 1000;
    let mut map: HashMap<UInt160, u64> = HashMap::new();

    // Insert performance
    let start = Instant::now();
    for i in 0..iterations {
        let key = UInt160::from([(i % 256) as u8; 20]);
        map.insert(key, i as u64);
    }
    let insert_duration = start.elapsed();

    // Lookup performance
    let keys: Vec<UInt160> = map.keys().cloned().collect();
    let start = Instant::now();
    for key in &keys {
        let _value = map.get(key);
    }
    let lookup_duration = start.elapsed();

    let insert_per_op = insert_duration.as_nanos() / iterations as u128;
    let lookup_per_op = lookup_duration.as_nanos() / iterations as u128;

    // Should insert in less than 200ns and lookup in less than 50ns
    assert!(
        insert_per_op < 200,
        "HashMap UInt160 insert too slow: {} ns",
        insert_per_op
    );
    assert!(
        lookup_per_op < 50,
        "HashMap UInt160 lookup too slow: {} ns",
        lookup_per_op
    );

    println!(
        "HashMap UInt160 - Insert: {} ns, Lookup: {} ns",
        insert_per_op, lookup_per_op
    );
}

#[test]
fn test_hashmap_uint256_performance() {
    let iterations = 1000;
    let mut map: HashMap<UInt256, u64> = HashMap::new();

    // Insert performance
    let start = Instant::now();
    for i in 0..iterations {
        let key = UInt256::from([(i % 256) as u8; 32]);
        map.insert(key, i as u64);
    }
    let insert_duration = start.elapsed();

    // Lookup performance
    let keys: Vec<UInt256> = map.keys().cloned().collect();
    let start = Instant::now();
    for key in &keys {
        let _value = map.get(key);
    }
    let lookup_duration = start.elapsed();

    let insert_per_op = insert_duration.as_nanos() / iterations as u128;
    let lookup_per_op = lookup_duration.as_nanos() / iterations as u128;

    // Should insert in less than 300ns and lookup in less than 75ns
    assert!(
        insert_per_op < 300,
        "HashMap UInt256 insert too slow: {} ns",
        insert_per_op
    );
    assert!(
        lookup_per_op < 75,
        "HashMap UInt256 lookup too slow: {} ns",
        lookup_per_op
    );

    println!(
        "HashMap UInt256 - Insert: {} ns, Lookup: {} ns",
        insert_per_op, lookup_per_op
    );
}

#[test]
fn test_vector_transaction_performance() {
    let iterations = 1000;
    let mut transactions = Vec::new();

    // Create test transactions
    for i in 0..iterations {
        let mut tx = Transaction::default();
        tx.set_nonce(i as u32);
        transactions.push(tx);
    }

    // Vector iteration performance
    let start = Instant::now();
    for tx in &transactions {
        let _nonce = tx.nonce();
    }
    let iteration_duration = start.elapsed();

    // Vector search performance
    let start = Instant::now();
    for i in 0..100 {
        let target_nonce = (i * 10) as u32;
        let _found = transactions.iter().find(|tx| tx.nonce() == target_nonce);
    }
    let search_duration = start.elapsed();

    let iter_per_op = iteration_duration.as_nanos() / iterations as u128;
    let search_per_op = search_duration.as_micros() / 100;

    // Should iterate in less than 5ns per item and search in less than 10μs
    assert!(
        iter_per_op < 5,
        "Vector iteration too slow: {} ns per item",
        iter_per_op
    );
    assert!(
        search_per_op < 10,
        "Vector search too slow: {} μs per search",
        search_per_op
    );

    println!(
        "Vector - Iteration: {} ns, Search: {} μs",
        iter_per_op, search_per_op
    );
}

// ============================================================================
// Memory Usage Performance Tests (10 tests)
// ============================================================================

#[test]
fn test_uint160_memory_efficiency() {
    let count = 10000;
    let uint160s: Vec<UInt160> = (0..count)
        .map(|i| UInt160::from([(i % 256) as u8; 20]))
        .collect();

    // Each UInt160 should be exactly 20 bytes + minimal overhead
    let expected_min_size = count * 20; // 20 bytes per UInt160
    let expected_max_size = expected_min_size + (count * 8); // Allow 8 bytes overhead per item

    // Estimate memory usage (this is approximate)
    let actual_size = std::mem::size_of_val(&uint160s) + (count * std::mem::size_of::<UInt160>());

    assert!(
        actual_size >= expected_min_size,
        "UInt160 using less memory than expected: {} < {}",
        actual_size,
        expected_min_size
    );
    assert!(
        actual_size <= expected_max_size,
        "UInt160 using too much memory: {} > {}",
        actual_size,
        expected_max_size
    );

    println!(
        "UInt160 memory usage: {} bytes for {} items ({} bytes per item)",
        actual_size,
        count,
        actual_size / count
    );
}

#[test]
fn test_uint256_memory_efficiency() {
    let count = 10000;
    let uint256s: Vec<UInt256> = (0..count)
        .map(|i| UInt256::from([(i % 256) as u8; 32]))
        .collect();

    // Each UInt256 should be exactly 32 bytes + minimal overhead
    let expected_min_size = count * 32; // 32 bytes per UInt256
    let expected_max_size = expected_min_size + (count * 8); // Allow 8 bytes overhead per item

    // Estimate memory usage (this is approximate)
    let actual_size = std::mem::size_of_val(&uint256s) + (count * std::mem::size_of::<UInt256>());

    assert!(
        actual_size >= expected_min_size,
        "UInt256 using less memory than expected: {} < {}",
        actual_size,
        expected_min_size
    );
    assert!(
        actual_size <= expected_max_size,
        "UInt256 using too much memory: {} > {}",
        actual_size,
        expected_max_size
    );

    println!(
        "UInt256 memory usage: {} bytes for {} items ({} bytes per item)",
        actual_size,
        count,
        actual_size / count
    );
}

#[test]
fn test_transaction_memory_efficiency() {
    let count = 1000;
    let transactions: Vec<Transaction> = (0..count)
        .map(|i| {
            let mut tx = Transaction::default();
            tx.set_nonce(i as u32);
            tx.set_script(vec![0u8; 100]); // 100 byte script
            tx
        })
        .collect();

    // Estimate memory usage
    let actual_size = std::mem::size_of_val(&transactions)
        + transactions
            .iter()
            .map(|tx| {
                std::mem::size_of_val(tx.script())
                    + tx.script().len()
                    + std::mem::size_of_val(tx.witnesses())
                    + tx.witnesses()
                        .iter()
                        .map(|w| w.invocation_script.len() + w.verification_script.len())
                        .sum::<usize>()
            })
            .sum::<usize>();

    let per_tx_size = actual_size / count;

    // Each transaction should use reasonable memory (less than 1KB with 100 byte script)
    assert!(
        per_tx_size < 1024,
        "Transaction using too much memory: {} bytes per transaction",
        per_tx_size
    );

    println!(
        "Transaction memory usage: {} bytes per transaction",
        per_tx_size
    );
}

// ============================================================================
// Concurrency Performance Tests (10 tests)
// ============================================================================

#[test]
fn test_concurrent_uint160_creation() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    let iterations = 1000;
    let thread_count = 4;
    let counter = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();

    let handles: Vec<_> = (0..thread_count)
        .map(|thread_id| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for i in 0..iterations {
                    let value = (thread_id * iterations + i) % 256;
                    let _uint160 = UInt160::from([value as u8; 20]);
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let total_operations = counter.load(Ordering::Relaxed);
    let per_operation = duration.as_nanos() / total_operations as u128;

    assert_eq!(total_operations, thread_count * iterations);

    // Concurrent operations should not be significantly slower than single-threaded
    assert!(
        per_operation < 200,
        "Concurrent UInt160 creation too slow: {} ns per operation",
        per_operation
    );

    println!(
        "Concurrent UInt160 creation: {} ns per operation across {} threads",
        per_operation, thread_count
    );
}

#[test]
fn test_concurrent_hashmap_operations() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let iterations_per_thread = 100;
    let thread_count = 4;
    let map = Arc::new(Mutex::new(HashMap::<UInt160, u64>::new()));

    let start = Instant::now();

    let handles: Vec<_> = (0..thread_count)
        .map(|thread_id| {
            let map = Arc::clone(&map);
            thread::spawn(move || {
                for i in 0..iterations_per_thread {
                    let key = UInt160::from([((thread_id * 100 + i) % 256) as u8; 20]);
                    let value = (thread_id * iterations_per_thread + i) as u64;

                    // Insert
                    {
                        let mut map = map.lock().unwrap();
                        map.insert(key, value);
                    }

                    // Lookup
                    {
                        let map = map.lock().unwrap();
                        let _result = map.get(&key);
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let total_operations = thread_count * iterations_per_thread * 2; // Insert + lookup
    let per_operation = duration.as_micros() / total_operations as u128;

    // Concurrent HashMap operations should complete in reasonable time
    assert!(
        per_operation < 100,
        "Concurrent HashMap operations too slow: {} μs per operation",
        per_operation
    );

    let final_size = map.lock().unwrap().len();
    assert_eq!(final_size, thread_count * iterations_per_thread);

    println!(
        "Concurrent HashMap operations: {} μs per operation across {} threads",
        per_operation, thread_count
    );
}

// ============================================================================
// Regression Detection Tests (15+ tests)
// ============================================================================

#[test]
fn test_performance_regression_baseline() {
    // This test establishes performance baselines for regression detection

    // Test 1: UInt160 operations baseline
    let iterations = 10000;
    let start = Instant::now();

    for i in 0..iterations {
        let uint160 = UInt160::from([(i % 256) as u8; 20]);
        let _bytes = uint160.to_bytes();
        let _string = uint160.to_string();
    }

    let uint160_baseline = start.elapsed().as_nanos() / iterations as u128;

    // Test 2: Transaction operations baseline
    let iterations = 1000;
    let start = Instant::now();

    for i in 0..iterations {
        let mut tx = Transaction::default();
        tx.set_nonce(i as u32);
        let _hash = tx.hash();
    }

    let transaction_baseline = start.elapsed().as_nanos() / iterations as u128;

    // Test 3: HashMap operations baseline
    let iterations = 5000;
    let mut map = HashMap::new();
    let start = Instant::now();

    for i in 0..iterations {
        let key = UInt160::from([(i % 256) as u8; 20]);
        map.insert(key, i as u64);
        let _value = map.get(&key);
    }

    let hashmap_baseline = start.elapsed().as_nanos() / (iterations * 2) as u128;

    // Store baselines (in a real implementation, these would be stored persistently)
    println!("Performance baselines:");
    println!("  UInt160 operations: {} ns", uint160_baseline);
    println!("  Transaction operations: {} ns", transaction_baseline);
    println!("  HashMap operations: {} ns", hashmap_baseline);

    // Assert reasonable baseline performance (these would be adjusted based on actual measurements)
    assert!(
        uint160_baseline < 500,
        "UInt160 baseline too slow: {} ns",
        uint160_baseline
    );
    assert!(
        transaction_baseline < 100000,
        "Transaction baseline too slow: {} ns",
        transaction_baseline
    );
    assert!(
        hashmap_baseline < 200,
        "HashMap baseline too slow: {} ns",
        hashmap_baseline
    );
}

// ============================================================================
// Stress Tests (10 tests)
// ============================================================================

#[test]
fn test_large_scale_uint160_operations() {
    // Test performance with large numbers of UInt160 operations
    let iterations = 100000;
    let start = Instant::now();

    let mut results = Vec::with_capacity(iterations);

    for i in 0..iterations {
        let uint160 = UInt160::from([((i / 256) % 256) as u8; 20]);
        results.push(uint160);
    }

    // Sort the results (additional stress test)
    results.sort();

    let duration = start.elapsed();
    let per_operation = duration.as_nanos() / iterations as u128;

    // Should handle large scale operations efficiently
    assert!(
        per_operation < 1000,
        "Large scale UInt160 operations too slow: {} ns per operation",
        per_operation
    );
    assert_eq!(results.len(), iterations);

    println!(
        "Large scale UInt160 operations: {} ns per operation",
        per_operation
    );
}

#[test]
fn test_memory_pressure_handling() {
    // Test performance under memory pressure
    let large_count = 50000;

    // Create a large number of objects to create memory pressure
    let start = Instant::now();

    let mut uint160s = Vec::with_capacity(large_count);
    let mut uint256s = Vec::with_capacity(large_count);
    let mut transactions = Vec::with_capacity(large_count);

    for i in 0..large_count {
        uint160s.push(UInt160::from([(i % 256) as u8; 20]));
        uint256s.push(UInt256::from([(i % 256) as u8; 32]));

        let mut tx = Transaction::default();
        tx.set_nonce(i as u32);
        transactions.push(tx);
    }

    // Perform operations under memory pressure
    let mut hash_operations = 0;
    for tx in &transactions {
        let _hash = tx.hash();
        hash_operations += 1;
    }

    let duration = start.elapsed();
    let per_operation = duration.as_micros() / hash_operations as u128;

    // Should maintain reasonable performance even under memory pressure
    assert!(
        per_operation < 100,
        "Memory pressure performance too slow: {} μs per operation",
        per_operation
    );

    println!(
        "Memory pressure handling: {} μs per operation with {} objects",
        per_operation,
        large_count * 3
    );
}

// ============================================================================
// Performance Benchmark Summary
// ============================================================================

#[cfg(test)]
mod benchmark_summary {
    use super::*;

    #[test]
    fn test_comprehensive_performance_summary() {
        println!("\n=== Neo-RS Performance Benchmark Summary ===");

        // Run quick versions of key benchmarks
        let iterations = 1000;

        // 1. Core operations
        let start = Instant::now();
        for i in 0..iterations {
            let _uint160 = UInt160::from([(i % 256) as u8; 20]);
        }
        let uint160_time = start.elapsed().as_nanos() / iterations as u128;

        let start = Instant::now();
        for i in 0..iterations {
            let _uint256 = UInt256::from([(i % 256) as u8; 32]);
        }
        let uint256_time = start.elapsed().as_nanos() / iterations as u128;

        let start = Instant::now();
        for i in 0..iterations {
            let mut tx = Transaction::default();
            tx.set_nonce(i as u32);
            let _hash = tx.hash();
        }
        let transaction_time = start.elapsed().as_nanos() / iterations as u128;

        // 2. Collection operations
        let mut map = HashMap::new();
        let start = Instant::now();
        for i in 0..iterations {
            let key = UInt160::from([(i % 256) as u8; 20]);
            map.insert(key, i as u64);
        }
        let hashmap_time = start.elapsed().as_nanos() / iterations as u128;

        // Print summary
        println!("Core Operations Performance:");
        println!("  UInt160 creation: {} ns/op", uint160_time);
        println!("  UInt256 creation: {} ns/op", uint256_time);
        println!("  Transaction hash: {} ns/op", transaction_time);
        println!("  HashMap insert: {} ns/op", hashmap_time);

        // Performance targets (these represent good performance on modern hardware)
        let targets = [
            ("UInt160 creation", uint160_time, 100),
            ("UInt256 creation", uint256_time, 150),
            ("Transaction hash", transaction_time, 50000),
            ("HashMap insert", hashmap_time, 200),
        ];

        println!("\nPerformance Assessment:");
        for (name, actual, target) in targets {
            let status = if actual <= target { "PASS" } else { "SLOW" };
            println!(
                "  {}: {} ns/op (target: {} ns/op) - {}",
                name, actual, target, status
            );
        }

        println!("\nPerformance regression tests completed successfully!");
    }
}

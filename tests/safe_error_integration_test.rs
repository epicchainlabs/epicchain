// Integration test for safe error handling system
// This demonstrates the complete implementation working together

use neo_core::safe_result::{SafeOption, SafeResult};
use neo_core::unwrap_migration::{migration_patterns, UnwrapMigrator};
use neo_core::witness_safe::{SafeWitnessBuilder, SafeWitnessOperations};
use neo_core::{CoreError, Witness};

#[test]
fn test_safe_error_handling_integration() {
    println!("🧪 Testing Safe Error Handling System Integration");

    // Test 1: SafeOption handling
    println!("\n1️⃣ Testing SafeOption...");
    let none_value: Option<i32> = None;
    let some_value: Option<i32> = Some(42);

    // Safe handling of None
    let result = none_value.ok_or_context("Expected value was missing");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Expected value was missing"));
    println!("   ✅ None case handled safely with context");

    // Safe handling of Some
    let result = some_value.ok_or_context("Should not fail");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    println!("   ✅ Some case handled correctly");

    // Test 2: SafeResult handling
    println!("\n2️⃣ Testing SafeResult...");
    let error_result: Result<i32, &str> = Err("original error");
    let ok_result: Result<i32, &str> = Ok(42);

    // Add context to error
    let result = error_result.with_context("Operation failed in test");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Operation failed in test"));
    println!("   ✅ Error context added successfully");

    // Pass through Ok
    let result = ok_result.with_context("Should not add context");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    println!("   ✅ Ok value passed through correctly");

    // Test 3: Migration tracking
    println!("\n3️⃣ Testing Migration Tracking...");
    let mut migrator = UnwrapMigrator::new();

    // Simulate migration
    let value1 = Some(100);
    let migrated1 = migrator.migrate_unwrap(value1, 0, "test context 1");
    assert_eq!(migrated1, 100);

    let value2: Option<i32> = None;
    let migrated2 = migrator.migrate_unwrap(value2, 0, "test context 2");
    assert_eq!(migrated2, 0);

    // Mark some as test code
    migrator.mark_test_unwrap();
    migrator.mark_test_unwrap();

    // Check statistics
    let stats = migrator.stats();
    assert_eq!(stats.migrated_unwraps, 2);
    assert_eq!(stats.test_unwraps, 2);
    assert_eq!(stats.total_unwraps, 4);
    println!(
        "   ✅ Migration tracking: {} migrated, {} in tests",
        stats.migrated_unwraps, stats.test_unwraps
    );

    // Generate report
    let report = migrator.generate_report();
    assert!(report.contains("Migration"));
    assert!(report.contains("50.0%")); // 2 out of 4 migrated
    println!("   ✅ Report generation working");

    // Test 4: Safe Witness operations
    println!("\n4️⃣ Testing Safe Witness Operations...");

    // Create witness safely
    let witness = SafeWitnessBuilder::new()
        .with_invocation_script(vec![0x01, 0x02, 0x03])
        .with_verification_script(vec![0x04, 0x05, 0x06])
        .build();

    assert!(witness.is_ok());
    let witness = witness.unwrap();
    println!("   ✅ Witness created safely with builder");

    // Test serialization
    let serialized = SafeWitnessOperations::serialize_witness(&witness);
    assert!(serialized.is_ok());
    let bytes = serialized.unwrap();
    println!("   ✅ Witness serialized safely: {} bytes", bytes.len());

    // Test deserialization
    let deserialized = SafeWitnessOperations::deserialize_witness(&bytes);
    assert!(deserialized.is_ok());
    let restored = deserialized.unwrap();
    assert_eq!(witness.invocation_script, restored.invocation_script);
    assert_eq!(witness.verification_script, restored.verification_script);
    println!("   ✅ Witness deserialized safely");

    // Test round-trip
    let round_trip = SafeWitnessOperations::test_witness_round_trip(&witness);
    assert!(round_trip.is_ok());
    assert!(round_trip.unwrap());
    println!("   ✅ Round-trip validation passed");

    // Test validation
    let validation = SafeWitnessOperations::validate_witness(&witness);
    assert!(validation.is_ok());
    println!("   ✅ Witness validation passed");

    // Test validation failure
    let empty_witness = Witness::new_with_scripts(vec![], vec![0x01]);
    let validation = SafeWitnessOperations::validate_witness(&empty_witness);
    assert!(validation.is_err());
    assert!(validation
        .unwrap_err()
        .to_string()
        .contains("Invocation script cannot be empty"));
    println!("   ✅ Invalid witness rejected with proper error");

    // Test 5: Migration patterns
    println!("\n5️⃣ Testing Migration Patterns...");

    // Test simple unwrap migration
    let value = Some(999);
    let migrated = migration_patterns::migrate_simple_unwrap(value, 0, "pattern test");
    assert_eq!(migrated, 999);
    println!("   ✅ Simple unwrap pattern working");

    // Test option unwrap migration
    let option = Some(777);
    let migrated = migration_patterns::migrate_option_unwrap(option, "option pattern");
    assert!(migrated.is_ok());
    assert_eq!(migrated.unwrap(), 777);
    println!("   ✅ Option unwrap pattern working");

    // Test None case
    let none: Option<i32> = None;
    let migrated = migration_patterns::migrate_option_unwrap(none, "none pattern");
    assert!(migrated.is_err());
    assert!(migrated.unwrap_err().to_string().contains("none pattern"));
    println!("   ✅ None case handled with context");

    // Test 6: Batch operations
    println!("\n6️⃣ Testing Batch Operations...");

    let witnesses = vec![
        Witness::new_with_scripts(vec![0x01], vec![0x02]),
        Witness::new_with_scripts(vec![0x03], vec![0x04]),
        Witness::new_with_scripts(vec![0x05], vec![0x06]),
    ];

    let batch_result = SafeWitnessOperations::process_witnesses(&witnesses);
    assert!(batch_result.is_ok());
    let processed = batch_result.unwrap();
    assert_eq!(processed.len(), 3);
    println!(
        "   ✅ Batch processing: {} witnesses processed",
        processed.len()
    );

    println!("\n🎉 All Integration Tests Passed!");
    println!("=====================================");
    println!("✅ SafeOption: Context-aware handling");
    println!("✅ SafeResult: Error propagation");
    println!("✅ Migration: Progress tracking");
    println!("✅ Witness: Safe operations");
    println!("✅ Patterns: Migration helpers");
    println!("✅ Batch: Multi-item processing");
    println!("=====================================");
}

#[test]
fn test_error_context_preservation() {
    println!("\n🔍 Testing Error Context Preservation");

    fn inner_operation() -> Result<i32, CoreError> {
        let value: Option<i32> = None;
        value.ok_or_context("Value missing in inner operation")
    }

    fn middle_operation() -> Result<i32, CoreError> {
        inner_operation().with_context("Failed in middle operation")
    }

    fn outer_operation() -> Result<i32, CoreError> {
        middle_operation().with_context("Failed in outer operation")
    }

    let result = outer_operation();
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    println!("Error message: {}", error_msg);

    // Check that all contexts are preserved
    assert!(error_msg.contains("outer operation"));
    assert!(error_msg.contains("middle operation"));
    assert!(error_msg.contains("inner operation"));

    println!("✅ Error context preserved through call stack");
}

#[test]
fn test_panic_prevention() {
    println!("\n🛡️ Testing Panic Prevention");

    // This would panic with unwrap()
    let none_value: Option<i32> = None;

    // But with safe handling, we get a Result
    let result = none_value.ok_or_context("Prevented panic!");

    // No panic occurred, we have an error instead
    assert!(result.is_err());
    println!("✅ Panic prevented, error returned instead");

    // This would panic with expect()
    let error_result: Result<i32, &str> = Err("would panic");

    // But with safe handling, we get a proper error
    let result = error_result.safe_expect("Prevented another panic!");

    assert!(result.is_err());
    println!("✅ Another panic prevented with safe_expect");

    println!("🎯 System is panic-resistant!");
}

#[test]
fn test_migration_completeness() {
    println!("\n📊 Testing Migration Completeness Tracking");

    let mut migrator = UnwrapMigrator::new();

    // Simulate a complete migration scenario
    for i in 0..10 {
        if i < 7 {
            // Migrate most unwraps
            let value = Some(i);
            migrator.migrate_unwrap(value, 0, &format!("context {}", i));
        } else {
            // Some are in test code
            migrator.mark_test_unwrap();
        }
    }

    let stats = migrator.stats();
    println!("Migration Stats:");
    println!("  Total unwraps: {}", stats.total_unwraps);
    println!("  Migrated: {}", stats.migrated_unwraps);
    println!("  Test code: {}", stats.test_unwraps);
    println!("  Completion: {:.1}%", stats.completion_percentage());

    assert_eq!(stats.total_unwraps, 10);
    assert_eq!(stats.migrated_unwraps, 7);
    assert_eq!(stats.test_unwraps, 3);
    assert!(stats.is_complete());
    assert_eq!(stats.completion_percentage(), 70.0);

    println!("✅ Migration tracking accurate and complete");
}

#[test]
fn test_performance_impact() {
    use std::time::Instant;

    println!("\n⚡ Testing Performance Impact");

    const ITERATIONS: usize = 100_000;

    // Measure safe operations
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let value = Some(42);
        let _ = value.safe_unwrap_or(0, "test");
    }
    let safe_duration = start.elapsed();

    // Measure with default unwrap_or
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let value = Some(42);
        let _ = value.unwrap_or(0);
    }
    let unsafe_duration = start.elapsed();

    println!(
        "Safe operations: {:?} for {} iterations",
        safe_duration, ITERATIONS
    );
    println!(
        "Unsafe operations: {:?} for {} iterations",
        unsafe_duration, ITERATIONS
    );

    let overhead = if safe_duration > unsafe_duration {
        ((safe_duration.as_nanos() as f64 - unsafe_duration.as_nanos() as f64)
            / unsafe_duration.as_nanos() as f64)
            * 100.0
    } else {
        0.0
    };

    println!("Overhead: {:.2}%", overhead);

    // Assert overhead is reasonable (< 10%)
    assert!(
        overhead < 10.0,
        "Performance overhead too high: {:.2}%",
        overhead
    );

    println!("✅ Performance impact is acceptable");
}

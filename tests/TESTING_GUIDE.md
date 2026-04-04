# Neo-RS Testing Guide

> Comprehensive guide for testing the Neo-RS blockchain implementation

## Table of Contents
1. [Quick Start](#quick-start)
2. [Test Organization](#test-organization)
3. [Running Tests](#running-tests)
4. [Writing Tests](#writing-tests)
5. [Advanced Testing](#advanced-testing)
6. [CI/CD Integration](#cicd-integration)
7. [Troubleshooting](#troubleshooting)

## Quick Start

### Basic Commands
```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p epicchain-core

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in parallel
cargo test -- --test-threads=4
```

### Using Test Scripts
```bash
# Quick test suite
./scripts/test-runner.sh --quick

# Full test suite with coverage
./scripts/test-runner.sh --coverage

# Automated improvements
./scripts/test-improvement.sh

# Complete orchestration
./scripts/test-orchestrator.sh
```

## Test Organization

### Directory Structure
```
neo-rs/
├── tests/                    # Integration tests
│   ├── examples/            # Example test patterns
│   ├── fixtures/            # Test data and fixtures
│   └── common/              # Shared test utilities
├── crates/
│   └── */
│       ├── src/
│       │   └── *.rs         # Unit tests in source files
│       └── tests/           # Crate-specific integration tests
├── benches/                 # Performance benchmarks
└── scripts/                 # Test automation scripts
```

### Test Categories

#### Unit Tests
Located in source files with `#[cfg(test)]` modules:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        assert_eq!(2 + 2, 4);
    }
}
```

#### Integration Tests
Located in `tests/` directories:
```rust
// tests/integration_test.rs
use neo_rs::*;

#[test]
fn test_integration() {
    // Test across module boundaries
}
```

#### Documentation Tests
In doc comments:
```rust
/// Adds two numbers
/// 
/// # Example
/// ```
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Running Tests

### Standard Test Execution
```bash
# All tests
cargo test --workspace

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Doc tests only
cargo test --doc

# With all features
cargo test --all-features
```

### Coverage Testing
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# With specific threshold
cargo tarpaulin --fail-under 80
```

### Mutation Testing
```bash
# Install mutants
cargo install cargo-mutants

# Run mutation tests
./scripts/mutation-testing.sh

# Quick mutation test
cargo mutants --package epicchain-core
```

### Performance Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench bench_name

# Compare with baseline
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

## Writing Tests

### Best Practices

#### 1. Test Naming
```rust
#[test]
fn test_component_behavior_when_condition() {
    // Clear, descriptive test names
}

#[test]
fn transfer_fails_when_insufficient_balance() {
    // Good: Describes behavior and condition
}
```

#### 2. Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Group related tests
    mod transaction_tests {
        #[test]
        fn creation() { }
        
        #[test]
        fn validation() { }
    }
    
    mod block_tests {
        #[test]
        fn creation() { }
    }
}
```

#### 3. Test Helpers
```rust
#[cfg(test)]
mod tests {
    // Create test fixtures
    fn create_test_transaction() -> Transaction {
        Transaction::new(/* ... */)
    }
    
    // Use builder patterns
    struct TestBlockBuilder { /* ... */ }
}
```

### Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_serialization_roundtrip(data: Vec<u8>) {
        let original = Transaction::from_bytes(&data);
        let serialized = original.to_bytes();
        let deserialized = Transaction::from_bytes(&serialized);
        prop_assert_eq!(original, deserialized);
    }
}
```

### Async Testing
```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Mock Testing
```rust
use mockall::*;

#[automock]
trait Database {
    fn get(&self, key: &str) -> Option<String>;
}

#[test]
fn test_with_mock() {
    let mut mock = MockDatabase::new();
    mock.expect_get()
        .with(eq("key"))
        .return_const(Some("value".to_string()));
    
    // Use mock in test
}
```

## Advanced Testing

### Fuzzing
```rust
// fuzz/fuzz_targets/fuzz_target.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz testing logic
    let _ = parse_transaction(data);
});
```

Run fuzzing:
```bash
cargo fuzz run fuzz_target
```

### Snapshot Testing
```rust
use insta::assert_snapshot;

#[test]
fn test_serialization() {
    let obj = create_complex_object();
    assert_snapshot!(obj.to_string());
}
```

### Test Fixtures
```rust
use once_cell::sync::Lazy;

static TEST_DATA: Lazy<TestData> = Lazy::new(|| {
    load_test_data()
});

#[test]
fn test_with_fixture() {
    assert_eq!(TEST_DATA.value, expected);
}
```

## CI/CD Integration

### GitHub Actions
The project includes comprehensive CI/CD in `.github/workflows/test-suite.yml`:
- Quick tests on every PR
- Full test matrix across OS and Rust versions
- Coverage reporting with Codecov
- Security auditing
- Performance benchmarking
- Weekly mutation testing

### Local CI Testing
```bash
# Install act for local GitHub Actions
brew install act  # macOS
# or
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run CI locally
act -j quick-tests
```

## Test Metrics

### Coverage Goals
- **Overall**: >80% line coverage
- **Core modules**: >90% coverage
- **Critical paths**: 100% coverage
- **Error handling**: >75% coverage

### Performance Targets
- **Unit tests**: <20ms per test
- **Integration tests**: <200ms per test
- **Full suite**: <5 minutes
- **CI pipeline**: <10 minutes

## Troubleshooting

### Common Issues

#### 1. Test Timeout
```bash
# Increase timeout
cargo test -- --test-threads=1 --nocapture

# For specific test
#[test]
#[timeout(1000)]  // milliseconds
fn slow_test() { }
```

#### 2. Flaky Tests
```rust
// Use retry for flaky tests
#[test]
#[retry(3)]
fn potentially_flaky_test() { }
```

#### 3. Resource Cleanup
```rust
// Use Drop trait for cleanup
struct TestResource;

impl Drop for TestResource {
    fn drop(&mut self) {
        // Cleanup code
    }
}
```

#### 4. Test Isolation
```bash
# Run tests serially
cargo test -- --test-threads=1

# Use separate test databases
#[test]
fn test_with_isolation() {
    let db = create_test_db();
    // Test code
    drop(db); // Cleanup
}
```

## Test Automation Scripts

### Available Scripts
- `test-runner.sh` - Enhanced test runner with options
- `test-improvement.sh` - Automated test improvements
- `test-orchestrator.sh` - Complete test orchestration
- `coverage-tracker.sh` - Coverage monitoring
- `mutation-testing.sh` - Mutation test execution
- `fix-test-warnings.sh` - Warning cleanup
- `add-documentation.sh` - Documentation generation

### Creating Custom Scripts
```bash
#!/bin/bash
# scripts/custom-test.sh

# Your custom test logic
cargo test --features custom
# Additional processing
```

## Best Practices Summary

1. **Write tests first** (TDD approach)
2. **Keep tests simple** and focused
3. **Use descriptive names** for tests
4. **Test edge cases** and error conditions
5. **Maintain test isolation** (no shared state)
6. **Mock external dependencies**
7. **Use property-based testing** for complex logic
8. **Monitor test coverage** continuously
9. **Run tests locally** before pushing
10. **Document test purposes** when not obvious

## Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Proptest Documentation](https://proptest-rs.github.io/proptest/)
- [Criterion.rs Guide](https://bheisler.github.io/criterion.rs/book/)
- [Tarpaulin Coverage Tool](https://github.com/xd009642/tarpaulin)

---

*Last updated: 2025-01-14*
//! Migration utilities for replacing unwrap() calls with safe alternatives
//!
//! This module provides automated helpers to migrate from panic-prone unwrap()
//! calls to safe error handling patterns.

use crate::error::CoreError;
use crate::safe_result::{SafeOption, SafeResult};

/// Helper struct to track unwrap migration progress
#[derive(Debug, Default)]
pub struct UnwrapMigrationStats {
    /// Total unwrap calls found
    pub total_unwraps: usize,
    /// Successfully migrated unwraps
    pub migrated_unwraps: usize,
    /// Unwraps in test code (can be kept)
    pub test_unwraps: usize,
    /// Unwraps that need manual review
    pub manual_review_needed: usize,
}

impl UnwrapMigrationStats {
    /// Calculate migration completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.total_unwraps == 0 {
            100.0
        } else {
            (self.migrated_unwraps as f64 / self.total_unwraps as f64) * 100.0
        }
    }

    /// Check if migration is complete
    pub fn is_complete(&self) -> bool {
        self.migrated_unwraps + self.test_unwraps == self.total_unwraps
    }
}

/// Example migration patterns for common unwrap scenarios
pub mod migration_patterns {
    use super::*;

    /// Before: value.unwrap()
    /// After: value.safe_unwrap_or(default, "context")
    pub fn migrate_simple_unwrap<T>(value: Option<T>, default: T, context: &str) -> T {
        value.safe_unwrap_or(default, context)
    }

    /// Before: result.unwrap()
    /// After: result.with_context("context")?
    pub fn migrate_result_unwrap<T, E: std::fmt::Display>(
        result: Result<T, E>,
        context: &str,
    ) -> Result<T, CoreError> {
        result.with_context(context)
    }

    /// Before: option.unwrap()
    /// After: option.ok_or_context("context")?
    pub fn migrate_option_unwrap<T>(option: Option<T>, context: &str) -> Result<T, CoreError> {
        option.ok_or_context(context)
    }

    /// Before: value.expect("message")
    /// After: value.safe_expect("message")?
    pub fn migrate_expect<T>(value: Option<T>, message: &str) -> Result<T, CoreError> {
        value.safe_expect(message)
    }
}

/// Automated migration helper for common patterns
pub struct UnwrapMigrator {
    stats: UnwrapMigrationStats,
}

impl UnwrapMigrator {
    /// Create a new migrator instance
    pub fn new() -> Self {
        Self {
            stats: UnwrapMigrationStats::default(),
        }
    }

    /// Get current migration statistics
    pub fn stats(&self) -> &UnwrapMigrationStats {
        &self.stats
    }

    /// Migrate a simple unwrap to safe alternative
    pub fn migrate_unwrap<T>(&mut self, value: Option<T>, default: T, context: &str) -> T {
        self.stats.total_unwraps += 1;
        self.stats.migrated_unwraps += 1;
        value.safe_unwrap_or(default, context)
    }

    /// Migrate a result unwrap to safe alternative
    pub fn migrate_result<T, E: std::fmt::Display>(
        &mut self,
        result: Result<T, E>,
        context: &str,
    ) -> Result<T, CoreError> {
        self.stats.total_unwraps += 1;
        self.stats.migrated_unwraps += 1;
        result.with_context(context)
    }

    /// Mark an unwrap as being in test code
    pub fn mark_test_unwrap(&mut self) {
        self.stats.total_unwraps += 1;
        self.stats.test_unwraps += 1;
    }

    /// Mark an unwrap as needing manual review
    pub fn mark_needs_review(&mut self) {
        self.stats.total_unwraps += 1;
        self.stats.manual_review_needed += 1;
    }

    /// Generate a migration report
    pub fn generate_report(&self) -> String {
        format!(
            "Unwrap Migration Report\n\
             =======================\n\
             Total unwraps found: {}\n\
             Successfully migrated: {}\n\
             Test code unwraps: {}\n\
             Needs manual review: {}\n\
             Completion: {:.1}%\n\
             Status: {}",
            self.stats.total_unwraps,
            self.stats.migrated_unwraps,
            self.stats.test_unwraps,
            self.stats.manual_review_needed,
            self.stats.completion_percentage(),
            if self.stats.is_complete() {
                "✅ Migration Complete"
            } else {
                "⚠️ Migration In Progress"
            }
        )
    }
}

impl Default for UnwrapMigrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_migration_stats() {
        let mut stats = UnwrapMigrationStats::default();
        stats.total_unwraps = 100;
        stats.migrated_unwraps = 75;
        stats.test_unwraps = 20;
        stats.manual_review_needed = 5;

        assert_eq!(stats.completion_percentage(), 75.0);
        assert!(!stats.is_complete());

        stats.migrated_unwraps = 80;
        assert!(stats.is_complete());
    }

    #[test]
    fn test_unwrap_migrator() {
        let mut migrator = UnwrapMigrator::new();

        // Test simple unwrap migration
        let value = Some(42);
        let result = migrator.migrate_unwrap(value, 0, "test context");
        assert_eq!(result, 42);
        assert_eq!(migrator.stats().migrated_unwraps, 1);

        // Test None case
        let value: Option<i32> = None;
        let result = migrator.migrate_unwrap(value, 0, "test context");
        assert_eq!(result, 0);
        assert_eq!(migrator.stats().migrated_unwraps, 2);

        // Mark test unwrap
        migrator.mark_test_unwrap();
        assert_eq!(migrator.stats().test_unwraps, 1);

        // Generate report - we have 2 migrated + 1 test = 3 total, which is complete
        let report = migrator.generate_report();
        assert!(report.contains("Migration Complete"));

        // Add one more that needs review to test "In Progress" state
        migrator.mark_needs_review();
        let report = migrator.generate_report();
        assert!(report.contains("Migration In Progress"));
    }

    #[test]
    fn test_migration_patterns() {
        use migration_patterns::*;

        // Test simple unwrap migration
        let value = Some(42);
        let result = migrate_simple_unwrap(value, 0, "test");
        assert_eq!(result, 42);

        // Test result unwrap migration
        let result: Result<i32, &str> = Ok(42);
        let migrated = migrate_result_unwrap(result, "test").unwrap();
        assert_eq!(migrated, 42);

        // Test option unwrap migration
        let option = Some(42);
        let migrated = migrate_option_unwrap(option, "test").unwrap();
        assert_eq!(migrated, 42);

        // Test expect migration
        let value = Some(42);
        let migrated = migrate_expect(value, "expected value").unwrap();
        assert_eq!(migrated, 42);
    }
}

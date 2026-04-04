//! Migration Helpers for Safe Error Handling
//!
//! This module provides utilities to help migrate existing code from
//! unsafe unwrap() and panic!() patterns to the new safe error handling system.

use crate::safe_error_handling::{SafeError, SafeExpect, SafeUnwrap};
use std::fmt::Debug;

/// Helper macro to migrate unwrap() calls with minimal code changes
///
/// Usage:
/// ```ignore
/// // Before:
/// let value = some_result.unwrap();
///
/// // After:
/// let value = migrate_unwrap!(some_result, "operation context")?;
/// ```
#[macro_export]
macro_rules! migrate_unwrap {
    ($expr:expr, $context:expr) => {
        $expr.safe_unwrap($context)
    };
}

/// Helper macro to migrate expect() calls
///
/// Usage:
/// ```ignore
/// // Before:
/// let value = some_option.expect("error message");
///
/// // After:
/// let value = migrate_expect!(some_option, "error message")?;
/// ```
#[macro_export]
macro_rules! migrate_expect {
    ($expr:expr, $msg:expr) => {
        $expr.safe_expect($msg)
    };
}

/// A wrapper type that provides gradual migration from unwrap to safe handling
pub struct MigrationWrapper<T> {
    inner: T,
    context: String,
}

impl<T> MigrationWrapper<T> {
    /// Create a new migration wrapper with context
    pub fn new(value: T, context: impl Into<String>) -> Self {
        Self {
            inner: value,
            context: context.into(),
        }
    }

    /// Get the inner value
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Get a reference to the inner value
    pub fn as_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the inner value
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> MigrationWrapper<Option<T>> {
    /// Safe version of unwrap for Option
    pub fn safe_unwrap(self) -> Result<T, SafeError> {
        self.inner.safe_expect(&self.context)
    }

    /// Safe version of unwrap_or
    pub fn safe_unwrap_or(self, default: T) -> T {
        self.inner.unwrap_or(default)
    }
}

impl<T, E> MigrationWrapper<Result<T, E>>
where
    E: std::error::Error + 'static,
{
    /// Safe version of unwrap for Result
    pub fn safe_unwrap(self) -> Result<T, SafeError> {
        self.inner.safe_unwrap(&self.context)
    }

    /// Safe version of unwrap_or for Result
    pub fn safe_unwrap_or(self, default: T) -> T {
        self.inner.unwrap_or(default)
    }
}

/// Trait for types that can be safely converted from legacy error handling
pub trait LegacyErrorConversion {
    /// Convert from a legacy error pattern to safe error
    fn to_safe_error(self, context: impl Into<String>) -> SafeError;
}

impl LegacyErrorConversion for String {
    fn to_safe_error(self, context: impl Into<String>) -> SafeError {
        SafeError::new(self, context)
    }
}

impl LegacyErrorConversion for &str {
    fn to_safe_error(self, context: impl Into<String>) -> SafeError {
        SafeError::new(self, context)
    }
}

/// Helper function to safely handle vector access
pub fn safe_vec_get<T: Clone>(vec: &[T], index: usize, context: &str) -> Result<T, SafeError> {
    vec.get(index).cloned().ok_or_else(|| {
        SafeError::new(
            format!(
                "Index {} out of bounds for vector of length {}",
                index,
                vec.len()
            ),
            context,
        )
    })
}

/// Helper function to safely handle hashmap access
pub fn safe_map_get<'a, K, V>(
    map: &'a std::collections::HashMap<K, V>,
    key: &K,
    context: &str,
) -> Result<&'a V, SafeError>
where
    K: Debug + Eq + std::hash::Hash,
{
    map.get(key)
        .ok_or_else(|| SafeError::new(format!("Key {:?} not found in map", key), context))
}

/// Batch error handler for collecting multiple potential errors
pub struct BatchErrorHandler {
    errors: Vec<SafeError>,
    context: String,
}

impl BatchErrorHandler {
    /// Create a new batch error handler
    pub fn new(context: impl Into<String>) -> Self {
        Self {
            errors: Vec::new(),
            context: context.into(),
        }
    }

    /// Try an operation and collect any errors
    pub fn try_operation<T, F>(&mut self, operation: F) -> Option<T>
    where
        F: FnOnce() -> Result<T, SafeError>,
    {
        match operation() {
            Ok(value) => Some(value),
            Err(err) => {
                self.errors.push(err.add_context(&self.context));
                None
            }
        }
    }

    /// Check if any errors occurred
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get all collected errors
    pub fn errors(&self) -> &[SafeError] {
        &self.errors
    }

    /// Convert to a result, returning error if any operations failed
    pub fn to_result<T>(self, success_value: T) -> Result<T, SafeError> {
        if self.errors.is_empty() {
            Ok(success_value)
        } else {
            let error_messages: Vec<String> =
                self.errors.iter().map(|e| format!("{}", e)).collect();
            Err(SafeError::new(
                format!("{} errors occurred", self.errors.len()),
                format!("{}: {}", self.context, error_messages.join("; ")),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_wrapper_option() {
        let wrapper = MigrationWrapper::new(Some(42), "test context");
        let result = wrapper.safe_unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_migration_wrapper_none() {
        let wrapper = MigrationWrapper::new(None::<i32>, "test context");
        let result = wrapper.safe_unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_error_handler() {
        let mut handler = BatchErrorHandler::new("batch operation");

        // Successful operation
        let result1 = handler.try_operation(|| Ok(42));
        assert_eq!(result1, Some(42));

        // Failed operation
        let result2: Option<i32> =
            handler.try_operation(|| Err(SafeError::new("test error", "test")));
        assert_eq!(result2, None);

        assert!(handler.has_errors());
        assert_eq!(handler.errors().len(), 1);
    }

    #[test]
    fn test_safe_vec_get() {
        let vec = vec![1, 2, 3];

        // Valid index
        let result = safe_vec_get(&vec, 1, "getting element");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);

        // Invalid index
        let result = safe_vec_get(&vec, 10, "getting element");
        assert!(result.is_err());
    }

    #[test]
    fn test_legacy_error_conversion() {
        let error_string = "legacy error".to_string();
        let safe_error = error_string.to_safe_error("migration context");
        assert_eq!(safe_error.message, "legacy error");
        assert_eq!(safe_error.context, "migration context");
    }
}

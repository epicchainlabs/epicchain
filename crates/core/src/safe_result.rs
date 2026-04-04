//! Safe Result handling utilities for EpicChain-RS
//!
//! This module provides safe alternatives to unwrap() and expect() calls,
//! ensuring proper error propagation and preventing runtime panics.

use crate::error::CoreError;
use std::fmt::Display;

/// Extension trait for Result types providing safe handling methods
pub trait SafeResult<T> {
    /// Safely unwrap with context-aware error message
    fn safe_unwrap_or(self, default: T, context: &str) -> T;

    /// Safely unwrap with error logging
    fn safe_unwrap_or_log(self, default: T, context: &str) -> T;

    /// Convert to CoreError with context
    fn with_context(self, context: &str) -> Result<T, CoreError>;

    /// Safe expect alternative that returns Result
    fn safe_expect(self, msg: &str) -> Result<T, CoreError>;
}

impl<T, E: Display> SafeResult<T> for Result<T, E> {
    fn safe_unwrap_or(self, default: T, context: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                log::warn!("Error in {}: {}", context, e);
                default
            }
        }
    }

    fn safe_unwrap_or_log(self, default: T, context: &str) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                log::error!("Critical error in {}: {}", context, e);
                // In production, we might want to increment error metrics here
                default
            }
        }
    }

    fn with_context(self, context: &str) -> Result<T, CoreError> {
        self.map_err(|e| CoreError::InvalidOperation {
            message: format!("{}: {}", context, e),
        })
    }

    fn safe_expect(self, msg: &str) -> Result<T, CoreError> {
        self.map_err(|e| CoreError::InvalidOperation {
            message: format!("{}: {}", msg, e),
        })
    }
}

/// Extension trait for Option types providing safe handling methods
pub trait SafeOption<T> {
    /// Safely unwrap Option with default value
    fn safe_unwrap_or(self, default: T, context: &str) -> T;

    /// Convert Option to Result with context
    fn ok_or_context(self, context: &str) -> Result<T, CoreError>;

    /// Safe expect alternative for Options
    fn safe_expect(self, msg: &str) -> Result<T, CoreError>;
}

impl<T> SafeOption<T> for Option<T> {
    fn safe_unwrap_or(self, default: T, context: &str) -> T {
        match self {
            Some(value) => value,
            None => {
                log::warn!("None value encountered in {}", context);
                default
            }
        }
    }

    fn ok_or_context(self, context: &str) -> Result<T, CoreError> {
        self.ok_or_else(|| CoreError::InvalidData {
            message: format!("Missing value in {}", context),
        })
    }

    fn safe_expect(self, msg: &str) -> Result<T, CoreError> {
        self.ok_or_else(|| CoreError::InvalidData {
            message: msg.to_string(),
        })
    }
}

/// Macro for safe try operations with automatic context
#[macro_export]
macro_rules! safe_try {
    ($expr:expr, $context:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                log::error!("Error in {}: {:?}", $context, e);
                return Err($crate::error::CoreError::InvalidOperation {
                    message: format!("{}: {}", $context, e),
                });
            }
        }
    };
}

/// Macro for safe option handling with automatic context
#[macro_export]
macro_rules! safe_some {
    ($expr:expr, $context:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                log::error!("None value in {}", $context);
                return Err($crate::error::CoreError::InvalidData {
                    message: format!("Missing value in {}", $context),
                });
            }
        }
    };
}

/// Safe wrapper for potentially panicking operations
pub fn safe_operation<T, F>(operation: F, context: &str) -> Result<T, CoreError>
where
    F: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
{
    operation().map_err(|e| CoreError::InvalidOperation {
        message: format!("{}: {}", context, e),
    })
}

/// Safe wrapper for async operations
pub async fn safe_async_operation<T, F, Fut>(operation: F, context: &str) -> Result<T, CoreError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    operation().await.map_err(|e| CoreError::InvalidOperation {
        message: format!("{}: {}", context, e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_result_unwrap_or() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(result.safe_unwrap_or(0, "test"), 42);

        let result: Result<i32, &str> = Err("error");
        assert_eq!(result.safe_unwrap_or(0, "test"), 0);
    }

    #[test]
    fn test_safe_option_unwrap_or() {
        let option: Option<i32> = Some(42);
        assert_eq!(option.safe_unwrap_or(0, "test"), 42);

        let option: Option<i32> = None;
        assert_eq!(option.safe_unwrap_or(0, "test"), 0);
    }

    #[test]
    fn test_with_context() {
        let result: Result<i32, &str> = Err("error");
        let contextualized = result.with_context("operation failed");
        assert!(contextualized.is_err());

        if let Err(CoreError::InvalidOperation { message }) = contextualized {
            assert!(message.contains("operation failed"));
        } else {
            panic!("Expected InvalidOperation error");
        }
    }

    #[test]
    fn test_safe_operation() {
        let result = safe_operation(
            || Ok::<i32, Box<dyn std::error::Error>>(42),
            "test operation",
        );
        assert_eq!(result.unwrap(), 42);

        let result = safe_operation(
            || Err::<i32, Box<dyn std::error::Error>>("error".into()),
            "test operation",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_try_macro() {
        fn test_function() -> Result<i32, CoreError> {
            let value = safe_try!(Ok::<i32, &str>(42), "getting value");
            Ok(value)
        }

        assert_eq!(test_function().unwrap(), 42);
    }

    #[test]
    fn test_safe_some_macro() {
        fn test_function() -> Result<i32, CoreError> {
            let value = safe_some!(Some(42), "getting value");
            Ok(value)
        }

        assert_eq!(test_function().unwrap(), 42);
    }
}

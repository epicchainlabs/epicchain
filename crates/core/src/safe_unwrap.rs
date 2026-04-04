//! Safe unwrap utilities for production-ready code
//!
//! This module provides safe alternatives to unwrap() and expect() that prevent
//! panics in production while maintaining good error context.

use std::fmt::Display;
use tracing::{error, warn};

/// Extension trait for Option types providing safe unwrap alternatives
pub trait SafeUnwrapOption<T> {
    /// Returns the value or a default if None, logging a warning
    fn unwrap_or_log_default(self, context: &str, _default: T) -> T;

    /// Returns the value or executes a fallback function, logging the error
    fn unwrap_or_else_log<F>(self, context: &str, _f: F) -> T
    where
        F: FnOnce() -> T;

    /// Converts to Result with a custom error message
    fn ok_or_log<E>(self, context: &str, _error: E) -> Result<T, E>;
}

/// Extension trait for Result types providing safe unwrap alternatives
pub trait SafeUnwrapResult<T, E: Display> {
    /// Returns the value or a default if Err, logging the error
    fn unwrap_or_log_default(self, context: &str, _default: T) -> T;

    /// Returns the value or executes a fallback function, logging the error
    fn unwrap_or_else_log<F>(self, context: &str, _f: F) -> T
    where
        F: FnOnce(E) -> T;

    /// Maps the error to a new type while logging the original error
    fn map_err_log<E2>(self, context: &str, _new_error: E2) -> Result<T, E2>;
}

impl<T> SafeUnwrapOption<T> for Option<T> {
    fn unwrap_or_log_default(self, context: &str, _default: T) -> T {
        match self {
            Some(value) => value,
            None => {
                warn!("Using default value in {}: None encountered", context);
                default
            }
        }
    }

    fn unwrap_or_else_log<F>(self, context: &str, _f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(value) => value,
            None => {
                warn!("Executing fallback in {}: None encountered", context);
                f()
            }
        }
    }

    fn ok_or_log<E>(self, context: &str, _error: E) -> Result<T, E> {
        match self {
            Some(value) => Ok(value),
            None => {
                warn!("Converting None to error in {}", context);
                Err(error)
            }
        }
    }
}

impl<T, E: Display> SafeUnwrapResult<T, E> for Result<T, E> {
    fn unwrap_or_log_default(self, context: &str, _default: T) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                error!("Error in {}: {}. Using default value", context, e);
                default
            }
        }
    }

    fn unwrap_or_else_log<F>(self, context: &str, _f: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Ok(value) => value,
            Err(e) => {
                error!("Error in {}: {}. Executing fallback", context, &e);
                f(e)
            }
        }
    }

    fn map_err_log<E2>(self, context: &str, _new_error: E2) -> Result<T, E2> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Mapping error in {}: {}", context, e);
                Err(new_error)
            }
        }
    }
}

/// Safe alternative to unwrap() for production code
#[macro_export]
macro_rules! safe_unwrap {
    ($expr:expr, $context:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                ::tracing::error!("Unwrap failed at {}: None value", $context);
                return Err($crate::error::CoreError::UnexpectedNone($context.into()));
            }
        }
    };
    ($expr:expr, $context:expr, $default:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                ::tracing::warn!("Using default value at {}: None encountered", $context);
                $default
            }
        }
    };
}

/// Safe alternative to expect() for production code
#[macro_export]
macro_rules! safe_expect {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                ::tracing::error!("Expectation failed: {}", $msg);
                return Err($crate::error::CoreError::ExpectationFailed($msg.into()));
            }
        }
    };
}

/// Try operation with error context
#[macro_export]
macro_rules! try_with_context {
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            ::tracing::error!("Operation failed in {}: {:?}", $context, e);
            $crate::error::CoreError::OperationFailed {
                context: $context.into(),
                details: format!("{:?}", e),
            }
        })?
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_unwrap_option() {
        let some_value: Option<i32> = Some(42);
        assert_eq!(some_value.unwrap_or_log_default("test", 0), 42);

        let none_value: Option<i32> = None;
        assert_eq!(none_value.unwrap_or_log_default("test", 99), 99);
    }

    #[test]
    fn test_safe_unwrap_result() {
        let ok_value: Result<i32, String> = Ok(42);
        assert_eq!(ok_value.unwrap_or_log_default("test", 0), 42);

        let err_value: Result<i32, String> = Err("error".to_string());
        assert_eq!(err_value.unwrap_or_log_default("test", 99), 99);
    }
}

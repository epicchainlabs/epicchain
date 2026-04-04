use std::fmt::Display;

/// Common error conversion utilities to reduce duplication
/// Defines a trait interface.
pub trait ErrorMapper<T> {
    /// Maps an error to a formatted error with context
    fn map_err_context<E, F>(self, context: F) -> Result<T, E>
    where
        F: FnOnce() -> String,
        E: From<String>;
}

impl<T, E: Display> ErrorMapper<T> for Result<T, E> {
    fn map_err_context<E2, F>(self, context: F) -> Result<T, E2>
    where
        F: FnOnce() -> String,
        E2: From<String>,
    {
        self.map_err(|e| E2::from(format!("{}: {}", context(), e)))
    }
}

/// Macro for consistent error formatting
#[macro_export]
macro_rules! format_err {
    ($msg:expr) => {
        format!($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        format!($fmt, $($arg)*)
    };
}

/// Macro for mapping errors with context
#[macro_export]
macro_rules! map_err_ctx {
    ($result:expr, $context:expr) => {
        $result.map_err(|e| format!("{}: {}", $context, e))
    };
}

/// Helper trait for consistent error conversion
/// Defines a trait interface.
pub trait IntoError<E> {
    /// Converts self into an error with context.
    fn into_error(self, context: &str) -> E;
}

impl<T: Display> IntoError<String> for T {
    fn into_error(self, context: &str) -> String {
        format!("{context}: {self}")
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{CoreError as Error, CoreResult as Result};

    #[test]
    fn test_error_mapper() {
        let result: Result<i32> = Err(Error::InvalidFormat {
            message: "original error".to_string(),
        });
        let mapped: std::result::Result<i32, String> =
            result.map_err_context(|| "Failed to process".to_string());
        assert!(mapped
            .unwrap_err()
            .to_string()
            .contains("Failed to process"));
    }

    #[test]
    fn test_format_err_macro() {
        let err = format_err!("Simple error");
        assert_eq!(err, "Simple error");

        let err = format_err!("Error with {}: {}", "context", 42);
        assert_eq!(err, "Error with context: 42");
    }

    #[test]
    fn test_into_error() {
        let err = "base error";
        let converted: String = err.into_error("Context");
        assert_eq!(converted, "Context: base error");
    }
}

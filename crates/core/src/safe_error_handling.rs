//! Safe Error Handling Module
//!
//! This module provides safe alternatives to unwrap() and panic!() calls,
//! improving the robustness and production-readiness of the EpicChain-RS codebase.

use std::error::Error as StdError;
use std::fmt;
use std::panic::Location;

/// A trait extension for Result types that provides safe unwrapping with context
pub trait SafeUnwrap<T> {
    /// Safely unwrap a Result with context information
    fn safe_unwrap(self, context: &str) -> Result<T, SafeError>;

    /// Unwrap with a default value on error
    fn unwrap_or_default_with_log(self, context: &str) -> T
    where
        T: Default;

    /// Convert error to SafeError with context
    fn with_context(self, context: impl Into<String>) -> Result<T, SafeError>;
}

/// A trait extension for Option types that provides safe unwrapping
pub trait SafeExpect<T> {
    /// Safely expect a value with context information
    fn safe_expect(self, context: &str) -> Result<T, SafeError>;

    /// Convert None to an error with context
    fn ok_or_context(self, context: impl Into<String>) -> Result<T, SafeError>;
}

/// Safe error type that captures context and location information
#[derive(Debug, Clone)]
pub struct SafeError {
    /// The error message
    pub message: String,
    /// Context about where/why the error occurred
    pub context: String,
    /// Source location of the error
    pub location: Option<String>,
    /// Original error if available
    pub source: Option<String>,
}

impl SafeError {
    /// Create a new SafeError with context
    #[track_caller]
    pub fn new(message: impl Into<String>, context: impl Into<String>) -> Self {
        let location = Location::caller();
        Self {
            message: message.into(),
            context: context.into(),
            location: Some(format!("{}:{}", location.file(), location.line())),
            source: None,
        }
    }

    /// Create SafeError from another error type
    #[track_caller]
    pub fn from_error<E: StdError>(error: E, context: impl Into<String>) -> Self {
        let location = Location::caller();
        Self {
            message: error.to_string(),
            context: context.into(),
            location: Some(format!("{}:{}", location.file(), location.line())),
            source: Some(format!("{:?}", error)),
        }
    }

    /// Add additional context to an existing error
    pub fn add_context(mut self, additional: impl Into<String>) -> Self {
        self.context = format!("{} | {}", self.context, additional.into());
        self
    }
}

impl fmt::Display for SafeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} | Context: {}", self.message, self.context)?;
        if let Some(ref loc) = self.location {
            write!(f, " | Location: {}", loc)?;
        }
        Ok(())
    }
}

impl StdError for SafeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// Implement SafeUnwrap for Result<T, E> where E: StdError
impl<T, E> SafeUnwrap<T> for Result<T, E>
where
    E: StdError + 'static,
{
    #[track_caller]
    fn safe_unwrap(self, context: &str) -> Result<T, SafeError> {
        self.map_err(|e| SafeError::from_error(e, context))
    }

    fn unwrap_or_default_with_log(self, context: &str) -> T
    where
        T: Default,
    {
        match self {
            Ok(val) => val,
            Err(e) => {
                log::error!(
                    "Using default value due to error: {} | Context: {}",
                    e,
                    context
                );
                T::default()
            }
        }
    }

    #[track_caller]
    fn with_context(self, context: impl Into<String>) -> Result<T, SafeError> {
        self.map_err(|e| SafeError::from_error(e, context))
    }
}

// Implement SafeExpect for Option<T>
impl<T> SafeExpect<T> for Option<T> {
    #[track_caller]
    fn safe_expect(self, context: &str) -> Result<T, SafeError> {
        self.ok_or_else(|| SafeError::new("Expected value was None", context))
    }

    #[track_caller]
    fn ok_or_context(self, context: impl Into<String>) -> Result<T, SafeError> {
        self.ok_or_else(|| SafeError::new("Value was None", context))
    }
}

/// Macro to replace panic! with logged error and graceful handling
#[macro_export]
macro_rules! safe_panic {
    ($msg:expr) => {{
        log::error!("Critical error (would panic): {}", $msg);
        Err($crate::safe_error_handling::SafeError::new(
            $msg,
            "Critical error encountered"
        ))
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        let msg = format!($fmt, $($arg)*);
        log::error!("Critical error (would panic): {}", msg);
        Err($crate::safe_error_handling::SafeError::new(
            msg,
            "Critical error encountered"
        ))
    }};
}

/// Macro for safe assertions that return errors instead of panicking
#[macro_export]
macro_rules! safe_assert {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            log::error!("Assertion failed: {}", $msg);
            return Err($crate::safe_error_handling::SafeError::new(
                format!("Assertion failed: {}", $msg),
                "Runtime assertion failure",
            ));
        }
    };
}

/// Helper function to convert multiple Results into a single Result
pub fn collect_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);

    let errs: Vec<E> = errs.into_iter().filter_map(Result::err).collect();

    if errs.is_empty() {
        Ok(oks.into_iter().filter_map(Result::ok).collect())
    } else {
        Err(errs)
    }
}

/// Error chain builder for complex error scenarios
pub struct ErrorChain {
    errors: Vec<SafeError>,
}

impl ErrorChain {
    /// Create a new error chain
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the chain
    pub fn add_error(&mut self, error: SafeError) {
        self.errors.push(error);
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Convert to Result
    pub fn to_result<T>(self) -> Result<T, SafeError> {
        if self.errors.is_empty() {
            return Err(SafeError::new(
                "ErrorChain::to_result called with no errors",
                "Invalid usage - call has_errors() first",
            ));
        }

        let combined_message = self
            .errors
            .iter()
            .map(|e| e.message.clone())
            .collect::<Vec<_>>()
            .join("; ");

        let combined_context = self
            .errors
            .iter()
            .map(|e| e.context.clone())
            .collect::<Vec<_>>()
            .join("; ");

        Err(SafeError::new(combined_message, combined_context))
    }
}

impl Default for ErrorChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_unwrap_ok() {
        let result: Result<i32, std::io::Error> = Ok(42);
        let value = result.safe_unwrap("test context").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_safe_unwrap_err() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        let err = result.safe_unwrap("loading configuration").unwrap_err();
        assert!(err.context.contains("loading configuration"));
        assert!(err.message.contains("file not found"));
    }

    #[test]
    fn test_safe_expect_some() {
        let option = Some(42);
        let value = option.safe_expect("expected value").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_safe_expect_none() {
        let option: Option<i32> = None;
        let err = option
            .safe_expect("expected configuration value")
            .unwrap_err();
        assert!(err.context.contains("expected configuration value"));
    }

    #[test]
    fn test_error_chain() {
        let mut chain = ErrorChain::new();
        chain.add_error(SafeError::new("Error 1", "Context 1"));
        chain.add_error(SafeError::new("Error 2", "Context 2"));

        assert!(chain.has_errors());
        let result: Result<(), SafeError> = chain.to_result();
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.message.contains("Error 1"));
        assert!(err.message.contains("Error 2"));
    }

    #[test]
    fn test_unwrap_or_default_with_log() {
        let result: Result<Vec<i32>, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "not found",
        ));
        let value = result.unwrap_or_default_with_log("getting list");
        assert_eq!(value, Vec::<i32>::new());
    }
}

//! Error types for the Neo I/O crate
//!
//! This module provides comprehensive error handling for I/O operations,
//! including binary serialization, deserialization, and stream operations.

use thiserror::Error;
#[allow(dead_code)]
const DEFAULT_TIMEOUT_MS: u64 = 30000;
/// I/O operation errors
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum IoError {
    /// I/O operation failed
    #[error("I/O operation failed: {operation}, reason: {reason}")]
    Operation { operation: String, reason: String },

    /// Serialization failed
    #[error("Serialization failed: type {type_name}, reason: {reason}")]
    Serialization { type_name: String, reason: String },

    /// Deserialization failed
    #[error("Deserialization failed: expected {expected}, got {actual} bytes, reason: {reason}")]
    Deserialization {
        expected: String,
        actual: usize,
        reason: String,
    },

    /// Invalid data format
    #[error("Invalid format: expected {expected_format}, reason: {reason}")]
    InvalidFormat {
        expected_format: String,
        reason: String,
    },

    /// Invalid data content
    #[error("Invalid data: {context}, value: {value}")]
    InvalidData { context: String, value: String },

    /// Buffer overflow during operation
    #[error("Buffer overflow: attempted to {operation} {attempted} bytes, capacity {capacity}")]
    BufferOverflow {
        operation: String,
        attempted: usize,
        capacity: usize,
    },

    /// Unexpected end of stream
    #[error("Unexpected end of stream: expected {expected} more bytes while reading {context}")]
    EndOfStream { expected: usize, context: String },

    /// Format exception during parsing
    #[error("Format exception: {context}, input: {input}")]
    FormatException { context: String, input: String },

    /// Invalid operation attempted
    #[error("Invalid operation: {operation} not allowed in {context}")]
    InvalidOperation { operation: String, context: String },

    /// Encoding/decoding error
    #[error("Encoding error: {encoding_type}, reason: {reason}")]
    Encoding {
        encoding_type: String,
        reason: String,
    },

    /// Memory allocation failed
    #[error("Memory allocation failed: requested {size} bytes for {purpose}")]
    MemoryAllocation { size: usize, purpose: String },

    /// Stream position error
    #[error("Stream position error: attempted to seek to {position}, stream size {size}")]
    StreamPosition { position: u64, size: u64 },

    /// Stream not readable
    #[error("Stream not readable: {reason}")]
    StreamNotReadable { reason: String },

    /// Stream not writable
    #[error("Stream not writable: {reason}")]
    StreamNotWritable { reason: String },

    /// Checksum mismatch
    #[error("Checksum mismatch: expected {expected}, calculated {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    /// Compression/decompression error
    #[error("Compression error: {algorithm}, reason: {reason}")]
    Compression { algorithm: String, reason: String },

    /// Type conversion error
    #[error("Type conversion error: cannot convert {from} to {to}, value: {value}")]
    TypeConversion {
        from: String,
        to: String,
        value: String,
    },

    /// Version mismatch
    #[error("Version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },

    /// Timeout during I/O operation
    #[error("I/O timeout: operation {operation} timed out after {timeout_ms}ms")]
    Timeout { operation: String, timeout_ms: u64 },

    /// Resource temporarily unavailable
    #[error("Resource temporarily unavailable: {resource}")]
    ResourceUnavailable { resource: String },

    /// Permission denied
    #[error("Permission denied: {operation} on {resource}")]
    PermissionDenied { operation: String, resource: String },

    /// Resource already exists
    #[error("Resource already exists: {resource}")]
    ResourceExists { resource: String },

    /// Resource not found
    #[error("Resource not found: {resource}")]
    ResourceNotFound { resource: String },
}

impl IoError {
    /// Create a new I/O operation error
    pub fn operation<S: Into<String>>(operation: S, reason: S) -> Self {
        Self::Operation {
            operation: operation.into(),
            reason: reason.into(),
        }
    }

    /// Create a new serialization error
    pub fn serialization<S: Into<String>>(type_name: S, reason: S) -> Self {
        Self::Serialization {
            type_name: type_name.into(),
            reason: reason.into(),
        }
    }

    /// Create a new deserialization error
    pub fn deserialization<S: Into<String>>(expected: S, actual: usize, reason: S) -> Self {
        Self::Deserialization {
            expected: expected.into(),
            actual,
            reason: reason.into(),
        }
    }

    /// Create a new invalid format error
    pub fn invalid_format<S: Into<String>>(expected_format: S, reason: S) -> Self {
        Self::InvalidFormat {
            expected_format: expected_format.into(),
            reason: reason.into(),
        }
    }

    /// Create a new invalid data error
    pub fn invalid_data<S: Into<String>>(context: S, value: S) -> Self {
        Self::InvalidData {
            context: context.into(),
            value: value.into(),
        }
    }

    /// Create a new buffer overflow error
    pub fn buffer_overflow<S: Into<String>>(
        operation: S,
        attempted: usize,
        capacity: usize,
    ) -> Self {
        Self::BufferOverflow {
            operation: operation.into(),
            attempted,
            capacity,
        }
    }

    /// Create a new end of stream error
    pub fn end_of_stream<S: Into<String>>(expected: usize, context: S) -> Self {
        Self::EndOfStream {
            expected,
            context: context.into(),
        }
    }

    /// Create a new format exception error
    pub fn format_exception<S: Into<String>>(context: S, input: S) -> Self {
        Self::FormatException {
            context: context.into(),
            input: input.into(),
        }
    }

    /// Create a new invalid operation error
    pub fn invalid_operation<S: Into<String>>(operation: S, context: S) -> Self {
        Self::InvalidOperation {
            operation: operation.into(),
            context: context.into(),
        }
    }

    /// Create a new encoding error
    pub fn encoding<S: Into<String>>(encoding_type: S, reason: S) -> Self {
        Self::Encoding {
            encoding_type: encoding_type.into(),
            reason: reason.into(),
        }
    }

    /// Create a new memory allocation error
    pub fn memory_allocation<S: Into<String>>(size: usize, purpose: S) -> Self {
        Self::MemoryAllocation {
            size,
            purpose: purpose.into(),
        }
    }

    /// Create a new stream position error
    pub fn stream_position(position: u64, size: u64) -> Self {
        Self::StreamPosition { position, size }
    }

    /// Create a new checksum mismatch error
    pub fn checksum_mismatch<S: Into<String>>(expected: S, actual: S) -> Self {
        Self::ChecksumMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create a new type conversion error
    pub fn type_conversion<S: Into<String>>(from: S, to: S, value: S) -> Self {
        Self::TypeConversion {
            from: from.into(),
            to: to.into(),
            value: value.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(operation: S, timeout_ms: u64) -> Self {
        Self::Timeout {
            operation: operation.into(),
            timeout_ms,
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            IoError::Operation { .. }
                | IoError::Timeout { .. }
                | IoError::ResourceUnavailable { .. }
                | IoError::StreamPosition { .. }
        )
    }

    /// Check if this error is a user error (vs system error)
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            IoError::InvalidFormat { .. }
                | IoError::InvalidData { .. }
                | IoError::InvalidOperation { .. }
                | IoError::TypeConversion { .. }
                | IoError::VersionMismatch { .. }
                | IoError::ChecksumMismatch { .. }
        )
    }

    /// Check if this error is a recoverable I/O error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            IoError::Timeout { .. }
                | IoError::ResourceUnavailable { .. }
                | IoError::StreamNotReadable { .. }
                | IoError::StreamNotWritable { .. }
        )
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            IoError::InvalidData { .. }
            | IoError::InvalidFormat { .. }
            | IoError::TypeConversion { .. } => ErrorSeverity::Low,

            IoError::Serialization { .. }
            | IoError::Deserialization { .. }
            | IoError::Encoding { .. }
            | IoError::FormatException { .. } => ErrorSeverity::Medium,

            IoError::BufferOverflow { .. }
            | IoError::MemoryAllocation { .. }
            | IoError::ChecksumMismatch { .. }
            | IoError::Compression { .. } => ErrorSeverity::High,

            IoError::PermissionDenied { .. }
            | IoError::ResourceNotFound { .. }
            | IoError::InvalidOperation { .. } => ErrorSeverity::Critical,

            _ => ErrorSeverity::Medium,
        }
    }

    /// Get error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            IoError::Operation { .. } => "operation",
            IoError::Serialization { .. } | IoError::Deserialization { .. } => "serialization",
            IoError::InvalidFormat { .. } | IoError::InvalidData { .. } => "validation",
            IoError::BufferOverflow { .. } | IoError::EndOfStream { .. } => "buffer",
            IoError::FormatException { .. } => "format",
            IoError::InvalidOperation { .. } => "operation",
            IoError::Encoding { .. } => "encoding",
            IoError::MemoryAllocation { .. } => "memory",
            IoError::StreamPosition { .. }
            | IoError::StreamNotReadable { .. }
            | IoError::StreamNotWritable { .. } => "stream",
            IoError::ChecksumMismatch { .. } => "checksum",
            IoError::Compression { .. } => "compression",
            IoError::TypeConversion { .. } => "conversion",
            IoError::VersionMismatch { .. } => "version",
            IoError::Timeout { .. } => "timeout",
            IoError::ResourceUnavailable { .. }
            | IoError::ResourceExists { .. }
            | IoError::ResourceNotFound { .. } => "resource",
            IoError::PermissionDenied { .. } => "permission",
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity - minor issues that don't affect functionality
    Low,
    /// Medium severity - issues that may affect performance or specific features
    Medium,
    /// High severity - serious issues that significantly impact functionality
    High,
    /// Critical severity - issues that prevent normal operation
    Critical,
}

/// Result type for I/O operations
pub type IoResult<T> = std::result::Result<T, IoError>;

/// Alias for compatibility with existing code
pub type Result<T> = IoResult<T>;

// Standard library error conversions
impl From<std::io::Error> for IoError {
    fn from(error: std::io::Error) -> Self {
        let reason = error.to_string();
        match error.kind() {
            std::io::ErrorKind::UnexpectedEof => IoError::end_of_stream(0, "file"),
            std::io::ErrorKind::PermissionDenied => IoError::PermissionDenied {
                operation: "io".to_string(),
                resource: "file".to_string(),
            },
            std::io::ErrorKind::NotFound => IoError::ResourceNotFound {
                resource: "file".to_string(),
            },
            std::io::ErrorKind::AlreadyExists => IoError::ResourceExists {
                resource: "file".to_string(),
            },
            std::io::ErrorKind::TimedOut => IoError::timeout("io", 0),
            _ => IoError::operation("io", &reason),
        }
    }
}

impl From<std::fmt::Error> for IoError {
    fn from(error: std::fmt::Error) -> Self {
        IoError::serialization("format", &error.to_string())
    }
}

impl From<std::num::ParseIntError> for IoError {
    fn from(error: std::num::ParseIntError) -> Self {
        IoError::type_conversion("string", "integer", &error.to_string())
    }
}

impl From<std::num::ParseFloatError> for IoError {
    fn from(error: std::num::ParseFloatError) -> Self {
        IoError::type_conversion("string", "float", &error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for IoError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        IoError::encoding("utf8", &error.to_string())
    }
}

impl From<std::str::Utf8Error> for IoError {
    fn from(error: std::str::Utf8Error) -> Self {
        IoError::encoding("utf8", &error.to_string())
    }
}

impl From<std::array::TryFromSliceError> for IoError {
    fn from(error: std::array::TryFromSliceError) -> Self {
        IoError::type_conversion("slice", "array", &error.to_string())
    }
}

// Backward compatibility with old Error type
impl From<IoError> for crate::Error {
    fn from(error: IoError) -> Self {
        match error {
            IoError::Operation { reason, .. } => crate::Error::Io(reason),
            IoError::Serialization { reason, .. } => crate::Error::Serialization(reason),
            IoError::Deserialization { reason, .. } => crate::Error::Deserialization(reason),
            IoError::InvalidFormat { reason, .. } => crate::Error::InvalidFormat(reason),
            IoError::InvalidData { value, .. } => crate::Error::InvalidData(value),
            IoError::BufferOverflow { .. } => crate::Error::BufferOverflow,
            IoError::EndOfStream { .. } => crate::Error::EndOfStream,
            IoError::FormatException { .. } => crate::Error::FormatException,
            IoError::InvalidOperation { operation, .. } => {
                crate::Error::InvalidOperation(operation)
            }
            _ => crate::Error::Io(error.to_string()),
        }
    }
}

impl From<crate::Error> for IoError {
    fn from(error: crate::Error) -> Self {
        match error {
            crate::Error::Io(msg) => IoError::operation("io", &msg),
            crate::Error::Serialization(msg) => IoError::serialization("unknown", &msg),
            crate::Error::Deserialization(msg) => IoError::deserialization("unknown", 0, &msg),
            crate::Error::InvalidFormat(msg) => IoError::invalid_format("unknown", &msg),
            crate::Error::InvalidData(msg) => IoError::invalid_data("unknown", &msg),
            crate::Error::BufferOverflow => IoError::buffer_overflow("unknown", 0, 0),
            crate::Error::EndOfStream => IoError::end_of_stream(0, "stream"),
            crate::Error::FormatException => IoError::format_exception("unknown", "unknown"),
            crate::Error::InvalidOperation(msg) => {
                IoError::invalid_operation(msg, "unknown".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = IoError::serialization("MyType", "invalid field");
        assert!(matches!(error, IoError::Serialization { .. }));
        assert_eq!(
            error.to_string(),
            "Serialization failed: type MyType, reason: invalid field"
        );
    }

    #[test]
    fn test_error_classification() {
        assert!(IoError::timeout("test", 5000).is_retryable());
        assert!(!IoError::invalid_data("field", "value").is_retryable());

        assert!(IoError::invalid_format("json", "syntax error").is_user_error());
        assert!(!IoError::operation("read", "disk failure").is_user_error());

        assert!(IoError::timeout("write", 1000).is_recoverable());
        assert!(!IoError::buffer_overflow("write", 100, 50).is_recoverable());
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(
            IoError::invalid_data("field", "value").severity(),
            ErrorSeverity::Low
        );
        assert_eq!(
            IoError::serialization("Type", "error").severity(),
            ErrorSeverity::Medium
        );
        assert_eq!(
            IoError::buffer_overflow("write", 100, 50).severity(),
            ErrorSeverity::High
        );
        assert_eq!(
            IoError::PermissionDenied {
                operation: "read".to_string(),
                resource: "file".to_string()
            }
            .severity(),
            ErrorSeverity::Critical
        );
    }

    #[test]
    fn test_error_categories() {
        assert_eq!(IoError::operation("read", "error").category(), "operation");
        assert_eq!(
            IoError::serialization("Type", "error").category(),
            "serialization"
        );
        assert_eq!(
            IoError::buffer_overflow("write", 100, 50).category(),
            "buffer"
        );
    }

    #[test]
    fn test_specific_errors() {
        let error = IoError::buffer_overflow("write", 1000, 512);
        assert!(matches!(error, IoError::BufferOverflow { .. }));
        assert_eq!(
            error.to_string(),
            "Buffer overflow: attempted to write 1000 bytes, capacity 512"
        );

        let error = IoError::end_of_stream(10, "reading header");
        assert_eq!(
            error.to_string(),
            "Unexpected end of stream: expected 10 more bytes while reading reading header"
        );

        let error = IoError::checksum_mismatch("abc123", "def456");
        assert_eq!(
            error.to_string(),
            "Checksum mismatch: expected abc123, calculated def456"
        );
    }

    #[test]
    fn test_backward_compatibility() {
        let io_error = IoError::serialization("Test", "failed");
        let old_error: crate::Error = io_error.into();
        assert!(matches!(old_error, crate::Error::Serialization(_)));

        let old_error = crate::Error::EndOfStream;
        let io_error: IoError = old_error.into();
        assert!(matches!(io_error, IoError::EndOfStream { .. }));
    }

    #[test]
    fn test_from_std_errors() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = IoError::from(io_error);
        assert!(matches!(error, IoError::ResourceNotFound { .. }));

        let parse_error = "abc".parse::<i32>().unwrap_err();
        let error = IoError::from(parse_error);
        assert!(matches!(error, IoError::TypeConversion { .. }));
    }
}

use thiserror::Error;

/// JSON-related errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum JsonError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid cast: {0}")]
    InvalidCast(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Format error: {0}")]
    FormatError(String),

    #[error("Overflow error: {0}")]
    OverflowError(String),

    #[error("Not supported: {0}")]
    NotSupported(String),

    #[error("Unterminated array")]
    UnterminatedArray,

    #[error("Unterminated object")]
    UnterminatedObject,

    #[error("Duplicate property name: {0}")]
    DuplicateProperty(String),

    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
}

/// Result type for JSON operations
pub type JsonResult<T> = Result<T, JsonError>;

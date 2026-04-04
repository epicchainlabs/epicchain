use thiserror::Error;

/// MPT Trie related errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MptError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid node: {0}")]
    InvalidNode(String),

    #[error("Corrupted node: {0}")]
    CorruptedNode(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type for MPT Trie operations
pub type MptResult<T> = Result<T, MptError>;

/// Convert from CoreError to MptError
impl From<neo_core::CoreError> for MptError {
    fn from(error: neo_core::CoreError) -> Self {
        MptError::InvalidNode(format!("Core error: {}", error))
    }
}

/// Convert from std::io::Error to MptError
impl From<std::io::Error> for MptError {
    fn from(error: std::io::Error) -> Self {
        MptError::StorageError(format!("IO error: {}", error))
    }
}

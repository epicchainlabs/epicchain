//! Error types for Neo Extensions

use thiserror::Error;

/// Result type for extension operations
pub type ExtensionResult<T> = Result<T, ExtensionError>;

/// Extension framework errors
#[derive(Error, Debug)]
pub enum ExtensionError {
    /// Plugin already exists
    #[error("Plugin '{0}' already exists")]
    PluginAlreadyExists(String),

    /// Missing plugin dependency
    #[error("Plugin '{plugin}' requires dependency '{dependency}'")]
    MissingDependency { plugin: String, dependency: String },

    /// Plugin not found
    #[error("Plugin '{0}' not found")]
    PluginNotFound(String),

    /// Plugin not initialized
    #[error("Plugin system not initialized")]
    NotInitialized,

    /// Plugin initialization failed
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),

    /// Plugin operation failed
    #[error("Plugin operation failed: {0}")]
    OperationFailed(String),

    /// Invalid plugin configuration
    #[error("Invalid plugin configuration: {0}")]
    InvalidConfiguration(String),

    /// Encoding/decoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Hex decoding error
    #[error("Hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),

    /// Base64 decoding error
    #[error("Base64 decoding error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    /// Generic error
    #[error("Extension error: {0}")]
    Generic(String),
}

impl ExtensionError {
    /// Create a new generic error
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }

    /// Create a new encoding error
    pub fn encoding(msg: impl Into<String>) -> Self {
        Self::EncodingError(msg.into())
    }

    /// Create a new operation failed error
    pub fn operation_failed(msg: impl Into<String>) -> Self {
        Self::OperationFailed(msg.into())
    }

    /// Create a new invalid configuration error
    pub fn invalid_config(msg: impl Into<String>) -> Self {
        Self::InvalidConfiguration(msg.into())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ExtensionError::PluginAlreadyExists("test".to_string());
        assert!(error.to_string().contains("test"));

        let error = ExtensionError::MissingDependency {
            plugin: "plugin1".to_string(),
            dependency: "dep1".to_string(),
        };
        assert!(error.to_string().contains("plugin1"));
        assert!(error.to_string().contains("dep1"));
    }

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let ext_error: ExtensionError = io_error.into();
        assert!(matches!(ext_error, ExtensionError::IoError(_)));
    }
}

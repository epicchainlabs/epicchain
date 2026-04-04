//! Error types for Neo RPC Client
//!
//! This module provides comprehensive error handling for RPC operations,
//! matching the C# Neo.Network.RPC error handling exactly.

use crate::models::JsonRpcError;
use thiserror::Error;

/// Result type for RPC operations
pub type RpcResult<T> = Result<T, RpcError>;

/// Comprehensive RPC error types (matches C# RpcException hierarchy exactly)
#[derive(Error, Debug)]
pub enum RpcError {
    /// Network connection error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON-RPC protocol error
    #[error("RPC protocol error: {0}")]
    Protocol(JsonRpcError),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Request timeout error
    #[error("Request timeout after {timeout}s")]
    Timeout { timeout: u64 },

    /// Invalid response format
    #[error("Invalid response format: {message}")]
    InvalidResponse { message: String },

    /// Server returned an error
    #[error("Server error: {code} - {message}")]
    ServerError { code: i32, message: String },

    /// Authentication error
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Rate limiting error
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Method not found
    #[error("Method not found: {method}")]
    MethodNotFound { method: String },

    /// Internal client error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// URL parsing error
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error with custom message
    #[error("{0}")]
    Custom(String),

    /// Data parsing error
    #[error("Parse error: {0}")]
    Parse(String),
}

impl RpcError {
    /// Creates a new custom error
    pub fn custom<T: Into<String>>(message: T) -> Self {
        Self::Custom(message.into())
    }

    /// Creates a new invalid response error
    pub fn invalid_response<T: Into<String>>(message: T) -> Self {
        Self::InvalidResponse {
            message: message.into(),
        }
    }

    /// Creates a new server error
    pub fn server_error(code: i32, message: String) -> Self {
        Self::ServerError { code, message }
    }

    /// Creates a new timeout error
    pub fn timeout(timeout: u64) -> Self {
        Self::Timeout { timeout }
    }

    /// Creates a new method not found error
    pub fn method_not_found<T: Into<String>>(method: T) -> Self {
        Self::MethodNotFound {
            method: method.into(),
        }
    }

    /// Checks if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            RpcError::Network(_) => true,
            RpcError::Timeout { .. } => true,
            RpcError::ServerError { code, .. } => {
                // Retry on server errors that might be temporary
                *code >= 500 && *code < 600
            }
            RpcError::RateLimit(_) => true,
            _ => false,
        }
    }

    /// Gets the error code if available
    pub fn code(&self) -> Option<i32> {
        match self {
            RpcError::Protocol(rpc_error) => Some(rpc_error.code),
            RpcError::ServerError { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Checks if this is a network-related error
    pub fn is_network_error(&self) -> bool {
        matches!(self, RpcError::Network(_) | RpcError::Timeout { .. })
    }

    /// Checks if this is a protocol-related error
    pub fn is_protocol_error(&self) -> bool {
        matches!(
            self,
            RpcError::Protocol(_) | RpcError::InvalidResponse { .. }
        )
    }
}

impl From<JsonRpcError> for RpcError {
    fn from(error: JsonRpcError) -> Self {
        // Map common JSON-RPC error codes to specific error types
        match error.code {
            -32601 => RpcError::MethodNotFound {
                method: error.message.clone(),
            },
            -32602 => RpcError::InvalidParams(error.message),
            -32603 => RpcError::Internal(error.message),
            code if code >= -32099 && code <= -32000 => RpcError::ServerError {
                code,
                message: error.message,
            },
            _ => RpcError::Protocol(error),
        }
    }
}

/// Standard JSON-RPC error codes (matches JSON-RPC 2.0 specification exactly)
pub mod error_codes {
    /// Parse error - Invalid JSON was received by the server
    pub const PARSE_ERROR: i32 = -32700;

    /// Invalid Request - The JSON sent is not a valid Request object
    pub const INVALID_REQUEST: i32 = -32600;

    /// Method not found - The method does not exist / is not available
    pub const METHOD_NOT_FOUND: i32 = -32601;

    /// Invalid params - Invalid method parameter(s)
    pub const INVALID_PARAMS: i32 = -32602;

    /// Internal error - Internal JSON-RPC error
    pub const INTERNAL_ERROR: i32 = -32603;

    /// Server error range start
    pub const SERVER_ERROR_START: i32 = -32099;

    /// Server error range end
    pub const SERVER_ERROR_END: i32 = -32000;
}

/// Neo-specific RPC error codes (matches C# Neo RPC error codes exactly)
pub mod neo_error_codes {
    /// Unknown block
    pub const UNKNOWN_BLOCK: i32 = -100;

    /// Unknown transaction
    pub const UNKNOWN_TRANSACTION: i32 = -101;

    /// Unknown contract
    pub const UNKNOWN_CONTRACT: i32 = -102;

    /// Unknown storage item
    pub const UNKNOWN_STORAGE: i32 = -103;

    /// Insufficient funds
    pub const INSUFFICIENT_FUNDS: i32 = -300;

    /// Wallet not found
    pub const WALLET_NOT_FOUND: i32 = -400;

    /// Wallet not open
    pub const WALLET_NOT_OPEN: i32 = -401;

    /// Invalid address format
    pub const INVALID_ADDRESS: i32 = -402;

    /// Invalid transaction format
    pub const INVALID_TRANSACTION: i32 = -500;

    /// Transaction verification failed
    pub const VERIFICATION_FAILED: i32 = -501;

    /// Transaction already exists
    pub const TRANSACTION_EXISTS: i32 = -502;

    /// Memory pool full
    pub const MEMPOOL_FULL: i32 = -503;

    /// Policy check failed
    pub const POLICY_FAILED: i32 = -504;

    /// Invalid script
    pub const INVALID_SCRIPT: i32 = -600;

    /// Script execution failed
    pub const EXECUTION_FAILED: i32 = -601;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{RpcError, RpcResult};
    use crate::models::JsonRpcError;

    #[test]
    fn test_error_creation() {
        let error = RpcError::custom("Test error");
        assert_eq!(error.to_string(), "Test error");
    }

    #[test]
    fn test_error_retryable() {
        // Test timeout error
        let timeout_error = RpcError::timeout(30);
        assert!(timeout_error.is_retryable());

        // Test server errors
        let server_error = RpcError::server_error(500, "Internal Server Error".to_string());
        assert!(server_error.is_retryable());

        let client_error = RpcError::server_error(400, "Bad Request".to_string());
        assert!(!client_error.is_retryable());

        // Test rate limit error
        let rate_limit_error = RpcError::RateLimit("Too many requests".to_string());
        assert!(rate_limit_error.is_retryable());

        // Test non-retryable errors
        let config_error = RpcError::Config("Invalid config".to_string());
        assert!(!config_error.is_retryable());
    }

    #[test]
    fn test_json_rpc_error_conversion() {
        let json_error = JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        };

        let rpc_error: RpcError = json_error.into();
        match rpc_error {
            RpcError::MethodNotFound { method } => {
                assert_eq!(method, "Method not found");
            }
            _ => panic!("Expected MethodNotFound error"),
        }
    }

    #[test]
    fn test_error_classification() {
        // Test timeout error as network error
        let timeout_error = RpcError::timeout(30);
        assert!(timeout_error.is_network_error());
        assert!(!timeout_error.is_protocol_error());

        // Test protocol error
        let protocol_error = RpcError::invalid_response("Invalid JSON");
        assert!(!protocol_error.is_network_error());
        assert!(protocol_error.is_protocol_error());

        // Test JSON-RPC protocol error
        let json_rpc_error = RpcError::Protocol(JsonRpcError {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        });
        assert!(!json_rpc_error.is_network_error());
        assert!(json_rpc_error.is_protocol_error());
    }
}

//! Neo Smart Contract Engine
//!
//! This crate provides the smart contract execution engine for the Neo blockchain,
//! including contract deployment, invocation, and native contract implementations.

pub use neo_core::{
    IVerifiable, Signer, Transaction, TransactionAttributeType, UInt160, UInt256, Witness,
    WitnessCondition, WitnessScope,
};
pub use neo_cryptography::ECPoint;
pub use neo_vm::{ApplicationEngine, TriggerType};

// Re-export smart contract types
pub use contract_state::{ContractState, MethodToken, NefFile};
pub use manifest::{
    ContractAbi, ContractEvent as ContractEventDescriptor, ContractGroup, ContractManifest,
    ContractMethod as ContractMethodDescriptor, ContractParameter as ContractParameterDefinition,
    ContractParameterType, ContractPermission, ContractPermissionDescriptor as PermissionContract,
};
pub use storage::{StorageItem, StorageKey};
// Note: Types are accessed through proper crate boundaries to maintain clean architecture

pub mod application_engine;
pub mod complete_application_engine;
pub mod contract_state;
pub mod deployment;
pub mod events;
pub mod interop;
pub mod manifest;
pub mod native;
pub mod performance;
pub mod storage;
pub mod validation;

use thiserror::Error;

/// Smart contract error types
#[derive(Debug, Error)]
pub enum Error {
    #[error("VM error: {0}")]
    VM(#[from] neo_vm::VmError),
    #[error("Core error: {0}")]
    Core(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Invalid contract: {0}")]
    InvalidContract(String),
    #[error("Invalid witness: {0}")]
    InvalidWitness(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Gas limit exceeded")]
    GasLimitExceeded,
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Storage not found: {0}")]
    StorageNotFound(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Event error: {0}")]
    Event(#[from] EventError),
    #[error("Native contract error: {0}")]
    NativeContractError(String),
    #[error("Execution halted: {0}")]
    ExecutionHalted(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("VM execution error: {0}")]
    VmError(String),
    #[error("Contract not found: {0}")]
    ContractNotFound(String),
    #[error("Insufficient gas: {0}")]
    InsufficientGas(String),
    #[error("Interop service error: {0}")]
    InteropServiceError(String),
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    #[error("Neo token error: {0}")]
    NeoToken(#[from] NeoTokenError),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Serialization: {0}")]
    Serialization(String),
    #[error("Deserialization: {0}")]
    Deserialization(String),
}

impl From<neo_io::IoError> for Error {
    fn from(err: neo_io::IoError) -> Self {
        match err {
            neo_io::IoError::EndOfStream { .. } => {
                Error::SerializationError("Unexpected end of stream".to_string())
            }
            neo_io::IoError::InvalidData { context, value } => {
                Error::SerializationError(format!("{}: {}", context, value))
            }
            neo_io::IoError::FormatException { context, .. } => {
                Error::SerializationError(format!("Format exception: {}", context))
            }
            neo_io::IoError::Deserialization {
                expected, reason, ..
            } => Error::SerializationError(format!(
                "Deserialization failed: expected {}, reason: {}",
                expected, reason
            )),
            neo_io::IoError::InvalidOperation { operation, context } => Error::InvalidOperation(
                format!("Invalid operation: {} in context: {}", operation, context),
            ),
            neo_io::IoError::Operation { operation, reason } => Error::IO(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}: {}", operation, reason),
            )),
            neo_io::IoError::Serialization { type_name, reason } => {
                Error::SerializationError(format!("{}: {}", type_name, reason))
            }
            neo_io::IoError::InvalidFormat {
                expected_format,
                reason,
            } => Error::SerializationError(format!(
                "Invalid format: expected {}, reason: {}",
                expected_format, reason
            )),
            neo_io::IoError::BufferOverflow { .. } => {
                Error::SerializationError("Buffer overflow".to_string())
            }
            neo_io::IoError::Encoding { reason, .. } => {
                Error::SerializationError(format!("Encoding error: {}", reason))
            }
            // Handle remaining variants with appropriate mappings
            neo_io::IoError::MemoryAllocation { .. } => {
                Error::SerializationError("Memory allocation failed".to_string())
            }
            neo_io::IoError::StreamPosition { .. } => {
                Error::SerializationError("Stream position error".to_string())
            }
            neo_io::IoError::StreamNotReadable { reason } => {
                Error::SerializationError(format!("Stream not readable: {}", reason))
            }
            neo_io::IoError::StreamNotWritable { reason } => {
                Error::SerializationError(format!("Stream not writable: {}", reason))
            }
            neo_io::IoError::ChecksumMismatch { .. } => {
                Error::SerializationError("Checksum mismatch".to_string())
            }
            neo_io::IoError::Compression { algorithm, reason } => {
                Error::SerializationError(format!("Compression error ({}): {}", algorithm, reason))
            }
            neo_io::IoError::TypeConversion { from, to, .. } => {
                Error::SerializationError(format!("Type conversion from {} to {} failed", from, to))
            }
            neo_io::IoError::VersionMismatch { expected, actual } => Error::SerializationError(
                format!("Version mismatch: expected {}, got {}", expected, actual),
            ),
            neo_io::IoError::Timeout { .. } => {
                Error::InvalidOperation("Operation timed out".to_string())
            }
            neo_io::IoError::ResourceUnavailable { resource } => {
                Error::InvalidOperation(format!("Resource unavailable: {}", resource))
            }
            neo_io::IoError::PermissionDenied {
                operation,
                resource,
            } => {
                Error::PermissionDenied(format!("Permission denied: {} on {}", operation, resource))
            }
            neo_io::IoError::ResourceExists { resource } => {
                Error::InvalidOperation(format!("Resource already exists: {}", resource))
            }
            neo_io::IoError::ResourceNotFound { resource } => {
                Error::InvalidOperation(format!("Resource not found: {}", resource))
            }
        }
    }
}

impl From<neo_core::CoreError> for Error {
    fn from(err: neo_core::CoreError) -> Self {
        match err {
            neo_core::CoreError::InvalidFormat { message } => Error::SerializationError(message),
            neo_core::CoreError::InvalidData { message } => Error::SerializationError(message),
            neo_core::CoreError::Io { message } => Error::SerializationError(message),
            neo_core::CoreError::Serialization { message } => Error::SerializationError(message),
            neo_core::CoreError::InvalidOperation { message } => Error::InvalidOperation(message),
            neo_core::CoreError::System { message } => Error::Core(message),
            neo_core::CoreError::InsufficientGas {
                required: _,
                available: _,
            } => Error::InsufficientGas("Insufficient gas".to_string()),
            neo_core::CoreError::Cryptographic { message } => Error::InvalidSignature(message),
            neo_core::CoreError::Deserialization { message } => Error::SerializationError(message),
            neo_core::CoreError::BufferOverflow { .. } => {
                Error::SerializationError("Buffer overflow".to_string())
            }
            neo_core::CoreError::EndOfStream => {
                Error::SerializationError("End of stream".to_string())
            }
            neo_core::CoreError::Configuration { message } => Error::InvalidOperation(message),
            neo_core::CoreError::Network { message } => Error::InvalidOperation(message),
            neo_core::CoreError::Timeout { .. } => Error::InvalidOperation("Timeout".to_string()),
            neo_core::CoreError::NotFound { resource } => {
                Error::InvalidOperation(format!("Not found: {}", resource))
            }
            neo_core::CoreError::Validation { message } => Error::InvalidOperation(message),
            neo_core::CoreError::AlreadyExists { resource } => {
                Error::InvalidOperation(format!("Already exists: {}", resource))
            }
            neo_core::CoreError::ValidationFailed { reason } => Error::InvalidOperation(reason),
            neo_core::CoreError::TypeConversion { from, to } => {
                Error::SerializationError(format!("Type conversion from {} to {} failed", from, to))
            }
        }
    }
}

/// Event-specific errors
#[derive(Debug, Error)]
pub enum EventError {
    #[error("Invalid delay: {0}")]
    InvalidDelay(String),
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// NeoToken-specific errors  
#[derive(Debug, Error)]
pub enum NeoTokenError {
    #[error("Invalid candidate: {0}")]
    InvalidCandidate(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

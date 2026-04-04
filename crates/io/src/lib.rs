//! IO operations and data structures for the Neo blockchain.
//!
//! This crate provides IO functionality for the Neo blockchain, including binary serialization,
//! caching, and other IO-related utilities.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod binary_reader;
pub mod binary_writer;
pub mod caching;
pub mod error;
pub mod memory_reader;
pub mod serializable;

#[cfg(feature = "async")]
pub mod actors;

pub use binary_reader::BinaryReader;
pub use binary_writer::BinaryWriter;
pub use error::{ErrorSeverity, IoError, IoResult, Result};
pub use memory_reader::MemoryReader;
pub use serializable::{helper, Serializable, SerializableExt};

/// Legacy error type for backward compatibility
///
/// **Deprecated**: Use [`IoError`] instead for new code.
#[deprecated(since = "0.3.0", note = "Use IoError instead")]
pub use LegacyError as Error;

/// Legacy I/O errors for backward compatibility
#[derive(Debug, thiserror::Error)]
pub enum LegacyError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Buffer overflow")]
    BufferOverflow,

    #[error("End of stream")]
    EndOfStream,

    #[error("Format exception")]
    FormatException,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

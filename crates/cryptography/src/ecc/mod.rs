//! Elliptic Curve Cryptography implementation for Neo.
//!
//! This module provides the necessary types and functions for elliptic curve
//! cryptography used in the Neo blockchain.

mod curve;
mod field_element;
mod point;

pub use curve::ECCurve;
pub use field_element::ECFieldElement;
pub use point::ECPoint;

use neo_config::HASH_SIZE;

/// Common error types for ECC operations
#[derive(Debug, thiserror::Error)]
pub enum ECCError {
    #[error("Invalid point format")]
    InvalidPointFormat,

    #[error("Point not on curve")]
    PointNotOnCurve,

    #[error("Invalid field element")]
    InvalidFieldElement,

    #[error("Invalid curve parameters")]
    InvalidCurveParameters,

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

/// Result type for ECC operations
pub type ECCResult<T> = Result<T, ECCError>;

/// ECC utility functions for wallet compatibility.
pub struct ECC;

impl ECC {
    /// Generates a public key from a private key.
    pub fn generate_public_key(private_key: &[u8; HASH_SIZE]) -> crate::Result<Vec<u8>> {
        crate::ecdsa::ECDsa::derive_public_key(private_key)
    }

    /// Compresses a public key.
    pub fn compress_public_key(public_key: &[u8]) -> crate::Result<Vec<u8>> {
        crate::ecdsa::ECDsa::compress_public_key(public_key)
    }

    /// Decompresses a public key.
    pub fn decompress_public_key(compressed_key: &[u8]) -> crate::Result<Vec<u8>> {
        crate::ecdsa::ECDsa::decompress_public_key(compressed_key)
    }

    /// Validates a private key.
    pub fn validate_private_key(private_key: &[u8; HASH_SIZE]) -> bool {
        crate::ecdsa::ECDsa::validate_private_key(private_key)
    }

    /// Validates a public key.
    pub fn validate_public_key(public_key: &[u8]) -> bool {
        crate::ecdsa::ECDsa::validate_public_key(public_key)
    }
}

/// Standalone functions for compatibility with wallet module.
pub fn generate_public_key(private_key: &[u8; HASH_SIZE]) -> crate::Result<Vec<u8>> {
    ECC::generate_public_key(private_key)
}

pub fn compress_public_key(public_key: &[u8]) -> crate::Result<Vec<u8>> {
    ECC::compress_public_key(public_key)
}

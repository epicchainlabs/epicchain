//! Error types for BLS12-381 operations.

use thiserror::Error;

/// Result type for BLS operations
pub type BlsResult<T> = Result<T, BlsError>;

/// Error types for BLS12-381 operations (matches C# Neo.Cryptography.BLS12_381 errors)
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BlsError {
    /// Invalid private key
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Invalid public key
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    /// Invalid signature
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    /// Invalid key size
    #[error("Invalid key size: expected {expected}, got {actual}")]
    InvalidKeySize { expected: usize, actual: usize },

    /// Invalid signature size
    #[error("Invalid signature size: expected {expected}, got {actual}")]
    InvalidSignatureSize { expected: usize, actual: usize },

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Aggregation error
    #[error("Aggregation error: {0}")]
    AggregationError(String),

    /// Verification failed
    #[error("Signature verification failed")]
    VerificationFailed,

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Cryptographic error
    #[error("Cryptographic error: {0}")]
    CryptographicError(String),

    /// Point not on curve
    #[error("Point is not on the curve")]
    PointNotOnCurve,

    /// Point at infinity
    #[error("Point is at infinity")]
    PointAtInfinity,

    /// Invalid scalar
    #[error("Invalid scalar value")]
    InvalidScalar,

    /// Empty input
    #[error("Empty input provided")]
    EmptyInput,

    /// Insufficient data
    #[error("Insufficient data: need at least {needed} bytes, got {actual}")]
    InsufficientData { needed: usize, actual: usize },

    /// Hash-to-curve error
    #[error("Hash-to-curve operation failed: {0}")]
    HashToCurveError(String),

    /// Domain separation tag error
    #[error("Invalid domain separation tag: {0}")]
    InvalidDst(String),

    /// Invalid signature scheme
    #[error("Invalid signature scheme")]
    InvalidSignatureScheme,

    /// Batch verification size limit exceeded
    #[error("Batch size too large for verification")]
    BatchTooLarge,
}

impl BlsError {
    /// Creates an invalid private key error
    pub fn invalid_private_key<S: Into<String>>(msg: S) -> Self {
        Self::InvalidPrivateKey(msg.into())
    }

    /// Creates an invalid public key error
    pub fn invalid_public_key<S: Into<String>>(msg: S) -> Self {
        Self::InvalidPublicKey(msg.into())
    }

    /// Creates an invalid signature error
    pub fn invalid_signature<S: Into<String>>(msg: S) -> Self {
        Self::InvalidSignature(msg.into())
    }

    /// Creates a serialization error
    pub fn serialization_error<S: Into<String>>(msg: S) -> Self {
        Self::SerializationError(msg.into())
    }

    /// Creates a deserialization error
    pub fn deserialization_error<S: Into<String>>(msg: S) -> Self {
        Self::DeserializationError(msg.into())
    }

    /// Creates an aggregation error
    pub fn aggregation_error<S: Into<String>>(msg: S) -> Self {
        Self::AggregationError(msg.into())
    }

    /// Creates an invalid input error
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Creates a cryptographic error
    pub fn cryptographic_error<S: Into<String>>(msg: S) -> Self {
        Self::CryptographicError(msg.into())
    }

    /// Creates a hash-to-curve error
    pub fn hash_to_curve_error<S: Into<String>>(msg: S) -> Self {
        Self::HashToCurveError(msg.into())
    }

    /// Creates an invalid DST error
    pub fn invalid_dst<S: Into<String>>(msg: S) -> Self {
        Self::InvalidDst(msg.into())
    }
}

/// Converts from hex decoding errors
impl From<hex::FromHexError> for BlsError {
    fn from(err: hex::FromHexError) -> Self {
        Self::DeserializationError(format!("Hex decoding error: {err}"))
    }
}

/// Converts from group errors (if the bls12_381 crate used them)
impl From<bls12_381::Scalar> for BlsError {
    fn from(_: bls12_381::Scalar) -> Self {
        Self::InvalidScalar
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = BlsError::invalid_private_key("test error");
        assert_eq!(err.to_string(), "Invalid private key: test error");

        let err = BlsError::InvalidKeySize {
            expected: 32,
            actual: 16,
        };
        assert_eq!(err.to_string(), "Invalid key size: expected 32, got 16");
    }

    #[test]
    fn test_error_conversion() {
        let hex_err = hex::decode("invalid_hex").unwrap_err();
        let bls_err: BlsError = hex_err.into();
        assert!(matches!(bls_err, BlsError::DeserializationError(_)));
    }

    #[test]
    fn test_error_equality() {
        let err1 = BlsError::VerificationFailed;
        let err2 = BlsError::VerificationFailed;
        assert_eq!(err1, err2);

        let err3 = BlsError::InvalidPrivateKey("test".to_string());
        let err4 = BlsError::InvalidPrivateKey("test".to_string());
        assert_eq!(err3, err4);
    }
}

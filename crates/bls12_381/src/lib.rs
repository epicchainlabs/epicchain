//! BLS12-381 cryptographic operations for Neo blockchain.
//!
//! This module provides BLS (Boneh-Lynn-Shacham) signature operations using the BLS12-381 curve,
//! exactly matching the C# Neo.Cryptography.BLS12_381 implementation.
//!
//! BLS signatures are used in Neo N3 for consensus operations, allowing signature aggregation
//! for efficient multi-signature schemes.

pub mod aggregation;
pub mod batch;
pub mod constants;
pub mod error;
pub mod keys;
pub mod signature;
pub mod utils;

pub use aggregation::{AggregatePublicKey, AggregateSignature};
pub use batch::BatchVerifier;
pub use error::{BlsError, BlsResult};
pub use keys::{KeyPair, PrivateKey, PublicKey};
pub use signature::{Signature, SignatureScheme};

use rand::RngCore;

/// BLS12-381 domain separation tag for Neo blockchain (matches C# exactly)
pub const NEO_BLS_DST: &[u8] = b"NEO_BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_";

/// Minimum signature scheme for Neo consensus (matches C# Neo.Cryptography.BLS12_381)
pub const NEO_SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::Basic;

/// BLS12-381 implementation for Neo blockchain
/// This matches the C# Neo.Cryptography.BLS12_381 class exactly
pub struct Bls12381;

impl Bls12381 {
    /// Generates a new random private key (matches C# GeneratePrivateKey)
    pub fn generate_private_key<R: RngCore>(rng: &mut R) -> PrivateKey {
        PrivateKey::generate(rng)
    }

    /// Derives a public key from a private key (matches C# GetPublicKey)
    pub fn derive_public_key(private_key: &PrivateKey) -> PublicKey {
        private_key.public_key()
    }

    /// Signs a message with a private key (matches C# Sign)
    pub fn sign(private_key: &PrivateKey, message: &[u8]) -> BlsResult<Signature> {
        private_key.sign(message, NEO_SIGNATURE_SCHEME)
    }

    /// Verifies a signature (matches C# Verify)
    pub fn verify(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
        public_key.verify(message, signature, NEO_SIGNATURE_SCHEME)
    }

    /// Aggregates multiple signatures (matches C# AggregateSignatures)
    pub fn aggregate_signatures(signatures: &[Signature]) -> BlsResult<AggregateSignature> {
        AggregateSignature::aggregate(signatures)
    }

    /// Aggregates multiple public keys (matches C# AggregatePublicKeys)
    pub fn aggregate_public_keys(public_keys: &[PublicKey]) -> BlsResult<AggregatePublicKey> {
        AggregatePublicKey::aggregate(public_keys)
    }

    /// Fast aggregate verification for same message (matches C# FastAggregateVerify)
    pub fn fast_aggregate_verify(
        public_keys: &[PublicKey],
        message: &[u8],
        aggregate_signature: &AggregateSignature,
    ) -> bool {
        // Aggregate public keys
        if let Ok(aggregate_pk) = Self::aggregate_public_keys(public_keys) {
            // Verify with single message
            aggregate_pk.verify_single_message(message, aggregate_signature, NEO_SIGNATURE_SCHEME)
        } else {
            false
        }
    }

    /// Validates a private key (matches C# ValidatePrivateKey)
    pub fn validate_private_key(private_key: &PrivateKey) -> bool {
        private_key.is_valid()
    }

    /// Validates a public key (matches C# ValidatePublicKey)
    pub fn validate_public_key(public_key: &PublicKey) -> bool {
        public_key.is_valid()
    }

    /// Validates a signature (matches C# ValidateSignature)
    pub fn validate_signature(signature: &Signature) -> bool {
        signature.is_valid()
    }

    /// Serializes a private key to bytes (matches C# PrivateKeyToBytes)
    pub fn private_key_to_bytes(private_key: &PrivateKey) -> Vec<u8> {
        private_key.to_bytes()
    }

    /// Deserializes a private key from bytes (matches C# PrivateKeyFromBytes)
    pub fn private_key_from_bytes(bytes: &[u8]) -> BlsResult<PrivateKey> {
        PrivateKey::from_bytes(bytes)
    }

    /// Serializes a public key to bytes (matches C# PublicKeyToBytes)
    pub fn public_key_to_bytes(public_key: &PublicKey) -> Vec<u8> {
        public_key.to_bytes()
    }

    /// Deserializes a public key from bytes (matches C# PublicKeyFromBytes)
    pub fn public_key_from_bytes(bytes: &[u8]) -> BlsResult<PublicKey> {
        PublicKey::from_bytes(bytes)
    }

    /// Serializes a signature to bytes (matches C# SignatureToBytes)
    pub fn signature_to_bytes(signature: &Signature) -> Vec<u8> {
        signature.to_bytes()
    }

    /// Deserialize signature from bytes
    /// Matches C# SignatureFromBytes(byte[]) method
    pub fn signature_from_bytes(bytes: &[u8]) -> BlsResult<Signature> {
        Signature::from_bytes(bytes)
    }

    /// Generates a key pair (matches C# GenerateKeyPair)
    pub fn generate_key_pair<R: RngCore>(rng: &mut R) -> KeyPair {
        KeyPair::generate(rng)
    }

    /// Creates a key pair from a private key (matches C# KeyPairFromPrivateKey)
    pub fn key_pair_from_private_key(private_key: PrivateKey) -> KeyPair {
        KeyPair::from_private_key(private_key)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::constants::HASH_SIZE;
    use rand::thread_rng;

    #[test]
    fn test_key_generation() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        assert!(Bls12381::validate_private_key(&private_key));
        assert!(Bls12381::validate_public_key(&public_key));
    }

    #[test]
    fn test_sign_and_verify() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Hello, Neo!";

        let signature = Bls12381::sign(&private_key, message).unwrap();
        assert!(Bls12381::verify(&public_key, message, &signature));

        // Test with different message
        let wrong_message = b"Wrong message";
        assert!(!Bls12381::verify(&public_key, wrong_message, &signature));
    }

    #[test]
    fn test_serialization() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Serialize me!";
        let signature = Bls12381::sign(&private_key, message).unwrap();

        // Test private key serialization
        let private_key_bytes = Bls12381::private_key_to_bytes(&private_key);
        let deserialized_private_key =
            Bls12381::private_key_from_bytes(&private_key_bytes).unwrap();
        assert_eq!(private_key, deserialized_private_key);

        // Test public key serialization
        let public_key_bytes = Bls12381::public_key_to_bytes(&public_key);
        let deserialized_public_key = Bls12381::public_key_from_bytes(&public_key_bytes).unwrap();
        assert_eq!(public_key, deserialized_public_key);

        // Test signature serialization
        let signature_bytes = Bls12381::signature_to_bytes(&signature);
        let deserialized_signature =
            Bls12381::signature_from_bytes(&signature_bytes).expect("Operation failed");
        assert_eq!(signature, deserialized_signature);
    }

    #[test]
    fn test_constants() {
        assert_eq!(constants::PRIVATE_KEY_SIZE, HASH_SIZE);
        assert_eq!(constants::PUBLIC_KEY_SIZE, 48);
        assert_eq!(constants::SIGNATURE_SIZE, 96);
    }
}

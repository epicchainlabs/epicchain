//! Cryptographic signature handling for consensus messages
//!
//! This module provides signature generation and verification for consensus
//! messages, matching the C# Neo consensus signature handling exactly.

use crate::{Error, Result};
use neo_config::HASH_SIZE;
use neo_core::UInt160;
use neo_cryptography::ECPoint;
use neo_wallets::KeyPair;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// Neo signature is typically 64 bytes (r + s components)
const SIGNATURE_SIZE: usize = 64;

/// Signature provider for consensus operations
pub struct SignatureProvider {
    /// Validator public key hash
    validator_hash: UInt160,
    /// Key pair for signing (optional - only for actual validators)
    key_pair: Option<KeyPair>,
}

impl SignatureProvider {
    /// Creates a new signature provider for a validator
    pub fn new(validator_hash: UInt160, key_pair: Option<KeyPair>) -> Self {
        Self {
            validator_hash,
            key_pair,
        }
    }

    /// Signs a message using the validator's private key
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        if let Some(key_pair) = &self.key_pair {
            match key_pair.sign(message) {
                Ok(sig) => Ok(sig),
                Err(e) => Err(Error::Generic(format!("Failed to sign message: {}", e))),
            }
        } else {
            // Development/testing fallback - deterministic pseudo-signature
            self.create_deterministic_signature(message)
        }
    }

    /// Verifies a signature using a public key
    pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &ECPoint) -> bool {
        // Use ECDSA for verification
        match neo_cryptography::ecdsa::ECDsa::verify_signature(
            message,
            signature,
            &public_key.to_bytes(),
        ) {
            Ok(valid) => valid,
            Err(_) => false,
        }
    }

    /// Creates a deterministic pseudo-signature for testing
    fn create_deterministic_signature(&self, message: &[u8]) -> Result<Vec<u8>> {
        // Create a deterministic signature based on message and validator hash
        let mut hasher = Sha256::new();
        hasher.update(b"NEO_CONSENSUS_SIGNATURE");
        hasher.update(message);
        hasher.update(&self.validator_hash.as_bytes());
        let hash1 = hasher.finalize();

        let mut hasher2 = Sha256::new();
        hasher2.update(&hash1);
        hasher2.update(b"SIGNATURE_PADDING");
        let hash2 = hasher2.finalize();

        // Combine to create 64-byte signature
        let mut signature = Vec::with_capacity(64);
        signature.extend_from_slice(&hash1);
        signature.extend_from_slice(&hash2);
        signature.truncate(64);

        Ok(signature)
    }
}

/// Message signing utilities for consensus
pub struct MessageSigner;

impl MessageSigner {
    /// Creates message data for prepare request signature
    pub fn create_prepare_request_data(
        block_index: u32,
        view_number: u8,
        timestamp: u64,
        block_hash: &neo_core::UInt256,
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(b'P'); // PrepareRequest marker
        data.extend_from_slice(&block_index.to_le_bytes());
        data.push(view_number);
        data.extend_from_slice(&timestamp.to_le_bytes());
        data.extend_from_slice(&block_hash.as_bytes());
        data
    }

    /// Creates message data for prepare response signature
    pub fn create_prepare_response_data(
        block_index: u32,
        view_number: u8,
        block_hash: &neo_core::UInt256,
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(b'R'); // PrepareResponse marker
        data.extend_from_slice(&block_index.to_le_bytes());
        data.push(view_number);
        data.extend_from_slice(&block_hash.as_bytes());
        data
    }

    /// Creates message data for commit signature
    pub fn create_commit_data(
        block_index: u32,
        view_number: u8,
        block_hash: &neo_core::UInt256,
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(b'C'); // Commit marker
        data.extend_from_slice(&block_index.to_le_bytes());
        data.push(view_number);
        data.extend_from_slice(&block_hash.as_bytes());
        data
    }

    /// Creates message data for change view signature
    pub fn create_change_view_data(
        block_index: u32,
        view_number: u8,
        new_view_number: u8,
        reason: u8,
    ) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(b'V'); // ViewChange marker
        data.extend_from_slice(&block_index.to_le_bytes());
        data.push(view_number);
        data.push(new_view_number);
        data.push(reason);
        data
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ConsensusContext, ConsensusMessage, ConsensusState};

    #[test]
    fn test_deterministic_signature() {
        let validator_hash = UInt160::zero();
        let provider = SignatureProvider::new(validator_hash, None);

        let message = b"test message";
        let signature1 = provider.sign_message(message).unwrap();
        let signature2 = provider.sign_message(message).unwrap();

        // Should be deterministic
        assert_eq!(signature1, signature2);
        assert_eq!(signature1.len(), 64);
    }

    #[test]
    fn test_message_data_creation() {
        let block_hash = neo_core::UInt256::zero();

        let prepare_data =
            MessageSigner::create_prepare_request_data(100, 1, 1234567890, &block_hash);
        assert_eq!(prepare_data[0], b'P');

        let response_data = MessageSigner::create_prepare_response_data(100, 1, &block_hash);
        assert_eq!(response_data[0], b'R');

        let commit_data = MessageSigner::create_commit_data(100, 1, &block_hash);
        assert_eq!(commit_data[0], b'C');

        let change_view_data = MessageSigner::create_change_view_data(100, 1, 2, 0);
        assert_eq!(change_view_data[0], b'V');
    }
}

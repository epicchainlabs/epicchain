//! Batch verification for BLS12-381 signatures
//!
//! This module provides efficient batch verification of multiple BLS signatures,
//! which is essential for consensus operations in Neo blockchain.

use crate::constants::{BATCH_VERIFICATION_THRESHOLD, MAX_AGGREGATE_SIZE};
use crate::error::{BlsError, BlsResult};
use crate::keys::PublicKey;
use crate::signature::{Signature, SignatureScheme};
use crate::utils;
use bls12_381::{pairing, G1Affine, G2Affine, G2Projective};
use group::Curve;
use rand::{thread_rng, Rng};

/// Batch verifier for efficient verification of multiple BLS signatures
/// This provides significant performance improvements when verifying many signatures
pub struct BatchVerifier {
    items: Vec<BatchItem>,
    scheme: Option<SignatureScheme>,
}

/// Internal structure for batch verification items
struct BatchItem {
    message: Vec<u8>,
    signature: G2Affine,
    public_key: G1Affine,
    scheme: SignatureScheme,
}

impl BatchVerifier {
    /// Create a new batch verifier
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            scheme: None,
        }
    }

    /// Create a new batch verifier with a specific scheme
    /// All signatures added must use the same scheme
    pub fn new_with_scheme(scheme: SignatureScheme) -> Self {
        Self {
            items: Vec::new(),
            scheme: Some(scheme),
        }
    }

    /// Add a signature to the batch for verification
    pub fn add(
        &mut self,
        message: &[u8],
        signature: &Signature,
        public_key: &PublicKey,
        scheme: SignatureScheme,
    ) -> BlsResult<()> {
        if let Some(expected_scheme) = self.scheme {
            if scheme != expected_scheme {
                return Err(BlsError::InvalidSignatureScheme);
            }
        }

        // Check batch size limit
        if self.items.len() >= MAX_AGGREGATE_SIZE {
            return Err(BlsError::BatchTooLarge);
        }

        let signature_affine = signature.point().to_affine();
        let public_key_affine = public_key.point();

        self.items.push(BatchItem {
            message: message.to_vec(),
            signature: signature_affine,
            public_key: public_key_affine,
            scheme,
        });

        Ok(())
    }

    /// Verify all signatures in the batch
    /// Returns true if all signatures are valid, false otherwise
    pub fn verify(&self) -> bool {
        if self.items.is_empty() {
            return true;
        }

        if self.items.len() < BATCH_VERIFICATION_THRESHOLD {
            return self.verify_individually();
        }

        self.verify_batch_randomized()
    }

    /// Verify signatures individually (used for small batches)
    fn verify_individually(&self) -> bool {
        for item in &self.items {
            let signature = Signature::from_affine(item.signature);
            let public_key = PublicKey::from_affine(item.public_key);

            if !signature.verify(&public_key, &item.message, item.scheme) {
                return false;
            }
        }
        true
    }

    /// Verify signatures using randomized batch verification
    /// This is more efficient for large batches but requires randomization for security
    fn verify_batch_randomized(&self) -> bool {
        let mut rng = thread_rng();

        let coefficients: Vec<u64> = (0..self.items.len())
            .map(|_| rng.gen_range(1..=u64::MAX))
            .collect();

        // Compute the left side of the pairing equation
        let mut left_g1 = G1Affine::identity();
        let mut left_g2 = G2Projective::identity();

        for (item, &coeff) in self.items.iter().zip(coefficients.iter()) {
            let prepared_message = match item.scheme {
                SignatureScheme::Basic => item.message.clone(),
                SignatureScheme::MessageAugmentation => {
                    let mut msg = item.public_key.to_compressed().to_vec();
                    msg.extend_from_slice(&item.message);
                    msg
                }
                SignatureScheme::ProofOfPossession => item.message.clone(),
            };

            // Hash message to G2
            let message_point = utils::hash_to_g2(&prepared_message, crate::NEO_BLS_DST);

            // Accumulate with random coefficient
            let coeff_scalar = bls12_381::Scalar::from(coeff);

            left_g1 = (left_g1 + (item.public_key * coeff_scalar)).to_affine();

            left_g2 += message_point * coeff_scalar;
        }

        // Compute the right side of the pairing equation
        let right_g1 = bls12_381::G1Affine::generator();
        let mut right_g2 = G2Projective::identity();

        for (item, &coeff) in self.items.iter().zip(coefficients.iter()) {
            let coeff_scalar = bls12_381::Scalar::from(coeff);
            right_g2 += G2Projective::from(item.signature) * coeff_scalar;
        }

        let left_pairing = pairing(&left_g1, &left_g2.to_affine());
        let right_pairing = pairing(&right_g1, &right_g2.to_affine());

        left_pairing == right_pairing
    }

    /// Get the number of signatures in the batch
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the batch is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all signatures from the batch
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get the current scheme (if enforced)
    pub fn scheme(&self) -> Option<SignatureScheme> {
        self.scheme
    }
}

impl Default for BatchVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::keys::KeyPair;
    use rand::thread_rng;

    #[test]
    fn test_batch_verifier_creation() {
        let verifier = BatchVerifier::new();
        assert!(verifier.is_empty());
        assert_eq!(verifier.len(), 0);
        assert_eq!(verifier.scheme(), None);
    }

    #[test]
    fn test_batch_verifier_with_scheme() {
        let verifier = BatchVerifier::new_with_scheme(SignatureScheme::Basic);
        assert_eq!(verifier.scheme(), Some(SignatureScheme::Basic));
    }

    #[test]
    fn test_single_signature_batch() {
        let mut rng = thread_rng();
        let keypair = KeyPair::generate(&mut rng);
        let message = b"test message";

        let signature = keypair.sign(message, SignatureScheme::Basic).unwrap();

        let mut verifier = BatchVerifier::new();
        verifier
            .add(
                message,
                &signature,
                keypair.public_key(),
                SignatureScheme::Basic,
            )
            .unwrap();

        assert_eq!(verifier.len(), 1);
        assert!(verifier.verify());
    }

    #[test]
    fn test_multiple_signatures_batch() {
        let mut rng = thread_rng();
        let mut verifier = BatchVerifier::new();

        // Create multiple valid signatures
        for i in 0..5 {
            let keypair = KeyPair::generate(&mut rng);
            let message = format!("test message {i}");
            let signature = keypair
                .sign(message.as_bytes(), SignatureScheme::Basic)
                .unwrap();

            verifier
                .add(
                    message.as_bytes(),
                    &signature,
                    keypair.public_key(),
                    SignatureScheme::Basic,
                )
                .unwrap();
        }

        assert_eq!(verifier.len(), 5);
        assert!(verifier.verify());
    }

    #[test]
    fn test_batch_with_invalid_signature() {
        let mut rng = thread_rng();
        let mut verifier = BatchVerifier::new();

        // Add valid signature
        let keypair1 = KeyPair::generate(&mut rng);
        let message1 = b"test message 1";
        let signature1 = keypair1.sign(message1, SignatureScheme::Basic).unwrap();
        verifier
            .add(
                message1,
                &signature1,
                keypair1.public_key(),
                SignatureScheme::Basic,
            )
            .unwrap();

        let keypair2 = KeyPair::generate(&mut rng);
        let message2 = b"test message 2";
        let wrong_message = b"wrong message";
        let signature2 = keypair2
            .sign(wrong_message, SignatureScheme::Basic)
            .unwrap();
        verifier
            .add(
                message2,
                &signature2,
                keypair2.public_key(),
                SignatureScheme::Basic,
            )
            .expect("Operation failed");

        assert_eq!(verifier.len(), 2);
        assert!(!verifier.verify()); // Should fail due to invalid signature
    }

    #[test]
    fn test_scheme_enforcement() {
        let mut rng = thread_rng();
        let mut verifier = BatchVerifier::new_with_scheme(SignatureScheme::Basic);

        let keypair = KeyPair::generate(&mut rng);
        let message = b"test message";
        let signature = keypair.sign(message, SignatureScheme::Basic).unwrap();

        // This should succeed
        assert!(verifier
            .add(
                message,
                &signature,
                keypair.public_key(),
                SignatureScheme::Basic
            )
            .is_ok());

        // This should fail due to scheme mismatch
        let signature2 = keypair
            .sign(message, SignatureScheme::MessageAugmentation)
            .unwrap();
        assert!(verifier
            .add(
                message,
                &signature2,
                keypair.public_key(),
                SignatureScheme::MessageAugmentation
            )
            .is_err());
    }

    #[test]
    fn test_clear_batch() {
        let mut rng = thread_rng();
        let mut verifier = BatchVerifier::new();

        let keypair = KeyPair::generate(&mut rng);
        let message = b"test message";
        let signature = keypair.sign(message, SignatureScheme::Basic).unwrap();

        verifier
            .add(
                message,
                &signature,
                keypair.public_key(),
                SignatureScheme::Basic,
            )
            .unwrap();
        assert_eq!(verifier.len(), 1);

        verifier.clear();
        assert_eq!(verifier.len(), 0);
        assert!(verifier.is_empty());
    }
}

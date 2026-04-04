//! BLS12-381 signature types and operations.

use crate::constants::SIGNATURE_SIZE;
use crate::error::{BlsError, BlsResult};
use crate::keys::{PrivateKey, PublicKey};
use crate::utils;
use crate::NEO_BLS_DST;
use bls12_381::{pairing, G1Affine, G2Affine, G2Projective};
use group::Curve;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// BLS signature schemes (matches C# Neo.Cryptography.BLS12_381 schemes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureScheme {
    /// Basic BLS signature scheme
    Basic,
    /// Message augmentation scheme
    MessageAugmentation,
    /// Proof of possession scheme
    ProofOfPossession,
}

/// BLS12-381 signature (matches C# Neo.Cryptography.BLS12_381.Signature)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    point: G2Projective,
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Self::from_bytes(&bytes).map_err(serde::de::Error::custom)
    }
}

impl Signature {
    /// Creates a signature from a G2 projective point
    pub fn from_g2_projective(point: G2Projective) -> Self {
        Self { point }
    }

    /// Creates a signature from bytes (compressed G2 point)
    pub fn from_bytes(bytes: &[u8]) -> BlsResult<Self> {
        if bytes.len() != SIGNATURE_SIZE {
            return Err(BlsError::InvalidSignatureSize {
                expected: SIGNATURE_SIZE,
                actual: bytes.len(),
            });
        }

        let mut array = [0u8; SIGNATURE_SIZE];
        array.copy_from_slice(bytes);

        let affine_point = G2Affine::from_compressed(&array);
        if affine_point.is_some().into() {
            Ok(Self {
                point: G2Projective::from(affine_point.expect("Operation failed")),
            })
        } else {
            Err(BlsError::invalid_signature("Invalid G2 point"))
        }
    }

    /// Converts the signature to bytes (compressed G2 point)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.point.to_affine().to_compressed().to_vec()
    }

    /// Gets the G2 projective point
    pub fn point(&self) -> &G2Projective {
        &self.point
    }

    /// Signs a message with a private key (matches C# Sign)
    pub fn sign(
        private_key: &PrivateKey,
        message: &[u8],
        scheme: SignatureScheme,
    ) -> BlsResult<Signature> {
        if message.is_empty() {
            return Err(BlsError::EmptyInput);
        }

        // Prepare message based on scheme
        let message_to_sign = match scheme {
            SignatureScheme::Basic => message.to_vec(),
            SignatureScheme::MessageAugmentation => {
                // Prepend public key to message
                let public_key = private_key.public_key();
                let mut augmented = public_key.to_bytes();
                augmented.extend_from_slice(message);
                augmented
            }
            SignatureScheme::ProofOfPossession => message.to_vec(),
        };

        // Hash message to G2 point
        let hash_point = utils::hash_to_g2(&message_to_sign, NEO_BLS_DST);

        let signature_point = hash_point * private_key.scalar();

        Ok(Signature {
            point: signature_point,
        })
    }

    /// Verifies a signature (matches C# Verify)
    pub fn verify(&self, public_key: &PublicKey, message: &[u8], scheme: SignatureScheme) -> bool {
        if message.is_empty() {
            return false;
        }

        // Prepare message based on scheme
        let message_to_verify = match scheme {
            SignatureScheme::Basic => message.to_vec(),
            SignatureScheme::MessageAugmentation => {
                // Prepend public key to message
                let mut augmented = public_key.to_bytes();
                augmented.extend_from_slice(message);
                augmented
            }
            SignatureScheme::ProofOfPossession => message.to_vec(),
        };

        // Hash message to G2 point
        let hash_point = utils::hash_to_g2(&message_to_verify, NEO_BLS_DST);

        let lhs = pairing(&public_key.point(), &hash_point.to_affine());
        let rhs = pairing(&G1Affine::generator(), &self.point.to_affine());

        lhs == rhs
    }

    /// Validates the signature
    pub fn is_valid(&self) -> bool {
        !bool::from(self.point.is_identity())
    }

    /// Creates a signature from hex string
    pub fn from_hex(hex: &str) -> BlsResult<Self> {
        let bytes = hex::decode(hex)?;
        Self::from_bytes(&bytes)
    }

    /// Converts the signature to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Adds two signatures (for signature aggregation)
    pub fn add(&self, other: &Signature) -> Signature {
        Signature {
            point: self.point + other.point,
        }
    }

    /// Create a signature from an affine G2 point
    pub fn from_affine(point: G2Affine) -> Self {
        Self {
            point: G2Projective::from(point),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::keys::KeyPair;
    // Removed unused imports
    use rand::thread_rng;

    #[test]
    fn test_basic_signature_scheme() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"Hello, BLS!";

        let signature =
            Signature::sign(key_pair.private_key(), message, SignatureScheme::Basic).unwrap();
        assert!(signature.verify(key_pair.public_key(), message, SignatureScheme::Basic));

        // Test with wrong message
        let wrong_message = b"Wrong message";
        assert!(!signature.verify(key_pair.public_key(), wrong_message, SignatureScheme::Basic));
    }

    #[test]
    fn test_message_augmentation_scheme() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"Augmented message";

        let signature = Signature::sign(
            key_pair.private_key(),
            message,
            SignatureScheme::MessageAugmentation,
        )
        .unwrap();
        assert!(signature.verify(
            key_pair.public_key(),
            message,
            SignatureScheme::MessageAugmentation
        ));

        // Should fail with basic scheme
        assert!(!signature.verify(key_pair.public_key(), message, SignatureScheme::Basic));
    }

    #[test]
    fn test_proof_of_possession_scheme() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"PoP message";

        let signature = Signature::sign(
            key_pair.private_key(),
            message,
            SignatureScheme::ProofOfPossession,
        )
        .unwrap();
        assert!(signature.verify(
            key_pair.public_key(),
            message,
            SignatureScheme::ProofOfPossession
        ));
    }

    #[test]
    fn test_signature_serialization() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"Serialize this signature";

        let signature =
            Signature::sign(key_pair.private_key(), message, SignatureScheme::Basic).unwrap();

        let bytes = signature.to_bytes();
        assert_eq!(bytes.len(), SIGNATURE_SIZE);

        let deserialized = Signature::from_bytes(&bytes).unwrap();
        assert_eq!(signature, deserialized);

        // Verify deserialized signature works
        assert!(deserialized.verify(key_pair.public_key(), message, SignatureScheme::Basic));
    }

    #[test]
    fn test_signature_hex() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"Hex signature test";

        let signature =
            Signature::sign(key_pair.private_key(), message, SignatureScheme::Basic).unwrap();

        let hex = signature.to_hex();
        let from_hex = Signature::from_hex(&hex).unwrap();
        assert_eq!(signature, from_hex);
    }

    #[test]
    fn test_signature_addition() {
        let mut rng = thread_rng();
        let key1 = KeyPair::generate(&mut rng);
        let key2 = KeyPair::generate(&mut rng);
        let message = b"Add signatures";

        let sig1 = Signature::sign(key1.private_key(), message, SignatureScheme::Basic).unwrap();
        let sig2 = Signature::sign(key2.private_key(), message, SignatureScheme::Basic).unwrap();

        let sum = sig1.add(&sig2);
        assert!(sum.is_valid());
    }

    #[test]
    fn test_empty_message() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let empty_message = b"";

        let result = Signature::sign(
            key_pair.private_key(),
            empty_message,
            SignatureScheme::Basic,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_signature_size() {
        let invalid_bytes = vec![0u8; 64]; // Wrong size
        let result = Signature::from_bytes(&invalid_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_signature_validation() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);
        let message = b"Validation test";

        let signature =
            Signature::sign(key_pair.private_key(), message, SignatureScheme::Basic).unwrap();
        assert!(signature.is_valid());
    }
}

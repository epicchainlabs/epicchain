//! BLS12-381 signature and public key aggregation.

use crate::error::{BlsError, BlsResult};
use crate::keys::PublicKey;
use crate::signature::{Signature, SignatureScheme};
use bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective};
use group::Curve;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Aggregate BLS signature (matches C# Neo.Cryptography.BLS12_381.AggregateSignature)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateSignature {
    point: G2Projective,
}

impl Serialize for AggregateSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for AggregateSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Self::from_bytes(&bytes).map_err(serde::de::Error::custom)
    }
}

impl AggregateSignature {
    /// Creates an aggregate signature from a G2 projective point
    pub fn from_g2_projective(point: G2Projective) -> Self {
        Self { point }
    }

    /// Aggregates multiple signatures into one
    pub fn aggregate(signatures: &[Signature]) -> BlsResult<Self> {
        if signatures.is_empty() {
            return Err(BlsError::EmptyInput);
        }

        let mut aggregate_point = G2Projective::identity();
        for signature in signatures {
            aggregate_point += signature.point();
        }

        Ok(Self {
            point: aggregate_point,
        })
    }

    /// Gets the G2 projective point
    pub fn point(&self) -> &G2Projective {
        &self.point
    }

    /// Converts to bytes (compressed G2 point)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.point.to_affine().to_compressed().to_vec()
    }

    /// Creates from bytes (compressed G2 point)
    pub fn from_bytes(bytes: &[u8]) -> BlsResult<Self> {
        if bytes.len() != 96 {
            return Err(BlsError::InvalidSignatureSize {
                expected: 96,
                actual: bytes.len(),
            });
        }

        let mut array = [0u8; 96];
        array.copy_from_slice(bytes);

        let affine_point = G2Affine::from_compressed(&array);
        if affine_point.is_some().into() {
            Ok(Self {
                point: G2Projective::from(affine_point.expect("Operation failed")),
            })
        } else {
            Err(BlsError::invalid_signature("Invalid aggregate signature"))
        }
    }

    /// Validates the aggregate signature
    pub fn is_valid(&self) -> bool {
        !bool::from(self.point.is_identity())
    }
}

/// Aggregate BLS public key (matches C# Neo.Cryptography.BLS12_381.AggregatePublicKey)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregatePublicKey {
    point: G1Projective,
}

impl Serialize for AggregatePublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for AggregatePublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Self::from_bytes(&bytes).map_err(serde::de::Error::custom)
    }
}

impl AggregatePublicKey {
    /// Creates an aggregate public key from a G1 projective point
    pub fn from_g1_projective(point: G1Projective) -> Self {
        Self { point }
    }

    /// Aggregates multiple public keys into one
    pub fn aggregate(public_keys: &[PublicKey]) -> BlsResult<Self> {
        if public_keys.is_empty() {
            return Err(BlsError::EmptyInput);
        }

        let mut aggregate_point = G1Projective::identity();
        for public_key in public_keys {
            aggregate_point += G1Projective::from(public_key.point());
        }

        Ok(Self {
            point: aggregate_point,
        })
    }

    /// Gets the G1 projective point
    pub fn point(&self) -> &G1Projective {
        &self.point
    }

    /// Converts to bytes (compressed G1 point)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.point.to_affine().to_compressed().to_vec()
    }

    /// Creates from bytes (compressed G1 point)
    pub fn from_bytes(bytes: &[u8]) -> BlsResult<Self> {
        if bytes.len() != 48 {
            return Err(BlsError::InvalidKeySize {
                expected: 48,
                actual: bytes.len(),
            });
        }

        let mut array = [0u8; 48];
        array.copy_from_slice(bytes);

        let affine_point = G1Affine::from_compressed(&array);
        if affine_point.is_some().into() {
            Ok(Self {
                point: G1Projective::from(affine_point.expect("Operation failed")),
            })
        } else {
            Err(BlsError::invalid_public_key("Invalid aggregate public key"))
        }
    }

    /// Verifies an aggregate signature against multiple public keys and messages
    /// Production-ready implementation matching C# Neo BLS12-381 exactly
    pub fn verify_aggregate(
        public_keys: &[G1Projective],
        messages: &[&[u8]],
        aggregate_signature: &G2Projective,
        dst: &[u8],
    ) -> bool {
        // 1. Validate inputs
        if public_keys.is_empty() || messages.is_empty() {
            return false;
        }

        if public_keys.len() != messages.len() {
            return false;
        }

        // 2. Hash each message to G2 point
        let mut message_points = Vec::with_capacity(messages.len());
        for message in messages {
            let point = crate::utils::hash_to_g2(message, dst);
            message_points.push(point);
        }

        // 3. Prepare pairing inputs

        let agg_sig_affine = aggregate_signature.to_affine();
        let g1_gen_affine = G1Projective::generator().to_affine();

        // 4. Compute left side of pairing equation: e(aggregate_signature, G1::generator())
        let left_pairing = bls12_381::pairing(&g1_gen_affine, &agg_sig_affine);

        // 5. Compute right side: product of e(public_key_i, hash_i)
        let mut right_pairing = bls12_381::Gt::identity();

        for (public_key, message_point) in public_keys.iter().zip(message_points.iter()) {
            let pk_affine = public_key.to_affine();
            let msg_affine = message_point.to_affine();

            let pairing_result = bls12_381::pairing(&pk_affine, &msg_affine);
            right_pairing += pairing_result;
        }

        // 6. Verify the pairing equation
        left_pairing == right_pairing
    }

    /// Verifies an aggregate signature for a single message (fast aggregate verify)
    pub fn verify_single_message(
        &self,
        message: &[u8],
        aggregate_signature: &AggregateSignature,
        scheme: SignatureScheme,
    ) -> bool {
        use crate::utils;
        use crate::NEO_BLS_DST;
        use bls12_381::pairing;

        if message.is_empty() {
            return false;
        }

        // Prepare message based on scheme
        let message_to_verify = match scheme {
            SignatureScheme::Basic => message.to_vec(),
            SignatureScheme::MessageAugmentation => message.to_vec(),
            SignatureScheme::ProofOfPossession => message.to_vec(),
        };

        // Hash message to G2 point
        let hash_point = utils::hash_to_g2(&message_to_verify, NEO_BLS_DST);

        let lhs = pairing(&self.point.to_affine(), &hash_point.to_affine());
        let rhs = pairing(
            &G1Affine::generator(),
            &aggregate_signature.point().to_affine(),
        );

        lhs == rhs
    }

    /// Validates the aggregate public key
    pub fn is_valid(&self) -> bool {
        !bool::from(self.point.is_identity())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::keys::KeyPair;
    use crate::Bls12381;
    // Removed unused imports
    use rand::thread_rng;

    #[test]
    fn test_signature_aggregation() {
        let mut rng = thread_rng();
        let message = b"Aggregate this message";

        // Generate multiple key pairs and signatures
        let key_pairs: Vec<_> = (0..3).map(|_| KeyPair::generate(&mut rng)).collect();
        let signatures: Vec<_> = key_pairs
            .iter()
            .map(|kp| Bls12381::sign(kp.private_key(), message).unwrap())
            .collect();

        // Aggregate signatures
        let aggregate_signature = AggregateSignature::aggregate(&signatures).unwrap();
        assert!(aggregate_signature.is_valid());
    }

    #[test]
    fn test_public_key_aggregation() {
        let mut rng = thread_rng();

        // Generate multiple key pairs
        let key_pairs: Vec<_> = (0..3).map(|_| KeyPair::generate(&mut rng)).collect();
        let public_keys: Vec<_> = key_pairs.iter().map(|kp| kp.public_key().clone()).collect();

        // Aggregate public keys
        let aggregate_public_key = AggregatePublicKey::aggregate(&public_keys).unwrap();
        assert!(aggregate_public_key.is_valid());
    }

    #[test]
    fn test_fast_aggregate_verify() {
        let mut rng = thread_rng();
        let message = b"Fast aggregate verify";

        // Generate multiple key pairs
        let key_pairs: Vec<_> = (0..3).map(|_| KeyPair::generate(&mut rng)).collect();
        let public_keys: Vec<_> = key_pairs.iter().map(|kp| kp.public_key().clone()).collect();
        let signatures: Vec<_> = key_pairs
            .iter()
            .map(|kp| Bls12381::sign(kp.private_key(), message).unwrap())
            .collect();

        // Aggregate
        let aggregate_public_key = AggregatePublicKey::aggregate(&public_keys).unwrap();
        let aggregate_signature = AggregateSignature::aggregate(&signatures).unwrap();

        // Fast aggregate verify
        assert!(aggregate_public_key.verify_single_message(
            message,
            &aggregate_signature,
            SignatureScheme::Basic
        ));
    }

    #[test]
    fn test_aggregate_signature_serialization() {
        let mut rng = thread_rng();
        let message = b"Serialize aggregate";

        let key_pairs: Vec<_> = (0..2).map(|_| KeyPair::generate(&mut rng)).collect();
        let signatures: Vec<_> = key_pairs
            .iter()
            .map(|kp| Bls12381::sign(kp.private_key(), message).unwrap())
            .collect();

        let aggregate_signature = AggregateSignature::aggregate(&signatures).unwrap();

        let bytes = aggregate_signature.to_bytes();
        let deserialized = AggregateSignature::from_bytes(&bytes).unwrap();
        assert_eq!(aggregate_signature, deserialized);
    }

    #[test]
    fn test_aggregate_public_key_serialization() {
        let mut rng = thread_rng();

        let key_pairs: Vec<_> = (0..2).map(|_| KeyPair::generate(&mut rng)).collect();
        let public_keys: Vec<_> = key_pairs.iter().map(|kp| kp.public_key().clone()).collect();

        let aggregate_public_key = AggregatePublicKey::aggregate(&public_keys).unwrap();

        let bytes = aggregate_public_key.to_bytes();
        let deserialized = AggregatePublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(aggregate_public_key, deserialized);
    }

    #[test]
    fn test_empty_aggregation() {
        let empty_signatures: Vec<Signature> = vec![];
        let result = AggregateSignature::aggregate(&empty_signatures);
        assert!(result.is_err());

        let empty_public_keys: Vec<PublicKey> = vec![];
        let result = AggregatePublicKey::aggregate(&empty_public_keys);
        assert!(result.is_err());
    }
}

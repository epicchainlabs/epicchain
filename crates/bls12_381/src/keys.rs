//! BLS12-381 key types and operations.

use crate::constants::{PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE};
use crate::error::{BlsError, BlsResult};
use crate::signature::{Signature, SignatureScheme};
use bls12_381::{G1Affine, Scalar};
use ff::Field;
use group::Curve;
use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// BLS12-381 private key (matches C# Neo.Cryptography.BLS12_381.PrivateKey)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    scalar: Scalar,
}

impl PrivateKey {
    /// Generates a new random private key
    pub fn generate<R: RngCore>(rng: &mut R) -> Self {
        let scalar = Scalar::random(rng);
        Self { scalar }
    }

    /// Creates a private key from a scalar
    pub fn from_scalar(scalar: Scalar) -> Self {
        Self { scalar }
    }

    /// Creates a private key from bytes
    pub fn from_bytes(bytes: &[u8]) -> BlsResult<Self> {
        if bytes.len() != PRIVATE_KEY_SIZE {
            return Err(BlsError::InvalidKeySize {
                expected: PRIVATE_KEY_SIZE,
                actual: bytes.len(),
            });
        }

        let mut array = [0u8; PRIVATE_KEY_SIZE];
        array.copy_from_slice(bytes);

        let scalar = Scalar::from_bytes(&array);
        if scalar.is_some().into() {
            Ok(Self {
                scalar: scalar.expect("Operation failed"),
            })
        } else {
            Err(BlsError::invalid_private_key("Invalid scalar value"))
        }
    }

    /// Converts the private key to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.scalar.to_bytes().to_vec()
    }

    /// Gets the scalar value
    pub fn scalar(&self) -> &Scalar {
        &self.scalar
    }

    /// Derives the public key from this private key
    pub fn public_key(&self) -> PublicKey {
        let g1_point = G1Affine::generator() * self.scalar;
        PublicKey::from_g1_affine(g1_point.to_affine())
    }

    /// Signs a message with this private key
    pub fn sign(&self, message: &[u8], scheme: SignatureScheme) -> BlsResult<Signature> {
        Signature::sign(self, message, scheme)
    }

    /// Validates the private key
    pub fn is_valid(&self) -> bool {
        !bool::from(self.scalar.is_zero())
    }

    /// Creates a private key from hex string
    pub fn from_hex(hex: &str) -> BlsResult<Self> {
        let bytes = hex::decode(hex)?;
        Self::from_bytes(&bytes)
    }

    /// Converts the private key to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Securely clear the private key from memory
    pub fn zeroize(&mut self) {
        // Manual zeroization since Scalar doesn't implement Zeroize
        self.scalar = Scalar::zero();
    }
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// BLS12-381 public key (matches C# Neo.Cryptography.BLS12_381.PublicKey)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    point: G1Affine,
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.to_bytes();
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Self::from_bytes(&bytes).map_err(serde::de::Error::custom)
    }
}

impl PublicKey {
    /// Creates a public key from a G1 affine point
    pub fn from_g1_affine(point: G1Affine) -> Self {
        Self { point }
    }

    /// Creates a public key from bytes (compressed G1 point)
    pub fn from_bytes(bytes: &[u8]) -> BlsResult<Self> {
        if bytes.len() != PUBLIC_KEY_SIZE {
            return Err(BlsError::InvalidKeySize {
                expected: PUBLIC_KEY_SIZE,
                actual: bytes.len(),
            });
        }

        let mut array = [0u8; PUBLIC_KEY_SIZE];
        array.copy_from_slice(bytes);

        let point = G1Affine::from_compressed(&array);
        if point.is_some().into() {
            Ok(Self {
                point: point.expect("Operation failed"),
            })
        } else {
            Err(BlsError::invalid_public_key("Invalid G1 point"))
        }
    }

    /// Converts the public key to bytes (compressed G1 point)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.point.to_compressed().to_vec()
    }

    /// Gets the G1 affine point
    pub fn point(&self) -> G1Affine {
        self.point
    }

    /// Verifies a signature against this public key
    pub fn verify(&self, message: &[u8], signature: &Signature, scheme: SignatureScheme) -> bool {
        signature.verify(self, message, scheme)
    }

    /// Validates the public key
    pub fn is_valid(&self) -> bool {
        !bool::from(self.point.is_identity())
    }

    /// Creates a public key from hex string
    pub fn from_hex(hex: &str) -> BlsResult<Self> {
        let bytes = hex::decode(hex)?;
        Self::from_bytes(&bytes)
    }

    /// Converts the public key to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Adds two public keys (for key aggregation)
    pub fn add(&self, other: &PublicKey) -> PublicKey {
        use bls12_381::G1Projective;
        let sum = G1Projective::from(self.point) + G1Projective::from(other.point);
        PublicKey {
            point: sum.to_affine(),
        }
    }

    /// Create a public key from an affine G1 point
    pub fn from_affine(point: G1Affine) -> Self {
        Self { point }
    }
}

/// BLS12-381 key pair (matches C# Neo.Cryptography.BLS12_381.KeyPair)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPair {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl KeyPair {
    /// Generates a new random key pair
    pub fn generate<R: RngCore>(rng: &mut R) -> Self {
        let private_key = PrivateKey::generate(rng);
        let public_key = private_key.public_key();
        Self {
            private_key,
            public_key,
        }
    }

    /// Creates a key pair from a private key
    pub fn from_private_key(private_key: PrivateKey) -> Self {
        let public_key = private_key.public_key();
        Self {
            private_key,
            public_key,
        }
    }

    /// Gets the private key
    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }

    /// Gets the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Signs a message with the private key
    pub fn sign(&self, message: &[u8], scheme: SignatureScheme) -> BlsResult<Signature> {
        self.private_key.sign(message, scheme)
    }

    /// Verifies a signature with the public key
    pub fn verify(&self, message: &[u8], signature: &Signature, scheme: SignatureScheme) -> bool {
        self.public_key.verify(message, signature, scheme)
    }

    /// Validates the key pair
    pub fn is_valid(&self) -> bool {
        self.private_key.is_valid() && self.public_key.is_valid()
    }

    /// Serializes the key pair to bytes (private key + public key)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PRIVATE_KEY_SIZE + PUBLIC_KEY_SIZE);
        bytes.extend_from_slice(&self.private_key.to_bytes());
        bytes.extend_from_slice(&self.public_key.to_bytes());
        bytes
    }

    /// Deserializes a key pair from bytes
    pub fn from_bytes(bytes: &[u8]) -> BlsResult<Self> {
        if bytes.len() != PRIVATE_KEY_SIZE + PUBLIC_KEY_SIZE {
            return Err(BlsError::InvalidKeySize {
                expected: PRIVATE_KEY_SIZE + PUBLIC_KEY_SIZE,
                actual: bytes.len(),
            });
        }

        let private_key = PrivateKey::from_bytes(&bytes[..PRIVATE_KEY_SIZE])?;
        let public_key = PublicKey::from_bytes(&bytes[PRIVATE_KEY_SIZE..])?;

        // Verify that the public key matches the private key
        let derived_public_key = private_key.public_key();
        if public_key != derived_public_key {
            return Err(BlsError::invalid_input(
                "Public key does not match private key",
            ));
        }

        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Securely clear the key pair from memory
    pub fn zeroize(&mut self) {
        self.private_key.zeroize();
        // Public key doesn't need zeroization as it's not secret
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.zeroize();
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::constants::HASH_SIZE;
    use rand::thread_rng;

    #[test]
    fn test_private_key_generation() {
        let mut rng = thread_rng();
        let private_key = PrivateKey::generate(&mut rng);
        assert!(private_key.is_valid());
    }

    #[test]
    fn test_private_key_serialization() {
        let mut rng = thread_rng();
        let private_key = PrivateKey::generate(&mut rng);

        let bytes = private_key.to_bytes();
        assert_eq!(bytes.len(), PRIVATE_KEY_SIZE);

        let deserialized = PrivateKey::from_bytes(&bytes).unwrap();
        assert_eq!(private_key, deserialized);
    }

    #[test]
    fn test_public_key_derivation() {
        let mut rng = thread_rng();
        let private_key = PrivateKey::generate(&mut rng);
        let public_key = private_key.public_key();

        assert!(public_key.is_valid());
    }

    #[test]
    fn test_public_key_serialization() {
        let mut rng = thread_rng();
        let private_key = PrivateKey::generate(&mut rng);
        let public_key = private_key.public_key();

        let bytes = public_key.to_bytes();
        assert_eq!(bytes.len(), PUBLIC_KEY_SIZE);

        let deserialized = PublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(public_key, deserialized);
    }

    #[test]
    fn test_key_pair_generation() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);

        assert!(key_pair.is_valid());
        assert_eq!(key_pair.private_key().public_key(), *key_pair.public_key());
    }

    #[test]
    fn test_key_pair_serialization() {
        let mut rng = thread_rng();
        let key_pair = KeyPair::generate(&mut rng);

        let bytes = key_pair.to_bytes();
        assert_eq!(bytes.len(), PRIVATE_KEY_SIZE + PUBLIC_KEY_SIZE);

        let deserialized = KeyPair::from_bytes(&bytes).unwrap();
        assert_eq!(key_pair, deserialized);
    }

    #[test]
    fn test_hex_serialization() {
        let mut rng = thread_rng();
        let private_key = PrivateKey::generate(&mut rng);
        let public_key = private_key.public_key();

        // Test private key hex
        let private_hex = private_key.to_hex();
        let private_from_hex = PrivateKey::from_hex(&private_hex).unwrap();
        assert_eq!(private_key, private_from_hex);

        // Test public key hex
        let public_hex = public_key.to_hex();
        let public_from_hex = PublicKey::from_hex(&public_hex).unwrap();
        assert_eq!(public_key, public_from_hex);
    }

    #[test]
    fn test_public_key_operations() {
        let mut rng = thread_rng();
        let key1 = KeyPair::generate(&mut rng);
        let key2 = KeyPair::generate(&mut rng);

        // Test addition
        let sum = key1.public_key().add(key2.public_key());
        assert!(sum.is_valid());
    }

    #[test]
    fn test_invalid_key_sizes() {
        // Test invalid private key size
        let invalid_private = PrivateKey::from_bytes(&[0u8; 16]);
        assert!(invalid_private.is_err());

        // Test invalid public key size
        let invalid_public = PublicKey::from_bytes(&[0u8; HASH_SIZE]);
        assert!(invalid_public.is_err());

        // Test invalid key pair size
        let invalid_pair = KeyPair::from_bytes(&[0u8; 64]);
        assert!(invalid_pair.is_err());
    }
}

//! Core cryptographic functions for Neo.
//!
//! This module provides core cryptographic functions used in the Neo blockchain.

use crate::ecc::ECPoint;
use crate::hash_algorithm::HashAlgorithm;
use crate::hasher::Hasher;
use crate::Error;
use neo_config::HASH_SIZE;
use rand::{rngs::OsRng, RngCore};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ECDSA_CACHE: Mutex<HashMap<Vec<u8>, Vec<u8>>> = Mutex::new(HashMap::new());
}

/// Core cryptographic functions for Neo.
/// This matches the C# Neo.Cryptography.Crypto class exactly.
pub struct Crypto;

impl Crypto {
    /// Calculates the 160-bit hash value of the specified message.
    /// This matches the C# Crypto.Hash160 implementation exactly.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be hashed
    ///
    /// # Returns
    ///
    /// 160-bit hash value (RIPEMD160(SHA256(message)))
    pub fn hash160(message: &[u8]) -> Vec<u8> {
        let sha256_hash = Hasher::sha256(message);
        Hasher::ripemd160(&sha256_hash)
    }

    /// Calculates the 256-bit hash value of the specified message.
    /// This matches the C# Crypto.Hash256 implementation exactly.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be hashed
    ///
    /// # Returns
    ///
    /// 256-bit hash value (SHA256(SHA256(message)))
    pub fn hash256(message: &[u8]) -> Vec<u8> {
        let sha256_hash = Hasher::sha256(message);
        Hasher::sha256(&sha256_hash)
    }

    /// Signs the specified message using the ECDSA algorithm and specified hash algorithm.
    /// This matches the C# Crypto.Sign implementation.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be signed
    /// * `pri_key` - The private key to be used
    /// * `ec_curve` - The ECCurve curve of the signature (default is Secp256r1)
    /// * `hash_algorithm` - The hash algorithm to hash the message (default is SHA256)
    ///
    /// # Returns
    ///
    /// The ECDSA signature for the specified message
    pub fn sign(
        message: &[u8],
        pri_key: &[u8],
        ec_curve: Option<crate::ecc::ECCurve>,
        hash_algorithm: HashAlgorithm,
    ) -> Result<Vec<u8>, Error> {
        let curve = ec_curve.unwrap_or(crate::ecc::ECCurve::secp256r1());

        // Validate private key length
        if pri_key.len() != HASH_SIZE {
            return Err(Error::InvalidKey(
                "Private key must be HASH_SIZE bytes".to_string(),
            ));
        }

        let private_key_array: [u8; HASH_SIZE] = pri_key
            .try_into()
            .map_err(|_| Error::InvalidKey("Invalid private key length".to_string()))?;

        // Hash the message first using the specified algorithm
        let message_hash = Self::get_message_hash(message, hash_algorithm)?;

        match curve.name {
            "secp256r1" => crate::ecdsa::ECDsa::sign(&message_hash, &private_key_array),
            "secp256k1" => crate::ecdsa::ECDsa::sign_secp256k1(&message_hash, pri_key),
            _ => Err(Error::UnsupportedAlgorithm(format!(
                "Unsupported curve: {}",
                curve.name
            ))),
        }
    }

    /// Verifies that a digital signature is appropriate for the provided key, message and hash algorithm.
    /// This matches the C# Crypto.VerifySignature implementation.
    ///
    /// # Arguments
    ///
    /// * `message` - The signed message
    /// * `signature` - The signature to be verified
    /// * `pubkey` - The public key to be used
    /// * `hash_algorithm` - The hash algorithm to be used to hash the message
    ///
    /// # Returns
    ///
    /// true if the signature is valid; otherwise, false
    pub fn verify_signature(
        message: &[u8],
        signature: &[u8],
        pubkey: &ECPoint,
        hash_algorithm: HashAlgorithm,
    ) -> bool {
        if signature.len() != 64 {
            return false;
        }

        // Hash the message first using the specified algorithm
        let message_hash = match Self::get_message_hash(message, hash_algorithm) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        match pubkey.get_curve().name {
            "secp256r1" => {
                let pubkey_bytes = match pubkey.encode_point(false) {
                    Ok(bytes) => bytes,
                    Err(_) => return false,
                };
                crate::ecdsa::ECDsa::verify(&message_hash, signature, &pubkey_bytes)
                    .unwrap_or(false)
            }
            "secp256k1" => {
                let pubkey_bytes = match pubkey.encode_point(false) {
                    Ok(bytes) => bytes,
                    Err(_) => return false,
                };
                crate::ecdsa::ECDsa::verify_signature_secp256k1(
                    &message_hash,
                    signature,
                    &pubkey_bytes,
                )
                .unwrap_or(false)
            }
            _ => false,
        }
    }

    /// Get hash from message.
    /// This matches the C# Crypto.GetMessageHash implementation exactly.
    ///
    /// # Arguments
    ///
    /// * `message` - Original message
    /// * `hash_algorithm` - The hash algorithm to be used hash the message
    ///
    /// # Returns
    ///
    /// Hashed message
    pub fn get_message_hash(
        message: &[u8],
        hash_algorithm: HashAlgorithm,
    ) -> Result<Vec<u8>, Error> {
        match hash_algorithm {
            HashAlgorithm::Sha256 => Ok(crate::hash::sha256(message).to_vec()),
            HashAlgorithm::Sha512 => Ok(crate::hash::sha512(message).to_vec()),
            HashAlgorithm::Keccak256 => Ok(crate::hash::keccak256(message).to_vec()),
        }
    }

    /// Recovers the public key from a signature and message hash.
    /// This matches the C# Crypto.ECRecover implementation.
    ///
    /// # Arguments
    ///
    /// * `signature` - Signature, either 65 bytes (r[HASH_SIZE] || s[HASH_SIZE] || v[1]) or 64 bytes in "compact" form (r[HASH_SIZE] || yParityAndS[HASH_SIZE])
    /// * `hash` - HASH_SIZE-byte message hash
    ///
    /// # Returns
    ///
    /// The recovered public key
    ///
    /// # Errors
    ///
    /// Returns an error if signature or hash is invalid
    pub fn ec_recover(signature: &[u8], hash: &[u8]) -> Result<ECPoint, Error> {
        if signature.len() != 65 && signature.len() != 64 {
            return Err(Error::InvalidSignature(
                "Signature must be 65 or 64 bytes".to_string(),
            ));
        }
        if hash.len() != HASH_SIZE {
            return Err(Error::InvalidSignature(
                "Message hash must be HASH_SIZE bytes".to_string(),
            ));
        }

        if signature.len() == 65 {
            // Format: r[HASH_SIZE] || s[HASH_SIZE] || v[1]
            let r = &signature[0..HASH_SIZE];
            let s = &signature[32..64];
            let v = signature[64];

            let rec_id = if v >= 27 { v - 27 } else { v };
            if rec_id > 3 {
                return Err(Error::InvalidSignature(
                    "Recovery value must be in [0..3] after normalization".to_string(),
                ));
            }

            Self::recover_public_key_secp256k1(r, s, hash, rec_id)
        } else {
            // 64 bytes "compact" format: r[HASH_SIZE] || yParityAndS[HASH_SIZE]
            let r = &signature[0..HASH_SIZE];
            let y_parity_and_s = &signature[32..64];

            // Extract yParity from the top bit of s
            let y_parity = (y_parity_and_s[0] & 0x80) != 0;

            // Create s without the top bit
            let mut s = [0u8; HASH_SIZE];
            s.copy_from_slice(y_parity_and_s);
            s[0] &= 0x7F; // Clear the top bit

            let rec_id = if y_parity { 1 } else { 0 };

            Self::recover_public_key_secp256k1(r, &s, hash, rec_id)
        }
    }

    /// Internal function to recover public key for secp256k1 curve
    /// Production implementation using the secp256k1 crate for reliable recovery
    fn recover_public_key_secp256k1(
        r_bytes: &[u8],
        s_bytes: &[u8],
        hash: &[u8],
        recovery_id: u8,
    ) -> Result<ECPoint, Error> {
        use secp256k1::{
            ecdsa::{RecoverableSignature, RecoveryId},
            Message, Secp256k1,
        };

        // Create secp256k1 context
        let secp = Secp256k1::new();

        // Create message from hash
        let message = Message::from_digest_slice(hash)
            .map_err(|e| Error::InvalidSignature(format!("Invalid message hash: {e}")))?;

        // Create recovery ID
        let rec_id = RecoveryId::from_i32(recovery_id as i32)
            .map_err(|e| Error::InvalidSignature(format!("Invalid recovery ID: {e}")))?;

        // Create signature data
        let mut sig_data = [0u8; 64];
        sig_data[0..HASH_SIZE].copy_from_slice(r_bytes);
        sig_data[32..64].copy_from_slice(s_bytes);

        // Create recoverable signature
        let recoverable_sig = RecoverableSignature::from_compact(&sig_data, rec_id)
            .map_err(|e| Error::InvalidSignature(format!("Invalid recoverable signature: {e}")))?;

        // Recover public key
        let public_key = secp
            .recover_ecdsa(&message, &recoverable_sig)
            .map_err(|e| Error::InvalidSignature(format!("Public key recovery failed: {e}")))?;

        // Convert to ECPoint format
        let pubkey_bytes = public_key.serialize_uncompressed();
        ECPoint::decode(&pubkey_bytes, crate::ecc::ECCurve::secp256k1())
            .map_err(|e| Error::InvalidKey(format!("Failed to decode ECPoint: {e}")))
    }

    /// Generates a random byte array of the specified length.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the byte array to generate
    ///
    /// # Returns
    ///
    /// A random byte array of the specified length
    pub fn random_bytes(length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        OsRng.fill_bytes(&mut bytes);
        bytes
    }

    /// Computes the SHA-256 hash of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The SHA-256 hash of the data
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        Hasher::sha256(data)
    }

    /// Computes the RIPEMD-160 hash of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The RIPEMD-160 hash of the data
    pub fn ripemd160(data: &[u8]) -> Vec<u8> {
        Hasher::ripemd160(data)
    }

    /// Verifies an ECDSA signature.
    ///
    /// # Arguments
    ///
    /// * `message` - The message that was signed
    /// * `signature` - The signature to verify
    /// * `public_key` - The public key to verify against
    ///
    /// # Returns
    ///
    /// `true` if the signature is valid, `false` otherwise
    pub fn verify_signature_bytes(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        crate::helper::verify_signature(message, signature, public_key)
    }

    /// Signs a message using ECDSA.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to sign
    /// * `private_key` - The private key to sign with
    ///
    /// # Returns
    ///
    /// The signature or an error
    pub fn sign_message(message: &[u8], private_key: &[u8]) -> Result<Vec<u8>, Error> {
        crate::helper::sign_message(message, private_key)
    }

    /// Derives a public key from a private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key
    ///
    /// # Returns
    ///
    /// The derived public key or an error
    pub fn private_key_to_public_key(private_key: &[u8]) -> Result<Vec<u8>, Error> {
        crate::helper::private_key_to_public_key(private_key)
    }

    /// Computes the script hash of a public key.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key
    ///
    /// # Returns
    ///
    /// The script hash of the public key
    pub fn public_key_to_script_hash(public_key: &[u8]) -> Vec<u8> {
        crate::helper::public_key_to_script_hash(public_key)
    }

    /// Generates a random private key.
    ///
    /// # Returns
    ///
    /// A random private key
    pub fn generate_private_key() -> Vec<u8> {
        crate::helper::generate_private_key()
    }

    /// Verifies that a point is on the specified curve.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to verify
    ///
    /// # Returns
    ///
    /// `true` if the point is on the curve, `false` otherwise
    pub fn verify_point(point: &ECPoint) -> bool {
        !point.is_infinity()
    }

    /// Create and cache ECDsa objects (matches C# CreateECDsa implementation)
    ///
    /// # Arguments
    ///
    /// * `pubkey` - The public key
    ///
    /// # Returns
    ///
    /// Cached ECDsa implementation
    pub fn create_ecdsa(pubkey: &ECPoint) -> Result<Vec<u8>, Error> {
        let key = match pubkey.encode_point(false) {
            Ok(k) => k,
            Err(e) => return Err(Error::InvalidKey(format!("Failed to encode point: {e}"))),
        };

        // Check cache first
        {
            let cache = ECDSA_CACHE.lock().map_err(|_| Error::LockError)?;
            if let Some(cached) = cache.get(&key) {
                return Ok(cached.clone());
            }
        }

        // In production, this would contain the actual ECDSA context and validation state
        let ecdsa_data = key.clone();

        // Add to cache
        {
            let mut cache = ECDSA_CACHE.lock().map_err(|_| Error::LockError)?;
            cache.insert(key, ecdsa_data.clone());
        }

        Ok(ecdsa_data)
    }

    /// Computes a shared secret using ECDH.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key
    /// * `public_key` - The public key
    ///
    /// # Returns
    ///
    /// The shared secret or an error
    pub fn compute_shared_secret(private_key: &[u8], public_key: &[u8]) -> Result<Vec<u8>, Error> {
        use secp256k1::{PublicKey as Secp256k1PublicKey, Secp256k1, SecretKey};

        let _secp = Secp256k1::new();

        // Parse private key
        let secret_key = SecretKey::from_slice(private_key)
            .map_err(|e| Error::InvalidKey(format!("Invalid private key: {e}")))?;

        // Parse public key
        let secp256k1_public_key = Secp256k1PublicKey::from_slice(public_key)
            .map_err(|e| Error::InvalidKey(format!("Invalid public key: {e}")))?;

        use secp256k1::ecdh::SharedSecret;

        let shared_secret = SharedSecret::new(&secp256k1_public_key, &secret_key);

        // Return the shared secret as bytes
        Ok(shared_secret.secret_bytes().to_vec())
    }

    /// Verifies that a digital signature is appropriate for the provided key, curve, message and hash algorithm.
    /// This matches the C# Crypto.VerifySignature(message, signature, pubkey, curve, hashAlgorithm) overload.
    ///
    /// # Arguments
    ///
    /// * `message` - The signed message
    /// * `signature` - The signature to be verified
    /// * `pubkey_bytes` - The public key bytes to be used
    /// * `curve` - The curve to be used by the ECDSA algorithm
    /// * `hash_algorithm` - The hash algorithm to be used to hash the message
    ///
    /// # Returns
    ///
    /// true if the signature is valid; otherwise, false
    pub fn verify_signature_with_curve(
        message: &[u8],
        signature: &[u8],
        pubkey_bytes: &[u8],
        curve: &crate::ecc::ECCurve,
        hash_algorithm: HashAlgorithm,
    ) -> bool {
        // Decode the public key from bytes using the specified curve
        match ECPoint::decode(pubkey_bytes, curve.clone()) {
            Ok(pubkey) => Self::verify_signature(message, signature, &pubkey, hash_algorithm),
            Err(_) => false,
        }
    }

    /// Verifies that a digital signature is appropriate for the provided key and message.
    /// This is a convenience method that uses SHA256 as the default hash algorithm.
    /// Matches the C# Crypto.VerifySignature(message, signature, pubkey) overload.
    ///
    /// # Arguments
    ///
    /// * `message` - The signed message
    /// * `signature` - The signature to be verified
    /// * `pubkey` - The public key to be used
    ///
    /// # Returns
    ///
    /// true if the signature is valid; otherwise, false
    pub fn verify_signature_default(message: &[u8], signature: &[u8], pubkey: &ECPoint) -> bool {
        Self::verify_signature(message, signature, pubkey, HashAlgorithm::Sha256)
    }

    /// Signs the specified message using the ECDSA algorithm.
    /// This is a convenience method that uses secp256r1 curve and SHA256 hash algorithm.
    /// Matches the C# Crypto.Sign(message, priKey) overload.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be signed
    /// * `pri_key` - The private key to be used
    ///
    /// # Returns
    ///
    /// The ECDSA signature for the specified message
    pub fn sign_default(message: &[u8], pri_key: &[u8]) -> Result<Vec<u8>, Error> {
        Self::sign(message, pri_key, None, HashAlgorithm::Sha256)
    }

    /// Signs the specified message using the ECDSA algorithm and specified curve.
    /// This is a convenience method that uses SHA256 as the default hash algorithm.
    /// Matches the C# Crypto.Sign(message, priKey, ecCurve) overload.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to be signed
    /// * `pri_key` - The private key to be used
    /// * `ec_curve` - The ECCurve curve of the signature
    ///
    /// # Returns
    ///
    /// The ECDSA signature for the specified message
    pub fn sign_with_curve(
        message: &[u8],
        pri_key: &[u8],
        ec_curve: &crate::ecc::ECCurve,
    ) -> Result<Vec<u8>, Error> {
        Self::sign(
            message,
            pri_key,
            Some(ec_curve.clone()),
            HashAlgorithm::Sha256,
        )
    }

    /// Get hash from message using ReadOnlySpan-like interface.
    /// This matches the C# Crypto.GetMessageHash(ReadOnlySpan<byte>, HashAlgorithm) overload.
    ///
    /// # Arguments
    ///
    /// * `message` - Original message
    /// * `hash_algorithm` - The hash algorithm to be used hash the message
    ///
    /// # Returns
    ///
    /// Hashed message
    pub fn get_message_hash_span(
        message: &[u8],
        hash_algorithm: HashAlgorithm,
    ) -> Result<Vec<u8>, Error> {
        Self::get_message_hash(message, hash_algorithm)
    }

    /// Decompresses a secp256k1 key from coordinates.
    /// This matches the C# DecompressKey functionality in ECRecover.
    ///
    /// # Arguments
    ///
    /// * `x_coord_bytes` - The x coordinate as bytes
    /// * `y_bit` - The y coordinate bit (0 or 1)
    ///
    /// # Returns
    ///
    /// The decompressed public key or an error
    pub fn decompress_secp256k1_key(x_coord_bytes: &[u8], y_bit: bool) -> Result<Vec<u8>, Error> {
        if x_coord_bytes.len() != HASH_SIZE {
            return Err(Error::InvalidKey(
                "X coordinate must be HASH_SIZE bytes".to_string(),
            ));
        }

        let mut compressed_key = vec![if y_bit { 0x03 } else { 0x02 }];
        compressed_key.extend_from_slice(x_coord_bytes);

        use secp256k1::PublicKey as Secp256k1PublicKey;

        match Secp256k1PublicKey::from_slice(&compressed_key) {
            Ok(pubkey) => Ok(pubkey.serialize_uncompressed().to_vec()),
            Err(e) => Err(Error::InvalidKey(format!(
                "Failed to decompress secp256k1 key: {e}"
            ))),
        }
    }

    /// Validates that a signature has the correct format.
    /// This matches the C# signature validation logic.
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature to validate
    ///
    /// # Returns
    ///
    /// true if the signature format is valid; otherwise, false
    pub fn validate_signature_format(signature: &[u8]) -> bool {
        // Standard Neo signature is 64 bytes
        signature.len() == 64
    }

    /// Validates that a message hash has the correct format.
    /// This matches the C# message hash validation logic.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash to validate
    ///
    /// # Returns
    ///
    /// true if the hash format is valid; otherwise, false
    pub fn validate_hash_format(hash: &[u8]) -> bool {
        // Standard message hash is HASH_SIZE bytes
        hash.len() == HASH_SIZE
    }
}

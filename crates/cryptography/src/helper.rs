//! Cryptographic helper functions for Neo.
//!
//! This module provides various cryptographic utility functions.

use crate::Error;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use neo_config::HASH_SIZE;
use p256::ecdh::diffie_hellman;
use p256::PublicKey as P256PublicKey;
use p256::SecretKey as P256SecretKey;
use rand::Rng;
use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

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
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    if signature.len() != 64 {
        return false;
    }

    let secp = Secp256k1::verification_only();

    // Create message hash
    let message_hash = Sha256::digest(message);
    let message = match Message::from_digest_slice(&message_hash) {
        Ok(m) => m,
        Err(_) => return false,
    };

    // Parse public key
    let public_key = match PublicKey::from_slice(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // Parse signature
    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(signature);

    let signature = match secp256k1::ecdsa::Signature::from_compact(&sig_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Verify signature
    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
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
    let secp = Secp256k1::signing_only();

    // Create message hash
    let message_hash = Sha256::digest(message);
    let message = Message::from_digest_slice(&message_hash)
        .map_err(|e| Error::InvalidFormat(format!("Invalid message: {e}")))?;

    // Parse private key
    let secret_key = SecretKey::from_slice(private_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid private key: {e}")))?;

    // Sign message
    let signature = secp.sign_ecdsa(&message, &secret_key);

    // Return signature bytes
    Ok(signature.serialize_compact().to_vec())
}

/// Signs a message using ECDSA and returns a recoverable signature.
///
/// # Arguments
///
/// * `message` - The message to sign
/// * `private_key` - The private key to sign with
///
/// # Returns
///
/// The recoverable signature or an error
pub fn sign_message_recoverable(message: &[u8], private_key: &[u8]) -> Result<Vec<u8>, Error> {
    let secp = Secp256k1::signing_only();

    // Create message hash
    let message_hash = Sha256::digest(message);
    let message = Message::from_digest_slice(&message_hash)
        .map_err(|e| Error::InvalidFormat(format!("Invalid message: {e}")))?;

    // Parse private key
    let secret_key = SecretKey::from_slice(private_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid private key: {e}")))?;

    // Sign message
    let signature = secp.sign_ecdsa_recoverable(&message, &secret_key);

    // Return signature bytes with recovery id
    let (recovery_id, signature) = signature.serialize_compact();
    let mut result = Vec::with_capacity(65);
    result.extend_from_slice(&signature);
    result.push(recovery_id.to_i32() as u8);

    Ok(result)
}

/// Recovers a public key from a message and signature.
///
/// # Arguments
///
/// * `message` - The message that was signed
/// * `signature` - The signature with recovery id
///
/// # Returns
///
/// The recovered public key or an error
pub fn recover_public_key(message: &[u8], signature: &[u8]) -> Result<Vec<u8>, Error> {
    if signature.len() != 65 {
        return Err(Error::InvalidSignature(
            "Invalid signature length".to_string(),
        ));
    }

    let secp = Secp256k1::verification_only();

    // Create message hash
    let message_hash = Sha256::digest(message);
    let message = Message::from_digest_slice(&message_hash)
        .map_err(|e| Error::InvalidFormat(format!("Invalid message: {e}")))?;

    // Parse signature
    let recovery_id = RecoveryId::from_i32(signature[64] as i32)
        .map_err(|e| Error::InvalidSignature(format!("Invalid recovery ID: {e}")))?;

    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(&signature[..64]);

    let signature = RecoverableSignature::from_compact(&sig_bytes, recovery_id)
        .map_err(|e| Error::InvalidSignature(format!("Invalid signature: {e}")))?;

    // Recover public key
    let public_key = secp
        .recover_ecdsa(&message, &signature)
        .map_err(|_e| Error::VerificationFailed)?;

    Ok(public_key.serialize().to_vec())
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
    let secp = Secp256k1::signing_only();

    // Parse private key
    let secret_key = SecretKey::from_slice(private_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid private key: {e}")))?;

    // Derive public key
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    Ok(public_key.serialize().to_vec())
}

/// Computes the RIPEMD-160 hash of a public key.
///
/// # Arguments
///
/// * `public_key` - The public key
///
/// # Returns
///
/// The RIPEMD-160 hash of the public key
pub fn public_key_to_script_hash(public_key: &[u8]) -> Vec<u8> {
    crate::hash::hash160(public_key).to_vec()
}

/// Generates a random private key.
///
/// # Returns
///
/// A random private key
pub fn generate_private_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key_bytes = [0u8; HASH_SIZE];
    for byte in &mut key_bytes {
        *byte = rng.gen();
    }
    key_bytes.to_vec()
}

/// Encrypts data using AES-256-GCM.
/// This matches the C# Helper.AES256Encrypt implementation.
///
/// # Arguments
///
/// * `plain_data` - The plaintext data to encrypt
/// * `key` - The encryption key (must be HASH_SIZE bytes)
/// * `nonce` - The nonce (must be 12 bytes)
/// * `associated_data` - Optional associated data for authentication
///
/// # Returns
///
/// The encrypted data (nonce + ciphertext + tag) or an error
pub fn aes256_encrypt(
    plain_data: &[u8],
    key: &[u8],
    nonce: &[u8],
    associated_data: Option<&[u8]>,
) -> Result<Vec<u8>, Error> {
    if key.len() != HASH_SIZE {
        return Err(Error::InvalidKey("Key must be HASH_SIZE bytes".to_string()));
    }
    if nonce.len() != 12 {
        return Err(Error::InvalidFormat("Nonce must be 12 bytes".to_string()));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| Error::InvalidKey(format!("Invalid key: {e}")))?;

    let nonce = Nonce::from_slice(nonce);

    let ciphertext = cipher
        .encrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: plain_data,
                aad: associated_data.unwrap_or(&[]),
            },
        )
        .map_err(|e| Error::InvalidFormat(format!("Encryption failed: {e}")))?;

    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(nonce);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypts data using AES-256-GCM.
/// This matches the C# Helper.AES256Decrypt implementation.
///
/// # Arguments
///
/// * `encrypted_data` - The encrypted data (nonce + ciphertext + tag)
/// * `key` - The decryption key (must be HASH_SIZE bytes)
/// * `associated_data` - Optional associated data for authentication
///
/// # Returns
///
/// The decrypted plaintext data or an error
pub fn aes256_decrypt(
    encrypted_data: &[u8],
    key: &[u8],
    associated_data: Option<&[u8]>,
) -> Result<Vec<u8>, Error> {
    if key.len() != HASH_SIZE {
        return Err(Error::InvalidKey("Key must be HASH_SIZE bytes".to_string()));
    }
    if encrypted_data.len() < 12 + 16 {
        return Err(Error::InvalidFormat("Encrypted data too short".to_string()));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| Error::InvalidKey(format!("Invalid key: {e}")))?;

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: ciphertext,
                aad: associated_data.unwrap_or(&[]),
            },
        )
        .map_err(|e| Error::InvalidFormat(format!("Decryption failed: {e}")))?;

    Ok(plaintext)
}

/// Derives a shared key using ECDH.
/// This matches the C# Helper.ECDHDeriveKey implementation.
///
/// # Arguments
///
/// * `local_private_key` - The local private key
/// * `remote_public_key` - The remote public key
///
/// # Returns
///
/// The derived shared key or an error
pub fn ecdh_derive_key(
    local_private_key: &[u8],
    remote_public_key: &[u8],
) -> Result<Vec<u8>, Error> {
    // Parse local private key
    let local_secret = P256SecretKey::from_slice(local_private_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid local private key: {e}")))?;

    // Parse remote public key
    let remote_pubkey = P256PublicKey::from_sec1_bytes(remote_public_key)
        .map_err(|e| Error::InvalidKey(format!("Invalid remote public key: {e}")))?;

    // Perform ECDH
    let shared_secret = diffie_hellman(local_secret.to_nonzero_scalar(), remote_pubkey.as_affine());

    let shared_key = crate::hash::sha256(shared_secret.raw_secret_bytes());

    Ok(shared_key.to_vec())
}

/// Rotates a HASH_SIZE-bit value left by the specified number of bits.
/// This matches the C# Helper.RotateLeft(uint, int) implementation.
///
/// # Arguments
///
/// * `value` - The value to rotate
/// * `offset` - The number of bits to rotate by
///
/// # Returns
///
/// The rotated value
pub fn rotate_left_u32(value: u32, offset: i32) -> u32 {
    let offset = offset as u32 & 31; // Ensure offset is in range [0..31]
    (value << offset) | (value >> (32 - offset))
}

/// Rotates a 64-bit value left by the specified number of bits.
/// This matches the C# Helper.RotateLeft(ulong, int) implementation.
///
/// # Arguments
///
/// * `value` - The value to rotate
/// * `offset` - The number of bits to rotate by
///
/// # Returns
///
/// The rotated value
pub fn rotate_left_u64(value: u64, offset: i32) -> u64 {
    let offset = offset as u64 & 63; // Ensure offset is in range [0..63]
    (value << offset) | (value >> (64 - offset))
}

/// Computes the hash value using SHA-256 for a slice of byte array.
/// This matches the C# Helper.Sha256(byte[], int, int) implementation.
///
/// # Arguments
///
/// * `value` - The input data
/// * `offset` - The offset into the byte array
/// * `count` - The number of bytes to hash
///
/// # Returns
///
/// The SHA-256 hash
pub fn sha256_slice(value: &[u8], offset: usize, count: usize) -> Result<Vec<u8>, Error> {
    if offset + count > value.len() {
        return Err(Error::InvalidFormat(
            "Offset + count exceeds array length".to_string(),
        ));
    }

    let slice = &value[offset..offset + count];
    Ok(crate::hash::sha256(slice).to_vec())
}

/// Computes the hash value using SHA-MAX_TRANSACTIONS_PER_BLOCK for a slice of byte array.
/// This matches the C# Helper.Sha512(byte[], int, int) implementation.
///
/// # Arguments
///
/// * `value` - The input data
/// * `offset` - The offset into the byte array
/// * `count` - The number of bytes to hash
///
/// # Returns
///
/// The SHA-MAX_TRANSACTIONS_PER_BLOCK hash
pub fn sha512_slice(value: &[u8], offset: usize, count: usize) -> Result<Vec<u8>, Error> {
    if offset + count > value.len() {
        return Err(Error::InvalidFormat(
            "Offset + count exceeds array length".to_string(),
        ));
    }

    let slice = &value[offset..offset + count];
    Ok(crate::hash::sha512(slice).to_vec())
}

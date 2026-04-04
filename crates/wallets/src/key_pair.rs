//! Key pair implementation for Neo wallets.
//!
//! This module provides cryptographic key pair functionality,
//! converted from the C# Neo KeyPair class (@neo-sharp/src/Neo/Wallets/KeyPair.cs).

use crate::{Error, Result};
use aes::Aes256;
use base64::Engine;
use cbc::{
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit},
    Decryptor, Encryptor,
};
use neo_config::HASH_SIZE;
use neo_core::UInt160;
use neo_cryptography::{ECCurve, ECDsa, ECC};
use rand::RngCore;
use scrypt::Params;
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A cryptographic key pair for Neo accounts.
/// This matches the C# KeyPair class functionality.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct KeyPair {
    private_key: [u8; HASH_SIZE],
    public_key: Vec<u8>,
    compressed_public_key: Vec<u8>,
}

impl KeyPair {
    /// Creates a new random key pair.
    pub fn generate() -> Result<Self> {
        let mut private_key = [0u8; HASH_SIZE];
        rand::thread_rng().fill_bytes(&mut private_key);
        Self::from_private_key(&private_key)
    }

    /// Creates a key pair from a private key.
    pub fn from_private_key(private_key: &[u8]) -> Result<Self> {
        if private_key.len() != HASH_SIZE {
            return Err(Error::InvalidPrivateKey);
        }

        let mut key_bytes = [0u8; HASH_SIZE];
        key_bytes.copy_from_slice(private_key);

        // Generate public key from private key
        let public_key = ECC::generate_public_key(&key_bytes)?;
        let compressed_public_key = ECC::compress_public_key(&public_key)?;

        Ok(Self {
            private_key: key_bytes,
            public_key,
            compressed_public_key,
        })
    }

    /// Creates a key pair from a WIF (Wallet Import Format) string.
    pub fn from_wif(wif: &str) -> Result<Self> {
        let private_key = Self::decode_wif(wif)?;
        Self::from_private_key(&private_key)
    }

    /// Creates a key pair from a NEP-2 encrypted private key.
    /// The encrypted_key should be base64-encoded NEP-2 data.
    pub fn from_nep2(encrypted_key: &[u8], password: &str) -> Result<Self> {
        // First try to decode as base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encrypted_key)
            .map_err(|_| Error::InvalidNep2Key)?;

        let private_key = Self::decrypt_nep2(&decoded, password)?;
        Self::from_private_key(&private_key)
    }

    /// Creates a key pair from a NEP-2 encrypted private key string.
    /// The encrypted_key should be a base64-encoded NEP-2 string.
    pub fn from_nep2_string(encrypted_key: &str, password: &str) -> Result<Self> {
        Self::from_nep2(encrypted_key.as_bytes(), password)
    }

    /// Gets the private key.
    pub fn private_key(&self) -> [u8; HASH_SIZE] {
        self.private_key
    }

    /// Gets the public key (uncompressed).
    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }

    /// Gets the compressed public key.
    pub fn compressed_public_key(&self) -> Vec<u8> {
        self.compressed_public_key.clone()
    }

    /// Gets the public key as an ECPoint.
    pub fn get_public_key_point(&self) -> Result<neo_cryptography::ECPoint> {
        let curve = ECCurve::secp256r1();
        neo_cryptography::ECPoint::decode_compressed(&self.compressed_public_key, curve)
            .map_err(|e| Error::Other(format!("Failed to create ECPoint: {}", e)))
    }

    /// Gets the script hash for this key pair.
    /// This matches the C# KeyPair.PublicKeyHash property.
    pub fn get_script_hash(&self) -> UInt160 {
        UInt160::from_script(&self.compressed_public_key)
    }

    /// Gets the verification script for this key pair.
    pub fn get_verification_script(&self) -> Vec<u8> {
        // Standard single-signature verification script
        let mut script = Vec::new();
        script.push(0x0c); // PUSHDATA1
        script.push(self.compressed_public_key.len() as u8);
        script.extend_from_slice(&self.compressed_public_key);
        script.push(0x41); // SYSCALL
        script.extend_from_slice(b"System.Crypto.CheckWitness");
        script
    }

    /// Signs data with this key pair.
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        ECDsa::sign(data, &self.private_key)
            .map_err(|e| Error::Other(format!("Signing failed: {}", e)))
    }

    /// Verifies a signature against data.
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        ECDsa::verify(data, signature, &self.public_key)
            .map_err(|e| Error::Other(format!("Verification failed: {}", e)))
    }

    /// Exports the key pair to WIF format.
    pub fn to_wif(&self) -> String {
        Self::encode_wif(&self.private_key)
    }

    /// Exports the key pair to NEP-2 format.
    pub fn to_nep2(&self, password: &str) -> Result<String> {
        let encrypted = Self::encrypt_nep2(&self.private_key, password)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(encrypted))
    }

    /// Decodes a WIF string to a private key.
    fn decode_wif(wif: &str) -> Result<[u8; HASH_SIZE]> {
        let decoded = bs58::decode(wif)
            .into_vec()
            .map_err(|e| Error::Base58Decode(e.to_string()))?;

        // Verify checksum manually
        if decoded.len() < 4 {
            return Err(Error::InvalidWif);
        }

        let (data, checksum) = decoded.split_at(decoded.len() - 4);
        let computed_checksum = &neo_cryptography::hash::hash256(data)[0..4];
        if checksum != computed_checksum {
            return Err(Error::InvalidWif);
        }

        if decoded.len() != 38 {
            return Err(Error::InvalidWif);
        }

        // Check version byte
        if data[0] != 0x80 {
            return Err(Error::InvalidWif);
        }

        // Check compressed flag
        if data[33] != 0x01 {
            return Err(Error::InvalidWif);
        }

        let mut private_key = [0u8; HASH_SIZE];
        private_key.copy_from_slice(&data[1..33]);
        Ok(private_key)
    }

    /// Encodes a private key to WIF format.
    fn encode_wif(private_key: &[u8; HASH_SIZE]) -> String {
        let mut data = Vec::with_capacity(37);
        data.push(0x80); // Version byte for mainnet
        data.extend_from_slice(private_key);
        data.push(0x01); // Compressed flag

        // Add checksum manually
        let checksum = &neo_cryptography::hash::hash256(&data)[0..4];
        data.extend_from_slice(checksum);

        bs58::encode(data).into_string()
    }

    /// Encrypts a private key using NEP-2 standard.
    fn encrypt_nep2(private_key: &[u8; HASH_SIZE], password: &str) -> Result<Vec<u8>> {
        // NEP-2 parameters
        let n = 16384; // CPU cost
        let r = 8; // Memory cost
        let p = 8; // Parallelization

        // Generate address hash
        let address =
            UInt160::from_script(&Self::get_verification_script_for_key(private_key)).to_address();
        let address_hash = &neo_cryptography::hash::sha256(address.as_bytes())[0..4];

        // Derive key using scrypt
        let n: u32 = n;
        let params = Params::new(n.trailing_zeros() as u8, r, p, 64)
            .map_err(|e| Error::Scrypt(e.to_string()))?;

        let mut derived_key = [0u8; 64];
        scrypt::scrypt(password.as_bytes(), address_hash, &params, &mut derived_key)
            .map_err(|e| Error::Scrypt(e.to_string()))?;

        // Split derived key
        let derived_half1 = &derived_key[0..HASH_SIZE];
        let derived_half2 = &derived_key[32..64];

        // XOR private key with derived_half1
        let mut xor_key = [0u8; HASH_SIZE];
        for i in 0..HASH_SIZE {
            xor_key[i] = private_key[i] ^ derived_half1[i];
        }

        let cipher = Encryptor::<Aes256>::new(derived_half2.into(), &[0u8; 16].into());
        let mut buffer = xor_key.to_vec();
        buffer.resize(HASH_SIZE, 0); // Ensure exactly HASH_SIZE bytes
        let encrypted = cipher
            .encrypt_padded_mut::<cbc::cipher::block_padding::NoPadding>(&mut buffer, HASH_SIZE)
            .map_err(|e| Error::Aes(e.to_string()))?;
        let encrypted = encrypted.to_vec();

        let mut result = Vec::with_capacity(39);
        result.extend_from_slice(b"\x01\x42"); // NEP-2 prefix
        result.push(0xe0); // Flags
        result.extend_from_slice(address_hash);
        result.extend_from_slice(&encrypted);

        Ok(result)
    }

    /// Decrypts a NEP-2 encrypted private key.
    fn decrypt_nep2(encrypted_key: &[u8], password: &str) -> Result<[u8; HASH_SIZE]> {
        if encrypted_key.len() != 39 {
            return Err(Error::InvalidNep2Key);
        }

        if &encrypted_key[0..2] != b"\x01\x42" {
            return Err(Error::InvalidNep2Key);
        }

        let flags = encrypted_key[2];
        let address_hash = &encrypted_key[3..7];
        let encrypted_data = &encrypted_key[7..39];

        // NEP-2 parameters
        let n = 16384;
        let r = 8;
        let p = 8;

        // Derive key using scrypt
        let n: u32 = n;
        let params = Params::new(n.trailing_zeros() as u8, r, p, 64)
            .map_err(|e| Error::Scrypt(e.to_string()))?;

        let mut derived_key = [0u8; 64];
        scrypt::scrypt(password.as_bytes(), address_hash, &params, &mut derived_key)
            .map_err(|e| Error::Scrypt(e.to_string()))?;

        let derived_half1 = &derived_key[0..HASH_SIZE];
        let derived_half2 = &derived_key[32..64];

        let cipher = Decryptor::<Aes256>::new(derived_half2.into(), &[0u8; 16].into());
        let mut buffer = encrypted_data.to_vec();
        let decrypted = cipher
            .decrypt_padded_mut::<cbc::cipher::block_padding::NoPadding>(&mut buffer)
            .map_err(|e| Error::Aes(e.to_string()))?;

        // XOR with derived_half1 to get private key
        let mut private_key = [0u8; HASH_SIZE];
        for i in 0..HASH_SIZE {
            private_key[i] = decrypted[i] ^ derived_half1[i];
        }

        // Verify by checking address hash
        let verification_script = Self::get_verification_script_for_key(&private_key);
        let script_hash = UInt160::from_script(&verification_script);
        let address = script_hash.to_address();
        let computed_hash = &neo_cryptography::hash::sha256(address.as_bytes())[0..4];

        if computed_hash != address_hash {
            return Err(Error::InvalidPassword);
        }

        Ok(private_key)
    }

    /// Gets verification script for a private key (helper function).
    fn get_verification_script_for_key(private_key: &[u8; HASH_SIZE]) -> Vec<u8> {
        let public_key = ECC::generate_public_key(private_key).expect("Operation failed");
        let compressed = ECC::compress_public_key(&public_key).expect("Operation failed");

        let mut script = Vec::new();
        script.push(0x0c); // PUSHDATA1
        script.push(compressed.len() as u8);
        script.extend_from_slice(&compressed);
        script.push(0x41); // SYSCALL
        script.extend_from_slice(b"System.Crypto.CheckWitness");
        script
    }
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.compressed_public_key))
    }
}

impl PartialEq for KeyPair {
    fn eq(&self, other: &Self) -> bool {
        self.private_key == other.private_key
    }
}

impl Eq for KeyPair {}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use neo_config::HASH_SIZE;

    #[test]
    fn test_key_pair_generation() {
        let key_pair = KeyPair::generate().unwrap();
        assert_eq!(key_pair.private_key().len(), HASH_SIZE);
        assert!(!key_pair.public_key().is_empty());
        assert!(!key_pair.compressed_public_key().is_empty());
    }

    #[test]
    fn test_wif_round_trip() {
        let key_pair = KeyPair::generate().unwrap();
        let wif = key_pair.to_wif();
        let restored = KeyPair::from_wif(&wif).unwrap();
        assert_eq!(key_pair.private_key(), restored.private_key());
    }

    #[test]
    fn test_sign_verify() {
        let key_pair = KeyPair::generate().unwrap();
        let data = b"test data";
        let signature = key_pair.sign(data).unwrap();
        assert!(key_pair.verify(data, &signature).unwrap());
    }
}

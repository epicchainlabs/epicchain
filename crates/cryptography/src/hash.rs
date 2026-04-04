//! Hash functions for Neo cryptography.
//!
//! This module provides hash functions commonly used in the Neo blockchain,
//! including SHA-256, RIPEMD-160, and Neo-specific hash combinations.

use neo_config::{ADDRESS_SIZE, HASH_SIZE};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256, Sha512};

/// Computes SHA-256 hash of the input data.
pub fn sha256(data: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes SHA-MAX_TRANSACTIONS_PER_BLOCK hash of the input data.
/// This matches the C# Neo Sha512 implementation exactly.
pub fn sha512(data: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes RIPEMD-160 hash of the input data.
pub fn ripemd160(data: &[u8]) -> [u8; ADDRESS_SIZE] {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes Hash160 (RIPEMD-160 of SHA-256) of the input data.
/// This is commonly used for Neo addresses.
pub fn hash160(data: &[u8]) -> [u8; ADDRESS_SIZE] {
    let sha256_hash = sha256(data);
    ripemd160(&sha256_hash)
}

/// Computes Hash256 (double SHA-256) of the input data.
/// This is commonly used for Neo transaction and block hashes.
pub fn hash256(data: &[u8]) -> [u8; HASH_SIZE] {
    let first_hash = sha256(data);
    sha256(&first_hash)
}

/// Computes Keccak-256 hash of the input data.
/// This is used for some Neo smart contract operations.
pub fn keccak256(data: &[u8]) -> [u8; HASH_SIZE] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes SHA-1 hash of the input data.
/// This is used for some legacy Neo operations.
pub fn sha1(data: &[u8]) -> [u8; ADDRESS_SIZE] {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes MD5 hash of the input data.
/// This is used for some legacy Neo operations (not recommended for security).
pub fn md5(data: &[u8]) -> [u8; 16] {
    md5::compute(data).into()
}

/// Computes BLAKE2b hash of the input data.
/// This is used for some Neo smart contract operations.
pub fn blake2b(data: &[u8]) -> [u8; 64] {
    use blake2::{Blake2b512, Digest};
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes BLAKE2s hash of the input data.
/// This is used for some Neo smart contract operations.
pub fn blake2s(data: &[u8]) -> [u8; HASH_SIZE] {
    use blake2::{Blake2s256, Digest};
    let mut hasher = Blake2s256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Hash function trait for generic hashing operations.
pub trait HashFunction {
    /// The output size of the hash function in bytes.
    const OUTPUT_SIZE: usize;

    /// Computes the hash of the input data.
    fn hash(&self, data: &[u8]) -> Vec<u8>;
}

/// SHA-256 hash function implementation.
pub struct Sha256Hash;

impl HashFunction for Sha256Hash {
    const OUTPUT_SIZE: usize = HASH_SIZE;

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        sha256(data).to_vec()
    }
}

/// RIPEMD-160 hash function implementation.
pub struct Ripemd160Hash;

impl HashFunction for Ripemd160Hash {
    const OUTPUT_SIZE: usize = ADDRESS_SIZE;

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        ripemd160(data).to_vec()
    }
}

/// Hash160 hash function implementation.
pub struct Hash160;

impl HashFunction for Hash160 {
    const OUTPUT_SIZE: usize = ADDRESS_SIZE;

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        hash160(data).to_vec()
    }
}

/// Hash256 hash function implementation.
pub struct Hash256;

impl HashFunction for Hash256 {
    const OUTPUT_SIZE: usize = HASH_SIZE;

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        hash256(data).to_vec()
    }
}

/// Merkle tree hash computation.
pub fn merkle_hash(left: &[u8], right: &[u8]) -> [u8; HASH_SIZE] {
    let mut combined = Vec::with_capacity(left.len() + right.len());
    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    hash256(&combined)
}

/// Computes the checksum for Neo addresses.
pub fn address_checksum(data: &[u8]) -> [u8; 4] {
    let hash = hash256(data);
    [hash[0], hash[1], hash[2], hash[3]]
}

/// Verifies the checksum for Neo addresses.
pub fn verify_checksum(data: &[u8], checksum: &[u8]) -> bool {
    let computed_checksum = address_checksum(data);
    computed_checksum == checksum
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use hex;
    use neo_config::{ADDRESS_SIZE, HASH_SIZE};

    #[test]
    fn test_sha256() {
        let data = b"hello world";
        let hash = sha256(data);
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert_eq!(hex::encode(hash), expected);
    }

    #[test]
    fn test_hash160() {
        let data = b"hello world";
        let hash = hash160(data);
        assert_eq!(hash.len(), ADDRESS_SIZE);
    }

    #[test]
    fn test_hash256() {
        let data = b"hello world";
        let hash = hash256(data);
        assert_eq!(hash.len(), HASH_SIZE);
    }

    #[test]
    fn test_merkle_hash() {
        let left = [1u8; HASH_SIZE];
        let right = [2u8; HASH_SIZE];
        let hash = merkle_hash(&left, &right);
        assert_eq!(hash.len(), HASH_SIZE);
    }

    #[test]
    fn test_address_checksum() {
        let data = b"test address data";
        let checksum = address_checksum(data);
        assert_eq!(checksum.len(), 4);
        assert!(verify_checksum(data, &checksum));
    }
}

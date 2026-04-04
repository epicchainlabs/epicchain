//! Hasher implementation for Neo.
//!
//! This module provides hash functions used in the Neo blockchain.

use crate::hash_algorithm::HashAlgorithm;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::io;

/// Provides hash functions for Neo.
pub struct Hasher;

impl Hasher {
    /// Computes the hash of the given data using the specified algorithm.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The hash algorithm to use
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The hash of the data
    pub fn hash(algorithm: HashAlgorithm, data: &[u8]) -> Vec<u8> {
        match algorithm {
            HashAlgorithm::Sha256 => Self::sha256(data),
            HashAlgorithm::Sha512 => Self::sha512(data),
            HashAlgorithm::Keccak256 => crate::hash::keccak256(data).to_vec(),
        }
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
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Computes the SHA-MAX_TRANSACTIONS_PER_BLOCK hash of the given data.
    /// This matches the C# Neo Sha512 implementation exactly.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The SHA-MAX_TRANSACTIONS_PER_BLOCK hash of the data
    pub fn sha512(data: &[u8]) -> Vec<u8> {
        use sha2::Sha512;
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
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
        let mut hasher = Ripemd160::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Computes the Hash160 (RIPEMD-160(SHA-256)) of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The Hash160 of the data
    pub fn hash160(data: &[u8]) -> Vec<u8> {
        let sha256_hash = Self::sha256(data);
        Self::ripemd160(&sha256_hash)
    }

    /// Computes the Hash256 (SHA-256(SHA-256)) of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The Hash256 of the data
    pub fn hash256(data: &[u8]) -> Vec<u8> {
        let sha256_hash = Self::sha256(data);
        Self::sha256(&sha256_hash)
    }

    /// Computes the Murmur32 hash of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    /// * `seed` - The seed for the hash function
    ///
    /// # Returns
    ///
    /// The Murmur32 hash of the data
    pub fn murmur32(data: &[u8], seed: u32) -> Vec<u8> {
        let hash = crate::murmur::murmur32(data, seed);
        hash.to_le_bytes().to_vec()
    }

    /// Computes the Murmur128 hash of the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    /// * `seed` - The seed for the hash function
    ///
    /// # Returns
    ///
    /// The Murmur128 hash of the data
    pub fn murmur128(data: &[u8], seed: u32) -> Vec<u8> {
        let (hash1, hash2) = crate::murmur::murmur128(data, seed);
        let mut result = Vec::with_capacity(16);
        result.extend_from_slice(&hash1.to_le_bytes());
        result.extend_from_slice(&hash2.to_le_bytes());
        result
    }

    /// Computes the hash of the given data using the specified algorithm and writes it to the output.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The hash algorithm to use
    /// * `data` - The data to hash
    /// * `output` - The output to write the hash to
    ///
    /// # Returns
    ///
    /// The number of bytes written or an error
    pub fn hash_to_writer<W: io::Write>(
        algorithm: HashAlgorithm,
        data: &[u8],
        mut output: W,
    ) -> io::Result<usize> {
        let hash = Self::hash(algorithm, data);
        output.write_all(&hash)?;
        Ok(hash.len())
    }

    /// Verifies that the hash of the given data matches the expected hash.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The hash algorithm to use
    /// * `data` - The data to hash
    /// * `expected` - The expected hash
    ///
    /// # Returns
    ///
    /// `true` if the hash of the data matches the expected hash, `false` otherwise
    pub fn verify(algorithm: HashAlgorithm, data: &[u8], expected: &[u8]) -> bool {
        let hash = Self::hash(algorithm, data);
        hash == expected
    }
}

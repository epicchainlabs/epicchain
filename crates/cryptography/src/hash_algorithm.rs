//! Hash algorithm enum for Neo.
//!
//! This module defines the hash algorithms used in the Neo blockchain.

use neo_config::HASH_SIZE;
use std::fmt;
use std::str::FromStr;

/// Hash algorithms used in Neo.
/// This matches the C# Neo.Cryptography.HashAlgorithm enum exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HashAlgorithm {
    /// The SHA256 hash algorithm
    Sha256 = 0x00,

    /// The Keccak256 hash algorithm
    Keccak256 = 0x01,

    /// The SHA512 hash algorithm  
    Sha512 = 0x02,
}

impl HashAlgorithm {
    /// Returns the size of the hash in bytes.
    pub fn size(&self) -> usize {
        match self {
            HashAlgorithm::Sha256 => HASH_SIZE,
            HashAlgorithm::Keccak256 => HASH_SIZE,
            HashAlgorithm::Sha512 => 64,
        }
    }

    /// Returns the name of the hash algorithm.
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "SHA256",
            HashAlgorithm::Keccak256 => "KECCAK256",
            HashAlgorithm::Sha512 => "SHA512",
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Error type for hash algorithm parsing.
#[derive(Debug, thiserror::Error)]
#[error("Unknown hash algorithm: {0}")]
pub struct UnknownHashAlgorithm(String);

impl FromStr for HashAlgorithm {
    type Err = UnknownHashAlgorithm;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SHA256" => Ok(HashAlgorithm::Sha256),
            "KECCAK256" => Ok(HashAlgorithm::Keccak256),
            "SHA512" => Ok(HashAlgorithm::Sha512),
            _ => Err(UnknownHashAlgorithm(s.to_string())),
        }
    }
}

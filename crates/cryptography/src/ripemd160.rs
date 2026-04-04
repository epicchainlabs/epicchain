//! RIPEMD-160 implementation for Neo.
//!
//! This module provides a wrapper around the ripemd crate for RIPEMD-160 hashing.

use neo_config::ADDRESS_SIZE;
use ripemd::{Digest, Ripemd160};

/// Computes the RIPEMD-160 hash of the given data.
///
/// # Arguments
///
/// * `data` - The data to hash
///
/// # Returns
///
/// The RIPEMD-160 hash of the data
pub fn ripemd160(data: &[u8]) -> [u8; ADDRESS_SIZE] {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// A wrapper around the ripemd crate for RIPEMD-160 hashing.
pub struct RIPEMD160Managed;

impl Default for RIPEMD160Managed {
    fn default() -> Self {
        Self::new()
    }
}

impl RIPEMD160Managed {
    /// Creates a new RIPEMD-160 hasher.
    pub fn new() -> Self {
        Self
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
    pub fn hash(&self, data: &[u8]) -> [u8; ADDRESS_SIZE] {
        ripemd160(data)
    }

    /// Computes the RIPEMD-160 hash of the given data and returns it as a vector.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Returns
    ///
    /// The RIPEMD-160 hash of the data as a vector
    pub fn hash_to_vec(&self, data: &[u8]) -> Vec<u8> {
        self.hash(data).to_vec()
    }
}

//! Base58 encoding and decoding implementation for Neo.
//!
//! This module provides functions for encoding and decoding data in Base58 format,
//! which is commonly used in Neo for representing addresses and other data.
//! This implementation uses the proven `bs58` crate for reliability and C# compatibility.

use crate::Error;
use sha2::{Digest, Sha256};

/// Encodes data to Base58 format.
/// This matches the C# Neo Base58 implementation exactly.
///
/// # Arguments
///
/// * `data` - The data to encode
///
/// # Returns
///
/// The Base58 encoded string
pub fn encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// Decodes a Base58 string to bytes.
/// This matches the C# Neo Base58 implementation exactly.
///
/// # Arguments
///
/// * `s` - The Base58 encoded string
///
/// # Returns
///
/// The decoded data or an error if the string contains invalid characters
pub fn decode(s: &str) -> Result<Vec<u8>, Error> {
    bs58::decode(s)
        .into_vec()
        .map_err(|e| Error::InvalidFormat(format!("Invalid Base58: {e}")))
}

/// Encodes data to Base58Check format (with checksum).
/// This matches the C# Neo Base58CheckEncode implementation exactly.
///
/// # Arguments
///
/// * `data` - The data to encode
///
/// # Returns
///
/// The Base58Check encoded string
pub fn encode_check(data: &[u8]) -> String {
    let mut buffer = Vec::with_capacity(data.len() + 4);
    buffer.extend_from_slice(data);

    let checksum = calculate_checksum(data);
    buffer.extend_from_slice(&checksum);

    bs58::encode(&buffer).into_string()
}

/// Decodes a Base58Check string to bytes, verifying the checksum.
/// This matches the C# Neo Base58CheckDecode implementation exactly.
///
/// # Arguments
///
/// * `s` - The Base58Check encoded string
///
/// # Returns
///
/// The decoded data (without checksum) or an error if the string is invalid or checksum fails
pub fn decode_check(s: &str) -> Result<Vec<u8>, Error> {
    let decoded = bs58::decode(s)
        .into_vec()
        .map_err(|e| Error::InvalidFormat(format!("Invalid Base58: {e}")))?;

    if decoded.len() < 4 {
        return Err(Error::InvalidFormat(
            "Invalid Base58Check string: too short".to_string(),
        ));
    }

    let data_len = decoded.len() - 4;
    let data = &decoded[..data_len];
    let checksum = &decoded[data_len..];

    let calculated_checksum = calculate_checksum(data);

    if checksum != calculated_checksum {
        return Err(Error::VerificationFailed);
    }

    Ok(data.to_vec())
}

/// Calculates a 4-byte checksum for the given data.
/// This matches the C# Neo checksum calculation exactly.
///
/// The checksum is the first 4 bytes of the double SHA256 hash of the data.
///
/// # Arguments
///
/// * `data` - The data to calculate the checksum for
///
/// # Returns
///
/// The 4-byte checksum
fn calculate_checksum(data: &[u8]) -> [u8; 4] {
    let hash1 = Sha256::digest(data);
    let hash2 = Sha256::digest(hash1);

    let mut checksum = [0u8; 4];
    checksum.copy_from_slice(&hash2[..4]);

    checksum
}

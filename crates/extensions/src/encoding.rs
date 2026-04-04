//! Encoding utilities for Neo Extensions

use crate::error::{ExtensionError, ExtensionResult};
use base64::Engine;

/// Encoding utilities
pub struct Encoding;

impl Encoding {
    /// Encode bytes to hex string with 0x prefix
    pub fn to_hex_string(data: &[u8]) -> String {
        format!("0x{}", hex::encode(data))
    }

    /// Encode bytes to hex string without prefix
    pub fn to_hex(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// Decode hex string (with or without 0x prefix)
    pub fn from_hex(hex_str: &str) -> ExtensionResult<Vec<u8>> {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };

        hex::decode(clean_hex).map_err(ExtensionError::from)
    }

    /// Encode bytes to base64 string
    pub fn to_base64(data: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    /// Decode base64 string
    pub fn from_base64(base64_str: &str) -> ExtensionResult<Vec<u8>> {
        base64::engine::general_purpose::STANDARD
            .decode(base64_str)
            .map_err(ExtensionError::from)
    }

    /// Encode bytes to base64 URL-safe string
    pub fn to_base64_url(data: &[u8]) -> String {
        base64::engine::general_purpose::URL_SAFE.encode(data)
    }

    /// Decode base64 URL-safe string
    pub fn from_base64_url(base64_str: &str) -> ExtensionResult<Vec<u8>> {
        base64::engine::general_purpose::URL_SAFE
            .decode(base64_str)
            .map_err(ExtensionError::from)
    }

    /// Convert bytes to UTF-8 string
    pub fn to_utf8_string(data: &[u8]) -> ExtensionResult<String> {
        String::from_utf8(data.to_vec())
            .map_err(|e| ExtensionError::encoding(format!("Invalid UTF-8: {}", e)))
    }

    /// Convert string to bytes
    pub fn from_utf8_string(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    /// Validate hex string format
    pub fn is_valid_hex(hex_str: &str) -> bool {
        let clean_hex = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
            &hex_str[2..]
        } else {
            hex_str
        };

        !clean_hex.is_empty() && clean_hex.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Validate base64 string format
    pub fn is_valid_base64(base64_str: &str) -> bool {
        base64::engine::general_purpose::STANDARD
            .decode(base64_str)
            .is_ok()
    }

    /// Convert integer to little-endian bytes
    pub fn int_to_le_bytes<T>(value: T) -> Vec<u8>
    where
        T: Into<u64>,
    {
        let val: u64 = value.into();
        val.to_le_bytes().to_vec()
    }

    /// Convert little-endian bytes to integer
    pub fn le_bytes_to_int(bytes: &[u8]) -> ExtensionResult<u64> {
        if bytes.len() > 8 {
            return Err(ExtensionError::encoding("Byte array too long for u64"));
        }

        let mut array = [0u8; 8];
        array[..bytes.len()].copy_from_slice(bytes);
        Ok(u64::from_le_bytes(array))
    }

    /// Convert integer to big-endian bytes
    pub fn int_to_be_bytes<T>(value: T) -> Vec<u8>
    where
        T: Into<u64>,
    {
        let val: u64 = value.into();
        val.to_be_bytes().to_vec()
    }

    /// Convert big-endian bytes to integer
    pub fn be_bytes_to_int(bytes: &[u8]) -> ExtensionResult<u64> {
        if bytes.len() > 8 {
            return Err(ExtensionError::encoding("Byte array too long for u64"));
        }

        let mut array = [0u8; 8];
        array[8 - bytes.len()..].copy_from_slice(bytes);
        Ok(u64::from_be_bytes(array))
    }
}

/// Extension trait for encoding operations on byte slices
pub trait EncodingExt {
    /// Convert to hex string with 0x prefix
    fn to_hex_string(&self) -> String;

    /// Convert to hex string without prefix
    fn to_hex(&self) -> String;

    /// Convert to base64 string
    fn to_base64(&self) -> String;

    /// Convert to base64 URL-safe string
    fn to_base64_url(&self) -> String;

    /// Convert to UTF-8 string
    fn to_utf8_string(&self) -> ExtensionResult<String>;
}

impl EncodingExt for [u8] {
    fn to_hex_string(&self) -> String {
        Encoding::to_hex_string(self)
    }

    fn to_hex(&self) -> String {
        Encoding::to_hex(self)
    }

    fn to_base64(&self) -> String {
        Encoding::to_base64(self)
    }

    fn to_base64_url(&self) -> String {
        Encoding::to_base64_url(self)
    }

    fn to_utf8_string(&self) -> ExtensionResult<String> {
        Encoding::to_utf8_string(self)
    }
}

impl EncodingExt for Vec<u8> {
    fn to_hex_string(&self) -> String {
        Encoding::to_hex_string(self)
    }

    fn to_hex(&self) -> String {
        Encoding::to_hex(self)
    }

    fn to_base64(&self) -> String {
        Encoding::to_base64(self)
    }

    fn to_base64_url(&self) -> String {
        Encoding::to_base64_url(self)
    }

    fn to_utf8_string(&self) -> ExtensionResult<String> {
        Encoding::to_utf8_string(self)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encoding() {
        let data = vec![0x01, 0x02, 0x03, 0x04];

        assert_eq!(Encoding::to_hex(&data), "01020304");
        assert_eq!(Encoding::to_hex_string(&data), "0x01020304");

        assert_eq!(Encoding::from_hex("01020304").unwrap(), data);
        assert_eq!(Encoding::from_hex("0x01020304").unwrap(), data);
        assert_eq!(Encoding::from_hex("0X01020304").unwrap(), data);
    }

    #[test]
    fn test_base64_encoding() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let base64_str = "AQIDBA==";

        assert_eq!(Encoding::to_base64(&data), base64_str);
        assert_eq!(Encoding::from_base64(base64_str).unwrap(), data);
    }

    #[test]
    fn test_base64_url_encoding() {
        let data = vec![0xff, 0xfe, 0xfd];
        let base64_url = Encoding::to_base64_url(&data);

        assert_eq!(Encoding::from_base64_url(&base64_url).unwrap(), data);
    }

    #[test]
    fn test_utf8_encoding() {
        let text = "Hello, Neo!";
        let bytes = Encoding::from_utf8_string(text);

        assert_eq!(Encoding::to_utf8_string(&bytes).unwrap(), text);
    }

    #[test]
    fn test_validation() {
        assert!(Encoding::is_valid_hex("01020304"));
        assert!(Encoding::is_valid_hex("0x01020304"));
        assert!(!Encoding::is_valid_hex("invalid"));

        assert!(Encoding::is_valid_base64("AQIDBA=="));
        assert!(!Encoding::is_valid_base64("invalid"));
    }

    #[test]
    fn test_integer_encoding() {
        let value = 0x12345678u32;

        let le_bytes = Encoding::int_to_le_bytes(value);
        assert_eq!(Encoding::le_bytes_to_int(&le_bytes).unwrap(), value as u64);

        let be_bytes = Encoding::int_to_be_bytes(value);
        assert_eq!(Encoding::be_bytes_to_int(&be_bytes).unwrap(), value as u64);
    }

    #[test]
    fn test_extension_trait() {
        let data = vec![0x01, 0x02, 0x03, 0x04];

        assert_eq!(data.to_hex(), "01020304");
        assert_eq!(data.to_hex_string(), "0x01020304");
        assert_eq!(data.to_base64(), "AQIDBA==");
    }
}

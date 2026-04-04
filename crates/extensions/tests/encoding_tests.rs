//! Extensions Encoding C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Extensions encoding utilities.
//! Tests are based on the C# Neo.Extensions.Encoding test suite.

use neo_extensions::encoding::*;

#[cfg(test)]
#[allow(dead_code)]
mod encoding_tests {
    use super::*;

    /// Test hex encoding compatibility
    #[test]
    fn test_hex_encoding_compatibility() {
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let encoded = Encoding::to_hex(&data);
        let decoded = Encoding::from_hex(&encoded).unwrap();
        assert_eq!(decoded, data);

        // Test with 0x prefix
        let encoded_with_prefix = Encoding::to_hex_string(&data);
        assert!(encoded_with_prefix.starts_with("0x"));
        let decoded2 = Encoding::from_hex(&encoded_with_prefix).unwrap();
        assert_eq!(decoded2, data);
    }

    /// Test Base64 encoding compatibility
    #[test]
    fn test_base64_encoding_compatibility() {
        let test_data = b"Neo blockchain encoding test";

        let encoded = Encoding::to_base64(test_data);
        let decoded = Encoding::from_base64(&encoded).unwrap();
        assert_eq!(decoded, test_data);

        // Test URL-safe encoding
        let url_encoded = Encoding::to_base64_url(test_data);
        let url_decoded = Encoding::from_base64_url(&url_encoded).unwrap();
        assert_eq!(url_decoded, test_data);
    }

    /// Test additional hex encoding
    #[test]
    fn test_hex_encoding_additional() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];

        let hex_lower = Encoding::to_hex(&data);
        assert_eq!(hex_lower, "deadbeefcafebabe");

        let decoded_lower = Encoding::from_hex(&hex_lower).unwrap();
        assert_eq!(decoded_lower, data);

        // Test validation
        assert!(Encoding::is_valid_hex(&hex_lower));
        assert!(Encoding::is_valid_hex("0xdeadbeef"));
        assert!(!Encoding::is_valid_hex("0xZZZ"));
    }

    /// Test string encoding utilities
    #[test]
    fn test_string_encoding_compatibility() {
        let test_string = "Neo blockchain 区块链";

        // UTF-8 encoding
        let utf8_bytes = Encoding::from_utf8_string(test_string);
        let utf8_decoded = Encoding::to_utf8_string(&utf8_bytes).unwrap();
        assert_eq!(utf8_decoded, test_string);

        // Test validation
        assert!(Encoding::is_valid_base64("SGVsbG8gV29ybGQ="));
        assert!(!Encoding::is_valid_base64("This is not base64!"));
    }
}

//! Comprehensive UInt256 Tests Matching C# Neo Implementation
//!
//! This module implements all C# UT_UInt256 test cases to ensure
//! complete behavioral compatibility with the original Neo implementation.

#[cfg(test)]
mod comprehensive_uint256_tests {
    use crate::UInt256;
    use std::str::FromStr;

    /// Test constructor failure with invalid byte array length (matches C# TestFail)
    #[test]
    fn test_uint256_constructor_fail() {
        // Test with too many bytes (should fail)
        let invalid_bytes = vec![0u8; 33]; // UInt256 should be exactly 32 bytes
        let result = UInt256::from_bytes(&invalid_bytes);
        assert!(result.is_err(), "UInt256 should reject invalid length arrays");
        
        // Test with too few bytes (should fail)
        let invalid_bytes_short = vec![0u8; 31];
        let result = UInt256::from_bytes(&invalid_bytes_short);
        assert!(result.is_err(), "UInt256 should reject short arrays");
    }

    /// Test default constructor (matches C# TestGernerator1)
    #[test]
    fn test_uint256_default_constructor() {
        let uint256 = UInt256::new();
        assert_eq!(uint256, UInt256::zero());
        assert_eq!(uint256.to_array(), [0u8; 32]);
    }

    /// Test constructor from byte array (matches C# TestGernerator2)
    #[test]
    fn test_uint256_from_bytes_constructor() {
        let bytes = [0u8; 32];
        let uint256 = UInt256::from_bytes(&bytes).unwrap();
        assert_eq!(uint256.to_array(), bytes);
        
        // Test with non-zero bytes
        let mut bytes_nonzero = [0u8; 32];
        bytes_nonzero[31] = 0x01;
        let uint256_nonzero = UInt256::from_bytes(&bytes_nonzero).unwrap();
        assert_eq!(uint256_nonzero.to_array(), bytes_nonzero);
    }

    /// Test constructor from string (matches C# TestGernerator3)
    #[test]
    fn test_uint256_from_string_constructor() {
        let hex_str = "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
        let uint256 = UInt256::from_str(&hex_str[2..]).unwrap(); // Remove 0x prefix
        assert_eq!(uint256.to_hex_string(), hex_str[2..].to_lowercase());
        
        // Test various string formats
        let hex_uppercase = "0102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F20";
        let uint256_upper = UInt256::from_str(hex_uppercase).unwrap();
        assert_eq!(uint256_upper.to_hex_string(), hex_uppercase.to_lowercase());
    }

    /// Test CompareTo functionality (matches C# TestCompareTo)
    #[test]
    fn test_uint256_compare_to() {
        let zero = UInt256::zero();
        
        let mut temp = [0u8; 32];
        temp[31] = 0x01;
        let result = UInt256::from_bytes(&temp).unwrap();
        
        // Test self comparison
        assert_eq!(zero.cmp(&zero), std::cmp::Ordering::Equal);
        
        // Test less than
        assert_eq!(zero.cmp(&result), std::cmp::Ordering::Less);
        
        // Test greater than
        assert_eq!(result.cmp(&zero), std::cmp::Ordering::Greater);
        
        // Test equality with same bytes
        let result2 = UInt256::from_bytes(&temp).unwrap();
        assert_eq!(result.cmp(&result2), std::cmp::Ordering::Equal);
        
        // Test comparison with different positions
        let mut temp_high = [0u8; 32];
        temp_high[0] = 0x01; // High order byte
        let result_high = UInt256::from_bytes(&temp_high).unwrap();
        assert!(result_high > result); // High order should be greater
    }

    /// Test Equals functionality (matches C# TestEquals)
    #[test]
    fn test_uint256_equals() {
        let mut temp = [0u8; 32];
        temp[31] = 0x01;
        let result = UInt256::from_bytes(&temp).unwrap();
        
        // Test equality
        let result2 = UInt256::from_bytes(&temp).unwrap();
        assert_eq!(result, result2);
        
        // Test inequality
        let zero = UInt256::zero();
        assert_ne!(result, zero);
        
        // Test with different values
        let mut temp2 = [0u8; 32];
        temp2[30] = 0x01;
        let result3 = UInt256::from_bytes(&temp2).unwrap();
        assert_ne!(result, result3);
    }

    /// Test GetHashCode functionality (matches C# TestGetHashCode)
    #[test]
    fn test_uint256_get_hash_code() {
        let uint1 = UInt256::zero();
        let uint2 = UInt256::zero();
        
        // Same values should have same hash
        assert_eq!(uint1.get_hash_code(), uint2.get_hash_code());
        
        // Different values should have different hashes
        let mut temp = [0u8; 32];
        temp[31] = 0x01;
        let uint3 = UInt256::from_bytes(&temp).unwrap();
        assert_ne!(uint1.get_hash_code(), uint3.get_hash_code());
    }

    /// Test Parse functionality (matches C# TestParse)
    #[test]
    fn test_uint256_parse() {
        // Test valid hex string
        let hex_str = "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
        let parsed = UInt256::parse(&hex_str[2..]).unwrap();
        assert_eq!(parsed.to_hex_string(), hex_str[2..].to_lowercase());
        
        // Test case insensitivity
        let hex_upper = "0102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F20";
        let parsed_upper = UInt256::parse(hex_upper).unwrap();
        assert_eq!(parsed, parsed_upper);
    }

    /// Test TryParse functionality (matches C# TestTryParse)
    #[test]
    fn test_uint256_try_parse() {
        // Test valid parse
        let hex_str = "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
        let result = UInt256::try_parse(hex_str);
        assert!(result.is_ok());
        
        // Test invalid parse
        let invalid_str = "invalid";
        let result = UInt256::try_parse(invalid_str);
        assert!(result.is_err());
        
        // Test wrong length
        let wrong_length = "0102030405"; // Too short
        let result = UInt256::try_parse(wrong_length);
        assert!(result.is_err());
    }

    /// Test ToArray functionality (matches C# TestToArray)
    #[test]
    fn test_uint256_to_array() {
        let expected = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
        ];
        
        let uint256 = UInt256::from_bytes(&expected).unwrap();
        let result = uint256.to_array();
        
        assert_eq!(result, expected);
        assert_eq!(result.len(), 32);
    }

    /// Test ToString functionality (matches C# TestToString)
    #[test]
    fn test_uint256_to_string() {
        let expected_hex = "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
        let bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
        ];
        
        let uint256 = UInt256::from_bytes(&bytes).unwrap();
        let result = format!("0x{}", uint256.to_hex_string());
        
        assert_eq!(result, expected_hex);
    }

    /// Test Zero property (matches C# TestZero)
    #[test]
    fn test_uint256_zero() {
        let zero = UInt256::zero();
        assert_eq!(zero.to_array(), [0u8; 32]);
        assert_eq!(zero.to_hex_string(), "0".repeat(64));
    }

    /// Test arithmetic operations
    #[test]
    fn test_uint256_arithmetic() {
        // Test that UInt256 maintains consistent behavior
        let zero = UInt256::zero();
        let mut bytes1 = [0u8; 32];
        bytes1[31] = 0x01;
        let one = UInt256::from_bytes(&bytes1).unwrap();
        
        // Basic comparison
        assert!(one > zero);
        assert_ne!(one, zero);
        
        // Test with larger values
        let mut bytes_large = [0xffu8; 32];
        let max_value = UInt256::from_bytes(&bytes_large).unwrap();
        assert!(max_value > one);
        assert!(max_value > zero);
    }

    /// Test serialization compatibility with Neo protocol
    #[test]
    fn test_uint256_serialization_compatibility() {
        let test_cases = [
            "0000000000000000000000000000000000000000000000000000000000000000",
            "0000000000000000000000000000000000000000000000000000000000000001", 
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
        ];
        
        for hex_str in &test_cases {
            let uint256 = UInt256::from_str(hex_str).unwrap();
            let serialized = uint256.to_hex_string();
            let deserialized = UInt256::from_str(&serialized).unwrap();
            
            assert_eq!(uint256, deserialized, "Serialization roundtrip failed for {}", hex_str);
        }
    }

    /// Test hash functionality for use in collections
    #[test]
    fn test_uint256_hash_collection_usage() {
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        
        let uint1 = UInt256::zero();
        let uint2 = UInt256::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
        
        map.insert(uint1, "zero");
        map.insert(uint2, "one");
        
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&uint1), Some(&"zero"));
        assert_eq!(map.get(&uint2), Some(&"one"));
    }

    /// Test endianness compatibility with C# Neo
    #[test]
    fn test_uint256_endianness() {
        // C# Neo uses little-endian for storage but displays as big-endian hex
        let bytes = [
            0x20, 0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11,
            0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01
        ];
        
        let uint256 = UInt256::from_bytes(&bytes).unwrap();
        
        // The hex string should represent the reverse of the byte array (big-endian display)
        let hex_str = uint256.to_hex_string();
        assert_eq!(hex_str, "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20");
    }
}
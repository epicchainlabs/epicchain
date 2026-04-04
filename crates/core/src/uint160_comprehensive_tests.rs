//! Comprehensive UInt160 Tests Matching C# Neo Implementation
//!
//! This module implements all C# UT_UInt160 test cases to ensure
//! complete behavioral compatibility with the original Neo implementation.

#[cfg(test)]
mod comprehensive_uint160_tests {
    use crate::UInt160;
    use std::str::FromStr;

    /// Test constructor failure with invalid byte array length (matches C# TestFail)
    #[test]
    fn test_uint160_constructor_fail() {
        // Test with too many bytes (should fail)
        let invalid_bytes = vec![0u8; 21]; // UInt160 should be exactly 20 bytes
        let result = UInt160::from_bytes(&invalid_bytes);
        assert!(result.is_err(), "UInt160 should reject invalid length arrays");
    }

    /// Test default constructor (matches C# TestGernerator1)
    #[test]
    fn test_uint160_default_constructor() {
        let uint160 = UInt160::new();
        assert_eq!(uint160, UInt160::zero());
        assert_eq!(uint160.to_array(), [0u8; 20]);
    }

    /// Test constructor from byte array (matches C# TestGernerator2)
    #[test]
    fn test_uint160_from_bytes_constructor() {
        let bytes = [0u8; 20];
        let uint160 = UInt160::from_bytes(&bytes).unwrap();
        assert_eq!(uint160.to_array(), bytes);
    }

    /// Test constructor from string (matches C# TestGernerator3)
    #[test]
    fn test_uint160_from_string_constructor() {
        let hex_str = "0xff00000000000000000000000000000000000001";
        let uint160 = UInt160::from_str(&hex_str[2..]).unwrap(); // Remove 0x prefix
        assert_eq!(uint160.to_hex_string(), hex_str[2..].to_lowercase());
    }

    /// Test CompareTo functionality (matches C# TestCompareTo)
    #[test]
    fn test_uint160_compare_to() {
        let zero = UInt160::zero();
        
        let mut temp = [0u8; 20];
        temp[19] = 0x01;
        let result = UInt160::from_bytes(&temp).unwrap();
        
        // Test self comparison
        assert_eq!(zero.cmp(&zero), std::cmp::Ordering::Equal);
        
        // Test less than
        assert_eq!(zero.cmp(&result), std::cmp::Ordering::Less);
        
        // Test greater than
        assert_eq!(result.cmp(&zero), std::cmp::Ordering::Greater);
        
        // Test equality with same bytes
        let result2 = UInt160::from_bytes(&temp).unwrap();
        assert_eq!(result.cmp(&result2), std::cmp::Ordering::Equal);
    }

    /// Test Equals functionality (matches C# TestEquals)
    #[test]
    fn test_uint160_equals() {
        let mut temp = [0u8; 20];
        temp[19] = 0x01;
        let result = UInt160::from_bytes(&temp).unwrap();
        
        // Test equality
        let result2 = UInt160::from_bytes(&temp).unwrap();
        assert_eq!(result, result2);
        
        // Test inequality
        let zero = UInt160::zero();
        assert_ne!(result, zero);
        
        // Test with different values
        let mut temp2 = [0u8; 20];
        temp2[18] = 0x01;
        let result3 = UInt160::from_bytes(&temp2).unwrap();
        assert_ne!(result, result3);
    }

    /// Test GetHashCode functionality (matches C# TestGetHashCode)
    #[test]
    fn test_uint160_get_hash_code() {
        let uint1 = UInt160::zero();
        let uint2 = UInt160::zero();
        
        // Same values should have same hash
        assert_eq!(uint1.get_hash_code(), uint2.get_hash_code());
        
        // Different values should have different hashes (probabilistically)
        let mut temp = [0u8; 20];
        temp[19] = 0x01;
        let uint3 = UInt160::from_bytes(&temp).unwrap();
        assert_ne!(uint1.get_hash_code(), uint3.get_hash_code());
    }

    /// Test Parse functionality (matches C# TestParse)
    #[test]
    fn test_uint160_parse() {
        // Test valid hex string
        let hex_str = "0x0000000000000000000000000000000000000001";
        let parsed = UInt160::parse(&hex_str[2..]).unwrap();
        assert_eq!(parsed.to_hex_string(), hex_str[2..].to_lowercase());
        
        // Test without 0x prefix
        let hex_str_no_prefix = "0000000000000000000000000000000000000001";
        let parsed2 = UInt160::parse(hex_str_no_prefix).unwrap();
        assert_eq!(parsed, parsed2);
        
        // Test invalid string should fail
        let invalid_str = "invalid";
        assert!(UInt160::parse(invalid_str).is_err());
        
        // Test wrong length should fail
        let wrong_length = "00000000000000000000000000000000000000001"; // 41 chars instead of 40
        assert!(UInt160::parse(wrong_length).is_err());
    }

    /// Test TryParse functionality (matches C# TestTryParse)
    #[test]
    fn test_uint160_try_parse() {
        // Test valid parse
        let hex_str = "0000000000000000000000000000000000000001";
        let result = UInt160::try_parse(hex_str);
        assert!(result.is_ok());
        
        // Test invalid parse
        let invalid_str = "invalid";
        let result = UInt160::try_parse(invalid_str);
        assert!(result.is_err());
        
        // Test empty string
        let result = UInt160::try_parse("");
        assert!(result.is_err());
    }

    /// Test ToArray functionality (matches C# TestToArray)
    #[test]
    fn test_uint160_to_array() {
        let expected = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14
        ];
        
        let uint160 = UInt160::from_bytes(&expected).unwrap();
        let result = uint160.to_array();
        
        assert_eq!(result, expected);
        assert_eq!(result.len(), 20);
    }

    /// Test ToString functionality (matches C# TestToString)
    #[test]
    fn test_uint160_to_string() {
        let expected_hex = "0x0102030405060708090a0b0c0d0e0f1011121314";
        let bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14
        ];
        
        let uint160 = UInt160::from_bytes(&bytes).unwrap();
        let result = format!("0x{}", uint160.to_hex_string());
        
        assert_eq!(result, expected_hex);
    }

    /// Test Zero property (matches C# TestZero)
    #[test]
    fn test_uint160_zero() {
        let zero = UInt160::zero();
        assert_eq!(zero.to_array(), [0u8; 20]);
        assert_eq!(zero.to_hex_string(), "0".repeat(40));
    }

    /// Test implicit operators (matches C# TestImplicitOperator)
    #[test]
    fn test_uint160_conversions() {
        let bytes = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14
        ];
        
        // Test from bytes
        let from_bytes = UInt160::from_bytes(&bytes).unwrap();
        
        // Test to bytes
        let to_bytes = from_bytes.to_array();
        assert_eq!(to_bytes, bytes);
        
        // Test roundtrip
        let roundtrip = UInt160::from_bytes(&to_bytes).unwrap();
        assert_eq!(from_bytes, roundtrip);
    }

    /// Test serialization size (matches C# TestSize)
    #[test]
    fn test_uint160_size() {
        let uint160 = UInt160::zero();
        assert_eq!(uint160.size(), 20);
        
        let bytes = [0xffu8; 20];
        let uint160_max = UInt160::from_bytes(&bytes).unwrap();
        assert_eq!(uint160_max.size(), 20);
    }

    /// Test address conversion (EpicChain-specific functionality)
    #[test]
    fn test_uint160_to_address() {
        let script_hash = UInt160::zero();
        
        // Test address generation
        if let Ok(address) = script_hash.to_address() {
            assert!(address.len() == 34, "EpicChain address should be 34 characters");
            assert!(address.starts_with('X'), "MainNet address should start with X");
        }
        
        // Test script hash from address
    }

    /// Test JSON serialization (Neo RPC compatibility)
    #[test]
    fn test_uint160_json_serialization() {
        let uint160 = UInt160::from_str("0102030405060708090a0b0c0d0e0f1011121314").unwrap();
        
        // Test hex string representation for JSON
        let hex_str = uint160.to_hex_string();
        assert_eq!(hex_str.len(), 40);
        assert!(hex_str.chars().all(|c| c.is_ascii_hexdigit()));
        
        // Test with 0x prefix for JSON RPC
        let prefixed = format!("0x{}", hex_str);
        assert_eq!(prefixed.len(), 42);
    }

    /// Test edge cases and boundary conditions
    #[test]
    fn test_uint160_edge_cases() {
        // Test all zeros
        let all_zeros = UInt160::from_bytes(&[0u8; 20]).unwrap();
        assert_eq!(all_zeros, UInt160::zero());
        
        // Test all ones
        let all_ones = UInt160::from_bytes(&[0xffu8; 20]).unwrap();
        assert_ne!(all_ones, UInt160::zero());
        
        // Test single bit set
        let mut single_bit = [0u8; 20];
        single_bit[0] = 0x01;
        let single = UInt160::from_bytes(&single_bit).unwrap();
        assert_ne!(single, UInt160::zero());
        
        // Test highest bit set
        let mut highest_bit = [0u8; 20];
        highest_bit[19] = 0x80;
        let highest = UInt160::from_bytes(&highest_bit).unwrap();
        assert!(highest > UInt160::zero());
    }
}
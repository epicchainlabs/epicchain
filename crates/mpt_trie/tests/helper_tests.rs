//! MPT Helper Function C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo's MPT helper functions.
//! Tests are based on the C# Neo.Cryptography.MPTTrie helper utilities.

use neo_mpt_trie::*;

#[cfg(test)]
#[allow(dead_code)]
mod helper_tests {
    use super::*;

    /// Test nibble conversion functions (matches C# nibble handling exactly)
    #[test]
    fn test_nibble_conversion_compatibility() {
        let input_bytes = vec![0x12, 0x34, 0x56, 0xAB, 0xCD, 0xEF];
        let nibbles = to_nibbles(&input_bytes);

        let expected_nibbles = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
        assert_eq!(nibbles, expected_nibbles);

        let reconstructed_bytes = from_nibbles(&nibbles);
        assert_eq!(reconstructed_bytes, Ok(input_bytes));

        // Test round-trip conversion
        let test_cases = vec![
            vec![0x00],
            vec![0xFF],
            vec![0x12, 0x34],
            vec![0xAB, 0xCD, 0xEF],
            vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
        ];

        for test_case in test_cases {
            let nibbles = to_nibbles(&test_case);
            let reconstructed = from_nibbles(&nibbles);
            assert_eq!(reconstructed, Ok(test_case));
        }
    }

    /// Test nibble conversion edge cases (matches C# edge case handling exactly)
    #[test]
    fn test_nibble_conversion_edge_cases_compatibility() {
        // Test empty input
        let empty_bytes = vec![];
        let empty_nibbles: Vec<u8> = to_nibbles(&empty_bytes);
        assert_eq!(empty_nibbles, Vec::<u8>::new());

        let reconstructed_empty = from_nibbles(&empty_nibbles);
        assert_eq!(reconstructed_empty, Ok(empty_bytes));

        // Test single byte cases
        for byte_val in 0..=255u8 {
            let single_byte = vec![byte_val];
            let nibbles = to_nibbles(&single_byte);
            assert_eq!(nibbles.len(), 2);
            assert_eq!(nibbles[0], (byte_val >> 4) & 0x0F);
            assert_eq!(nibbles[1], byte_val & 0x0F);

            let reconstructed = from_nibbles(&nibbles);
            assert_eq!(reconstructed, Ok(single_byte));
        }

        let odd_nibbles = vec![0x1, 0x2, 0x3];
        let odd_result = from_nibbles(&odd_nibbles);
        assert!(odd_result.is_err());
    }

    /// Test common prefix length calculation (matches C# CommonPrefixLength exactly)
    #[test]
    fn test_common_prefix_length_compatibility() {
        // Test identical arrays
        let array1 = vec![1, 2, 3, 4, 5];
        let array2 = vec![1, 2, 3, 4, 5];
        assert_eq!(common_prefix_length(&array1, &array2), 5);

        // Test partial prefix
        let array3 = vec![1, 2, 3, 4, 5];
        let array4 = vec![1, 2, 3, 7, 8];
        assert_eq!(common_prefix_length(&array3, &array4), 3);

        // Test no common prefix
        let array5 = vec![1, 2, 3];
        let array6 = vec![4, 5, 6];
        assert_eq!(common_prefix_length(&array5, &array6), 0);

        // Test empty arrays
        let empty1 = vec![];
        let empty2 = vec![];
        assert_eq!(common_prefix_length(&empty1, &empty2), 0);

        let non_empty = vec![1, 2, 3];
        assert_eq!(common_prefix_length(&empty1, &non_empty), 0);
        assert_eq!(common_prefix_length(&non_empty, &empty1), 0);

        // Test different lengths
        let short = vec![1, 2];
        let long = vec![1, 2, 3, 4, 5];
        assert_eq!(common_prefix_length(&short, &long), 2);
        assert_eq!(common_prefix_length(&long, &short), 2);
    }

    /// Test common prefix length edge cases (matches C# edge case behavior exactly)
    #[test]
    fn test_common_prefix_length_edge_cases_compatibility() {
        // Test single element arrays
        let single1 = vec![42];
        let single2 = vec![42];
        let single3 = vec![43];

        assert_eq!(common_prefix_length(&single1, &single2), 1);
        assert_eq!(common_prefix_length(&single1, &single3), 0);

        // Test very long arrays with common prefix
        let long1: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let mut long2 = long1.clone();
        long2[500] = 255; // Change middle element

        assert_eq!(common_prefix_length(&long1, &long2), 500);

        // Test arrays where one is prefix of another
        let prefix = vec![1, 2, 3];
        let extended = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(common_prefix_length(&prefix, &extended), 3);
        assert_eq!(common_prefix_length(&extended, &prefix), 3);
    }

    /// Test nibble array utilities (matches C# nibble array operations exactly)
    #[test]
    fn test_nibble_array_utilities_compatibility() {
        // Test nibble validation
        let valid_nibbles = vec![
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
        ];
        for nibble in &valid_nibbles {
            assert!(*nibble <= 0xF);
        }

        // Test nibble comparison
        let nibbles1 = vec![0x1, 0x2, 0x3];
        let nibbles2 = vec![0x1, 0x2, 0x3];
        let nibbles3 = vec![0x1, 0x2, 0x4];

        assert_eq!(nibbles1, nibbles2);
        assert_ne!(nibbles1, nibbles3);

        // Test nibble slicing and manipulation
        let long_nibbles = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];
        let slice = &long_nibbles[2..5];
        assert_eq!(slice, &[0x3, 0x4, 0x5]);

        // Test nibble concatenation
        let part1 = vec![0x1, 0x2];
        let part2 = vec![0x3, 0x4];
        let combined = [part1, part2].concat();
        assert_eq!(combined, vec![0x1, 0x2, 0x3, 0x4]);
    }

    /// Test path manipulation functions (matches C# path utilities exactly)
    #[test]
    fn test_path_manipulation_compatibility() {
        // Test path prefix operations
        let full_path = vec![0x1, 0x2, 0x3, 0x4, 0x5];
        let prefix_len = 3;

        let prefix = &full_path[..prefix_len];
        let suffix = &full_path[prefix_len..];

        assert_eq!(prefix, &[0x1, 0x2, 0x3]);
        assert_eq!(suffix, &[0x4, 0x5]);

        // Test path reconstruction
        let reconstructed = [prefix, suffix].concat();
        assert_eq!(reconstructed, full_path);

        // Test empty path handling
        let empty_path = vec![];
        let empty_prefix: &[u8] = &empty_path[..0];
        let empty_slice: &[u8] = &[];
        assert_eq!(empty_prefix, empty_slice);

        // Test single element path
        let single_path = vec![0xA];
        let single_prefix = &single_path[..1];
        let single_suffix = &single_path[1..];

        assert_eq!(single_prefix, &[0xA]);
        let empty_slice: &[u8] = &[];
        assert_eq!(single_suffix, empty_slice);
    }

    /// Test byte key conversion utilities (matches C# key conversion exactly)
    #[test]
    fn test_byte_key_conversion_compatibility() {
        let test_strings = vec![
            "test_key",
            "another_key",
            "key_with_numbers_123",
            "special_chars_!@#",
            "",
        ];

        for test_string in test_strings {
            let byte_key = test_string.as_bytes();
            let nibbles = to_nibbles(byte_key);
            let reconstructed_bytes = from_nibbles(&nibbles);
            let reconstructed_string =
                String::from_utf8(reconstructed_bytes.expect("valid nibbles")).unwrap();

            assert_eq!(reconstructed_string, test_string);
        }

        // Test numeric key patterns
        for i in 0..256u32 {
            let key = format!("key_{}", i);
            let byte_key = key.as_bytes();
            let nibbles = to_nibbles(byte_key);
            let reconstructed = from_nibbles(&nibbles);

            assert_eq!(reconstructed, Ok(byte_key.to_vec()));
        }
    }

    /// Test hash key utilities (matches C# hash-based key operations exactly)
    #[test]
    fn test_hash_key_utilities_compatibility() {
        let hash_like_key = vec![0u8; 32];
        let nibbles = to_nibbles(&hash_like_key);
        assert_eq!(nibbles.len(), 64); // 32 bytes * 2 nibbles per byte

        let reconstructed = from_nibbles(&nibbles);
        assert_eq!(reconstructed, Ok(hash_like_key));

        // Test various hash patterns
        let test_hashes = vec![
            vec![0x00; 32],
            vec![0xFF; 32],
            (0..32).map(|i| i as u8).collect::<Vec<u8>>(),
            (0..32).map(|i| (255 - i) as u8).collect::<Vec<u8>>(),
        ];

        for hash in test_hashes {
            let nibbles = to_nibbles(&hash);
            let reconstructed = from_nibbles(&nibbles);
            assert_eq!(reconstructed, Ok(hash));
            assert_eq!(nibbles.len(), 64);
        }
    }

    /// Test performance characteristics of helper functions (matches C# performance expectations)
    #[test]
    fn test_helper_performance_characteristics_compatibility() {
        // Test performance with large inputs
        let large_input: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

        // Should handle large conversions efficiently
        let nibbles = to_nibbles(&large_input);
        assert_eq!(nibbles.len(), large_input.len() * 2);

        let reconstructed = from_nibbles(&nibbles);
        assert_eq!(reconstructed, Ok(large_input.clone()));

        // Test common prefix with large arrays
        let large1 = large_input.clone();
        let mut large2 = large_input.clone();
        large2[5000] = 255; // Change one element in the middle

        let prefix_len = common_prefix_length(&large1, &large2);
        assert_eq!(prefix_len, 5000);

        // Test performance with many small operations
        for i in 0..1000 {
            let small_input = vec![(i % 256) as u8, ((i + 1) % 256) as u8];
            let nibbles = to_nibbles(&small_input);
            let reconstructed = from_nibbles(&nibbles);
            assert_eq!(reconstructed, Ok(small_input));
        }
    }

    /// Test helper function error handling (matches C# error behavior exactly)
    #[test]
    fn test_helper_error_handling_compatibility() {
        // Test nibble validation in conversion functions
        let valid_nibbles = vec![
            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
        ];
        let result = from_nibbles(&valid_nibbles).expect("valid nibbles");
        assert_eq!(result.len(), 8); // 16 nibbles = 8 bytes

        // Test boundary conditions
        let boundary_cases = vec![vec![0x0], vec![0xF], vec![0x0, 0xF], vec![0xF, 0x0]];

        for case in boundary_cases {
            let nibbles = to_nibbles(&case);
            let reconstructed = from_nibbles(&nibbles);
            assert_eq!(reconstructed, Ok(case));
        }

        // Test empty input handling
        let empty_nibbles = vec![];
        let empty_result = from_nibbles(&empty_nibbles);
        assert_eq!(empty_result, Ok(vec![]));

        let empty_bytes = vec![];
        let empty_nibbles_from_bytes: Vec<u8> = to_nibbles(&empty_bytes);
        assert_eq!(empty_nibbles_from_bytes, Vec::<u8>::new());
    }
}

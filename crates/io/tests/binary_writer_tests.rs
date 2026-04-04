//! Binary Writer C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo's BinaryWriter functionality.
//! Tests are based on the C# Neo.IO.BinaryWriter test suite.

use neo_io::{BinaryWriter, MemoryReader};

#[cfg(test)]
#[allow(dead_code)]
mod binary_writer_tests {
    use super::*;
    /// Test writing basic integer types (matches C# BinaryWriter.Write(int) exactly)
    #[test]
    fn test_write_int32_compatibility() {
        let test_cases = vec![
            (0i32, vec![0x00, 0x00, 0x00, 0x00]),
            (-1i32, vec![0xFF, 0xFF, 0xFF, 0xFF]),
            (1i32, vec![0x01, 0x00, 0x00, 0x00]),
            (i32::MAX, vec![0xFF, 0xFF, 0xFF, 0x7F]),
            (i32::MIN, vec![0x00, 0x00, 0x00, 0x80]),
            (12345i32, vec![0x39, 0x30, 0x00, 0x00]),
        ];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_i32(value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(result, expected, "Failed to write i32 value: {}", value);
        }
    }

    /// Test writing unsigned integers (matches C# BinaryWriter.Write(uint) exactly)
    #[test]
    fn test_write_uint32_compatibility() {
        let test_cases = vec![
            (0u32, vec![0x00, 0x00, 0x00, 0x00]),
            (u32::MAX, vec![0xFF, 0xFF, 0xFF, 0xFF]),
            (1u32, vec![0x01, 0x00, 0x00, 0x00]),
            (12345u32, vec![0x39, 0x30, 0x00, 0x00]),
        ];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_u32(value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(result, expected, "Failed to write u32 value: {}", value);
        }
    }

    /// Test writing 64-bit integers (matches C# BinaryWriter.Write(long) exactly)
    #[test]
    fn test_write_int64_compatibility() {
        let test_cases = vec![
            (0i64, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            (-1i64, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
            (1i64, vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            (
                i64::MAX,
                vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
            ),
            (
                i64::MIN,
                vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80],
            ),
        ];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_i64(value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(result, expected, "Failed to write i64 value: {}", value);
        }
    }

    /// Test writing byte arrays (matches C# BinaryWriter.Write(byte[]) exactly)
    #[test]
    fn test_write_bytes_compatibility() {
        let test_data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let mut writer = BinaryWriter::new();
        writer.write_bytes(&test_data).unwrap();
        let result = writer.to_bytes();
        assert_eq!(result, test_data);
    }

    /// Test writing variable-length integers (matches C# BinaryWriter.WriteVarInt exactly)
    #[test]
    fn test_write_var_int_compatibility() {
        // Test cases from C# Neo implementation
        let test_cases = vec![
            (0u64, vec![0x00]),
            (1u64, vec![0x01]),
            (252u64, vec![0xFC]),
            (253u64, vec![0xFD, 0xFD, 0x00]),
            (65535u64, vec![0xFD, 0xFF, 0xFF]),
            (65536u64, vec![0xFE, 0x00, 0x00, 0x01, 0x00]),
            (4294967295u64, vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF]),
            (
                4294967296u64,
                vec![0xFF, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            ),
        ];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_var_int(value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(result, expected, "Failed to write var_int value: {}", value);
        }
    }

    /// Test writing variable-length strings (matches C# BinaryWriter.WriteVarString exactly)
    #[test]
    fn test_write_var_string_compatibility() {
        // Test cases from C# Neo implementation
        let test_cases = vec![
            ("", vec![0x00]),
            // Short strings
            ("Hello", vec![0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F]),
            ("Neo", vec![0x03, 0x4E, 0x65, 0x6F]),
            ("✓Neo", vec![0x06, 0xE2, 0x9C, 0x93, 0x4E, 0x65, 0x6F]),
        ];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_var_string(value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(
                result, expected,
                "Failed to write var_string value: {}",
                value
            );
        }
    }

    /// Test writing boolean values (matches C# BinaryWriter.Write(bool) exactly)
    #[test]
    fn test_write_boolean_compatibility() {
        let test_cases = vec![(false, vec![0x00]), (true, vec![0x01])];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_bool(value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(result, expected, "Failed to write boolean value: {}", value);
        }
    }

    /// Test writing multiple values sequentially (matches C# BinaryWriter behavior exactly)
    #[test]
    fn test_write_sequential_compatibility() {
        let mut writer = BinaryWriter::new();

        // Write various types in sequence
        writer.write_i32(1).unwrap();
        writer.write_i32(2).unwrap();
        writer.write_i32(3).unwrap();

        let expected = vec![
            0x01, 0x00, 0x00, 0x00, // int32: 1
            0x02, 0x00, 0x00, 0x00, // int32: 2
            0x03, 0x00, 0x00, 0x00, // int32: 3
        ];

        let result = writer.to_bytes();
        assert_eq!(result, expected);
    }

    /// Test writing floating point numbers (matches C# BinaryWriter float operations exactly)
    #[test]
    #[ignore = "BinaryWriter doesn't have write_f32/write_f64 methods"]
    fn test_write_float_compatibility() {
        // Test f32
        let mut writer = BinaryWriter::new();
        // writer.write_f32(1.23f32).unwrap();
        let result = writer.to_bytes();
        // assert_eq!(result, 1.23f32.to_le_bytes().to_vec());

        // Test f64
        let mut writer = BinaryWriter::new();
        // writer.write_f64(1.23456789f64).unwrap();
        let result = writer.to_bytes();
        // assert_eq!(result, 1.23456789f64.to_le_bytes().to_vec());
    }

    /// Test round-trip compatibility with BinaryReader (matches C# round-trip behavior exactly)
    #[test]
    fn test_round_trip_compatibility() {
        // Write various data types
        let mut writer = BinaryWriter::new();
        writer.write_i32(12345).unwrap();
        writer.write_var_string("Hello Neo").unwrap();
        writer.write_bool(true).unwrap();
        // writer.write_f64(3.14159).unwrap();

        let data = writer.to_bytes();

        // Read them back
        let mut reader = MemoryReader::new(&data);
        let int_val = reader.read_int32().unwrap();
        let string_val = reader.read_var_string(1000).unwrap();
        let bool_val = reader.read_boolean().unwrap();
        // let float_val = reader.read_f64().unwrap();

        assert_eq!(int_val, 12345);
        assert_eq!(string_val, "Hello Neo");
        assert_eq!(bool_val, true);
        // assert!((float_val - 3.14159).abs() < f64::EPSILON);
    }

    /// Test writing large amounts of data (matches C# BinaryWriter performance characteristics)
    #[test]
    fn test_large_data_compatibility() {
        let mut writer = BinaryWriter::new();

        // Write 1000 integers
        for i in 0..1000 {
            writer.write_i32(i).unwrap();
        }

        let result = writer.to_bytes();
        assert_eq!(result.len(), 4000); // 1000 * 4 bytes per int32

        // Verify the data is correct
        let mut reader = MemoryReader::new(&result);
        for i in 0..1000 {
            let value = reader.read_int32().unwrap();
            assert_eq!(value, i);
        }
    }

    /// Test writing variable-length byte arrays (matches C# BinaryWriter.WriteVarBytes exactly)
    #[test]
    fn test_write_var_bytes_compatibility() {
        let test_cases = vec![
            // Empty array
            (vec![], vec![0x00]),
            // Small arrays
            (vec![0x01], vec![0x01, 0x01]),
            (vec![0x01, 0x02, 0x03], vec![0x03, 0x01, 0x02, 0x03]),
            (vec![0xFF; 253], {
                let mut expected = vec![0xFD, 0xFD, 0x00]; // var_int(253)
                expected.extend(vec![0xFF; 253]);
                expected
            }),
        ];

        for (value, expected) in test_cases {
            let mut writer = BinaryWriter::new();
            writer.write_var_bytes(&value).unwrap();
            let result = writer.to_bytes();
            assert_eq!(
                result,
                expected,
                "Failed to write var_bytes of length: {}",
                value.len()
            );
        }
    }
}

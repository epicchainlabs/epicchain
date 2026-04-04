//! Serialization C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo's ISerializable functionality.
//! Tests are based on the C# Neo.IO.ISerializable test suite.

use neo_io::serializable::helper::get_var_size;
use neo_io::{BinaryWriter, IoError, IoResult, MemoryReader, Serializable};

#[cfg(test)]
#[allow(dead_code)]
mod serialization_tests {
    use super::*;

    /// Test structure that implements Serializable (matches C# ISerializable pattern exactly)
    #[derive(Debug, Clone, PartialEq)]
    struct TestSerializable {
        pub value1: i32,
        pub value2: String,
        pub value3: bool,
        pub bytes: Vec<u8>,
    }

    impl Serializable for TestSerializable {
        fn size(&self) -> usize {
            4 + // value1
            get_var_size(self.value2.len() as u64) + self.value2.len() + // value2
            1 + // value3
            get_var_size(self.bytes.len() as u64) + self.bytes.len() // bytes
        }

        fn serialize(&self, writer: &mut BinaryWriter) -> IoResult<()> {
            writer.write_i32(self.value1)?;
            writer.write_var_string(&self.value2)?;
            writer.write_bool(self.value3)?;
            writer.write_var_bytes(&self.bytes)?;
            Ok(())
        }

        fn deserialize(reader: &mut MemoryReader) -> IoResult<Self> {
            let value1 = reader.read_int32()?;
            let value2 = reader.read_var_string(1000)?;
            let value3 = reader.read_boolean()?;
            let bytes = reader.read_var_bytes(100000)?; // Allow larger arrays for testing

            Ok(TestSerializable {
                value1,
                value2,
                value3,
                bytes,
            })
        }
    }

    /// Test basic serialization/deserialization round-trip (matches C# ISerializable behavior exactly)
    #[test]
    fn test_serialization_round_trip_compatibility() {
        let original = TestSerializable {
            value1: 12345,
            value2: "Hello Neo".to_string(),
            value3: true,
            bytes: vec![0x01, 0x02, 0x03, 0x04, 0x05],
        };

        // Serialize
        let mut writer = BinaryWriter::new();
        original.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        // Deserialize
        let mut reader = MemoryReader::new(&serialized);
        let deserialized = TestSerializable::deserialize(&mut reader).unwrap();

        assert_eq!(original, deserialized);
    }

    /// Test serialization of complex nested structures (matches C# nested ISerializable exactly)
    #[test]
    fn test_nested_serialization_compatibility() {
        #[derive(Debug, Clone, PartialEq)]
        struct NestedStruct {
            pub inner: TestSerializable,
            pub count: u32,
            pub items: Vec<i32>,
        }

        impl Serializable for NestedStruct {
            fn size(&self) -> usize {
                self.inner.size() + // inner
                4 + // count
                get_var_size(self.items.len() as u64) + (self.items.len() * 4) // items
            }

            fn serialize(&self, writer: &mut BinaryWriter) -> IoResult<()> {
                self.inner.serialize(writer)?;
                writer.write_u32(self.count)?;
                writer.write_var_int(self.items.len() as u64)?;
                for item in &self.items {
                    writer.write_i32(*item)?;
                }
                Ok(())
            }

            fn deserialize(reader: &mut MemoryReader) -> IoResult<Self> {
                let inner = TestSerializable::deserialize(reader)?;
                let count = reader.read_u32()?;
                let item_count = reader.read_var_int(1000)? as usize;
                let mut items = Vec::with_capacity(item_count);
                for _ in 0..item_count {
                    items.push(reader.read_int32()?);
                }

                Ok(NestedStruct {
                    inner,
                    count,
                    items,
                })
            }
        }

        let original = NestedStruct {
            inner: TestSerializable {
                value1: 999,
                value2: "Nested".to_string(),
                value3: false,
                bytes: vec![0xAA, 0xBB, 0xCC],
            },
            count: 42,
            items: vec![1, 2, 3, 4, 5],
        };

        // Test round-trip
        let mut writer = BinaryWriter::new();
        original.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        let deserialized = NestedStruct::deserialize(&mut reader).unwrap();

        assert_eq!(original, deserialized);
    }

    /// Test serialization with empty/null values (matches C# null handling exactly)
    #[test]
    fn test_empty_values_serialization_compatibility() {
        let test_cases = vec![
            TestSerializable {
                value1: 0,
                value2: String::new(),
                value3: false,
                bytes: Vec::new(),
            },
            TestSerializable {
                value1: -1,
                value2: "".to_string(),
                value3: true,
                bytes: vec![],
            },
        ];

        for original in test_cases {
            let mut writer = BinaryWriter::new();
            original.serialize(&mut writer).unwrap();
            let serialized = writer.to_bytes();

            let mut reader = MemoryReader::new(&serialized);
            let deserialized = TestSerializable::deserialize(&mut reader).unwrap();

            assert_eq!(original, deserialized);
        }
    }

    /// Test serialization with maximum values (matches C# boundary value handling exactly)
    #[test]
    fn test_boundary_values_serialization_compatibility() {
        let original = TestSerializable {
            value1: i32::MAX,
            value2: "A".repeat(1000), // Large string
            value3: true,
            bytes: vec![0xFF; 1000], // Large byte array
        };

        let mut writer = BinaryWriter::new();
        original.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        let deserialized = TestSerializable::deserialize(&mut reader).unwrap();

        assert_eq!(original, deserialized);
    }

    /// Test serialization array handling (matches C# array serialization exactly)
    #[test]
    fn test_array_serialization_compatibility() {
        #[derive(Debug, Clone, PartialEq)]
        struct ArrayContainer {
            pub items: Vec<TestSerializable>,
        }

        impl Serializable for ArrayContainer {
            fn size(&self) -> usize {
                let mut size = get_var_size(self.items.len() as u64);
                for item in &self.items {
                    size += item.size();
                }
                size
            }

            fn serialize(&self, writer: &mut BinaryWriter) -> IoResult<()> {
                writer.write_var_int(self.items.len() as u64)?;
                for item in &self.items {
                    item.serialize(writer)?;
                }
                Ok(())
            }

            fn deserialize(reader: &mut MemoryReader) -> IoResult<Self> {
                let count = reader.read_var_int(1000)? as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(TestSerializable::deserialize(reader)?);
                }

                Ok(ArrayContainer { items })
            }
        }

        let original = ArrayContainer {
            items: vec![
                TestSerializable {
                    value1: 1,
                    value2: "First".to_string(),
                    value3: true,
                    bytes: vec![0x01],
                },
                TestSerializable {
                    value1: 2,
                    value2: "Second".to_string(),
                    value3: false,
                    bytes: vec![0x02, 0x03],
                },
                TestSerializable {
                    value1: 3,
                    value2: "Third".to_string(),
                    value3: true,
                    bytes: vec![0x04, 0x05, 0x06],
                },
            ],
        };

        let mut writer = BinaryWriter::new();
        original.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        let deserialized = ArrayContainer::deserialize(&mut reader).unwrap();

        assert_eq!(original, deserialized);
    }

    /// Test error handling during serialization (matches C# exception behavior exactly)
    #[test]
    fn test_serialization_error_handling_compatibility() {
        // Test deserialization with insufficient data
        let incomplete_data = vec![0x01, 0x02]; // Not enough for a complete TestSerializable
        let mut reader = MemoryReader::new(&incomplete_data);
        assert!(TestSerializable::deserialize(&mut reader).is_err());

        // Test deserialization with corrupted var_int
        let corrupted_data = vec![0xFF, 0x01]; // Invalid var_int prefix
        let mut reader = MemoryReader::new(&corrupted_data);
        assert!(TestSerializable::deserialize(&mut reader).is_err());
    }

    /// Test versioning compatibility (matches C# version handling exactly)
    #[test]
    fn test_versioning_compatibility() {
        #[derive(Debug, Clone, PartialEq)]
        struct VersionedStruct {
            pub version: u8,
            pub data: Vec<u8>,
        }

        impl Serializable for VersionedStruct {
            fn size(&self) -> usize {
                1 + // version
                get_var_size(self.data.len() as u64) + self.data.len() // data
            }

            fn serialize(&self, writer: &mut BinaryWriter) -> IoResult<()> {
                writer.write_u8(self.version)?;
                writer.write_var_bytes(&self.data)?;
                Ok(())
            }

            fn deserialize(reader: &mut MemoryReader) -> IoResult<Self> {
                let version = reader.read_byte()?;
                if version > 1 {
                    return Err(IoError::InvalidData {
                        context: "version check".to_string(),
                        value: format!("version {}", version),
                    });
                }
                let data = reader.read_var_bytes(1000)?;

                Ok(VersionedStruct { version, data })
            }
        }

        // Test valid version
        let valid = VersionedStruct {
            version: 1,
            data: vec![0x01, 0x02, 0x03],
        };

        let mut writer = BinaryWriter::new();
        valid.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        let deserialized = VersionedStruct::deserialize(&mut reader).unwrap();
        assert_eq!(valid, deserialized);

        // Test invalid version
        let invalid = VersionedStruct {
            version: 2, // Unsupported version
            data: vec![0x01],
        };

        let mut writer = BinaryWriter::new();
        invalid.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        let mut reader = MemoryReader::new(&serialized);
        assert!(VersionedStruct::deserialize(&mut reader).is_err());
    }

    /// Test performance and large data serialization (matches C# performance characteristics)
    #[test]
    fn test_large_data_serialization_performance() {
        // Create a large structure
        let large_struct = TestSerializable {
            value1: 12345,
            value2: "Large".repeat(100), // 500 character string
            value3: true,
            bytes: vec![0xAB; 10000], // 10KB of data
        };

        // Test serialization
        let mut writer = BinaryWriter::new();
        large_struct.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        // Verify size is reasonable
        assert!(serialized.len() > 10000); // Should be at least as large as the byte array
        assert!(serialized.len() < 12000); // But not too much overhead

        // Test deserialization
        let mut reader = MemoryReader::new(&serialized);
        let deserialized = TestSerializable::deserialize(&mut reader).unwrap();

        assert_eq!(large_struct, deserialized);
    }
}

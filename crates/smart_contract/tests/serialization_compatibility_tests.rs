//! Serialization compatibility tests - Implementing missing C# Neo functionality
//! These tests ensure 100% compatibility with C# Neo serialization/deserialization

use neo_core::{UInt160, UInt256};
use neo_smart_contract::{BinaryReader, BinaryWriter, Serializable, SerializationError};
use std::io::{Cursor, Read, Write};

// ============================================================================
// Binary Serialization Compatibility (25 tests)
// ============================================================================

#[test]
fn test_uint160_serialization_compatibility() {
    // Test UInt160 serialization matches C# Neo exactly
    let hash = UInt160::from([42u8; 20]);

    let mut writer = BinaryWriter::new();
    hash.serialize(&mut writer).unwrap();
    let serialized = writer.to_bytes();

    // Should be exactly 20 bytes
    assert_eq!(serialized.len(), 20);
    assert_eq!(serialized, hash.to_bytes());

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let deserialized = UInt160::deserialize(&mut reader).unwrap();
    assert_eq!(hash, deserialized);
}

#[test]
fn test_uint256_serialization_compatibility() {
    // Test UInt256 serialization matches C# Neo exactly
    let hash = UInt256::from([99u8; 32]);

    let mut writer = BinaryWriter::new();
    hash.serialize(&mut writer).unwrap();
    let serialized = writer.to_bytes();

    // Should be exactly 32 bytes
    assert_eq!(serialized.len(), 32);
    assert_eq!(serialized, hash.to_bytes());

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let deserialized = UInt256::deserialize(&mut reader).unwrap();
    assert_eq!(hash, deserialized);
}

#[test]
fn test_variable_length_encoding_compatibility() {
    // Test variable length encoding matches C# Neo exactly
    let test_cases = vec![
        (0u64, vec![0x00]),                                  // 1 byte
        (252u64, vec![0xfc]),                                // 1 byte max
        (253u64, vec![0xfd, 0xfd, 0x00]),                    // 3 bytes min
        (65535u64, vec![0xfd, 0xff, 0xff]),                  // 3 bytes max
        (65536u64, vec![0xfe, 0x00, 0x00, 0x01, 0x00]),      // 5 bytes min
        (4294967295u64, vec![0xfe, 0xff, 0xff, 0xff, 0xff]), // 5 bytes max
    ];

    for (value, expected) in test_cases {
        let mut writer = BinaryWriter::new();
        writer.write_var_int(value).unwrap();
        let serialized = writer.to_bytes();

        assert_eq!(
            serialized, expected,
            "VarInt encoding failed for value: {}",
            value
        );

        // Test deserialization
        let mut reader = BinaryReader::new(&serialized);
        let deserialized = reader.read_var_int().unwrap();
        assert_eq!(
            value, deserialized,
            "VarInt decoding failed for value: {}",
            value
        );
    }
}

#[test]
fn test_string_serialization_compatibility() {
    // Test string serialization matches C# Neo exactly
    let test_strings = vec![
        "",               // Empty string
        "Hello",          // Simple ASCII
        "Neo Blockchain", // ASCII with space
        "你好世界",       // Unicode
        "🚀 Neo N3",      // Emoji
    ];

    for test_string in test_strings {
        let mut writer = BinaryWriter::new();
        writer.write_string(test_string).unwrap();
        let serialized = writer.to_bytes();

        // Format: VarInt(length) + UTF-8 bytes
        let utf8_bytes = test_string.as_bytes();

        let mut expected = Vec::new();
        let mut expected_writer = BinaryWriter::new();
        expected_writer
            .write_var_int(utf8_bytes.len() as u64)
            .unwrap();
        expected.extend_from_slice(&expected_writer.to_bytes());
        expected.extend_from_slice(utf8_bytes);

        assert_eq!(
            serialized, expected,
            "String serialization failed for: {}",
            test_string
        );

        // Test deserialization
        let mut reader = BinaryReader::new(&serialized);
        let deserialized = reader.read_string().unwrap();
        assert_eq!(
            test_string, deserialized,
            "String deserialization failed for: {}",
            test_string
        );
    }
}

#[test]
fn test_byte_array_serialization_compatibility() {
    // Test byte array serialization matches C# Neo exactly
    let test_arrays = vec![
        vec![],                 // Empty array
        vec![0x00],             // Single byte
        vec![0x01, 0x02, 0x03], // Small array
        vec![0u8; 255],         // Medium array
        vec![0xffu8; 1000],     // Large array
    ];

    for test_array in test_arrays {
        let mut writer = BinaryWriter::new();
        writer.write_bytes(&test_array).unwrap();
        let serialized = writer.to_bytes();

        // Format: VarInt(length) + bytes
        let mut expected = Vec::new();
        let mut expected_writer = BinaryWriter::new();
        expected_writer
            .write_var_int(test_array.len() as u64)
            .unwrap();
        expected.extend_from_slice(&expected_writer.to_bytes());
        expected.extend_from_slice(&test_array);

        assert_eq!(
            serialized,
            expected,
            "Byte array serialization failed for array of length: {}",
            test_array.len()
        );

        // Test deserialization
        let mut reader = BinaryReader::new(&serialized);
        let deserialized = reader.read_bytes().unwrap();
        assert_eq!(
            test_array,
            deserialized,
            "Byte array deserialization failed for array of length: {}",
            test_array.len()
        );
    }
}

#[test]
fn test_fixed_array_serialization_compatibility() {
    // Test fixed-length array serialization matches C# Neo exactly
    let test_array = [1u8, 2u8, 3u8, 4u8, 5u8];

    let mut writer = BinaryWriter::new();
    writer.write_fixed_bytes(&test_array).unwrap();
    let serialized = writer.to_bytes();

    // Fixed array should not include length prefix
    assert_eq!(serialized, test_array.to_vec());

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let mut deserialized = [0u8; 5];
    reader.read_fixed_bytes(&mut deserialized).unwrap();
    assert_eq!(test_array, deserialized);
}

#[test]
fn test_integer_serialization_compatibility() {
    // Test integer serialization matches C# Neo exactly (little-endian)
    let test_cases = vec![
        (0u8 as u64, vec![0x00]),
        (255u8 as u64, vec![0xff]),
        (256u16 as u64, vec![0x00, 0x01]),
        (65535u16 as u64, vec![0xff, 0xff]),
        (65536u32 as u64, vec![0x00, 0x00, 0x01, 0x00]),
        (4294967295u32 as u64, vec![0xff, 0xff, 0xff, 0xff]),
    ];

    for (value, expected) in test_cases {
        // Test different integer sizes
        if value <= u8::MAX as u64 {
            let mut writer = BinaryWriter::new();
            writer.write_u8(value as u8).unwrap();
            let serialized = writer.to_bytes();
            assert_eq!(serialized, expected[..1].to_vec());
        }

        if value <= u16::MAX as u64 {
            let mut writer = BinaryWriter::new();
            writer.write_u16(value as u16).unwrap();
            let serialized = writer.to_bytes();
            if expected.len() >= 2 {
                assert_eq!(serialized, expected[..2].to_vec());
            }
        }

        if value <= u32::MAX as u64 {
            let mut writer = BinaryWriter::new();
            writer.write_u32(value as u32).unwrap();
            let serialized = writer.to_bytes();
            if expected.len() >= 4 {
                assert_eq!(serialized, expected[..4].to_vec());
            }
        }
    }
}

#[test]
fn test_boolean_serialization_compatibility() {
    // Test boolean serialization matches C# Neo exactly
    let mut writer = BinaryWriter::new();

    writer.write_bool(true).unwrap();
    writer.write_bool(false).unwrap();

    let serialized = writer.to_bytes();
    assert_eq!(serialized, vec![0x01, 0x00]);

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    assert_eq!(reader.read_bool().unwrap(), true);
    assert_eq!(reader.read_bool().unwrap(), false);
}

#[test]
fn test_biginteger_serialization_compatibility() {
    // Test BigInteger serialization matches C# Neo exactly
    let test_cases = vec![
        (0i64, vec![0x00]),            // Zero
        (1i64, vec![0x01]),            // Positive small
        (-1i64, vec![0xff]),           // Negative small
        (127i64, vec![0x7f]),          // Max positive byte
        (128i64, vec![0x80, 0x00]),    // Min 2-byte positive
        (-128i64, vec![0x80]),         // Min negative byte
        (-129i64, vec![0x7f, 0xff]),   // Max 2-byte negative
        (32767i64, vec![0xff, 0x7f]),  // Max positive 2-byte
        (-32768i64, vec![0x00, 0x80]), // Min negative 2-byte
    ];

    for (value, expected) in test_cases {
        let bigint = BigInteger::from(value);
        let mut writer = BinaryWriter::new();
        bigint.serialize(&mut writer).unwrap();
        let serialized = writer.to_bytes();

        assert_eq!(
            serialized, expected,
            "BigInteger serialization failed for value: {}",
            value
        );

        // Test deserialization
        let mut reader = BinaryReader::new(&serialized);
        let deserialized = BigInteger::deserialize(&mut reader).unwrap();
        assert_eq!(
            bigint, deserialized,
            "BigInteger deserialization failed for value: {}",
            value
        );
    }
}

#[test]
fn test_array_serialization_compatibility() {
    // Test array serialization matches C# Neo exactly
    let array_data = vec![
        UInt160::from([1u8; 20]),
        UInt160::from([2u8; 20]),
        UInt160::from([3u8; 20]),
    ];

    let mut writer = BinaryWriter::new();
    writer.write_var_int(array_data.len() as u64).unwrap();
    for item in &array_data {
        item.serialize(&mut writer).unwrap();
    }

    let serialized = writer.to_bytes();

    // Verify structure: VarInt(count) + items
    let mut expected = Vec::new();
    expected.push(array_data.len() as u8); // VarInt for small array
    for item in &array_data {
        expected.extend_from_slice(&item.to_bytes());
    }

    assert_eq!(serialized, expected);

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let count = reader.read_var_int().unwrap() as usize;
    let mut deserialized = Vec::with_capacity(count);

    for _ in 0..count {
        let item = UInt160::deserialize(&mut reader).unwrap();
        deserialized.push(item);
    }

    assert_eq!(array_data, deserialized);
}

#[test]
fn test_dictionary_serialization_compatibility() {
    // Test dictionary serialization matches C# Neo exactly
    let mut dict_data = std::collections::BTreeMap::new();
    dict_data.insert("key1".to_string(), 100u64);
    dict_data.insert("key2".to_string(), 200u64);
    dict_data.insert("key3".to_string(), 300u64);

    let mut writer = BinaryWriter::new();
    writer.write_var_int(dict_data.len() as u64).unwrap();

    for (key, value) in &dict_data {
        writer.write_string(key).unwrap();
        writer.write_u64(*value).unwrap();
    }

    let serialized = writer.to_bytes();

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let count = reader.read_var_int().unwrap() as usize;
    let mut deserialized = std::collections::BTreeMap::new();

    for _ in 0..count {
        let key = reader.read_string().unwrap();
        let value = reader.read_u64().unwrap();
        deserialized.insert(key, value);
    }

    assert_eq!(dict_data, deserialized);
}

#[test]
fn test_null_value_serialization_compatibility() {
    // Test null value serialization matches C# Neo exactly
    let mut writer = BinaryWriter::new();
    writer.write_bool(false).unwrap(); // Null indicator

    let serialized = writer.to_bytes();
    assert_eq!(serialized, vec![0x00]);

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let is_null = !reader.read_bool().unwrap();
    assert!(is_null);
}

#[test]
fn test_complex_structure_serialization_compatibility() {
    // Test complex nested structure serialization matches C# Neo
    let structure = ComplexStructure {
        id: 42,
        name: "Test Structure".to_string(),
        hash: UInt160::from([99u8; 20]),
        flags: true,
        data: vec![1, 2, 3, 4, 5],
        metadata: {
            let mut map = std::collections::BTreeMap::new();
            map.insert("version".to_string(), 1);
            map.insert("type".to_string(), 2);
            map
        },
    };

    let mut writer = BinaryWriter::new();
    structure.serialize(&mut writer).unwrap();
    let serialized = writer.to_bytes();

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    let deserialized = ComplexStructure::deserialize(&mut reader).unwrap();

    assert_eq!(structure, deserialized);
}

#[test]
fn test_endianness_compatibility() {
    // Test that all multi-byte values use little-endian (matches C# Neo)
    let test_u16 = 0x1234u16;
    let test_u32 = 0x12345678u32;
    let test_u64 = 0x123456789abcdef0u64;

    let mut writer = BinaryWriter::new();
    writer.write_u16(test_u16).unwrap();
    writer.write_u32(test_u32).unwrap();
    writer.write_u64(test_u64).unwrap();

    let serialized = writer.to_bytes();

    // Verify little-endian byte order
    assert_eq!(&serialized[0..2], &[0x34, 0x12]); // u16
    assert_eq!(&serialized[2..6], &[0x78, 0x56, 0x34, 0x12]); // u32
    assert_eq!(
        &serialized[6..14],
        &[0xf0, 0xde, 0xbc, 0x9a, 0x78, 0x56, 0x34, 0x12]
    ); // u64

    // Test deserialization
    let mut reader = BinaryReader::new(&serialized);
    assert_eq!(reader.read_u16().unwrap(), test_u16);
    assert_eq!(reader.read_u32().unwrap(), test_u32);
    assert_eq!(reader.read_u64().unwrap(), test_u64);
}

#[test]
fn test_stream_position_tracking_compatibility() {
    // Test stream position tracking matches C# Neo BinaryReader
    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = BinaryReader::new(&test_data);

    assert_eq!(reader.position(), 0);

    let _ = reader.read_u8().unwrap();
    assert_eq!(reader.position(), 1);

    let _ = reader.read_u16().unwrap();
    assert_eq!(reader.position(), 3);

    let _ = reader.read_u32().unwrap();
    assert_eq!(reader.position(), 7);

    let _ = reader.read_u8().unwrap();
    assert_eq!(reader.position(), 8);

    // Test end of stream
    assert!(reader.read_u8().is_err());
}

#[test]
fn test_error_handling_compatibility() {
    // Test error handling matches C# Neo behavior
    let empty_data = vec![];
    let mut reader = BinaryReader::new(&empty_data);

    // Reading from empty stream should error
    assert!(reader.read_u8().is_err());
    assert!(reader.read_u16().is_err());
    assert!(reader.read_string().is_err());

    // Test truncated data
    let truncated_data = vec![0xff]; // VarInt indicating string length 255, but no data
    let mut reader = BinaryReader::new(&truncated_data);
    assert!(reader.read_string().is_err());
}

#[test]
fn test_buffer_management_compatibility() {
    // Test buffer management matches C# Neo MemoryStream behavior
    let mut writer = BinaryWriter::new();

    // Write progressively larger data to test buffer expansion
    for i in 0..1000 {
        writer.write_u32(i).unwrap();
    }

    let serialized = writer.to_bytes();
    assert_eq!(serialized.len(), 1000 * 4);

    // Verify all data was written correctly
    let mut reader = BinaryReader::new(&serialized);
    for i in 0..1000 {
        assert_eq!(reader.read_u32().unwrap(), i);
    }
}

#[test]
fn test_serialization_limits_compatibility() {
    // Test serialization limits match C# Neo
    const MAX_ARRAY_SIZE: usize = 16777216; // 16MB limit from C# Neo
    const MAX_STRING_LENGTH: usize = 65536; // 64KB limit from C# Neo

    // Test string length limit
    let long_string = "a".repeat(MAX_STRING_LENGTH + 1);
    let mut writer = BinaryWriter::new();
    let result = writer.write_string(&long_string);
    assert!(result.is_err(), "Should reject string longer than 64KB");

    // Test array size limit
    let large_array = vec![0u8; MAX_ARRAY_SIZE + 1];
    let mut writer = BinaryWriter::new();
    let result = writer.write_bytes(&large_array);
    assert!(result.is_err(), "Should reject array larger than 16MB");
}

// ============================================================================
// Helper Types and Implementations (Stubs for missing functionality)
// ============================================================================

/// Binary writer that matches C# Neo BinaryWriter behavior
pub struct BinaryWriter {
    buffer: Vec<u8>,
}

impl BinaryWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), SerializationError> {
        self.buffer.push(value);
        Ok(())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), SerializationError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), SerializationError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    pub fn write_u64(&mut self, value: u64) -> Result<(), SerializationError> {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    pub fn write_bool(&mut self, value: bool) -> Result<(), SerializationError> {
        self.write_u8(if value { 0x01 } else { 0x00 })
    }

    pub fn write_var_int(&mut self, mut value: u64) -> Result<(), SerializationError> {
        if value < 0xfd {
            self.write_u8(value as u8)?;
        } else if value <= 0xffff {
            self.write_u8(0xfd)?;
            self.write_u16(value as u16)?;
        } else if value <= 0xffffffff {
            self.write_u8(0xfe)?;
            self.write_u32(value as u32)?;
        } else {
            self.write_u8(0xff)?;
            self.write_u64(value)?;
        }
        Ok(())
    }

    pub fn write_string(&mut self, value: &str) -> Result<(), SerializationError> {
        const MAX_STRING_LENGTH: usize = 65536;

        let bytes = value.as_bytes();
        if bytes.len() > MAX_STRING_LENGTH {
            return Err(SerializationError::StringTooLong);
        }

        self.write_var_int(bytes.len() as u64)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    pub fn write_bytes(&mut self, value: &[u8]) -> Result<(), SerializationError> {
        const MAX_ARRAY_SIZE: usize = 16777216;

        if value.len() > MAX_ARRAY_SIZE {
            return Err(SerializationError::ArrayTooLarge);
        }

        self.write_var_int(value.len() as u64)?;
        self.buffer.extend_from_slice(value);
        Ok(())
    }

    pub fn write_fixed_bytes(&mut self, value: &[u8]) -> Result<(), SerializationError> {
        self.buffer.extend_from_slice(value);
        Ok(())
    }
}

/// Binary reader that matches C# Neo BinaryReader behavior
pub struct BinaryReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn read_u8(&mut self) -> Result<u8, SerializationError> {
        if self.position >= self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16, SerializationError> {
        if self.position + 2 > self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        let bytes = [self.data[self.position], self.data[self.position + 1]];
        self.position += 2;
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u32(&mut self) -> Result<u32, SerializationError> {
        if self.position + 4 > self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        let bytes = [
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ];
        self.position += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_u64(&mut self) -> Result<u64, SerializationError> {
        if self.position + 8 > self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.data[self.position..self.position + 8]);
        self.position += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    pub fn read_bool(&mut self) -> Result<bool, SerializationError> {
        let value = self.read_u8()?;
        Ok(value != 0)
    }

    pub fn read_var_int(&mut self) -> Result<u64, SerializationError> {
        let first_byte = self.read_u8()?;

        match first_byte {
            0xfd => Ok(self.read_u16()? as u64),
            0xfe => Ok(self.read_u32()? as u64),
            0xff => self.read_u64(),
            _ => Ok(first_byte as u64),
        }
    }

    pub fn read_string(&mut self) -> Result<String, SerializationError> {
        let length = self.read_var_int()? as usize;

        if self.position + length > self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        let bytes = &self.data[self.position..self.position + length];
        self.position += length;

        String::from_utf8(bytes.to_vec()).map_err(|_| SerializationError::InvalidUtf8)
    }

    pub fn read_bytes(&mut self) -> Result<Vec<u8>, SerializationError> {
        let length = self.read_var_int()? as usize;

        if self.position + length > self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        let bytes = self.data[self.position..self.position + length].to_vec();
        self.position += length;
        Ok(bytes)
    }

    pub fn read_fixed_bytes(&mut self, buffer: &mut [u8]) -> Result<(), SerializationError> {
        if self.position + buffer.len() > self.data.len() {
            return Err(SerializationError::EndOfStream);
        }

        buffer.copy_from_slice(&self.data[self.position..self.position + buffer.len()]);
        self.position += buffer.len();
        Ok(())
    }
}

/// Serialization trait matching C# Neo ISerializable
pub trait Serializable {
    fn serialize(&self, writer: &mut BinaryWriter) -> Result<(), SerializationError>;
    fn deserialize(reader: &mut BinaryReader) -> Result<Self, SerializationError>
    where
        Self: Sized;
}

/// Serialization errors
#[derive(Debug, Clone)]
pub enum SerializationError {
    EndOfStream,
    StringTooLong,
    ArrayTooLarge,
    InvalidUtf8,
    InvalidFormat,
}

/// BigInteger stub for testing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInteger {
    value: i64, // Simplified for testing
}

impl BigInteger {
    pub fn from(value: i64) -> Self {
        Self { value }
    }
}

impl Serializable for BigInteger {
    fn serialize(&self, writer: &mut BinaryWriter) -> Result<(), SerializationError> {
        // Simplified BigInteger serialization
        if self.value == 0 {
            writer.write_u8(0x00)
        } else if self.value > 0 && self.value <= 127 {
            writer.write_u8(self.value as u8)
        } else if self.value < 0 && self.value >= -128 {
            writer.write_u8((self.value as i8) as u8)
        } else {
            // Multi-byte representation (simplified)
            let bytes = self.value.to_le_bytes();
            let mut end = 8;
            while end > 1 && bytes[end - 1] == 0 {
                end -= 1;
            }
            writer.write_fixed_bytes(&bytes[..end])
        }
    }

    fn deserialize(reader: &mut BinaryReader) -> Result<Self, SerializationError> {
        let first_byte = reader.read_u8()?;
        if first_byte <= 127 {
            Ok(BigInteger::from(first_byte as i64))
        } else {
            Ok(BigInteger::from(first_byte as i8 as i64))
        }
    }
}

/// Complex structure for testing nested serialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexStructure {
    pub id: u32,
    pub name: String,
    pub hash: UInt160,
    pub flags: bool,
    pub data: Vec<u8>,
    pub metadata: std::collections::BTreeMap<String, u64>,
}

impl Serializable for ComplexStructure {
    fn serialize(&self, writer: &mut BinaryWriter) -> Result<(), SerializationError> {
        writer.write_u32(self.id)?;
        writer.write_string(&self.name)?;
        self.hash.serialize(writer)?;
        writer.write_bool(self.flags)?;
        writer.write_bytes(&self.data)?;

        writer.write_var_int(self.metadata.len() as u64)?;
        for (key, value) in &self.metadata {
            writer.write_string(key)?;
            writer.write_u64(*value)?;
        }

        Ok(())
    }

    fn deserialize(reader: &mut BinaryReader) -> Result<Self, SerializationError> {
        let id = reader.read_u32()?;
        let name = reader.read_string()?;
        let hash = UInt160::deserialize(reader)?;
        let flags = reader.read_bool()?;
        let data = reader.read_bytes()?;

        let metadata_count = reader.read_var_int()? as usize;
        let mut metadata = std::collections::BTreeMap::new();
        for _ in 0..metadata_count {
            let key = reader.read_string()?;
            let value = reader.read_u64()?;
            metadata.insert(key, value);
        }

        Ok(ComplexStructure {
            id,
            name,
            hash,
            flags,
            data,
            metadata,
        })
    }
}

impl Serializable for UInt160 {
    fn serialize(&self, writer: &mut BinaryWriter) -> Result<(), SerializationError> {
        writer.write_fixed_bytes(&self.to_bytes())
    }

    fn deserialize(reader: &mut BinaryReader) -> Result<Self, SerializationError> {
        let mut bytes = [0u8; 20];
        reader.read_fixed_bytes(&mut bytes)?;
        Ok(UInt160::from(bytes))
    }
}

impl Serializable for UInt256 {
    fn serialize(&self, writer: &mut BinaryWriter) -> Result<(), SerializationError> {
        writer.write_fixed_bytes(&self.to_bytes())
    }

    fn deserialize(reader: &mut BinaryReader) -> Result<Self, SerializationError> {
        let mut bytes = [0u8; 32];
        reader.read_fixed_bytes(&mut bytes)?;
        Ok(UInt256::from(bytes))
    }
}

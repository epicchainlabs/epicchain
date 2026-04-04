//! Storage key implementation for smart contract storage.

use crate::{Error, Result};
use neo_config::{ADDRESS_SIZE, MAX_SCRIPT_SIZE};
use neo_core::UInt160;
use neo_io::Serializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a key in the smart contract storage system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct StorageKey {
    /// The contract hash that owns this storage key.
    pub contract: UInt160,

    /// The key data.
    pub key: Vec<u8>,
}

impl StorageKey {
    /// Creates a new storage key.
    pub fn new(contract: UInt160, key: Vec<u8>) -> Self {
        Self { contract, key }
    }

    /// Creates a storage key from a contract and string key.
    pub fn from_string(contract: UInt160, key: &str) -> Self {
        Self::new(contract, key.as_bytes().to_vec())
    }

    /// Creates a storage key from a contract and integer key.
    pub fn from_int(contract: UInt160, key: i32) -> Self {
        Self::new(contract, key.to_le_bytes().to_vec())
    }

    /// Gets the size of the storage key in bytes.
    pub fn size(&self) -> usize {
        ADDRESS_SIZE + // contract hash
        4 + // key length
        self.key.len() // key data
    }

    /// Converts the key to a hex string.
    pub fn to_hex_string(&self) -> String {
        hex::encode(&self.key)
    }

    /// Creates a storage key from a hex string.
    pub fn from_hex_string(contract: UInt160, hex: &str) -> Result<Self> {
        let key = hex::decode(hex)
            .map_err(|e| Error::StorageError(format!("Invalid hex string: {}", e)))?;
        Ok(Self::new(contract, key))
    }

    /// Checks if this key has a specific prefix.
    pub fn has_prefix(&self, prefix: &[u8]) -> bool {
        self.key.starts_with(prefix)
    }

    /// Creates a new key with an additional suffix.
    pub fn with_suffix(&self, suffix: &[u8]) -> Self {
        let mut new_key = self.key.clone();
        new_key.extend_from_slice(suffix);
        Self::new(self.contract, new_key)
    }

    /// Creates a new key with an additional prefix.
    pub fn with_prefix(&self, prefix: &[u8]) -> Self {
        let mut new_key = prefix.to_vec();
        new_key.extend_from_slice(&self.key);
        Self::new(self.contract, new_key)
    }

    /// Gets the key as a string if it's valid UTF-8.
    pub fn as_string(&self) -> Option<String> {
        String::from_utf8(self.key.clone()).ok()
    }

    /// Gets the key as an integer if it's 4 bytes.
    pub fn as_int(&self) -> Option<i32> {
        if self.key.len() == 4 {
            Some(i32::from_le_bytes([
                self.key[0],
                self.key[1],
                self.key[2],
                self.key[3],
            ]))
        } else {
            None
        }
    }
}

impl fmt::Display for StorageKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.contract, self.to_hex_string())
    }
}

impl Serializable for StorageKey {
    fn size(&self) -> usize {
        // Calculate the size of the serialized StorageKey
        // This matches C# Neo's StorageKey.Size property exactly
        ADDRESS_SIZE + // contract (UInt160)
        1 + // key length prefix
        self.key.len() // key bytes
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::Result<()> {
        writer.write_bytes(&self.contract.as_bytes())?;
        writer.write_var_bytes(&self.key)?;
        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::Result<Self> {
        let contract_bytes = reader.read_bytes(ADDRESS_SIZE)?;
        let contract = UInt160::from_bytes(&contract_bytes)
            .map_err(|e| neo_io::Error::InvalidData(e.to_string()))?;
        let key = reader.read_var_bytes(MAX_SCRIPT_SIZE)?; // Max MAX_SCRIPT_SIZE bytes for storage key

        Ok(StorageKey { contract, key })
    }
}

impl From<(UInt160, Vec<u8>)> for StorageKey {
    fn from((contract, key): (UInt160, Vec<u8>)) -> Self {
        Self::new(contract, key)
    }
}

impl From<(UInt160, &str)> for StorageKey {
    fn from((contract, key): (UInt160, &str)) -> Self {
        Self::from_string(contract, key)
    }
}

impl From<(UInt160, i32)> for StorageKey {
    fn from((contract, key): (UInt160, i32)) -> Self {
        Self::from_int(contract, key)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{StorageError, StorageKey, Store};

    #[test]
    fn test_storage_key_creation() {
        let contract = UInt160::zero();
        let key_data = b"test_key".to_vec();
        let storage_key = StorageKey::new(contract, key_data.clone());

        assert_eq!(storage_key.contract, contract);
        assert_eq!(storage_key.key, key_data);
    }

    #[test]
    fn test_storage_key_from_string() {
        let contract = UInt160::zero();
        let key_str = "test_key";
        let storage_key = StorageKey::from_string(contract, key_str);

        assert_eq!(storage_key.key, key_str.as_bytes());
        assert_eq!(storage_key.as_string(), Some(key_str.to_string()));
    }

    #[test]
    fn test_storage_key_from_int() {
        let contract = UInt160::zero();
        let key_int = 12345i32;
        let storage_key = StorageKey::from_int(contract, key_int);

        assert_eq!(storage_key.key, key_int.to_le_bytes().to_vec());
        assert_eq!(storage_key.as_int(), Some(key_int));
    }

    #[test]
    fn test_storage_key_hex_conversion() {
        let contract = UInt160::zero();
        let key_data = vec![0x01, 0x02, 0x03, 0x04];
        let storage_key = StorageKey::new(contract, key_data);

        let hex_string = storage_key.to_hex_string();
        assert_eq!(hex_string, "01020304");

        let from_hex = StorageKey::from_hex_string(contract, &hex_string).unwrap();
        assert_eq!(from_hex, storage_key);
    }

    #[test]
    fn test_storage_key_prefix_suffix() {
        let contract = UInt160::zero();
        let storage_key = StorageKey::from_string(contract, "key");

        let with_prefix = storage_key.with_prefix(b"prefix_");
        assert!(with_prefix.has_prefix(b"prefix_"));
        assert_eq!(with_prefix.as_string(), Some("prefix_key".to_string()));

        let with_suffix = storage_key.with_suffix(b"_suffix");
        assert_eq!(with_suffix.as_string(), Some("key_suffix".to_string()));
    }

    #[test]
    fn test_storage_key_size() {
        let contract = UInt160::zero();
        let storage_key = StorageKey::from_string(contract, "test");

        let expected_size = ADDRESS_SIZE + 4 + 4; // contract + length + key
        assert_eq!(storage_key.size(), expected_size);
    }

    #[test]
    fn test_storage_key_display() {
        let contract = UInt160::zero();
        let storage_key = StorageKey::from_string(contract, "test");

        let display_string = storage_key.to_string();
        assert!(display_string.contains(&contract.to_string()));
        assert!(display_string.contains(&storage_key.to_hex_string()));
    }

    #[test]
    fn test_storage_key_from_conversions() {
        let contract = UInt160::zero();

        let from_vec: StorageKey = (contract, b"test".to_vec()).into();
        assert_eq!(from_vec.key, b"test");

        let from_str: StorageKey = (contract, "test").into();
        assert_eq!(from_str.key, b"test");

        let from_int: StorageKey = (contract, 123i32).into();
        assert_eq!(from_int.as_int(), Some(123));
    }
}

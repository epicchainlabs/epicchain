// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Implementation of UInt160, a 160-bit unsigned integer.

use crate::error::{CoreError, CoreResult};
use neo_config::ADDRESS_SIZE;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use tracing::error;

/// The length of UInt160 values in bytes.
pub const UINT160_SIZE: usize = ADDRESS_SIZE;

/// Represents a 160-bit unsigned integer.
///
/// This is implemented as a reference type to match the C# implementation.
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UInt160 {
    /// First 8 bytes of the UInt160 (least significant).
    pub value1: u64,
    /// Next 8 bytes of the UInt160.
    pub value2: u64,
    /// Last 4 bytes of the UInt160 (most significant).
    pub value3: u32,
}

/// Zero value for UInt160.
pub static ZERO: UInt160 = UInt160 {
    value1: 0,
    value2: 0,
    value3: 0,
};

impl UInt160 {
    /// Creates a new UInt160 instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a zero UInt160.
    pub fn zero() -> Self {
        Self::default()
    }

    /// Checks if this UInt160 is zero (matches C# IsZero property).
    pub fn is_zero(&self) -> bool {
        self.value1 == 0 && self.value2 == 0 && self.value3 == 0
    }

    /// Returns the bytes representation of this UInt160.
    pub fn as_bytes(&self) -> [u8; ADDRESS_SIZE] {
        self.to_array()
    }

    /// Returns the bytes as a Vec<u8>
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ADDRESS_SIZE);
        bytes.extend_from_slice(&self.value1.to_le_bytes());
        bytes.extend_from_slice(&self.value2.to_le_bytes());
        bytes.extend_from_slice(&self.value3.to_le_bytes());
        bytes
    }

    /// Determines whether this instance and another specified UInt160 object have the same value.
    ///
    /// # Arguments
    ///
    /// * `other` - The UInt160 to compare to this instance.
    ///
    /// # Returns
    ///
    /// true if the value of the value parameter is the same as this instance; otherwise, false.
    pub fn equals(&self, other: Option<&Self>) -> bool {
        if let Some(other) = other {
            self.value1 == other.value1
                && self.value2 == other.value2
                && self.value3 == other.value3
        } else {
            false
        }
    }

    /// Creates a new UInt160 from a byte array.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte array to create the UInt160 from.
    ///
    /// # Returns
    ///
    /// A new UInt160 instance.
    pub fn from_bytes(value: &[u8]) -> CoreResult<Self> {
        if value.len() != UINT160_SIZE {
            return Err(CoreError::InvalidFormat {
                message: format!("Invalid length: {}", value.len()),
            });
        }

        let mut result = Self::new();

        let mut value1_bytes = [0u8; 8];
        let mut value2_bytes = [0u8; 8];
        let mut value3_bytes = [0u8; 4];

        value1_bytes.copy_from_slice(&value[0..8]);
        value2_bytes.copy_from_slice(&value[8..16]);
        value3_bytes.copy_from_slice(&value[16..ADDRESS_SIZE]);

        result.value1 = u64::from_le_bytes(value1_bytes);
        result.value2 = u64::from_le_bytes(value2_bytes);
        result.value3 = u32::from_le_bytes(value3_bytes);

        Ok(result)
    }

    /// Creates a new UInt160 from a byte span.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte span to create the UInt160 from.
    ///
    /// # Returns
    ///
    /// A new UInt160 instance.
    pub fn from_span(value: &[u8]) -> Self {
        if value.len() != UINT160_SIZE {
            // Return zero hash instead of panicking in production
            error!(
                "Invalid UInt160 length: {} (expected {})",
                value.len(),
                UINT160_SIZE
            );
            return Self::zero();
        }

        let mut result = Self::new();

        let mut value1_bytes = [0u8; 8];
        let mut value2_bytes = [0u8; 8];
        let mut value3_bytes = [0u8; 4];

        value1_bytes.copy_from_slice(&value[0..8]);
        value2_bytes.copy_from_slice(&value[8..16]);
        value3_bytes.copy_from_slice(&value[16..ADDRESS_SIZE]);

        result.value1 = u64::from_le_bytes(value1_bytes);
        result.value2 = u64::from_le_bytes(value2_bytes);
        result.value3 = u32::from_le_bytes(value3_bytes);

        result
    }

    /// Gets a byte array representation of the UInt160.
    ///
    /// # Returns
    ///
    /// A byte array representation of the UInt160.
    pub fn to_array(&self) -> [u8; UINT160_SIZE] {
        let mut result = [0u8; UINT160_SIZE];

        let value1_bytes = self.value1.to_le_bytes();
        let value2_bytes = self.value2.to_le_bytes();
        let value3_bytes = self.value3.to_le_bytes();

        result[0..8].copy_from_slice(&value1_bytes);
        result[8..16].copy_from_slice(&value2_bytes);
        result[16..ADDRESS_SIZE].copy_from_slice(&value3_bytes);

        result
    }

    /// Gets a span that represents the current value in little-endian.
    ///
    /// # Returns
    ///
    /// A byte array that represents the current value in little-endian.
    pub fn get_span(&self) -> [u8; UINT160_SIZE] {
        self.to_array()
    }

    /// Parses a UInt160 from a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `s` - The hexadecimal string to parse.
    ///
    /// # Returns
    ///
    /// A Result containing either a new UInt160 instance or an error.
    pub fn parse(s: &str) -> CoreResult<Self> {
        let mut result = None;
        if !Self::try_parse(s, &mut result) {
            return Err(CoreError::InvalidFormat {
                message: "Invalid format".to_string(),
            });
        }

        match result {
            Some(value) => Ok(value),
            None => Err(CoreError::InvalidFormat {
                message: "Failed to parse UInt160".to_string(),
            }),
        }
    }

    /// Tries to parse a UInt160 from a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `s` - The hexadecimal string to parse.
    /// * `result` - Optional mutable reference to store the result.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the parsing was successful.
    pub fn try_parse(s: &str, result: &mut Option<Self>) -> bool {
        let s = s.strip_prefix("0x").unwrap_or(s);

        if s.len() != UINT160_SIZE * 2 {
            return false;
        }

        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }

        match hex::decode(s) {
            Ok(bytes) => {
                // Map big-endian hex bytes to little-endian internal storage
                let mut value1_bytes = [0u8; 8];
                let mut value2_bytes = [0u8; 8];
                let mut value3_bytes = [0u8; 4];

                // Correct mapping:

                value3_bytes[3] = bytes[19];
                value3_bytes[2] = bytes[18];
                value3_bytes[1] = bytes[17];
                value3_bytes[0] = bytes[16];

                for i in 0..8 {
                    value2_bytes[7 - i] = bytes[8 + i];
                }

                for i in 0..8 {
                    value1_bytes[7 - i] = bytes[i];
                }

                // Convert to little-endian u64/u32 values
                let mut uint = Self::new();
                uint.value1 = u64::from_le_bytes(value1_bytes);
                uint.value2 = u64::from_le_bytes(value2_bytes);
                uint.value3 = u32::from_le_bytes(value3_bytes);

                *result = Some(uint);

                true
            }
            Err(_) => false,
        }
    }

    /// Converts the UInt160 to a hexadecimal string.
    ///
    /// # Returns
    ///
    /// A hexadecimal string representation of the UInt160.
    pub fn to_hex_string(&self) -> String {
        let mut result_bytes = [0u8; UINT160_SIZE];

        let value1_bytes = self.value1.to_le_bytes();
        let value2_bytes = self.value2.to_le_bytes();
        let value3_bytes = self.value3.to_le_bytes();

        // Reverse the parsing process:
        // value3 -> bytes[16..ADDRESS_SIZE]
        // value2 -> bytes[8..16]
        // value1 -> bytes[0..8]

        result_bytes[19] = value3_bytes[3];
        result_bytes[18] = value3_bytes[2];
        result_bytes[17] = value3_bytes[1];
        result_bytes[16] = value3_bytes[0];

        for i in 0..8 {
            result_bytes[15 - i] = value2_bytes[i];
        }

        for i in 0..8 {
            result_bytes[7 - i] = value1_bytes[i];
        }

        format!("0x{}", hex::encode(result_bytes))
    }

    /// Gets a hash code for the current UInt160 instance.
    ///
    /// # Returns
    ///
    /// A HASH_SIZE-bit signed integer hash code.
    pub fn get_hash_code(&self) -> i32 {
        // Combine the hash codes of the three fields, similar to C#'s HashCode.Combine
        let mut hash = 17;
        hash = hash * 31 + (self.value1 as i32);
        hash = hash * 31 + (self.value2 as i32);
        hash = hash * 31 + self.value3 as i32;
        hash
    }

    /// Creates a UInt160 from a script by computing its hash.
    ///
    /// # Arguments
    ///
    /// * `script` - The script bytes to hash
    ///
    /// # Returns
    ///
    /// A new UInt160 instance representing the script hash.
    pub fn from_script(script: &[u8]) -> Self {
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(script);
        let sha256_hash = sha256_hasher.finalize();

        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha256_hash);
        let hash160 = ripemd_hasher.finalize();

        Self::from_bytes(&hash160).unwrap_or_default()
    }

    /// Converts this UInt160 to a Neo address string.
    ///
    /// # Returns
    ///
    /// A Neo address string in Base58Check format.
    pub fn to_address(&self) -> String {
        let version_byte = 0x35u8; // Neo N3 address version
        let mut data = Vec::with_capacity(21);
        data.push(version_byte);
        data.extend_from_slice(&self.to_array());

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let first_hash = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(first_hash);
        let second_hash = hasher.finalize();

        let checksum = &second_hash[0..4];
        data.extend_from_slice(checksum);

        // Encode in Base58
        bs58::encode(data).into_string()
    }

    // Duplicate method removed - using implementation at line 307

    // Duplicate method removed - using implementation at line 183

    /// Parses a Neo address string to a UInt160.
    ///
    /// # Arguments
    ///
    /// * `address` - The Neo address string to parse
    ///
    /// # Returns
    ///
    /// A Result containing either a new UInt160 instance or an error.
    pub fn from_address(address: &str) -> CoreResult<Self> {
        // Decode from Base58
        let decoded = bs58::decode(address)
            .into_vec()
            .map_err(|_| CoreError::InvalidFormat {
                message: "Invalid Base58 address".to_string(),
            })?;

        if decoded.len() != 25 {
            return Err(CoreError::InvalidFormat {
                message: "Invalid address length".to_string(),
            });
        }

        // Check version byte
        if decoded[0] != 0x35 {
            return Err(CoreError::InvalidFormat {
                message: "Invalid address version".to_string(),
            });
        }

        // Verify checksum
        let data = &decoded[0..21];
        let checksum = &decoded[21..25];

        let mut hasher = Sha256::new();
        hasher.update(data);
        let first_hash = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(first_hash);
        let second_hash = hasher.finalize();

        let computed_checksum = &second_hash[0..4];
        if checksum != computed_checksum {
            return Err(CoreError::InvalidFormat {
                message: "Invalid address checksum".to_string(),
            });
        }

        let script_hash = &decoded[1..21];
        Self::from_bytes(script_hash)
    }
}

impl neo_io::Serializable for UInt160 {
    fn size(&self) -> usize {
        UINT160_SIZE
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::IoResult<()> {
        writer.write_u64(self.value1)?;
        writer.write_u64(self.value2)?;
        writer.write_u32(self.value3)?;
        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::IoResult<Self> {
        let value1 = reader.read_u64()?;
        let value2 = reader.read_u64()?;
        let value3 = reader.read_u32()?;
        Ok(UInt160 {
            value1,
            value2,
            value3,
        })
    }
}

impl FromStr for UInt160 {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl fmt::Display for UInt160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl fmt::Debug for UInt160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UInt160({})", self.to_hex_string())
    }
}

impl PartialOrd for UInt160 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UInt160 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Most significant bytes first
        let result = self.value3.cmp(&other.value3);
        if result != Ordering::Equal {
            return result;
        }
        let result = self.value2.cmp(&other.value2);
        if result != Ordering::Equal {
            return result;
        }
        self.value1.cmp(&other.value1)
    }
}

impl From<[u8; UINT160_SIZE]> for UInt160 {
    fn from(data: [u8; UINT160_SIZE]) -> Self {
        // This is safe because we have exactly UINT160_SIZE bytes
        Self::from_bytes(&data).expect("Fixed-size array should always be valid")
    }
}

impl TryFrom<&[u8]> for UInt160 {
    type Error = CoreError;

    fn try_from(data: &[u8]) -> std::result::Result<Self, Self::Error> {
        Self::from_bytes(data)
    }
}

impl From<&str> for UInt160 {
    fn from(s: &str) -> Self {
        Self::parse(s).unwrap_or_default()
    }
}

impl From<Vec<u8>> for UInt160 {
    fn from(data: Vec<u8>) -> Self {
        Self::from_bytes(&data).unwrap_or_default()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_uint160_new() {
        let uint = UInt160::new();
        assert_eq!(uint.value1, 0);
        assert_eq!(uint.value2, 0);
        assert_eq!(uint.value3, 0);
    }
    #[test]
    fn test_uint160_from_bytes() {
        let mut bytes = [0u8; ADDRESS_SIZE];
        bytes[0] = 1; // Set first byte to 1
        let uint = UInt160::from_bytes(&bytes).unwrap();
        assert_eq!(uint.value1, 1); // Should be 1 in little-endian
        assert_eq!(uint.value2, 0);
        assert_eq!(uint.value3, 0);
    }
    #[test]
    fn test_uint160_to_array() {
        let mut uint = UInt160::new();
        uint.value1 = 1;
        let bytes = uint.to_array();
        assert_eq!(bytes[0], 1); // First byte should be 1
        assert_eq!(bytes[1], 0);
    }
    #[test]
    fn test_uint160_parse() {
        let hex_str = "0x0000000000000000000000000000000000000001";
        let uint = UInt160::parse(hex_str).unwrap();
        assert_eq!(uint.value1, 0);
        assert_eq!(uint.value2, 0);
        assert_eq!(uint.value3, 0x01000000);
    }
    #[test]
    fn test_uint160_try_parse() {
        let mut result = None;
        assert!(UInt160::try_parse(
            "0x0000000000000000000000000000000000000001",
            &mut result
        ));
        assert!(result.is_some());
        result = None;
        assert!(!UInt160::try_parse("0x01", &mut result));
        result = None;
        assert!(!UInt160::try_parse(
            "0x000000000000000000000000000000000000000g",
            &mut result
        ));
    }
    #[test]
    fn test_uint160_to_hex_string() {
        let mut uint = UInt160::new();
        uint.value3 = 0x01000000; // 1 in little-endian
        let hex_str = uint.to_hex_string();
        assert_eq!(hex_str, "0x0000000000000000000000000000000000000001");
    }
    #[test]
    fn test_uint160_serialization() {
        let mut uint = UInt160::new();
        uint.value1 = 0x1234567890abcdef;
        uint.value2 = 0xfedcba0987654321;
        uint.value3 = 0x12345678;
        // Test serialization
        let mut writer = neo_io::BinaryWriter::new();
        <UInt160 as neo_io::Serializable>::serialize(&uint, &mut writer).unwrap();
        let bytes = writer.to_bytes();
        // Test deserialization
        let mut reader = neo_io::MemoryReader::new(&bytes);
        let deserialized = <UInt160 as neo_io::Serializable>::deserialize(&mut reader).unwrap();
        assert_eq!(uint, deserialized);
    }
    #[test]
    fn test_uint160_ordering() {
        let uint1 = UInt160 {
            value1: 1,
            value2: 0,
            value3: 0,
        };
        let uint2 = UInt160 {
            value1: 0,
            value2: 1,
            value3: 0,
        };
        let uint3 = UInt160 {
            value1: 0,
            value2: 0,
            value3: 1,
        };
        assert!(uint3 > uint2);
        assert!(uint2 > uint1);
        assert!(uint3 > uint1);
    }
    #[test]
    fn test_uint160_from_string() {
        let hex_str = "0x0000000000000000000000000000000000000001";
        let uint1 = UInt160::from(hex_str);
        let uint2 = UInt160::parse(hex_str).unwrap();
        assert_eq!(uint1, uint2);
    }
    #[test]
    fn test_uint160_equals() {
        let uint1 = UInt160 {
            value1: 1,
            value2: 2,
            value3: 3,
        };
        let uint2 = UInt160 {
            value1: 1,
            value2: 2,
            value3: 3,
        };
        let uint3 = UInt160 {
            value1: 1,
            value2: 2,
            value3: 4,
        };
        assert!(uint1.equals(Some(&uint2)));
        assert!(!uint1.equals(Some(&uint3)));
        assert!(!uint1.equals(None));
    }
    #[test]
    fn test_uint160_get_hash_code() {
        let uint1 = UInt160 {
            value1: 1,
            value2: 2,
            value3: 3,
        };
        let uint2 = UInt160 {
            value1: 1,
            value2: 2,
            value3: 3,
        };
        // Same values should produce same hash code
        assert_eq!(uint1.get_hash_code(), uint2.get_hash_code());
    }
    #[test]
    fn test_uint160_from_script() {
        let script = b"Hello, Neo!";
        let uint = UInt160::from_script(script);
        assert_eq!(uint.value1, 11694377989620867361);
        assert_eq!(uint.value2, 788152587155704251);
        assert_eq!(uint.value3, 4052539232);
    }
    #[test]
    fn test_uint160_to_address() {
        let uint = UInt160 {
            value1: 0x3530303030303030,
            value2: 0x3030303030303030,
            value3: 0x30303030,
        };
        let address = uint.to_address();
        assert!(!address.is_empty());
        assert!(address.starts_with('N') || address.starts_with('A')); // Neo addresses typically start with N or A
    }
}

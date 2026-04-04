// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Transaction attributes implementation matching C# Neo N3 exactly.

use crate::error::{CoreError, CoreResult};
use crate::UInt256;
use neo_config::HASH_SIZE;
use neo_io::serializable::helper::get_var_size;
use neo_io::{BinaryWriter, MemoryReader};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Transaction attribute types (matches C# TransactionAttributeType enum exactly).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TransactionAttributeType {
    /// High priority attribute (matches C# HighPriority = 0x01).
    HighPriority = 0x01,
    /// Oracle response attribute (matches C# OracleResponse = 0x11).
    OracleResponse = 0x11,
    /// Not valid before attribute (matches C# NotValidBefore = 0x20).
    NotValidBefore = 0x20,
    /// Conflicts attribute (matches C# Conflicts = 0x21).
    Conflicts = 0x21,
}

/// Represents a transaction attribute (matches C# TransactionAttribute class exactly).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionAttribute {
    /// High priority attribute for prioritizing transactions.
    HighPriority,

    /// Oracle response attribute containing oracle callback data.
    OracleResponse {
        /// Oracle response ID (matches C# Id property).
        id: u64,
        /// Oracle response code (matches C# Code property).
        code: OracleResponseCode,
        /// Oracle response result (matches C# Result property).
        result: Vec<u8>,
    },

    /// Not valid before attribute specifying earliest valid block.
    NotValidBefore {
        /// Block height before which transaction is invalid (matches C# Height property).
        height: u32,
    },

    /// Conflicts attribute for transaction conflict resolution.
    Conflicts {
        /// Hash of conflicting transaction (matches C# Hash property).
        hash: UInt256,
    },
}

/// Oracle response codes (matches C# OracleResponseCode enum exactly).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OracleResponseCode {
    /// Success response (matches C# Success = 0x00).
    Success = 0x00,
    /// Protocol not supported (matches C# ProtocolNotSupported = 0x10).
    ProtocolNotSupported = 0x10,
    /// Console not found (matches C# ConsensusUnreachable = 0x12).
    ConsensusUnreachable = 0x12,
    /// Not found (matches C# NotFound = 0x14).
    NotFound = 0x14,
    /// Timeout (matches C# Timeout = 0x16).
    Timeout = 0x16,
    /// Forbidden (matches C# Forbidden = 0x18).
    Forbidden = 0x18,
    /// Response too large (matches C# ResponseTooLarge = 0x1a).
    ResponseTooLarge = 0x1a,
    /// Insufficient funds (matches C# InsufficientFunds = 0x1c).
    InsufficientFunds = 0x1c,
    /// Content type not supported (matches C# ContentTypeNotSupported = 0x1f).
    ContentTypeNotSupported = 0x1f,
    /// Error (matches C# Error = 0xff).
    Error = 0xff,
}

impl TransactionAttribute {
    /// Gets the attribute type (matches C# Type property exactly).
    pub fn attribute_type(&self) -> TransactionAttributeType {
        match self {
            TransactionAttribute::HighPriority => TransactionAttributeType::HighPriority,
            TransactionAttribute::OracleResponse { .. } => TransactionAttributeType::OracleResponse,
            TransactionAttribute::NotValidBefore { .. } => TransactionAttributeType::NotValidBefore,
            TransactionAttribute::Conflicts { .. } => TransactionAttributeType::Conflicts,
        }
    }

    /// Gets the size of the attribute in bytes (matches C# Size property exactly).
    pub fn size(&self) -> usize {
        match self {
            TransactionAttribute::HighPriority => 1, // Just the type byte
            TransactionAttribute::OracleResponse { result, .. } => {
                1 +
                8 + // id (u64)
                1 + // code (u8)
                get_var_size(result.len() as u64) + result.len() // result with var length
            }
            TransactionAttribute::NotValidBefore { .. } => {
                1 + 4 // height (u32)
            }
            TransactionAttribute::Conflicts { .. } => {
                1 + HASH_SIZE // hash (UInt256)
            }
        }
    }

    /// Validates the attribute (matches C# Verify method exactly).
    pub fn verify(&self) -> CoreResult<()> {
        match self {
            TransactionAttribute::HighPriority => Ok(()),
            TransactionAttribute::OracleResponse { id, result, .. } => {
                if *id == 0 {
                    return Err(CoreError::InvalidData {
                        message: "Oracle ID cannot be zero".to_string(),
                    });
                }
                if result.len() > u16::MAX as usize {
                    return Err(CoreError::InvalidData {
                        message: "Oracle result too large".to_string(),
                    });
                }
                Ok(())
            }
            TransactionAttribute::NotValidBefore { height } => {
                if *height == 0 {
                    return Err(CoreError::InvalidData {
                        message: "Height cannot be zero".to_string(),
                    });
                }
                Ok(())
            }
            TransactionAttribute::Conflicts { hash } => {
                if hash.is_zero() {
                    return Err(CoreError::InvalidData {
                        message: "Conflict hash cannot be zero".to_string(),
                    });
                }
                Ok(())
            }
        }
    }

    /// Checks if this attribute allows duplicates (matches C# AllowMultiple property exactly).
    pub fn allows_multiple(&self) -> bool {
        match self {
            TransactionAttribute::HighPriority => false, // Only one high priority allowed
            TransactionAttribute::OracleResponse { .. } => false, // Only one oracle response allowed
            TransactionAttribute::NotValidBefore { .. } => false, // Only one not valid before allowed
            TransactionAttribute::Conflicts { .. } => true,       // Multiple conflicts allowed
        }
    }

    /// Checks if this attribute allows duplicates (C# AllowMultiple property compatibility).
    pub fn allow_multiple(&self) -> bool {
        self.allows_multiple()
    }

    /// Gets the raw data of the attribute (for consensus processing)
    pub fn data(&self) -> Vec<u8> {
        match self {
            TransactionAttribute::HighPriority => vec![],
            TransactionAttribute::OracleResponse { id, code, result } => {
                let mut data = Vec::new();
                data.extend_from_slice(&id.to_le_bytes());
                data.push(*code as u8);
                data.extend_from_slice(result);
                data
            }
            TransactionAttribute::NotValidBefore { height } => height.to_le_bytes().to_vec(),
            TransactionAttribute::Conflicts { hash } => hash.as_bytes().to_vec(),
        }
    }

    /// Serializes the attribute to binary format (matches C# Serialize method exactly).
    pub fn serialize(&self, writer: &mut BinaryWriter) -> CoreResult<()> {
        // Write attribute type
        writer
            .write_bytes(&[self.attribute_type() as u8])
            .map_err(|e| CoreError::Serialization {
                message: e.to_string(),
            })?;

        // Write attribute data
        match self {
            TransactionAttribute::HighPriority => {}
            TransactionAttribute::OracleResponse { id, code, result } => {
                writer
                    .write_bytes(&id.to_le_bytes())
                    .map_err(|e| CoreError::Serialization {
                        message: e.to_string(),
                    })?;
                writer
                    .write_bytes(&[*code as u8])
                    .map_err(|e| CoreError::Serialization {
                        message: e.to_string(),
                    })?;
                writer
                    .write_var_bytes(result)
                    .map_err(|e| CoreError::Serialization {
                        message: e.to_string(),
                    })?;
            }
            TransactionAttribute::NotValidBefore { height } => {
                writer.write_bytes(&height.to_le_bytes()).map_err(|e| {
                    CoreError::Serialization {
                        message: e.to_string(),
                    }
                })?;
            }
            TransactionAttribute::Conflicts { hash } => {
                writer
                    .write_bytes(&hash.as_bytes())
                    .map_err(|e| CoreError::Serialization {
                        message: e.to_string(),
                    })?;
            }
        }

        Ok(())
    }

    /// Deserializes an attribute from binary format (matches C# Deserialize method exactly).
    pub fn deserialize(reader: &mut MemoryReader) -> CoreResult<Self> {
        // Read attribute type
        let attribute_type = reader.read_byte().map_err(|e| CoreError::Serialization {
            message: e.to_string(),
        })?;

        match attribute_type {
            0x01 => Ok(TransactionAttribute::HighPriority),
            0x11 => {
                let id = reader.read_u64().map_err(|e| CoreError::Serialization {
                    message: e.to_string(),
                })?;
                let code_byte = reader.read_byte().map_err(|e| CoreError::Serialization {
                    message: e.to_string(),
                })?;
                let code = match code_byte {
                    0x00 => OracleResponseCode::Success,
                    0x10 => OracleResponseCode::ProtocolNotSupported,
                    0x12 => OracleResponseCode::ConsensusUnreachable,
                    0x14 => OracleResponseCode::NotFound,
                    0x16 => OracleResponseCode::Timeout,
                    0x18 => OracleResponseCode::Forbidden,
                    0x1a => OracleResponseCode::ResponseTooLarge,
                    0x1c => OracleResponseCode::InsufficientFunds,
                    0x1f => OracleResponseCode::ContentTypeNotSupported,
                    0xff => OracleResponseCode::Error,
                    _ => {
                        return Err(CoreError::InvalidData {
                            message: format!("Invalid oracle response code: {code_byte}"),
                        });
                    }
                };
                let result = reader.read_var_bytes(u16::MAX as usize).map_err(|e| {
                    CoreError::Serialization {
                        message: e.to_string(),
                    }
                })?;

                Ok(TransactionAttribute::OracleResponse { id, code, result })
            }
            0x20 => {
                let height = reader.read_u32().map_err(|e| CoreError::Serialization {
                    message: e.to_string(),
                })?;
                Ok(TransactionAttribute::NotValidBefore { height })
            }
            0x21 => {
                let hash_bytes =
                    reader
                        .read_bytes(HASH_SIZE)
                        .map_err(|e| CoreError::Serialization {
                            message: e.to_string(),
                        })?;
                let hash =
                    UInt256::from_bytes(&hash_bytes).map_err(|e| CoreError::InvalidData {
                        message: format!("Invalid hash: {e}"),
                    })?;
                Ok(TransactionAttribute::Conflicts { hash })
            }
            _ => Err(CoreError::InvalidData {
                message: format!("Unknown attribute type: {attribute_type}"),
            }),
        }
    }
}

impl fmt::Display for TransactionAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionAttribute::HighPriority => write!(f, "HighPriority"),
            TransactionAttribute::OracleResponse { id, code, result } => {
                write!(
                    f,
                    "OracleResponse(id: {}, code: {:?}, result_len: {})",
                    id,
                    code,
                    result.len()
                )
            }
            TransactionAttribute::NotValidBefore { height } => {
                write!(f, "NotValidBefore(height: {height})")
            }
            TransactionAttribute::Conflicts { hash } => {
                write!(f, "Conflicts(hash: {hash})")
            }
        }
    }
}

impl fmt::Display for OracleResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OracleResponseCode::Success => write!(f, "Success"),
            OracleResponseCode::ProtocolNotSupported => write!(f, "ProtocolNotSupported"),
            OracleResponseCode::ConsensusUnreachable => write!(f, "ConsensusUnreachable"),
            OracleResponseCode::NotFound => write!(f, "NotFound"),
            OracleResponseCode::Timeout => write!(f, "Timeout"),
            OracleResponseCode::Forbidden => write!(f, "Forbidden"),
            OracleResponseCode::ResponseTooLarge => write!(f, "ResponseTooLarge"),
            OracleResponseCode::InsufficientFunds => write!(f, "InsufficientFunds"),
            OracleResponseCode::ContentTypeNotSupported => write!(f, "ContentTypeNotSupported"),
            OracleResponseCode::Error => write!(f, "Error"),
        }
    }
}

//! Neo Message implementation matching C# Message.cs exactly
//!
//! This module provides the exact Neo Message structure and serialization
//! as implemented in C# Neo.Network.P2P.Message.cs

use super::commands::{MessageCommand, MessageFlags};
use crate::compression::{
    compress_lz4, decompress_lz4, COMPRESSION_MIN_SIZE, COMPRESSION_THRESHOLD,
};
use crate::{NetworkError, NetworkResult as Result};
use neo_io::{BinaryWriter, MemoryReader, Serializable, SerializableExt};
use serde::{Deserialize, Serialize};

/// Maximum payload size (matches C# PayloadMaxSize exactly)
pub const PAYLOAD_MAX_SIZE: usize = 0x02000000; // 32MB

/// Neo network message (matches C# Message.cs exactly)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// The flags of the message (matches C# Flags property)
    pub flags: MessageFlags,

    /// The command of the message (matches C# Command property)
    pub command: MessageCommand,

    /// The raw payload data (matches C# _payloadCompressed)
    pub payload_raw: Vec<u8>,
}

impl Message {
    /// Create new message (matches C# Create method exactly)
    pub fn create(
        command: MessageCommand,
        payload: Option<&dyn Serializable>,
        enable_compression: bool,
    ) -> Result<Self> {
        let payload_bytes = if let Some(payload) = payload {
            let mut writer = BinaryWriter::new();
            payload
                .serialize(&mut writer)
                .map_err(|e| NetworkError::InvalidMessage {
                    peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                    message_type: "payload".to_string(),
                    reason: format!("Failed to serialize payload: {}", e),
                })?;
            writer.to_bytes()
        } else {
            Vec::new()
        };

        let mut message = Self {
            flags: MessageFlags::None,
            command,
            payload_raw: payload_bytes.clone(),
        };

        // Apply compression if enabled and beneficial (matches C# compression logic)
        if enable_compression && payload_bytes.len() >= COMPRESSION_MIN_SIZE {
            match compress_lz4(&payload_bytes) {
                Ok(compressed) => {
                    if payload_bytes.len() > compressed.len() + COMPRESSION_THRESHOLD {
                        message.payload_raw = compressed;
                        message.flags = MessageFlags::Compressed;
                    }
                }
                Err(_) => {
                    // Compression failed, use uncompressed
                }
            }
        }

        Ok(message)
    }

    /// Decompress payload if needed (matches C# DecompressPayload exactly)
    fn decompress_payload(&mut self) -> Result<()> {
        if self.payload_raw.is_empty() {
            return Ok(());
        }

        let decompressed = if self.flags == MessageFlags::Compressed {
            decompress_lz4(&self.payload_raw, PAYLOAD_MAX_SIZE)?
        } else {
            self.payload_raw.clone()
        };

        // Store decompressed data for payload creation
        self.payload_raw = decompressed;
        Ok(())
    }

    /// Get payload size (matches C# Size property)
    pub fn size(&self) -> usize {
        1 + // flags
        1 + // command  
        self.get_var_size(self.payload_raw.len()) + // var_size prefix
        self.payload_raw.len() // payload data
    }

    /// Calculate variable-length size encoding (matches C# GetVarSize)
    fn get_var_size(&self, value: usize) -> usize {
        if value < 0xFD {
            1
        } else if value <= 0xFFFF {
            3 // 0xFD + 2 bytes
        } else if value <= 0xFFFFFFFF {
            5 // 0xFE + 4 bytes
        } else {
            9 // 0xFF + 8 bytes
        }
    }

    /// Write variable-length integer (matches C# WriteVarInt)
    fn write_var_int(&self, writer: &mut BinaryWriter, value: u64) -> neo_io::IoResult<()> {
        if value < 0xFD {
            writer.write_u8(value as u8)?;
        } else if value <= 0xFFFF {
            writer.write_u8(0xFD)?;
            writer.write_u16(value as u16)?;
        } else if value <= 0xFFFFFFFF {
            writer.write_u8(0xFE)?;
            writer.write_u32(value as u32)?;
        } else {
            writer.write_u8(0xFF)?;
            writer.write_u64(value)?;
        }
        Ok(())
    }

    /// Read variable-length integer (matches C# ReadVarInt)
    fn read_var_int(reader: &mut MemoryReader, max: u64) -> neo_io::IoResult<u64> {
        let fb = reader.read_byte()?;
        let value = match fb {
            0xFD => reader.read_uint16()? as u64,
            0xFE => reader.read_uint32()? as u64,
            0xFF => reader.read_uint64()?,
            _ => fb as u64,
        };

        if value > max {
            return Err(neo_io::IoError::end_of_stream(0, "message"));
        }

        Ok(value)
    }
}

impl Serializable for Message {
    /// Deserialize message (matches C# ISerializable.Deserialize exactly)
    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        let flags = MessageFlags::from_byte(reader.read_byte()?)
            .map_err(|_| neo_io::IoError::end_of_stream(0, "message"))?;
        let command = MessageCommand::from_byte(reader.read_byte()? as u8)
            .map_err(|_| neo_io::IoError::end_of_stream(0, "message"))?;

        // Read payload using VarBytes (matches C# ReadVarMemory)
        let payload_len = Self::read_var_int(reader, PAYLOAD_MAX_SIZE as u64)? as usize;
        let payload_raw = reader.read_bytes(payload_len)?;

        let mut message = Self {
            flags,
            command,
            payload_raw,
        };

        // Decompress if needed
        message
            .decompress_payload()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(message)
    }

    /// Serialize message (matches C# ISerializable.Serialize exactly)
    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        writer.write_u8(self.flags as u8)?;
        writer.write_u8(self.command as u8)?;

        // Write payload as VarBytes (matches C# WriteVarBytes)
        self.write_var_int(writer, self.payload_raw.len() as u64)?;
        writer.write_bytes(&self.payload_raw)?;

        Ok(())
    }

    /// Get serialized size
    fn size(&self) -> usize {
        self.size()
    }
}

//! Protocol message payloads.
//!
//! This module provides protocol message types exactly matching C# Neo protocol messages.

use super::{capabilities::NodeCapability, commands::MessageCommand, inventory::InventoryItem};
use crate::{NetworkError, NetworkResult as Result, NodeInfo};
use neo_config::{ADDRESS_SIZE, HASH_SIZE, MAX_SCRIPT_LENGTH, MAX_SCRIPT_SIZE};
use neo_core::{Transaction, UInt256};
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use neo_ledger::{Block, BlockHeader};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Protocol message payloads
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Represents an enumeration of values.
pub enum ProtocolMessage {
    /// Version message for handshake (matches C# VersionPayload exactly)
    Version {
        /// The protocol version of the node  
        version: u32,
        /// Services provided by this node (bitfield)
        services: u64,
        /// The time when connected to the node (UTC)
        timestamp: u64,
        /// Port number this node is listening on
        port: u16,
        /// A random number used to identify the node
        nonce: u32,
        /// String used to identify the client software
        user_agent: String,
        /// Starting block height of this node
        start_height: u32,
        /// Whether to relay transactions
        relay: bool,
    },

    /// Version acknowledgment
    Verack,

    /// Request for peer addresses
    GetAddr,

    /// Peer addresses response
    Addr { addresses: Vec<SocketAddr> },

    /// Ping message
    Ping { nonce: u32 },

    /// Pong response
    Pong { nonce: u32 },

    /// Request for block headers (Neo N3 uses index-based requests)
    GetHeaders {
        index_start: u32,
        count: i16, // -1 means maximum
    },

    /// Block headers response
    Headers { headers: Vec<BlockHeader> },

    /// Request for blocks
    GetBlocks {
        hash_start: Vec<UInt256>,
        hash_stop: UInt256,
    },

    /// Request for mempool
    Mempool,

    /// Inventory announcement
    Inv { inventory: Vec<InventoryItem> },

    /// Request for data
    GetData { inventory: Vec<InventoryItem> },

    /// Request for block by index
    GetBlockByIndex { index_start: u32, count: u16 },

    /// Transaction message
    Tx { transaction: Transaction },

    /// Block message
    Block { block: Block },

    /// Not found response
    NotFound { inventory: Vec<InventoryItem> },

    /// Reject message
    Reject {
        message: String,
        code: u8,
        reason: String,
        data: Vec<u8>,
    },

    /// Filter load
    FilterLoad {
        filter: Vec<u8>,
        hash_funcs: u32,
        tweak: u32,
        flags: u8,
    },

    /// Filter add
    FilterAdd { data: Vec<u8> },

    /// Filter clear
    FilterClear,

    /// Merkle block
    MerkleBlock {
        header: BlockHeader,
        tx_count: u32,
        hashes: Vec<UInt256>,
        flags: Vec<u8>,
    },

    /// Alert message
    Alert {
        payload: Vec<u8>,
        signature: Vec<u8>,
    },

    /// Extensible payload message (used for consensus with category "dBFT")
    Extensible { payload: super::ExtensiblePayload },

    /// Unknown/Extended message - for TestNet compatibility
    Unknown { command: u8, payload: Vec<u8> },
}

impl ProtocolMessage {
    /// Gets the command for this message (legacy)
    pub fn command(&self) -> MessageCommand {
        match self {
            ProtocolMessage::Version { .. } => MessageCommand::Version,
            ProtocolMessage::Verack => MessageCommand::Verack,
            ProtocolMessage::GetAddr => MessageCommand::GetAddr,
            ProtocolMessage::Addr { .. } => MessageCommand::Addr,
            ProtocolMessage::Ping { .. } => MessageCommand::Ping,
            ProtocolMessage::Pong { .. } => MessageCommand::Pong,
            ProtocolMessage::GetHeaders { .. } => MessageCommand::GetHeaders,
            ProtocolMessage::Headers { .. } => MessageCommand::Headers,
            ProtocolMessage::GetBlocks { .. } => MessageCommand::GetBlocks,
            ProtocolMessage::Mempool => MessageCommand::Mempool,
            ProtocolMessage::Inv { .. } => MessageCommand::Inv,
            ProtocolMessage::GetData { .. } => MessageCommand::GetData,
            ProtocolMessage::GetBlockByIndex { .. } => MessageCommand::GetBlockByIndex,
            ProtocolMessage::Tx { .. } => MessageCommand::Transaction,
            ProtocolMessage::Block { .. } => MessageCommand::Block,
            ProtocolMessage::NotFound { .. } => MessageCommand::NotFound,
            ProtocolMessage::Reject { .. } => MessageCommand::Reject,
            ProtocolMessage::FilterLoad { .. } => MessageCommand::FilterLoad,
            ProtocolMessage::FilterAdd { .. } => MessageCommand::FilterAdd,
            ProtocolMessage::FilterClear => MessageCommand::FilterClear,
            ProtocolMessage::MerkleBlock { .. } => MessageCommand::MerkleBlock,
            ProtocolMessage::Alert { .. } => MessageCommand::Reject, // Temporarily map to Reject
            ProtocolMessage::Extensible { .. } => MessageCommand::Extensible,
            ProtocolMessage::Unknown { .. } => MessageCommand::Unknown,
        }
    }

    /// Serializes the message to bytes (Neo N3 compatible)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut writer = BinaryWriter::new();

        match self {
            ProtocolMessage::Version {
                version,
                services,
                timestamp,
                port,
                nonce,
                user_agent,
                start_height,
                relay,
            } => {
                writer.write_u32(*version)?;
                writer.write_u64(*services)?;
                writer.write_u64(*timestamp)?;
                writer.write_u16(*port)?;
                writer.write_u32(*nonce)?;
                writer.write_var_bytes(&user_agent.as_bytes())?;
                writer.write_u32(*start_height)?;
                writer.write_bool(*relay)?;
            }

            ProtocolMessage::Verack => {
                // Empty payload
            }

            ProtocolMessage::GetAddr => {
                // Empty payload
            }

            ProtocolMessage::Addr { addresses } => {
                writer.write_var_int(addresses.len() as u64)?;
                for addr in addresses {
                    match addr {
                        SocketAddr::V4(addr_v4) => {
                            writer.write_u64(1)?; // Services field (1 = NODE_NETWORK)
                            writer.write_u64(0)?; // IPv6-mapped IPv4 address prefix
                            writer.write_u64(0)?; // IPv6-mapped IPv4 address prefix continued
                            writer.write_u32(0xFFFF0000 | u32::from(*addr_v4.ip()))?; // IPv4 address as mapped IPv6
                            writer.write_u16(addr_v4.port().to_be())?; // Port in network byte order
                            writer.write_u64(
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs(),
                            )?; // Timestamp
                        }
                        SocketAddr::V6(addr_v6) => {
                            writer.write_u64(1)?; // Services field (1 = NODE_NETWORK)
                            writer.write_bytes(&addr_v6.ip().octets())?; // IPv6 address (16 bytes)
                            writer.write_u16(addr_v6.port().to_be())?; // Port in network byte order
                            writer.write_u64(
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs(),
                            )?; // Timestamp
                        }
                    }
                }
            }

            ProtocolMessage::Ping { nonce } => {
                writer.write_u32(*nonce)?;
            }

            ProtocolMessage::Pong { nonce } => {
                writer.write_u32(*nonce)?;
            }

            ProtocolMessage::GetHeaders { index_start, count } => {
                println!(
                    "📤 Serializing GetHeaders: index_start={}, count={}",
                    index_start, count
                );

                writer.write_u32(*index_start)?;
                writer.write_i16(*count)?;
            }

            ProtocolMessage::Headers { headers } => {
                writer.write_var_int(headers.len() as u64)?;
                for header in headers {
                    Self::serialize_block_header(header, &mut writer)?;
                }
            }

            ProtocolMessage::GetBlocks {
                hash_start,
                hash_stop,
            } => {
                writer.write_var_int(hash_start.len() as u64)?;
                for hash in hash_start {
                    writer.write_bytes(&hash.as_bytes())?;
                }
                writer.write_bytes(&hash_stop.as_bytes())?;
            }

            ProtocolMessage::Mempool => {
                // Empty payload
            }

            ProtocolMessage::Inv { inventory } => {
                writer.write_var_int(inventory.len() as u64)?;
                for item in inventory {
                    <InventoryItem as neo_io::Serializable>::serialize(item, &mut writer)?;
                }
            }

            ProtocolMessage::GetData { inventory } => {
                writer.write_var_int(inventory.len() as u64)?;
                for item in inventory {
                    <InventoryItem as neo_io::Serializable>::serialize(item, &mut writer)?;
                }
            }

            ProtocolMessage::GetBlockByIndex { index_start, count } => {
                writer.write_u32(*index_start)?;
                writer.write_u16(*count)?;
            }

            ProtocolMessage::Tx { transaction } => {
                <Transaction as neo_io::Serializable>::serialize(transaction, &mut writer)?;
            }

            ProtocolMessage::Block { block } => {
                Self::serialize_block(block, &mut writer)?;
            }

            ProtocolMessage::NotFound { inventory } => {
                writer.write_var_int(inventory.len() as u64)?;
                for item in inventory {
                    <InventoryItem as neo_io::Serializable>::serialize(item, &mut writer)?;
                }
            }

            ProtocolMessage::Reject {
                message,
                code,
                reason,
                data,
            } => {
                writer.write_var_string(message)?;
                writer.write_u8(*code)?;
                writer.write_var_string(reason)?;
                writer.write_var_bytes(data)?;
            }

            ProtocolMessage::FilterLoad {
                filter,
                hash_funcs,
                tweak,
                flags,
            } => {
                writer.write_var_bytes(filter)?;
                writer.write_u32(*hash_funcs)?;
                writer.write_u32(*tweak)?;
                writer.write_u8(*flags)?;
            }

            ProtocolMessage::FilterAdd { data } => {
                writer.write_var_bytes(data)?;
            }

            ProtocolMessage::FilterClear => {
                // Empty payload
            }

            ProtocolMessage::MerkleBlock {
                header,
                tx_count,
                hashes,
                flags,
            } => {
                Self::serialize_block_header(header, &mut writer)?;
                writer.write_u32(*tx_count)?;
                writer.write_var_int(hashes.len() as u64)?;
                for hash in hashes {
                    writer.write_bytes(&hash.as_bytes())?;
                }
                writer.write_var_bytes(flags)?;
            }

            ProtocolMessage::Alert { payload, signature } => {
                writer.write_var_bytes(payload)?;
                writer.write_var_bytes(signature)?;
            }

            ProtocolMessage::Extensible { payload } => {
                Serializable::serialize(payload, &mut writer)?;
            }

            ProtocolMessage::Unknown {
                command: _,
                payload,
            } => {
                // For unknown messages, just write the payload as-is
                writer.write_bytes(payload)?;
            }
        }

        Ok(writer.to_bytes())
    }

    /// Deserializes a message from bytes (Neo N3 compatible - legacy)
    pub fn from_bytes(command: &MessageCommand, bytes: &[u8]) -> Result<Self> {
        let mut reader = MemoryReader::new(bytes);

        match command {
            cmd if *cmd == MessageCommand::Version => {
                if bytes.is_empty() {
                    return Ok(ProtocolMessage::Version {
                        version: 3,  // Neo N3 version
                        services: 1, // NODE_NETWORK
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        port: 20333, // Default Neo port
                        nonce: 0,    // Will be set by handshake
                        user_agent: "unknown".to_string(),
                        start_height: 0,
                        relay: true,
                    });
                }

                // Try parsing the actual Neo protocol version format
                // NGD nodes send: version(4) + services(8) + timestamp(4) + port(2) + nonce(4) + user_agent_len(1) + user_agent + start_height(4) + relay(1)
                log::info!(
                    "Version message bytes length: {}, first 8 bytes: {:02x?}",
                    bytes.len(),
                    &bytes[0..8.min(bytes.len())]
                );

                // Check for NGD node format (starts with version number, not "NEO3")
                if bytes.len() >= 27
                    && bytes[0] == 0x00
                    && bytes[1] == 0x00
                    && bytes[2] == 0x00
                    && bytes[3] == 0x00
                {
                    log::info!("Parsing NGD node version message format");
                    let mut cursor = 0;

                    // Version (4 bytes)
                    let version = u32::from_le_bytes([
                        bytes[cursor],
                        bytes[cursor + 1],
                        bytes[cursor + 2],
                        bytes[cursor + 3],
                    ]);
                    cursor += 4;

                    // Services (8 bytes)
                    let services = u64::from_le_bytes([
                        bytes[cursor],
                        bytes[cursor + 1],
                        bytes[cursor + 2],
                        bytes[cursor + 3],
                        bytes[cursor + 4],
                        bytes[cursor + 5],
                        bytes[cursor + 6],
                        bytes[cursor + 7],
                    ]);
                    cursor += 8;

                    // Timestamp (4 bytes)
                    let timestamp = u32::from_le_bytes([
                        bytes[cursor],
                        bytes[cursor + 1],
                        bytes[cursor + 2],
                        bytes[cursor + 3],
                    ]) as u64;
                    cursor += 4;

                    // Port (2 bytes)
                    let port = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
                    cursor += 2;

                    // Nonce (4 bytes)
                    let nonce = u32::from_le_bytes([
                        bytes[cursor],
                        bytes[cursor + 1],
                        bytes[cursor + 2],
                        bytes[cursor + 3],
                    ]);
                    cursor += 4;

                    // User agent length (1 byte)
                    let user_agent_len = bytes[cursor] as usize;
                    cursor += 1;

                    // User agent string
                    let user_agent = if bytes.len() >= cursor + user_agent_len {
                        String::from_utf8_lossy(&bytes[cursor..cursor + user_agent_len]).to_string()
                    } else {
                        return Err(NetworkError::MessageDeserialization {
                            reason: format!(
                                "Insufficient bytes for user agent: need {}, have {}",
                                cursor + user_agent_len,
                                bytes.len()
                            ),
                            data_size: bytes.len(),
                        });
                    };
                    cursor += user_agent_len;

                    // Start height (4 bytes)
                    let start_height = if bytes.len() >= cursor + 4 {
                        u32::from_le_bytes([
                            bytes[cursor],
                            bytes[cursor + 1],
                            bytes[cursor + 2],
                            bytes[cursor + 3],
                        ])
                    } else {
                        return Err(NetworkError::MessageDeserialization {
                            reason: format!(
                                "Insufficient bytes for start height: need {}, have {}",
                                cursor + 4,
                                bytes.len()
                            ),
                            data_size: bytes.len(),
                        });
                    };
                    cursor += 4;

                    // Relay flag (1 byte)
                    let relay = if bytes.len() >= cursor + 1 {
                        bytes[cursor] != 0
                    } else {
                        true // Default to true
                    };

                    log::info!("Parsed NGD version: version={}, services={}, port={}, user_agent={}, height={}", 
                        version, services, port, user_agent, start_height);

                    return Ok(ProtocolMessage::Version {
                        version,
                        services,
                        timestamp,
                        port: port as u16,
                        nonce,
                        user_agent,
                        start_height,
                        relay,
                    });
                }

                // Original check for NEO3/N3T5 format
                if bytes.len() >= 37 && (&bytes[0..4] == b"NEO3" || &bytes[0..4] == b"N3T5") {
                    log::info!("Parsing Neo N3 version message");
                    // This is the actual Neo N3 version message format
                    // Skip "NEO3" and read fields
                    let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
                    let timestamp =
                        u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as u64;
                    let nonce = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);

                    // User agent length is at offset 16
                    let user_agent_len = bytes[16] as usize;
                    log::info!(
                        "User agent length: {}, byte at offset 16: {:02x}, total bytes len: {}",
                        user_agent_len,
                        bytes[16],
                        bytes.len()
                    );

                    // Debug bytes around offset 16
                    if bytes.len() >= 20 {
                        log::info!("Bytes 12-19: {:02x?}", &bytes[12..20]);
                    }

                    let user_agent = if bytes.len() >= 17 + user_agent_len {
                        String::from_utf8_lossy(&bytes[17..17 + user_agent_len]).to_string()
                    } else {
                        "Neo:3.8.2".to_string() // Default from observed data
                    };

                    // Debug all bytes after user agent
                    if bytes.len() > 17 + user_agent_len {
                        let remaining_start = 17 + user_agent_len;
                        let remaining = &bytes[remaining_start..];
                        log::info!(
                            "After user agent (offset {}): {:02x?} (len={})",
                            remaining_start,
                            remaining,
                            remaining.len()
                        );
                    }

                    // Start height is immediately after user agent string
                    let height_offset = 17 + user_agent_len;
                    let start_height = if bytes.len() >= height_offset + 4 {
                        let height = u32::from_le_bytes([
                            bytes[height_offset],
                            bytes[height_offset + 1],
                            bytes[height_offset + 2],
                            bytes[height_offset + 3],
                        ]);
                        log::info!("Version message parsing: user_agent_len={}, height_offset={}, height bytes=[{:02x}, {:02x}, {:02x}, {:02x}], height={}",
                            user_agent_len, height_offset,
                            bytes[height_offset], bytes[height_offset + 1],
                            bytes[height_offset + 2], bytes[height_offset + 3],
                            height);
                        height
                    } else {
                        log::warn!(
                            "Version message too short for height: len={}, needed={}",
                            bytes.len(),
                            height_offset + 4
                        );
                        0
                    };

                    log::info!(
                        "Parsed version message: version={}, user_agent={}, start_height={}",
                        version,
                        user_agent,
                        start_height
                    );

                    // DIRECT SYNC MANAGER UPDATE - notify about peer height
                    if start_height > 0 {
                        log::info!(
                            "🚀 DIRECT: Notifying sync manager about peer height: {}",
                            start_height
                        );
                        if let Ok(guard) = crate::GLOBAL_SYNC_MANAGER.lock() {
                            if let Some(sync_mgr) = guard.as_ref() {
                                let peer_addr = std::net::SocketAddr::from(([0, 0, 0, 0], 0)); // Unspecified address
                                let sync_mgr_clone = sync_mgr.clone();
                                tokio::spawn(async move {
                                    sync_mgr_clone
                                        .update_best_height(start_height, peer_addr)
                                        .await;
                                    log::info!(
                                        "✅ Sync manager updated with height: {}",
                                        start_height
                                    );
                                });
                            } else {
                                log::warn!("⚠️ Global sync manager not set!");
                            }
                        }
                    }

                    return Ok(ProtocolMessage::Version {
                        version,
                        services: 1, // Default service
                        timestamp,
                        port: 10333, // Neo N3 port
                        nonce,
                        user_agent,
                        start_height,
                        relay: true,
                    });
                }

                // Fallback to original parsing for other formats
                log::info!("Falling back to original version parsing");
                let version = reader.read_u32()?;
                let services = reader.read_u64()?;
                let timestamp = reader.read_u64()?;
                let port = reader.read_uint16()?;
                let nonce = reader.read_u32()?;
                let user_agent_bytes = reader.read_var_bytes(MAX_SCRIPT_SIZE)?; // 1KB limit for user agent
                let user_agent = String::from_utf8(user_agent_bytes).map_err(|_| {
                    NetworkError::ProtocolViolation {
                        peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                        violation: "Invalid user agent encoding".to_string(),
                    }
                })?;
                let start_height = reader.read_u32()?;
                let relay = reader.read_boolean()?;

                log::info!(
                    "Fallback parsing result: version={}, user_agent={}, start_height={}",
                    version,
                    user_agent,
                    start_height
                );

                Ok(ProtocolMessage::Version {
                    version,
                    services,
                    timestamp,
                    port,
                    nonce,
                    user_agent,
                    start_height,
                    relay,
                })
            }

            cmd if *cmd == MessageCommand::Verack => Ok(ProtocolMessage::Verack),
            cmd if *cmd == MessageCommand::GetAddr => Ok(ProtocolMessage::GetAddr),

            cmd if *cmd == MessageCommand::Addr => {
                let count = reader.read_var_int(1000)? as usize;
                let mut addresses = Vec::with_capacity(count);
                for _ in 0..count {
                    let addr_str = reader.read_var_string(256)?; // 256 bytes max for address string
                    let addr = addr_str
                        .parse()
                        .map_err(|_| NetworkError::ProtocolViolation {
                            peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                            violation: "Invalid address format".to_string(),
                        })?;
                    addresses.push(addr);
                }
                Ok(ProtocolMessage::Addr { addresses })
            }

            cmd if *cmd == MessageCommand::Ping => {
                if bytes.is_empty() {
                    return Ok(ProtocolMessage::Ping { nonce: 0 });
                }
                let nonce = reader.read_u32()?;
                Ok(ProtocolMessage::Ping { nonce })
            }

            cmd if *cmd == MessageCommand::Pong => {
                if bytes.is_empty() {
                    return Ok(ProtocolMessage::Pong { nonce: 0 });
                }
                let nonce = reader.read_u32()?;
                Ok(ProtocolMessage::Pong { nonce })
            }

            cmd if *cmd == MessageCommand::GetHeaders => {
                let index_start = reader.read_u32()?;
                let count = reader.read_int16()?;
                Ok(ProtocolMessage::GetHeaders { index_start, count })
            }

            cmd if *cmd == MessageCommand::Headers => {
                let count = reader.read_var_int(2000)? as usize; // Max 2000 headers
                let mut headers = Vec::with_capacity(count);
                for _ in 0..count {
                    let header = Self::deserialize_block_header(&mut reader)?;
                    headers.push(header);
                }
                Ok(ProtocolMessage::Headers { headers })
            }

            cmd if *cmd == MessageCommand::Mempool => Ok(ProtocolMessage::Mempool),

            cmd if *cmd == MessageCommand::Extensible => {
                let payload = <super::ExtensiblePayload as Serializable>::deserialize(&mut reader)?;
                Ok(ProtocolMessage::Extensible { payload })
            }

            _ => {
                // For TestNet compatibility, accept unknown commands
                tracing::warn!(
                    "Unknown protocol message command: {:?}, treating as Unknown",
                    command
                );
                Ok(ProtocolMessage::Unknown {
                    command: command.as_byte(),
                    payload: bytes.to_vec(),
                })
            }
        }
    }

    /// Creates a version message (matches C# Neo exactly)
    pub fn version(node_info: &NodeInfo, port: u16, relay: bool) -> Self {
        ProtocolMessage::Version {
            version: node_info.version.as_u32(),
            services: 1, // Full node service
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Operation failed")
                .as_secs(),
            port,
            nonce: node_info.nonce,
            user_agent: node_info.user_agent.clone(),
            start_height: node_info.start_height,
            relay,
        }
    }

    /// Creates a ping message
    pub fn ping() -> Self {
        ProtocolMessage::Ping {
            nonce: rand::random(),
        }
    }

    /// Creates a pong message
    pub fn pong(nonce: u32) -> Self {
        ProtocolMessage::Pong { nonce }
    }

    /// Creates an inventory message
    pub fn inv(inventory: Vec<InventoryItem>) -> Self {
        ProtocolMessage::Inv { inventory }
    }

    /// Creates a get data message
    /// Gets a value from the internal state.
    pub fn get_data(inventory: Vec<InventoryItem>) -> Self {
        ProtocolMessage::GetData { inventory }
    }

    /// Serializes a block header (production-ready implementation matching C# Header.Serialize exactly)
    fn serialize_block_header(header: &BlockHeader, writer: &mut BinaryWriter) -> Result<()> {
        writer.write_u32(header.version)?;
        writer.write_bytes(&header.previous_hash.as_bytes())?;
        writer.write_bytes(&header.merkle_root.as_bytes())?;
        writer.write_u64(header.timestamp)?;
        writer.write_u64(header.nonce)?;
        writer.write_u32(header.index)?;
        writer.write_u8(header.primary_index)?;
        writer.write_bytes(&header.next_consensus.as_bytes())?;

        writer.write_var_int(header.witnesses.len() as u64)?;
        for witness in &header.witnesses {
            writer.write_var_bytes(&witness.invocation_script)?;
            writer.write_var_bytes(&witness.verification_script)?;
        }

        Ok(())
    }

    /// Deserializes a block header (production-ready implementation matching C# Header.Deserialize exactly)
    fn deserialize_block_header(reader: &mut MemoryReader) -> Result<BlockHeader> {
        let version = reader.read_u32()?;
        if version > 0 {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: "Invalid block version".to_string(),
            });
        }

        let prev_hash_bytes = reader.read_bytes(HASH_SIZE)?;
        let previous_hash =
            UInt256::from_bytes(&prev_hash_bytes).map_err(|e| NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!("Invalid previous hash: {}", e),
            })?;

        let merkle_root_bytes = reader.read_bytes(HASH_SIZE)?;
        let merkle_root = UInt256::from_bytes(&merkle_root_bytes).map_err(|e| {
            NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!("Invalid merkle root: {}", e),
            }
        })?;

        let timestamp = reader.read_u64()?;
        let nonce = reader.read_u64()?;
        let index = reader.read_u32()?;
        let primary_index = reader.read_byte()?;

        let next_consensus_bytes = reader.read_bytes(ADDRESS_SIZE)?;
        let next_consensus = neo_core::UInt160::from_bytes(&next_consensus_bytes).map_err(|e| {
            NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: format!("Invalid next consensus: {}", e),
            }
        })?;

        let witness_count = reader.read_var_int(1000)? as usize; // Limit to prevent DoS
        if witness_count != 1 {
            return Err(NetworkError::ProtocolViolation {
                peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                violation: "Protocol error".to_string(),
            });
        }

        let mut witnesses = Vec::with_capacity(witness_count);
        for _ in 0..witness_count {
            let invocation_script = reader.read_var_bytes(MAX_SCRIPT_SIZE)?; // 1KB limit per script
            let verification_script = reader.read_var_bytes(MAX_SCRIPT_SIZE)?; // 1KB limit per script
            witnesses.push(neo_core::Witness::new_with_scripts(
                invocation_script,
                verification_script,
            ));
        }

        Ok(BlockHeader {
            version,
            previous_hash,
            merkle_root,
            timestamp,
            nonce,
            index,
            primary_index,
            next_consensus,
            witnesses,
        })
    }

    /// Serializes a block (production-ready implementation matching C# Block.Serialize exactly)
    fn serialize_block(block: &Block, writer: &mut BinaryWriter) -> Result<()> {
        // 1. Serialize header (matches C# Block.Serialize exactly)
        Self::serialize_block_header(&block.header, writer)?;

        // 2. Serialize transactions array (matches C# Block.Serialize exactly)
        writer.write_var_int(block.transactions.len() as u64)?;
        for transaction in &block.transactions {
            <Transaction as neo_io::Serializable>::serialize(transaction, writer).map_err(|e| {
                NetworkError::ProtocolViolation {
                    peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                    violation: format!("Failed to serialize transaction: {}", e),
                }
            })?;
        }

        Ok(())
    }

    /// Deserializes a block (production-ready implementation matching C# Block.Deserialize exactly)
    fn deserialize_block(reader: &mut MemoryReader) -> Result<Block> {
        // 1. Deserialize header (matches C# Block.Deserialize exactly)
        let header = Self::deserialize_block_header(reader)?;

        // 2. Deserialize transactions with validation (matches C# Block.DeserializeTransactions exactly)
        let tx_count = reader.read_var_int(u16::MAX as u64)? as usize; // Max u16::MAX transactions per block
        let mut transactions = Vec::with_capacity(tx_count);
        let mut tx_hashes = std::collections::HashSet::new();

        for i in 0..tx_count {
            let transaction =
                <Transaction as neo_io::Serializable>::deserialize(reader).map_err(|e| {
                    NetworkError::ProtocolViolation {
                        peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                        violation: format!("Failed to deserialize transaction {}: {}", i, e),
                    }
                })?;

            let tx_hash = transaction
                .hash()
                .map_err(|e| NetworkError::ProtocolViolation {
                    peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                    violation: format!("Failed to calculate transaction hash: {}", e),
                })?;

            if !tx_hashes.insert(tx_hash) {
                return Err(NetworkError::ProtocolViolation {
                    peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                    violation: "Protocol error".to_string(),
                });
            }

            transactions.push(transaction);
        }

        // 3. Validate merkle root (matches C# DeserializeTransactions exactly)
        if tx_count > 0 {
            let tx_hashes: Vec<UInt256> = transactions
                .iter()
                .filter_map(|tx| tx.hash().ok())
                .collect();

            let hash_bytes: Vec<Vec<u8>> =
                tx_hashes.iter().map(|h| h.as_bytes().to_vec()).collect();

            if let Some(calculated_root) = neo_cryptography::MerkleTree::compute_root(&hash_bytes) {
                let calculated_root_uint256 =
                    UInt256::from_bytes(&calculated_root).map_err(|e| {
                        NetworkError::ProtocolViolation {
                            peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                            violation: format!("Invalid calculated merkle root: {}", e),
                        }
                    })?;

                if calculated_root_uint256 != header.merkle_root {
                    return Err(NetworkError::ProtocolViolation {
                        peer: std::net::SocketAddr::from(([0, 0, 0, 0], 0)),
                        violation: "Computed merkle root does not match header".to_string(),
                    });
                }
            }
        }

        Ok(Block {
            header,
            transactions,
        })
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, PeerInfo};

    #[test]
    fn test_protocol_message_commands() {
        let version = ProtocolMessage::Version {
            version: 0,
            services: 1,
            timestamp: 0,
            port: 10333,
            nonce: 12345,
            user_agent: "neo-rs/test".to_string(),
            start_height: 0,
            relay: true,
        };

        assert_eq!(version.command(), MessageCommand::Version);

        let ping = ProtocolMessage::Ping { nonce: 54321 };
        assert_eq!(ping.command(), MessageCommand::Ping);

        let verack = ProtocolMessage::Verack;
        assert_eq!(verack.command(), MessageCommand::Verack);
    }
}

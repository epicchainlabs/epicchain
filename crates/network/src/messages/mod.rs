//! Network message types and protocol handling.
//!
//! This module defines all network message types used in the Neo P2P protocol,
//! following C# Neo's message structure exactly:
//! - commands: Message command definitions (matches C# MessageCommand)
//! - header: Message header structure (matches C# MessageHeader)
//! - inventory: Inventory item types (matches C# InventoryItem)
//! - protocol: Protocol message payloads (matches C# protocol messages)
//! - network: Complete network message wrapper (matches C# NetworkMessage)

pub mod capabilities;
pub mod commands;
pub mod extensible_payload;
pub mod header;
pub mod inventory;
pub mod message;
pub mod network;
pub mod protocol;
pub mod validation;
pub mod version_payload;

// Compatibility message wrappers expected by some tests (C# parity)
pub mod compat {
    use super::{inventory::InventoryType, protocol::ProtocolMessage};
    use neo_core::{Transaction, UInt256};
    use neo_io::{BinaryWriter, MemoryReader, Serializable};
    use neo_ledger::{Block, BlockHeader};
    use std::net::SocketAddr;

    pub type Result<T> = std::result::Result<T, neo_io::Error>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NetworkAddress {
        pub timestamp: u64,
        pub services: u64,
        pub address: SocketAddr,
        pub port: u16,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AddressMessage {
        pub addresses: Vec<NetworkAddress>,
    }

    impl AddressMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            let mut w = BinaryWriter::new();
            w.write_var_int(self.addresses.len() as u64)?;
            for a in &self.addresses {
                w.write_u64(a.timestamp)?;
                w.write_u64(a.services)?;
                // Encode address as string for simplicity
                w.write_var_string(&a.address.to_string())?;
                w.write_u16(a.port)?;
            }
            Ok(w.to_bytes())
        }

        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            let mut r = MemoryReader::new(bytes);
            let count = r.read_var_int(10_000)? as usize;
            let mut out = Vec::with_capacity(count);
            for _ in 0..count {
                let timestamp = r.read_u64()?;
                let services = r.read_u64()?;
                let addr_str = r.read_var_string(256)?;
                let port = r.read_uint16()?;
                let address = format!("{}:{}", addr_str, port)
                    .parse()
                    .map_err(|_| neo_io::Error::InvalidData("Invalid socket address".into()))?;
                out.push(NetworkAddress {
                    timestamp,
                    services,
                    address,
                    port,
                });
            }
            Ok(Self { addresses: out })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct VerackMessage;
    impl VerackMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            Ok(Vec::new())
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            if bytes.is_empty() {
                Ok(Self)
            } else {
                Err(neo_io::Error::InvalidData("Non-empty verack".into()))
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FilterAddMessage {
        pub data: Vec<u8>,
    }
    impl FilterAddMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            let mut w = BinaryWriter::new();
            w.write_var_bytes(&self.data)?;
            Ok(w.to_bytes())
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            let mut r = MemoryReader::new(bytes);
            let data = r.read_var_bytes(1024 * 1024)?;
            Ok(Self { data })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FilterClearMessage;
    impl FilterClearMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            Ok(Vec::new())
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            if bytes.is_empty() {
                Ok(Self)
            } else {
                Err(neo_io::Error::InvalidData("Non-empty filterclear".into()))
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FilterLoadMessage {
        pub filter: Vec<u8>,
        pub hash_functions: u32,
        pub tweak: u32,
        pub flags: u8,
    }
    impl FilterLoadMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            let mut w = BinaryWriter::new();
            w.write_var_bytes(&self.filter)?;
            w.write_u32(self.hash_functions)?;
            w.write_u32(self.tweak)?;
            w.write_u8(self.flags)?;
            Ok(w.to_bytes())
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            let mut r = MemoryReader::new(bytes);
            let filter = r.read_var_bytes(1024 * 1024)?;
            let hash_functions = r.read_u32()?;
            let tweak = r.read_u32()?;
            let flags = r.read_byte()?;
            Ok(Self {
                filter,
                hash_functions,
                tweak,
                flags,
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MerkleBlockMessage {
        pub header: BlockHeader,
        pub tx_count: u32,
        pub hashes: Vec<UInt256>,
        pub flags: Vec<u8>,
    }
    impl MerkleBlockMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            // Delegate to ProtocolMessage for correctness
            ProtocolMessage::MerkleBlock {
                header: self.header.clone(),
                tx_count: self.tx_count,
                hashes: self.hashes.clone(),
                flags: self.flags.clone(),
            }
            .to_bytes()
            .map_err(|e| neo_io::Error::InvalidData(e.to_string()))
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            match ProtocolMessage::from_bytes(&super::commands::MessageCommand::MerkleBlock, bytes)
                .map_err(|e| neo_io::Error::InvalidData(e.to_string()))?
            {
                ProtocolMessage::MerkleBlock {
                    header,
                    tx_count,
                    hashes,
                    flags,
                } => Ok(Self {
                    header,
                    tx_count,
                    hashes,
                    flags,
                }),
                _ => Err(neo_io::Error::InvalidData("Wrong message".into())),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NotFoundMessage {
        pub type_: InventoryType,
        pub hashes: Vec<UInt256>,
    }
    impl NotFoundMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            // Reuse ProtocolMessage::NotFound with inventory list
            let inventory: Vec<super::inventory::InventoryItem> = self
                .hashes
                .iter()
                .map(|h| match self.type_ {
                    InventoryType::Transaction => super::inventory::InventoryItem::transaction(*h),
                    InventoryType::Block => super::inventory::InventoryItem::block(*h),
                    InventoryType::Consensus => super::inventory::InventoryItem::consensus(*h),
                })
                .collect();
            ProtocolMessage::NotFound { inventory }
                .to_bytes()
                .map_err(|e| neo_io::Error::InvalidData(e.to_string()))
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            match ProtocolMessage::from_bytes(&super::commands::MessageCommand::NotFound, bytes)
                .map_err(|e| neo_io::Error::InvalidData(e.to_string()))?
            {
                ProtocolMessage::NotFound { inventory } => {
                    let mut type_opt: Option<InventoryType> = None;
                    let mut hashes = Vec::with_capacity(inventory.len());
                    for item in inventory {
                        type_opt.get_or_insert(item.item_type);
                        hashes.push(item.hash);
                    }
                    Ok(Self {
                        type_: type_opt.unwrap_or(InventoryType::Block),
                        hashes,
                    })
                }
                _ => Err(neo_io::Error::InvalidData("Wrong message".into())),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TransactionMessage {
        pub transaction: Transaction,
    }
    impl TransactionMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            ProtocolMessage::Tx {
                transaction: self.transaction.clone(),
            }
            .to_bytes()
            .map_err(|e| neo_io::Error::InvalidData(e.to_string()))
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            match ProtocolMessage::from_bytes(&super::commands::MessageCommand::Transaction, bytes)
                .map_err(|e| neo_io::Error::InvalidData(e.to_string()))?
            {
                ProtocolMessage::Tx { transaction } => Ok(Self { transaction }),
                _ => Err(neo_io::Error::InvalidData("Wrong message".into())),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BlockMessage {
        pub block: Block,
    }
    impl BlockMessage {
        pub fn serialize(&self) -> Result<Vec<u8>> {
            ProtocolMessage::Block {
                block: self.block.clone(),
            }
            .to_bytes()
            .map_err(|e| neo_io::Error::InvalidData(e.to_string()))
        }
        pub fn deserialize(bytes: &[u8]) -> Result<Self> {
            match ProtocolMessage::from_bytes(&super::commands::MessageCommand::Block, bytes)
                .map_err(|e| neo_io::Error::InvalidData(e.to_string()))?
            {
                ProtocolMessage::Block { block } => Ok(Self { block }),
                _ => Err(neo_io::Error::InvalidData("Wrong message".into())),
            }
        }
    }
}

pub use capabilities::NodeCapability;
pub use commands::{varlen, MessageCommand, MessageFlags};
pub use extensible_payload::ExtensiblePayload;
pub use header::{Neo3Message, MAX_MESSAGE_SIZE};
pub use inventory::{InventoryItem, InventoryType};
pub use message::{Message, PAYLOAD_MAX_SIZE};
pub use network::NetworkMessage;
pub use protocol::ProtocolMessage;
pub use validation::MessageValidator;
pub use version_payload::{VersionPayload, MAX_CAPABILITIES};

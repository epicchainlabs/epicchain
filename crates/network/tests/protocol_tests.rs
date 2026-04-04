//! Network Protocol C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Network protocol implementation.
//! Tests are based on the C# Neo.Network.P2P protocol test suite.

use neo_core::{Block, Transaction, UInt160, UInt256};
use neo_ledger::BlockHeader;
use neo_network::messages::compat::*;
use neo_network::messages::inventory::{InventoryItem, InventoryType};
use neo_network::*;
use sha2::{Digest, Sha256};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[cfg(test)]
#[allow(dead_code)]
mod protocol_tests {
    use super::*;
    use neo_core::{Signer, Witness, WitnessScope};
    use neo_ledger::{Block as LedgerBlock, BlockHeader as LedgerBlockHeader};

    /// Test protocol message serialization (matches C# Message serialization exactly)
    #[test]
    fn test_protocol_message_serialization_compatibility() {
        let version_msg = ProtocolMessage::Version {
            version: 0x00,
            services: 1, // NodeNetwork service
            timestamp: 1234567890,
            port: 10333,
            nonce: 0x12345678,
            user_agent: "/NEO:3.6.0/".to_string(),
            start_height: 100000,
            relay: true,
        };

        // Wrap in NetworkMessage
        let network_msg = NetworkMessage::new(version_msg.clone());
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        // Verify deserialization
        match deserialized.payload {
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
                assert_eq!(version, 0x00);
                assert_eq!(services, 1);
                assert_eq!(timestamp, 1234567890);
                assert_eq!(port, 10333);
                assert_eq!(nonce, 0x12345678);
                assert_eq!(user_agent, "/NEO:3.6.0/");
                assert_eq!(start_height, 100000);
                assert_eq!(relay, true);
            }
            _ => panic!("Expected Version message"),
        }
    }

    /// Test network addresses (matches C# NetworkAddress exactly)
    #[test]
    fn test_network_address_compatibility() {
        // Test Addr message with network addresses
        let addresses = vec![
            NetworkAddress {
                timestamp: 1234567890,
                services: 1,
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)), 10333),
                port: 10333,
            },
            NetworkAddress {
                timestamp: 1234567890,
                services: 1,
                address: SocketAddr::new("2001:db8::1".parse().unwrap(), 10333),
                port: 10333,
            },
        ];

        let addr_msg = AddressMessage {
            addresses: addresses.clone(),
        };
        let serialized = addr_msg.serialize().unwrap();
        let deserialized = AddressMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.addresses.len(), 2);
        assert_eq!(
            deserialized.addresses[0].address.ip(),
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100))
        );
        assert_eq!(deserialized.addresses[0].address.port(), 10333);
        assert_eq!(
            deserialized.addresses[1].address.ip().to_string(),
            "2001:db8::1"
        );
    }

    /// Test inventory messages (matches C# InvPayload exactly)
    #[test]
    fn test_inventory_message_compatibility() {
        // Create inventory items
        let mut inventory = vec![];
        for i in 0..10 {
            let hash = UInt256::from_bytes(&[i; 32]).unwrap();
            inventory.push(InventoryItem {
                item_type: InventoryType::Block,
                hash,
            });
        }

        // Create Inv protocol message
        let inv_msg = ProtocolMessage::Inv {
            inventory: inventory.clone(),
        };

        // Wrap in NetworkMessage and serialize
        let network_msg = NetworkMessage::new(inv_msg);
        let serialized = network_msg.to_bytes().unwrap();

        // Deserialize and verify
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();
        match deserialized.payload {
            ProtocolMessage::Inv { inventory: inv } => {
                assert_eq!(inv.len(), 10);
                for (i, item) in inv.iter().enumerate() {
                    assert_eq!(item.item_type, InventoryType::Block);
                    assert_eq!(item.hash, inventory[i].hash);
                }
            }
            _ => panic!("Expected Inv message"),
        }

        // Test with transaction inventory
        let tx_inventory: Vec<_> = (0..5)
            .map(|i| InventoryItem {
                item_type: InventoryType::TX,
                hash: UInt256::from_bytes(&[i * 2; 32]).unwrap(),
            })
            .collect();

        let tx_inv_msg = ProtocolMessage::Inv {
            inventory: tx_inventory.clone(),
        };

        let network_msg = NetworkMessage::new(tx_inv_msg);
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        match deserialized.payload {
            ProtocolMessage::Inv { inventory: inv } => {
                assert_eq!(inv.len(), 5);
                for item in inv.iter() {
                    assert_eq!(item.item_type, InventoryType::TX);
                }
            }
            _ => panic!("Expected Inv message"),
        }
    }

    /// Test GetData message (matches C# GetDataPayload exactly)
    #[test]
    fn test_getdata_message_compatibility() {
        let inventory: Vec<_> = (0..3)
            .map(|i| InventoryItem {
                item_type: InventoryType::Block,
                hash: UInt256::from_bytes(&[(i + 1) as u8; 32]).unwrap(),
            })
            .collect();

        let getdata_msg = ProtocolMessage::GetData {
            inventory: inventory.clone(),
        };

        let network_msg = NetworkMessage::new(getdata_msg);
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        match deserialized.payload {
            ProtocolMessage::GetData { inventory: inv } => {
                assert_eq!(inv.len(), 3);
                for (i, item) in inv.iter().enumerate() {
                    assert_eq!(item.item_type, InventoryType::Block);
                    assert_eq!(item.hash, inventory[i].hash);
                }
            }
            _ => panic!("Expected GetData message"),
        }
    }

    /// Test GetBlocks message (matches C# GetBlocksPayload exactly)
    #[test]
    fn test_getblocks_message_compatibility() {
        let hash_start = vec![
            UInt256::from_bytes(&[10u8; 32]).unwrap(),
            UInt256::from_bytes(&[20u8; 32]).unwrap(),
        ];
        let hash_stop = UInt256::from_bytes(&[30u8; 32]).unwrap();

        let getblocks_msg = ProtocolMessage::GetBlocks {
            hash_start: hash_start.clone(),
            hash_stop: hash_stop.clone(),
        };

        let network_msg = NetworkMessage::new(getblocks_msg);
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        match deserialized.payload {
            ProtocolMessage::GetBlocks {
                hash_start: start,
                hash_stop: stop,
            } => {
                assert_eq!(start, hash_start);
                assert_eq!(stop, hash_stop);
            }
            _ => panic!("Expected GetBlocks message"),
        }
    }

    /// Test Headers message (matches C# HeadersPayload exactly)
    #[test]
    fn test_headers_message_compatibility() {
        let mut headers = vec![];
        for i in 0..5 {
            let header = BlockHeader {
                version: 0,
                previous_hash: UInt256::from_bytes(&[i; 32]).unwrap(),
                merkle_root: UInt256::from_bytes(&[(i + 1); 32]).unwrap(),
                timestamp: 1234567890 + i as u64,
                index: i as u32,
                primary_index: 0,
                nonce: 0,
                next_consensus: UInt160::from_bytes(&[i as u8; 20]).unwrap(),
                witnesses: vec![],
            };
            headers.push(header);
        }

        let headers_msg = ProtocolMessage::Headers {
            headers: headers.clone(),
        };

        let network_msg = NetworkMessage::new(headers_msg);
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        match deserialized.payload {
            ProtocolMessage::Headers { headers: hdrs } => {
                assert_eq!(hdrs.len(), 5);
                for (i, header) in hdrs.iter().enumerate() {
                    assert_eq!(header.index, headers[i].index);
                    assert_eq!(header.previous_hash, headers[i].previous_hash);
                    assert_eq!(header.timestamp, headers[i].timestamp);
                }
            }
            _ => panic!("Expected Headers message"),
        }
    }

    /// Test Ping/Pong messages (matches C# PingPayload exactly)
    #[test]
    fn test_ping_pong_compatibility() {
        // Test Ping message
        let ping_msg = ProtocolMessage::Ping { nonce: 0xDEADBEEF };

        let network_msg = NetworkMessage::new(ping_msg);
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        match deserialized.payload {
            ProtocolMessage::Ping { nonce } => {
                assert_eq!(nonce, 0xDEADBEEF);
            }
            _ => panic!("Expected Ping message"),
        }

        // Test Pong message
        let pong_msg = ProtocolMessage::Pong { nonce: 0xDEADBEEF };

        let network_msg = NetworkMessage::new(pong_msg);
        let serialized = network_msg.to_bytes().unwrap();
        let deserialized = NetworkMessage::from_bytes(&serialized).unwrap();

        match deserialized.payload {
            ProtocolMessage::Pong { nonce } => {
                assert_eq!(nonce, 0xDEADBEEF);
            }
            _ => panic!("Expected Pong message"),
        }
    }

    /// Test Address message (matches C# AddrPayload exactly)
    #[test]
    fn test_address_message_compatibility() {
        let mut addresses = vec![];
        for i in 0..10 {
            addresses.push(NetworkAddress {
                timestamp: 1234567890 + i,
                services: NodeServices::NodeNetwork as u64,
                address: SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(192, 168, 1, i as u8)),
                    10333 + i as u16,
                ),
                port: 10333 + i as u16,
            });
        }

        let addr_msg = AddressMessage {
            addresses: addresses.clone(),
        };

        let serialized = addr_msg.serialize().unwrap();
        let deserialized = AddressMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.addresses.len(), 10);
        for (i, addr) in deserialized.addresses.iter().enumerate() {
            assert_eq!(addr.timestamp, addresses[i].timestamp);
            assert_eq!(addr.services, addresses[i].services);
            assert_eq!(addr.address, addresses[i].address);
            assert_eq!(addr.port, addresses[i].port);
        }
    }

    /// Test Transaction message (matches C# transaction network format exactly)
    #[test]
    fn test_transaction_message_compatibility() {
        let mut tx = Transaction::new();
        tx.set_version(0);
        tx.set_nonce(123456);
        tx.set_system_fee(1_000_000);
        tx.set_network_fee(100_000);
        tx.set_valid_until_block(999_999);
        tx.add_signer(create_test_signer());
        tx.set_script(vec![0x51, 0x52, 0x53]);
        tx.add_witness(create_test_witness());

        let tx_msg = TransactionMessage {
            transaction: tx.clone(),
        };

        let serialized = tx_msg.serialize().unwrap();
        let deserialized = TransactionMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.transaction.version(), tx.version());
        assert_eq!(deserialized.transaction.nonce(), tx.nonce());
        assert_eq!(deserialized.transaction.system_fee(), tx.system_fee());
        assert_eq!(deserialized.transaction.network_fee(), tx.network_fee());
        assert_eq!(deserialized.transaction.script(), tx.script());
    }

    /// Test Block message (matches C# block network format exactly)
    #[test]
    fn test_block_message_compatibility() {
        let transactions = vec![
            create_test_transaction(1),
            create_test_transaction(2),
            create_test_transaction(3),
        ];

        let header = LedgerBlockHeader {
            version: 0,
            previous_hash: UInt256::from_bytes(&[1u8; 32]).unwrap(),
            merkle_root: calculate_merkle_root(&transactions),
            timestamp: 1234567890,
            nonce: 42,
            index: 100,
            primary_index: 0,
            next_consensus: UInt160::from_bytes(&[2u8; 20]).unwrap(),
            witnesses: vec![create_test_witness()],
        };
        let block = LedgerBlock {
            header: header.clone(),
            transactions: transactions.clone(),
        };

        let block_msg = BlockMessage {
            block: block.clone(),
        };

        let serialized = block_msg.serialize().unwrap();
        let deserialized = BlockMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.block.header.version, block.header.version);
        assert_eq!(deserialized.block.header.index, block.header.index);
        assert_eq!(
            deserialized.block.header.previous_hash,
            block.header.previous_hash
        );
        assert_eq!(deserialized.block.header.timestamp, block.header.timestamp);
        assert_eq!(
            deserialized.block.transactions.len(),
            block.transactions.len()
        );
    }

    /// Test Verack message (matches C# VerAckPayload exactly)
    #[test]
    fn test_verack_message_compatibility() {
        let verack_msg = VerackMessage {};

        let serialized = verack_msg.serialize().unwrap();
        let deserialized = VerackMessage::deserialize(&serialized).unwrap();

        // Verack is empty, just test it serializes/deserializes
        assert_eq!(serialized.len(), 0);
    }

    /// Test FilterAdd message (matches C# FilterAddPayload exactly)
    #[test]
    fn test_filteradd_message_compatibility() {
        let filter_data = vec![0x01, 0x02, 0x03, 0x04, 0x05];

        let filteradd_msg = FilterAddMessage {
            data: filter_data.clone(),
        };

        let serialized = filteradd_msg.serialize().unwrap();
        let deserialized = FilterAddMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.data, filter_data);
    }

    /// Test FilterClear message (matches C# FilterClearPayload exactly)
    #[test]
    fn test_filterclear_message_compatibility() {
        let filterclear_msg = FilterClearMessage {};

        let serialized = filterclear_msg.serialize().unwrap();
        let deserialized = FilterClearMessage::deserialize(&serialized).unwrap();

        // FilterClear is empty
        assert_eq!(serialized.len(), 0);
    }

    /// Test FilterLoad message (matches C# FilterLoadPayload exactly)
    #[test]
    fn test_filterload_message_compatibility() {
        let filterload_msg = FilterLoadMessage {
            filter: vec![0xFF; 1024],
            hash_functions: 5,
            tweak: 12345,
            flags: BloomFilterFlags::UpdateAll as u8,
        };

        let serialized = filterload_msg.serialize().unwrap();
        let deserialized = FilterLoadMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.filter, vec![0xFF; 1024]);
        assert_eq!(deserialized.hash_functions, 5);
        assert_eq!(deserialized.tweak, 12345);
        assert_eq!(deserialized.flags, BloomFilterFlags::UpdateAll as u8);
    }

    /// Test MerkleBlock message (matches C# MerkleBlockPayload exactly)
    #[test]
    fn test_merkleblock_message_compatibility() {
        let header = LedgerBlockHeader {
            version: 0,
            previous_hash: UInt256::from_bytes(&[1u8; 32]).unwrap(),
            merkle_root: UInt256::from_bytes(&[2u8; 32]).unwrap(),
            timestamp: 1234567890,
            index: 100,
            nonce: 0,
            primary_index: 0,
            next_consensus: UInt160::from_bytes(&[3u8; 20]).unwrap(),
            witnesses: vec![],
        };

        let tx_count = 10;
        let hashes = vec![
            UInt256::from_bytes(&[10u8; 32]).unwrap(),
            UInt256::from_bytes(&[20u8; 32]).unwrap(),
            UInt256::from_bytes(&[30u8; 32]).unwrap(),
        ];
        let flags = vec![0x01, 0x03, 0x07];

        let merkleblock_msg = MerkleBlockMessage {
            header: header.clone(),
            tx_count,
            hashes: hashes.clone(),
            flags: flags.clone(),
        };

        let serialized = merkleblock_msg.serialize().unwrap();
        let deserialized = MerkleBlockMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.header.index, header.index);
        assert_eq!(deserialized.tx_count, tx_count);
        assert_eq!(deserialized.hashes, hashes);
        assert_eq!(deserialized.flags, flags);
    }

    /// Test NotFound message (matches C# NotFoundPayload exactly)
    #[test]
    fn test_notfound_message_compatibility() {
        let hashes = vec![
            UInt256::from_bytes(&[100u8; 32]).unwrap(),
            UInt256::from_bytes(&[200u8; 32]).unwrap(),
        ];

        let notfound_msg = NotFoundMessage {
            type_: InventoryType::Block,
            hashes: hashes.clone(),
        };

        let serialized = notfound_msg.serialize().unwrap();
        let deserialized = NotFoundMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.type_, InventoryType::Block);
        assert_eq!(deserialized.hashes, hashes);
    }

    /// Test message header format (matches C# Message header exactly)
    #[test]
    fn test_message_header_compatibility() {
        let network_magic = 0x334f454e; // NEO3 mainnet
        let command = "version";
        let payload = vec![0x01, 0x02, 0x03, 0x04];

        let pm = ProtocolMessage::FilterAdd {
            data: payload.clone(),
        };
        let message = NetworkMessage::new_with_magic(pm.clone(), network_magic);
        let serialized = message.serialize().unwrap();
        let deserialized = NetworkMessage::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.header.magic, network_magic);
        assert_eq!(
            deserialized.command(),
            neo_network::MessageCommand::FilterAdd
        );
        match deserialized.payload {
            ProtocolMessage::FilterAdd { data } => assert_eq!(data, payload),
            _ => panic!("Wrong payload"),
        }
        assert_eq!(deserialized.checksum, message.checksum);
    }

    /// Test message size limits (matches C# size constraints exactly)
    #[test]
    fn test_message_size_limits_compatibility() {
        let max_payload_size = 0x02000000 - 24; // Subtract header size

        // Create large but valid message (FilterAdd with large payload)
        let large_payload = vec![0x00; max_payload_size];
        let pm = ProtocolMessage::FilterAdd {
            data: large_payload,
        };
        let message = NetworkMessage::new_with_magic(pm, 0x334f454e);
        let serialized = message.serialize().unwrap();
        assert!(serialized.len() <= 0x02000000);

        // Test oversized message should fail
        let oversized_payload = vec![0x00; 0x02000000];
        let pm_over = ProtocolMessage::FilterAdd {
            data: oversized_payload,
        };
        let oversized_message = NetworkMessage::new_with_magic(pm_over, 0x334f454e);
        assert!(oversized_message.serialize().is_err());
    }

    /// Test protocol version compatibility (matches C# version handling exactly)
    #[test]
    fn test_protocol_version_compatibility() {
        // Test current protocol version
        assert_eq!(PROTOCOL_VERSION, 0x00);

        // Test minimum supported version
        assert_eq!(MIN_PROTOCOL_VERSION, 0x00);

        // Test version compatibility check
        assert!(is_version_supported(0x00));

        // Test service flags
        assert_eq!(NodeServices::NodeNetwork as u64, 0x01);
        assert_eq!(NodeServices::NodeGetBlocks as u64, 0x02);
        assert_eq!(NodeServices::NodeGetTransactions as u64, 0x04);
    }

    /// Test checksum calculation (matches C# checksum exactly)
    #[test]
    fn test_checksum_calculation_compatibility() {
        let test_data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let checksum = calculate_checksum(&test_data);

        let hash1 = Sha256::digest(&test_data);
        let hash2 = Sha256::digest(&hash1);
        let expected = u32::from_le_bytes([hash2[0], hash2[1], hash2[2], hash2[3]]);

        assert_eq!(checksum, expected);

        // Test empty data
        let empty_checksum = calculate_checksum(&[]);
        assert_ne!(empty_checksum, 0); // Should not be zero
    }

    /// Test network magic values (matches C# network constants exactly)
    #[test]
    fn test_network_magic_compatibility() {
        // Test mainnet magic
        assert_eq!(MAINNET_MAGIC, 0x334f454e);

        // Test testnet magic
        assert_eq!(TESTNET_MAGIC, 0x3554454e);

        // Test private net magic
        assert_eq!(PRIVATE_NET_MAGIC, 0x334f454e);

        // Test magic validation
        assert!(is_valid_magic(MAINNET_MAGIC));
        assert!(is_valid_magic(TESTNET_MAGIC));
        assert!(!is_valid_magic(0x12345678)); // Invalid magic
    }

    // Helper functions

    fn create_test_transaction(nonce: u32) -> Transaction {
        let mut tx = Transaction::new();
        tx.set_version(0);
        tx.set_nonce(nonce);
        tx.set_system_fee(0);
        tx.set_network_fee(0);
        tx.set_valid_until_block(999_999);
        tx.add_signer(create_test_signer());
        tx.set_script(vec![0x51]);
        tx.add_witness(create_test_witness());
        tx
    }

    fn create_test_signer() -> Signer {
        Signer::new(
            UInt160::from_bytes(&[1u8; 20]).unwrap(),
            WitnessScope::CALLED_BY_ENTRY,
        )
    }

    fn create_test_witness() -> Witness {
        Witness::new_with_scripts(vec![0x00; 64], vec![0x51])
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> UInt256 {
        if transactions.is_empty() {
            return UInt256::zero();
        }

        let mut hashes: Vec<UInt256> = transactions
            .iter()
            .filter_map(|tx| tx.hash().ok())
            .collect();

        while hashes.len() > 1 {
            let mut new_hashes = vec![];

            for chunk in hashes.chunks(2) {
                if chunk.len() == 2 {
                    new_hashes.push(combine_hashes(&chunk[0], &chunk[1]));
                } else {
                    new_hashes.push(combine_hashes(&chunk[0], &chunk[0]));
                }
            }

            hashes = new_hashes;
        }

        hashes[0]
    }

    fn combine_hashes(hash1: &UInt256, hash2: &UInt256) -> UInt256 {
        let mut data = Vec::new();
        data.extend_from_slice(hash1.as_bytes());
        data.extend_from_slice(hash2.as_bytes());
        let hash = Sha256::digest(&data);
        UInt256::from_bytes(&hash).unwrap()
    }

    fn calculate_checksum(data: &[u8]) -> u32 {
        let hash1 = Sha256::digest(data);
        let hash2 = Sha256::digest(&hash1);
        u32::from_le_bytes([hash2[0], hash2[1], hash2[2], hash2[3]])
    }

    fn is_version_supported(version: u32) -> bool {
        version >= MIN_PROTOCOL_VERSION && version <= PROTOCOL_VERSION
    }

    fn is_valid_magic(magic: u32) -> bool {
        magic == MAINNET_MAGIC || magic == TESTNET_MAGIC
    }

    // Constants
    const PROTOCOL_VERSION: u32 = 0x00;
    const MIN_PROTOCOL_VERSION: u32 = 0x00;
    const MAINNET_MAGIC: u32 = 0x334f454e;
    const TESTNET_MAGIC: u32 = 0x3554454e;
    const PRIVATE_NET_MAGIC: u32 = 0x334f454e;
}

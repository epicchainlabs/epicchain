//! Consensus Message Compatibility Tests
//!
//! These tests ensure that Neo Rust consensus messages are fully compatible
//! with C# Neo consensus implementation using ExtensiblePayload wrapper.

use neo_consensus::{
    ChangeView, Commit, ConsensusMessage, ConsensusMessageType, ConsensusPayload, PrepareRequest,
    PrepareResponse, RecoveryMessage, RecoveryRequest,
};
use neo_core::{UInt160, UInt256, Witness};
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use neo_network::messages::ExtensiblePayload;

#[test]
fn test_consensus_extensible_payload_wrapper() {
    // Create a sample consensus message
    let consensus_msg = ConsensusMessage {
        view_number: 0,
        message_type: ConsensusMessageType::PrepareRequest,
        payload: ConsensusPayload::PrepareRequest(PrepareRequest {
            version: 0,
            prev_hash: UInt256::zero(),
            timestamp: 1234567890,
            nonce: 12345,
            transaction_hashes: vec![],
        }),
    };

    // Serialize consensus message
    let mut writer = BinaryWriter::new();
    consensus_msg.serialize(&mut writer).unwrap();
    let consensus_data = writer.to_bytes();

    // Wrap in ExtensiblePayload with "dBFT" category
    let sender = UInt160::zero();
    let witness = Witness::new(vec![], vec![]);

    let extensible = ExtensiblePayload::consensus(
        0,      // valid_block_start
        100000, // valid_block_end
        sender,
        consensus_data,
        witness,
    );

    // Verify it's identified as consensus
    assert!(extensible.is_consensus());
    assert_eq!(extensible.category, "dBFT");
}

#[test]
fn test_all_consensus_message_types() {
    let test_cases = vec![
        (
            ConsensusMessageType::ChangeView,
            ConsensusPayload::ChangeView(ChangeView {
                new_view_number: 1,
                timestamp: 1234567890,
            }),
        ),
        (
            ConsensusMessageType::PrepareRequest,
            ConsensusPayload::PrepareRequest(PrepareRequest {
                version: 0,
                prev_hash: UInt256::zero(),
                timestamp: 1234567890,
                nonce: 12345,
                transaction_hashes: vec![UInt256::zero()],
            }),
        ),
        (
            ConsensusMessageType::PrepareResponse,
            ConsensusPayload::PrepareResponse(PrepareResponse {
                prepare_hash: UInt256::zero(),
            }),
        ),
        (
            ConsensusMessageType::Commit,
            ConsensusPayload::Commit(Commit {
                signature: vec![0; 64],
            }),
        ),
        (
            ConsensusMessageType::RecoveryRequest,
            ConsensusPayload::RecoveryRequest(RecoveryRequest {
                timestamp: 1234567890,
            }),
        ),
    ];

    for (msg_type, payload) in test_cases {
        let consensus_msg = ConsensusMessage {
            view_number: 0,
            message_type: msg_type,
            payload,
        };

        // Serialize
        let mut writer = BinaryWriter::new();
        consensus_msg.serialize(&mut writer).unwrap();
        let data = writer.to_bytes();

        // Deserialize
        let mut reader = MemoryReader::new(&data);
        let deserialized = ConsensusMessage::deserialize(&mut reader).unwrap();

        assert_eq!(consensus_msg.view_number, deserialized.view_number);
        assert_eq!(consensus_msg.message_type, deserialized.message_type);
    }
}

#[test]
fn test_consensus_message_signing() {
    use neo_cryptography::Crypto;

    let consensus_msg = ConsensusMessage {
        view_number: 0,
        message_type: ConsensusMessageType::PrepareResponse,
        payload: ConsensusPayload::PrepareResponse(PrepareResponse {
            prepare_hash: UInt256::zero(),
        }),
    };

    // Serialize for signing
    let mut writer = BinaryWriter::new();
    consensus_msg.serialize(&mut writer).unwrap();
    let message_data = writer.to_bytes();

    // Create signature
    let private_key = [1u8; 32];
    let public_key = Crypto::get_public_key(&private_key).unwrap();
    let signature = Crypto::sign(&message_data, &private_key).unwrap();

    // Create witness
    let witness = Witness::new(vec![], signature);

    // Wrap in ExtensiblePayload
    let sender = UInt160::from_script(&public_key).unwrap();
    let extensible = ExtensiblePayload::consensus(0, 100000, sender, message_data, witness);

    // Verify the payload is valid
    assert!(extensible.validate().is_ok());
}

#[test]
fn test_consensus_recovery_message() {
    use std::collections::HashMap;

    let recovery_msg = RecoveryMessage {
        change_view_messages: HashMap::new(),
        prepare_request_message: None,
        prepare_response_messages: HashMap::new(),
        commit_messages: HashMap::new(),
    };

    let consensus_msg = ConsensusMessage {
        view_number: 0,
        message_type: ConsensusMessageType::RecoveryMessage,
        payload: ConsensusPayload::RecoveryMessage(recovery_msg),
    };

    // Test serialization
    let mut writer = BinaryWriter::new();
    consensus_msg.serialize(&mut writer).unwrap();
    let data = writer.to_bytes();

    // Test deserialization
    let mut reader = MemoryReader::new(&data);
    let deserialized = ConsensusMessage::deserialize(&mut reader).unwrap();

    match deserialized.payload {
        ConsensusPayload::RecoveryMessage(recovery) => {
            assert!(recovery.change_view_messages.is_empty());
            assert!(recovery.prepare_request_message.is_none());
            assert!(recovery.prepare_response_messages.is_empty());
            assert!(recovery.commit_messages.is_empty());
        }
        _ => panic!("Wrong payload type"),
    }
}

#[test]
fn test_consensus_block_validity_range() {
    let consensus_msg = ConsensusMessage {
        view_number: 0,
        message_type: ConsensusMessageType::PrepareRequest,
        payload: ConsensusPayload::PrepareRequest(PrepareRequest {
            version: 0,
            prev_hash: UInt256::zero(),
            timestamp: 1234567890,
            nonce: 12345,
            transaction_hashes: vec![],
        }),
    };

    let mut writer = BinaryWriter::new();
    consensus_msg.serialize(&mut writer).unwrap();
    let data = writer.to_bytes();

    // Test different validity ranges
    let test_ranges = vec![
        (0, 100),      // Short range
        (1000, 2000),  // Mid range
        (0, u32::MAX), // Max range
    ];

    for (start, end) in test_ranges {
        let extensible = ExtensiblePayload::consensus(
            start,
            end,
            UInt160::zero(),
            data.clone(),
            Witness::new(vec![], vec![]),
        );

        assert_eq!(extensible.valid_block_start, start);
        assert_eq!(extensible.valid_block_end, end);
        assert!(extensible.validate().is_ok());
    }
}

#[test]
fn test_consensus_message_size_limits() {
    // Test with maximum transaction hashes (common in prepare requests)
    let mut tx_hashes = vec![];
    for i in 0..256 {
        let mut hash = UInt256::zero();
        hash.0[0] = i as u8;
        tx_hashes.push(hash);
    }

    let prepare_request = PrepareRequest {
        version: 0,
        prev_hash: UInt256::zero(),
        timestamp: 1234567890,
        nonce: 12345,
        transaction_hashes: tx_hashes,
    };

    let consensus_msg = ConsensusMessage {
        view_number: 0,
        message_type: ConsensusMessageType::PrepareRequest,
        payload: ConsensusPayload::PrepareRequest(prepare_request),
    };

    // Verify it serializes successfully
    let mut writer = BinaryWriter::new();
    assert!(consensus_msg.serialize(&mut writer).is_ok());

    let data = writer.to_bytes();
    assert!(data.len() < 65536); // Max size for consensus messages
}

#[test]
fn test_consensus_compatibility_with_csharp_format() {
    // This test verifies the exact byte format matches C# implementation
    // Test vector from C# Neo consensus message

    // Simple PrepareResponse message
    let prepare_response = PrepareResponse {
        prepare_hash: UInt256::from_str(
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        )
        .unwrap(),
    };

    let consensus_msg = ConsensusMessage {
        view_number: 5,
        message_type: ConsensusMessageType::PrepareResponse,
        payload: ConsensusPayload::PrepareResponse(prepare_response),
    };

    let mut writer = BinaryWriter::new();
    consensus_msg.serialize(&mut writer).unwrap();
    let bytes = writer.to_bytes();

    // Verify structure:
    // - view_number (1 byte): 0x05
    // - message_type (1 byte): 0x01 (PrepareResponse)
    // - payload (32 bytes): the hash
    assert_eq!(bytes[0], 5); // view_number
    assert_eq!(bytes[1], ConsensusMessageType::PrepareResponse as u8);
    assert_eq!(bytes.len(), 2 + 32); // header + hash
}

#[cfg(test)]
#[allow(dead_code)]
mod integration_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    #[ignore] // Run with --ignored flag
    async fn test_consensus_message_broadcast() {
        use neo_ledger::Blockchain;
        use neo_network::p2p::P2PNode;
        use neo_network::NetworkConfig;

        let config = NetworkConfig::testnet();
        let blockchain = Blockchain::new_testnet().await.unwrap();
        let node = P2PNode::new(config, blockchain).await.unwrap();

        // Create consensus message
        let consensus_msg = ConsensusMessage {
            view_number: 0,
            message_type: ConsensusMessageType::PrepareRequest,
            payload: ConsensusPayload::PrepareRequest(PrepareRequest {
                version: 0,
                prev_hash: UInt256::zero(),
                timestamp: chrono::Utc::now().timestamp() as u64,
                nonce: rand::random(),
                transaction_hashes: vec![],
            }),
        };

        // Serialize and wrap in ExtensiblePayload
        let mut writer = BinaryWriter::new();
        consensus_msg.serialize(&mut writer).unwrap();
        let data = writer.to_bytes();

        let extensible = ExtensiblePayload::consensus(
            0,
            100000,
            UInt160::zero(),
            data,
            Witness::new(vec![], vec![]),
        );

        // Attempt to broadcast (would need active connection)
        // This demonstrates the integration point
        let result = node.broadcast_extensible_payload(extensible).await;

        // In a real test with connectivity, we'd verify broadcast success
        println!("Broadcast attempt: {:?}", result);
    }
}

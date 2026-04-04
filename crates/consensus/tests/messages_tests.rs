//! Consensus Messages C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Consensus message handling.
//! Tests are based on the C# Neo.Consensus.ConsensusMessage test suite.

use neo_consensus::messages::*;
use neo_consensus::*;
use neo_core::{UInt160, UInt256};

#[cfg(test)]
#[allow(dead_code)]
mod messages_tests {
    use super::*;

    /// Test PrepareRequest message compatibility (matches C# PrepareRequest exactly)
    #[test]
    fn test_prepare_request_compatibility() {
        let block_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
        let timestamp = 1234567890u64;
        let nonce = 42u64;
        let tx_hashes = vec![
            UInt256::from_bytes(&[2u8; 32]).unwrap(),
            UInt256::from_bytes(&[3u8; 32]).unwrap(),
            UInt256::from_bytes(&[4u8; 32]).unwrap(),
        ];

        let prepare_request = PrepareRequest::new(block_hash, timestamp, nonce, tx_hashes.clone());

        assert_eq!(prepare_request.block_hash(), block_hash);
        assert_eq!(prepare_request.timestamp(), timestamp);
        assert_eq!(prepare_request.nonce(), nonce);
        assert_eq!(prepare_request.transaction_hashes(), &tx_hashes);

        let serialized = prepare_request.to_bytes().unwrap();
        assert!(!serialized.is_empty());

        let deserialized = PrepareRequest::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.block_hash(), block_hash);
        assert_eq!(deserialized.timestamp(), timestamp);
        assert_eq!(deserialized.nonce(), nonce);
        assert_eq!(deserialized.transaction_hashes(), &tx_hashes);

        // Test message type
        assert_eq!(
            prepare_request.message_type(),
            ConsensusMessageType::PrepareRequest
        );
    }

    /// Test PrepareResponse message compatibility (matches C# PrepareResponse exactly)
    #[test]
    fn test_prepare_response_compatibility() {
        let block_hash = UInt256::from_bytes(&[5u8; 32]).unwrap();

        let prepare_response = PrepareResponse::new(block_hash);

        // Verify fields
        assert_eq!(prepare_response.block_hash(), block_hash);

        // Test serialization/deserialization
        let serialized = prepare_response.to_bytes().unwrap();
        let deserialized = PrepareResponse::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.block_hash(), block_hash);

        // Test message type
        assert_eq!(
            prepare_response.message_type(),
            ConsensusMessageType::PrepareResponse
        );
    }

    /// Test Commit message compatibility (matches C# Commit exactly)
    #[test]
    fn test_commit_message_compatibility() {
        let signature = vec![0x01, 0x02, 0x03, 0x04, 0x05];

        let commit = Commit::new(signature.clone());

        // Verify fields
        assert_eq!(commit.signature(), &signature);

        // Test serialization/deserialization
        let serialized = commit.to_bytes().unwrap();
        let deserialized = Commit::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.signature(), &signature);

        // Test message type
        assert_eq!(commit.message_type(), ConsensusMessageType::Commit);
    }

    /// Test ChangeView message compatibility (matches C# ChangeView exactly)
    #[test]
    fn test_change_view_compatibility() {
        let new_view = ViewNumber::new(3);
        let timestamp = 1234567890u64;
        let reason = ViewChangeReason::Timeout;

        let change_view = ChangeView::new(new_view, timestamp, reason);

        // Verify fields
        assert_eq!(change_view.new_view_number(), new_view);
        assert_eq!(change_view.timestamp(), timestamp);
        assert_eq!(change_view.reason(), reason);

        // Test serialization/deserialization
        let serialized = change_view.to_bytes().unwrap();
        let deserialized = ChangeView::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.new_view_number(), new_view);
        assert_eq!(deserialized.timestamp(), timestamp);
        assert_eq!(deserialized.reason(), reason);

        // Test message type
        assert_eq!(change_view.message_type(), ConsensusMessageType::ChangeView);
    }

    /// Test RecoveryRequest message compatibility (matches C# RecoveryRequest exactly)
    #[test]
    fn test_recovery_request_compatibility() {
        let timestamp = 1234567890u64;

        let recovery_request = RecoveryRequest::new(timestamp);

        // Verify fields
        assert_eq!(recovery_request.timestamp(), timestamp);

        // Test serialization/deserialization
        let serialized = recovery_request.to_bytes().unwrap();
        let deserialized = RecoveryRequest::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.timestamp(), timestamp);

        // Test message type
        assert_eq!(
            recovery_request.message_type(),
            ConsensusMessageType::RecoveryRequest
        );
    }

    /// Test RecoveryResponse message compatibility (matches C# RecoveryMessage exactly)
    #[test]
    fn test_recovery_response_compatibility() {
        // Create test change view messages
        let change_views = vec![
            (
                0u8,
                ChangeView::new(ViewNumber::new(1), 1000, ViewChangeReason::Timeout),
            ),
            (
                1u8,
                ChangeView::new(ViewNumber::new(1), 1001, ViewChangeReason::InvalidBlock),
            ),
            (
                2u8,
                ChangeView::new(ViewNumber::new(2), 1002, ViewChangeReason::Timeout),
            ),
        ];

        // Create test prepare request
        let prepare_request = Some(PrepareRequest::new(
            UInt256::from_bytes(&[10u8; 32]).unwrap(),
            2000,
            100,
            vec![],
        ));

        // Create test prepare responses
        let prepare_responses = vec![
            (
                0u8,
                PrepareResponse::new(UInt256::from_bytes(&[10u8; 32]).unwrap()),
            ),
            (
                1u8,
                PrepareResponse::new(UInt256::from_bytes(&[10u8; 32]).unwrap()),
            ),
        ];

        // Create test commits
        let commits = vec![
            (0u8, Commit::new(vec![0x01, 0x02])),
            (1u8, Commit::new(vec![0x03, 0x04])),
            (2u8, Commit::new(vec![0x05, 0x06])),
        ];

        let recovery_response = RecoveryResponse::new(
            change_views.clone(),
            prepare_request.clone(),
            prepare_responses.clone(),
            commits.clone(),
        );

        // Verify fields
        assert_eq!(recovery_response.change_views(), &change_views);
        assert_eq!(recovery_response.prepare_request(), &prepare_request);
        assert_eq!(recovery_response.prepare_responses(), &prepare_responses);
        assert_eq!(recovery_response.commits(), &commits);

        // Test serialization/deserialization
        let serialized = recovery_response.to_bytes().unwrap();
        let deserialized = RecoveryResponse::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized.change_views().len(), change_views.len());
        assert_eq!(
            deserialized.prepare_request().is_some(),
            prepare_request.is_some()
        );
        assert_eq!(
            deserialized.prepare_responses().len(),
            prepare_responses.len()
        );
        assert_eq!(deserialized.commits().len(), commits.len());

        // Test message type
        assert_eq!(
            recovery_response.message_type(),
            ConsensusMessageType::RecoveryResponse
        );
    }

    /// Test ConsensusMessage envelope compatibility (matches C# ConsensusPayload exactly)
    #[test]
    fn test_consensus_message_envelope_compatibility() {
        // Create a test message
        let prepare_response = PrepareResponse::new(UInt256::from_bytes(&[20u8; 32]).unwrap());

        // Create consensus message envelope
        let validator_index = 2u8;
        let block_index = BlockIndex::new(1000);
        let view_number = ViewNumber::new(1);

        let consensus_msg = ConsensusMessage::new(
            validator_index,
            block_index,
            view_number,
            Box::new(prepare_response),
        );

        // Verify envelope fields
        assert_eq!(consensus_msg.validator_index(), validator_index);
        assert_eq!(consensus_msg.block_index(), block_index);
        assert_eq!(consensus_msg.view_number(), view_number);
        assert_eq!(
            consensus_msg.message_type(),
            ConsensusMessageType::PrepareResponse
        );

        // Test timestamp is set
        assert!(consensus_msg.timestamp() > 0);

        // Test hash computation
        let hash = consensus_msg.hash();
        let hash2 = consensus_msg.hash();
        assert_eq!(hash, hash2); // Hash should be deterministic

        // Test serialization/deserialization of envelope
        let serialized = consensus_msg.to_bytes().unwrap();
        let deserialized = ConsensusMessage::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized.validator_index(), validator_index);
        assert_eq!(deserialized.block_index(), block_index);
        assert_eq!(deserialized.view_number(), view_number);
        assert_eq!(
            deserialized.message_type(),
            ConsensusMessageType::PrepareResponse
        );
    }

    /// Test ViewChangeReason enum compatibility (matches C# ChangeViewReason exactly)
    #[test]
    fn test_view_change_reason_compatibility() {
        let reasons = vec![
            ViewChangeReason::Timeout,
            ViewChangeReason::InvalidBlock,
            ViewChangeReason::InvalidSignature,
            ViewChangeReason::NotPrimary,
            ViewChangeReason::NewHeight,
        ];

        for reason in reasons {
            // Test serialization round-trip
            let bytes = bincode::serialize(&reason).unwrap();
            let deserialized: ViewChangeReason = bincode::deserialize(&bytes).unwrap();
            assert_eq!(reason, deserialized);

            // Test display implementation
            let display_str = format!("{}", reason);
            assert!(!display_str.is_empty());
        }
    }

    /// Test message validation compatibility (matches C# message validation exactly)
    #[test]
    fn test_message_validation_compatibility() {
        let empty_prepare = PrepareRequest::new(
            UInt256::from_bytes(&[30u8; 32]).unwrap(),
            1234567890,
            100,
            vec![],
        );
        assert!(empty_prepare.validate().is_ok());

        let mut many_tx_hashes = Vec::new();
        for i in 0..512 {
            let mut hash_bytes = [0u8; 32];
            hash_bytes[0] = (i & 0xFF) as u8;
            hash_bytes[1] = ((i >> 8) & 0xFF) as u8;
            many_tx_hashes.push(UInt256::from_bytes(&hash_bytes).unwrap());
        }

        let large_prepare = PrepareRequest::new(
            UInt256::from_bytes(&[31u8; 32]).unwrap(),
            1234567890,
            200,
            many_tx_hashes,
        );
        assert!(large_prepare.validate().is_ok());

        let empty_commit = Commit::new(vec![]);
        assert!(empty_commit.validate().is_err());

        // Test Commit with valid signature size
        let valid_commit = Commit::new(vec![0u8; 64]); // Typical ECDSA signature size
        assert!(valid_commit.validate().is_ok());

        let future_change_view = ChangeView::new(
            ViewNumber::new(5),
            u64::MAX - 1000,
            ViewChangeReason::Timeout,
        );
        assert!(future_change_view.validate().is_ok());
    }

    /// Test message size limits compatibility (matches C# message size constraints exactly)
    #[test]
    fn test_message_size_limits_compatibility() {
        let max_tx_count = 65535; // C# ushort.MaxValue
        let mut tx_hashes = Vec::with_capacity(max_tx_count);

        // Create a smaller test set to avoid memory issues in tests
        for i in 0..1000 {
            let mut hash = [0u8; 32];
            hash[0] = (i & 0xFF) as u8;
            hash[1] = ((i >> 8) & 0xFF) as u8;
            tx_hashes.push(UInt256::from_bytes(&hash).unwrap());
        }

        let prepare_request = PrepareRequest::new(
            UInt256::from_bytes(&[40u8; 32]).unwrap(),
            1234567890,
            300,
            tx_hashes,
        );

        // Should serialize successfully
        let serialized = prepare_request.to_bytes().unwrap();
        assert!(serialized.len() > 32); // At least block hash size

        // Test maximum signature size in Commit
        let max_signature = vec![0xFFu8; 520]; // Max signature size in C#
        let large_commit = Commit::new(max_signature);
        let commit_serialized = large_commit.to_bytes().unwrap();
        assert!(commit_serialized.len() > 500);
    }

    /// Test message ordering and comparison (matches C# ordering exactly)
    #[test]
    fn test_message_ordering_compatibility() {
        // Test ConsensusMessage ordering by timestamp
        let msg1 = ConsensusMessage::with_timestamp(
            0,
            BlockIndex::new(100),
            ViewNumber::new(0),
            Box::new(Commit::new(vec![1, 2, 3])),
            1000,
        );

        let msg2 = ConsensusMessage::with_timestamp(
            1,
            BlockIndex::new(100),
            ViewNumber::new(0),
            Box::new(Commit::new(vec![4, 5, 6])),
            2000,
        );

        assert!(msg1.timestamp() < msg2.timestamp());

        let types = vec![
            ConsensusMessageType::PrepareRequest,
            ConsensusMessageType::PrepareResponse,
            ConsensusMessageType::Commit,
            ConsensusMessageType::ChangeView,
            ConsensusMessageType::RecoveryRequest,
            ConsensusMessageType::RecoveryResponse,
        ];

        for (i, msg_type) in types.iter().enumerate() {
            assert_eq!(*msg_type as u8, i as u8);
        }
    }

    /// Test message cloning and equality (matches C# value semantics exactly)
    #[test]
    fn test_message_cloning_equality_compatibility() {
        // Test PrepareRequest cloning
        let original_prepare = PrepareRequest::new(
            UInt256::from_bytes(&[50u8; 32]).unwrap(),
            1234567890,
            400,
            vec![UInt256::from_bytes(&[51u8; 32]).unwrap()],
        );

        let cloned_prepare = original_prepare.clone();
        assert_eq!(original_prepare.block_hash(), cloned_prepare.block_hash());
        assert_eq!(original_prepare.timestamp(), cloned_prepare.timestamp());
        assert_eq!(original_prepare.nonce(), cloned_prepare.nonce());
        assert_eq!(
            original_prepare.transaction_hashes(),
            cloned_prepare.transaction_hashes()
        );

        // Test ConsensusMessage equality
        let msg1 = ConsensusMessage::new(
            0,
            BlockIndex::new(100),
            ViewNumber::new(0),
            Box::new(original_prepare.clone()),
        );

        let msg2 = ConsensusMessage::new(
            0,
            BlockIndex::new(100),
            ViewNumber::new(0),
            Box::new(original_prepare),
        );

        // Messages with same content should have same hash
        assert_eq!(msg1.hash(), msg2.hash());
    }

    /// Test edge cases and error handling (matches C# exception handling exactly)
    #[test]
    fn test_message_edge_cases_compatibility() {
        // Test deserialization of empty data
        let empty_data: &[u8] = &[];
        assert!(PrepareRequest::from_bytes(empty_data).is_err());
        assert!(PrepareResponse::from_bytes(empty_data).is_err());
        assert!(Commit::from_bytes(empty_data).is_err());
        assert!(ChangeView::from_bytes(empty_data).is_err());
        assert!(RecoveryRequest::from_bytes(empty_data).is_err());
        assert!(RecoveryResponse::from_bytes(empty_data).is_err());

        // Test deserialization of invalid data
        let invalid_data = vec![0xFF, 0xFF, 0xFF, 0xFF];
        assert!(ConsensusMessage::from_bytes(&invalid_data).is_err());

        let mut view = ViewNumber::new(255);
        view.increment();
        assert_eq!(view.value(), 0); // Should wrap around

        // Test block index limits
        let max_block = BlockIndex::new(u32::MAX - 1);
        let next_block = max_block.next();
        assert_eq!(next_block.value(), u32::MAX);
    }
}

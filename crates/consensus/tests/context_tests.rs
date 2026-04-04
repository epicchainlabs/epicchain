//! Consensus Context C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Consensus context management.
//! Tests are based on the C# Neo.Consensus.ConsensusContext test suite.

use neo_consensus::context::*;
use neo_consensus::*;
use neo_core::{Block, Transaction, UInt160, UInt256};
use std::collections::HashMap;

#[cfg(test)]
#[allow(dead_code)]
mod context_tests {
    use super::*;

    /// Test ConsensusContext creation and initialization (matches C# ConsensusContext exactly)
    #[test]
    fn test_consensus_context_creation_compatibility() {
        let validator_count = 7;
        let my_index = 2;
        let block_index = BlockIndex::new(1000);
        let view_number = ViewNumber::new(0);

        let context = ConsensusContext::new(validator_count, my_index, block_index, view_number);

        // Verify initial state matches C#
        assert_eq!(context.validator_count(), validator_count);
        assert_eq!(context.my_index(), my_index);
        assert_eq!(context.block_index(), block_index);
        assert_eq!(context.view_number(), view_number);
        assert_eq!(context.phase(), ConsensusPhase::Initial);
        assert!(!context.is_primary());
        assert!(context.is_backup());

        // Test timestamps
        assert!(context.start_time() > 0);
        assert_eq!(context.last_activity_time(), context.start_time());

        // Test message tracking
        assert_eq!(context.prepare_request_received(), false);
        assert_eq!(context.prepare_responses_count(), 0);
        assert_eq!(context.commits_count(), 0);
        assert_eq!(context.change_views_count(), 0);
    }

    /// Test ConsensusPhase transitions (matches C# state machine exactly)
    #[test]
    fn test_consensus_phase_transitions_compatibility() {
        let mut context = ConsensusContext::new(7, 0, BlockIndex::new(100), ViewNumber::new(0));

        // Initial state
        assert_eq!(context.phase(), ConsensusPhase::Initial);

        context.set_phase(ConsensusPhase::RequestSending);
        assert_eq!(context.phase(), ConsensusPhase::RequestSending);

        // Transition to RequestReceived
        context.set_phase(ConsensusPhase::RequestReceived);
        assert_eq!(context.phase(), ConsensusPhase::RequestReceived);

        // Transition to SignatureSending
        context.set_phase(ConsensusPhase::SignatureSending);
        assert_eq!(context.phase(), ConsensusPhase::SignatureSending);

        // Transition to SignatureReceived
        context.set_phase(ConsensusPhase::SignatureReceived);
        assert_eq!(context.phase(), ConsensusPhase::SignatureReceived);

        // Transition to BlockSending
        context.set_phase(ConsensusPhase::BlockSending);
        assert_eq!(context.phase(), ConsensusPhase::BlockSending);

        // Transition to BlockSent
        context.set_phase(ConsensusPhase::BlockSent);
        assert_eq!(context.phase(), ConsensusPhase::BlockSent);

        // Test ViewChanging phase
        context.set_phase(ConsensusPhase::ViewChanging);
        assert_eq!(context.phase(), ConsensusPhase::ViewChanging);
    }

    /// Test primary node calculation (matches C# GetPrimaryIndex exactly)
    #[test]
    fn test_primary_calculation_compatibility() {
        // Test with different view numbers
        let validator_count = 7;

        for view in 0..20 {
            let context = ConsensusContext::new(
                validator_count,
                0,
                BlockIndex::new(100),
                ViewNumber::new(view),
            );

            let expected_primary = (view as usize) % validator_count;
            assert_eq!(context.primary_index(), expected_primary);

            for i in 0..validator_count {
                let validator_context = ConsensusContext::new(
                    validator_count,
                    i,
                    BlockIndex::new(100),
                    ViewNumber::new(view),
                );

                if i == expected_primary {
                    assert!(validator_context.is_primary());
                    assert!(!validator_context.is_backup());
                } else {
                    assert!(!validator_context.is_primary());
                    assert!(validator_context.is_backup());
                }
            }
        }
    }

    /// Test message tracking (matches C# message collection exactly)
    #[test]
    fn test_message_tracking_compatibility() {
        let mut context = ConsensusContext::new(7, 2, BlockIndex::new(100), ViewNumber::new(0));

        // Test PrepareRequest tracking
        assert!(!context.prepare_request_received());
        let prepare_request = PrepareRequest::new(
            UInt256::from_bytes(&[1u8; 32]).unwrap(),
            1234567890,
            42,
            vec![],
        );
        context.set_prepare_request(prepare_request);
        assert!(context.prepare_request_received());

        // Test PrepareResponse tracking
        assert_eq!(context.prepare_responses_count(), 0);
        for i in 0..5 {
            let response = PrepareResponse::new(UInt256::from_bytes(&[1u8; 32]).unwrap());
            context.add_prepare_response(i, response);
        }
        assert_eq!(context.prepare_responses_count(), 5);

        let duplicate_response = PrepareResponse::new(UInt256::from_bytes(&[1u8; 32]).unwrap());
        context.add_prepare_response(0, duplicate_response);
        assert_eq!(context.prepare_responses_count(), 5);

        // Test Commit tracking
        assert_eq!(context.commits_count(), 0);
        for i in 0..6 {
            let commit = Commit::new(vec![i; 64]);
            context.add_commit(i, commit);
        }
        assert_eq!(context.commits_count(), 6);

        // Test ChangeView tracking
        assert_eq!(context.change_views_count(), 0);
        for i in 0..3 {
            let change_view =
                ChangeView::new(ViewNumber::new(1), 1234567890, ViewChangeReason::Timeout);
            context.add_change_view(i, change_view);
        }
        assert_eq!(context.change_views_count(), 3);
    }

    /// Test consensus round management (matches C# round tracking exactly)
    #[test]
    fn test_consensus_round_management_compatibility() {
        let mut context = ConsensusContext::new(7, 1, BlockIndex::new(100), ViewNumber::new(0));

        // Initial round
        let round = context.current_round();
        assert_eq!(round.block_index(), BlockIndex::new(100));
        assert_eq!(round.view_number(), ViewNumber::new(0));
        assert_eq!(round.start_time(), context.start_time());

        // Test round timing
        std::thread::sleep(std::time::Duration::from_millis(10));
        context.update_activity_time();
        assert!(context.last_activity_time() > context.start_time());

        // Test new round creation
        context.new_round(BlockIndex::new(101), ViewNumber::new(0));
        let new_round = context.current_round();
        assert_eq!(new_round.block_index(), BlockIndex::new(101));
        assert_eq!(new_round.view_number(), ViewNumber::new(0));

        // Test view change within round
        context.change_view(ViewNumber::new(1));
        assert_eq!(context.view_number(), ViewNumber::new(1));
        assert_eq!(context.block_index(), BlockIndex::new(101)); // Block index unchanged

        // Verify message collections are cleared
        assert_eq!(context.prepare_responses_count(), 0);
        assert_eq!(context.commits_count(), 0);
        assert!(!context.prepare_request_received());
    }

    /// Test timer management (matches C# timer behavior exactly)
    #[test]
    fn test_timer_management_compatibility() {
        let mut timer = ConsensusTimer::new();

        // Test timer states
        assert!(!timer.is_running());
        assert_eq!(timer.elapsed_ms(), 0);

        // Start timer
        timer.start();
        assert!(timer.is_running());

        // Test elapsed time
        std::thread::sleep(std::time::Duration::from_millis(50));
        let elapsed = timer.elapsed_ms();
        assert!(elapsed >= 50);
        assert!(elapsed < 100); // Should not be too far off

        // Stop timer
        timer.stop();
        assert!(!timer.is_running());
        let final_elapsed = timer.elapsed_ms();

        // Elapsed time should not increase after stopping
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert_eq!(timer.elapsed_ms(), final_elapsed);

        // Reset timer
        timer.reset();
        assert_eq!(timer.elapsed_ms(), 0);
        assert!(!timer.is_running());

        // Test restart
        timer.restart();
        assert!(timer.is_running());
        assert!(timer.elapsed_ms() < 10); // Should be close to 0
    }

    /// Test block proposal tracking (matches C# block creation exactly)
    #[test]
    fn test_block_proposal_tracking_compatibility() {
        let mut context = ConsensusContext::new(7, 0, BlockIndex::new(100), ViewNumber::new(0));

        // Test proposed block tracking
        assert!(context.proposed_block().is_none());

        // Create test block
        let block = Block {
            version: 0,
            prev_hash: UInt256::from_bytes(&[1u8; 32]).unwrap(),
            merkle_root: UInt256::from_bytes(&[2u8; 32]).unwrap(),
            timestamp: 1234567890,
            index: 100,
            next_consensus: UInt160::from_bytes(&[3u8; 20]).unwrap(),
            witness: vec![],
            consensus_data: ConsensusData {
                primary_index: 0,
                nonce: 42,
            },
            transactions: vec![],
        };

        context.set_proposed_block(block.clone());
        assert!(context.proposed_block().is_some());
        assert_eq!(context.proposed_block().unwrap().index, 100);

        // Test transaction collection
        assert_eq!(context.transaction_count(), 0);

        let tx = Transaction::new();
        context.add_transaction(tx);
        assert_eq!(context.transaction_count(), 1);

        // Test transaction hashes
        let hashes = context.transaction_hashes();
        assert_eq!(hashes.len(), 1);

        // Test clearing transactions
        context.clear_transactions();
        assert_eq!(context.transaction_count(), 0);
    }

    /// Test signature collection (matches C# signature handling exactly)
    #[test]
    fn test_signature_collection_compatibility() {
        let mut context = ConsensusContext::new(7, 2, BlockIndex::new(100), ViewNumber::new(0));

        // Test signature threshold calculation
        assert_eq!(context.required_signatures(), 5); // 7 - (7-1)/3 = 5

        // Test signature collection
        let mut signatures = HashMap::new();
        for i in 0..4 {
            signatures.insert(i as u8, vec![i; 64]);
        }
        context.set_signatures(signatures.clone());

        assert!(!context.has_enough_signatures());
        assert_eq!(context.signature_count(), 4);

        // Add one more signature to reach threshold
        signatures.insert(4, vec![4; 64]);
        context.set_signatures(signatures.clone());

        assert!(context.has_enough_signatures());
        assert_eq!(context.signature_count(), 5);

        // Test getting specific signature
        assert_eq!(context.get_signature(0), Some(&vec![0; 64]));
        assert_eq!(context.get_signature(7), None); // Out of range

        // Test clearing signatures
        context.clear_signatures();
        assert_eq!(context.signature_count(), 0);
        assert!(!context.has_enough_signatures());
    }

    /// Test view timeout calculation (matches C# timeout logic exactly)
    #[test]
    fn test_view_timeout_calculation_compatibility() {
        let base_timeout_ms = 15000; // 15 seconds base

        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(0), base_timeout_ms),
            15000
        );
        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(1), base_timeout_ms),
            30000
        );
        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(2), base_timeout_ms),
            60000
        );
        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(3), base_timeout_ms),
            120000
        );

        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(6), base_timeout_ms),
            960000
        ); // 64x base
        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(7), base_timeout_ms),
            960000
        ); // Still 64x
        assert_eq!(
            ConsensusContext::calculate_view_timeout(ViewNumber::new(10), base_timeout_ms),
            960000
        ); // Still 64x
    }

    /// Test context reset (matches C# Reset method exactly)
    #[test]
    fn test_context_reset_compatibility() {
        let mut context = ConsensusContext::new(7, 3, BlockIndex::new(100), ViewNumber::new(2));

        // Add some state
        context.set_phase(ConsensusPhase::SignatureSending);
        let pr = PrepareRequest::new(UInt256::zero(), 123, 456, vec![]);
        context.set_prepare_request(pr);
        context.add_prepare_response(0, PrepareResponse::new(UInt256::zero()));
        context.add_commit(1, Commit::new(vec![1; 64]));

        // Reset context
        context.reset();

        // Verify all state is cleared
        assert_eq!(context.phase(), ConsensusPhase::Initial);
        assert!(!context.prepare_request_received());
        assert_eq!(context.prepare_responses_count(), 0);
        assert_eq!(context.commits_count(), 0);
        assert_eq!(context.change_views_count(), 0);
        assert_eq!(context.signature_count(), 0);
        assert!(context.proposed_block().is_none());
        assert_eq!(context.transaction_count(), 0);

        // View and block index should remain
        assert_eq!(context.view_number(), ViewNumber::new(2));
        assert_eq!(context.block_index(), BlockIndex::new(100));
    }

    /// Test consensus message validation (matches C# validation rules exactly)
    #[test]
    fn test_message_validation_compatibility() {
        let context = ConsensusContext::new(7, 1, BlockIndex::new(100), ViewNumber::new(1));

        // Test valid message
        let valid_msg = ConsensusMessage::new(
            2,
            BlockIndex::new(100),
            ViewNumber::new(1),
            Box::new(Commit::new(vec![1; 64])),
        );
        assert!(context.validate_message(&valid_msg).is_ok());

        // Test wrong block index
        let wrong_block_msg = ConsensusMessage::new(
            2,
            BlockIndex::new(99),
            ViewNumber::new(1),
            Box::new(Commit::new(vec![1; 64])),
        );
        assert!(context.validate_message(&wrong_block_msg).is_err());

        let wrong_view_msg = ConsensusMessage::new(
            2,
            BlockIndex::new(100),
            ViewNumber::new(0),
            Box::new(Commit::new(vec![1; 64])),
        );
        assert!(context.validate_message(&wrong_view_msg).is_err());

        // Test invalid validator index
        let invalid_validator_msg = ConsensusMessage::new(
            7, // Out of range for 7 validators (0-6)
            BlockIndex::new(100),
            ViewNumber::new(1),
            Box::new(Commit::new(vec![1; 64])),
        );
        assert!(context.validate_message(&invalid_validator_msg).is_err());

        let future_view_msg = ConsensusMessage::new(
            2,
            BlockIndex::new(100),
            ViewNumber::new(1), // Current view
            Box::new(ChangeView::new(
                ViewNumber::new(2),
                123,
                ViewChangeReason::Timeout,
            )),
        );
        assert!(context.validate_message(&future_view_msg).is_ok());
    }

    /// Test fault tolerance calculations (matches C# Byzantine fault tolerance exactly)
    #[test]
    fn test_fault_tolerance_calculations_compatibility() {
        // Test different validator counts
        let test_cases = vec![
            (4, 1, 3),   // f=1, required=3
            (7, 2, 5),   // f=2, required=5
            (10, 3, 7),  // f=3, required=7
            (13, 4, 9),  // f=4, required=9
            (16, 5, 11), // f=5, required=11
        ];

        for (validator_count, expected_f, expected_required) in test_cases {
            let context =
                ConsensusContext::new(validator_count, 0, BlockIndex::new(100), ViewNumber::new(0));

            assert_eq!(context.f_count(), expected_f);
            assert_eq!(context.required_signatures(), expected_required);

            assert_eq!(context.m_count(), expected_f + 1);
        }
    }

    /// Test state persistence (matches C# state serialization exactly)
    #[test]
    fn test_state_persistence_compatibility() {
        let mut context = ConsensusContext::new(7, 2, BlockIndex::new(100), ViewNumber::new(1));

        // Set up complex state
        context.set_phase(ConsensusPhase::SignatureReceived);
        context.set_prepare_request(PrepareRequest::new(
            UInt256::from_bytes(&[10u8; 32]).unwrap(),
            123456,
            789,
            vec![UInt256::from_bytes(&[11u8; 32]).unwrap()],
        ));

        for i in 0..4 {
            context.add_prepare_response(
                i,
                PrepareResponse::new(UInt256::from_bytes(&[10u8; 32]).unwrap()),
            );
            context.add_commit(i, Commit::new(vec![i; 64]));
        }

        // Serialize state
        let serialized = context.serialize_state().unwrap();
        assert!(!serialized.is_empty());

        // Deserialize to new context
        let restored_context = ConsensusContext::deserialize_state(&serialized).unwrap();

        // Verify all state is preserved
        assert_eq!(
            restored_context.validator_count(),
            context.validator_count()
        );
        assert_eq!(restored_context.my_index(), context.my_index());
        assert_eq!(restored_context.block_index(), context.block_index());
        assert_eq!(restored_context.view_number(), context.view_number());
        assert_eq!(restored_context.phase(), context.phase());
        assert_eq!(
            restored_context.prepare_request_received(),
            context.prepare_request_received()
        );
        assert_eq!(
            restored_context.prepare_responses_count(),
            context.prepare_responses_count()
        );
        assert_eq!(restored_context.commits_count(), context.commits_count());
    }

    /// Test edge cases and error conditions (matches C# error handling exactly)
    #[test]
    fn test_context_edge_cases_compatibility() {
        // Test minimum validator count
        let min_context = ConsensusContext::new(4, 0, BlockIndex::new(0), ViewNumber::new(0));
        assert_eq!(min_context.f_count(), 1);
        assert_eq!(min_context.required_signatures(), 3);

        // Test view number wrapping
        let mut wrap_context =
            ConsensusContext::new(7, 0, BlockIndex::new(100), ViewNumber::new(255));
        wrap_context.change_view(ViewNumber::new(0)); // Wraps to 0
        assert_eq!(wrap_context.view_number(), ViewNumber::new(0));

        let mut context = ConsensusContext::new(7, 0, BlockIndex::new(100), ViewNumber::new(0));

        // Should handle gracefully
        context.add_prepare_response(10, PrepareResponse::new(UInt256::zero())); // Out of range
        assert_eq!(context.prepare_responses_count(), 0); // Not added

        // Test duplicate message handling
        context.add_commit(0, Commit::new(vec![1; 64]));
        context.add_commit(0, Commit::new(vec![2; 64])); // Different content, same validator
        assert_eq!(context.commits_count(), 1); // Should not duplicate

        // Test phase transitions from invalid states
        context.set_phase(ConsensusPhase::BlockSent);
        context.set_phase(ConsensusPhase::RequestSending); // Invalid transition
                                                           // Implementation should handle this gracefully
    }
}

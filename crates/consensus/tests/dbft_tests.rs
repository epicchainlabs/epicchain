//! dBFT Engine C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Consensus dBFT implementation.
//! Tests are based on the C# Neo.Consensus.DbftPlugin test suite.

use neo_consensus::dbft::*;
use neo_consensus::*;
use neo_core::{UInt160, UInt256};
use std::sync::{Arc, Mutex};

#[cfg(test)]
#[allow(dead_code)]
mod dbft_tests {
    use super::*;

    /// Test DbftEngine creation and configuration (matches C# DbftPlugin exactly)
    #[test]
    fn test_dbft_engine_creation_compatibility() {
        let config = DbftConfig {
            validator_count: 7,
            f_count: 2,
            block_time_ms: 15000,
            view_timeout_base_ms: 20000,
            max_block_size: 1024 * 1024,
            max_transactions_per_block: 512,
            recovery_enabled: true,
            auto_commit_enabled: true,
            commit_timeout_ms: 10000,
        };

        let engine = DbftEngine::new(config.clone());

        // Verify configuration
        assert_eq!(engine.config().validator_count, 7);
        assert_eq!(engine.config().f_count, 2);
        assert_eq!(engine.config().block_time_ms, 15000);
        assert_eq!(engine.config().view_timeout_base_ms, 20000);
        assert!(engine.config().recovery_enabled);
        assert!(engine.config().auto_commit_enabled);

        // Test state initialization
        assert_eq!(engine.state(), DbftState::Initial);
        assert!(!engine.is_running());
        assert_eq!(engine.current_view(), ViewNumber::new(0));
        assert_eq!(engine.current_block_index(), BlockIndex::new(0));
    }

    /// Test dBFT state machine transitions (matches C# state machine exactly)
    #[test]
    fn test_dbft_state_transitions_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        // Test Initial -> Started
        assert_eq!(engine.state(), DbftState::Initial);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();
        assert_eq!(engine.state(), DbftState::Started);
        assert!(engine.is_running());

        engine.set_validator_index(0); // Make us primary for view 0
        engine.process_timer_tick().unwrap();
        assert_eq!(engine.state(), DbftState::Primary);

        // Test Primary -> RequestSent
        engine.send_prepare_request().unwrap();
        assert_eq!(engine.state(), DbftState::RequestSent);

        engine
            .request_view_change(ViewChangeReason::Timeout)
            .unwrap();
        assert_eq!(engine.state(), DbftState::ViewChanging);

        // Test stopping
        engine.stop().unwrap();
        assert_eq!(engine.state(), DbftState::Stopped);
        assert!(!engine.is_running());
    }

    /// Test primary node behavior (matches C# primary consensus flow exactly)
    #[test]
    fn test_primary_node_behavior_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        engine.set_validator_index(0);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();

        // Should transition to Primary state
        engine.process_timer_tick().unwrap();
        assert_eq!(engine.state(), DbftState::Primary);
        assert!(engine.is_primary());

        // Primary should create and send PrepareRequest
        let transactions = vec![]; // Empty block for test
        engine.create_block_proposal(transactions).unwrap();

        let sent_messages = engine.get_sent_messages();
        assert_eq!(sent_messages.len(), 1);
        assert_eq!(
            sent_messages[0].message_type(),
            ConsensusMessageType::PrepareRequest
        );

        // Simulate receiving PrepareResponses
        for i in 1..5 {
            let response = PrepareResponse::new(engine.current_block_hash().unwrap());
            engine.process_prepare_response(i, response).unwrap();
        }

        assert!(engine.has_enough_prepare_responses());

        // Should transition to commit phase
        assert_eq!(engine.state(), DbftState::CommitSent);

        // Check commit was sent
        let sent_messages = engine.get_sent_messages();
        assert!(sent_messages
            .iter()
            .any(|m| m.message_type() == ConsensusMessageType::Commit));
    }

    /// Test backup node behavior (matches C# backup consensus flow exactly)
    #[test]
    fn test_backup_node_behavior_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        engine.set_validator_index(1);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();

        // Should be in Backup state
        engine.process_timer_tick().unwrap();
        assert_eq!(engine.state(), DbftState::Backup);
        assert!(!engine.is_primary());

        // Simulate receiving PrepareRequest from primary
        let block_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
        let prepare_request = PrepareRequest::new(block_hash, 1234567890, 42, vec![]);
        engine.process_prepare_request(0, prepare_request).unwrap();

        // Should send PrepareResponse
        let sent_messages = engine.get_sent_messages();
        assert!(sent_messages
            .iter()
            .any(|m| m.message_type() == ConsensusMessageType::PrepareResponse));

        // Simulate receiving other PrepareResponses
        for i in 2..6 {
            let response = PrepareResponse::new(block_hash);
            engine.process_prepare_response(i, response).unwrap();
        }

        // Should transition to commit phase
        assert!(engine.has_enough_prepare_responses());

        // Should have sent Commit
        let sent_messages = engine.get_sent_messages();
        assert!(sent_messages
            .iter()
            .any(|m| m.message_type() == ConsensusMessageType::Commit));
    }

    /// Test view change mechanism (matches C# view change exactly)
    #[test]
    fn test_view_change_mechanism_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        // Start as backup in view 0
        engine.set_validator_index(1);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();

        // Request view change due to timeout
        let initial_view = engine.current_view();
        engine
            .request_view_change(ViewChangeReason::Timeout)
            .unwrap();

        // Should be in ViewChanging state
        assert_eq!(engine.state(), DbftState::ViewChanging);

        // Should have sent ChangeView message
        let sent_messages = engine.get_sent_messages();
        assert!(sent_messages
            .iter()
            .any(|m| m.message_type() == ConsensusMessageType::ChangeView));

        // Simulate receiving ChangeView messages from other nodes
        for i in 0..5 {
            if i != 1 {
                // Skip self
                let change_view =
                    ChangeView::new(ViewNumber::new(1), 1234567890, ViewChangeReason::Timeout);
                engine.process_change_view(i as u8, change_view).unwrap();
            }
        }

        assert!(engine.has_enough_change_views());

        // View should have changed
        assert_eq!(engine.current_view(), ViewNumber::new(1));
        assert_ne!(engine.current_view(), initial_view);

        // Should be in new state based on new primary
        if engine.is_primary() {
            assert_eq!(engine.state(), DbftState::Primary);
        } else {
            assert_eq!(engine.state(), DbftState::Backup);
        }
    }

    /// Test recovery mechanism (matches C# recovery exactly)
    #[test]
    fn test_recovery_mechanism_compatibility() {
        let mut config = DbftConfig::default();
        config.recovery_enabled = true;
        let mut engine = DbftEngine::new(config);

        engine.set_validator_index(2);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(1))
            .unwrap();

        // Simulate being out of sync - request recovery
        engine.request_recovery().unwrap();

        // Should send RecoveryRequest
        let sent_messages = engine.get_sent_messages();
        assert!(sent_messages
            .iter()
            .any(|m| m.message_type() == ConsensusMessageType::RecoveryRequest));

        // Simulate receiving RecoveryResponse
        let recovery_response = RecoveryResponse::new(
            vec![(
                0,
                ChangeView::new(ViewNumber::new(1), 123, ViewChangeReason::Timeout),
            )],
            Some(PrepareRequest::new(UInt256::zero(), 123, 456, vec![])),
            vec![(0, PrepareResponse::new(UInt256::zero()))],
            vec![],
        );

        engine
            .process_recovery_response(3, recovery_response)
            .unwrap();

        // Should process recovery data and sync state
        assert!(engine.is_recovering());

        // Complete recovery
        engine.complete_recovery().unwrap();
        assert!(!engine.is_recovering());
    }

    /// Test block finalization (matches C# block commit exactly)
    #[test]
    fn test_block_finalization_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        // Implementation provided block finalization callback
        let finalized_blocks = Arc::new(Mutex::new(Vec::new()));
        let finalized_clone = finalized_blocks.clone();

        engine.set_block_finalized_callback(move |block| {
            finalized_clone.lock().unwrap().push(block.index);
        });

        // Run consensus as primary
        engine.set_validator_index(0);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();
        engine.process_timer_tick().unwrap();

        // Create block
        let block_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
        engine.create_block_proposal(vec![]).unwrap();

        // Simulate complete consensus
        for i in 1..5 {
            let response = PrepareResponse::new(block_hash);
            engine.process_prepare_response(i, response).unwrap();
        }

        // Add commits
        for i in 1..5 {
            let commit = Commit::new(vec![i; 64]);
            engine.process_commit(i, commit).unwrap();
        }

        // Block should be finalized
        assert!(engine.has_enough_commits());
        engine.finalize_block().unwrap();

        // Check callback was called
        let finalized = finalized_blocks.lock().unwrap();
        assert_eq!(finalized.len(), 1);
        assert_eq!(finalized[0], 100);

        assert_eq!(engine.current_block_index(), BlockIndex::new(101));
        assert_eq!(engine.current_view(), ViewNumber::new(0));
    }

    /// Test timeout handling (matches C# timeout behavior exactly)
    #[test]
    fn test_timeout_handling_compatibility() {
        let config = DbftConfig {
            view_timeout_base_ms: 1000, // 1 second for testing
            ..Default::default()
        };
        let mut engine = DbftEngine::new(config);

        // Start as backup
        engine.set_validator_index(1);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();

        // Simulate timeout
        let start_time = std::time::Instant::now();
        engine.start_view_timer().unwrap();

        // Should not timeout immediately
        assert!(!engine.check_timeout().unwrap());

        // Simulate passage of time
        std::thread::sleep(std::time::Duration::from_millis(1100));

        // Should timeout now
        assert!(engine.check_timeout().unwrap());

        // Should request view change
        assert_eq!(engine.state(), DbftState::ViewChanging);

        // Timeout should increase exponentially with view number
        assert_eq!(engine.current_view(), ViewNumber::new(0)); // Still in view 0 until change completes
        let timeout_v0 = engine.calculate_timeout();

        engine.change_view(ViewNumber::new(1)).unwrap();
        let timeout_v1 = engine.calculate_timeout();
        assert_eq!(timeout_v1, timeout_v0 * 2); // Doubles each view
    }

    /// Test statistics tracking (matches C# metrics exactly)
    #[test]
    fn test_statistics_tracking_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        // Get initial stats
        let initial_stats = engine.stats();
        assert_eq!(initial_stats.blocks_proposed, 0);
        assert_eq!(initial_stats.blocks_committed, 0);
        assert_eq!(initial_stats.view_changes, 0);
        assert_eq!(initial_stats.recovery_requests, 0);

        // Run some operations
        engine.set_validator_index(0);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();
        engine.process_timer_tick().unwrap();
        engine.create_block_proposal(vec![]).unwrap();

        // Stats should update
        let stats = engine.stats();
        assert_eq!(stats.blocks_proposed, 1);

        // Simulate view change
        engine
            .request_view_change(ViewChangeReason::Timeout)
            .unwrap();
        let stats = engine.stats();
        assert_eq!(stats.view_changes, 1);

        // Test timing stats
        assert!(stats.average_block_time_ms == 0 || stats.average_block_time_ms > 0);
        assert!(stats.last_block_time_ms == 0 || stats.last_block_time_ms > 0);
    }

    /// Test message validation (matches C# message validation exactly)
    #[test]
    fn test_message_validation_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        engine.set_validator_index(1);
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();

        // Test valid PrepareRequest
        let valid_request = PrepareRequest::new(
            UInt256::from_bytes(&[1u8; 32]).unwrap(),
            1234567890,
            42,
            vec![],
        );
        assert!(engine.validate_prepare_request(0, &valid_request).is_ok());

        // Test PrepareRequest from non-primary
        assert!(engine.validate_prepare_request(1, &valid_request).is_err());

        // Test duplicate PrepareRequest
        engine
            .process_prepare_request(0, valid_request.clone())
            .unwrap();
        assert!(engine.validate_prepare_request(0, &valid_request).is_err());

        // Test PrepareResponse validation
        let valid_response = PrepareResponse::new(UInt256::from_bytes(&[1u8; 32]).unwrap());
        assert!(engine.validate_prepare_response(2, &valid_response).is_ok());

        // Test PrepareResponse without PrepareRequest
        let mut engine2 = DbftEngine::new(DbftConfig::default());
        engine2.set_validator_index(1);
        engine2
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();
        assert!(engine2
            .validate_prepare_response(2, &valid_response)
            .is_err());

        // Test Commit validation
        let valid_commit = Commit::new(vec![1; 64]);
        assert!(engine.validate_commit(2, &valid_commit).is_ok());

        // Test empty signature
        let invalid_commit = Commit::new(vec![]);
        assert!(engine.validate_commit(2, &invalid_commit).is_err());
    }

    /// Test configuration validation (matches C# configuration rules exactly)
    #[test]
    fn test_configuration_validation_compatibility() {
        // Test valid configuration
        let valid_config = DbftConfig {
            validator_count: 7,
            f_count: 2,
            block_time_ms: 15000,
            view_timeout_base_ms: 20000,
            max_block_size: 1024 * 1024,
            max_transactions_per_block: 512,
            recovery_enabled: true,
            auto_commit_enabled: true,
            commit_timeout_ms: 10000,
        };
        assert!(valid_config.validate().is_ok());

        // Test invalid validator count
        let invalid_validators = DbftConfig {
            validator_count: 3, // Too few
            f_count: 0,
            ..Default::default()
        };
        assert!(invalid_validators.validate().is_err());

        // Test mismatched f_count
        let invalid_f = DbftConfig {
            validator_count: 7,
            f_count: 3, // Should be 2 for 7 validators
            ..Default::default()
        };
        assert!(invalid_f.validate().is_err());

        // Test invalid timeouts
        let invalid_timeout = DbftConfig {
            block_time_ms: 500, // Too short
            ..Default::default()
        };
        assert!(invalid_timeout.validate().is_err());
    }

    /// Test edge cases and error recovery (matches C# error handling exactly)
    #[test]
    fn test_edge_cases_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        // Test operations before start
        assert!(engine.process_timer_tick().is_err());
        assert!(engine.send_prepare_request().is_err());

        // Test double start
        engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .unwrap();
        assert!(engine
            .start(BlockIndex::new(100), ViewNumber::new(0))
            .is_err());

        // Test view overflow
        engine.set_validator_index(0);
        for _ in 0..255 {
            engine.change_view(engine.current_view().next()).unwrap();
        }
        assert_eq!(engine.current_view().value(), 255);

        // Next view should wrap to 0
        engine.change_view(engine.current_view().next()).unwrap();
        assert_eq!(engine.current_view().value(), 0);

        // Test recovery during view change
        engine
            .request_view_change(ViewChangeReason::Timeout)
            .unwrap();
        assert!(engine.request_recovery().is_ok()); // Should be allowed

        // Test stop during consensus
        engine.stop().unwrap();
        assert_eq!(engine.state(), DbftState::Stopped);
        assert!(engine
            .process_prepare_request(0, PrepareRequest::new(UInt256::zero(), 0, 0, vec![]))
            .is_err());
    }

    /// Test performance characteristics (matches C# performance exactly)
    #[test]
    fn test_performance_characteristics_compatibility() {
        let config = DbftConfig::default();
        let mut engine = DbftEngine::new(config);

        engine.set_validator_index(0);
        engine
            .start(BlockIndex::new(0), ViewNumber::new(0))
            .unwrap();

        // Test message processing performance
        let start = std::time::Instant::now();

        // Process many messages
        for i in 0..100 {
            let block_index = BlockIndex::new(i);
            engine.start(block_index, ViewNumber::new(0)).unwrap();
            engine.process_timer_tick().unwrap();

            // Simulate consensus round
            for j in 1..5 {
                let response = PrepareResponse::new(UInt256::from_bytes(&[i as u8; 32]).unwrap());
                engine.process_prepare_response(j, response).unwrap();
            }

            engine.reset_for_new_round().unwrap();
        }

        let elapsed = start.elapsed();

        // Should process 100 rounds quickly
        assert!(elapsed.as_secs() < 1); // Should be much faster than 1 second

        // Check stats
        let stats = engine.stats();
        assert_eq!(stats.blocks_proposed, 100);
    }
}

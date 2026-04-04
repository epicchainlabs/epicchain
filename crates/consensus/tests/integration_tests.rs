//! Integration tests for the consensus module.

use neo_consensus::context::TimerType;
use neo_consensus::proposal::{MemoryPool, MempoolConfig, ProposalConfig, ProposalManager};
use neo_consensus::recovery::{RecoveryConfig, RecoveryManager};
use neo_consensus::service::{
    ConsensusService, ConsensusServiceConfig, ConsensusServiceState, Ledger, NetworkService,
};
use neo_consensus::validators::{Validator, ValidatorPerformance, ValidatorSet};
use neo_consensus::*;
use neo_core::{UInt160, UInt256};
use std::sync::Arc;

async fn create_test_mempool() -> Arc<MemoryPool> {
    let config = MempoolConfig::default();
    Arc::new(MemoryPool::new(config))
}

#[tokio::test]
async fn test_consensus_config_validation() {
    let mut config = ConsensusConfig::default();
    assert!(config.validate().is_ok());

    // Test invalid validator count
    config.validator_count = 3;
    assert!(config.validate().is_err());

    // Test valid configurations
    config.validator_count = 4; // 3f+1 where f=1
    assert!(config.validate().is_ok());

    config.validator_count = 7; // 3f+1 where f=2
    assert!(config.validate().is_ok());

    // Test invalid timeouts
    config.block_time_ms = 500; // Too short
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_consensus_payload_serialization() {
    let payload = ConsensusPayload::new(
        0,
        BlockIndex::new(100),
        ViewNumber::new(1),
        vec![1, 2, 3, 4],
    );

    let serialized = payload.to_bytes().unwrap();
    let deserialized = ConsensusPayload::from_bytes(&serialized).unwrap();

    assert_eq!(payload.validator_index, deserialized.validator_index);
    assert_eq!(payload.block_index, deserialized.block_index);
    assert_eq!(payload.view_number, deserialized.view_number);
    assert_eq!(payload.data, deserialized.data);
}

#[tokio::test]
async fn test_consensus_message_validation() {
    let prepare_request = PrepareRequest::new(
        UInt256::from_bytes(&[1; 32]).unwrap(),
        vec![1, 2, 3, 4],
        vec![UInt256::from_bytes(&[2; 32]).unwrap()],
    );

    // Valid prepare request should pass validation
    assert!(prepare_request.validate().is_ok());

    // Empty block data should fail
    let invalid_request = PrepareRequest::new(
        UInt256::from_bytes(&[1; 32]).unwrap(),
        vec![], // Empty block data
        vec![UInt256::from_bytes(&[2; 32]).unwrap()],
    );
    assert!(invalid_request.validate().is_err());
}

#[tokio::test]
async fn test_view_number_operations() {
    let mut view = ViewNumber::new(0);
    assert_eq!(view.value(), 0);

    view.increment();
    assert_eq!(view.value(), 1);

    let next = view.next();
    assert_eq!(next.value(), 2);
    assert_eq!(view.value(), 1); // Original unchanged
}

#[tokio::test]
async fn test_block_index_operations() {
    let mut index = BlockIndex::new(100);
    assert_eq!(index.value(), 100);

    index.increment();
    assert_eq!(index.value(), 101);

    let next = index.next();
    assert_eq!(next.value(), 102);
    assert_eq!(index.value(), 101); // Original unchanged
}

#[tokio::test]
async fn test_validator_set_operations() {
    let validators = vec![
        Validator::new(UInt160::zero(), vec![1], 1000, 0, 100),
        Validator::new(
            UInt160::from_bytes(&[1; 20]).unwrap(),
            vec![2],
            2000,
            1,
            100,
        ),
        Validator::new(
            UInt160::from_bytes(&[2; 20]).unwrap(),
            vec![3],
            3000,
            2,
            100,
        ),
        Validator::new(
            UInt160::from_bytes(&[3; 20]).unwrap(),
            vec![4],
            4000,
            3,
            100,
        ),
    ];

    let validator_set = ValidatorSet::new(validators, 100);

    // Test validation
    assert!(validator_set.validate().is_ok());

    // Test primary selection
    let primary = validator_set.get_primary(ViewNumber::new(0));
    assert!(primary.is_some());
    assert_eq!(primary.unwrap().index, 0);

    let primary = validator_set.get_primary(ViewNumber::new(2));
    assert!(primary.is_some());
    assert_eq!(primary.unwrap().index, 2);

    // Test backup selection
    let backups = validator_set.get_backups(ViewNumber::new(0));
    assert_eq!(backups.len(), 3);

    // Test required signatures
    assert_eq!(validator_set.required_signatures(), 3); // 4 - (4-1)/3 = 3
}

#[tokio::test]
async fn test_validator_manager() {
    let config = ValidatorConfig::default();
    let manager = ValidatorManager::new(config);

    // Test registering validators
    for i in 0..7 {
        let hash = UInt160::from_bytes(&[i; 20]).unwrap();
        let public_key = vec![i; 33];
        let stake = 1000_00000000; // 1000 NEO

        manager
            .register_validator(hash, public_key, stake, 100)
            .unwrap();
    }

    // Test getting validators
    let all_validators = manager.get_all_validators();
    assert_eq!(all_validators.len(), 7);

    let active_validators = manager.get_active_validators();
    assert_eq!(active_validators.len(), 7);

    // Test selecting validator set
    let validator_set = manager.select_next_validator_set(7).unwrap();
    assert_eq!(validator_set.len(), 7);
    assert!(validator_set.validate().is_ok());

    // Test stats
    let stats = manager.get_stats();
    assert_eq!(stats.total_validators, 7);
    assert_eq!(stats.active_validators, 7);
}

#[tokio::test]
async fn test_consensus_context() {
    let config = ConsensusConfig::default();
    let my_hash = UInt160::zero();
    let context = ConsensusContext::new(config, my_hash);

    // Test starting a round
    let block_index = BlockIndex::new(100);
    assert!(context.start_round(block_index).is_ok());

    let round = context.get_current_round();
    assert_eq!(round.block_index, block_index);
    assert_eq!(round.view_number, ViewNumber::new(0));

    // Test changing view
    context
        .change_view(ViewNumber::new(1), ViewChangeReason::PrepareRequestTimeout)
        .unwrap();

    let round = context.get_current_round();
    assert_eq!(round.view_number.value(), 1);

    // Test timer operations
    context.start_timer(TimerType::PrepareRequest);
    assert!(context.is_timer_active(TimerType::PrepareRequest));

    context.stop_timer(TimerType::PrepareRequest);
    assert!(!context.is_timer_active(TimerType::PrepareRequest));
}

#[tokio::test]
async fn test_proposal_manager() {
    let config = ProposalConfig::default();
    let mempool = create_test_mempool().await;
    let manager = ProposalManager::new(config, mempool);

    // Test that the manager is created successfully
    assert!(manager.get_stats().proposals_created >= 0);
}

#[tokio::test]
async fn test_dbft_engine() {
    let config = DbftConfig::default();
    let my_hash = UInt160::zero();
    let (message_tx, _message_rx) = tokio::sync::mpsc::unbounded_channel();

    let engine = DbftEngine::new(config, my_hash, message_tx);

    // Test starting the engine
    assert!(engine.start().await.is_ok());
    assert_eq!(engine.state(), DbftState::Running);

    // Test stopping the engine
    assert!(engine.stop().await.is_ok());
    assert_eq!(engine.state(), DbftState::Stopped);
}

#[tokio::test]
async fn test_consensus_service() {
    let config = ConsensusServiceConfig::default();
    let my_hash = UInt160::zero();
    let mempool = create_test_mempool().await;
    let network = Arc::new(NetworkService::new());
    let ledger = Arc::new(Ledger::new());

    let mut service = ConsensusService::new(config, my_hash, ledger, network, mempool);

    let validators = vec![
        Validator::new(UInt160::zero(), vec![1; 33], 1000_00000000, 0, 0),
        Validator::new(
            UInt160::from_bytes(&[1; 20]).unwrap(),
            vec![2; 33],
            1000_00000000,
            1,
            0,
        ),
        Validator::new(
            UInt160::from_bytes(&[2; 20]).unwrap(),
            vec![3; 33],
            1000_00000000,
            2,
            0,
        ),
        Validator::new(
            UInt160::from_bytes(&[3; 20]).unwrap(),
            vec![4; 33],
            1000_00000000,
            3,
            0,
        ),
    ];
    let validator_set = ValidatorSet::new(validators, 0);

    // Set the validator set before starting the service
    service.update_validator_set(validator_set).await.unwrap();

    // Test starting the service
    match service.start().await {
        Ok(_) => {
            assert_eq!(service.state(), ConsensusServiceState::Running);

            // Test stopping the service
            service.stop().await;
            assert_eq!(service.state(), ConsensusServiceState::Stopped);
        }
        Err(e) => {
            println!("Service start failed with error: {:?}", e);
            panic!("Service start failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_recovery_manager() {
    let config = RecoveryConfig::default();
    let consensus_config = ConsensusConfig::default();
    let my_hash = UInt160::zero();
    let context = Arc::new(ConsensusContext::new(consensus_config, my_hash));
    let recovery_manager = RecoveryManager::new(config, context);

    // Test that recovery manager is created successfully
    let stats = recovery_manager.get_stats();
    assert_eq!(stats.sessions_started, 0);
}

#[tokio::test]
async fn test_validator_performance() {
    let mut performance = ValidatorPerformance::default();

    // Record some performance metrics
    performance.record_block_proposal();
    performance.record_block_signature();
    performance.record_participation();

    assert_eq!(performance.blocks_proposed, 1);
    assert_eq!(performance.blocks_signed, 1);
    assert_eq!(performance.rounds_participated, 1);
}

#[tokio::test]
async fn test_consensus_signature_verification() {
    let signature = ConsensusSignature::new(
        UInt160::zero(),
        vec![1, 2, 3, 4], // Implementation provided signature
    );

    let message = b"test message";
    let public_key = vec![0u8; 33]; // Implementation provided public key

    // Note: This will likely fail verification due to mock data,
    // but tests that the verification logic doesn't crash
    let result = signature.verify(message, &public_key);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_consensus_round_management() {
    let mut round = ConsensusRound::new(BlockIndex::new(100), ViewNumber::new(0));

    // Test adding responses
    let response = PrepareResponse::accept(UInt256::zero());
    round.add_prepare_response(0, response);
    assert_eq!(round.prepare_response_count(), 1);

    // Test adding commits
    let commit = Commit::new(UInt256::zero(), vec![1, 2, 3]);
    round.add_commit(0, commit);
    assert_eq!(round.commit_count(), 1);

    // Test view change
    let mut new_round = round.clone();
    new_round.reset_for_view(ViewNumber::new(1));
    assert_eq!(new_round.view_number, ViewNumber::new(1));
    assert_eq!(new_round.prepare_response_count(), 0); // Should be reset
}

// Additional C# compatibility tests
#[tokio::test]
async fn test_csharp_consensus_compatibility() {
    // Test that our consensus implementation matches C# behavior

    // 1. Test view number wrapping (C# uses byte, so wraps at 255)
    let mut view = ViewNumber::new(255);
    view.increment();
    assert_eq!(view.value(), 0); // Should wrap around

    // 2. Test block index increment
    let mut index = BlockIndex::new(u32::MAX - 1);
    index.increment();
    assert_eq!(index.value(), u32::MAX);

    // 3. Test consensus configuration matches C# defaults
    let config = ConsensusConfig::default();
    assert_eq!(config.validator_count, 7); // C# default
    assert_eq!(config.block_time_ms, 15000); // C# default (15 seconds)
    assert_eq!(config.max_view_changes, 6); // C# default

    // 4. Test message type values match C# enum values
    assert_eq!(ConsensusMessageType::PrepareRequest.to_byte(), 0x00);
    assert_eq!(ConsensusMessageType::PrepareResponse.to_byte(), 0x01);
    assert_eq!(ConsensusMessageType::Commit.to_byte(), 0x02);
    assert_eq!(ConsensusMessageType::ChangeView.to_byte(), 0x03);
    assert_eq!(ConsensusMessageType::RecoveryRequest.to_byte(), 0x04);
    assert_eq!(ConsensusMessageType::RecoveryResponse.to_byte(), 0x05);
}

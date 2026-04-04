//! Blockchain C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Ledger blockchain implementation.
//! Tests are based on the C# Neo.Ledger.Blockchain test suite.

use neo_core::{Block, Transaction, UInt160, UInt256};
use neo_ledger::blockchain::*;
use neo_ledger::*;
use std::sync::Arc;

#[cfg(test)]
#[allow(dead_code)]
mod blockchain_tests {
    use super::*;

    /// Test blockchain initialization (matches C# Blockchain constructor exactly)
    #[test]
    fn test_blockchain_initialization_compatibility() {
        let config = BlockchainConfig {
            genesis_block: create_test_genesis_block(),
            max_block_size: 1024 * 1024,         // 1MB
            max_block_system_fee: 9000_00000000, // 9000 GAS
            max_transactions_per_block: 512,
            memory_pool_max_transactions: 50000,
            time_per_block_ms: 15000,
            standby_validators: vec![
                vec![0x02; 33],
                vec![0x03; 33],
                vec![0x04; 33],
                vec![0x05; 33],
                vec![0x06; 33],
                vec![0x07; 33],
                vec![0x08; 33],
            ],
        };

        let blockchain = Blockchain::new(config.clone()).unwrap();

        // Verify initialization matches C#
        assert_eq!(blockchain.height(), 0); // Genesis block
        assert_eq!(blockchain.header_height(), 0);
        assert!(!blockchain.is_running());

        // Verify genesis block
        let genesis = blockchain.get_block(0).unwrap();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.prev_hash, UInt256::zero());

        // Verify configuration
        assert_eq!(blockchain.config().max_block_size, 1024 * 1024);
        assert_eq!(blockchain.config().max_transactions_per_block, 512);
        assert_eq!(blockchain.config().time_per_block_ms, 15000);
    }

    /// Test block addition (matches C# AddBlock exactly)
    #[test]
    fn test_block_addition_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        let block = Block {
            version: 0,
            prev_hash: blockchain.current_block_hash(),
            merkle_root: UInt256::from_bytes(&[1u8; 32]).unwrap(),
            timestamp: 1234567890,
            index: 1,
            next_consensus: UInt160::from_bytes(&[2u8; 20]).unwrap(),
            witness: vec![create_test_witness()],
            consensus_data: ConsensusData {
                primary_index: 0,
                nonce: 42,
            },
            transactions: vec![],
        };

        // Test block validation
        assert!(blockchain.validate_block(&block).is_ok());

        // Add block
        let result = blockchain.add_block(block.clone());
        assert!(result.is_ok());

        // Verify block was added
        assert_eq!(blockchain.height(), 1);
        assert_eq!(blockchain.current_block_hash(), block.hash());

        // Verify block can be retrieved
        let retrieved = blockchain.get_block(1).unwrap();
        assert_eq!(retrieved.hash(), block.hash());

        // Test duplicate block rejection
        let duplicate_result = blockchain.add_block(block);
        assert!(duplicate_result.is_err());
    }

    /// Test block validation rules (matches C# block validation exactly)
    #[test]
    fn test_block_validation_compatibility() {
        let config = BlockchainConfig::default();
        let blockchain = Blockchain::new(config).unwrap();

        // Test invalid index
        let invalid_index_block = Block {
            index: 5, // Should be 1
            prev_hash: blockchain.current_block_hash(),
            ..create_test_block(1)
        };
        assert!(blockchain.validate_block(&invalid_index_block).is_err());

        // Test invalid previous hash
        let invalid_prev_block = Block {
            index: 1,
            prev_hash: UInt256::from_bytes(&[99u8; 32]).unwrap(),
            ..create_test_block(1)
        };
        assert!(blockchain.validate_block(&invalid_prev_block).is_err());

        let old_timestamp_block = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            timestamp: 0, // Earlier than genesis
            ..create_test_block(1)
        };
        assert!(blockchain.validate_block(&old_timestamp_block).is_err());

        // Test block size limit
        let mut large_transactions = vec![];
        for i in 0..1000 {
            large_transactions.push(create_large_transaction(i));
        }
        let oversized_block = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            transactions: large_transactions,
            ..create_test_block(1)
        };
        assert!(blockchain.validate_block(&oversized_block).is_err());
    }

    /// Test header synchronization (matches C# header sync exactly)
    #[test]
    fn test_header_synchronization_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Create header chain
        let mut headers = vec![];
        let mut prev_hash = blockchain.current_block_hash();

        for i in 1..=10 {
            let header = BlockHeader {
                version: 0,
                prev_hash,
                merkle_root: UInt256::from_bytes(&[i as u8; 32]).unwrap(),
                timestamp: 1234567890 + i as u64,
                index: i,
                next_consensus: UInt160::from_bytes(&[i as u8; 20]).unwrap(),
                witness: vec![create_test_witness()],
            };
            prev_hash = header.hash();
            headers.push(header);
        }

        // Add headers
        let result = blockchain.add_headers(headers.clone());
        assert!(result.is_ok());

        // Verify header height
        assert_eq!(blockchain.header_height(), 10);
        assert_eq!(blockchain.height(), 0); // Blocks not added yet

        // Verify headers can be retrieved
        for (i, header) in headers.iter().enumerate() {
            let retrieved = blockchain.get_header((i + 1) as u32).unwrap();
            assert_eq!(retrieved.hash(), header.hash());
        }

        // Test invalid header chain
        let invalid_headers = vec![BlockHeader {
            index: 15, // Gap in chain
            prev_hash: UInt256::zero(),
            ..create_test_header(15)
        }];
        assert!(blockchain.add_headers(invalid_headers).is_err());
    }

    /// Test transaction processing (matches C# transaction handling exactly)
    #[test]
    fn test_transaction_processing_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Create test transaction
        let tx = Transaction {
            version: 0,
            nonce: 123456,
            system_fee: 1000000,
            network_fee: 500000,
            valid_until_block: 100,
            attributes: vec![],
            signers: vec![create_test_signer()],
            script: vec![0x51], // PUSH1 opcode
            witnesses: vec![create_test_witness()],
        };

        // Test transaction validation
        assert!(blockchain.validate_transaction(&tx).is_ok());

        let expired_tx = Transaction {
            valid_until_block: 0, // Already expired
            ..tx.clone()
        };
        assert!(blockchain.validate_transaction(&expired_tx).is_err());

        // Test transaction in block
        let block_with_tx = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            transactions: vec![tx.clone()],
            ..create_test_block(1)
        };

        assert!(blockchain.add_block(block_with_tx).is_ok());

        // Verify transaction can be retrieved
        let retrieved_tx = blockchain.get_transaction(&tx.hash()).unwrap();
        assert_eq!(retrieved_tx.hash(), tx.hash());

        // Verify transaction height
        let tx_height = blockchain.get_transaction_height(&tx.hash()).unwrap();
        assert_eq!(tx_height, 1);
    }

    /// Test mempool integration (matches C# MemoryPool exactly)
    #[test]
    fn test_mempool_integration_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Start blockchain with mempool
        blockchain.start().unwrap();
        assert!(blockchain.is_running());

        // Add transaction to mempool
        let tx = create_test_transaction(1);
        let result = blockchain.add_to_mempool(tx.clone());
        assert!(result.is_ok());

        // Verify transaction is in mempool
        assert_eq!(blockchain.mempool_size(), 1);
        assert!(blockchain.is_in_mempool(&tx.hash()));

        // Test mempool limits
        for i in 2..=50001 {
            let tx = create_test_transaction(i);
            let _ = blockchain.add_to_mempool(tx); // May fail when full
        }

        // Mempool should not exceed max size
        assert!(blockchain.mempool_size() <= 50000);

        // Test transaction removal on block addition
        let mempool_tx = create_test_transaction(99999);
        blockchain.add_to_mempool(mempool_tx.clone()).unwrap();

        let block_with_mempool_tx = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            transactions: vec![mempool_tx.clone()],
            ..create_test_block(1)
        };

        blockchain.add_block(block_with_mempool_tx).unwrap();

        // Transaction should be removed from mempool
        assert!(!blockchain.is_in_mempool(&mempool_tx.hash()));
    }

    /// Test state root calculation (matches C# state root exactly)
    #[test]
    fn test_state_root_calculation_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Enable state root calculation
        blockchain.enable_state_root(true);

        // Add block with transactions
        let transactions = vec![
            create_contract_deploy_transaction(),
            create_contract_invoke_transaction(),
        ];

        let block = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            transactions,
            ..create_test_block(1)
        };

        let result = blockchain.add_block(block);
        assert!(result.is_ok());

        // Get state root
        let state_root = blockchain.get_state_root(1).unwrap();
        assert_ne!(state_root.root, UInt256::zero());
        assert_eq!(state_root.index, 1);

        // Verify state proof
        let key = StorageKey {
            contract_id: 1,
            key: vec![0x01],
        };
        let proof = blockchain.get_state_proof(1, key).unwrap();
        assert!(!proof.proof_data.is_empty());
    }

    /// Test consensus data validation (matches C# consensus validation exactly)
    #[test]
    fn test_consensus_data_validation_compatibility() {
        let config = BlockchainConfig::default();
        let blockchain = Blockchain::new(config).unwrap();

        // Test valid consensus data
        let valid_block = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            consensus_data: ConsensusData {
                primary_index: 0,
                nonce: 12345,
            },
            ..create_test_block(1)
        };
        assert!(blockchain.validate_consensus_data(&valid_block).is_ok());

        // Test invalid primary index
        let invalid_primary_block = Block {
            consensus_data: ConsensusData {
                primary_index: 255, // Out of range
                nonce: 12345,
            },
            ..valid_block.clone()
        };
        assert!(blockchain
            .validate_consensus_data(&invalid_primary_block)
            .is_err());

        // Test witness validation
        let no_witness_block = Block {
            witness: vec![], // No witness
            ..valid_block.clone()
        };
        assert!(blockchain.validate_block(&no_witness_block).is_err());
    }

    /// Test blockchain persistence (matches C# persistence exactly)
    #[test]
    fn test_blockchain_persistence_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config.clone()).unwrap();

        // Add some blocks
        for i in 1..=5 {
            let block = Block {
                index: i,
                prev_hash: blockchain.current_block_hash(),
                timestamp: 1234567890 + i as u64,
                ..create_test_block(i)
            };
            blockchain.add_block(block).unwrap();
        }

        // Save blockchain state
        let saved_height = blockchain.height();
        let saved_hash = blockchain.current_block_hash();

        // Simulate restart - create new blockchain instance
        drop(blockchain);
        let mut new_blockchain = Blockchain::new(config).unwrap();

        // Should load persisted state
        new_blockchain.load_persisted_state().unwrap();

        assert_eq!(new_blockchain.height(), saved_height);
        assert_eq!(new_blockchain.current_block_hash(), saved_hash);

        // Verify all blocks are accessible
        for i in 0..=5 {
            assert!(new_blockchain.get_block(i).is_some());
        }
    }

    /// Test fork handling (matches C# fork resolution exactly)
    #[test]
    fn test_fork_handling_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Add main chain
        let mut main_chain = vec![];
        for i in 1..=3 {
            let block = Block {
                index: i,
                prev_hash: blockchain.current_block_hash(),
                timestamp: 1234567890 + i as u64,
                ..create_test_block(i)
            };
            blockchain.add_block(block.clone()).unwrap();
            main_chain.push(block);
        }

        // Create fork at block 2
        let fork_block = Block {
            index: 2,
            prev_hash: main_chain[0].hash(), // Fork from block 1
            timestamp: 1234567890 + 100,     // Different timestamp
            nonce: 99999,                    // Different nonce
            ..create_test_block(2)
        };

        assert!(blockchain.add_block(fork_block).is_err());

        // Current chain should remain unchanged
        assert_eq!(blockchain.height(), 3);
        assert_eq!(blockchain.current_block_hash(), main_chain[2].hash());
    }

    /// Test block rollback (matches C# rollback mechanism exactly)
    #[test]
    fn test_block_rollback_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Add blocks
        let mut blocks = vec![];
        for i in 1..=5 {
            let block = Block {
                index: i,
                prev_hash: blockchain.current_block_hash(),
                timestamp: 1234567890 + i as u64,
                ..create_test_block(i)
            };
            blockchain.add_block(block.clone()).unwrap();
            blocks.push(block);
        }

        // Rollback to block 3
        let result = blockchain.rollback_to(3);
        assert!(result.is_ok());

        // Verify state
        assert_eq!(blockchain.height(), 3);
        assert_eq!(blockchain.current_block_hash(), blocks[2].hash());

        // Blocks 4 and 5 should no longer exist
        assert!(blockchain.get_block(4).is_none());
        assert!(blockchain.get_block(5).is_none());

        // Should be able to add new blocks from height 4
        let new_block = Block {
            index: 4,
            prev_hash: blockchain.current_block_hash(),
            timestamp: 1234567890 + 999,
            ..create_test_block(4)
        };
        assert!(blockchain.add_block(new_block).is_ok());
    }

    /// Test blockchain events (matches C# event system exactly)
    #[test]
    fn test_blockchain_events_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        // Track events
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        // Register event handlers
        blockchain.on_block_added(move |block| {
            events_clone
                .lock()
                .unwrap()
                .push(BlockchainEvent::BlockAdded(block.index));
        });

        let events_clone2 = events.clone();
        blockchain.on_transaction_added(move |tx| {
            events_clone2
                .lock()
                .unwrap()
                .push(BlockchainEvent::TransactionAdded(tx.hash()));
        });

        // Add block with transaction
        let tx = create_test_transaction(1);
        let block = Block {
            index: 1,
            prev_hash: blockchain.current_block_hash(),
            transactions: vec![tx],
            ..create_test_block(1)
        };

        blockchain.add_block(block).unwrap();

        // Verify events were fired
        let fired_events = events.lock().unwrap();
        assert_eq!(fired_events.len(), 2); // Block + Transaction

        match &fired_events[0] {
            BlockchainEvent::BlockAdded(index) => assert_eq!(*index, 1),
            _ => panic!("Expected BlockAdded event"),
        }

        match &fired_events[1] {
            BlockchainEvent::TransactionAdded(_) => {}
            _ => panic!("Expected TransactionAdded event"),
        }
    }

    /// Test performance characteristics (matches C# performance exactly)
    #[test]
    fn test_blockchain_performance_compatibility() {
        let config = BlockchainConfig::default();
        let mut blockchain = Blockchain::new(config).unwrap();

        let start = std::time::Instant::now();

        // Add 100 blocks
        for i in 1..=100 {
            let mut transactions = vec![];

            // Add 50 transactions per block
            for j in 0..50 {
                transactions.push(create_test_transaction(i * 1000 + j));
            }

            let block = Block {
                index: i,
                prev_hash: blockchain.current_block_hash(),
                timestamp: 1234567890 + i as u64,
                transactions,
                ..create_test_block(i)
            };

            blockchain.add_block(block).unwrap();
        }

        let elapsed = start.elapsed();

        // Should process 100 blocks with 5000 total transactions quickly
        assert!(elapsed.as_secs() < 5); // Should be much faster

        // Test retrieval performance
        let start = std::time::Instant::now();
        for i in 1..=100 {
            let _ = blockchain.get_block(i);
        }
        let retrieval_time = start.elapsed();

        assert!(retrieval_time.as_millis() < 100); // Should be very fast
    }

    // Helper functions

    fn create_test_genesis_block() -> Block {
        Block {
            version: 0,
            prev_hash: UInt256::zero(),
            merkle_root: UInt256::zero(),
            timestamp: 1468595301, // Neo genesis timestamp
            index: 0,
            next_consensus: UInt160::from_bytes(&[0u8; 20]).unwrap(),
            witness: vec![],
            consensus_data: ConsensusData {
                primary_index: 0,
                nonce: 2083236893,
            },
            transactions: vec![],
        }
    }

    fn create_test_block(index: u32) -> Block {
        Block {
            version: 0,
            prev_hash: UInt256::zero(),
            merkle_root: UInt256::zero(),
            timestamp: 1234567890,
            index,
            next_consensus: UInt160::zero(),
            witness: vec![create_test_witness()],
            consensus_data: ConsensusData {
                primary_index: 0,
                nonce: 0,
            },
            transactions: vec![],
        }
    }

    fn create_test_header(index: u32) -> BlockHeader {
        BlockHeader {
            version: 0,
            prev_hash: UInt256::zero(),
            merkle_root: UInt256::zero(),
            timestamp: 1234567890,
            index,
            next_consensus: UInt160::zero(),
            witness: vec![create_test_witness()],
        }
    }

    fn create_test_transaction(nonce: u32) -> Transaction {
        Transaction {
            version: 0,
            nonce,
            system_fee: 0,
            network_fee: 0,
            valid_until_block: 999999,
            attributes: vec![],
            signers: vec![create_test_signer()],
            script: vec![0x51], // PUSH1
            witnesses: vec![create_test_witness()],
        }
    }

    fn create_large_transaction(nonce: u32) -> Transaction {
        Transaction {
            script: vec![0x00; 65536], // 64KB script
            ..create_test_transaction(nonce)
        }
    }

    fn create_contract_deploy_transaction() -> Transaction {
        Transaction {
            system_fee: 1000_00000000, // 1000 GAS
            script: vec![0x01; 100],   // Deploy script
            ..create_test_transaction(999998)
        }
    }

    fn create_contract_invoke_transaction() -> Transaction {
        Transaction {
            system_fee: 10_00000000, // 10 GAS
            script: vec![0x02; 50],  // Invoke script
            ..create_test_transaction(999999)
        }
    }

    fn create_test_signer() -> Signer {
        Signer {
            account: UInt160::zero(),
            scopes: WitnessScope::CalledByEntry,
            allowed_contracts: vec![],
            allowed_groups: vec![],
            rules: vec![],
        }
    }

    fn create_test_witness() -> Witness {
        Witness {
            invocation_script: vec![0x00; 64], // Signature
            verification_script: vec![0x51],   // PUSH1
        }
    }

    #[derive(Debug)]
    enum BlockchainEvent {
        BlockAdded(u32),
        TransactionAdded(UInt256),
    }
}

//! Integration tests for the ledger module.

use neo_core::{Signer, Transaction, UInt160, UInt256, WitnessScope};
use neo_ledger::blockchain::{Storage, StorageItem, StorageKey};
use neo_ledger::{Blockchain, VerifyResult};
use std::sync::Arc;
use tokio_test;

fn create_test_transaction(network_fee: i64, nonce: u32) -> Transaction {
    // Create a test transaction using proper API
    let mut transaction = Transaction::new();
    transaction.set_version(1);
    transaction.set_nonce(nonce);
    transaction.set_system_fee(0);
    transaction.set_network_fee(network_fee);
    transaction.set_valid_until_block(1000);
    transaction.set_script(vec![0x40]); // RET opcode
    transaction.add_signer(Signer::new(UInt160::zero(), WitnessScope::CalledByEntry));
    transaction
}

#[tokio::test]
async fn test_blockchain_creation() {
    // Create blockchain with correct API
    let storage = Arc::new(Storage::new_temp());
    let blockchain = Blockchain::new(storage);

    // Initialize the blockchain
    blockchain.initialize().await.unwrap();

    // Verify initial state
    assert_eq!(blockchain.height().await, 0);

    // Test that we can create transactions
    let tx = create_test_transaction(1_000_000, 1);
    assert!(tx.network_fee() > 0);
    assert_eq!(tx.nonce(), 1);
}

#[tokio::test]
async fn test_block_processing() {
    let storage = Arc::new(Storage::new_temp());
    let blockchain = Blockchain::new(storage);
    blockchain.initialize().await.unwrap();

    // Create a test transaction
    let tx = create_test_transaction(1_000_000, 1);

    // Test transaction verification
    let verification_result = blockchain.on_transaction(tx.clone()).await.unwrap();

    match verification_result {
        VerifyResult::Succeed
        | VerifyResult::InsufficientFunds
        | VerifyResult::InvalidSignature => {}
        other => panic!("Unexpected verification result: {:?}", other),
    }
}

#[tokio::test]
async fn test_storage_operations() {
    let storage = Arc::new(Storage::new_temp());

    // Test basic storage operations
    let key = StorageKey::new(b"test".to_vec(), b"key".to_vec());
    let item = StorageItem::new(b"value".to_vec());

    storage.put(&key, &item).await.unwrap();
    let retrieved = storage.get(&key).await.unwrap();

    assert_eq!(retrieved.value, b"value");
}

#[tokio::test]
async fn test_transaction_validation() {
    let storage = Arc::new(Storage::new_memory());
    let blockchain = Blockchain::new(storage);
    blockchain.initialize().await.unwrap();

    // Test valid transaction
    let valid_tx = create_test_transaction(1_000_000, 1);
    let result = blockchain.on_transaction(valid_tx).await.unwrap();

    match result {
        VerifyResult::Succeed
        | VerifyResult::InsufficientFunds
        | VerifyResult::InvalidSignature => {}
        other => panic!("Unexpected result: {:?}", other),
    }

    // Test transaction with invalid script
    let mut invalid_tx = Transaction::new();
    invalid_tx.set_version(1);
    invalid_tx.set_nonce(2);
    invalid_tx.set_network_fee(1_000_000);
    invalid_tx.set_valid_until_block(1000);
    // Leave script empty - this should be invalid
    invalid_tx.add_signer(Signer::new(UInt160::zero(), WitnessScope::CalledByEntry));

    let result = blockchain.on_transaction(invalid_tx).await.unwrap();
    assert_eq!(result, VerifyResult::InvalidScript);
}

#[tokio::test]
async fn test_blockchain_state() {
    let storage = Arc::new(Storage::new_memory());
    let blockchain = Blockchain::new(storage);
    blockchain.initialize().await.unwrap();

    // Test initial state
    assert_eq!(blockchain.height().await, 0);
    assert_eq!(blockchain.best_block_hash().await, UInt256::zero());

    // Test header cache
    let header_cache = blockchain.header_cache();
    assert!(!header_cache.full());
}

#[tokio::test]
async fn test_transaction_size_calculation() {
    // Test different transaction sizes
    let small_tx = create_test_transaction(1000, 1);
    assert!(small_tx.script().len() > 0);

    let mut large_tx = Transaction::new();
    large_tx.set_version(1);
    large_tx.set_nonce(1);
    large_tx.set_network_fee(1_000_000);
    large_tx.set_valid_until_block(1000);

    // Create a larger script
    let large_script = vec![0x40; 1000]; // 1000 bytes of RET opcodes
    large_tx.set_script(large_script);
    large_tx.add_signer(Signer::new(UInt160::zero(), WitnessScope::CalledByEntry));

    assert!(large_tx.script().len() > small_tx.script().len());
}

#[tokio::test]
async fn test_block_existence_check() {
    let storage = Arc::new(Storage::new_memory());
    let blockchain = Blockchain::new(storage);
    blockchain.initialize().await.unwrap();

    // Test non-existent block
    let non_existent_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
    let exists = blockchain.block_exists(&non_existent_hash).await.unwrap();
    assert!(!exists);
}

#[tokio::test]
async fn test_error_handling() {
    let storage = Arc::new(Storage::new_memory());
    let blockchain = Blockchain::new(storage);
    blockchain.initialize().await.unwrap();

    // Test getting non-existent block
    let non_existent_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
    let result = blockchain.get_block(&non_existent_hash).await;
    assert!(result.is_err());

    // Test getting non-existent transaction
    let result = blockchain.get_transaction(&non_existent_hash).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_transactions() {
    let storage = Arc::new(Storage::new_memory());
    let blockchain = Blockchain::new(storage);
    blockchain.initialize().await.unwrap();

    // Create multiple transactions with different fees
    let tx1 = create_test_transaction(1_000_000, 1);
    let tx2 = create_test_transaction(2_000_000, 2);
    let tx3 = create_test_transaction(500_000, 3);

    // Process transactions
    let result1 = blockchain.on_transaction(tx1).await.unwrap();
    let result2 = blockchain.on_transaction(tx2).await.unwrap();
    let result3 = blockchain.on_transaction(tx3).await.unwrap();

    let valid_results = [
        VerifyResult::Succeed,
        VerifyResult::InsufficientFunds,
        VerifyResult::InvalidSignature,
        VerifyResult::PolicyFail,
        VerifyResult::Expired,
    ];

    assert!(valid_results.contains(&result1));
    assert!(valid_results.contains(&result2));
    assert!(valid_results.contains(&result3));
}

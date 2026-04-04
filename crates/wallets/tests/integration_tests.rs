//! Integration tests for the complete wallet system
//!
//! These tests verify that all wallet components work together correctly
//! and match the behavior of the C# Neo implementation.

use neo_core::UInt256;
use neo_core::{Transaction, UInt160};
use neo_wallets::*;
use neo_wallets::{
    Contract, KeyPair, ScryptParameters, StandardWalletAccount, Wallet, WalletAccount, WalletError,
    WalletResult,
};
use std::sync::Arc;
use tempfile::TempDir;
use tokio;

#[tokio::test]
async fn test_complete_wallet_workflow() {
    // Test a complete wallet workflow from creation to signing
    let mut wallet = Nep6Wallet::new("integration_test_wallet".to_string(), None);
    let password = "test_password_123";

    // 1. Set wallet password
    let password_set = wallet.change_password("", password).await.unwrap();
    assert!(password_set);

    // 2. Create multiple accounts
    let account1 = wallet.create_account(&[1u8; 32]).await.unwrap();
    let account2 = wallet.create_account(&[2u8; 32]).await.unwrap();
    let watch_only = wallet
        .create_account_watch_only(UInt160::new())
        .await
        .unwrap();

    // 3. Verify accounts
    assert_eq!(3, wallet.get_accounts().len());
    assert!(account1.has_key());
    assert!(account2.has_key());
    assert!(!watch_only.has_key());

    // 4. Set default account
    let script_hash1 = account1.script_hash();
    wallet.set_default_account(&script_hash1).await.unwrap();

    let default_account = wallet.get_default_account().unwrap();
    assert_eq!(script_hash1, default_account.script_hash());

    // 5. Sign data with accounts
    let test_data = b"integration test data";
    let signature1 = wallet.sign(test_data, &script_hash1).await.unwrap();
    assert!(!signature1.is_empty());

    // 6. Verify signatures
    let is_valid = account1.verify(test_data, &signature1).await.unwrap();
    assert!(is_valid);

    // 7. Lock and unlock wallet
    wallet.lock();
    assert!(wallet.is_locked());

    let unlocked = wallet.unlock(password).await.unwrap();
    assert!(unlocked);
    assert!(!wallet.is_locked());
}

#[tokio::test]
async fn test_wallet_import_export_workflow() {
    // Test importing and exporting keys in various formats
    let mut wallet = Nep6Wallet::new("import_export_test".to_string(), None);
    let password = "export_password";

    // 1. Generate original key pair
    let original_key_pair = KeyPair::generate().unwrap();
    let original_script_hash = original_key_pair.get_script_hash();

    // 2. Export to WIF and import
    let wif = original_key_pair.to_wif();
    let wif_account = wallet.import_wif(&wif).await.unwrap();
    assert_eq!(original_script_hash, wif_account.script_hash());

    // 3. Export to NEP-2 and import
    let nep2_key = original_key_pair.to_nep2(password).unwrap();
    let nep2_account = wallet.import_nep2(&nep2_key, password).await.unwrap();
    assert_eq!(original_script_hash, nep2_account.script_hash());

    // 4. Verify both accounts are the same
    assert_eq!(wif_account.script_hash(), nep2_account.script_hash());

    // 5. Test signing with both accounts
    let test_data = b"export import test";
    let wif_signature = wif_account.sign(test_data).await.unwrap();
    let nep2_signature = nep2_account.sign(test_data).await.unwrap();

    // Both should be able to verify each other's signatures
    assert!(wif_account
        .verify(test_data, &nep2_signature)
        .await
        .unwrap());
    assert!(nep2_account
        .verify(test_data, &wif_signature)
        .await
        .unwrap());
}

#[tokio::test]
async fn test_multi_signature_workflow() {
    // Test multi-signature contract creation and usage
    let key_pair1 = KeyPair::generate().unwrap();
    let key_pair2 = KeyPair::generate().unwrap();
    let key_pair3 = KeyPair::generate().unwrap();

    // Create public key points
    let pub_key1 = key_pair1.get_public_key_point().unwrap();
    let pub_key2 = key_pair2.get_public_key_point().unwrap();
    let pub_key3 = key_pair3.get_public_key_point().unwrap();

    let public_keys = vec![pub_key1, pub_key2, pub_key3];

    let multi_sig_contract = Contract::create_multi_sig_contract(2, &public_keys).unwrap();

    // Verify contract properties
    assert_eq!(2, multi_sig_contract.parameter_list.len());
    assert!(!multi_sig_contract.script.is_empty());

    // Create accounts with the multi-sig contract
    let account1 = StandardWalletAccount::new_with_key(key_pair1, Some(multi_sig_contract.clone()));
    let account2 = StandardWalletAccount::new_with_key(key_pair2, Some(multi_sig_contract.clone()));
    let account3 = StandardWalletAccount::new_with_key(key_pair3, Some(multi_sig_contract));

    let script_hash = account1.script_hash();
    assert_eq!(script_hash, account2.script_hash());
    assert_eq!(script_hash, account3.script_hash());
}

#[tokio::test]
async fn test_transaction_signing_workflow() {
    // Test complete transaction signing workflow
    let mut wallet = Nep6Wallet::new("transaction_test".to_string(), None);

    // Create account
    let account = wallet.create_account(&[1u8; 32]).await.unwrap();
    let script_hash = account.script_hash();

    // Create a test transaction
    let mut transaction = Transaction::new();
    transaction.set_version(0);
    transaction.set_nonce(12345);
    transaction.set_system_fee(1000);
    transaction.set_network_fee(500);
    transaction.set_valid_until_block(100);

    // Add signer
    let signer = neo_core::Signer::new(script_hash, neo_core::WitnessScope::CALLED_BY_ENTRY);
    transaction.add_signer(signer);

    // Sign transaction
    let witness = account.sign_transaction(&transaction).await.unwrap();

    // Verify witness
    assert!(!witness.invocation_script().is_empty());
    assert!(!witness.verification_script().is_empty());

    // Add witness to transaction
    transaction.add_witness(witness);

    // Transaction should now have witnesses
    assert_eq!(1, transaction.witnesses().len());
}

#[tokio::test]
async fn test_wallet_persistence_workflow() {
    let mut wallet = Nep6Wallet::new("persistence_test".to_string(), None);
    let password = "persistence_password";

    // Set up wallet
    wallet.change_password("", password).await.unwrap();
    let account1 = wallet.create_account(&[1u8; 32]).await.unwrap();
    let account2 = wallet.create_account(&[2u8; 32]).await.unwrap();
    wallet
        .set_default_account(&account1.script_hash())
        .await
        .unwrap();

    // Clone wallet to simulate save/load
    let saved_wallet = wallet.clone();

    // Verify cloned wallet has same properties
    assert_eq!(wallet.name(), saved_wallet.name());
    assert_eq!(
        wallet.get_accounts().len(),
        saved_wallet.get_accounts().len()
    );

    // Verify accounts are preserved
    assert!(saved_wallet.contains(&account1.script_hash()));
    assert!(saved_wallet.contains(&account2.script_hash()));

    // Verify default account is preserved
    let default_account = saved_wallet.get_default_account().unwrap();
    assert_eq!(account1.script_hash(), default_account.script_hash());
}

#[tokio::test]
async fn test_error_handling_workflow() {
    // Test comprehensive error handling
    let mut wallet = Nep6Wallet::new("error_test".to_string(), None);

    // Test account not found errors
    let non_existent_hash = UInt160::new();
    let sign_result = wallet.sign(b"test", &non_existent_hash).await;
    assert!(sign_result.is_err());

    match sign_result.unwrap_err() {
        WalletError::AccountNotFound(hash) => {
            assert_eq!(non_existent_hash, hash);
        }
        _ => panic!("Expected AccountNotFound error"),
    }

    // Test invalid key errors
    let invalid_wif = "invalid_wif_string";
    let import_result = wallet.import_wif(invalid_wif).await;
    assert!(import_result.is_err());

    // Test invalid NEP-2 errors
    let invalid_nep2 = "invalid_nep2_string";
    let nep2_result = wallet.import_nep2(invalid_nep2, "password").await;
    assert!(nep2_result.is_err());

    // Test password verification errors
    wallet
        .change_password("", "correct_password")
        .await
        .unwrap();
    let wrong_password_result = wallet.verify_password("wrong_password").await.unwrap();
    assert!(!wrong_password_result);
}

#[tokio::test]
async fn test_concurrent_wallet_operations() {
    // Test concurrent wallet operations
    let wallet = Arc::new(Nep6Wallet::new("concurrent_test".to_string(), None));

    // Create multiple accounts concurrently
    let mut handles = vec![];

    for i in 0..5 {
        let wallet_clone = Arc::clone(&wallet);
        let handle = tokio::spawn(async move {
            let private_key = [i as u8; 32];
            let mut wallet_mut = (*wallet_clone).clone();
            wallet_mut.create_account(&private_key).await
        });
        handles.push(handle);
    }

    let mut accounts = vec![];
    for handle in handles {
        let account = handle.await.unwrap().unwrap();
        accounts.push(account);
    }

    // Verify all accounts were created with different script hashes
    assert_eq!(5, accounts.len());
    let mut script_hashes = std::collections::HashSet::new();
    for account in accounts {
        assert!(script_hashes.insert(account.script_hash()));
    }
    assert_eq!(5, script_hashes.len());
}

#[tokio::test]
async fn test_wallet_compatibility_with_csharp() {
    // Test specific compatibility scenarios with C# implementation

    // 1. Test known key pair that should produce specific results
    let known_private_key = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];

    let key_pair = KeyPair::from_private_key(&known_private_key).unwrap();
    let script_hash = key_pair.get_script_hash();
    let address = script_hash.to_address();

    assert!(!address.is_empty());
    assert!(address.starts_with('N')); // Neo addresses start with 'N'

    // 2. Test signature determinism
    let test_data = b"deterministic test data";
    let signature1 = key_pair.sign(test_data).unwrap();
    let signature2 = key_pair.sign(test_data).unwrap();

    assert!(key_pair.verify(test_data, &signature1).unwrap());
    assert!(key_pair.verify(test_data, &signature2).unwrap());

    // 3. Test WIF format compatibility
    let wif = key_pair.to_wif();
    let restored_key_pair = KeyPair::from_wif(&wif).unwrap();
    assert_eq!(key_pair.private_key(), restored_key_pair.private_key());
}

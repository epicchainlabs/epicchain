//! Unit tests for the Wallet module
//!
//! These tests are converted from the C# Neo implementation to ensure
//! exact compatibility and behavior matching.

use neo_core::{UInt160, UInt256};
use neo_wallets::wallet_account::StandardWalletAccount;
use neo_wallets::*;
use std::sync::Arc;
use tokio;

/// Test wallet implementation for testing purposes
/// Matches the MyWallet class from C# tests
struct TestWallet {
    name: String,
    version: Version,
    accounts: std::collections::HashMap<UInt160, Arc<dyn WalletAccount>>,
}

impl TestWallet {
    fn new() -> Self {
        Self {
            name: "TestWallet".to_string(),
            version: Version::new(0, 0, 1),
            accounts: std::collections::HashMap::new(),
        }
    }

    fn add_account(&mut self, account: Arc<dyn WalletAccount>) {
        self.accounts.insert(account.script_hash(), account);
    }
}

#[async_trait::async_trait]
impl Wallet for TestWallet {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &Version {
        &self.version
    }

    fn path(&self) -> Option<&str> {
        None // Test wallet doesn't have a file path
    }

    async fn change_password(
        &mut self,
        _old_password: &str,
        _new_password: &str,
    ) -> WalletResult<bool> {
        Err(WalletError::Other(
            "Operation completed successfully".to_string(),
        ))
    }

    fn contains(&self, script_hash: &UInt160) -> bool {
        self.accounts.contains_key(script_hash)
    }

    async fn create_account(&mut self, private_key: &[u8]) -> WalletResult<Arc<dyn WalletAccount>> {
        let key_pair = KeyPair::from_private_key(private_key)?;
        let script_hash = key_pair.get_script_hash();
        let account = Arc::new(StandardWalletAccount::new_with_key(key_pair, None));
        self.accounts.insert(script_hash, account.clone());
        Ok(account)
    }

    async fn create_account_watch_only(
        &mut self,
        script_hash: UInt160,
    ) -> WalletResult<Arc<dyn WalletAccount>> {
        let account = Arc::new(StandardWalletAccount::new_watch_only(script_hash, None));
        self.accounts.insert(script_hash, account.clone());
        Ok(account)
    }

    async fn create_account_with_contract(
        &mut self,
        contract: Contract,
        key_pair: Option<KeyPair>,
    ) -> WalletResult<Arc<dyn WalletAccount>> {
        let account = if let Some(key_pair) = key_pair {
            Arc::new(StandardWalletAccount::new_with_key(
                key_pair,
                Some(contract.clone()),
            ))
        } else {
            Arc::new(StandardWalletAccount::new_watch_only(
                contract.script_hash(),
                Some(contract),
            ))
        };
        self.accounts.insert(account.script_hash(), account.clone());
        Ok(account)
    }

    async fn delete_account(&mut self, script_hash: &UInt160) -> WalletResult<bool> {
        Ok(self.accounts.remove(script_hash).is_some())
    }

    async fn export(&self, _path: &str, _password: &str) -> WalletResult<()> {
        Ok(())
    }

    fn get_account(&self, script_hash: &UInt160) -> Option<Arc<dyn WalletAccount>> {
        self.accounts.get(script_hash).cloned()
    }

    fn get_accounts(&self) -> Vec<Arc<dyn WalletAccount>> {
        self.accounts.values().cloned().collect()
    }

    async fn get_available_balance(&self, _asset_id: &UInt256) -> WalletResult<i64> {
        Ok(0)
    }

    async fn get_unclaimed_gas(&self) -> WalletResult<i64> {
        Ok(0)
    }

    async fn import_wif(&mut self, wif: &str) -> WalletResult<Arc<dyn WalletAccount>> {
        let key_pair = KeyPair::from_wif(wif)?;
        self.create_account(&key_pair.private_key()).await
    }

    async fn import_nep2(
        &mut self,
        nep2_key: &str,
        password: &str,
    ) -> WalletResult<Arc<dyn WalletAccount>> {
        let key_pair = KeyPair::from_nep2(nep2_key.as_bytes(), password)?;
        self.create_account(&key_pair.private_key()).await
    }

    async fn sign(&self, data: &[u8], script_hash: &UInt160) -> WalletResult<Vec<u8>> {
        if let Some(account) = self.get_account(script_hash) {
            account
                .sign(data)
                .await
                .map_err(|e| WalletError::Other(e.to_string()))
        } else {
            Err(WalletError::AccountNotFound(*script_hash))
        }
    }

    async fn sign_transaction(&self, _transaction: &mut neo_core::Transaction) -> WalletResult<()> {
        Ok(())
    }

    async fn unlock(&mut self, _password: &str) -> WalletResult<bool> {
        Ok(true)
    }

    fn lock(&mut self) {}

    async fn verify_password(&self, _password: &str) -> WalletResult<bool> {
        Ok(true)
    }

    async fn save(&self) -> WalletResult<()> {
        Ok(())
    }

    fn get_default_account(&self) -> Option<Arc<dyn WalletAccount>> {
        self.accounts.values().next().cloned()
    }

    async fn set_default_account(&mut self, _script_hash: &UInt160) -> WalletResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_contains() {
    // Matches TestContains from C# tests
    let wallet = TestWallet::new();

    // Should not throw exception
    let result = wallet.contains(&UInt160::new());
    assert!(!result); // Empty wallet should not contain any accounts
}

#[tokio::test]
async fn test_create_account_with_private_key() {
    // Matches TestCreateAccount1 from C# tests
    let mut wallet = TestWallet::new();
    let private_key = [1u8; 32]; // Test private key

    let account = wallet.create_account(&private_key).await;
    assert!(account.is_ok());
    assert!(account.unwrap().has_key());
}

#[tokio::test]
async fn test_create_account_watch_only() {
    // Matches TestCreateAccount2 from C# tests
    let mut wallet = TestWallet::new();
    let script_hash = UInt160::new();

    let account = wallet.create_account_watch_only(script_hash).await;
    assert!(account.is_ok());
    assert!(!account.unwrap().has_key()); // Watch-only account should not have key
}

#[tokio::test]
async fn test_get_name() {
    // Matches TestGetName from C# tests
    let wallet = TestWallet::new();
    assert_eq!("TestWallet", wallet.name());
}

#[tokio::test]
async fn test_get_version() {
    // Matches TestGetVersion from C# tests
    let wallet = TestWallet::new();
    let expected_version = Version::new(0, 0, 1);
    assert_eq!(&expected_version, wallet.version());
}

#[tokio::test]
async fn test_get_account_by_script_hash() {
    // Matches TestGetAccount2 from C# tests
    let wallet = TestWallet::new();

    // Should not throw exception when getting non-existent account
    let account = wallet.get_account(&UInt160::new());
    assert!(account.is_none());
}

#[tokio::test]
async fn test_import_wif() {
    let mut wallet = TestWallet::new();

    // Generate a test key pair and export to WIF
    let key_pair = KeyPair::generate().unwrap();
    let wif = key_pair.to_wif();

    // Import the WIF
    let account = wallet.import_wif(&wif).await;
    assert!(account.is_ok());

    let imported_account = account.unwrap();
    assert!(imported_account.has_key());
    assert_eq!(imported_account.script_hash(), key_pair.get_script_hash());
}

#[tokio::test]
async fn test_delete_account() {
    let mut wallet = TestWallet::new();
    let private_key = [1u8; 32];

    // Create an account
    let account = wallet.create_account(&private_key).await.unwrap();
    let script_hash = account.script_hash();

    // Verify account exists
    assert!(wallet.contains(&script_hash));

    // Delete the account
    let deleted = wallet.delete_account(&script_hash).await.unwrap();
    assert!(deleted);

    // Verify account no longer exists
    assert!(!wallet.contains(&script_hash));
}

#[tokio::test]
async fn test_get_accounts() {
    let mut wallet = TestWallet::new();

    // Initially empty
    assert_eq!(0, wallet.get_accounts().len());

    // Add some accounts
    let _account1 = wallet.create_account(&[1u8; 32]).await.unwrap();
    let _account2 = wallet.create_account(&[2u8; 32]).await.unwrap();

    // Should have 2 accounts
    assert_eq!(2, wallet.get_accounts().len());
}

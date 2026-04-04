//! Wallet account implementation.
//!
//! This module provides the wallet account trait and functionality,
//! converted from the C# Neo WalletAccount class (@neo-sharp/src/Neo/Wallets/WalletAccount.cs).

use crate::{contract::Contract, key_pair::KeyPair, Error, Result};
use async_trait::async_trait;
use neo_core::{Signer, Transaction, UInt160, UInt256, Witness};
use std::sync::Arc;

/// The base trait for wallet accounts.
/// This matches the C# WalletAccount abstract class.
#[async_trait]
pub trait WalletAccount: Send + Sync {
    /// The script hash of the account.
    fn script_hash(&self) -> UInt160;

    /// The address of the account.
    fn address(&self) -> String;

    /// The label of the account.
    fn label(&self) -> Option<&str>;

    /// Sets the label of the account.
    fn set_label(&mut self, label: Option<String>);

    /// Indicates whether the account has a key.
    fn has_key(&self) -> bool;

    /// Gets the key pair of the account.
    fn get_key(&self) -> Option<KeyPair>;

    /// Gets the contract of the account.
    fn get_contract(&self) -> Option<&Contract>;

    /// Indicates whether the account is locked.
    fn is_locked(&self) -> bool;

    /// Locks the account.
    fn lock(&mut self);

    /// Unlocks the account with the specified password.
    async fn unlock(&mut self, password: &str) -> Result<bool>;

    /// Signs the specified data.
    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;

    /// Signs the specified transaction.
    async fn sign_transaction(&self, transaction: &Transaction) -> Result<Witness>;

    /// Verifies the specified password.
    async fn verify_password(&self, password: &str) -> Result<bool>;

    /// Exports the account to WIF format.
    async fn export_wif(&self) -> Result<String>;

    /// Exports the account to NEP-2 format.
    async fn export_nep2(&self, password: &str) -> Result<String>;

    /// Verifies a signature against data.
    async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool>;

    /// Indicates whether this is a watch-only account.
    fn is_watch_only(&self) -> bool {
        !self.has_key()
    }

    /// Indicates whether this is a multi-signature account.
    fn is_multi_sig(&self) -> bool {
        if let Some(contract) = self.get_contract() {
            contract.is_multi_sig()
        } else {
            false
        }
    }

    /// Gets the public key of the account (if available).
    fn get_public_key(&self) -> Option<Vec<u8>> {
        self.get_key().map(|key| key.public_key())
    }
}

/// A concrete implementation of WalletAccount.
/// This matches the C# WalletAccount class implementation.
#[derive(Debug, Clone)]
pub struct StandardWalletAccount {
    script_hash: UInt160,
    label: Option<String>,
    key_pair: Option<KeyPair>,
    contract: Option<Contract>,
    is_locked: bool,
    encrypted_key: Option<Vec<u8>>,
}

impl StandardWalletAccount {
    /// Creates a new wallet account with a key pair.
    pub fn new_with_key(key_pair: KeyPair, contract: Option<Contract>) -> Self {
        let script_hash = key_pair.get_script_hash();

        Self {
            script_hash,
            label: None,
            key_pair: Some(key_pair),
            contract,
            is_locked: false,
            encrypted_key: None,
        }
    }

    /// Creates a new watch-only wallet account.
    pub fn new_watch_only(script_hash: UInt160, contract: Option<Contract>) -> Self {
        Self {
            script_hash,
            label: None,
            key_pair: None,
            contract,
            is_locked: false,
            encrypted_key: None,
        }
    }

    /// Creates a new wallet account from encrypted key.
    pub fn new_from_encrypted(
        script_hash: UInt160,
        encrypted_key: Vec<u8>,
        contract: Option<Contract>,
    ) -> Self {
        Self {
            script_hash,
            label: None,
            key_pair: None,
            contract,
            is_locked: true,
            encrypted_key: Some(encrypted_key),
        }
    }

    /// Decrypts the private key with the specified password.
    async fn decrypt_key(&self, password: &str) -> Result<KeyPair> {
        if let Some(ref encrypted_key) = self.encrypted_key {
            // Decrypt the key using the password
            KeyPair::from_nep2(encrypted_key, password)
        } else {
            Err(Error::Other("No encrypted key available".to_string()))
        }
    }
}

#[async_trait]
impl WalletAccount for StandardWalletAccount {
    fn script_hash(&self) -> UInt160 {
        self.script_hash
    }

    fn address(&self) -> String {
        // Convert script hash to Neo address format
        self.script_hash.to_address()
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    fn has_key(&self) -> bool {
        self.key_pair.is_some() || self.encrypted_key.is_some()
    }

    fn get_key(&self) -> Option<KeyPair> {
        self.key_pair.clone()
    }

    fn get_contract(&self) -> Option<&Contract> {
        self.contract.as_ref()
    }

    fn is_locked(&self) -> bool {
        self.is_locked
    }

    fn lock(&mut self) {
        if self.key_pair.is_some() && self.encrypted_key.is_some() {
            // Clear the decrypted key and mark as locked
            self.key_pair = None;
            self.is_locked = true;
        }
    }

    async fn unlock(&mut self, password: &str) -> Result<bool> {
        if !self.is_locked {
            return Ok(true);
        }

        if let Some(_) = &self.encrypted_key {
            match self.decrypt_key(password).await {
                Ok(key_pair) => {
                    self.key_pair = Some(key_pair);
                    self.is_locked = false;
                    Ok(true)
                }
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        if let Some(ref key_pair) = self.key_pair {
            key_pair.sign(data)
        } else {
            Err(Error::AccountLocked)
        }
    }

    async fn sign_transaction(&self, transaction: &Transaction) -> Result<Witness> {
        if let Some(ref key_pair) = self.key_pair {
            let signature = key_pair.sign(&transaction.get_hash_data())?;

            // Create witness based on contract type
            if let Some(ref contract) = self.contract {
                contract.create_witness(signature)
            } else {
                // Standard single-signature witness
                Ok(Witness::new_with_scripts(
                    vec![0x0c, 0x40] // PUSHDATA1 64 bytes
                        .into_iter()
                        .chain(signature)
                        .collect(),
                    key_pair.get_verification_script(),
                ))
            }
        } else {
            Err(Error::AccountLocked)
        }
    }

    async fn verify_password(&self, password: &str) -> Result<bool> {
        if let Some(_) = &self.encrypted_key {
            Ok(self.decrypt_key(password).await.is_ok())
        } else {
            Ok(true) // No password protection
        }
    }

    async fn export_wif(&self) -> Result<String> {
        if let Some(ref key_pair) = self.key_pair {
            Ok(key_pair.to_wif())
        } else {
            Err(Error::AccountLocked)
        }
    }

    async fn export_nep2(&self, password: &str) -> Result<String> {
        if let Some(ref key_pair) = self.key_pair {
            key_pair.to_nep2(password)
        } else {
            Err(Error::AccountLocked)
        }
    }

    async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool> {
        if let Some(ref key_pair) = self.key_pair {
            key_pair.verify(data, signature)
        } else {
            Err(Error::AccountLocked)
        }
    }
}

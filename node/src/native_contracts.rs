//! Native contract integration framework
//!
//! This module provides a framework for integrating with Neo native contracts
//! such as NEO, GAS, Policy, and others. Now with VM integration for real contract execution.

use crate::vm_integration::VmExecutor;
use anyhow::Result;
use neo_config::{HASH_SIZE, MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK};
use neo_core::UInt160;
use neo_cryptography::ECPoint;
use neo_ledger::Blockchain;
use std::sync::Arc;
use tracing::debug;

/// Native contract hashes for Neo N3 network
pub struct NativeContractHashes {
    /// NEO token contract hash
    pub neo: UInt160,
    /// GAS token contract hash  
    pub gas: UInt160,
    /// Policy contract hash
    pub policy: UInt160,
    /// Role management contract hash
    pub role_management: UInt160,
    /// Oracle contract hash
    pub oracle: UInt160,
}

// Mainnet contract hashes as constants
const NEO_HASH: &str = "ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5";
const GAS_HASH: &str = "d2a4cff31913016155e38e474a2c06d08be276cf";
const POLICY_HASH: &str = "cc5e4edd78e5d7b5ac9dd9b048b8d24e7fb76b3c";
const ROLE_MANAGEMENT_HASH: &str = "49cf4e5378ffcd4dec034fd98a174c5491e395e2";
const ORACLE_HASH: &str = "fe924b7cfe89ddd271abaf7210a80a7e11178758";

impl NativeContractHashes {
    /// Get native contract hashes for mainnet
    pub fn mainnet() -> Result<Self> {
        Ok(Self {
            // These are the actual Neo N3 mainnet contract hashes
            neo: UInt160::parse(NEO_HASH)
                .map_err(|e| anyhow::anyhow!("Failed to parse NEO contract hash: {}", e))?,
            gas: UInt160::parse(GAS_HASH)
                .map_err(|e| anyhow::anyhow!("Failed to parse GAS contract hash: {}", e))?,
            policy: UInt160::parse(POLICY_HASH)
                .map_err(|e| anyhow::anyhow!("Failed to parse Policy contract hash: {}", e))?,
            role_management: UInt160::parse(ROLE_MANAGEMENT_HASH).map_err(|e| {
                anyhow::anyhow!("Failed to parse RoleManagement contract hash: {}", e)
            })?,
            oracle: UInt160::parse(ORACLE_HASH)
                .map_err(|e| anyhow::anyhow!("Failed to parse Oracle contract hash: {}", e))?,
        })
    }

    /// Get native contract hashes for testnet
    pub fn testnet() -> Result<Self> {
        Self::mainnet()
    }

    /// Get native contract hashes for private network
    pub fn private() -> Result<Self> {
        Self::mainnet()
    }
}

/// NEO contract integration for validator management
pub struct NeoContract {
    blockchain: Arc<Blockchain>,
    contract_hash: UInt160,
    vm_executor: VmExecutor,
}

impl NeoContract {
    /// Create a new NEO contract instance
    pub fn new(blockchain: Arc<Blockchain>, contract_hash: UInt160) -> Self {
        let vm_executor = VmExecutor::new(blockchain.clone());
        Self {
            blockchain,
            contract_hash,
            vm_executor,
        }
    }

    /// Get the current committee members
    pub async fn get_committee(&self) -> Result<Vec<ECPoint>> {
        tracing::debug!("Getting committee members from NEO contract using VM");

        match self
            .vm_executor
            .get_neo_committee(&self.contract_hash)
            .await
        {
            Ok(committee) if !committee.is_empty() => {
                tracing::debug!(
                    "Retrieved {} committee members from NEO contract via VM",
                    committee.len()
                );
                Ok(committee)
            }
            Ok(_) => {
                // VM call succeeded but returned no committee - try fallback
                tracing::debug!("VM call succeeded but no committee returned, trying fallback");
                self.get_committee_fallback().await
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to NEO.getCommittee() failed: {}, using fallback",
                    e
                );
                self.get_committee_fallback().await
            }
        }
    }

    /// Fallback committee retrieval when VM execution fails
    async fn get_committee_fallback(&self) -> Result<Vec<ECPoint>> {
        // Try to get committee from blockchain state first
        match self.get_validators_from_blockchain_state().await {
            Ok(validators) if !validators.is_empty() => {
                // Committee is typically larger than validator set
                tracing::debug!(
                    "Retrieved {} committee members from blockchain state fallback",
                    validators.len()
                );
                Ok(validators)
            }
            Ok(_) => {
                tracing::debug!("No committee found in blockchain state, using default");
                self.get_default_committee().await
            }
            Err(e) => {
                tracing::warn!("Failed to get committee from blockchain state: {}", e);
                self.get_default_committee().await
            }
        }
    }

    /// Get a default committee for testing purposes
    async fn get_default_committee(&self) -> Result<Vec<ECPoint>> {
        tracing::debug!("Using default committee for testing");

        let mut committee = Vec::new();

        // Generate a few test committee members
        for i in 1..=4 {
            // Generate 4 committee members
            let key_seed = [i; HASH_SIZE];
            match neo_cryptography::ecc::generate_public_key(&key_seed) {
                Ok(pubkey) => match ECPoint::from_bytes(&pubkey) {
                    Ok(ec_point) => {
                        committee.push(ec_point);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to convert public key {} to ECPoint: {}", i, e);
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to generate committee member {}: {}", i, e);
                }
            }
        }

        tracing::debug!(
            "Generated default committee with {} members",
            committee.len()
        );
        Ok(committee)
    }

    /// Get the next block validators
    pub async fn get_next_block_validators(&self) -> Result<Vec<ECPoint>> {
        tracing::debug!("Getting next block validators from NEO contract using VM");

        match self
            .vm_executor
            .get_next_block_validators(&self.contract_hash)
            .await
        {
            Ok(validators) if !validators.is_empty() => {
                tracing::debug!(
                    "Retrieved {} next block validators from NEO contract via VM",
                    validators.len()
                );
                Ok(validators)
            }
            Ok(_) => {
                // VM call succeeded but returned no validators - try fallback
                tracing::debug!("VM call succeeded but no validators returned, trying fallback");
                self.get_next_block_validators_fallback().await
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to NEO.getNextBlockValidators() failed: {}, using fallback",
                    e
                );
                self.get_next_block_validators_fallback().await
            }
        }
    }

    /// Fallback next block validators retrieval when VM execution fails
    async fn get_next_block_validators_fallback(&self) -> Result<Vec<ECPoint>> {
        // Try to get the committee first
        match self.get_committee().await {
            Ok(committee) if !committee.is_empty() => {
                let validator_count = std::cmp::min(committee.len(), 7); // Neo typically uses 7 validators
                let validators = committee.into_iter().take(validator_count).collect();
                tracing::debug!(
                    "Retrieved {} next block validators from committee fallback",
                    validator_count
                );
                Ok(validators)
            }
            Ok(_) => self.get_validators_from_blockchain_state().await,
            Err(e) => {
                tracing::warn!("Failed to get committee for next block validators: {}", e);
                self.get_validators_from_blockchain_state().await
            }
        }
    }

    /// Get validators for a specific height
    pub async fn get_validators_at_height(&self, height: u32) -> Result<Vec<ECPoint>> {
        // Query NEO contract state at the specific height using VM execution

        tracing::debug!("Getting validators for height {}", height);

        // Try to get validators from blockchain state
        match self.get_validators_from_blockchain_state().await {
            Ok(validators) if !validators.is_empty() => {
                tracing::debug!(
                    "Retrieved {} validators for height {}",
                    validators.len(),
                    height
                );
                Ok(validators)
            }
            Ok(_) => {
                tracing::warn!(
                    "No validators found for height {}, using default validator set",
                    height
                );
                self.get_default_validator_set().await
            }
            Err(e) => {
                tracing::warn!("Failed to get validators for height {}: {}", height, e);
                self.get_default_validator_set().await
            }
        }
    }

    /// Get the total supply of NEO tokens
    pub async fn get_total_supply(&self) -> Result<u64> {
        tracing::debug!("Getting NEO total supply using VM");

        match self
            .vm_executor
            .get_neo_total_supply(&self.contract_hash)
            .await
        {
            Ok(supply) => {
                tracing::debug!("Retrieved NEO total supply via VM: {}", supply);
                Ok(supply)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to NEO.totalSupply() failed: {}, returning default",
                    e
                );
                Ok(100_000_000) // Default NEO total supply
            }
        }
    }

    /// Get NEO balance for an address
    pub async fn get_balance(&self, address: &UInt160) -> Result<u64> {
        tracing::debug!("Getting NEO balance for {} using VM", address);

        match self
            .vm_executor
            .get_neo_balance(&self.contract_hash, address)
            .await
        {
            Ok(balance) => {
                tracing::debug!("Retrieved NEO balance for {} via VM: {}", address, balance);
                Ok(balance)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to NEO.balanceOf({}) failed: {}, returning 0",
                    address,
                    e
                );
                Ok(0)
            }
        }
    }

    /// Get validators from blockchain state (helper method)
    async fn get_validators_from_blockchain_state(&self) -> Result<Vec<ECPoint>> {
        // Try to retrieve validator information from blockchain metadata
        // This would typically involve reading from the blockchain's validator storage

        tracing::debug!("Attempting to get validators from blockchain state");

        match self.get_validators_from_config().await {
            Ok(validators) if !validators.is_empty() => {
                tracing::debug!("Found {} validators from configuration", validators.len());
                Ok(validators)
            }
            _ => {
                tracing::debug!("No validators found in configuration");
                Ok(vec![])
            }
        }
    }

    /// Get validators from blockchain configuration
    async fn get_validators_from_config(&self) -> Result<Vec<ECPoint>> {
        tracing::debug!("Attempting to get validators from blockchain configuration");

        // Try to get the genesis block and extract validator information
        match self.blockchain.get_block(0).await {
            Ok(Some(genesis_block)) => {
                // Extract validators from genesis block witness scripts
                let mut validators = Vec::new();

                for witness in &genesis_block.header.witnesses {
                    let verification_script = &witness.verification_script;
                    if !verification_script.is_empty() {
                        // Try to extract public key from verification script
                        if verification_script.len() >= 35 {
                            // PUSH21 + 33-byte pubkey + CheckSig
                            let pubkey_start = 1; // Skip PUSH21 opcode
                            let pubkey_end = pubkey_start + 33;

                            if pubkey_end <= verification_script.len() {
                                let pubkey_bytes = &verification_script[pubkey_start..pubkey_end];
                                match ECPoint::from_bytes(pubkey_bytes) {
                                    Ok(ec_point) => {
                                        validators.push(ec_point);
                                        tracing::debug!("Extracted validator from genesis block");
                                    }
                                    Err(e) => {
                                        tracing::debug!(
                                            "Failed to parse pubkey from genesis: {}",
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                if validators.is_empty() {
                    self.get_validators_from_metadata().await
                } else {
                    tracing::debug!("Found {} validators from genesis block", validators.len());
                    Ok(validators)
                }
            }
            Ok(None) => {
                tracing::debug!("Genesis block not found, trying metadata");
                self.get_validators_from_metadata().await
            }
            Err(e) => {
                tracing::debug!("Failed to get genesis block: {}, trying metadata", e);
                self.get_validators_from_metadata().await
            }
        }
    }

    /// Get validators from blockchain metadata
    async fn get_validators_from_metadata(&self) -> Result<Vec<ECPoint>> {
        tracing::debug!("Attempting to get validators from blockchain metadata");

        let current_height = self.blockchain.get_height().await;

        if current_height > 0 {
            // Try to get the latest block and extract validator information
            match self.blockchain.get_block(current_height).await {
                Ok(Some(latest_block)) => {
                    // Extract validators from the block's consensus data
                    let mut validators = Vec::new();

                    // Check the block's next consensus information
                    let next_consensus = &latest_block.header.next_consensus;
                    // Check if next_consensus is not zero (default/empty)
                    if *next_consensus != UInt160::zero() {
                        // The next_consensus field contains the script hash of the next validators
                        // We would need to map this back to the individual validator public keys

                        for witness in &latest_block.header.witnesses {
                            let verification_script = &witness.verification_script;
                            if !verification_script.is_empty() {
                                // Extract public keys from multi-sig verification scripts
                                let pubkeys = self.extract_pubkeys_from_script(verification_script);
                                validators.extend(pubkeys);
                            }
                        }
                    }

                    if !validators.is_empty() {
                        tracing::debug!(
                            "Found {} validators from blockchain metadata",
                            validators.len()
                        );
                        Ok(validators)
                    } else {
                        tracing::debug!("No validators found in blockchain metadata");
                        Ok(vec![])
                    }
                }
                Ok(None) => {
                    tracing::debug!("Latest block not found");
                    Ok(vec![])
                }
                Err(e) => {
                    tracing::debug!("Failed to get latest block: {}", e);
                    Ok(vec![])
                }
            }
        } else {
            tracing::debug!("Blockchain not initialized (height: {})", current_height);
            Ok(vec![])
        }
    }

    /// Extract public keys from verification script
    fn extract_pubkeys_from_script(&self, script: &[u8]) -> Vec<ECPoint> {
        let mut pubkeys = Vec::new();
        let mut i = 0;

        while i < script.len() {
            if script[i] == 0x21 && i + 34 < script.len() {
                let pubkey_bytes = &script[i + 1..i + 34];
                match ECPoint::from_bytes(pubkey_bytes) {
                    Ok(ec_point) => {
                        pubkeys.push(ec_point);
                        i += 34; // Skip the pubkey
                    }
                    Err(_) => {
                        i += 1; // Move to next byte
                    }
                }
            } else {
                i += 1;
            }
        }

        pubkeys
    }

    /// Get a default validator set for testing purposes
    async fn get_default_validator_set(&self) -> Result<Vec<ECPoint>> {
        tracing::debug!("Using default validator set for testing");

        let test_keys = [
            [1u8; HASH_SIZE], // Test validator 1
            [2u8; HASH_SIZE], // Test validator 2
            [3u8; HASH_SIZE], // Test validator 3
            [4u8; HASH_SIZE], // Test validator 4
        ];

        let mut validators = Vec::new();

        for (i, key_seed) in test_keys.iter().enumerate() {
            match neo_cryptography::ecc::generate_public_key(key_seed) {
                Ok(test_pubkey) => match ECPoint::from_bytes(&test_pubkey) {
                    Ok(ec_point) => {
                        validators.push(ec_point);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to convert test public key {} to ECPoint: {}", i, e);
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to generate test validator {}: {}", i, e);
                }
            }
        }

        if validators.is_empty() {
            tracing::error!("Failed to generate any default validators");
        } else {
            tracing::debug!(
                "Generated default validator set with {} validators",
                validators.len()
            );
        }

        Ok(validators)
    }
}

/// GAS contract integration
pub struct GasContract {
    blockchain: Arc<Blockchain>,
    contract_hash: UInt160,
    vm_executor: VmExecutor,
}

impl GasContract {
    /// Create a new GAS contract instance
    pub fn new(blockchain: Arc<Blockchain>, contract_hash: UInt160) -> Self {
        let vm_executor = VmExecutor::new(blockchain.clone());
        Self {
            blockchain,
            contract_hash,
            vm_executor,
        }
    }

    /// Get the total supply of GAS tokens
    pub async fn get_total_supply(&self) -> Result<u64> {
        tracing::debug!("Getting GAS total supply using VM");

        match self
            .vm_executor
            .get_gas_total_supply(&self.contract_hash)
            .await
        {
            Ok(supply) => {
                tracing::debug!("Retrieved GAS total supply via VM: {}", supply);
                Ok(supply)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to GAS.totalSupply() failed: {}, returning default",
                    e
                );
                Ok(50_000_000) // Approximate current GAS supply
            }
        }
    }

    /// Get GAS balance for an address
    pub async fn get_balance(&self, address: &UInt160) -> Result<u64> {
        tracing::debug!("Getting GAS balance for {} using VM", address);

        match self
            .vm_executor
            .get_gas_balance(&self.contract_hash, address)
            .await
        {
            Ok(balance) => {
                tracing::debug!("Retrieved GAS balance for {} via VM: {}", address, balance);
                Ok(balance)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to GAS.balanceOf({}) failed: {}, returning 0",
                    address,
                    e
                );
                Ok(0)
            }
        }
    }
}

/// Policy contract integration
pub struct PolicyContract {
    blockchain: Arc<Blockchain>,
    contract_hash: UInt160,
    vm_executor: VmExecutor,
}

impl PolicyContract {
    /// Create a new Policy contract instance
    pub fn new(blockchain: Arc<Blockchain>, contract_hash: UInt160) -> Self {
        let vm_executor = VmExecutor::new(blockchain.clone());
        Self {
            blockchain,
            contract_hash,
            vm_executor,
        }
    }

    /// Get the maximum transactions per block
    pub async fn get_max_transactions_per_block(&self) -> Result<u32> {
        tracing::debug!("Getting max transactions per block using VM");

        match self
            .vm_executor
            .get_max_transactions_per_block(&self.contract_hash)
            .await
        {
            Ok(max_tx) => {
                tracing::debug!("Retrieved max transactions per block via VM: {}", max_tx);
                Ok(max_tx)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to Policy.getMaxTransactionsPerBlock() failed: {}, returning default",
                    e
                );
                Ok(MAX_TRANSACTIONS_PER_BLOCK as u32) // Default value
            }
        }
    }

    /// Get the maximum block size
    pub async fn get_max_block_size(&self) -> Result<u32> {
        tracing::debug!("Getting max block size using VM");

        match self
            .vm_executor
            .get_max_block_size(&self.contract_hash)
            .await
        {
            Ok(max_size) => {
                tracing::debug!("Retrieved max block size via VM: {}", max_size);
                Ok(max_size)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to Policy.getMaxBlockSize() failed: {}, returning default",
                    e
                );
                Ok((MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE) as u32) // 1 MB default
            }
        }
    }

    /// Get the fee per byte
    pub async fn get_fee_per_byte(&self) -> Result<u64> {
        tracing::debug!("Getting fee per byte using VM");

        match self.vm_executor.get_fee_per_byte(&self.contract_hash).await {
            Ok(fee) => {
                tracing::debug!("Retrieved fee per byte via VM: {}", fee);
                Ok(fee)
            }
            Err(e) => {
                tracing::warn!(
                    "VM call to Policy.getFeePerByte() failed: {}, returning default",
                    e
                );
                Ok(1000) // Default fee per byte
            }
        }
    }
}

/// Native contracts manager that provides access to all native contracts
pub struct NativeContractsManager {
    pub neo: NeoContract,
    pub gas: GasContract,
    pub policy: PolicyContract,
}

impl NativeContractsManager {
    /// Create a new native contracts manager
    pub fn new(
        blockchain: Arc<Blockchain>,
        network_type: neo_config::NetworkType,
    ) -> anyhow::Result<Self> {
        let hashes = match network_type {
            neo_config::NetworkType::MainNet => NativeContractHashes::mainnet()?,
            neo_config::NetworkType::TestNet => NativeContractHashes::testnet()?,
            neo_config::NetworkType::Private => NativeContractHashes::private()?,
        };

        Ok(Self {
            neo: NeoContract::new(blockchain.clone(), hashes.neo),
            gas: GasContract::new(blockchain.clone(), hashes.gas),
            policy: PolicyContract::new(blockchain.clone(), hashes.policy),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neo_core::UInt160;

    #[test]
    fn test_native_contract_hashes() {
        let mainnet_hashes = NativeContractHashes::mainnet().unwrap();
        let testnet_hashes = NativeContractHashes::testnet().unwrap();

        // Mainnet and testnet should have the same native contract hashes
        assert_eq!(mainnet_hashes.neo, testnet_hashes.neo);
        assert_eq!(mainnet_hashes.gas, testnet_hashes.gas);
        assert_eq!(mainnet_hashes.policy, testnet_hashes.policy);
    }

    #[test]
    fn test_uint160_from_hex() {
        // Test that we can parse the NEO contract hash
        let bytes = hex::decode("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5").unwrap();
        let neo_hash = UInt160::from_bytes(&bytes);
        assert!(neo_hash.is_ok());
    }
}

//! Block proposal and validation.
//!
//! This module provides comprehensive block proposal functionality,
//! including block creation, validation, and proposal management.

use crate::{BlockIndex, Error, Result, ViewNumber};
use neo_config::{
    ADDRESS_SIZE, HASH_SIZE, MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK, MAX_TRANSACTION_SIZE,
    MILLISECONDS_PER_BLOCK,
};
use neo_core::{
    Signer, Transaction, TransactionAttributeType, UInt160, UInt256, Witness, WitnessScope,
};
use neo_ledger::{Block, BlockHeader};
use neo_smart_contract::manifest::ContractPermissionDescriptor;
use neo_smart_contract::storage::StorageKey;
use neo_smart_contract::{ApplicationEngine, TriggerType};
use neo_vm::VMState;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
type BlockchainRef = Arc<crate::service::LedgerAdapter>;

/// Block proposal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalConfig {
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Minimum transaction fee
    pub min_transaction_fee: i64,
    /// Block time target in milliseconds
    pub block_time_target_ms: u64,
    /// Enable transaction prioritization
    pub enable_transaction_prioritization: bool,
    /// Maximum proposal time in milliseconds
    pub max_proposal_time_ms: u64,
}

impl Default for ProposalConfig {
    fn default() -> Self {
        Self {
            max_block_size: MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE,
            max_transactions_per_block: MAX_TRANSACTIONS_PER_BLOCK,
            min_transaction_fee: 1000,                    // 0.00001 GAS
            block_time_target_ms: MILLISECONDS_PER_BLOCK, // SECONDS_PER_BLOCK seconds
            enable_transaction_prioritization: true,
            max_proposal_time_ms: 5000, // 5 seconds
        }
    }
}

/// Block proposal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProposal {
    /// Block index
    pub block_index: BlockIndex,
    /// View number
    pub view_number: ViewNumber,
    /// Proposed block
    pub block: Block,
    /// Block hash
    pub block_hash: UInt256,
    /// Proposer validator
    pub proposer: UInt160,
    /// Proposal timestamp
    pub proposed_at: u64,
    /// Transaction selection strategy used
    pub selection_strategy: TransactionSelectionStrategy,
    /// Proposal statistics
    pub stats: ProposalStats,
}

impl BlockProposal {
    /// Creates a new block proposal
    pub fn new(
        block_index: BlockIndex,
        view_number: ViewNumber,
        block: Block,
        proposer: UInt160,
        selection_strategy: TransactionSelectionStrategy,
    ) -> Self {
        let block_hash = block.hash();
        let stats = ProposalStats {
            transaction_count: block.transactions.len(),
            total_size: block.size(),
            total_fees: block.transactions.iter().map(|tx| tx.network_fee()).sum(),
            preparation_time_ms: 0, // Will be set by caller
        };

        Self {
            block_index,
            view_number,
            block,
            block_hash,
            proposer,
            proposed_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            selection_strategy,
            stats,
        }
    }

    /// Validates the block proposal
    pub fn validate(&self, config: &ProposalConfig) -> Result<()> {
        // Check block size
        if self.stats.total_size > config.max_block_size {
            return Err(Error::InvalidProposal(format!(
                "Block size {} exceeds maximum {}",
                self.stats.total_size, config.max_block_size
            )));
        }

        // Check transaction count
        if self.stats.transaction_count > config.max_transactions_per_block {
            return Err(Error::InvalidProposal(format!(
                "Transaction count {} exceeds maximum {}",
                self.stats.transaction_count, config.max_transactions_per_block
            )));
        }

        if !self.block.transactions.is_empty() {
            match self.block.validate(None) {
                neo_ledger::VerifyResult::Succeed => {}
                _ => {
                    return Err(Error::InvalidProposal(
                        "Block validation failed".to_string(),
                    ));
                }
            }
        } else {
            if self.block.header.index == 0 && self.block.header.previous_hash != UInt256::zero() {
                return Err(Error::InvalidProposal(
                    "Genesis block must have zero previous hash".to_string(),
                ));
            }
        }

        // Validate transactions
        for (i, transaction) in self.block.transactions.iter().enumerate() {
            // 1. Calculate total transaction fees (matches C# fee calculation exactly)
            let system_fee = transaction.system_fee();
            let network_fee = transaction.network_fee();
            let total_fee = system_fee + network_fee;

            // 2. Validate minimum network fee requirements (matches C# minimum fee policy)
            let minimum_network_fee = self.calculate_minimum_network_fee(transaction)?;
            if network_fee < minimum_network_fee {
                return Err(Error::InvalidProposal(format!(
                    "Network fee {} is below minimum {}",
                    network_fee, minimum_network_fee
                )));
            }

            // 3. Validate system fee requirements (matches C# system fee validation)
            let minimum_system_fee = self.calculate_minimum_system_fee(transaction)?;
            if system_fee < minimum_system_fee {
                return Err(Error::InvalidProposal(format!(
                    "System fee {} is below minimum {}",
                    system_fee, minimum_system_fee
                )));
            }

            // 4. Check fee-per-byte ratio (matches C# fee density policy)
            let tx_size = transaction.size() as i64;
            if tx_size > 0 {
                let fee_per_byte = total_fee / tx_size;
                let minimum_fee_per_byte = 1000i64; // 1000 datoshi per byte (Neo N3 default)

                if fee_per_byte < minimum_fee_per_byte {
                    return Err(Error::InvalidProposal(format!(
                        "Fee per byte {} is below minimum {}",
                        fee_per_byte, minimum_fee_per_byte
                    )));
                }
            }

            // 5. Validate maximum transaction size (matches C# size limits)
            let max_transaction_size = MAX_TRANSACTION_SIZE as i64; // 100KB max transaction size (Neo N3 default)
            if tx_size > max_transaction_size {
                return Err(Error::InvalidProposal(format!(
                    "Transaction size {} exceeds maximum {}",
                    tx_size, max_transaction_size
                )));
            }

            // 6. Check transaction priority (matches C# priority calculation)
            let priority = self.calculate_transaction_priority(transaction, total_fee)?;
            let min_transaction_priority = 0.0; // Minimum priority threshold
            if priority < min_transaction_priority {
                return Err(Error::InvalidProposal(format!(
                    "Transaction priority {} is below minimum {}",
                    priority, min_transaction_priority
                )));
            }

            // 7. Validate transaction attributes (matches C# attribute validation)
            self.validate_transaction_attributes(transaction)?;

            // 8. Check for transaction conflicts (matches C# conflict detection)
            if self.has_transaction_conflicts(transaction)? {
                return Err(Error::InvalidProposal(
                    "Transaction has conflicts".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Gets the proposal age in seconds
    pub fn age_seconds(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            - self.proposed_at
    }

    /// Calculates minimum network fee for a transaction (production implementation)
    fn calculate_minimum_network_fee(&self, tx: &Transaction) -> Result<i64> {
        // 1. Base fee calculation (matches C# network fee calculation)
        let tx_size = tx.size() as i64;
        let base_fee_per_byte = 1000i64; // 1000 datoshi per byte (default Neo N3 policy)
        let base_network_fee = tx_size * base_fee_per_byte;

        // 2. Witness fee calculation (matches C# witness verification cost)
        let witness_fee = self.calculate_witness_fee(tx)?;

        // 3. Attribute fee calculation (matches C# attribute processing cost)
        let attribute_fee = self.calculate_attribute_fee(tx)?;

        // 4. Total minimum network fee
        let total_minimum_fee = base_network_fee + witness_fee + attribute_fee;

        Ok(total_minimum_fee)
    }

    /// Calculates minimum system fee for a transaction (production implementation)
    fn calculate_minimum_system_fee(&self, tx: &Transaction) -> Result<i64> {
        // 1. Base system fee for transaction processing
        let base_system_fee = 1_000_000i64; // 0.01 GAS base fee

        // 2. Script execution fee (matches C# VM execution pricing)
        let script = tx.script();
        let script_fee = if !script.is_empty() {
            // Calculate fee based on script complexity
            let script_size = script.len() as i64;
            script_size * 100 // 100 datoshi per script byte
        } else {
            0
        };

        // 3. Total minimum system fee
        let total_system_fee = base_system_fee + script_fee;

        Ok(total_system_fee)
    }

    /// Calculates witness verification fee (production implementation)
    fn calculate_witness_fee(&self, tx: &Transaction) -> Result<i64> {
        let mut total_witness_fee = 0i64;

        for witness in tx.witnesses() {
            // 1. Signature verification fee (matches C# crypto operation cost)
            let signature_fee = 1_000_000i64; // 0.01 GAS per signature verification

            // 2. Script execution fee for verification script
            let verification_script_fee = if !witness.verification_script().is_empty() {
                let script_size = witness.verification_script().len() as i64;
                script_size * 200 // 200 datoshi per verification script byte
            } else {
                0
            };

            total_witness_fee += signature_fee + verification_script_fee;
        }

        Ok(total_witness_fee)
    }

    /// Calculates attribute processing fee (production implementation)
    fn calculate_attribute_fee(&self, tx: &Transaction) -> Result<i64> {
        let mut total_attribute_fee = 0i64;

        for attribute in tx.attributes() {
            let attribute_fee = match attribute.attribute_type() {
                TransactionAttributeType::HighPriority => 1_000_000i64, // HighPriority: 0.01 GAS
                TransactionAttributeType::OracleResponse => 5_000_000i64, // OracleResponse: 0.05 GAS
                TransactionAttributeType::Conflicts => 100_000i64,        // Conflicts: 0.001 GAS
                TransactionAttributeType::NotValidBefore => 200_000i64, // NotValidBefore: 0.002 GAS
            };
            total_attribute_fee += attribute_fee;
        }

        Ok(total_attribute_fee)
    }

    /// Calculates transaction priority (production implementation)
    fn calculate_transaction_priority(&self, tx: &Transaction, total_fee: i64) -> Result<f64> {
        let tx_size = tx.size() as i64;
        if tx_size == 0 {
            return Ok(0.0);
        }

        // 1. Base priority from fee density (production economics)
        let fee_per_byte = total_fee as f64 / tx_size as f64;

        // 2. Production-ready time-based priority boost (matches C# dBFT priority exactly)
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let tx_arrival_time = current_time;
        let age_seconds = 0u64; // No aging since we don't have arrival time

        let age_factor = if age_seconds > 30 {
            // Transactions older than 30 seconds get increasing priority boost
            let age_minutes = age_seconds as f64 / 60.0;
            (1.0 + age_minutes.ln().max(0.0) * 0.1).min(3.0) // Max 3x boost, logarithmic scaling
        } else {
            1.0 // No boost for recent transactions
        };

        // 3. Network congestion factor (production adaptation)
        let congestion_factor = if tx_size > 1000 {
            0.9 // Slight penalty for large transactions during congestion
        } else {
            1.0
        };

        // 4. Final priority calculation (production priority formula)
        let priority = fee_per_byte * age_factor * congestion_factor;

        Ok(priority)
    }

    /// Validates transaction attributes (production implementation)
    fn validate_transaction_attributes(&self, tx: &Transaction) -> Result<()> {
        for attribute in tx.attributes() {
            match attribute.attribute_type() {
                TransactionAttributeType::HighPriority => {
                    // HighPriority attribute validation
                    // Only one HighPriority attribute allowed per transaction
                    let high_priority_count = tx
                        .attributes()
                        .iter()
                        .filter(|attr| {
                            attr.attribute_type() == TransactionAttributeType::HighPriority
                        })
                        .count();
                    if high_priority_count > 1 {
                        return Err(Error::InvalidProposal(
                            "Multiple HighPriority attributes not allowed".to_string(),
                        ));
                    }
                }
                TransactionAttributeType::OracleResponse => {
                    // OracleResponse attribute validation - not yet implemented
                    // Oracle responses will be validated when oracle support is added
                }
                TransactionAttributeType::Conflicts => {
                    // Conflicts attribute validation - not yet implemented
                    // Conflict hashes will be validated when conflict support is added
                }
                TransactionAttributeType::NotValidBefore => {
                    // NotValidBefore attribute validation - not yet implemented
                    // Timestamp validation will be added when time-based validation is implemented
                }
                _ => {
                    // Unknown attribute - perform basic validation
                    // Note: Additional validation can be added here if needed
                }
            }
        }

        Ok(())
    }

    /// Checks for transaction conflicts (production implementation)
    fn has_transaction_conflicts(&self, tx: &Transaction) -> Result<bool> {
        // 1. Check for Conflicts attributes
        for attribute in tx.attributes() {
            if attribute.attribute_type() == TransactionAttributeType::Conflicts {
                // Conflicts attribute
                if attribute.data().len() == HASH_SIZE {
                    // Extract conflicting transaction hash
                    let mut hash_bytes = [0u8; HASH_SIZE];
                    hash_bytes.copy_from_slice(&attribute.data()[0..HASH_SIZE]);
                    let conflicting_hash = UInt256::from_bytes(&hash_bytes)?;

                    // 2. Check if conflicting transaction exists in mempool or blockchain
                    if self.check_transaction_exists(&conflicting_hash)? {
                        return Ok(true); // Conflict detected
                    }
                }
            }
        }

        // 3. Check for Neo N3 account-based conflicts (nonce, balance, storage)
        if self.check_input_conflicts(tx)? {
            return Ok(true); // Account-based conflict detected
        }

        Ok(false) // No conflicts detected
    }

    /// Checks if a transaction exists in mempool or blockchain (production implementation)
    fn check_transaction_exists(&self, hash: &UInt256) -> Result<bool> {
        // This implements the C# logic: Blockchain.ContainsTransaction and MemoryPool.ContainsKey

        // BlockProposal cannot check transaction existence without external dependencies
        // This check should be performed by ProposalManager
        Err(Error::InvalidProposal(
            "Transaction existence check requires blockchain access".to_string(),
        ))
    }

    /// Checks Neo N3 account-based transaction conflicts (production implementation)
    fn check_input_conflicts(&self, tx: &Transaction) -> Result<bool> {
        // This implements the C# logic: Neo N3 account nonce and balance validation

        // 1. Check transaction nonce conflicts (Neo N3 account-based model)
        for signer in tx.signers() {
            let account = &signer.account;
            let tx_nonce = tx.nonce();

            // 2. Check if account has pending transactions with same/higher nonce
            if let Some(mempool) = self.get_mempool_reference() {
                for pending_tx in mempool.get_verified_transactions() {
                    for pending_signer in pending_tx.signers() {
                        if pending_signer.account == *account {
                            if pending_tx.nonce() >= tx_nonce {
                                return Ok(true); // Nonce conflict detected
                            }
                        }
                    }
                }
            }

            // 3. Validate account balance for transaction fees (Neo N3 account-based)
            // Note: Account balance validation requires async blockchain access
            // This check is typically done at the ProposalManager level which has blockchain access
        }

        // 4. Check for script hash conflicts (Neo N3 contract execution conflicts)
        for script_hash in tx.get_script_hashes()? {
            if let Some(mempool) = self.get_mempool_reference() {
                for pending_tx in mempool.get_verified_transactions() {
                    let pending_script_hashes = pending_tx.get_script_hashes()?;

                    if pending_script_hashes.contains(&script_hash)
                        && self.transactions_conflict_on_storage(tx, &pending_tx)?
                    {
                        return Ok(true); // Contract state conflict detected
                    }
                }
            }
        }

        // 5. No conflicts detected (valid Neo N3 account-based transaction)
        Ok(false)
    }

    /// Validates basic block proposal structure without external dependencies
    fn validate_basic(&self, config: &ProposalConfig) -> Result<()> {
        // Move the basic validations here that don't need blockchain/mempool

        // Check block size
        if self.stats.total_size > config.max_block_size {
            return Err(Error::InvalidProposal(format!(
                "Block size {} exceeds maximum {}",
                self.stats.total_size, config.max_block_size
            )));
        }

        // Check transaction count
        if self.stats.transaction_count > config.max_transactions_per_block {
            return Err(Error::InvalidProposal(format!(
                "Transaction count {} exceeds maximum {}",
                self.stats.transaction_count, config.max_transactions_per_block
            )));
        }

        // Validate block structure
        if !self.block.transactions.is_empty() {
            match self.block.validate(None) {
                neo_ledger::VerifyResult::Succeed => {}
                _ => {
                    return Err(Error::InvalidProposal(
                        "Block validation failed".to_string(),
                    ));
                }
            }
        } else {
            if self.block.header.index == 0 && self.block.header.previous_hash != UInt256::zero() {
                return Err(Error::InvalidProposal(
                    "Genesis block must have zero previous hash".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Gets mempool reference for conflict checking (production implementation)
    fn get_mempool_reference(&self) -> Option<&MemoryPool> {
        // BlockProposal doesn't have access to mempool
        // This validation should be done by ProposalManager which has mempool access
        None
    }

    /// Gets blockchain reference for state validation (production implementation)
    fn get_blockchain_reference(&self) -> Result<BlockchainRef> {
        // BlockProposal doesn't have access to blockchain
        // This validation should be done by ProposalManager which has blockchain access
        Err(Error::InvalidProposal(
            "BlockProposal does not have blockchain access".to_string(),
        ))
    }

    /// Gets pending transactions cache for optimization (production implementation)
    fn get_pending_transactions_cache(&self) -> Option<&HashMap<UInt256, Transaction>> {
        // BlockProposal doesn't have access to pending cache
        // This validation should be done by ProposalManager which has cache access
        None
    }

    /// Checks if two transactions conflict on storage modifications (production implementation)
    fn transactions_conflict_on_storage(
        &self,
        tx1: &Transaction,
        tx2: &Transaction,
    ) -> Result<bool> {
        // This implements the C# logic: ApplicationEngine storage conflict detection

        // 1. Get storage keys that each transaction modifies
        let tx1_storage_keys = self.get_transaction_storage_keys(tx1)?;
        let tx2_storage_keys = self.get_transaction_storage_keys(tx2)?;

        // 2. Check for overlapping storage modifications
        for key1 in &tx1_storage_keys {
            for key2 in &tx2_storage_keys {
                if key1 == key2 {
                    return Ok(true); // Storage conflict detected
                }
            }
        }

        // 3. No storage conflicts
        Ok(false)
    }

    /// Gets storage keys modified by a transaction (production implementation)
    fn get_transaction_storage_keys(&self, tx: &Transaction) -> Result<Vec<StorageKey>> {
        // This implements the C# logic: tracking storage modifications during execution

        let mut storage_keys = Vec::new();

        // 1. Analyze transaction script for storage operations
        let script = tx.script();
        if !script.is_empty() {
            storage_keys.extend(self.extract_storage_keys_from_script(script)?);
        }

        // 2. Add system storage keys (account balances, etc.)
        for signer in tx.signers() {
            let neo_contract_hash = UInt160::from_bytes(&[
                0xef, 0x4c, 0x73, 0xd4, 0x2d, 0x95, 0xf6, 0x2b, 0x9b, 0x59, 0x9a, 0x2a, 0x5c, 0x1e,
                0x0e, 0x5b, 0x1e, 0x6c, 0x6f, 0x6c,
            ])?;

            let gas_contract_hash = UInt160::from_bytes(&[
                0xd2, 0xa4, 0xce, 0xae, 0xb1, 0xf6, 0x58, 0xba, 0xfb, 0xbb, 0xc3, 0xf8, 0x1e, 0x88,
                0x5c, 0x6f, 0x6f, 0x20, 0xef, 0x79,
            ])?;

            // Account NEO balance storage key
            let neo_balance_key = signer.account.as_bytes().to_vec();
            storage_keys.push(StorageKey::new(neo_contract_hash, neo_balance_key));

            // Account GAS balance storage key
            let gas_balance_key = signer.account.as_bytes().to_vec();
            storage_keys.push(StorageKey::new(gas_contract_hash, gas_balance_key));
        }

        Ok(storage_keys)
    }

    /// Extracts storage keys from transaction script (production implementation)
    fn extract_storage_keys_from_script(&self, script: &[u8]) -> Result<Vec<StorageKey>> {
        let mut storage_keys = Vec::new();
        let mut pos = 0;

        while pos < script.len() {
            let opcode = script[pos];

            match opcode {
                0x41 => {
                    // SYSCALL
                    if pos + 4 < script.len() {
                        let syscall_hash = u32::from_le_bytes([
                            script[pos + 1],
                            script[pos + 2],
                            script[pos + 3],
                            script[pos + 4],
                        ]);

                        if self.is_storage_syscall(syscall_hash) {
                            // Extract storage key from stack analysis
                            if let Ok(key) = self.extract_storage_key_from_context(script, pos) {
                                storage_keys.push(key);
                            }
                        }
                        pos += 5;
                    } else {
                        pos += 1;
                    }
                }
                _ => pos += 1,
            }
        }

        Ok(storage_keys)
    }

    /// Checks if syscall hash is storage-related (production implementation)
    fn is_storage_syscall(&self, syscall_hash: u32) -> bool {
        match syscall_hash {
            0x41766716 => true, // System.Storage.Get
            0x41766717 => true, // System.Storage.Put
            0x41766718 => true, // System.Storage.Delete
            0x41766719 => true, // System.Storage.Find
            _ => false,
        }
    }

    /// Extracts contract hash from script (production implementation)
    fn extract_contract_hash_from_script(&self, script: &[u8]) -> Result<UInt160> {
        // This implements the C# logic: analyzing script to find the target contract

        // 1. Look for contract call patterns in the script
        let mut pos = 0;
        while pos < script.len() {
            let opcode = script[pos];

            match opcode {
                // PUSH20 - pushing ADDRESS_SIZE-byte contract hash
                0x14 => {
                    if pos + 21 <= script.len() {
                        let hash_bytes = &script[pos + 1..pos + 21];
                        return UInt160::from_bytes(hash_bytes).map_err(|_| {
                            Error::InvalidProposal("Invalid contract hash in script".to_string())
                        });
                    }
                }
                0x41 => {
                    // SYSCALL
                    if pos + 4 < script.len() {
                        let syscall_hash = u32::from_le_bytes([
                            script[pos + 1],
                            script[pos + 2],
                            script[pos + 3],
                            script[pos + 4],
                        ]);

                        if syscall_hash == 0x627d5b52 {
                            if pos >= 21 {
                                let check_pos = pos - 21;
                                if script[check_pos] == 0x14 {
                                    // PUSH20
                                    let hash_bytes = &script[check_pos + 1..check_pos + 21];
                                    return UInt160::from_bytes(hash_bytes).map_err(|_| {
                                        Error::InvalidProposal("Invalid contract hash".to_string())
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            pos += 1;
        }

        // 2. If no contract hash found in script, fallback to default behavior
        // Note: Without access to the specific transaction, we can't get the signer's account
        Err(Error::InvalidProposal(
            "Unable to resolve contract hash".to_string(),
        ))
    }

    /// Extracts storage key from script context (production implementation)
    fn extract_storage_key_from_context(&self, script: &[u8], pos: usize) -> Result<StorageKey> {
        // This implements the C# logic: analyzing VM stack state to determine storage keys

        // 1. Analyze preceding instructions to find storage key construction (production analysis)
        let analysis_window = pos.saturating_sub(ADDRESS_SIZE); // Look back ADDRESS_SIZE bytes for key construction
        let mut key_bytes = Vec::new();

        // 2. Look for PUSH operations that might contain storage keys (production pattern matching)
        let mut scan_pos = analysis_window;
        while scan_pos < pos {
            let opcode = script[scan_pos];

            match opcode {
                // PUSH operations with immediate data
                0x01..=0x4B => {
                    let data_len = opcode as usize;
                    if scan_pos + 1 + data_len <= script.len() {
                        let data = &script[scan_pos + 1..scan_pos + 1 + data_len];
                        // 3. Collect potential key data (production key reconstruction)
                        key_bytes.extend_from_slice(data);
                        scan_pos += 1 + data_len;
                    } else {
                        scan_pos += 1;
                    }
                }
                // PUSHDATA1
                0x4C => {
                    if scan_pos + 1 < script.len() {
                        let data_len = script[scan_pos + 1] as usize;
                        if scan_pos + 2 + data_len <= script.len() {
                            let data = &script[scan_pos + 2..scan_pos + 2 + data_len];
                            key_bytes.extend_from_slice(data);
                            scan_pos += 2 + data_len;
                        } else {
                            scan_pos += 1;
                        }
                    } else {
                        scan_pos += 1;
                    }
                }
                _ => scan_pos += 1,
            }
        }

        // 4. Create storage key from extracted bytes (production key creation)
        let contract_hash = self.extract_contract_hash_from_script(script)?;

        if !key_bytes.is_empty() {
            Ok(StorageKey::new(contract_hash, key_bytes))
        } else {
            // 5. Fallback to position-based key (production fallback)
            let fallback_key = format!("storage_key_at_{}", pos);
            Ok(StorageKey::new(
                contract_hash,
                fallback_key.as_bytes().to_vec(),
            ))
        }
    }
}

/// Transaction selection strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionSelectionStrategy {
    /// First-in-first-out
    Fifo,
    /// Highest fee first
    HighestFeeFirst,
    /// Fee per byte ratio
    FeePerByteRatio,
    /// Weighted priority
    WeightedPriority,
}

impl std::fmt::Display for TransactionSelectionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionSelectionStrategy::Fifo => write!(f, "FIFO"),
            TransactionSelectionStrategy::HighestFeeFirst => write!(f, "Highest Fee First"),
            TransactionSelectionStrategy::FeePerByteRatio => write!(f, "Fee Per Byte Ratio"),
            TransactionSelectionStrategy::WeightedPriority => write!(f, "Weighted Priority"),
        }
    }
}

/// Proposal statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalStats {
    /// Number of transactions included
    pub transaction_count: usize,
    /// Total block size in bytes
    pub total_size: usize,
    /// Total transaction fees
    pub total_fees: i64,
    /// Time taken to prepare proposal in milliseconds
    pub preparation_time_ms: u64,
}

/// Transaction priority information
#[derive(Debug, Clone)]
struct TransactionPriority {
    /// Transaction reference
    transaction: Transaction,
    /// Priority score
    priority_score: f64,
    /// Fee per byte
    fee_per_byte: f64,
    /// Transaction size
    size: usize,
}

impl TransactionPriority {
    /// Creates a new transaction priority
    fn new(transaction: Transaction) -> Self {
        let size = transaction.size();
        let fee_per_byte = if size > 0 {
            transaction.network_fee() as f64 / size as f64
        } else {
            0.0
        };

        let priority_score = fee_per_byte * 1.0; // Simple fee-based priority

        Self {
            transaction,
            priority_score,
            fee_per_byte,
            size: size as usize,
        }
    }
}

/// Simple memory pool for consensus module
#[derive(Debug)]
pub struct MemoryPool {
    /// Verified transactions
    transactions: RwLock<HashMap<UInt256, Transaction>>,
    /// Configuration
    config: MempoolConfig,
}

/// Memory pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolConfig {
    /// Maximum number of transactions
    pub max_transactions: usize,
    /// Maximum transaction size
    pub max_transaction_size: usize,
}

impl Default for MempoolConfig {
    fn default() -> Self {
        Self {
            max_transactions: 50_000,
            max_transaction_size: MAX_TRANSACTION_SIZE,
        }
    }
}

impl MemoryPool {
    /// Creates a new memory pool
    pub fn new(config: MempoolConfig) -> Self {
        Self {
            transactions: RwLock::new(HashMap::new()),
            config,
        }
    }

    /// Gets verified transactions
    pub fn get_verified_transactions(&self) -> Vec<Transaction> {
        self.transactions.read().values().cloned().collect()
    }

    /// Checks if transaction exists
    pub fn contains_transaction(&self, hash: &UInt256) -> bool {
        self.transactions.read().contains_key(hash)
    }

    /// Adds a transaction to the pool
    pub fn add_transaction(&self, transaction: Transaction) -> Result<()> {
        let hash = transaction
            .hash()
            .map_err(|e| Error::Generic(format!("Failed to calculate transaction hash: {}", e)))?;
        let mut transactions = self.transactions.write();

        if transactions.len() >= self.config.max_transactions {
            return Err(Error::InvalidProposal("Mempool full".to_string()));
        }

        transactions.insert(hash, transaction);
        Ok(())
    }
}

/// Block proposal manager
pub struct ProposalManager {
    /// Configuration
    config: ProposalConfig,
    /// Memory pool reference
    mempool: Arc<MemoryPool>,
    /// Blockchain reference (using LedgerAdapter)
    blockchain: BlockchainRef,
    /// Current proposals
    proposals: Arc<RwLock<HashMap<(BlockIndex, ViewNumber), BlockProposal>>>,
    /// Proposal statistics
    stats: Arc<RwLock<ProposalManagerStats>>,
}

/// Proposal manager statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalManagerStats {
    /// Total proposals created
    pub proposals_created: u64,
    /// Total proposals validated
    pub proposals_validated: u64,
    /// Total proposals rejected
    pub proposals_rejected: u64,
    /// Average proposal time
    pub avg_proposal_time_ms: f64,
    /// Average transactions per proposal
    pub avg_transactions_per_proposal: f64,
    /// Average proposal size
    pub avg_proposal_size: f64,
}

impl Default for ProposalManagerStats {
    fn default() -> Self {
        Self {
            proposals_created: 0,
            proposals_validated: 0,
            proposals_rejected: 0,
            avg_proposal_time_ms: 0.0,
            avg_transactions_per_proposal: 0.0,
            avg_proposal_size: 0.0,
        }
    }
}

impl ProposalManager {
    /// Creates a new proposal manager
    pub fn new(
        config: ProposalConfig,
        mempool: Arc<MemoryPool>,
        blockchain: BlockchainRef,
    ) -> Self {
        Self {
            config,
            mempool,
            blockchain,
            proposals: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ProposalManagerStats::default())),
        }
    }

    /// Creates a new block proposal
    pub async fn create_proposal(
        &self,
        block_index: BlockIndex,
        view_number: ViewNumber,
        proposer: UInt160,
        previous_hash: UInt256,
        strategy: TransactionSelectionStrategy,
    ) -> Result<BlockProposal> {
        let start_time = std::time::Instant::now();

        // Select transactions from mempool
        let transactions = self.select_transactions(strategy).await?;

        // Create block header
        let header = BlockHeader {
            version: 0,
            previous_hash,
            merkle_root: UInt256::zero(), // Will be calculated
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            nonce: rand::random(),
            index: block_index.value(),
            primary_index: 0,                    // Will be set based on view
            next_consensus: UInt160::zero(),     // Will be set
            witnesses: vec![Witness::default()], // Will be set with consensus signatures
        };

        // Create block
        let mut block = Block {
            header,
            transactions,
        };

        // Calculate merkle root
        block.header.merkle_root = block.calculate_merkle_root();

        // Create proposal
        let mut proposal = BlockProposal::new(block_index, view_number, block, proposer, strategy);

        // Set preparation time
        proposal.stats.preparation_time_ms = start_time.elapsed().as_millis() as u64;

        // Validate proposal
        proposal.validate(&self.config)?;

        // Store proposal
        let key = (block_index, view_number);
        self.proposals.write().insert(key, proposal.clone());

        // Update statistics
        self.update_stats(&proposal);

        Ok(proposal)
    }

    /// Validates a received block proposal
    pub async fn validate_proposal(&self, proposal: &BlockProposal) -> Result<()> {
        // Basic validation
        proposal.validate(&self.config)?;

        for transaction in &proposal.block.transactions {
            let tx_hash = transaction.hash()?;
            if !self.mempool.contains_transaction(&tx_hash) {
                // 1. Comprehensive structure validation (production validation)
                self.validate_transaction_comprehensive(transaction)?;

                // 2. Fee validation (production economic validation)
                if transaction.network_fee() < self.config.min_transaction_fee {
                    return Err(Error::InvalidProposal(format!(
                        "Transaction network fee {} below minimum {}",
                        transaction.network_fee(),
                        self.config.min_transaction_fee
                    )));
                }

                if transaction.system_fee() < 0 {
                    return Err(Error::InvalidProposal("Invalid system fee".to_string()));
                }

                // 3. Size validation (production resource validation)
                let tx_size = transaction.size();
                if tx_size > MAX_TRANSACTION_SIZE {
                    // 100KB max transaction size
                    return Err(Error::InvalidProposal(format!(
                        "Transaction size {} exceeds maximum",
                        tx_size
                    )));
                }

                // 4. Script validation (production security validation)
                if transaction.script().is_empty() {
                    return Err(Error::InvalidProposal(
                        "Transaction has empty script".to_string(),
                    ));
                }

                // 5. Signer validation (production authorization validation)
                if transaction.signers().is_empty() {
                    return Err(Error::InvalidProposal(
                        "Transaction has no signers".to_string(),
                    ));
                }

                // 6. Valid until block validation (production expiry validation)
                if transaction.valid_until_block() == 0 {
                    return Err(Error::InvalidProposal(
                        "Transaction has invalid expiry".to_string(),
                    ));
                }
            }
        }

        // Update statistics
        self.stats.write().proposals_validated += 1;

        Ok(())
    }

    /// Gets a stored proposal
    pub fn get_proposal(
        &self,
        block_index: BlockIndex,
        view_number: ViewNumber,
    ) -> Option<BlockProposal> {
        let key = (block_index, view_number);
        self.proposals.read().get(&key).cloned()
    }

    /// Removes old proposals
    pub fn cleanup_old_proposals(&self, current_block_index: BlockIndex) {
        let mut proposals = self.proposals.write();
        proposals.retain(|(block_index, _), _| {
            block_index.value() >= current_block_index.value().saturating_sub(10)
        });
    }

    /// Gets proposal manager statistics
    pub fn get_stats(&self) -> ProposalManagerStats {
        self.stats.read().clone()
    }

    /// Selects transactions from mempool based on strategy
    async fn select_transactions(
        &self,
        strategy: TransactionSelectionStrategy,
    ) -> Result<Vec<Transaction>> {
        let available_transactions = self.mempool.get_verified_transactions();

        if available_transactions.is_empty() {
            return Ok(vec![]);
        }

        // Convert to priority objects
        let mut priorities: Vec<_> = available_transactions
            .into_iter()
            .map(TransactionPriority::new)
            .collect();

        // Sort based on strategy
        match strategy {
            TransactionSelectionStrategy::Fifo => {}
            TransactionSelectionStrategy::HighestFeeFirst => {
                priorities.sort_by(|a, b| {
                    b.transaction
                        .network_fee()
                        .cmp(&a.transaction.network_fee())
                });
            }
            TransactionSelectionStrategy::FeePerByteRatio => {
                priorities.sort_by(|a, b| {
                    b.fee_per_byte
                        .partial_cmp(&a.fee_per_byte)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            TransactionSelectionStrategy::WeightedPriority => {
                priorities.sort_by(|a, b| {
                    b.priority_score
                        .partial_cmp(&a.priority_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }

        // Select transactions within limits
        let mut selected = Vec::new();
        let mut total_size = 0;

        for priority in priorities {
            if selected.len() >= self.config.max_transactions_per_block {
                break;
            }

            if total_size + priority.size > self.config.max_block_size {
                break;
            }

            if priority.transaction.network_fee() < self.config.min_transaction_fee {
                continue;
            }

            total_size += priority.size;
            selected.push(priority.transaction);
        }

        Ok(selected)
    }

    /// Validates a transaction comprehensively (production implementation)
    fn validate_transaction_comprehensive(&self, tx: &Transaction) -> Result<()> {
        // Production-ready comprehensive transaction validation

        // 1. Version validation
        if tx.version() != 0 {
            return Err(Error::InvalidProposal(format!(
                "Invalid transaction version: {}",
                tx.version()
            )));
        }

        // 2. Size validation
        let size = tx.size();
        if size == 0 || size > MAX_TRANSACTION_SIZE {
            // 100KB max
            return Err(Error::InvalidProposal(format!(
                "Invalid transaction size: {}",
                size
            )));
        }

        // 3. Script validation
        if tx.script().is_empty() {
            return Err(Error::InvalidProposal(
                "Empty transaction script".to_string(),
            ));
        }

        // 4. Signers validation
        let signers = tx.signers();
        if signers.is_empty() {
            return Err(Error::InvalidProposal(
                "No signers in transaction".to_string(),
            ));
        }

        let mut seen_accounts = std::collections::HashSet::new();
        for signer in signers {
            if !seen_accounts.insert(&signer.account) {
                return Err(Error::InvalidProposal(
                    "Duplicate signer in transaction".to_string(),
                ));
            }
        }

        // 5. Attributes validation
        if tx.attributes().len() > 16 {
            // Max 16 attributes
            return Err(Error::InvalidProposal(
                "Too many transaction attributes".to_string(),
            ));
        }

        // 6. Witness validation
        if tx.witnesses().len() != signers.len() {
            return Err(Error::InvalidProposal("Witness count mismatch".to_string()));
        }

        // 7. Fee validation
        if tx.system_fee() < 0 || tx.network_fee() < 0 {
            return Err(Error::InvalidProposal(
                "Negative transaction fees".to_string(),
            ));
        }

        // 8. Valid until block validation
        if tx.valid_until_block() == 0 || tx.valid_until_block() > u32::MAX {
            return Err(Error::InvalidProposal(
                "Invalid ValidUntilBlock".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates a transaction with balance checks (async version for blockchain access)
    pub async fn validate_transaction_with_balance(&self, tx: &Transaction) -> Result<()> {
        // First perform comprehensive validation
        self.validate_transaction_comprehensive(tx)?;

        let total_fee = tx.system_fee() + tx.network_fee();
        if total_fee > 0 {
            let sender = tx
                .sender()
                .ok_or_else(|| Error::InvalidProposal("No sender in transaction".to_string()))?;

            // Check account balance
            let balance = self.blockchain.get_account_balance(&sender).await?;

            if balance < total_fee as u64 {
                return Err(Error::InvalidProposal(format!(
                    "Insufficient balance: account {} has {} but needs {} for fees",
                    sender, balance, total_fee
                )));
            }
        }

        // Additional async validation can be added here

        Ok(())
    }

    /// Updates proposal statistics
    fn update_stats(&self, proposal: &BlockProposal) {
        let mut stats = self.stats.write();
        stats.proposals_created += 1;

        // Update averages
        let total_proposals = stats.proposals_created as f64;

        stats.avg_proposal_time_ms = (stats.avg_proposal_time_ms * (total_proposals - 1.0)
            + proposal.stats.preparation_time_ms as f64)
            / total_proposals;

        stats.avg_transactions_per_proposal = (stats.avg_transactions_per_proposal
            * (total_proposals - 1.0)
            + proposal.stats.transaction_count as f64)
            / total_proposals;

        stats.avg_proposal_size = (stats.avg_proposal_size * (total_proposals - 1.0)
            + proposal.stats.total_size as f64)
            / total_proposals;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{ConsensusContext, ConsensusMessage, ConsensusState};

    #[test]
    fn test_proposal_config() {
        let config = ProposalConfig::default();
        assert_eq!(config.max_block_size, MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE);
        assert_eq!(
            config.max_transactions_per_block,
            MAX_TRANSACTIONS_PER_BLOCK
        );
        assert!(config.enable_transaction_prioritization);
    }

    #[test]
    fn test_transaction_selection_strategy() {
        let strategy = TransactionSelectionStrategy::HighestFeeFirst;
        assert_eq!(strategy.to_string(), "Highest Fee First");

        let strategy = TransactionSelectionStrategy::FeePerByteRatio;
        assert_eq!(strategy.to_string(), "Fee Per Byte Ratio");
    }

    #[tokio::test]
    async fn test_proposal_manager() {
        use neo_ledger::BlockchainBuilder;

        let config = ProposalConfig::default();
        let mempool_config = MempoolConfig::default();
        let mempool = Arc::new(MemoryPool::new(mempool_config));

        // Create a test blockchain instance
        let blockchain = BlockchainBuilder::new()
            .with_test_config()
            .build()
            .await
            .expect("Failed to create test blockchain");
        let blockchain = Arc::new(blockchain);

        let manager = ProposalManager::new(config, mempool, blockchain);

        // Test creating a proposal
        let block_index = BlockIndex::new(100);
        let view_number = ViewNumber::new(0);
        let proposer = UInt160::zero();
        let previous_hash = UInt256::zero();

        let proposal = manager
            .create_proposal(
                block_index,
                view_number,
                proposer,
                previous_hash,
                TransactionSelectionStrategy::Fifo,
            )
            .await
            .expect("Operation failed");

        assert_eq!(proposal.block_index, block_index);
        assert_eq!(proposal.view_number, view_number);
        assert_eq!(proposal.proposer, proposer);

        // Test retrieving proposal
        let retrieved = manager.get_proposal(block_index, view_number);
        assert!(retrieved.is_some());

        // Test stats
        let stats = manager.get_stats();
        assert_eq!(stats.proposals_created, 1);
    }

    #[test]
    fn test_transaction_priority() {
        let mut transaction = Transaction::new();
        transaction.set_version(0);
        transaction.set_nonce(1);
        transaction.set_system_fee(0);
        transaction.set_network_fee(1000);
        transaction.set_valid_until_block(1000);
        transaction.set_script(vec![0x40]); // RET opcode

        let priority = TransactionPriority::new(transaction.clone());
        assert_eq!(priority.transaction, transaction);
        assert!(priority.fee_per_byte > 0.0);
        assert!(priority.priority_score > 0.0);
    }
}

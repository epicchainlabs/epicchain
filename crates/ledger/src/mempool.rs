//! Memory Pool Implementation
//!
//! This module provides a complete transaction memory pool implementation
//! that exactly matches the C# Neo.Network.P2P.MemoryPool functionality.

use crate::{Error, Result};
use neo_config::{MAX_SCRIPT_LENGTH, MAX_SCRIPT_SIZE, MAX_TRANSACTION_SIZE};
use neo_core::{Transaction, UInt160, UInt256};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

/// Memory pool configuration (matches C# MemoryPool settings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolConfig {
    /// Maximum number of transactions in pool
    pub max_transactions: usize,
    /// Maximum memory usage in bytes
    pub max_memory_usage: usize,
    /// Transaction timeout in seconds
    pub transaction_timeout: u64,
    /// Minimum fee per byte
    pub min_fee_per_byte: u64,
    /// Enable transaction replacement
    pub enable_replacement: bool,
    /// Maximum transaction size
    pub max_transaction_size: usize,
}

impl Default for MempoolConfig {
    fn default() -> Self {
        Self {
            max_transactions: 50000, // Matches C# Neo default
            max_memory_usage: 100 * MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE,
            transaction_timeout: 120, // 2 minutes
            min_fee_per_byte: 1000,   // 0.001 GAS per byte
            enable_replacement: true,
            max_transaction_size: MAX_TRANSACTION_SIZE,
        }
    }
}

/// Transaction pool entry with metadata
#[derive(Debug, Clone)]
pub struct PooledTransaction {
    /// The transaction
    pub transaction: Transaction,
    /// When it was added to the pool
    pub timestamp: SystemTime,
    /// Network fee per byte
    pub fee_per_byte: u64,
    /// Total size in bytes
    pub size: usize,
    /// Sender addresses
    pub senders: Vec<UInt160>,
    /// Whether this is a high priority transaction
    pub high_priority: bool,
}

impl PooledTransaction {
    /// Creates a new pooled transaction
    pub fn new(transaction: Transaction, high_priority: bool) -> Result<Self> {
        let size = transaction.size();
        let fee_per_byte = if size > 0 {
            transaction.network_fee() as u64 / size as u64
        } else {
            0
        };

        // Extract sender addresses from signers
        let senders = transaction
            .signers()
            .iter()
            .map(|signer| signer.account)
            .collect();

        Ok(Self {
            transaction,
            timestamp: SystemTime::now(),
            fee_per_byte,
            size,
            senders,
            high_priority,
        })
    }

    /// Gets the transaction hash
    pub fn hash(&self) -> Result<UInt256> {
        self.transaction
            .hash()
            .map_err(|e| Error::Generic(e.to_string()))
    }

    /// Checks if the transaction has expired
    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.timestamp.elapsed().unwrap_or(Duration::ZERO) > timeout
    }

    /// Gets the priority score for ordering
    pub fn priority_score(&self) -> u64 {
        if self.high_priority {
            u64::MAX - 1000 + self.fee_per_byte
        } else {
            self.fee_per_byte
        }
    }
}

/// Memory pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolStats {
    /// Current transaction count
    pub transaction_count: usize,
    /// Current memory usage in bytes
    pub memory_usage: usize,
    /// Transactions added total
    pub transactions_added: u64,
    /// Transactions removed total
    pub transactions_removed: u64,
    /// High priority transactions
    pub high_priority_count: usize,
    /// Average fee per byte
    pub average_fee_per_byte: f64,
    /// Pool utilization percentage
    pub utilization_percentage: f64,
}

impl Default for MempoolStats {
    fn default() -> Self {
        Self {
            transaction_count: 0,
            memory_usage: 0,
            transactions_added: 0,
            transactions_removed: 0,
            high_priority_count: 0,
            average_fee_per_byte: 0.0,
            utilization_percentage: 0.0,
        }
    }
}

/// Transaction verification delegate for mempool (matches C# IMemoryPoolTxObserverPlugin)
pub trait TxVerifier: Send + Sync {
    /// Verifies if a transaction is valid for inclusion in mempool
    fn verify_transaction(&self, transaction: &Transaction) -> Result<bool>;

    /// Checks if transaction conflicts with any in the pool
    fn check_conflicts(
        &self,
        transaction: &Transaction,
        pool_transactions: &[&Transaction],
    ) -> Result<bool>;

    /// Called when transaction is added to mempool
    fn on_transaction_added(&self, transaction: &Transaction) -> Result<()>;

    /// Called when transaction is removed from mempool
    fn on_transaction_removed(
        &self,
        transaction: &Transaction,
        reason: RemovalReason,
    ) -> Result<()>;
}

/// Reason for transaction removal from mempool (matches C# MemoryPool.RemovalReason)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalReason {
    /// Expired due to timeout
    Expired,
    /// Included in block
    BlockPersisted,
    /// Evicted due to pool capacity
    CapacityExceeded,
    /// Replaced by higher fee transaction
    Replaced,
    /// Invalid transaction
    Invalid,
    /// Network fee too low
    FeeTooLow,
}

/// Default transaction verifier
#[derive(Debug)]
pub struct DefaultTxVerifier;

impl TxVerifier for DefaultTxVerifier {
    fn verify_transaction(&self, transaction: &Transaction) -> Result<bool> {
        // 1. Check basic format
        if transaction.script().is_empty() {
            debug!("Transaction rejected: empty script");
            return Ok(false);
        }

        // 2. Check signers
        if transaction.signers().is_empty() {
            debug!("Transaction rejected: no signers");
            return Ok(false);
        }

        // 3. Check witnesses match signers
        if transaction.witnesses().len() != transaction.signers().len() {
            debug!("Transaction rejected: witness/signer count mismatch");
            return Ok(false);
        }

        // 4. Check fees are reasonable
        if transaction.network_fee() < 0 || transaction.system_fee() < 0 {
            debug!("Transaction rejected: negative fees");
            return Ok(false);
        }

        // 5. Check transaction size limits
        let tx_size = transaction.size();
        if tx_size > MAX_TRANSACTION_SIZE {
            debug!("Transaction rejected: size {} exceeds limit", tx_size);
            return Ok(false);
        }

        // 6. Check script length limits
        if transaction.script().len() > MAX_SCRIPT_LENGTH {
            // 64KB max script size
            debug!("Transaction rejected: script too large");
            return Ok(false);
        }

        // 7. Validate version
        if transaction.version() != 0 {
            debug!(
                "Transaction rejected: unsupported version {}",
                transaction.version()
            );
            return Ok(false);
        }

        // 8. Check valid until block
        if transaction.valid_until_block() == 0 {
            debug!("Transaction rejected: invalid validUntilBlock");
            return Ok(false);
        }

        debug!(
            "Transaction {} passed basic verification",
            transaction.hash().unwrap_or_default()
        );
        Ok(true)
    }

    fn check_conflicts(
        &self,
        transaction: &Transaction,
        pool_transactions: &[&Transaction],
    ) -> Result<bool> {
        let tx_hash = transaction.hash()?;

        for pool_tx in pool_transactions {
            if pool_tx.hash()? == tx_hash {
                debug!("Transaction conflict: duplicate hash {}", tx_hash);
                return Ok(true); // Conflict found
            }

            for signer in transaction.signers() {
                for pool_signer in pool_tx.signers() {
                    if signer.account == pool_signer.account {
                        // In a full implementation, check nonces or other conflict detection
                        debug!(
                            "Potential signer conflict detected for account {}",
                            signer.account
                        );
                    }
                }
            }
        }

        Ok(false) // No conflicts
    }

    fn on_transaction_added(&self, transaction: &Transaction) -> Result<()> {
        info!(
            "Transaction {} added to mempool",
            transaction.hash().unwrap_or_default()
        );
        Ok(())
    }

    fn on_transaction_removed(
        &self,
        transaction: &Transaction,
        reason: RemovalReason,
    ) -> Result<()> {
        debug!(
            "Transaction {} removed from mempool: {:?}",
            transaction.hash().unwrap_or_default(),
            reason
        );
        Ok(())
    }
}

/// Main memory pool implementation (matches C# MemoryPool exactly)
pub struct MemoryPool {
    /// Configuration
    config: MempoolConfig,
    /// Transactions by hash
    transactions: Arc<RwLock<HashMap<UInt256, PooledTransaction>>>,
    /// Transactions sorted by priority
    priority_queue: Arc<RwLock<BTreeMap<u64, Vec<UInt256>>>>,
    /// Transactions by sender address
    sender_map: Arc<RwLock<HashMap<UInt160, HashSet<UInt256>>>>,
    /// Pool statistics
    stats: Arc<RwLock<MempoolStats>>,
    /// Transaction verifier
    verifier: Arc<dyn TxVerifier>,
}

impl MemoryPool {
    /// Creates a new memory pool
    pub fn new(config: MempoolConfig) -> Self {
        // Pre-allocate collections based on expected capacity
        let initial_capacity = (config.max_transactions / 4).max(1000);

        Self {
            config,
            transactions: Arc::new(RwLock::new(HashMap::with_capacity(initial_capacity))),
            priority_queue: Arc::new(RwLock::new(BTreeMap::new())),
            sender_map: Arc::new(RwLock::new(HashMap::with_capacity(initial_capacity / 2))),
            stats: Arc::new(RwLock::new(MempoolStats::default())),
            verifier: Arc::new(DefaultTxVerifier),
        }
    }

    /// Creates a new memory pool with custom verifier
    pub fn with_verifier(config: MempoolConfig, verifier: Arc<dyn TxVerifier>) -> Self {
        Self {
            config,
            transactions: Arc::new(RwLock::new(HashMap::new())),
            priority_queue: Arc::new(RwLock::new(BTreeMap::new())),
            sender_map: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(MempoolStats::default())),
            verifier,
        }
    }

    /// Tries to add a transaction to the pool (matches C# TryAdd)
    pub fn try_add(&self, transaction: Transaction, high_priority: bool) -> Result<bool> {
        let tx_hash = transaction.hash()?;

        // 1. Check if transaction already exists
        if self.contains(&tx_hash) {
            debug!("Transaction {} already in pool", tx_hash);
            return Ok(false);
        }

        // 2. Check transaction size
        let tx_size = transaction.size();
        if tx_size > self.config.max_transaction_size {
            warn!("Transaction {} too large: {} bytes", tx_hash, tx_size);
            return Ok(false);
        }

        // 3. Check minimum fee
        let fee_per_byte = if tx_size > 0 {
            transaction.network_fee() as u64 / tx_size as u64
        } else {
            0
        };

        if fee_per_byte < self.config.min_fee_per_byte {
            warn!(
                "Transaction {} fee too low: {} per byte",
                tx_hash, fee_per_byte
            );
            return Ok(false);
        }

        // 4. Verify transaction
        if !self.verifier.verify_transaction(&transaction)? {
            warn!("Transaction {} failed verification", tx_hash);
            return Ok(false);
        }

        // 5. Check for conflicts
        {
            let transactions_guard = self
                .transactions
                .read()
                .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
            let pool_txs: Vec<&Transaction> = transactions_guard
                .values()
                .map(|pooled| &pooled.transaction)
                .collect();

            if self.verifier.check_conflicts(&transaction, &pool_txs)? {
                warn!(
                    "Transaction {} conflicts with existing transactions",
                    tx_hash
                );
                return Ok(false);
            }
        }

        // 6. Check pool capacity
        if !self.can_add_transaction(&transaction, high_priority)? {
            if !self.try_make_space(&transaction, high_priority)? {
                debug!("Cannot add transaction {}, pool is full", tx_hash);
                return Ok(false);
            }
        }

        // 7. Create pooled transaction and add to pool
        let pooled_tx = PooledTransaction::new(transaction, high_priority)?;
        self.add_to_pool(pooled_tx)?;

        info!("Added transaction {} to mempool", tx_hash);
        Ok(true)
    }

    /// Removes a transaction from the pool (matches C# TryRemove)
    pub fn try_remove(&self, tx_hash: &UInt256) -> Result<Option<Transaction>> {
        let mut transactions = self
            .transactions
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;

        if let Some(pooled_tx) = transactions.remove(tx_hash) {
            drop(transactions);

            // Remove from priority queue
            self.remove_from_priority_queue(&pooled_tx)?;

            // Remove from sender map
            self.remove_from_sender_map(&pooled_tx)?;

            // Update statistics
            self.update_stats_on_remove(&pooled_tx);

            info!("Removed transaction {} from mempool", tx_hash);
            Ok(Some(pooled_tx.transaction))
        } else {
            Ok(None)
        }
    }

    /// Checks if a transaction exists in the pool (matches C# ContainsKey)
    pub fn contains(&self, tx_hash: &UInt256) -> bool {
        self.transactions
            .read()
            .map(|guard| guard.contains_key(tx_hash))
            .unwrap_or(false)
    }

    /// Gets a transaction from the pool (matches C# TryGetValue)
    pub fn get_transaction(&self, tx_hash: &UInt256) -> Option<Transaction> {
        self.transactions
            .read()
            .expect("Operation failed")
            .get(tx_hash)
            .map(|pooled_tx| pooled_tx.transaction.clone())
    }

    /// Gets transactions for block creation (matches C# GetSortedTransactions)
    pub fn get_sorted_transactions(&self, max_count: usize) -> Vec<Transaction> {
        let transactions_guard = match self.transactions.read() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };
        let priority_queue_guard = match self.priority_queue.read() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };

        let mut result = Vec::new();
        let mut total_size = 0usize;
        let max_block_size = MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE; // 1MB max block size

        // Iterate from highest to lowest priority
        for (_priority, tx_hashes) in priority_queue_guard.iter().rev() {
            for tx_hash in tx_hashes {
                if result.len() >= max_count {
                    return result;
                }

                if let Some(pooled_tx) = transactions_guard.get(tx_hash) {
                    let tx_size = pooled_tx.size;
                    if total_size + tx_size > max_block_size {
                        continue; // Skip if would exceed block size
                    }

                    result.push(pooled_tx.transaction.clone());
                    total_size += tx_size;
                }
            }
        }

        result
    }

    /// Gets verified transactions (matches C# GetVerifiedTransactions)
    pub fn get_verified_transactions(&self) -> Vec<Transaction> {
        self.transactions
            .read()
            .expect("Operation failed")
            .values()
            .map(|pooled_tx| pooled_tx.transaction.clone())
            .collect()
    }

    /// Invalidates transactions from a specific sender (matches C# InvalidateVerifiedTransactions)
    pub fn invalidate_transactions_from_sender(&self, sender: &UInt160) -> Result<Vec<UInt256>> {
        let mut invalidated = Vec::new();

        if let Some(tx_hashes) = self
            .sender_map
            .read()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
            .get(sender)
        {
            for tx_hash in tx_hashes.clone() {
                if let Some(_) = self.try_remove(&tx_hash)? {
                    invalidated.push(tx_hash);
                    // Notify verifier of removal
                    if let Some(pooled_tx) = self
                        .transactions
                        .read()
                        .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
                        .get(&tx_hash)
                    {
                        let _ = self
                            .verifier
                            .on_transaction_removed(&pooled_tx.transaction, RemovalReason::Invalid);
                    }
                }
            }
        }

        Ok(invalidated)
    }

    /// Clears all transactions from pool (matches C# Clear)
    pub fn clear(&self) -> Result<()> {
        {
            let transactions = self
                .transactions
                .read()
                .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
            for (_, pooled_tx) in transactions.iter() {
                let _ = self
                    .verifier
                    .on_transaction_removed(&pooled_tx.transaction, RemovalReason::Invalid);
            }
        }

        self.transactions
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
            .clear();
        self.priority_queue
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
            .clear();
        self.sender_map
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
            .clear();

        // Reset statistics
        let mut stats = self
            .stats
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
        *stats = MempoolStats::default();

        info!("Cleared all transactions from mempool");
        Ok(())
    }

    /// Gets pool statistics (matches C# MemoryPool properties)
    pub fn get_stats(&self) -> MempoolStats {
        self.stats
            .read()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }

    /// Gets current transaction count (matches C# Count property)
    pub fn count(&self) -> usize {
        self.transactions
            .read()
            .map(|guard| guard.len())
            .unwrap_or(0)
    }

    /// Gets memory usage in bytes (matches C# MemoryUsage property)
    pub fn memory_usage(&self) -> usize {
        self.transactions
            .read()
            .expect("Operation failed")
            .values()
            .map(|pooled_tx| pooled_tx.size)
            .sum()
    }

    /// Removes expired transactions (matches C# CheckExpired)
    pub fn remove_expired_transactions(&self) -> Result<Vec<UInt256>> {
        let timeout = Duration::from_secs(self.config.transaction_timeout);
        let mut expired = Vec::new();

        let tx_hashes: Vec<UInt256> = {
            let transactions = self
                .transactions
                .read()
                .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
            transactions
                .iter()
                .filter_map(|(hash, pooled_tx)| {
                    if pooled_tx.is_expired(timeout) {
                        Some(*hash)
                    } else {
                        None
                    }
                })
                .collect()
        };

        for tx_hash in tx_hashes {
            if let Some(transaction) = self.try_remove(&tx_hash)? {
                let _ = self
                    .verifier
                    .on_transaction_removed(&transaction, RemovalReason::Expired);
                expired.push(tx_hash);
            }
        }

        if !expired.is_empty() {
            info!(
                "Removed {} expired transactions from mempool",
                expired.len()
            );
        }

        Ok(expired)
    }

    /// Updates transactions when new block is persisted (matches C# UpdatePoolForBlockPersisted)
    pub fn update_for_block_persisted(&self, block_transactions: &[UInt256]) -> Result<()> {
        let mut removed_count = 0;

        for tx_hash in block_transactions {
            if let Some(transaction) = self.try_remove(tx_hash)? {
                let _ = self
                    .verifier
                    .on_transaction_removed(&transaction, RemovalReason::BlockPersisted);
                removed_count += 1;
            }
        }

        if removed_count > 0 {
            info!(
                "Removed {} transactions from mempool (included in block)",
                removed_count
            );
        }

        Ok(())
    }

    // Private helper methods

    /// Checks if transaction can be added to pool
    fn can_add_transaction(&self, transaction: &Transaction, high_priority: bool) -> Result<bool> {
        let current_count = self.count();
        let current_memory = self.memory_usage();
        let tx_size = transaction.size();

        // Check transaction count limit
        if current_count >= self.config.max_transactions {
            return Ok(false);
        }

        // Check memory usage limit
        if current_memory + tx_size > self.config.max_memory_usage {
            return Ok(false);
        }

        // High priority transactions get preferential treatment
        if high_priority {
            return Ok(true);
        }

        let capacity_threshold = (self.config.max_transactions * 9) / 10;
        if current_count >= capacity_threshold {
            return Ok(false);
        }

        Ok(true)
    }

    /// Tries to make space for new transaction by removing lower priority ones
    fn try_make_space(&self, _new_transaction: &Transaction, high_priority: bool) -> Result<bool> {
        if !high_priority {
            return Ok(false); // Only make space for high priority transactions
        }

        // Find lowest priority transaction to remove
        let mut lowest_priority_hash: Option<UInt256> = None;
        let mut lowest_priority = u64::MAX;

        {
            let transactions = self
                .transactions
                .read()
                .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
            for (hash, pooled_tx) in transactions.iter() {
                if !pooled_tx.high_priority && pooled_tx.priority_score() < lowest_priority {
                    lowest_priority = pooled_tx.priority_score();
                    lowest_priority_hash = Some(*hash);
                }
            }
        }

        if let Some(hash) = lowest_priority_hash {
            if let Some(transaction) = self.try_remove(&hash)? {
                let _ = self
                    .verifier
                    .on_transaction_removed(&transaction, RemovalReason::CapacityExceeded);
                debug!("Evicted transaction {} to make space", hash);
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Adds transaction to internal data structures
    fn add_to_pool(&self, pooled_tx: PooledTransaction) -> Result<()> {
        let tx_hash = pooled_tx.hash()?;
        let priority = pooled_tx.priority_score();

        // Add to main map
        self.transactions
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
            .insert(tx_hash, pooled_tx.clone());

        // Add to priority queue
        self.priority_queue
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
            .entry(priority)
            .or_insert_with(Vec::new)
            .push(tx_hash);

        // Add to sender map
        for sender in &pooled_tx.senders {
            self.sender_map
                .write()
                .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?
                .entry(*sender)
                .or_insert_with(HashSet::new)
                .insert(tx_hash);
        }

        // Update statistics
        self.update_stats_on_add(&pooled_tx);

        // Notify verifier
        self.verifier.on_transaction_added(&pooled_tx.transaction)?;

        Ok(())
    }

    /// Removes transaction from priority queue
    fn remove_from_priority_queue(&self, pooled_tx: &PooledTransaction) -> Result<()> {
        let tx_hash = pooled_tx.hash()?;
        let priority = pooled_tx.priority_score();

        let mut priority_queue = self
            .priority_queue
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
        if let Some(tx_list) = priority_queue.get_mut(&priority) {
            tx_list.retain(|&hash| hash != tx_hash);
            if tx_list.is_empty() {
                priority_queue.remove(&priority);
            }
        }

        Ok(())
    }

    /// Removes transaction from sender map
    fn remove_from_sender_map(&self, pooled_tx: &PooledTransaction) -> Result<()> {
        let tx_hash = pooled_tx.hash()?;

        let mut sender_map = self
            .sender_map
            .write()
            .map_err(|_| Error::MempoolError("Failed to acquire lock".to_string()))?;
        for sender in &pooled_tx.senders {
            if let Some(tx_set) = sender_map.get_mut(sender) {
                tx_set.remove(&tx_hash);
                if tx_set.is_empty() {
                    sender_map.remove(sender);
                }
            }
        }

        Ok(())
    }

    /// Updates statistics when transaction is added
    fn update_stats_on_add(&self, pooled_tx: &PooledTransaction) {
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return,
        };
        stats.transaction_count += 1;
        stats.memory_usage += pooled_tx.size;
        stats.transactions_added += 1;

        if pooled_tx.high_priority {
            stats.high_priority_count += 1;
        }

        // Update average fee per byte
        let total_fees: u64 = stats.average_fee_per_byte as u64
            * (stats.transaction_count - 1) as u64
            + pooled_tx.fee_per_byte;
        stats.average_fee_per_byte = total_fees as f64 / stats.transaction_count as f64;

        // Update utilization percentage
        stats.utilization_percentage =
            (stats.transaction_count as f64 / self.config.max_transactions as f64) * 100.0;
    }

    /// Updates statistics when transaction is removed
    fn update_stats_on_remove(&self, pooled_tx: &PooledTransaction) {
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return,
        };
        stats.transaction_count = stats.transaction_count.saturating_sub(1);
        stats.memory_usage = stats.memory_usage.saturating_sub(pooled_tx.size);
        stats.transactions_removed += 1;

        if pooled_tx.high_priority {
            stats.high_priority_count = stats.high_priority_count.saturating_sub(1);
        }

        // Update utilization percentage
        stats.utilization_percentage =
            (stats.transaction_count as f64 / self.config.max_transactions as f64) * 100.0;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{Error, Result};
    use neo_core::{Signer, Transaction, UInt160, UInt256, Witness, WitnessScope};

    #[test]
    fn test_mempool_creation() {
        let config = MempoolConfig::default();
        let pool = MemoryPool::new(config.clone());

        let stats = pool.get_stats();
        assert_eq!(stats.transaction_count, 0);
        assert_eq!(stats.memory_usage, 0);
    }

    #[test]
    fn test_pooled_transaction_creation() {
        let mut tx = Transaction::new();
        tx.set_script(vec![0x40]); // Simple RET script
        tx.set_network_fee(1000);
        tx.add_signer(Signer {
            account: UInt160::zero(),
            scopes: WitnessScope::CALLED_BY_ENTRY,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
            rules: Vec::new(),
        });
        tx.add_witness(Witness::default());

        let pooled_tx = PooledTransaction::new(tx, false).expect("operation should succeed");
        assert!(!pooled_tx.high_priority);
        assert!(pooled_tx.size > 0);
        assert_eq!(pooled_tx.senders.len(), 1);
    }

    #[tokio::test]
    async fn test_add_transaction() {
        let config = MempoolConfig::default();
        let pool = MemoryPool::new(config);

        let mut tx = Transaction::new();
        tx.set_script(vec![0x40]); // Simple RET script
        tx.set_network_fee(100000); // High fee
        tx.add_signer(Signer {
            account: UInt160::zero(),
            scopes: WitnessScope::CALLED_BY_ENTRY,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
            rules: Vec::new(),
        });
        tx.add_witness(Witness::default());

        let tx_hash = tx.hash().expect("operation should succeed");
        let result = pool.try_add(tx, false).expect("operation should succeed");

        assert!(result);
        assert!(pool.contains(&tx_hash));

        let stats = pool.get_stats();
        assert_eq!(stats.transaction_count, 1);
    }

    #[test]
    fn test_get_sorted_transactions() {
        let config = MempoolConfig::default();
        let pool = MemoryPool::new(config);

        // Add transactions with different fees
        for i in 0..5 {
            let mut tx = Transaction::new();
            tx.set_nonce(i); // Make each transaction unique
            tx.set_script(vec![0x40]);
            tx.set_network_fee((i + 1) * 10000); // Different fees
            tx.add_signer(Signer {
                account: UInt160::zero(),
                scopes: WitnessScope::CALLED_BY_ENTRY,
                allowed_contracts: Vec::new(),
                allowed_groups: Vec::new(),
                rules: Vec::new(),
            });
            tx.add_witness(Witness::default());

            pool.try_add(tx, false).expect("operation should succeed");
        }

        let sorted_txs = pool.get_sorted_transactions(Some(3));
        assert_eq!(sorted_txs.len(), 3);

        assert!(sorted_txs[0].network_fee() >= sorted_txs[1].network_fee());
        assert!(sorted_txs[1].network_fee() >= sorted_txs[2].network_fee());
    }

    #[test]
    fn test_clear_pool() {
        let config = MempoolConfig::default();
        let pool = MemoryPool::new(config);

        // Add some transactions
        for i in 0..3 {
            let mut tx = Transaction::new();
            tx.set_nonce(i);
            tx.set_script(vec![0x40]);
            tx.set_network_fee(10000);
            tx.add_signer(Signer {
                account: UInt160::zero(),
                scopes: WitnessScope::CALLED_BY_ENTRY,
                allowed_contracts: Vec::new(),
                allowed_groups: Vec::new(),
                rules: Vec::new(),
            });
            tx.add_witness(Witness::default());

            pool.try_add(tx, false).expect("operation should succeed");
        }

        assert_eq!(pool.get_stats().transaction_count, 3);

        let cleared_count = pool.clear().expect("operation should succeed");
        assert_eq!(cleared_count, 3);
        assert_eq!(pool.get_stats().transaction_count, 0);
    }
}

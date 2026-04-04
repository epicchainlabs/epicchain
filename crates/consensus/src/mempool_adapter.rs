//! Mempool adapter for consensus integration.
//!
//! This module provides an adapter that implements the MempoolService trait
//! for the ledger's MemoryPool, avoiding circular dependencies.

use crate::service::MempoolService;
use neo_core::{Transaction, UInt256};
use neo_ledger::MemoryPool;
use std::sync::Arc;

/// Adapter that wraps MemoryPool to implement MempoolService
pub struct MempoolAdapter {
    inner: Arc<MemoryPool>,
}

impl MempoolAdapter {
    /// Creates a new mempool adapter
    pub fn new(mempool: Arc<MemoryPool>) -> Self {
        Self { inner: mempool }
    }

    /// Gets the underlying mempool reference
    pub fn inner(&self) -> &Arc<MemoryPool> {
        &self.inner
    }
}

#[async_trait::async_trait]
impl MempoolService for MempoolAdapter {
    async fn get_verified_transactions(&self, count: usize) -> Vec<Transaction> {
        self.inner.get_sorted_transactions(count)
    }

    async fn contains_transaction(&self, hash: &UInt256) -> bool {
        self.inner.contains(hash)
    }

    async fn add_transaction(&self, tx: Transaction) -> crate::Result<()> {
        self.inner
            .try_add(tx, false)
            .map_err(|e| crate::Error::Generic(e.to_string()))
            .map(|_| ())
    }

    async fn remove_transaction(&self, hash: &UInt256) -> crate::Result<()> {
        self.inner
            .try_remove(hash)
            .map_err(|e| crate::Error::Generic(e.to_string()))
            .map(|_| ())
    }

    async fn get_transaction_count(&self) -> usize {
        self.inner.get_stats().transaction_count
    }

    async fn clear(&self) -> crate::Result<()> {
        self.inner
            .clear()
            .map_err(|e| crate::Error::Generic(e.to_string()))
            .map(|_| ())
    }
}

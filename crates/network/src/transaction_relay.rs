//! Transaction Relay Implementation
//!
//! This module provides transaction relay functionality for the Neo blockchain,
//! handling transaction broadcasting, mempool integration, and peer synchronization.

use crate::relay_cache::RelayCache;
const DEFAULT_MAX_LIMIT: usize = 100;
const DEFAULT_CHANNEL_SIZE: usize = 1000;
use crate::{InventoryItem, InventoryType, NetworkError, NetworkResult, ProtocolMessage};
use neo_config::MAX_TRANSACTION_SIZE;
use neo_core::{Transaction, UInt256};
use neo_ledger::MemoryPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};
/// Transaction relay events
#[derive(Debug, Clone)]
/// Represents an enumeration of values.
pub enum TransactionRelayEvent {
    /// New transaction received and validated
    TransactionReceived {
        /// Transaction hash
        transaction_hash: UInt256,
        /// Source peer address
        from_peer: SocketAddr,
        /// Whether transaction was relayed
        relayed: bool,
    },
    /// Transaction added to mempool
    TransactionAddedToMempool {
        /// Transaction hash
        transaction_hash: UInt256,
        /// Fee per byte
        fee_per_byte: u64,
    },
    /// Transaction rejected
    TransactionRejected {
        /// Transaction hash
        transaction_hash: UInt256,
        /// Reason for action
        reason: String,
        /// Source peer address
        from_peer: SocketAddr,
    },
    /// Inventory broadcast
    InventoryBroadcast {
        /// Number of inventory items
        inventory_count: usize,
        /// List of excluded peers
        excluded_peers: Vec<SocketAddr>,
    },
}

/// Transaction relay configuration
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct TransactionRelayConfig {
    /// Maximum transactions to relay per batch
    pub max_relay_batch_size: usize,
    /// Relay cache capacity
    pub relay_cache_capacity: usize,
    /// Relay cache TTL in seconds
    pub relay_cache_ttl: u64,
    /// Enable transaction validation before relay
    pub enable_validation: bool,
    /// Maximum transaction size to relay
    pub max_transaction_size: usize,
}

impl Default for TransactionRelayConfig {
    fn default() -> Self {
        Self {
            max_relay_batch_size: DEFAULT_MAX_LIMIT,
            relay_cache_capacity: DEFAULT_CHANNEL_SIZE,
            relay_cache_ttl: 300, // 5 minutes
            enable_validation: true,
            max_transaction_size: MAX_TRANSACTION_SIZE,
        }
    }
}

/// Transaction relay handler for P2P network
/// Represents a data structure.
pub struct TransactionRelay {
    /// Configuration
    config: TransactionRelayConfig,
    /// Memory pool for transaction storage
    mempool: Arc<RwLock<MemoryPool>>,
    /// Relay cache to prevent re-broadcasting
    relay_cache: Arc<RwLock<RelayCache<UInt256>>>,
    /// Connected peers for broadcasting
    connected_peers: Arc<RwLock<HashMap<SocketAddr, PeerConnection>>>,
    /// Event broadcaster
    event_tx: broadcast::Sender<TransactionRelayEvent>,
    /// Transaction relay statistics
    relay_stats: Arc<RwLock<RelayStatistics>>,
}

/// Peer connection info for transaction relay
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct PeerConnection {
    /// Peer address
    pub address: SocketAddr,
    /// Whether peer accepts transaction relay
    pub relay_enabled: bool,
    /// Last activity time
    pub last_activity: std::time::SystemTime,
    /// Message sender channel
    pub message_sender: tokio::sync::mpsc::UnboundedSender<ProtocolMessage>,
}

/// Transaction relay statistics
#[derive(Debug, Clone, Default)]
/// Represents a data structure.
pub struct RelayStatistics {
    /// Total transactions received
    pub transactions_received: u64,
    /// Total transactions validated
    pub transactions_validated: u64,
    /// Total transactions added to mempool
    pub transactions_added_to_mempool: u64,
    /// Total transactions relayed
    pub transactions_relayed: u64,
    /// Total transactions rejected
    pub transactions_rejected: u64,
    /// Total inventory requests handled
    pub inventory_requests_handled: u64,
    /// Total get data requests handled
    pub get_data_requests_handled: u64,
}

impl TransactionRelay {
    /// Creates a new transaction relay handler
    /// Creates a new instance.
    pub fn new(config: TransactionRelayConfig, mempool: Arc<RwLock<MemoryPool>>) -> Self {
        let relay_cache = Arc::new(RwLock::new(RelayCache::new(
            config.relay_cache_capacity,
            config.relay_cache_ttl,
        )));
        let (event_tx, _) = broadcast::channel(1000);

        Self {
            config,
            mempool,
            relay_cache,
            connected_peers: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            relay_stats: Arc::new(RwLock::new(RelayStatistics::default())),
        }
    }

    /// Handles incoming transaction message
    ///
    /// # Arguments
    /// * `transaction` - The transaction to handle
    /// * `from_peer` - Source peer address
    pub async fn handle_transaction(
        &self,
        transaction: Transaction,
        from_peer: SocketAddr,
    ) -> NetworkResult<()> {
        let tx_hash = transaction
            .hash()
            .map_err(|e| NetworkError::TransactionValidation {
                hash: UInt256::zero(),
                reason: format!("Failed to compute transaction hash: {}", e),
            })?;
        debug!("Received transaction {} from peer {}", tx_hash, from_peer);

        // Update statistics
        {
            let mut stats = self.relay_stats.write().await;
            stats.transactions_received += 1;
        }

        {
            let relay_cache = self.relay_cache.read().await;
            if relay_cache.contains(&tx_hash) {
                debug!("Transaction {} already seen, ignoring", tx_hash);
                return Ok(());
            }
        }

        if self.config.enable_validation {
            if let Err(e) = self.validate_transaction(&transaction).await {
                warn!("Transaction {} validation failed: {}", tx_hash, e);

                // Update statistics
                {
                    let mut stats = self.relay_stats.write().await;
                    stats.transactions_rejected += 1;
                }

                // Broadcast rejection event
                let _ = self
                    .event_tx
                    .send(TransactionRelayEvent::TransactionRejected {
                        transaction_hash: tx_hash,
                        reason: e.to_string(),
                        from_peer,
                    });

                return Err(e);
            }
        }

        // Update validation statistics
        {
            let mut stats = self.relay_stats.write().await;
            stats.transactions_validated += 1;
        }

        // Try to add to mempool
        let mut should_relay = false;
        {
            let mempool = self.mempool.read().await;
            match mempool.try_add(transaction.clone(), false) {
                Ok(_) => {
                    info!("Transaction {} added to mempool", tx_hash);
                    should_relay = true;

                    // Update statistics
                    {
                        let mut stats = self.relay_stats.write().await;
                        stats.transactions_added_to_mempool += 1;
                    }

                    // Broadcast mempool addition event
                    let fee_per_byte = self.calculate_fee_per_byte(&transaction);
                    let _ = self
                        .event_tx
                        .send(TransactionRelayEvent::TransactionAddedToMempool {
                            transaction_hash: tx_hash,
                            fee_per_byte,
                        });
                }
                Err(e) => {
                    warn!("Failed to add transaction {} to mempool: {}", tx_hash, e);

                    // Update statistics
                    {
                        let mut stats = self.relay_stats.write().await;
                        stats.transactions_rejected += 1;
                    }

                    // Broadcast rejection event
                    let _ = self
                        .event_tx
                        .send(TransactionRelayEvent::TransactionRejected {
                            transaction_hash: tx_hash,
                            reason: e.to_string(),
                            from_peer,
                        });
                }
            }
        }

        // Add to relay cache to prevent re-processing
        {
            let mut relay_cache = self.relay_cache.write().await;
            relay_cache.insert(tx_hash);
        }

        if should_relay {
            self.relay_transaction_to_peers(tx_hash, from_peer).await?;

            // Update relay statistics
            {
                let mut stats = self.relay_stats.write().await;
                stats.transactions_relayed += 1;
            }
        }

        // Broadcast transaction received event
        let _ = self
            .event_tx
            .send(TransactionRelayEvent::TransactionReceived {
                transaction_hash: tx_hash,
                from_peer,
                relayed: should_relay,
            });

        Ok(())
    }

    /// Handles inventory message (transaction announcements)
    ///
    /// # Arguments
    /// * `inventory` - Vector of inventory items to process
    /// * `from_peer` - Source peer address
    pub async fn handle_inventory(
        &self,
        inventory: Vec<InventoryItem>,
        from_peer: SocketAddr,
    ) -> NetworkResult<()> {
        debug!(
            "Received inventory with {} items from peer {}",
            inventory.len(),
            from_peer
        );

        // Update statistics
        {
            let mut stats = self.relay_stats.write().await;
            stats.inventory_requests_handled += 1;
        }

        let mut missing_transactions = Vec::new();

        for item in inventory {
            if item.item_type == InventoryType::Transaction {
                let mempool = self.mempool.read().await;
                if !mempool.contains(&item.hash) {
                    // Check relay cache too
                    let relay_cache = self.relay_cache.read().await;
                    if !relay_cache.contains(&item.hash) {
                        missing_transactions.push(item);
                    }
                }
            }
        }

        // Request missing transactions
        if !missing_transactions.is_empty() {
            debug!(
                "Requesting {} missing transactions from peer {}",
                missing_transactions.len(),
                from_peer
            );

            self.send_get_data_to_peer(from_peer, missing_transactions)
                .await?;
        }

        Ok(())
    }

    /// Handles get data requests
    ///
    /// # Arguments
    /// * `requested_items` - Vector of inventory items requested by peer
    /// * `from_peer` - Source peer address
    pub async fn handle_get_data(
        &self,
        requested_items: Vec<InventoryItem>,
        from_peer: SocketAddr,
    ) -> NetworkResult<()> {
        debug!(
            "Received get data request for {} items from peer {}",
            requested_items.len(),
            from_peer
        );

        // Update statistics
        {
            let mut stats = self.relay_stats.write().await;
            stats.get_data_requests_handled += 1;
        }

        let mut found_transactions = Vec::new();
        let mut not_found_items = Vec::new();

        // Look up requested transactions in mempool
        {
            let mempool = self.mempool.read().await;

            for item in requested_items {
                if item.item_type == InventoryType::Transaction {
                    if let Some(transaction) = mempool.get_transaction(&item.hash) {
                        found_transactions.push(transaction);
                    } else {
                        not_found_items.push(item);
                    }
                }
            }
        }

        // Send found transactions
        for transaction in found_transactions {
            self.send_transaction_to_peer(from_peer, transaction)
                .await?;
        }

        if !not_found_items.is_empty() {
            self.send_not_found_to_peer(from_peer, not_found_items)
                .await?;
        }

        Ok(())
    }

    /// Handles mempool request
    pub async fn handle_mempool_request(&self, from_peer: SocketAddr) -> NetworkResult<()> {
        debug!("Received mempool request from peer {}", from_peer);

        // Get current mempool transactions
        let inventory_items = {
            let mempool = self.mempool.read().await;
            let transactions = mempool.get_sorted_transactions(self.config.max_relay_batch_size);

            transactions
                .into_iter()
                .filter_map(|tx| {
                    tx.hash().ok().map(|hash| InventoryItem {
                        item_type: InventoryType::Transaction,
                        hash,
                    })
                })
                .collect::<Vec<_>>()
        };

        // Send inventory response
        if !inventory_items.is_empty() {
            self.send_inventory_to_peer(from_peer, inventory_items)
                .await?;
        }

        Ok(())
    }

    /// Broadcasts a new transaction to all peers
    pub async fn broadcast_transaction(&self, transaction: Transaction) -> NetworkResult<()> {
        let tx_hash = transaction
            .hash()
            .map_err(|e| NetworkError::TransactionValidation {
                hash: UInt256::zero(),
                reason: format!("Failed to compute transaction hash: {}", e),
            })?;
        info!("Broadcasting transaction {}", tx_hash);

        // Add to mempool first
        {
            let mempool = self.mempool.read().await;
            mempool.try_add(transaction.clone(), false).map_err(|e| {
                NetworkError::TransactionValidation {
                    hash: tx_hash,
                    reason: e.to_string(),
                }
            })?;
        }

        // Add to relay cache
        {
            let mut relay_cache = self.relay_cache.write().await;
            relay_cache.insert(tx_hash);
        }

        // Broadcast inventory to all peers
        let inventory_item = InventoryItem {
            item_type: InventoryType::Transaction,
            hash: tx_hash,
        };

        self.broadcast_inventory_to_peers(vec![inventory_item], None)
            .await?;

        Ok(())
    }

    /// Validates a transaction
    async fn validate_transaction(&self, transaction: &Transaction) -> NetworkResult<()> {
        let tx_hash = transaction
            .hash()
            .map_err(|e| NetworkError::TransactionValidation {
                hash: UInt256::zero(),
                reason: format!("Failed to compute transaction hash: {}", e),
            })?;

        // Check transaction size
        let tx_size = transaction.size();
        if tx_size > self.config.max_transaction_size {
            return Err(NetworkError::TransactionValidation {
                hash: tx_hash,
                reason: format!("Transaction too large: {} bytes", tx_size),
            });
        }

        // Note: Full transaction verification requires blockchain snapshot
        // by the mempool when adding transactions.

        // Check basic transaction format
        if transaction.version() != 0 {
            return Err(NetworkError::TransactionValidation {
                hash: tx_hash,
                reason: "Invalid transaction version".to_string(),
            });
        }

        Ok(())
    }

    /// Calculates fee per byte for a transaction
    fn calculate_fee_per_byte(&self, transaction: &Transaction) -> u64 {
        let size = transaction.size() as u64;
        if size == 0 {
            return 0;
        }

        let fee = transaction.system_fee() as u64;
        fee / size
    }

    /// Relays transaction to all peers except the source
    async fn relay_transaction_to_peers(
        &self,
        tx_hash: UInt256,
        exclude_peer: SocketAddr,
    ) -> NetworkResult<()> {
        let inventory_item = InventoryItem {
            item_type: InventoryType::Transaction,
            hash: tx_hash,
        };

        self.broadcast_inventory_to_peers(vec![inventory_item], Some(exclude_peer))
            .await
    }

    /// Broadcasts inventory to all peers
    async fn broadcast_inventory_to_peers(
        &self,
        inventory: Vec<InventoryItem>,
        exclude_peer: Option<SocketAddr>,
    ) -> NetworkResult<()> {
        let peers = self.connected_peers.read().await;
        let mut excluded_peers = Vec::new();

        for (peer_addr, peer_conn) in peers.iter() {
            // Skip excluded peer
            if let Some(excluded) = exclude_peer {
                if *peer_addr == excluded {
                    excluded_peers.push(*peer_addr);
                    continue;
                }
            }

            // Skip peers that don't want relay
            if !peer_conn.relay_enabled {
                continue;
            }

            // Send inventory message
            let message = ProtocolMessage::Inv {
                inventory: inventory.clone(),
            };

            if let Err(e) = peer_conn.message_sender.send(message) {
                warn!("Failed to send inventory to peer {}: {}", peer_addr, e);
            }
        }

        // Broadcast inventory event
        let _ = self
            .event_tx
            .send(TransactionRelayEvent::InventoryBroadcast {
                inventory_count: inventory.len(),
                excluded_peers,
            });

        Ok(())
    }

    /// Sends get data request to a specific peer
    async fn send_get_data_to_peer(
        &self,
        peer_addr: SocketAddr,
        items: Vec<InventoryItem>,
    ) -> NetworkResult<()> {
        let peers = self.connected_peers.read().await;

        if let Some(peer_conn) = peers.get(&peer_addr) {
            let message = ProtocolMessage::GetData { inventory: items };

            peer_conn
                .message_sender
                .send(message)
                .map_err(|e| NetworkError::MessageSend {
                    peer: peer_addr,
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    /// Sends transaction to a specific peer
    async fn send_transaction_to_peer(
        &self,
        peer_addr: SocketAddr,
        transaction: Transaction,
    ) -> NetworkResult<()> {
        let peers = self.connected_peers.read().await;

        if let Some(peer_conn) = peers.get(&peer_addr) {
            let message = ProtocolMessage::Tx { transaction };

            peer_conn
                .message_sender
                .send(message)
                .map_err(|e| NetworkError::MessageSend {
                    peer: peer_addr,
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    /// Sends inventory to a specific peer
    async fn send_inventory_to_peer(
        &self,
        peer_addr: SocketAddr,
        inventory: Vec<InventoryItem>,
    ) -> NetworkResult<()> {
        let peers = self.connected_peers.read().await;

        if let Some(peer_conn) = peers.get(&peer_addr) {
            let message = ProtocolMessage::Inv { inventory };

            peer_conn
                .message_sender
                .send(message)
                .map_err(|e| NetworkError::MessageSend {
                    peer: peer_addr,
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    /// Sends not found response to a specific peer
    async fn send_not_found_to_peer(
        &self,
        peer_addr: SocketAddr,
        items: Vec<InventoryItem>,
    ) -> NetworkResult<()> {
        let peers = self.connected_peers.read().await;

        if let Some(peer_conn) = peers.get(&peer_addr) {
            let message = ProtocolMessage::NotFound { inventory: items };

            peer_conn
                .message_sender
                .send(message)
                .map_err(|e| NetworkError::MessageSend {
                    peer: peer_addr,
                    reason: e.to_string(),
                })?;
        }

        Ok(())
    }

    /// Adds a peer connection for transaction relay
    pub async fn add_peer_connection(&self, peer_conn: PeerConnection) {
        let mut peers = self.connected_peers.write().await;
        peers.insert(peer_conn.address, peer_conn);
    }

    /// Removes a peer connection
    pub async fn remove_peer_connection(&self, peer_addr: SocketAddr) {
        let mut peers = self.connected_peers.write().await;
        peers.remove(&peer_addr);
    }

    /// Gets current relay statistics
    pub async fn get_statistics(&self) -> RelayStatistics {
        self.relay_stats.read().await.clone()
    }

    /// Gets event receiver for transaction relay events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<TransactionRelayEvent> {
        self.event_tx.subscribe()
    }

    /// Cleans up expired entries from relay cache
    pub async fn cleanup_relay_cache(&self) {
        let mut relay_cache = self.relay_cache.write().await;
        relay_cache.cleanup_expired();
    }
}

impl TransactionRelay {
    /// Gets the number of connected peers
    pub async fn get_connected_peer_count(&self) -> usize {
        let peers = self.connected_peers.read().await;
        peers.len()
    }

    /// Gets mempool transaction count
    pub async fn get_mempool_transaction_count(&self) -> usize {
        let mempool = self.mempool.read().await;
        mempool.count()
    }
}

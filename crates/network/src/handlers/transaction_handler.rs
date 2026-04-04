//! Transaction Message Handler
//!
//! This module provides message handling for transaction-related P2P messages.

use crate::p2p::protocol::MessageHandler;
use crate::{NetworkMessage, NetworkResult, ProtocolMessage, TransactionRelay};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, warn};

/// Transaction message handler for P2P network
pub struct TransactionMessageHandler {
    /// Transaction relay manager
    transaction_relay: Arc<TransactionRelay>,
}

impl TransactionMessageHandler {
    /// Creates a new transaction message handler
    pub fn new(transaction_relay: Arc<TransactionRelay>) -> Self {
        Self { transaction_relay }
    }
}

#[async_trait::async_trait]
impl MessageHandler for TransactionMessageHandler {
    /// Handles incoming network messages
    async fn handle_message(
        &self,
        peer_address: SocketAddr,
        message: &NetworkMessage,
    ) -> NetworkResult<()> {
        // Extract protocol message from network message
        let _protocol_message = match &message.payload {
            ProtocolMessage::Tx { transaction } => {
                debug!("Handling transaction message from peer {}", peer_address);
                self.transaction_relay
                    .handle_transaction(transaction.clone(), peer_address)
                    .await?;
                return Ok(());
            }

            ProtocolMessage::Inv { inventory } => {
                debug!("Handling inventory message from peer {}", peer_address);
                self.transaction_relay
                    .handle_inventory(inventory.clone(), peer_address)
                    .await?;
                return Ok(());
            }

            ProtocolMessage::GetData { inventory } => {
                debug!("Handling get data message from peer {}", peer_address);
                self.transaction_relay
                    .handle_get_data(inventory.clone(), peer_address)
                    .await?;
                return Ok(());
            }

            ProtocolMessage::Mempool => {
                debug!("Handling mempool request from peer {}", peer_address);
                self.transaction_relay
                    .handle_mempool_request(peer_address)
                    .await?;
                return Ok(());
            }

            ProtocolMessage::NotFound { inventory: _ } => {
                debug!("Received not found message from peer {}", peer_address);
                // Handle not found - could update request tracking
                return Ok(());
            }

            _ => {
                // Not a transaction-related message, ignore
                debug!(
                    "Ignoring non-transaction message from peer {}",
                    peer_address
                );
                return Ok(());
            }
        };
    }
}

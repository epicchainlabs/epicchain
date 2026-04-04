#!/bin/bash
# Network Module Warning Cleanup Script

echo "🧹 Cleaning up network module warnings..."

# Fix unused imports
echo "Fixing unused imports..."

# Remove unused config imports
sed -i '/use neo_config::DEFAULT_NEO_PORT;/d' crates/network/src/server.rs
sed -i '/use neo_config::DEFAULT_RPC_PORT;/d' crates/network/src/server.rs  
sed -i '/use neo_config::DEFAULT_TESTNET_PORT;/d' crates/network/src/server.rs
sed -i '/use neo_config::DEFAULT_TESTNET_RPC_PORT;/d' crates/network/src/server.rs

# Fix shutdown_impl unused imports
sed -i 's/use crate::{NetworkError, NetworkResult as Result, P2pNode, PeerManager, SyncManager};/use crate::{P2pNode, PeerManager, SyncManager};/' crates/network/src/shutdown_impl.rs
sed -i '/use neo_config::DEFAULT_NEO_PORT;/d' crates/network/src/shutdown_impl.rs
sed -i '/use neo_config::DEFAULT_RPC_PORT;/d' crates/network/src/shutdown_impl.rs
sed -i '/use neo_config::DEFAULT_TESTNET_PORT;/d' crates/network/src/shutdown_impl.rs
sed -i '/use neo_config::DEFAULT_TESTNET_RPC_PORT;/d' crates/network/src/shutdown_impl.rs

# Fix snapshot_config unused import
sed -i '/use neo_core::UInt256;/d' crates/network/src/snapshot_config.rs

# Fix sync unused import
sed -i '/use tokio::time::sleep;/d' crates/network/src/sync.rs

# Fix transaction_relay unused import
sed -i 's/use tracing::{debug, error, info, warn};/use tracing::{debug, info, warn};/' crates/network/src/transaction_relay.rs

# Fix safe_p2p unused imports
sed -i 's/use neo_core::safe_error_handling::{SafeUnwrap, SafeExpected, SafeError};/use neo_core::safe_error_handling;/' crates/network/src/safe_p2p.rs || true

# Fix lib.rs unused imports
sed -i '/use neo_config::DEFAULT_TESTNET_RPC_PORT;/d' crates/network/src/lib.rs

echo "✅ Fixed unused imports"

# Fix unused variables by prefixing with underscore
echo "Fixing unused variables..."

# Fix server.rs unused variables  
sed -i 's/if let Some(rpc_server) = &self.rpc_server {/if let Some(_rpc_server) = \&self.rpc_server {/' crates/network/src/server.rs
sed -i 's/let sync_manager = self.sync_manager.clone();/let _sync_manager = self.sync_manager.clone();/' crates/network/src/server.rs
sed -i 's/let (command_sender, command_receiver)/let (_command_sender, command_receiver)/' crates/network/src/server.rs

# Fix peer_manager.rs unused variables
sed -i 's/pub async fn complete_ping(&self, address: SocketAddr, nonce: u32)/pub async fn complete_ping(\&self, address: SocketAddr, _nonce: u32)/' crates/network/src/peer_manager.rs
sed -i 's/let event_sender = self.event_sender.clone();/let _event_sender = self.event_sender.clone();/' crates/network/src/peer_manager.rs

# Fix peers.rs unused variables
sed -i 's/pub async fn disconnect_peer(&self, address: SocketAddr, reason: String)/pub async fn disconnect_peer(\&self, address: SocketAddr, _reason: String)/' crates/network/src/peers.rs

# Fix rpc.rs unused variables
sed -i 's/state: &RpcState,/_state: \&RpcState,/' crates/network/src/rpc.rs

# Fix safe_p2p.rs unused variable
sed -i 's/mut command_receiver: mpsc::Receiver/command_receiver: mpsc::Receiver/' crates/network/src/safe_p2p.rs
sed -i 's/pub fn validate_size(&self, size: usize, context: &str)/pub fn validate_size(\&self, size: usize, _context: \&str)/' crates/network/src/safe_p2p.rs

echo "✅ Fixed unused variables"

# Fix error.rs pattern matches
echo "Fixing error pattern matches..."
sed -i 's/crate::Error::Peer(msg) =>/crate::Error::Peer(_msg) =>/' crates/network/src/error.rs
sed -i 's/crate::Error::Timeout(msg) =>/crate::Error::Timeout(_msg) =>/' crates/network/src/error.rs  
sed -i 's/crate::Error::RateLimit(msg) =>/crate::Error::RateLimit(_msg) =>/' crates/network/src/error.rs

echo "✅ Fixed error patterns"

# Fix message handling unused variables
echo "Fixing message handler variables..."
sed -i 's/let protocol_message = match &message.payload {/let _protocol_message = match \&message.payload {/' crates/network/src/handlers/transaction_handler.rs
sed -i 's/let checksum = u32::from_le_bytes/let _checksum = u32::from_le_bytes/' crates/network/src/messages/network.rs

echo "✅ Fixed message handler variables"

# Fix validation unused variables  
echo "Fixing validation variables..."
sed -i 's/timestamp: u64,/_timestamp: u64,/' crates/network/src/messages/validation.rs
sed -i 's/relay: bool,/_relay: bool,/' crates/network/src/messages/validation.rs
sed -i 's/hash_stop: &UInt256,/_hash_stop: \&UInt256,/' crates/network/src/messages/validation.rs
sed -i 's/for (i, item) in inventory.iter().enumerate() {/for (_i, item) in inventory.iter().enumerate() {/' crates/network/src/messages/validation.rs

echo "✅ Fixed validation variables"

echo "🎉 Network module warning cleanup completed!"
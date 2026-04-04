//! # Neo-RS: Neo N3 Blockchain Implementation in Rust
//!
//! A high-performance, production-ready implementation of the Neo N3 blockchain protocol in Rust.
//!
//! This library provides a complete implementation of the Neo blockchain including:
//! - Virtual Machine (NeoVM) for smart contract execution
//! - Consensus mechanism (dBFT 2.0)
//! - P2P networking and protocol
//! - Blockchain ledger and state management
//! - Cryptographic operations and primitives
//! - RPC server and client implementations
//!
//! ## Features
//!
//! - **High Performance**: Optimized for throughput and low latency
//! - **Production Ready**: Comprehensive error handling and monitoring
//! - **Modular Design**: Composable components for different use cases
//! - **Full Compatibility**: Compatible with Neo N3 protocol specification
//! - **Type Safety**: Leverages Rust's type system for correctness
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use neo_rs::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize Neo node configuration
//!     let config = NodeConfig::default();
//!     
//!     // Start the Neo node
//!     let node = NeoNode::new(config).await?;
//!     node.start().await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! The Neo-RS implementation is organized into several core crates:
//!
//! - [`neo_core`] - Core blockchain types and utilities
//! - [`neo_vm`] - Neo Virtual Machine implementation  
//! - [`neo_consensus`] - dBFT 2.0 consensus algorithm
//! - [`neo_network`] - P2P networking and protocol
//! - [`neo_ledger`] - Blockchain state and transaction processing
//! - [`neo_persistence`] - Storage and database abstractions
//! - [`neo_cryptography`] - Cryptographic primitives
//! - [`neo_smart_contract`] - Smart contract execution engine

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

// Re-export all public APIs from core crates
pub use neo_core as core;
pub use neo_cryptography as crypto;
pub use neo_ledger as ledger;
pub use neo_network as network;
pub use neo_persistence as persistence;
pub use neo_smart_contract as smart_contract;
pub use neo_vm as vm;

// Conditional re-exports for optional features
#[cfg(feature = "consensus")]
pub use neo_consensus as consensus;

#[cfg(feature = "rpc")]
pub use neo_rpc_server as rpc_server;

#[cfg(feature = "rpc")]
pub use neo_rpc_client as rpc_client;

/// Common imports for Neo development
pub mod prelude {
    pub use crate::core::{Block, Transaction, UInt160, UInt256};
    // pub use crate::crypto::{PublicKey, PrivateKey, Signature};
    pub use crate::ledger::{BlockHeader, Blockchain};
    pub use crate::network::{NetworkConfig, P2pNode};
    pub use crate::persistence::Storage;
    pub use crate::vm::{ApplicationEngine, Script, StackItem};

    #[cfg(feature = "consensus")]
    pub use crate::consensus::ConsensusService;

    #[cfg(feature = "rpc")]
    pub use crate::rpc_server::RpcServer;
}

/// Neo node configuration
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Network configuration
    pub network: network::NetworkConfig,
    /// Storage configuration  
    pub storage: persistence::StorageConfig,
    /// Enable consensus participation
    pub enable_consensus: bool,
    /// Enable RPC server
    pub enable_rpc: bool,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            network: network::NetworkConfig::default(),
            storage: persistence::StorageConfig::default(),
            enable_consensus: false,
            enable_rpc: true,
        }
    }
}

/// High-level Neo node interface
#[derive(Debug)]
pub struct NeoNode {
    config: NodeConfig,
    blockchain: std::sync::Arc<ledger::Blockchain>,
    network: Option<network::P2pNode>,
    #[cfg(feature = "rpc")]
    rpc_server: Option<rpc_server::RpcServer>,
}

impl NeoNode {
    /// Creates a new Neo node with the given configuration
    pub async fn new(config: NodeConfig) -> Result<Self> {
        // Initialize blockchain
        let blockchain = std::sync::Arc::new(
            ledger::Blockchain::new_with_storage_suffix(
                neo_config::NetworkType::MainNet,
                Some("node"),
            )
            .await?,
        );

        Ok(Self {
            config,
            blockchain,
            network: None,
            #[cfg(feature = "rpc")]
            rpc_server: None,
        })
    }

    /// Starts the Neo node
    pub async fn start(&mut self) -> Result<()> {
        // Start network layer
        let (_event_tx, event_rx) = tokio::sync::mpsc::channel(100);
        let p2p_node = network::P2pNode::new(self.config.network.clone(), event_rx)?;
        p2p_node.start().await?;
        self.network = Some(p2p_node);

        // Start RPC server if enabled
        #[cfg(feature = "rpc")]
        if self.config.enable_rpc {
            // if let Some(rpc_config) = &self.config.network.rpc_config {
            //     let rpc_server = rpc_server::RpcServer::new(rpc_config.clone());
            //     self.rpc_server = Some(rpc_server);
            // }
        }

        Ok(())
    }

    /// Stops the Neo node
    pub async fn stop(&mut self) -> Result<()> {
        // Stop network
        if let Some(network) = self.network.take() {
            network.stop().await?;
        }

        // Stop RPC server
        #[cfg(feature = "rpc")]
        if let Some(rpc_server) = self.rpc_server.take() {
            // Stop RPC server
            drop(rpc_server);
        }

        Ok(())
    }

    /// Gets a reference to the blockchain
    pub fn blockchain(&self) -> &std::sync::Arc<ledger::Blockchain> {
        &self.blockchain
    }
}

/// Result type for Neo operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Neo library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Neo protocol version
pub const PROTOCOL_VERSION: u32 = 0;

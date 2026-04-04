//! RPC Methods
//!
//! Implementation of Neo N3 RPC methods.

use hex;
use neo_config::HASH_SIZE;
use neo_ledger::Ledger;
use neo_persistence::RocksDbStore;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::warn;

// Define Error and Result types locally
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// Import RPC types from rpc_client
use neo_rpc_client::models::{
    RpcBlock, RpcPeer, RpcPeers, RpcSigner, RpcTransaction, RpcVersion, RpcWitness,
};

/// RPC methods implementation
#[derive(Clone, Debug)]
pub struct RpcMethods {
    ledger: Arc<Ledger>,
    storage: Arc<RocksDbStore>,
    peer_registry: Arc<std::sync::RwLock<PeerRegistry>>,
}

/// Production peer registry for managing real peer connections
#[derive(Debug, Default)]
pub struct PeerRegistry {
    connected_peers: HashMap<String, ConnectedPeer>,
    known_peers: HashMap<String, KnownPeer>,
}

impl PeerRegistry {
    /// Add a connected peer to the registry
    pub fn add_connected_peer(&mut self, peer: ConnectedPeer) {
        let key = format!("{}:{}", peer.address, peer.port);
        self.connected_peers.insert(key.clone(), peer.clone());

        // Update known peers list
        if let Some(known_peer) = self.known_peers.get_mut(&key) {
            known_peer.last_seen = peer.connected_at;
            known_peer.connection_attempts = 0; // Reset failed attempts on successful connection
        }
    }

    /// Remove a connected peer from the registry
    pub fn remove_connected_peer(&mut self, address: &str, port: u16) {
        let key = format!("{}:{}", address, port);
        self.connected_peers.remove(&key);
    }

    /// Add a known peer to the registry
    pub fn add_known_peer(&mut self, peer: KnownPeer) {
        let key = format!("{}:{}", peer.address, peer.port);
        self.known_peers.insert(key, peer);
    }

    /// Increment connection attempt count for a peer
    pub fn increment_connection_attempts(&mut self, address: &str, port: u16) {
        let key = format!("{}:{}", address, port);
        if let Some(peer) = self.known_peers.get_mut(&key) {
            peer.connection_attempts += 1;
        }
    }

    /// Get connection count
    pub fn get_connection_count(&self) -> usize {
        self.connected_peers.len()
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedPeer {
    pub address: String,
    pub port: u16,
    pub connected_at: std::time::SystemTime,
    pub version: u32,
    pub user_agent: String,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct KnownPeer {
    pub address: String,
    pub port: u16,
    pub last_seen: std::time::SystemTime,
    pub connection_attempts: u32,
    pub is_seed_node: bool,
}

impl RpcMethods {
    /// Creates a new RpcMethods instance
    pub fn new(ledger: Arc<Ledger>, storage: Arc<RocksDbStore>) -> Self {
        let mut peer_registry = PeerRegistry::default();

        // Initialize with known seed nodes
        let seed_nodes = vec![
            ("seed1.neo.org", 10333),
            ("seed2.neo.org", 10333),
            ("seed3.neo.org", 10333),
            ("seed4.neo.org", 10333),
            ("seed5.neo.org", 10333),
        ];

        for (address, port) in seed_nodes {
            peer_registry.known_peers.insert(
                format!("{}:{}", address, port),
                KnownPeer {
                    address: address.to_string(),
                    port,
                    last_seen: std::time::SystemTime::now(),
                    connection_attempts: 0,
                    is_seed_node: true,
                },
            );
        }

        Self {
            ledger,
            storage,
            peer_registry: Arc::new(std::sync::RwLock::new(peer_registry)),
        }
    }

    /// Converts a Block to RpcBlock
    fn block_to_rpc_block(
        &self,
        block: &neo_ledger::Block,
        confirmations: u32,
        next_block_hash: Option<neo_core::UInt256>,
    ) -> RpcBlock {
        RpcBlock {
            hash: block.hash(),
            size: block.size() as u32,
            version: block.header.version,
            merkleroot: block.header.merkle_root,
            time: block.header.timestamp,
            index: block.header.index,
            primary: block.header.primary_index,
            nextconsensus: block.header.next_consensus,
            witnesses: block
                .header
                .witnesses
                .iter()
                .map(|w| RpcWitness {
                    invocation: hex::encode(&w.invocation_script),
                    verification: hex::encode(&w.verification_script),
                })
                .collect(),
            tx: block
                .transactions
                .iter()
                .map(|tx| RpcTransaction {
                    hash: match tx.hash() {
                        Ok(hash) => hash,
                        Err(_) => neo_core::UInt256::zero(),
                    },
                    size: tx.size() as u32,
                    version: tx.version(),
                    nonce: tx.nonce(),
                    sender: tx.sender(),
                    sysfee: tx.system_fee().to_string(),
                    netfee: tx.network_fee().to_string(),
                    validuntilblock: tx.valid_until_block(),
                    signers: tx
                        .signers()
                        .iter()
                        .map(|s| RpcSigner {
                            account: s.account,
                            scopes: s.scopes.to_string(),
                            allowedcontracts: if s.allowed_contracts.is_empty() {
                                None
                            } else {
                                Some(s.allowed_contracts.clone())
                            },
                            allowedgroups: if s.allowed_groups.is_empty() {
                                None
                            } else {
                                Some(s.allowed_groups.iter().map(hex::encode).collect())
                            },
                            rules: None,
                        })
                        .collect(),
                    attributes: vec![],
                    script: hex::encode(tx.script()),
                    witnesses: tx
                        .witnesses()
                        .iter()
                        .map(|w| RpcWitness {
                            invocation: hex::encode(&w.invocation_script),
                            verification: hex::encode(&w.verification_script),
                        })
                        .collect(),
                    blockhash: None,
                    confirmations: None,
                    blocktime: None,
                })
                .collect(),
            confirmations: Some(confirmations),
            nextblockhash: next_block_hash,
        }
    }

    /// Gets the current block count
    pub async fn get_block_count(&self) -> Result<Value> {
        let height = self.ledger.get_height().await;
        Ok(json!(height + 1)) // Neo returns count (height + 1)
    }

    /// Gets a block by hash or index
    pub async fn get_block(&self, params: Option<Value>) -> Result<Value> {
        let params = params.ok_or("Missing parameters")?;
        let params_array = params.as_array().ok_or("Invalid parameters format")?;

        if params_array.is_empty() {
            return Err("Missing block identifier".into());
        }

        let block_param = &params_array[0];

        let block = if let Some(hash_str) = block_param.as_str() {
            // Try to parse as hex hash
            if let Ok(hash_bytes) = hex::decode(hash_str.trim_start_matches("0x")) {
                if hash_bytes.len() == HASH_SIZE {
                    let mut bytes = [0u8; HASH_SIZE];
                    bytes.copy_from_slice(&hash_bytes);
                    let hash = neo_core::UInt256::from(bytes);
                    self.ledger.get_block_by_hash(&hash).await?
                } else {
                    return Err("Invalid hash format".into());
                }
            } else {
                return Err("Invalid hash format".into());
            }
        } else if let Some(index) = block_param.as_u64() {
            // Parse as block index
            self.ledger.get_block(index as u32).await?
        } else {
            return Err("Invalid block identifier format".into());
        };

        match block {
            Some(block) => {
                // Calculate confirmations
                let current_height = self.ledger.get_height().await;
                let confirmations = if current_height >= block.header.index {
                    current_height - block.header.index + 1
                } else {
                    0
                };

                let next_block_hash = if block.header.index < current_height {
                    match self.ledger.get_block(block.header.index + 1).await? {
                        Some(next_block) => Some(next_block.hash()),
                        None => None,
                    }
                } else {
                    None
                };

                let rpc_block = self.block_to_rpc_block(&block, confirmations, next_block_hash);
                Ok(serde_json::to_value(rpc_block)?)
            }
            None => Err("Block not found".into()),
        }
    }

    /// Gets block hash by index
    pub async fn get_block_hash(&self, params: Option<Value>) -> Result<Value> {
        let params = params.ok_or("Missing parameters")?;
        let params_array = params.as_array().ok_or("Invalid parameters format")?;

        if params_array.is_empty() {
            return Err("Missing block index".into());
        }

        let index = params_array[0].as_u64().ok_or("Invalid block index")? as u32;

        // Get the actual block and return its hash
        match self.ledger.get_block(index).await? {
            Some(block) => {
                let hash_hex = hex::encode(block.hash().as_bytes());
                Ok(json!(format!("0x{}", hash_hex)))
            }
            None => Err("Block not found".into()),
        }
    }

    /// Gets the best block hash
    pub async fn get_best_block_hash(&self) -> Result<Value> {
        let best_hash = self.ledger.get_best_block_hash().await?;
        let hash_hex = hex::encode(best_hash.as_bytes());
        Ok(json!(format!("0x{}", hash_hex)))
    }

    /// Gets version information
    pub async fn get_version(&self) -> Result<Value> {
        // Get version from environment at compile time
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const NEO_VERSION: &str = "3.6.0";

        let version = RpcVersion {
            tcpport: 20333,
            wsport: 20334,
            nonce: rand::random_u32(),
            useragent: format!("neo-rs/{}", VERSION),
            protocol: neo_rpc_client::models::RpcProtocolConfiguration {
                addressversion: 53,
                network: 5195086,
                validatorscount: 7,
                msperblock: 15000,
                maxtraceableblocks: 2102400,
                maxvaliduntilblockincrement: 5760,
                maxtransactionsperblock: 512,
                memorypoolmaxtransactions: 50000,
                initialgasdistribution: "52000000".to_string(),
            },
        };

        Ok(serde_json::to_value(version)?)
    }

    /// Gets peer information from the production peer registry
    pub async fn get_peers(&self) -> Result<Value> {
        let peer_registry = self
            .peer_registry
            .read()
            .map_err(|e| format!("Failed to acquire peer registry lock: {}", e))?;

        // Build connected peers list from real connections
        let connected = peer_registry
            .connected_peers
            .values()
            .map(|peer| RpcPeer {
                address: peer.address.clone(),
                port: peer.port,
            })
            .collect();

        // Build unconnected peers list from known peers not currently connected
        let unconnected = peer_registry
            .known_peers
            .values()
            .filter(|known_peer| {
                let key = format!("{}:{}", known_peer.address, known_peer.port);
                !peer_registry.connected_peers.contains_key(&key)
            })
            .map(|peer| RpcPeer {
                address: peer.address.clone(),
                port: peer.port,
            })
            .collect();

        // Build bad peers list (peers with excessive failed connection attempts)
        let bad = peer_registry
            .known_peers
            .values()
            .filter(|peer| peer.connection_attempts > 5)
            .map(|peer| RpcPeer {
                address: peer.address.clone(),
                port: peer.port,
            })
            .collect();

        let peers = RpcPeers {
            connected,
            unconnected,
            bad,
        };

        Ok(serde_json::to_value(peers)?)
    }

    /// Gets real connection count from peer registry
    pub async fn get_connection_count(&self) -> Result<Value> {
        let peer_registry = self
            .peer_registry
            .read()
            .map_err(|e| format!("Failed to acquire peer registry lock: {}", e))?;

        let connection_count = peer_registry.get_connection_count();
        Ok(json!(connection_count))
    }

    /// Validates an address
    pub async fn validate_address(&self, params: Option<Value>) -> Result<Value> {
        let params = params.ok_or("Missing parameters")?;
        let params_array = params.as_array().ok_or("Invalid parameters format")?;

        if params_array.is_empty() {
            return Err("Missing address".into());
        }

        let address = params_array[0].as_str().ok_or("Invalid address format")?;

        let is_valid =
            address.len() == 34 && (address.starts_with('N') || address.starts_with('A'));

        let validation = json!({
            "address": address,
            "isvalid": is_valid
        });

        Ok(validation)
    }

    /// Gets native contracts
    pub async fn get_native_contracts(&self) -> Result<Value> {
        let contracts = json!([
            {
                "id": -1,
                "hash": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
                "nef": {
                    "checksum": 0
                },
                "updatehistory": [0]
            },
            {
                "id": -2,
                "hash": "0x43cf98eddbe047e198a3e5d57006311442a0ca15",
                "nef": {
                    "checksum": 0
                },
                "updatehistory": [0]
            }
        ]);

        Ok(contracts)
    }
}

// Add random number generation
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random_u32() -> u32 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .hash(&mut hasher);
        hasher.finish() as u32
    }
}

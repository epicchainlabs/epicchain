//! RPC Types
//!
//! JSON-RPC request and response types for Neo N3 RPC server.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    pub id: Option<Value>,
}

impl RpcResponse {
    /// Creates a success response
    pub fn success(result: Value, id: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Creates an error response
    pub fn error(code: i32, message: &str, id: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(RpcError {
                code,
                message: message.to_string(),
                data: None,
            }),
            id,
        }
    }
}

/// JSON-RPC error object
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Block information for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcBlock {
    pub hash: String,
    pub size: u32,
    pub version: u32,
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    pub time: u64,
    pub index: u32,
    #[serde(rename = "nextconsensus")]
    pub next_consensus: String,
    pub witnesses: Vec<RpcWitness>,
    pub tx: Vec<RpcTransaction>,
    pub confirmations: u32,
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

/// Transaction information for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcTransaction {
    pub hash: String,
    pub size: u32,
    pub version: u8,
    pub nonce: u32,
    #[serde(rename = "sysfee")]
    pub system_fee: String,
    #[serde(rename = "netfee")]
    pub network_fee: String,
    #[serde(rename = "validuntilblock")]
    pub valid_until_block: u32,
    pub signers: Vec<RpcSigner>,
    pub attributes: Vec<Value>,
    pub script: String,
    pub witnesses: Vec<RpcWitness>,
}

/// Witness information for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcWitness {
    pub invocation: String,
    pub verification: String,
}

/// Signer information for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcSigner {
    pub account: String,
    pub scopes: String,
}

/// Version information for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcVersion {
    #[serde(rename = "tcpport")]
    pub tcp_port: u16,
    #[serde(rename = "wsport")]
    pub ws_port: u16,
    pub nonce: u32,
    #[serde(rename = "useragent")]
    pub user_agent: String,
}

/// Peer information for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcPeer {
    pub address: String,
    pub port: u16,
}

/// Peer list for RPC responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcPeers {
    pub unconnected: Vec<RpcPeer>,
    pub bad: Vec<RpcPeer>,
    pub connected: Vec<RpcPeer>,
}

/// Address validation result
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcAddressValidation {
    pub address: String,
    #[serde(rename = "isvalid")]
    pub is_valid: bool,
}

/// Native contract information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RpcNativeContract {
    pub id: i32,
    pub hash: String,
    #[serde(rename = "nef")]
    pub nef_checksum: String,
    #[serde(rename = "updatecounter")]
    pub update_counter: u32,
}

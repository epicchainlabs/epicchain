//! RPC Models
//!
//! This module contains data models for RPC requests and responses.

use neo_core::{UInt160, UInt256};
use serde::{Deserialize, Serialize};
use std::fmt;

/// RPC request structure (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC request structure (alias for RpcRequest)
pub type JsonRpcRequest = RpcRequest;

/// JSON-RPC response structure (alias for RpcResponse)
pub type JsonRpcResponse = RpcResponse;

/// RPC response structure (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<JsonRpcError>,
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC error structure (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RPC Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for JsonRpcError {}

/// Block information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcBlock {
    pub hash: UInt256,
    pub size: u32,
    pub version: u32,
    pub merkleroot: UInt256,
    pub time: u64,
    pub index: u32,
    pub primary: u8,
    pub nextconsensus: UInt160,
    pub witnesses: Vec<RpcWitness>,
    pub tx: Vec<RpcTransaction>,
    pub confirmations: Option<u32>,
    pub nextblockhash: Option<UInt256>,
}

/// Transaction information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcTransaction {
    pub hash: UInt256,
    pub size: u32,
    pub version: u8,
    pub nonce: u32,
    pub sender: Option<UInt160>,
    pub sysfee: String,
    pub netfee: String,
    pub validuntilblock: u32,
    pub signers: Vec<RpcSigner>,
    pub attributes: Vec<RpcTransactionAttribute>,
    pub script: String,
    pub witnesses: Vec<RpcWitness>,
    pub blockhash: Option<UInt256>,
    pub confirmations: Option<u32>,
    pub blocktime: Option<u64>,
}

/// Witness information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcWitness {
    pub invocation: String,
    pub verification: String,
}

/// Signer information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcSigner {
    pub account: UInt160,
    pub scopes: String,
    pub allowedcontracts: Option<Vec<UInt160>>,
    pub allowedgroups: Option<Vec<String>>,
    pub rules: Option<Vec<RpcWitnessRule>>,
}

/// Witness rule for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcWitnessRule {
    pub action: String,
    pub condition: serde_json::Value,
}

/// Transaction attribute for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcTransactionAttribute {
    #[serde(rename = "type")]
    pub attr_type: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Application log for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcApplicationLog {
    pub txid: Option<UInt256>,
    pub blockhash: UInt256,
    pub executions: Vec<RpcExecution>,
}

/// Execution information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcExecution {
    pub trigger: String,
    pub vmstate: String,
    pub exception: Option<String>,
    pub gasconsumed: String,
    pub stack: Vec<serde_json::Value>,
    pub notifications: Vec<RpcNotification>,
}

/// Notification information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcNotification {
    pub contract: UInt160,
    pub eventname: String,
    pub state: serde_json::Value,
}

/// NEP-17 token balance information (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcNep17Balance {
    pub assethash: UInt160,
    pub amount: String,
    pub lastupdatedblock: u32,
}

/// NEP-11 token information (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcNep11Token {
    pub assethash: UInt160,
    pub tokenid: String,
    pub amount: String,
    pub lastupdatedblock: u32,
}

/// Validator information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcValidator {
    pub publickey: String,
    pub votes: String,
    pub active: bool,
}

/// Peers information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcPeers {
    pub unconnected: Vec<RpcPeer>,
    pub bad: Vec<RpcPeer>,
    pub connected: Vec<RpcPeer>,
}

/// Peer information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcPeer {
    pub address: String,
    pub port: u16,
}

/// Version information for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcVersion {
    pub tcpport: u16,
    pub wsport: u16,
    pub nonce: u32,
    pub useragent: String,
    pub protocol: RpcProtocolConfiguration,
}

/// Protocol configuration for RPC responses (matches Neo C# RPC protocol)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcProtocolConfiguration {
    pub addressversion: u8,
    pub network: u32,
    pub validatorscount: u8,
    pub msperblock: u32,
    pub maxtraceableblocks: u32,
    pub maxvaliduntilblockincrement: u32,
    pub maxtransactionsperblock: u32,
    pub memorypoolmaxtransactions: u32,
    pub initialgasdistribution: String,
}

impl RpcRequest {
    /// Creates a new RPC request
    pub fn new(method: String, params: serde_json::Value, id: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
            id,
        }
    }
}

impl RpcResponse {
    /// Creates a successful RPC response
    pub fn success(result: serde_json::Value, id: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Creates an error RPC response
    pub fn error(error: JsonRpcError, id: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id,
        }
    }
}

impl JsonRpcError {
    /// Creates a new RPC error
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    /// Creates an RPC error with additional data
    pub fn with_data(code: i32, message: String, data: serde_json::Value) -> Self {
        Self {
            code,
            message,
            data: Some(data),
        }
    }
}

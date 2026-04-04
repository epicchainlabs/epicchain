//! Neo N3 RPC Client Library
//!
//! This library provides a complete RPC client implementation for Neo N3 blockchain,
//! matching the C# Neo.Network.RPC library exactly.
//!
//! ## Features
//!
//! - **Complete RPC API Coverage**: All Neo N3 RPC methods
//! - **Type Safety**: Strongly typed request/response objects
//! - **Async/Await Support**: Full async support with tokio
//! - **Error Handling**: Comprehensive error types
//! - **Connection Management**: Automatic retry and connection pooling
//! - **C# Compatibility**: Exact API compatibility with C# implementation

pub mod client;
pub mod error;
pub mod methods;
pub mod models;
pub mod neo_rpc;

pub use client::{RpcClient, RpcClientBuilder};
pub use error::{RpcError, RpcResult};
pub use models::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, RpcRequest, RpcResponse};

use neo_config::DEFAULT_RPC_PORT;

/// Default Neo network ports
/// RPC client configuration (matches C# RpcClient settings exactly)
#[derive(Debug, Clone)]
pub struct RpcConfig {
    /// RPC server endpoint URL
    pub endpoint: String,
    /// Request timeout in seconds
    pub timeout: u64,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay: u64,
    /// User agent string for HTTP requests
    pub user_agent: String,
    /// Custom HTTP headers
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            endpoint: format!("http://localhost:{}", DEFAULT_RPC_PORT),
            timeout: 30,
            max_retries: 3,
            retry_delay: 1000,
            user_agent: "neo-rpc-client/0.1.0".to_string(),
            headers: std::collections::HashMap::new(),
        }
    }
}

/// RPC method names (matches C# RpcMethods exactly)
pub mod rpc_methods {
    /// Blockchain methods
    pub const GET_BEST_BLOCK_HASH: &str = "getbestblockhash";
    pub const GET_BLOCK: &str = "getblock";
    pub const GET_BLOCK_COUNT: &str = "getblockcount";
    pub const GET_BLOCK_HASH: &str = "getblockhash";
    pub const GET_BLOCK_HEADER: &str = "getblockheader";
    pub const GET_BLOCK_HEADER_COUNT: &str = "getblockheadercount";
    pub const GET_COMMITTEE: &str = "getcommittee";
    pub const GET_CONNECTION_COUNT: &str = "getconnectioncount";
    pub const GET_CONTRACT_STATE: &str = "getcontractstate";
    pub const GET_NATIVE_CONTRACTS: &str = "getnativecontracts";
    pub const GET_NEXT_BLOCK_VALIDATORS: &str = "getnextblockvalidators";
    pub const GET_PEERS: &str = "getpeers";
    pub const GET_RAW_MEMPOOL: &str = "getrawmempool";
    pub const GET_RAW_TRANSACTION: &str = "getrawtransaction";
    pub const GET_STORAGE: &str = "getstorage";
    pub const GET_TRANSACTION_HEIGHT: &str = "gettransactionheight";
    pub const GET_VALIDATORS: &str = "getvalidators";
    pub const GET_VERSION: &str = "getversion";

    /// Wallet methods
    pub const CALCULATE_NETWORK_FEE: &str = "calculatenetworkfee";
    pub const GET_APPLICATION_LOG: &str = "getapplicationlog";
    pub const GET_BALANCE: &str = "getbalance";
    pub const GET_NEP17_BALANCES: &str = "getnep17balances";
    pub const GET_NEP17_TRANSFERS: &str = "getnep17transfers";
    pub const GET_NEW_ADDRESS: &str = "getnewaddress";
    pub const GET_UNCLAIMED_GAS: &str = "getunclaimedgas";
    pub const GET_WALLET_BALANCE: &str = "getwalletbalance";
    pub const GET_WALLET_UNCLAIMED_GAS: &str = "getwalletunclaimedgas";
    pub const IMPORT_PRIVATE_KEY: &str = "importprivkey";
    pub const LIST_ADDRESS: &str = "listaddress";
    pub const SEND_FROM: &str = "sendfrom";
    pub const SEND_MANY: &str = "sendmany";
    pub const SEND_RAW_TRANSACTION: &str = "sendrawtransaction";
    pub const SEND_TO_ADDRESS: &str = "sendtoaddress";

    /// Smart contract methods
    pub const INVOKE_FUNCTION: &str = "invokefunction";
    pub const INVOKE_SCRIPT: &str = "invokescript";
    pub const TEST_INVOKE: &str = "testinvoke";

    /// Utility methods
    pub const LIST_PLUGINS: &str = "listplugins";
    pub const VALIDATE_ADDRESS: &str = "validateaddress";
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_config_default() {
        let config = RpcConfig::default();
        assert!(config.endpoint.starts_with("http://localhost:"));
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_json_rpc_request() {
        let request = RpcRequest::new(
            "getblockcount".to_string(),
            serde_json::Value::Array(vec![]),
            Some(serde_json::json!(1)),
        );
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "getblockcount");
        assert_eq!(request.id, Some(serde_json::json!(1)));
    }

    #[test]
    fn test_json_rpc_serialization() {
        let request = RpcRequest::new(
            "getblock".to_string(),
            serde_json::json!(["0x1234", true]),
            Some(serde_json::json!(42)),
        );

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: RpcRequest =
            serde_json::from_str(&json).expect("Failed to parse from string");

        assert_eq!(request.method, deserialized.method);
        assert_eq!(request.id, deserialized.id);
    }
}

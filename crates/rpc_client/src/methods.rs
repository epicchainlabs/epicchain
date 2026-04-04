//! RPC Methods
//!
//! This module contains specific RPC method implementations.

use crate::models::{
    RpcApplicationLog, RpcBlock, RpcExecution, RpcNep17Balance, RpcPeers, RpcRequest, RpcResponse,
    RpcTransaction, RpcValidator, RpcVersion,
};
use neo_core::{UInt160, UInt256};
use serde_json::{json, Value};
use std::str::FromStr;

/// RPC client for Neo blockchain (matches C# Neo RPC protocol)
pub struct RpcClient {
    endpoint: String,
    client: reqwest::Client,
}

impl RpcClient {
    /// Creates a new RPC client
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    /// Sends an RPC request and returns the response
    pub async fn send_request(
        &self,
        request: RpcRequest,
    ) -> Result<RpcResponse, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        let rpc_response: RpcResponse = response.json().await?;
        Ok(rpc_response)
    }

    /// Gets the best block hash
    pub async fn get_best_block_hash(&self) -> Result<UInt256, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getbestblockhash".to_string(),
            params: json!([]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let hash_str = result.as_str().ok_or("Invalid hash format")?;
            let hash = UInt256::from_str(hash_str)?;
            Ok(hash)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets a block by hash
    pub async fn get_block(
        &self,
        hash: &UInt256,
        verbose: bool,
    ) -> Result<RpcBlock, Box<dyn std::error::Error>> {
        let params = if verbose {
            json!([hash.to_string(), 1])
        } else {
            json!([hash.to_string(), 0])
        };

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getblock".to_string(),
            params,
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let block: RpcBlock = serde_json::from_value(result)?;
            Ok(block)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets a block by index
    pub async fn get_block_by_index(
        &self,
        index: u32,
        verbose: bool,
    ) -> Result<RpcBlock, Box<dyn std::error::Error>> {
        let params = if verbose {
            json!([index, 1])
        } else {
            json!([index, 0])
        };

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getblock".to_string(),
            params,
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let block: RpcBlock = serde_json::from_value(result)?;
            Ok(block)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets block count
    pub async fn get_block_count(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getblockcount".to_string(),
            params: json!([]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let count = result.as_u64().ok_or("Invalid count format")? as u32;
            Ok(count)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets a transaction by hash
    pub async fn get_transaction(
        &self,
        hash: &UInt256,
        verbose: bool,
    ) -> Result<RpcTransaction, Box<dyn std::error::Error>> {
        let params = if verbose {
            json!([hash.to_string(), 1])
        } else {
            json!([hash.to_string(), 0])
        };

        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getrawtransaction".to_string(),
            params,
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let transaction: RpcTransaction = serde_json::from_value(result)?;
            Ok(transaction)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Sends a raw transaction
    pub async fn send_raw_transaction(
        &self,
        raw_transaction: &str,
    ) -> Result<UInt256, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "sendrawtransaction".to_string(),
            params: json!([raw_transaction]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let hash_str = result.as_str().ok_or("Invalid hash format")?;
            let hash = UInt256::from_str(hash_str)?;
            Ok(hash)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets application log
    pub async fn get_application_log(
        &self,
        hash: &UInt256,
    ) -> Result<RpcApplicationLog, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getapplicationlog".to_string(),
            params: json!([hash.to_string()]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let app_log: RpcApplicationLog = serde_json::from_value(result)?;
            Ok(app_log)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets NEP-17 balance for an address
    pub async fn get_nep17_balances(
        &self,
        address: &UInt160,
    ) -> Result<Vec<RpcNep17Balance>, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getnep17balances".to_string(),
            params: json!([address.to_string()]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let balances_obj = result.as_object().ok_or("Invalid balances format")?;
            let balance_array = balances_obj.get("balance").ok_or("No balance field")?;
            let balances: Vec<RpcNep17Balance> = serde_json::from_value(balance_array.clone())?;
            Ok(balances)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets validators
    pub async fn get_validators(&self) -> Result<Vec<RpcValidator>, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getvalidators".to_string(),
            params: json!([]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let validators: Vec<RpcValidator> = serde_json::from_value(result)?;
            Ok(validators)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets connected peers
    pub async fn get_peers(&self) -> Result<RpcPeers, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getpeers".to_string(),
            params: json!([]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let peers: RpcPeers = serde_json::from_value(result)?;
            Ok(peers)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets version information
    pub async fn get_version(&self) -> Result<RpcVersion, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getversion".to_string(),
            params: json!([]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let version: RpcVersion = serde_json::from_value(result)?;
            Ok(version)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Invokes a smart contract function (read-only)
    pub async fn invoke_function(
        &self,
        script_hash: &UInt160,
        operation: &str,
        params: Vec<Value>,
    ) -> Result<RpcExecution, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "invokefunction".to_string(),
            params: json!([script_hash.to_string(), operation, params]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let execution: RpcExecution = serde_json::from_value(result)?;
            Ok(execution)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Invokes a script (read-only)
    pub async fn invoke_script(
        &self,
        script: &str,
    ) -> Result<RpcExecution, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "invokescript".to_string(),
            params: json!([script]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let execution: RpcExecution = serde_json::from_value(result)?;
            Ok(execution)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets storage value by contract and key
    pub async fn get_storage(
        &self,
        script_hash: &UInt160,
        key: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getstorage".to_string(),
            params: json!([script_hash.to_string(), key]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            if result.is_null() {
                Ok(None)
            } else {
                let value = result.as_str().ok_or("Invalid storage value format")?;
                Ok(Some(value.to_string()))
            }
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }

    /// Gets memory pool transactions
    pub async fn get_raw_mempool(
        &self,
        should_get_unverified: bool,
    ) -> Result<Vec<UInt256>, Box<dyn std::error::Error>> {
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getrawmempool".to_string(),
            params: json!([should_get_unverified]),
            id: Some(json!(1)),
        };
        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let txids: Vec<String> = serde_json::from_value(result)?;
            let mut hashes = Vec::new();
            for txid in txids {
                let hash = UInt256::from_str(&txid)?;
                hashes.push(hash);
            }
            Ok(hashes)
        } else if let Some(error) = response.error {
            Err(format!("RPC error: {}", error.message).into())
        } else {
            Err("No result or error in response".into())
        }
    }
}

/// Collection of RPC method implementations
pub struct RpcMethods;

impl RpcMethods {
    /// Creates a new RPC client instance
    pub fn create_client(endpoint: String) -> RpcClient {
        RpcClient::new(endpoint)
    }

    /// Validates RPC request format
    pub fn validate_request(request: &RpcRequest) -> Result<(), String> {
        if request.jsonrpc != "2.0" {
            return Err("Invalid JSON-RPC version".to_string());
        }

        if request.method.is_empty() {
            return Err("Method cannot be empty".to_string());
        }

        Ok(())
    }

    /// Creates standard RPC error responses
    pub fn create_error_response(code: i32, message: String, id: Option<Value>) -> RpcResponse {
        RpcResponse::error(crate::models::JsonRpcError::new(code, message), id)
    }

    /// Parses standard RPC parameters
    pub fn parse_params<T: serde::de::DeserializeOwned>(params: &Value) -> Result<T, String> {
        serde_json::from_value(params.clone())
            .map_err(|e| format!("Parameter parsing error: {}", e))
    }

    /// Converts hex string to bytes
    pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("Hex string length must be even".to_string());
        }

        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| format!("Invalid hex character in: {}", byte_str))?;
            bytes.push(byte);
        }
        Ok(bytes)
    }

    /// Converts bytes to hex string
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

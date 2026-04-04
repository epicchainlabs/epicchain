//! Neo N3 RPC Methods Implementation
//!
//! This module provides complete RPC method implementations that exactly match
//! the C# Neo.Network.RPC.RpcClient interface, ensuring 100% compatibility.

use crate::{RpcClient, RpcError, RpcResult};
use neo_core::{UInt160, UInt256};
use serde_json::{json, Value};

/// Default Neo network ports
/// Complete Neo N3 RPC interface implementation (matches C# RpcClient exactly)
impl RpcClient {
    // ===== Blockchain Information Methods =====

    /// Gets the best block hash (matches C# GetBestBlockHashAsync)
    pub async fn get_best_block_hash(&self) -> RpcResult<UInt256> {
        let result = self
            .call_raw("getbestblockhash".to_string(), json!([]))
            .await?;
        self.parse_uint256(&result)
    }

    /// Gets a block by hash or index (matches C# GetBlockAsync)
    pub async fn get_block(&self, hash_or_index: Value, verbose: Option<bool>) -> RpcResult<Value> {
        let params = if let Some(v) = verbose {
            json!([hash_or_index, v])
        } else {
            json!([hash_or_index])
        };
        self.call_raw("getblock".to_string(), params).await
    }

    /// Gets a block header by hash or index (matches C# GetBlockHeaderAsync)
    pub async fn get_block_header(
        &self,
        hash_or_index: Value,
        verbose: Option<bool>,
    ) -> RpcResult<Value> {
        let params = if let Some(v) = verbose {
            json!([hash_or_index, v])
        } else {
            json!([hash_or_index])
        };
        self.call_raw("getblockheader".to_string(), params).await
    }

    /// Gets the block count (matches C# GetBlockCountAsync)
    pub async fn get_block_count(&self) -> RpcResult<u32> {
        let result = self
            .call_raw("getblockcount".to_string(), json!([]))
            .await?;
        self.parse_u32(&result)
    }

    /// Gets the block hash by index (matches C# GetBlockHashAsync)
    pub async fn get_block_hash(&self, index: u32) -> RpcResult<UInt256> {
        let result = self
            .call_raw("getblockhash".to_string(), json!([index]))
            .await?;
        self.parse_uint256(&result)
    }

    /// Gets the connection count (matches C# GetConnectionCountAsync)
    pub async fn get_connection_count(&self) -> RpcResult<u32> {
        let result = self
            .call_raw("getconnectioncount".to_string(), json!([]))
            .await?;
        self.parse_u32(&result)
    }

    /// Gets the committee (matches C# GetCommitteeAsync)
    pub async fn get_committee(&self) -> RpcResult<Vec<String>> {
        let result = self.call_raw("getcommittee".to_string(), json!([])).await?;
        self.parse_string_array(&result)
    }

    /// Gets the next block validators (matches C# GetNextBlockValidatorsAsync)
    pub async fn get_next_block_validators(&self) -> RpcResult<Vec<Value>> {
        let result = self
            .call_raw("getnextblockvalidators".to_string(), json!([]))
            .await?;
        self.parse_array(&result)
    }

    /// Gets the raw memory pool (matches C# GetRawMempoolAsync)
    pub async fn get_raw_mempool(&self, should_get_unverified: Option<bool>) -> RpcResult<Value> {
        let params = if let Some(unverified) = should_get_unverified {
            json!([unverified])
        } else {
            json!([])
        };
        self.call_raw("getrawmempool".to_string(), params).await
    }

    /// Gets a transaction by hash (matches C# GetRawTransactionAsync)
    pub async fn get_raw_transaction(
        &self,
        tx_hash: UInt256,
        verbose: Option<bool>,
    ) -> RpcResult<Value> {
        let params = if let Some(v) = verbose {
            json!([tx_hash.to_string(), v])
        } else {
            json!([tx_hash.to_string()])
        };
        self.call_raw("getrawtransaction".to_string(), params).await
    }

    /// Gets storage by contract hash and key (matches C# GetStorageAsync)
    pub async fn get_storage(
        &self,
        contract_hash: UInt160,
        key: &str,
    ) -> RpcResult<Option<String>> {
        let result = self
            .call_raw(
                "getstorage".to_string(),
                json!([contract_hash.to_string(), key]),
            )
            .await?;

        if result.is_null() {
            Ok(None)
        } else {
            Ok(Some(self.parse_string(&result)?))
        }
    }

    /// Gets transaction height (matches C# GetTransactionHeightAsync)
    pub async fn get_transaction_height(&self, tx_hash: UInt256) -> RpcResult<u32> {
        let result = self
            .call_raw(
                "gettransactionheight".to_string(),
                json!([tx_hash.to_string()]),
            )
            .await?;
        self.parse_u32(&result)
    }

    /// Gets the version information (matches C# GetVersionAsync)
    pub async fn get_version(&self) -> RpcResult<Value> {
        self.call_raw("getversion".to_string(), json!([])).await
    }

    // ===== Node Information Methods =====

    /// Gets peers information (matches C# GetPeersAsync)
    pub async fn get_peers(&self) -> RpcResult<Value> {
        self.call_raw("getpeers".to_string(), json!([])).await
    }

    /// Gets state height (matches C# GetStateHeightAsync)
    pub async fn get_state_height(&self) -> RpcResult<Value> {
        self.call_raw("getstateheight".to_string(), json!([])).await
    }

    /// Gets state root by index (matches C# GetStateRootAsync)
    pub async fn get_state_root(&self, index: u32) -> RpcResult<Value> {
        self.call_raw("getstateroot".to_string(), json!([index]))
            .await
    }

    /// Gets the proof (matches C# GetProofAsync)
    pub async fn get_proof(
        &self,
        root_hash: UInt256,
        contract_hash: UInt160,
        key: &str,
    ) -> RpcResult<Value> {
        self.call_raw(
            "getproof".to_string(),
            json!([root_hash.to_string(), contract_hash.to_string(), key]),
        )
        .await
    }

    /// Verifies proof (matches C# VerifyProofAsync)
    pub async fn verify_proof(&self, root_hash: UInt256, proof: &str) -> RpcResult<Value> {
        self.call_raw(
            "verifyproof".to_string(),
            json!([root_hash.to_string(), proof]),
        )
        .await
    }

    // ===== Contract and State Methods =====

    /// Invokes a contract function (matches C# InvokeFunctionAsync)
    pub async fn invoke_function(
        &self,
        contract_hash: UInt160,
        method: &str,
        params: Vec<Value>,
        signers: Option<Vec<Value>>,
    ) -> RpcResult<Value> {
        let mut rpc_params = vec![
            json!(contract_hash.to_string()),
            json!(method),
            json!(params),
        ];

        if let Some(s) = signers {
            rpc_params.push(json!(s));
        }

        self.call_raw("invokefunction".to_string(), json!(rpc_params))
            .await
    }

    /// Invokes a script (matches C# InvokeScriptAsync)
    pub async fn invoke_script(
        &self,
        script: &str,
        signers: Option<Vec<Value>>,
    ) -> RpcResult<Value> {
        let mut params = vec![json!(script)];

        if let Some(s) = signers {
            params.push(json!(s));
        }

        self.call_raw("invokescript".to_string(), json!(params))
            .await
    }

    /// Gets contract state (matches C# GetContractStateAsync)
    pub async fn get_contract_state(&self, contract_hash: UInt160) -> RpcResult<Value> {
        self.call_raw(
            "getcontractstate".to_string(),
            json!([contract_hash.to_string()]),
        )
        .await
    }

    /// Gets NEP-17 balances (matches C# GetNep17BalancesAsync)
    pub async fn get_nep17_balances(&self, address: &str) -> RpcResult<Value> {
        self.call_raw("getnep17balances".to_string(), json!([address]))
            .await
    }

    /// Gets NEP-17 transfers (matches C# GetNep17TransfersAsync)
    pub async fn get_nep17_transfers(
        &self,
        address: &str,
        timestamp: Option<u64>,
    ) -> RpcResult<Value> {
        let params = if let Some(ts) = timestamp {
            json!([address, ts])
        } else {
            json!([address])
        };
        self.call_raw("getnep17transfers".to_string(), params).await
    }

    /// Gets NEP-11 balances (matches C# GetNep11BalancesAsync)
    pub async fn get_nep11_balances(&self, address: &str) -> RpcResult<Value> {
        self.call_raw("getnep11balances".to_string(), json!([address]))
            .await
    }

    /// Gets NEP-11 transfers (matches C# GetNep11TransfersAsync)
    pub async fn get_nep11_transfers(
        &self,
        address: &str,
        timestamp: Option<u64>,
    ) -> RpcResult<Value> {
        let params = if let Some(ts) = timestamp {
            json!([address, ts])
        } else {
            json!([address])
        };
        self.call_raw("getnep11transfers".to_string(), params).await
    }

    /// Gets NEP-11 properties (matches C# GetNep11PropertiesAsync)
    pub async fn get_nep11_properties(
        &self,
        contract_hash: UInt160,
        token_id: &str,
    ) -> RpcResult<Value> {
        self.call_raw(
            "getnep11properties".to_string(),
            json!([contract_hash.to_string(), token_id]),
        )
        .await
    }

    // ===== Transaction Methods =====

    /// Sends raw transaction (matches C# SendRawTransactionAsync)
    pub async fn send_raw_transaction(&self, raw_transaction: &str) -> RpcResult<Value> {
        self.call_raw("sendrawtransaction".to_string(), json!([raw_transaction]))
            .await
    }

    /// Submits a block (matches C# SubmitBlockAsync)
    pub async fn submit_block(&self, block: &str) -> RpcResult<Value> {
        self.call_raw("submitblock".to_string(), json!([block]))
            .await
    }

    /// Validates an address (matches C# ValidateAddressAsync)
    pub async fn validate_address(&self, address: &str) -> RpcResult<Value> {
        self.call_raw("validateaddress".to_string(), json!([address]))
            .await
    }

    // ===== Wallet Methods =====

    /// Closes the wallet (matches C# CloseWalletAsync)
    pub async fn close_wallet(&self) -> RpcResult<Value> {
        self.call_raw("closewallet".to_string(), json!([])).await
    }

    /// Dumps private key (matches C# DumpPrivKeyAsync)
    pub async fn dump_priv_key(&self, address: &str) -> RpcResult<String> {
        let result = self
            .call_raw("dumpprivkey".to_string(), json!([address]))
            .await?;
        self.parse_string(&result)
    }

    /// Gets wallet balance (matches C# GetWalletBalanceAsync)
    pub async fn get_wallet_balance(&self, asset_id: &str) -> RpcResult<Value> {
        self.call_raw("getwalletbalance".to_string(), json!([asset_id]))
            .await
    }

    /// Gets new address (matches C# GetNewAddressAsync)
    pub async fn get_new_address(&self) -> RpcResult<String> {
        let result = self
            .call_raw("getnewaddress".to_string(), json!([]))
            .await?;
        self.parse_string(&result)
    }

    /// Gets wallet unclaimed GAS (matches C# GetWalletUnclaimedGasAsync)
    pub async fn get_wallet_unclaimed_gas(&self) -> RpcResult<Value> {
        self.call_raw("getwalletunclaimedgas".to_string(), json!([]))
            .await
    }

    /// Imports private key (matches C# ImportPrivKeyAsync)
    pub async fn import_priv_key(&self, private_key: &str) -> RpcResult<Value> {
        self.call_raw("importprivkey".to_string(), json!([private_key]))
            .await
    }

    /// Calculates network fee (matches C# CalculateNetworkFeeAsync)
    pub async fn calculate_network_fee(&self, tx: &str) -> RpcResult<Value> {
        self.call_raw("calculatenetworkfee".to_string(), json!([tx]))
            .await
    }

    /// Lists address (matches C# ListAddressAsync)
    pub async fn list_address(&self) -> RpcResult<Vec<Value>> {
        let result = self.call_raw("listaddress".to_string(), json!([])).await?;
        self.parse_array(&result)
    }

    /// Opens wallet (matches C# OpenWalletAsync)
    pub async fn open_wallet(&self, path: &str, password: &str) -> RpcResult<Value> {
        self.call_raw("openwallet".to_string(), json!([path, password]))
            .await
    }

    /// Sends from address (matches C# SendFromAsync)
    pub async fn send_from(
        &self,
        asset_id: &str,
        from: &str,
        to: &str,
        value: &str,
    ) -> RpcResult<Value> {
        self.call_raw("sendfrom".to_string(), json!([asset_id, from, to, value]))
            .await
    }

    /// Sends to address (matches C# SendToAddressAsync)
    pub async fn send_to_address(&self, asset_id: &str, to: &str, value: &str) -> RpcResult<Value> {
        self.call_raw("sendtoaddress".to_string(), json!([asset_id, to, value]))
            .await
    }

    /// Sends many (matches C# SendManyAsync)
    pub async fn send_many(&self, outputs: Vec<Value>) -> RpcResult<Value> {
        self.call_raw("sendmany".to_string(), json!([outputs]))
            .await
    }

    // ===== ApplicationLogs and Notifications =====

    /// Gets application log (matches C# GetApplicationLogAsync)
    pub async fn get_application_log(&self, tx_hash: UInt256) -> RpcResult<Value> {
        self.call_raw(
            "getapplicationlog".to_string(),
            json!([tx_hash.to_string()]),
        )
        .await
    }

    // ===== Utility and Helper Methods =====

    /// Lists plugins (matches C# ListPluginsAsync)
    pub async fn list_plugins(&self) -> RpcResult<Vec<Value>> {
        let result = self.call_raw("listplugins".to_string(), json!([])).await?;
        self.parse_array(&result)
    }

    // ===== Helper parsing methods =====

    /// Parses a UInt256 from JSON value
    fn parse_uint256(&self, value: &Value) -> RpcResult<UInt256> {
        let str_val = self.parse_string(value)?;
        UInt256::parse(&str_val).map_err(|e| RpcError::Parse(format!("Invalid UInt256: {}", e)))
    }

    /// Parses a UInt160 from JSON value
    fn parse_uint160(&self, value: &Value) -> RpcResult<UInt160> {
        let str_val = self.parse_string(value)?;
        UInt160::parse(&str_val).map_err(|e| RpcError::Parse(format!("Invalid UInt160: {}", e)))
    }

    /// Parses a string from JSON value
    fn parse_string(&self, value: &Value) -> RpcResult<String> {
        value
            .as_str()
            .ok_or_else(|| RpcError::Parse("Expected string value".to_string()))
            .map(|s| s.to_string())
    }

    /// Parses a u32 from JSON value
    fn parse_u32(&self, value: &Value) -> RpcResult<u32> {
        value
            .as_u64()
            .ok_or_else(|| RpcError::Parse("Expected numeric value".to_string()))?
            .try_into()
            .map_err(|e| RpcError::Parse(format!("Invalid u32: {}", e)))
    }

    /// Parses an array from JSON value
    fn parse_array(&self, value: &Value) -> RpcResult<Vec<Value>> {
        value
            .as_array()
            .ok_or_else(|| RpcError::Parse("Expected array value".to_string()))
            .map(|arr| arr.clone())
    }

    /// Parses a string array from JSON value
    fn parse_string_array(&self, value: &Value) -> RpcResult<Vec<String>> {
        let array = self.parse_array(value)?;
        array.into_iter().map(|v| self.parse_string(&v)).collect()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{RpcError, RpcResult};

    #[tokio::test]
    async fn test_rpc_client_creation() {
        let client = RpcClient::new("http://localhost:10332".to_string()).unwrap();
        assert!(client.endpoint().contains("localhost"));
    }

    #[test]
    fn test_parse_helpers() {
        let client = RpcClient::new("http://localhost:10332".to_string()).unwrap();

        let string_val = json!("test_string");
        assert_eq!(client.parse_string(&string_val).unwrap(), "test_string");

        // Test u32 parsing
        let u32_val = json!(12345);
        assert_eq!(client.parse_u32(&u32_val).unwrap(), 12345);

        // Test array parsing
        let array_val = json!(["item1", "item2", "item3"]);
        let parsed_array = client.parse_array(&array_val).unwrap();
        assert_eq!(parsed_array.len(), 3);
    }
}

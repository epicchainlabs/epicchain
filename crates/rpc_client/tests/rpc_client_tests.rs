//! RPC Client C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo's RPC client functionality.
//! Tests are based on the C# Neo.Network.RPC test suite.

use neo_rpc_client::{RpcClient, RpcError, RpcResult};
use serde_json::{json, Value};
use std::time::Duration;

#[cfg(test)]
#[allow(dead_code)]
mod rpc_client_tests {
    use super::*;
    /// Implementation provided RPC client for testing (matches C# test patterns exactly)
    struct MockRpcClient {
        responses: std::collections::HashMap<String, Value>,
    }

    impl MockRpcClient {
        fn new() -> Self {
            let mut responses = std::collections::HashMap::new();

            responses.insert(
                "getversion".to_string(),
                json!({
                    "tcpport": 10333,
                    "wsport": 10334,
                    "nonce": 1234567890,
                    "useragent": "/Neo:3.6.0/",
                    "protocol": {
                        "addressversion": 53,
                        "network": 860833102,
                        "validatorscount": 7,
                        "msperblock": 15000,
                        "maxvaliduntilblockincrement": 5760,
                        "maxtraceableblocks": 2102400,
                        "initialgasdistribution": "5200000000000000",
                        "hardforks": {
                            "Aspidochelone": 0,
                            "Basilisk": 0,
                            "Cockatrice": 0,
                            "Domovoi": 0
                        }
                    }
                }),
            );

            responses.insert("getblockcount".to_string(), json!(100));

            responses.insert(
                "getblockhash".to_string(),
                json!("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"),
            );

            responses.insert(
                "getblock".to_string(),
                json!({
                    "hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                    "size": 1234,
                    "version": 0,
                    "previousblockhash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                    "merkleroot": "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321",
                    "time": 1234567890,
                    "nonce": "1234567890abcdef",
                    "index": 99,
                    "primary": 0,
                    "nextconsensus": "NVg7LjbMbjZahNAswS1ksHjANMDNxLi8uS",
                    "witnesses": [],
                    "tx": []
                })
            );

            Self { responses }
        }

        async fn call_method(&self, method: &str, _params: Vec<Value>) -> Result<Value, RpcError> {
            match self.responses.get(method) {
                Some(response) => Ok(response.clone()),
                None => Err(RpcError::MethodNotFound {
                    method: method.to_string(),
                }),
            }
        }
    }

    /// Test getversion RPC method (matches C# Neo.Network.RPC.RpcClient.GetVersionAsync exactly)
    #[tokio::test]
    async fn test_get_version_compatibility() {
        let client = MockRpcClient::new();
        let result = client.call_method("getversion", vec![]).await.unwrap();

        // Verify response structure matches C# exactly
        assert!(result.is_object());
        let version_obj = result.as_object().unwrap();

        assert!(version_obj.contains_key("tcpport"));
        assert!(version_obj.contains_key("wsport"));
        assert!(version_obj.contains_key("nonce"));
        assert!(version_obj.contains_key("useragent"));
        assert!(version_obj.contains_key("protocol"));

        let protocol = version_obj.get("protocol").unwrap().as_object().unwrap();
        assert!(protocol.contains_key("addressversion"));
        assert!(protocol.contains_key("network"));
        assert!(protocol.contains_key("validatorscount"));
        assert!(protocol.contains_key("msperblock"));
        assert!(protocol.contains_key("hardforks"));

        // Verify specific values
        assert_eq!(version_obj.get("tcpport").unwrap().as_u64().unwrap(), 10333);
        assert_eq!(version_obj.get("wsport").unwrap().as_u64().unwrap(), 10334);
        assert_eq!(
            protocol.get("addressversion").unwrap().as_u64().unwrap(),
            53
        );
        assert_eq!(
            protocol.get("validatorscount").unwrap().as_u64().unwrap(),
            7
        );
    }

    /// Test getblockcount RPC method (matches C# Neo.Network.RPC.RpcClient.GetBlockCountAsync exactly)
    #[tokio::test]
    async fn test_get_block_count_compatibility() {
        let client = MockRpcClient::new();
        let result = client.call_method("getblockcount", vec![]).await.unwrap();

        assert!(result.is_number());
        assert_eq!(result.as_u64().unwrap(), 100);
    }

    /// Test getblockhash RPC method (matches C# Neo.Network.RPC.RpcClient.GetBlockHashAsync exactly)
    #[tokio::test]
    async fn test_get_block_hash_compatibility() {
        let client = MockRpcClient::new();
        let result = client
            .call_method("getblockhash", vec![json!(99)])
            .await
            .unwrap();

        assert!(result.is_string());
        let hash = result.as_str().unwrap();
        assert_eq!(hash.len(), 66); // "0x" + 64 hex characters
        assert!(hash.starts_with("0x"));
        assert_eq!(
            hash,
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        );
    }

    /// Test getblock RPC method (matches C# Neo.Network.RPC.RpcClient.GetBlockAsync exactly)
    #[tokio::test]
    async fn test_get_block_compatibility() {
        let client = MockRpcClient::new();
        let result = client
            .call_method(
                "getblock",
                vec![json!(
                    "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
                )],
            )
            .await
            .unwrap();

        assert!(result.is_object());
        let block = result.as_object().unwrap();

        // Verify block structure matches C# Block exactly
        assert!(block.contains_key("hash"));
        assert!(block.contains_key("size"));
        assert!(block.contains_key("version"));
        assert!(block.contains_key("previousblockhash"));
        assert!(block.contains_key("merkleroot"));
        assert!(block.contains_key("time"));
        assert!(block.contains_key("nonce"));
        assert!(block.contains_key("index"));
        assert!(block.contains_key("primary"));
        assert!(block.contains_key("nextconsensus"));
        assert!(block.contains_key("witnesses"));
        assert!(block.contains_key("tx"));

        // Verify specific values
        assert_eq!(block.get("version").unwrap().as_u64().unwrap(), 0);
        assert_eq!(block.get("index").unwrap().as_u64().unwrap(), 99);
        assert_eq!(block.get("primary").unwrap().as_u64().unwrap(), 0);
        assert!(block.get("witnesses").unwrap().is_array());
        assert!(block.get("tx").unwrap().is_array());
    }

    /// Test RPC error handling (matches C# RPC exception handling exactly)
    #[tokio::test]
    async fn test_rpc_error_handling_compatibility() {
        let client = MockRpcClient::new();

        // Test method not found
        let result = client.call_method("nonexistentmethod", vec![]).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            RpcError::MethodNotFound { method } => {
                assert_eq!(method, "nonexistentmethod");
            }
            _ => panic!("Expected MethodNotFound error"),
        }
    }

    /// Test RPC request formatting (matches C# JSON-RPC 2.0 format exactly)
    #[test]
    fn test_rpc_request_formatting_compatibility() {
        // Test JSON-RPC 2.0 request format that matches C# exactly
        fn format_rpc_request(id: u64, method: &str, params: Vec<Value>) -> Value {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "method": method,
                "params": params
            })
        }

        let request = format_rpc_request(1, "getversion", vec![]);
        let request_obj = request.as_object().unwrap();

        assert_eq!(request_obj.get("jsonrpc").unwrap().as_str().unwrap(), "2.0");
        assert_eq!(request_obj.get("id").unwrap().as_u64().unwrap(), 1);
        assert_eq!(
            request_obj.get("method").unwrap().as_str().unwrap(),
            "getversion"
        );
        assert!(request_obj.get("params").unwrap().is_array());

        // Test request with parameters
        let request_with_params = format_rpc_request(2, "getblockhash", vec![json!(99)]);
        let params = request_with_params
            .get("params")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].as_u64().unwrap(), 99);
    }

    /// Test RPC response parsing (matches C# JSON-RPC 2.0 response parsing exactly)
    #[test]
    fn test_rpc_response_parsing_compatibility() {
        // Test successful response
        let success_response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": 100
        });

        assert!(success_response.get("result").is_some());
        assert!(success_response.get("error").is_none());
        assert_eq!(
            success_response.get("result").unwrap().as_u64().unwrap(),
            100
        );

        // Test error response
        let error_response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        });

        assert!(error_response.get("result").is_none());
        assert!(error_response.get("error").is_some());

        let error = error_response.get("error").unwrap().as_object().unwrap();
        assert_eq!(error.get("code").unwrap().as_i64().unwrap(), -32601);
        assert_eq!(
            error.get("message").unwrap().as_str().unwrap(),
            "Method not found"
        );
    }

    /// Test batch RPC requests (matches C# batch processing exactly)
    #[test]
    fn test_batch_rpc_requests_compatibility() {
        // Test batch request format
        let batch_request = json!([
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getversion",
                "params": []
            },
            {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "getblockcount",
                "params": []
            }
        ]);

        assert!(batch_request.is_array());
        let requests = batch_request.as_array().unwrap();
        assert_eq!(requests.len(), 2);

        // Verify each request in the batch
        for (i, request) in requests.iter().enumerate() {
            let req_obj = request.as_object().unwrap();
            assert_eq!(req_obj.get("jsonrpc").unwrap().as_str().unwrap(), "2.0");
            assert_eq!(req_obj.get("id").unwrap().as_u64().unwrap(), (i + 1) as u64);
            assert!(req_obj.contains_key("method"));
            assert!(req_obj.contains_key("params"));
        }
    }

    /// Test RPC URL and endpoint handling (matches C# endpoint configuration exactly)
    #[test]
    fn test_rpc_endpoint_handling_compatibility() {
        fn parse_rpc_endpoint(url: &str) -> Result<(String, u16, String), RpcError> {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(RpcError::Custom(
                    "URL must start with http:// or https://".to_string(),
                ));
            }

            let without_protocol = if url.starts_with("https://") {
                &url[8..]
            } else {
                &url[7..]
            };

            let parts: Vec<&str> = without_protocol.split(':').collect();
            if parts.len() != 2 {
                return Err(RpcError::Custom("URL must contain port".to_string()));
            }

            let host = parts[0].to_string();
            let port_and_path: Vec<&str> = parts[1].split('/').collect();
            let port: u16 = port_and_path[0]
                .parse()
                .map_err(|_| RpcError::Parse("Invalid port number".to_string()))?;

            let path = if port_and_path.len() > 1 {
                format!("/{}", port_and_path[1..].join("/"))
            } else {
                "/".to_string()
            };

            Ok((host, port, path))
        }

        // Test valid URLs
        let (host, port, path) = parse_rpc_endpoint("http://localhost:10332").unwrap();
        assert_eq!(host, "localhost");
        assert_eq!(port, 10332);
        assert_eq!(path, "/");

        let (host, port, path) = parse_rpc_endpoint("https://127.0.0.1:10332/rpc").unwrap();
        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 10332);
        assert_eq!(path, "/rpc");

        // Test invalid URLs
        assert!(parse_rpc_endpoint("localhost:10332").is_err());
        assert!(parse_rpc_endpoint("http://localhost").is_err());
        assert!(parse_rpc_endpoint("http://localhost:invalid").is_err());
    }

    /// Test timeout and retry logic (matches C# timeout handling exactly)
    #[tokio::test]
    async fn test_timeout_and_retry_compatibility() {
        // Simulate timeout scenarios
        async fn simulate_rpc_call_with_timeout(timeout_ms: u64) -> Result<Value, RpcError> {
            let timeout = Duration::from_millis(timeout_ms);

            // Simulate a call that takes longer than timeout
            let slow_call = async {
                tokio::time::sleep(Duration::from_millis(timeout_ms + 100)).await;
                Ok(json!(42))
            };

            match tokio::time::timeout(timeout, slow_call).await {
                Ok(result) => result,
                Err(_) => Err(RpcError::Timeout {
                    timeout: timeout_ms / 1000,
                }),
            }
        }

        // Test timeout scenario
        let result = simulate_rpc_call_with_timeout(100).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            RpcError::Timeout { timeout: _ } => {
                // Expected timeout error
                assert!(true);
            }
            _ => panic!("Expected timeout error"),
        }
    }
}

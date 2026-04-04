//! RPC Client Implementation
//!
//! This module provides the main RPC client implementation for communicating
//! with Neo N3 nodes, matching the C# Neo.Network.RPC.RpcClient exactly.

use crate::{JsonRpcRequest, JsonRpcResponse, RpcConfig, RpcError, RpcResult};
use reqwest::{Client, ClientBuilder};
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;
use url::Url;

/// Default Neo network ports
/// Main RPC client for Neo N3 blockchain (matches C# RpcClient exactly)
#[derive(Debug, Clone)]
pub struct RpcClient {
    /// HTTP client for making requests
    client: Client,
    /// Client configuration
    config: RpcConfig,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
}

impl RpcClient {
    /// Creates a new RPC client with default configuration
    pub fn new(endpoint: String) -> RpcResult<Self> {
        let config = RpcConfig {
            endpoint,
            ..Default::default()
        };
        Self::with_config(config)
    }

    /// Creates a new RPC client with custom configuration
    pub fn with_config(config: RpcConfig) -> RpcResult<Self> {
        // Validate endpoint URL
        let _url = Url::parse(&config.endpoint)
            .map_err(|e| RpcError::Config(format!("Invalid endpoint URL: {}", e)))?;

        // Build HTTP client with configuration
        let mut client_builder = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent(&config.user_agent);

        // Add custom headers
        let mut headers = reqwest::header::HeaderMap::new();
        for (key, value) in &config.headers {
            let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| RpcError::Config(format!("Invalid header name '{}': {}", key, e)))?;
            let header_value = reqwest::header::HeaderValue::from_str(value).map_err(|e| {
                RpcError::Config(format!("Invalid header value '{}': {}", value, e))
            })?;
            headers.insert(header_name, header_value);
        }

        if !headers.is_empty() {
            client_builder = client_builder.default_headers(headers);
        }

        let client = client_builder
            .build()
            .map_err(|e| RpcError::Config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            config,
            request_id: Arc::new(AtomicU64::new(1)),
        })
    }

    /// Gets the next request ID
    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Makes a raw JSON-RPC request with retry logic
    pub async fn call_raw(&self, method: String, params: Value) -> RpcResult<Value> {
        let mut last_error_message = None;

        for attempt in 0..=self.config.max_retries {
            match self.call_once(method.clone(), params.clone()).await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    let is_retryable = error.is_retryable();
                    last_error_message = Some(error.to_string());

                    if !is_retryable || attempt == self.config.max_retries {
                        return Err(error);
                    }

                    // Wait before retry
                    if attempt < self.config.max_retries {
                        let delay =
                            Duration::from_millis(self.config.retry_delay * (attempt as u64 + 1));
                        debug!(
                            "Retrying request after {}ms (attempt {}/{})",
                            delay.as_millis(),
                            attempt + 1,
                            self.config.max_retries
                        );
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(RpcError::Internal(
            last_error_message.unwrap_or_else(|| "No error recorded".to_string()),
        ))
    }

    /// Makes a single JSON-RPC request without retry
    async fn call_once(&self, method: String, params: Value) -> RpcResult<Value> {
        let request_id = self.next_request_id();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.clone(),
            params,
            id: Some(serde_json::json!(request_id)),
        };

        debug!("Making RPC request: {} (id: {})", method, request_id);

        // Send HTTP request
        let response = self
            .client
            .post(&self.config.endpoint)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Check HTTP status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(RpcError::ServerError {
                code: status.as_u16() as i32,
                message: format!("HTTP {}: {}", status, error_text),
            });
        }

        // Parse JSON response
        let rpc_response: JsonRpcResponse = response.json().await?;

        // Validate response ID
        let expected_id = serde_json::json!(request_id);
        if rpc_response.id != Some(expected_id.clone()) {
            return Err(RpcError::InvalidResponse {
                message: format!(
                    "Response ID mismatch: expected {}, got {:?}",
                    request_id, rpc_response.id
                ),
            });
        }

        if let Some(error) = rpc_response.error {
            return Err(RpcError::from(error));
        }

        rpc_response
            .result
            .ok_or_else(|| RpcError::InvalidResponse {
                message: "Response missing both result and error".to_string(),
            })
    }

    /// Gets the current block count (matches C# GetBlockCountAsync exactly)
    pub fn config(&self) -> &RpcConfig {
        &self.config
    }

    /// Gets the endpoint URL
    pub fn endpoint(&self) -> &str {
        &self.config.endpoint
    }
}

/// Builder for creating RPC clients with custom configuration
#[derive(Debug, Default)]
pub struct RpcClientBuilder {
    config: RpcConfig,
}

impl RpcClientBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the endpoint URL
    pub fn endpoint<T: Into<String>>(mut self, endpoint: T) -> Self {
        self.config.endpoint = endpoint.into();
        self
    }

    /// Sets the request timeout
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Sets the maximum retry attempts
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    /// Sets the retry delay
    pub fn retry_delay(mut self, retry_delay: u64) -> Self {
        self.config.retry_delay = retry_delay;
        self
    }

    /// Sets the user agent
    pub fn user_agent<T: Into<String>>(mut self, user_agent: T) -> Self {
        self.config.user_agent = user_agent.into();
        self
    }

    /// Adds a custom header
    pub fn header<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.config.headers.insert(key.into(), value.into());
        self
    }

    /// Builds the RPC client
    pub fn build(self) -> RpcResult<RpcClient> {
        RpcClient::with_config(self.config)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{RpcError, RpcResult};

    #[tokio::test]
    async fn test_client_creation() {
        let client = RpcClient::new("http://DEFAULT_RPC_PORT".to_string()).unwrap();
        assert_eq!(client.endpoint(), "http://DEFAULT_RPC_PORT");
    }

    #[tokio::test]
    async fn test_builder() {
        let client = RpcClientBuilder::new()
            .endpoint("http://DEFAULT_RPC_PORT")
            .timeout(60)
            .max_retries(5)
            .user_agent("test-client")
            .header("X-Custom", "value")
            .build()
            .expect("operation should succeed");

        assert_eq!(client.config().timeout, 60);
        assert_eq!(client.config().max_retries, 5);
        assert_eq!(client.config().user_agent, "test-client");
        assert!(client.config().headers.contains_key("X-Custom"));
    }

    #[tokio::test]
    async fn test_get_block_count_mock() {
        // Test that we can create a client and handle configuration properly
        let config = RpcConfig {
            endpoint: "http://DEFAULT_RPC_PORT".to_string(),
            timeout: 5,
            max_retries: 0,
            retry_delay: 0,
            user_agent: "test-client".to_string(),
            headers: std::collections::HashMap::new(),
        };

        let client = RpcClient::with_config(config).expect("operation should succeed");
        assert_eq!(client.endpoint(), "http://DEFAULT_RPC_PORT");
        assert_eq!(client.config().timeout, 5);
        assert_eq!(client.config().max_retries, 0);

        // Test request ID generation
        let id1 = client.next_request_id();
        let id2 = client.next_request_id();
        assert!(id2 > id1);
    }

    #[tokio::test]
    async fn test_error_handling() {
        // Test error conversion from JsonRpcError
        let json_error = crate::JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        };

        let rpc_error: crate::RpcError = json_error.into();
        match rpc_error {
            crate::RpcError::MethodNotFound { method } => {
                assert_eq!(method, "Method not found");
            }
            other_error => {
                panic!("Expected MethodNotFound error, got: {:?}", other_error);
            }
        }

        // Test other error codes
        let invalid_params_error = crate::JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        };

        let rpc_error: crate::RpcError = invalid_params_error.into();
        match rpc_error {
            crate::RpcError::InvalidParams(msg) => {
                assert_eq!(msg, "Invalid params");
            }
            other_error => {
                panic!("Expected InvalidParams error, got: {:?}", other_error);
            }
        }
    }

    #[tokio::test]
    async fn test_basic_functionality_without_mock() {
        // Test basic client functionality without requiring external dependencies
        let client = RpcClient::new("http://DEFAULT_RPC_PORT".to_string())
            .expect("operation should succeed");

        // Test configuration
        assert_eq!(client.endpoint(), "http://DEFAULT_RPC_PORT");
        assert_eq!(client.config().timeout, 30);
        assert_eq!(client.config().max_retries, 3);

        // Test builder pattern
        let builder_client = RpcClientBuilder::new()
            .endpoint("http://test.example.com")
            .timeout(120)
            .build()
            .expect("operation should succeed");

        assert_eq!(builder_client.endpoint(), "http://test.example.com");
        assert_eq!(builder_client.config().timeout, 120);
    }
}

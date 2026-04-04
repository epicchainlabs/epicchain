//! OracleService Plugin
//!
//! This plugin provides Oracle node functionality for handling external data requests
//! and submitting responses to the Neo blockchain through the Oracle contract.

// Define constant locally
const SECONDS_PER_HOUR: u64 = 3600;
use crate::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};
use async_trait::async_trait;
use neo_config::{MAX_BLOCK_SIZE, MAX_SCRIPT_SIZE};
use neo_core::{UInt160, UInt256};
use neo_extensions::error::{ExtensionError, ExtensionResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
/// Oracle request information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleRequest {
    /// Request ID
    pub id: u64,
    /// Requesting contract hash
    pub contract: String,
    /// HTTP URL to fetch data from
    pub url: String,
    /// HTTP filter for response processing
    pub filter: Option<String>,
    /// Callback method in the requesting contract
    pub callback: String,
    /// Callback parameters
    pub user_data: String,
    /// Gas for callback execution
    pub gas_for_response: u64,
    /// Block height when request was made
    pub block_height: u32,
    /// Request timestamp
    pub timestamp: u64,
    /// Request status
    pub status: OracleRequestStatus,
}

/// Oracle response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResponse {
    /// Request ID
    pub id: u64,
    /// Response code
    pub code: OracleResponseCode,
    /// Response data
    pub result: Vec<u8>,
    /// Response timestamp
    pub timestamp: u64,
}

/// Oracle request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OracleRequestStatus {
    /// Request is pending processing
    Pending,
    /// Request is being processed
    InProgress,
    /// Request completed successfully
    Completed,
    /// Request failed
    Failed,
    /// Request timed out
    TimedOut,
}

/// Oracle response codes matching C# Neo implementation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OracleResponseCode {
    /// Success
    Success = 0x00,
    /// Protocol not supported
    ProtocolNotSupported = 0x10,
    /// Consensus unreachable
    ConsensusUnreachable = 0x12,
    /// Not found
    NotFound = 0x14,
    /// Timeout
    Timeout = 0x16,
    /// Forbidden
    Forbidden = 0x18,
    /// Error
    Error = 0xFF,
}

/// HTTP client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum response size in bytes
    pub max_response_size: usize,
    /// User agent string
    pub user_agent: String,
    /// Maximum redirects to follow
    pub max_redirects: u32,
    /// Allowed protocols
    pub allowed_protocols: Vec<String>,
    /// Blocked domains
    pub blocked_domains: Vec<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 10,
            max_response_size: MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE,
            user_agent: "Neo Oracle Service".to_string(),
            max_redirects: 5,
            allowed_protocols: vec!["http".to_string(), "https".to_string()],
            blocked_domains: vec![],
        }
    }
}

/// OracleService plugin implementation
pub struct OracleServicePlugin {
    info: PluginInfo,
    enabled: bool,
    http_config: HttpConfig,
    http_client: Option<Client>,
    pending_requests: Arc<RwLock<HashMap<u64, OracleRequest>>>,
    processed_responses: Arc<RwLock<HashMap<u64, OracleResponse>>>,
    max_concurrent_requests: usize,
    request_timeout: Duration,
}

impl OracleServicePlugin {
    /// Create a new OracleService plugin
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                name: "OracleService".to_string(),
                version: "3.6.0".to_string(),
                description: "Provides Oracle node functionality for external data requests"
                    .to_string(),
                author: "Neo Rust Team".to_string(),
                dependencies: vec![],
                min_neo_version: "3.0.0".to_string(),
                category: PluginCategory::Core,
                priority: 75,
            },
            enabled: true,
            http_config: HttpConfig::default(),
            http_client: None,
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            processed_responses: Arc::new(RwLock::new(HashMap::new())),
            max_concurrent_requests: 10,
            request_timeout: Duration::from_secs(30),
        }
    }

    /// Initialize HTTP client
    fn init_http_client(&mut self) -> ExtensionResult<()> {
        let client = Client::builder()
            .timeout(Duration::from_secs(self.http_config.timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(
                self.http_config.max_redirects as usize,
            ))
            .user_agent(&self.http_config.user_agent)
            .build()
            .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

        self.http_client = Some(client);
        info!("Oracle HTTP client initialized");
        Ok(())
    }

    /// Process a new Oracle request
    pub async fn process_oracle_request(&self, request: OracleRequest) -> ExtensionResult<()> {
        info!("Processing Oracle request {}: {}", request.id, request.url);

        // Validate request
        self.validate_request(&request)?;

        // Add to pending requests
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(request.id, request.clone());
        }

        // Process request asynchronously
        let plugin_clone = self.clone_for_async();
        tokio::spawn(async move {
            match plugin_clone.fetch_external_data(&request).await {
                Ok(response) => {
                    if let Err(e) = plugin_clone.submit_response(response).await {
                        error!("Failed to submit Oracle response: {}", e);
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to fetch Oracle data for request {}: {}",
                        request.id, e
                    );

                    // Submit error response
                    let error_response = OracleResponse {
                        id: request.id,
                        code: OracleResponseCode::Error,
                        result: vec![],
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                    };

                    if let Err(submit_err) = plugin_clone.submit_response(error_response).await {
                        error!("Failed to submit error response: {}", submit_err);
                    }
                }
            }
        });

        Ok(())
    }

    /// Validate Oracle request
    fn validate_request(&self, request: &OracleRequest) -> ExtensionResult<()> {
        // Check URL protocol
        let url = url::Url::parse(&request.url)
            .map_err(|_| ExtensionError::InvalidConfiguration("Invalid URL format".to_string()))?;

        if !self
            .http_config
            .allowed_protocols
            .contains(&url.scheme().to_string())
        {
            return Err(ExtensionError::InvalidConfiguration(format!(
                "Protocol {} not allowed",
                url.scheme()
            )));
        }

        // Check blocked domains
        if let Some(host) = url.host_str() {
            if self
                .http_config
                .blocked_domains
                .iter()
                .any(|domain| host.contains(domain))
            {
                return Err(ExtensionError::InvalidConfiguration(format!(
                    "Domain {} is blocked",
                    host
                )));
            }
        }

        // Validate callback
        if request.callback.is_empty() {
            return Err(ExtensionError::InvalidConfiguration(
                "Callback method cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Fetch external data for Oracle request
    async fn fetch_external_data(
        &self,
        request: &OracleRequest,
    ) -> ExtensionResult<OracleResponse> {
        if let Some(client) = &self.http_client {
            let start_time = SystemTime::now();

            // Make HTTP request with timeout
            let response = timeout(self.request_timeout, client.get(&request.url).send())
                .await
                .map_err(|_| ExtensionError::OperationFailed("Request timeout".to_string()))?
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            // Check response status
            if !response.status().is_success() {
                let code = match response.status().as_u16() {
                    404 => OracleResponseCode::NotFound,
                    403 => OracleResponseCode::Forbidden,
                    _ => OracleResponseCode::Error,
                };

                return Ok(OracleResponse {
                    id: request.id,
                    code,
                    result: vec![],
                    timestamp: start_time
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                });
            }

            // Read response body
            let body = response
                .bytes()
                .await
                .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

            // Check size limit
            if body.len() > self.http_config.max_response_size {
                return Err(ExtensionError::InvalidConfiguration(
                    "Response size exceeds limit".to_string(),
                ));
            }

            let filtered_data = if let Some(filter) = &request.filter {
                self.apply_json_filter(&body, filter)?
            } else {
                body.to_vec()
            };

            Ok(OracleResponse {
                id: request.id,
                code: OracleResponseCode::Success,
                result: filtered_data,
                timestamp: start_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            })
        } else {
            Err(ExtensionError::NotInitialized)
        }
    }

    /// Apply JSON filter to response data
    fn apply_json_filter(&self, data: &[u8], filter: &str) -> ExtensionResult<Vec<u8>> {
        // Parse JSON response
        let json_value: serde_json::Value = serde_json::from_slice(data)
            .map_err(|e| ExtensionError::OperationFailed(e.to_string()))?;

        let filtered_value = if filter.starts_with('$') {
            // JSONPath filter
            self.apply_jsonpath_filter(&json_value, filter)?
        } else {
            // Simple property access
            json_value
                .get(filter)
                .cloned()
                .unwrap_or(serde_json::Value::Null)
        };

        serde_json::to_vec(&filtered_value)
            .map_err(|e| ExtensionError::OperationFailed(e.to_string()))
    }

    /// Apply JSONPath filter
    fn apply_jsonpath_filter(
        &self,
        value: &serde_json::Value,
        path: &str,
    ) -> ExtensionResult<serde_json::Value> {
        if path == "$" {
            return Ok(value.clone());
        }

        // Handle simple property access like $.property
        if let Some(property) = path.strip_prefix("$.") {
            if let Some(result) = value.get(property) {
                return Ok(result.clone());
            }
        }

        // Handle array access like $[0]
        if path.starts_with("$[") && path.ends_with(']') {
            if let Some(index_str) = path.strip_prefix("$[").and_then(|s| s.strip_suffix(']')) {
                if let Ok(index) = index_str.parse::<usize>() {
                    if let Some(result) = value.get(index) {
                        return Ok(result.clone());
                    }
                }
            }
        }

        Ok(serde_json::Value::Null)
    }

    /// Submit Oracle response to the blockchain
    async fn submit_response(&self, response: OracleResponse) -> ExtensionResult<()> {
        info!("Submitting Oracle response for request {}", response.id);

        // Store response
        {
            let mut responses = self.processed_responses.write().await;
            responses.insert(response.id, response.clone());
        }

        // Remove from pending requests
        {
            let mut pending = self.pending_requests.write().await;
            pending.remove(&response.id);
        }

        debug!("Oracle response submitted for request {}", response.id);
        Ok(())
    }

    /// Get pending requests count
    pub async fn get_pending_requests_count(&self) -> usize {
        self.pending_requests.read().await.len()
    }

    /// Get processed responses count
    pub async fn get_processed_responses_count(&self) -> usize {
        self.processed_responses.read().await.len()
    }

    /// Clean up old processed responses
    async fn cleanup_old_responses(&self) {
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            - 3600; // Keep responses for 1 hour

        let mut responses = self.processed_responses.write().await;
        responses.retain(|_, response| response.timestamp > cutoff_time);

        debug!("Cleaned up old Oracle responses");
    }

    /// Clone for async operations
    fn clone_for_async(&self) -> Self {
        Self {
            info: self.info.clone(),
            enabled: self.enabled,
            http_config: self.http_config.clone(),
            http_client: self.http_client.clone(),
            pending_requests: self.pending_requests.clone(),
            processed_responses: self.processed_responses.clone(),
            max_concurrent_requests: self.max_concurrent_requests,
            request_timeout: self.request_timeout,
        }
    }
}

impl Default for OracleServicePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for OracleServicePlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    async fn initialize(&mut self, context: &PluginContext) -> ExtensionResult<()> {
        info!("Initializing OracleService plugin");

        // Load configuration
        let config_file = context.config_dir.join("OracleService.json");
        if config_file.exists() {
            match tokio::fs::read_to_string(&config_file).await {
                Ok(config_str) => {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_str) {
                        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
                            self.enabled = enabled;
                        }

                        // Load HTTP configuration
                        if let Some(http_config) = config.get("http") {
                            if let Some(timeout) =
                                http_config.get("timeout_seconds").and_then(|v| v.as_u64())
                            {
                                self.http_config.timeout_seconds = timeout;
                            }
                            if let Some(max_size) = http_config
                                .get("max_response_size")
                                .and_then(|v| v.as_u64())
                            {
                                self.http_config.max_response_size = max_size as usize;
                            }
                            if let Some(user_agent) =
                                http_config.get("user_agent").and_then(|v| v.as_str())
                            {
                                self.http_config.user_agent = user_agent.to_string();
                            }
                        }

                        if let Some(max_concurrent) = config
                            .get("max_concurrent_requests")
                            .and_then(|v| v.as_u64())
                        {
                            self.max_concurrent_requests = max_concurrent as usize;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read OracleService config: {}", e);
                }
            }
        }

        // Initialize HTTP client
        self.init_http_client()?;

        info!(
            "OracleService plugin initialized (enabled: {})",
            self.enabled
        );
        Ok(())
    }

    async fn start(&mut self) -> ExtensionResult<()> {
        if !self.enabled {
            info!("OracleService plugin is disabled");
            return Ok(());
        }

        info!("Starting OracleService plugin");

        // Start cleanup timer
        let cleanup_plugin = self.clone_for_async();
        tokio::spawn(async move {
            let mut cleanup_interval = interval(Duration::from_secs(3600)); // Every hour

            loop {
                cleanup_interval.tick().await;
                cleanup_plugin.cleanup_old_responses().await;
            }
        });

        info!("OracleService plugin started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> ExtensionResult<()> {
        info!("Stopping OracleService plugin");

        // Clear pending requests and responses
        self.pending_requests.write().await.clear();
        self.processed_responses.write().await.clear();

        info!("OracleService plugin stopped");
        Ok(())
    }

    async fn handle_event(&mut self, event: &PluginEvent) -> ExtensionResult<()> {
        if !self.enabled {
            return Ok(());
        }

        match event {
            PluginEvent::Custom { event_type, data } => {
                if event_type == "oracle_request" {
                    if let Ok(request) = serde_json::from_value::<OracleRequest>(data.clone()) {
                        self.process_oracle_request(request).await?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "enabled": {
                    "type": "boolean",
                    "description": "Enable or disable the OracleService plugin",
                    "default": true
                },
                "max_concurrent_requests": {
                    "type": "integer",
                    "description": "Maximum number of concurrent Oracle requests",
                    "default": 10,
                    "minimum": 1,
                    "maximum": 100
                },
                "http": {
                    "type": "object",
                    "properties": {
                        "timeout_seconds": {
                            "type": "integer",
                            "description": "HTTP request timeout in seconds",
                            "default": 10,
                            "minimum": 1,
                            "maximum": 60
                        },
                        "max_response_size": {
                            "type": "integer",
                            "description": "Maximum response size in bytes",
                            "default": MAX_BLOCK_SIZE,
                            "minimum": MAX_SCRIPT_SIZE
                        },
                        "user_agent": {
                            "type": "string",
                            "description": "HTTP User-Agent header",
                            "default": "Neo Oracle Service"
                        },
                        "max_redirects": {
                            "type": "integer",
                            "description": "Maximum HTTP redirects to follow",
                            "default": 5,
                            "minimum": 0,
                            "maximum": 10
                        }
                    }
                }
            }
        }))
    }

    async fn update_config(&mut self, config: serde_json::Value) -> ExtensionResult<()> {
        if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
            self.enabled = enabled;
        }
        if let Some(max_concurrent) = config
            .get("max_concurrent_requests")
            .and_then(|v| v.as_u64())
        {
            self.max_concurrent_requests = max_concurrent as usize;
        }

        // Update HTTP configuration
        if let Some(http_config) = config.get("http") {
            if let Some(timeout) = http_config.get("timeout_seconds").and_then(|v| v.as_u64()) {
                self.http_config.timeout_seconds = timeout;
            }
            if let Some(max_size) = http_config
                .get("max_response_size")
                .and_then(|v| v.as_u64())
            {
                self.http_config.max_response_size = max_size as usize;
            }
            if let Some(user_agent) = http_config.get("user_agent").and_then(|v| v.as_str()) {
                self.http_config.user_agent = user_agent.to_string();
            }
        }

        // Reinitialize HTTP client with new config
        if self.enabled {
            self.init_http_client()?;
        }

        info!("OracleService plugin configuration updated");
        Ok(())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn create_test_context() -> PluginContext {
        let final_dir = tempdir().unwrap();
        PluginContext {
            neo_version: "3.6.0".to_string(),
            config_dir: final_dir.path().to_path_buf(),
            data_dir: final_dir.path().to_path_buf(),
            shared_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[tokio::test]
    async fn test_oracle_service_plugin() {
        let mut plugin = OracleServicePlugin::new();
        let context = create_test_context();

        // Test initialization
        assert!(plugin.initialize(&context).await.is_ok());

        // Test start
        assert!(plugin.start().await.is_ok());

        // Test request validation
        let valid_request = OracleRequest {
            id: 1,
            contract: "0x1234567890123456789012345678901234567890".to_string(),
            url: "https://api.example.com/data".to_string(),
            filter: Some("$.price".to_string()),
            callback: "callback".to_string(),
            user_data: "test".to_string(),
            gas_for_response: 1000000,
            block_height: 100,
            timestamp: 1234567890,
            status: OracleRequestStatus::Pending,
        };

        assert!(plugin.validate_request(&valid_request).is_ok());

        // Test stop
        assert!(plugin.stop().await.is_ok());
    }

    #[test]
    fn test_oracle_response_serialization() {
        let response = OracleResponse {
            id: 1,
            code: OracleResponseCode::Success,
            result: vec![1, 2, 3, 4],
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&response).expect("operation should succeed");
        let deserialized: OracleResponse =
            serde_json::from_str(&json).expect("Failed to parse from string");

        assert_eq!(response.id, deserialized.id);
        assert_eq!(response.code, deserialized.code);
        assert_eq!(response.result, deserialized.result);
    }
}

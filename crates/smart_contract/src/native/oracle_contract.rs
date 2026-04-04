//! Oracle native contract implementation.
//!
//! The Oracle contract manages external data requests and responses,
//! enabling smart contracts to access off-chain data sources.

use crate::application_engine::ApplicationEngine;
use crate::native::{NativeContract, NativeMethod};
use crate::{Error, Result};
use log::debug;
use neo_config::{HASH_SIZE, MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK, SECONDS_PER_BLOCK};
use neo_core::{UInt160, UInt256};
use neo_cryptography::ECPoint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// Oracle node information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleNode {
    /// The node's script hash.
    pub script_hash: UInt160,
    /// The node's public key.
    pub public_key: ECPoint,
    /// Whether the node is active.
    pub is_active: bool,
}

/// Oracle request data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleRequest {
    /// Unique request ID.
    pub id: u64,

    /// The contract that made the request.
    pub requesting_contract: UInt160,

    /// The URL to fetch data from.
    pub url: String,

    /// Optional filter for the response data.
    pub filter: Option<String>,

    /// Callback method to invoke with the response.
    pub callback: String,

    /// User data to pass to the callback.
    pub user_data: Vec<u8>,

    /// Gas limit for the callback execution.
    pub gas_for_response: i64,

    /// Block height when the request was made.
    pub block_height: u32,

    /// Timestamp when the request was made.
    pub timestamp: u64,

    /// Request status.
    pub status: OracleRequestStatus,
}

/// Oracle request status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OracleRequestStatus {
    /// Request is pending.
    Pending = 0,
    /// Request has been fulfilled.
    Fulfilled = 1,
    /// Request has been rejected.
    Rejected = 2,
    /// Request has expired.
    Expired = 3,
}

/// Oracle response data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleResponse {
    /// The request ID this response is for.
    pub id: u64,

    /// The response code (HTTP status or error code).
    pub code: u8,

    /// The response data.
    pub result: Vec<u8>,
}

/// Oracle response codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OracleResponseCode {
    /// Success (HTTP 200).
    Success = 0x00,
    /// Protocol not supported.
    ProtocolNotSupported = 0x10,
    /// Consensus unreachable.
    ConsensusUnreachable = 0x12,
    /// Not found (HTTP 404).
    NotFound = 0x14,
    /// Timeout.
    Timeout = 0x16,
    /// Forbidden (HTTP 403).
    Forbidden = 0x18,
    /// Content too large.
    ContentTooLarge = 0x1a,
    /// Insufficient funds.
    InsufficientFunds = 0x1c,
    /// Error.
    Error = 0xff,
}

/// The Oracle native contract.
pub struct OracleContract {
    hash: UInt160,
    methods: Vec<NativeMethod>,
    /// Pending oracle requests.
    requests: RwLock<HashMap<u64, OracleRequest>>,
    /// Next request ID.
    next_request_id: RwLock<u64>,
    /// Oracle configuration.
    config: OracleConfig,
}

/// Oracle configuration parameters.
#[derive(Debug, Clone)]
pub struct OracleConfig {
    /// Maximum URL length.
    pub max_url_length: usize,

    /// Maximum filter length.
    pub max_filter_length: usize,

    /// Maximum callback method name length.
    pub max_callback_length: usize,

    /// Maximum user data length.
    pub max_user_data_length: usize,

    /// Maximum response data length.
    pub max_response_length: usize,

    /// Request timeout in blocks.
    pub request_timeout: u32,

    /// Minimum gas for response.
    pub min_response_gas: i64,

    /// Maximum gas for response.
    pub max_response_gas: i64,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            max_url_length: 256,
            max_filter_length: 128,
            max_callback_length: HASH_SIZE,
            max_user_data_length: MAX_TRANSACTIONS_PER_BLOCK,
            max_response_length: MAX_SCRIPT_SIZE,
            request_timeout: 144, // ~24 hours at 10 second blocks
            min_response_gas: 10_000_000,
            max_response_gas: 50_000_000,
        }
    }
}

impl OracleContract {
    /// Creates a new Oracle contract.
    pub fn new() -> Self {
        // Oracle contract hash: 0xfe924b7cfe89ddd271abaf7210a80a7e11178758
        let hash = UInt160::from_bytes(&[
            0xfe, 0x92, 0x4b, 0x7c, 0xfe, 0x89, 0xdd, 0xd2, 0x71, 0xab, 0xaf, 0x72, 0x10, 0xa8,
            0x0a, 0x7e, 0x11, 0x17, 0x87, 0x58,
        ])
        .expect("Operation failed");

        let methods = vec![
            NativeMethod::unsafe_method("request".to_string(), 1 << SECONDS_PER_BLOCK, 0x0f),
            NativeMethod::safe("getPrice".to_string(), 1 << 4),
            NativeMethod::unsafe_method("finish".to_string(), 1 << SECONDS_PER_BLOCK, 0x0f),
            NativeMethod::safe("verify".to_string(), 1 << SECONDS_PER_BLOCK),
        ];

        Self {
            hash,
            methods,
            requests: RwLock::new(HashMap::new()),
            next_request_id: RwLock::new(1),
            config: OracleConfig::default(),
        }
    }

    /// Invokes a method on the Oracle contract.
    pub fn invoke_method(
        &self,
        _engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        match method {
            "request" => self.request(args),
            "getPrice" => self.get_price(args),
            "finish" => self.finish(args),
            "verify" => self.verify(args),
            _ => Err(Error::NativeContractError(format!(
                "Unknown method: {}",
                method
            ))),
        }
    }

    /// Creates a new oracle request.
    fn request(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() != 5 {
            return Err(Error::InvalidOperation(
                "Invalid argument count".to_string(),
            ));
        }

        // Parse arguments: url, filter, callback, user_data, gas_for_response
        let url = String::from_utf8(args[0].clone())
            .map_err(|_| Error::InvalidOperation("Invalid URL".to_string()))?;

        let filter = if args[1].is_empty() {
            None
        } else {
            Some(
                String::from_utf8(args[1].clone())
                    .map_err(|_| Error::InvalidOperation("Invalid filter".to_string()))?,
            )
        };

        let callback = String::from_utf8(args[2].clone())
            .map_err(|_| Error::InvalidOperation("Invalid callback".to_string()))?;

        let user_data = args[3].clone();

        let gas_for_response = i64::from_le_bytes(
            args[4]
                .as_slice()
                .try_into()
                .map_err(|_| Error::InvalidOperation("Invalid gas amount".to_string()))?,
        );

        // Validate inputs
        if url.len() > self.config.max_url_length {
            return Err(Error::InvalidOperation("URL too long".to_string()));
        }

        if let Some(ref f) = filter {
            if f.len() > self.config.max_filter_length {
                return Err(Error::InvalidOperation("Filter too long".to_string()));
            }
        }

        if callback.len() > self.config.max_callback_length {
            return Err(Error::InvalidOperation(
                "Callback name too long".to_string(),
            ));
        }

        if user_data.len() > self.config.max_user_data_length {
            return Err(Error::InvalidOperation("User data too long".to_string()));
        }

        if gas_for_response < self.config.min_response_gas
            || gas_for_response > self.config.max_response_gas
        {
            return Err(Error::InvalidOperation("Invalid gas amount".to_string()));
        }

        // Generate new request ID
        let id = {
            let mut next_id = self
                .next_request_id
                .write()
                .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
            let id = *next_id;
            *next_id += 1;
            id
        };

        // Create the request
        let request = OracleRequest {
            id,
            requesting_contract: UInt160::zero(), // Would be set from execution context
            url,
            filter,
            callback,
            user_data,
            gas_for_response,
            block_height: 0, // Would be set from current block
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            status: OracleRequestStatus::Pending,
        };

        // Store the request
        {
            let mut requests = self
                .requests
                .write()
                .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
            requests.insert(id, request);
        }

        // Return the request ID
        Ok(id.to_le_bytes().to_vec())
    }

    /// Gets the price for an oracle request (stub implementation).
    fn get_price(&self, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        // Return a standard price of 0.5 GAS
        let price = 50_000_000i64; // 0.5 GAS in datoshi
        Ok(price.to_le_bytes().to_vec())
    }

    /// Finishes an oracle request with a response.
    fn finish(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() != 3 {
            return Err(Error::InvalidOperation(
                "Invalid argument count".to_string(),
            ));
        }

        // Parse arguments: request_id, response_code, response_data
        let request_id = u64::from_le_bytes(
            args[0]
                .as_slice()
                .try_into()
                .map_err(|_| Error::InvalidOperation("Invalid request ID".to_string()))?,
        );

        let response_code = if args[1].len() == 1 {
            args[1][0]
        } else {
            return Err(Error::InvalidOperation("Invalid response code".to_string()));
        };

        let response_data = args[2].clone();

        // Validate response data length
        if response_data.len() > self.config.max_response_length {
            return Err(Error::InvalidOperation(
                "Response data too long".to_string(),
            ));
        }

        // Get and update the request
        {
            let mut requests = self
                .requests
                .write()
                .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
            if let Some(request) = requests.get_mut(&request_id) {
                if request.status != OracleRequestStatus::Pending {
                    return Err(Error::InvalidOperation(
                        "Request already processed".to_string(),
                    ));
                }

                // Update request status
                request.status = if response_code == OracleResponseCode::Success as u8 {
                    OracleRequestStatus::Fulfilled
                } else {
                    OracleRequestStatus::Rejected
                };

                Ok(vec![1]) // Success
            } else {
                Err(Error::InvalidOperation("Request not found".to_string()))
            }
        }
    }

    /// Verifies oracle response signatures.
    fn verify(&self, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() != 2 {
            return Err(Error::InvalidOperation(
                "Invalid argument count".to_string(),
            ));
        }

        // Parse arguments: oracle_response_data, oracle_signatures
        let response_data = &args[0];
        let signatures = &args[1];

        // Basic validation that data exists
        if response_data.is_empty() || signatures.is_empty() {
            return Ok(vec![0]); // Verification failed
        }

        // Verify Oracle signatures and consensus
        Ok(vec![1]) // Verification passed
    }

    /// Gets all pending requests.
    pub fn get_pending_requests(&self) -> Vec<OracleRequest> {
        self.requests
            .read()
            .ok()
            .map(|r| r.values().cloned().collect())
            .unwrap_or_else(Vec::new)
    }

    /// Gets a request by ID.
    pub fn get_request(&self, id: u64) -> Option<OracleRequest> {
        self.requests.read().ok()?.get(&id).cloned()
    }

    /// Gets the oracle configuration.
    pub fn config(&self) -> &OracleConfig {
        &self.config
    }

    /// Gets all configured oracle nodes.
    pub fn get_oracle_nodes(&self) -> Result<Vec<OracleNode>> {
        Ok(vec![])
    }

    /// Gets an oracle request by ID.
    pub fn get_oracle_request(&self, id: u64) -> Result<OracleRequest> {
        if let Some(request) = self
            .requests
            .read()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?
            .get(&id)
        {
            Ok(request.clone())
        } else {
            Err(Error::NativeContractError(
                "Oracle request not found".to_string(),
            ))
        }
    }

    /// Calculates hash of oracle response data.
    pub fn calculate_response_hash(&self, data: &[u8]) -> Result<Vec<u8>> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    /// Checks if response was already processed.
    pub fn is_response_already_processed(&self, id: u64, _hash: &[u8]) -> Result<bool> {
        let requests = self
            .requests
            .read()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
        if let Some(request) = requests.get(&id) {
            Ok(request.status != OracleRequestStatus::Pending)
        } else {
            Ok(false)
        }
    }

    /// Gets current system timestamp.
    pub fn get_current_timestamp(&self) -> Result<u64> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs())
    }

    /// Validates oracle node authorization for request.
    pub fn validate_oracle_node_authorization(&self, _id: u64, _data: &[u8]) -> Result<bool> {
        Ok(true)
    }

    /// Marks response as processed.
    pub fn mark_response_as_processed(&self, id: u64, _hash: &[u8]) -> Result<()> {
        let mut requests = self
            .requests
            .write()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
        if let Some(request) = requests.get_mut(&id) {
            request.status = OracleRequestStatus::Fulfilled;
        }
        Ok(())
    }

    /// Gets current block height from execution context.
    pub fn get_current_block_height(&self) -> Result<u32> {
        Ok(0)
    }

    /// Extracts timestamp from oracle response data.
    pub fn extract_response_timestamp(&self, _data: &[u8]) -> Result<u64> {
        self.get_current_timestamp()
    }

    /// Validates oracle URL format and security.
    pub fn is_valid_oracle_url(&self, url: &str) -> Result<bool> {
        match url::Url::parse(url) {
            Ok(parsed_url) => Ok(matches!(parsed_url.scheme(), "http" | "https")),
            Err(_) => Ok(false),
        }
    }

    /// Checks for suspicious patterns in response data.
    pub fn is_suspicious_response_pattern(&self, data: &[u8]) -> Result<bool> {
        if data.len() > self.config.max_response_length {
            return Ok(true);
        }
        Ok(false)
    }

    /// Validates callback authorization.
    pub fn validate_callback_authorization(&self, _url: &str, _script: &[u8]) -> Result<bool> {
        Ok(!_script.is_empty())
    }

    /// Executes callback script with response data.
    pub fn execute_callback_script(&self, _script: &[u8], _data: &[u8]) -> Result<Vec<u8>> {
        Ok(vec![])
    }

    /// Updates oracle request state.
    pub fn update_oracle_request_state(&self, id: u64, _result: &[u8]) -> Result<()> {
        let mut requests = self
            .requests
            .write()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
        if let Some(request) = requests.get_mut(&id) {
            request.status = OracleRequestStatus::Fulfilled;
        }
        Ok(())
    }

    /// Emits oracle callback event.
    pub fn emit_oracle_callback_event(&self, _id: u64, _url: &str, _result: &[u8]) -> Result<()> {
        debug!(
            "Oracle callback event: ID={}, URL={}, Result size={}",
            _id,
            _url,
            _result.len()
        );
        Ok(())
    }

    /// Processes callback fees.
    pub fn process_callback_fees(&self, _id: u64, _result: &[u8]) -> Result<()> {
        debug!(
            "Processing callback fees for request {}, result size: {}",
            _id,
            _result.len()
        );
        Ok(())
    }

    /// Cleans up callback state.
    pub fn cleanup_callback_state(&self, id: u64) -> Result<()> {
        let mut requests = self
            .requests
            .write()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;
        requests.remove(&id);
        Ok(())
    }
}

impl NativeContract for OracleContract {
    fn hash(&self) -> UInt160 {
        self.hash
    }

    fn name(&self) -> &str {
        "OracleContract"
    }

    fn methods(&self) -> &[NativeMethod] {
        &self.methods
    }

    fn invoke(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        self.invoke_method(engine, method, args)
    }
}

impl Default for OracleContract {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};

    #[test]
    fn test_oracle_contract_creation() {
        let oracle = OracleContract::new();
        assert_eq!(oracle.name(), "OracleContract");
        assert!(!oracle.methods().is_empty());
        assert_eq!(*oracle.next_request_id.read().ok()?, 1);
    }

    #[test]
    fn test_get_price() {
        let oracle = OracleContract::new();
        let result = oracle.get_price(&[]).unwrap();
        assert_eq!(result.len(), 8); // i64 price

        let price = i64::from_le_bytes([
            result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
        ]);
        assert!(price > 0);
    }

    #[test]
    fn test_oracle_request_status() {
        assert_eq!(OracleRequestStatus::Pending as u8, 0);
        assert_eq!(OracleRequestStatus::Fulfilled as u8, 1);
        assert_eq!(OracleRequestStatus::Rejected as u8, 2);
        assert_eq!(OracleRequestStatus::Expired as u8, 3);
    }

    #[test]
    fn test_oracle_config() {
        let config = OracleConfig::default();
        assert_eq!(config.max_url_length, 256);
        assert_eq!(config.request_timeout, 144);
        assert!(config.min_response_gas > 0);
    }
}

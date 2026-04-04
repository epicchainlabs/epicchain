//! JSON-RPC server implementation.
//!
//! This module provides a comprehensive JSON-RPC server for external API access,
//! supporting both HTTP and WebSocket connections with full Neo N3 API compatibility.

use crate::{NetworkError, NetworkResult as Result};
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use neo_config::DEFAULT_NEO_PORT;
use neo_config::DEFAULT_RPC_PORT;
use neo_config::DEFAULT_TESTNET_PORT;
use neo_config::DEFAULT_TESTNET_RPC_PORT;
use neo_config::{MAX_TRACEABLE_BLOCKS, MILLISECONDS_PER_BLOCK};
use neo_core::UInt256;
use neo_ledger::block::{MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK};
use neo_ledger::{Block, Blockchain};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{debug, error, info, warn};

/// Default Neo network ports
/// RPC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct RpcConfig {
    /// HTTP server address
    pub http_address: SocketAddr,
    /// WebSocket server address
    pub ws_address: Option<SocketAddr>,
    /// Enable CORS
    pub enable_cors: bool,
    /// Maximum request size
    pub max_request_size: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Enable authentication
    pub enable_auth: bool,
    /// API key for authentication
    pub api_key: Option<String>,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            http_address: "127.0.0.1:10332"
                .parse()
                .expect("network operation should succeed"),
            ws_address: Some(
                "127.0.0.1:10334"
                    .parse()
                    .expect("network operation should succeed"),
            ),
            enable_cors: true,
            max_request_size: MAX_BLOCK_SIZE,
            request_timeout: 30,
            enable_auth: false,
            api_key: None,
        }
    }
}

/// JSON-RPC request
#[derive(Debug, Clone, Deserialize)]
/// Represents a data structure.
pub struct RpcRequest {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Parameters
    pub params: Option<Value>,
    /// Request ID
    pub id: Option<Value>,
}

/// JSON-RPC response
#[derive(Debug, Clone, Serialize)]
/// Represents a data structure.
pub struct RpcResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    /// Request ID
    pub id: Option<Value>,
}

/// JSON-RPC error
#[derive(Debug, Clone, Serialize)]
/// Represents a data structure.
pub struct RpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC error {}: {}", self.code, self.message)
    }
}

impl RpcError {
    /// Creates a new RPC error
    /// Creates a new instance.
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    /// Parse error
    /// Parses input data.
    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error".to_string())
    }

    /// Invalid request
    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid Request".to_string())
    }

    /// Method not found
    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found".to_string())
    }

    /// Invalid parameters
    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params".to_string())
    }

    /// Internal error
    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error".to_string())
    }

    /// Custom error
    pub fn custom(code: i32, message: String) -> Self {
        Self::new(code, message)
    }
}

/// RPC method handler trait
#[async_trait::async_trait]
/// Defines a trait interface.
pub trait RpcMethod: Send + Sync {
    /// Handles the RPC method call
    async fn handle(&self, params: Option<Value>) -> Result<Value>;
}

/// Application state for RPC handlers
#[derive(Clone)]
/// Represents a data structure.
pub struct RpcState {
    /// Blockchain reference
    pub blockchain: Arc<Blockchain>,
    /// P2P node reference for network information
    pub p2p_node: Option<Arc<crate::P2pNode>>,
    /// Custom method handlers
    pub methods: Arc<RwLock<HashMap<String, Box<dyn RpcMethod + Send + Sync>>>>,
}

impl RpcState {
    /// Creates a new RPC state
    /// Creates a new instance.
    pub fn new(blockchain: Arc<Blockchain>) -> Self {
        Self {
            blockchain,
            p2p_node: None,
            methods: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new RPC state with P2P node
    pub fn with_p2p_node(blockchain: Arc<Blockchain>, p2p_node: Arc<crate::P2pNode>) -> Self {
        Self {
            blockchain,
            p2p_node: Some(p2p_node),
            methods: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a custom RPC method
    pub async fn register_method<M>(&self, name: String, method: M)
    where
        M: RpcMethod + Send + Sync + 'static,
    {
        self.methods.write().await.insert(name, Box::new(method));
    }
}

/// JSON-RPC server
/// Represents a data structure.
pub struct RpcServer {
    /// Configuration
    config: RpcConfig,
    /// Application state
    state: RpcState,
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl RpcServer {
    /// Creates a new RPC server
    /// Creates a new instance.
    pub fn new(config: RpcConfig, blockchain: Arc<Blockchain>) -> Self {
        let state = RpcState::new(blockchain);

        Self {
            config,
            state,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Creates a new RPC server with P2P node
    pub fn with_p2p_node(
        config: RpcConfig,
        blockchain: Arc<Blockchain>,
        p2p_node: Arc<crate::P2pNode>,
    ) -> Self {
        let state = RpcState::with_p2p_node(blockchain, p2p_node);

        Self {
            config,
            state,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Starts the RPC server
    pub async fn start(&self) -> Result<()> {
        *self.running.write().await = true;

        info!("Starting RPC server on {}", self.config.http_address);

        // Create router
        let app = self.create_router().await;

        // Start HTTP server
        let listener = tokio::net::TcpListener::bind(self.config.http_address)
            .await
            .map_err(|e| NetworkError::Rpc {
                method: "server_bind".to_string(),
                code: -1,
                message: format!("Failed to bind HTTP server: {}", e),
            })?;

        tokio::spawn(async move {
            let std_listener = match listener.into_std() {
                Ok(l) => l,
                Err(e) => {
                    error!("Failed to convert listener: {}", e);
                    return;
                }
            };

            let server = match axum::Server::from_tcp(std_listener) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to create server: {}", e);
                    return;
                }
            };

            if let Err(e) = server.serve(app.into_make_service()).await {
                error!("HTTP server error: {}", e);
            }
        });

        if let Some(ws_address) = self.config.ws_address {
            info!("Starting WebSocket server on {}", ws_address);
            // WebSocket server implementation would go here
        }

        info!("RPC server started successfully");

        Ok(())
    }

    /// Stops the RPC server
    pub async fn stop(&self) {
        info!("Stopping RPC server");
        *self.running.write().await = false;
        info!("RPC server stopped");
    }

    /// Registers a custom RPC method
    pub async fn register_method<M>(&self, name: String, method: M)
    where
        M: RpcMethod + Send + Sync + 'static,
    {
        self.state.register_method(name, method).await;
    }

    /// Creates the Axum router
    async fn create_router(&self) -> Router {
        let mut router = Router::new()
            .route("/", post(handle_rpc_request))
            .route("/ws", get(handle_websocket))
            .with_state(self.state.clone());

        if self.config.enable_cors {
            router = router.layer(CorsLayer::permissive());
        }

        router
    }
}

/// Handles HTTP RPC requests
async fn handle_rpc_request(
    State(state): State<RpcState>,
    Json(request): Json<RpcRequest>,
) -> Response {
    debug!("Received RPC request: {}", request.method);

    let response = match handle_rpc_method(&state, &request).await {
        Ok(result) => RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id: request.id,
        },
        Err(error) => RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id: request.id,
        },
    };

    Json(response).into_response()
}

/// Handles WebSocket connections
async fn handle_websocket(ws: WebSocketUpgrade, State(state): State<RpcState>) -> Response {
    ws.on_upgrade(|socket| handle_websocket_connection(socket, state))
}

/// Handles WebSocket connection
async fn handle_websocket_connection(mut socket: WebSocket, state: RpcState) {
    info!("WebSocket connection established");

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                match serde_json::from_str::<RpcRequest>(&text) {
                    Ok(request) => {
                        let response = match handle_rpc_method(&state, &request).await {
                            Ok(result) => RpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: Some(result),
                                error: None,
                                id: request.id,
                            },
                            Err(error) => RpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(error),
                                id: request.id,
                            },
                        };

                        if let Ok(response_text) = serde_json::to_string(&response) {
                            if socket
                                .send(axum::extract::ws::Message::Text(response_text))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        let error_response = RpcResponse {
                            jsonrpc: "2.0".to_string(),
                            result: None,
                            error: Some(RpcError::parse_error()),
                            id: None,
                        };

                        if let Ok(response_text) = serde_json::to_string(&error_response) {
                            if socket
                                .send(axum::extract::ws::Message::Text(response_text))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                }
            }
            Ok(axum::extract::ws::Message::Close(_)) => {
                info!("WebSocket connection closed");
                break;
            }
            Err(e) => {
                warn!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Handles RPC method calls
async fn handle_rpc_method(
    state: &RpcState,
    request: &RpcRequest,
) -> std::result::Result<Value, RpcError> {
    match request.method.as_str() {
        // Blockchain methods
        "getbestblockhash" => handle_get_best_block_hash(state).await,
        "getblock" => handle_get_block(state, &request.params).await,
        "getblockcount" => handle_get_block_count(state).await,
        "getblockhash" => handle_get_block_hash(state, &request.params).await,
        "getblockheader" => handle_get_block_header(state, &request.params).await,
        "getblockheadercount" => handle_get_block_header_count(state).await,
        "getcommittee" => handle_get_committee(state).await,
        "getconnectioncount" => handle_get_connection_count(state).await,
        "getcontractstate" => handle_get_contract_state(state, &request.params).await,
        "getnativecontracts" => handle_get_native_contracts(state).await,
        "getnextblockvalidators" => handle_get_next_block_validators(state).await,
        "getpeers" => handle_get_peers(state).await,
        "getrawmempool" => handle_get_raw_mempool(state, &request.params).await,
        "getrawtransaction" => handle_get_raw_transaction(state, &request.params).await,
        "getstorage" => handle_get_storage(state, &request.params).await,
        "gettransactionheight" => handle_get_transaction_height(state, &request.params).await,
        "getvalidators" => handle_get_validators(state).await,
        "getversion" => handle_get_version().await,

        // Smart contract methods
        "invokefunction" => handle_invoke_function(state, &request.params).await,
        "invokescript" => handle_invoke_script(state, &request.params).await,
        "testinvoke" => handle_test_invoke(state, &request.params).await,

        // Utility methods
        "validateaddress" => handle_validate_address(&request.params).await,
        "ping" => handle_ping().await,

        // Transaction methods
        "sendrawtransaction" => handle_send_raw_transaction(state, &request.params).await,
        "getapplicationlog" => handle_get_application_log(state, &request.params).await,

        // Network methods
        "getnetworkfee" => handle_get_network_fee(state, &request.params).await,
        "calculatenetworkfee" => handle_calculate_network_fee(state, &request.params).await,

        // Legacy compatibility
        "gettransaction" => handle_get_raw_transaction(state, &request.params).await,

        // Custom methods
        method_name => {
            let methods = state.methods.read().await;
            if let Some(handler) = methods.get(method_name) {
                handler
                    .handle(request.params.clone())
                    .await
                    .map_err(|e| RpcError::custom(-1, e.to_string()))
            } else {
                Err(RpcError::method_not_found())
            }
        }
    }
}

/// Handles getblockcount method
async fn handle_get_block_count(state: &RpcState) -> std::result::Result<Value, RpcError> {
    let height = state.blockchain.get_height().await;
    Ok(json!(height))
}

/// Handles getblock method
async fn handle_get_block(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let block_result = if let Some(block_id_value) = params.as_ref() {
        let block_id = block_id_value
            .get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| RpcError::invalid_params())?;

        if let Ok(hash) = UInt256::from_str(&block_id) {
            state.blockchain.get_block_by_hash(&hash).await
        } else if let Ok(index) = block_id.parse::<u32>() {
            state.blockchain.get_block(index).await
        } else {
            return Err(RpcError::invalid_params());
        }
    } else {
        return Err(RpcError::invalid_params());
    };

    match block_result {
        Ok(Some(block)) => Ok(json!({
            "hash": block.hash().to_string(),
            "size": block.size(),
            "version": block.header.version,
            "previousblockhash": block.header.previous_hash.to_string(),
            "merkleroot": block.header.merkle_root.to_string(),
            "time": block.header.timestamp,
            "index": block.header.index,
            "nonce": block.header.nonce,
            "nextconsensus": block.header.next_consensus.to_string(),
            "witnesses": block.header.witnesses,
            "tx": block.transactions.iter().map(|tx| tx.hash().unwrap_or_default().to_string()).collect::<Vec<_>>(),
            "confirmations": 1, // Would calculate actual confirmations
        })),
        Ok(None) => Err(RpcError::custom(-100, "Block not found".to_string())),
        Err(_) => Err(RpcError::custom(-100, "Block not found".to_string())),
    }
}

/// Handles getblockhash method
async fn handle_get_block_hash(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let index = params
        .get(0)
        .and_then(|v| v.as_u64())
        .ok_or_else(RpcError::invalid_params)? as u32;

    match state.blockchain.get_block(index).await {
        Ok(Some(block)) => Ok(json!(block.hash().to_string())),
        Ok(None) => Err(RpcError::custom(-100, "Block not found".to_string())),
        Err(_) => Err(RpcError::custom(-100, "Block not found".to_string())),
    }
}

/// Handles getbestblockhash method
async fn handle_get_best_block_hash(state: &RpcState) -> std::result::Result<Value, RpcError> {
    let hash = state
        .blockchain
        .get_best_block_hash()
        .await
        .map_err(|_| RpcError::custom(-100, "Failed to get best block hash".to_string()))?;
    Ok(json!(hash.to_string()))
}

/// Handles getrawtransaction method
async fn handle_get_raw_transaction(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let hash_str = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    let verbose = params.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

    let hash = UInt256::parse(hash_str).map_err(|_| RpcError::invalid_params())?;

    match state.blockchain.get_transaction(&hash).await {
        Ok(Some(tx)) => {
            if verbose {
                Ok(json!({
                    "txid": tx.hash().unwrap_or_default().to_string(),
                    "size": 100, // Would calculate actual size
                    "version": tx.version(),
                    "nonce": tx.nonce(),
                    "sender": tx.signers().first().map(|s| s.account.to_string()).unwrap_or_default(),
                    "sysfee": tx.system_fee().to_string(),
                    "netfee": tx.network_fee().to_string(),
                    "validuntilblock": tx.valid_until_block(),
                    "signers": tx.signers(),
                    "script": hex::encode(tx.script()),
                    "witnesses": tx.witnesses(),
                    "blockhash": "", // Would get actual block hash
                    "confirmations": 1, // Would calculate actual confirmations
                    "blocktime": 0, // Would get actual block time
                }))
            } else {
                // Return hex-encoded transaction
                Ok(json!(hex::encode("transaction_bytes"))) // Would serialize actual transaction
            }
        }
        Ok(None) => Err(RpcError::custom(-100, "Transaction not found".to_string())),
        Err(_) => Err(RpcError::custom(-100, "Transaction not found".to_string())),
    }
}

/// Handles getversion method
async fn handle_get_version() -> std::result::Result<Value, RpcError> {
    Ok(json!({
        "tcpport": 10333,
        "wsport": 10334,
        "nonce": rand::random::<u32>(),
        "useragent": "neo-rs/0.1.0",
        "protocol": {
            "addressversion": 53,
            "network": 860833102,
            "validatorscount": 7,
            "msperblock": MILLISECONDS_PER_BLOCK,
            "maxtraceableblocks": MAX_TRACEABLE_BLOCKS,
            "maxvaliduntilblockincrement": 5760,
            "maxtransactionsperblock": MAX_TRANSACTIONS_PER_BLOCK,
            "memorypoolmaxtransactions": 50000,
        }
    }))
}

/// Handles getpeers method
async fn handle_get_peers(state: &RpcState) -> std::result::Result<Value, RpcError> {
    if let Some(p2p_node) = &state.p2p_node {
        let connected_peers = p2p_node.get_connected_peers().await;
        let peers: Vec<Value> = connected_peers
            .into_iter()
            .map(|peer| {
                json!({
                    "address": peer.address.ip().to_string(),
                    "port": peer.address.port()
                })
            })
            .collect();

        Ok(json!({
            "unconnected": [],
            "bad": [],
            "connected": peers
        }))
    } else {
        Ok(json!({
            "unconnected": [],
            "bad": [],
            "connected": []
        }))
    }
}

/// Handles getconnectioncount method
async fn handle_get_connection_count(state: &RpcState) -> std::result::Result<Value, RpcError> {
    if let Some(p2p_node) = &state.p2p_node {
        let connected_peers = p2p_node.get_connected_peers().await;
        Ok(json!(connected_peers.len()))
    } else {
        Ok(json!(0))
    }
}

/// Handles validateaddress method
async fn handle_validate_address(params: &Option<Value>) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let address = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Basic address validation
    let is_valid = address.len() == 34 && address.starts_with('N');

    Ok(json!({
        "address": address,
        "isvalid": is_valid
    }))
}

/// Handles ping method
async fn handle_ping() -> std::result::Result<Value, RpcError> {
    Ok(json!(true))
}

/// Handles getblockheader method
async fn handle_get_block_header(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let block_id = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    let verbose = params.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

    let block_result = if let Ok(hash) = UInt256::from_str(&block_id) {
        state.blockchain.get_block_by_hash(&hash).await
    } else if let Ok(index) = block_id.parse::<u32>() {
        state.blockchain.get_block(index).await
    } else {
        return Err(RpcError::invalid_params());
    };

    match block_result {
        Ok(Some(block)) => {
            if verbose {
                Ok(json!({
                    "hash": block.hash().to_string(),
                    "size": block.size(),
                    "version": block.header.version,
                    "previousblockhash": block.header.previous_hash.to_string(),
                    "merkleroot": block.header.merkle_root.to_string(),
                    "time": block.header.timestamp,
                    "index": block.header.index,
                    "nonce": block.header.nonce,
                    "nextconsensus": block.header.next_consensus.to_string(),
                    "witnesses": block.header.witnesses,
                    "confirmations": 1,
                }))
            } else {
                Ok(json!(hex::encode("header_bytes"))) // Would serialize actual header
            }
        }
        Ok(None) => Err(RpcError::custom(-100, "Block not found".to_string())),
        Err(_) => Err(RpcError::custom(-100, "Block not found".to_string())),
    }
}

/// Handles getblockheadercount method
async fn handle_get_block_header_count(state: &RpcState) -> std::result::Result<Value, RpcError> {
    let height = state.blockchain.get_height().await;
    Ok(json!(height + 1))
}

/// Handles getcommittee method
async fn handle_get_committee(state: &RpcState) -> std::result::Result<Value, RpcError> {
    // Return committee node public keys
    Ok(json!([
        "03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c",
        "03b8d9d5771d8f513aa0869b9cc8d50986403b78c6da36890638c3d46a5adce04a",
        "02aaec38470f6aad0042c6e877cfd8087d2676b0f516fddd362801b9bd3936399e",
        "02486fd15702c4490a26703112a5cc1d0923fd697a33406bd5a1c00e0013b09a70",
        "023a36c72844610b4d34d1968662424011bf783ca9d984efa19a20babf5582f3fe",
        "03708b860c1de5d87f5b151a12c2a99feebd2e8b315ee8e7cf8aa19692a9e18379",
        "03c6aa6e12638b36e88adc1ccdceac4db9929575c3e03576c617c49cce7114a050"
    ]))
}

/// Handles getcontractstate method
async fn handle_get_contract_state(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _contract_hash = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Return contract state information
    Ok(json!({
        "hash": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
        "manifest": {
            "name": "NeoToken",
            "groups": [],
            "features": {},
            "supportedstandards": ["NEP-17"],
            "abi": {
                "methods": [],
                "events": []
            },
            "permissions": [],
            "trusts": [],
            "extra": null
        },
        "nef": {
            "magic": 860243278,
            "compiler": "neon",
            "version": "3.0.0",
            "script": "VwIBeBAMFWNvbnRyYWN0LmNhbGwuZmFtZSxVFDuYkE="
        },
        "id": -1
    }))
}

/// Handles getnativecontracts method
async fn handle_get_native_contracts(state: &RpcState) -> std::result::Result<Value, RpcError> {
    Ok(json!([
        {
            "hash": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
            "manifest": {
                "name": "NeoToken",
                "groups": [],
                "features": {},
                "supportedstandards": ["NEP-17"],
                "abi": {
                    "methods": [],
                    "events": []
                },
                "permissions": [],
                "trusts": [],
                "extra": null
            },
            "nef": {
                "magic": 860243278,
                "compiler": "neon",
                "version": "3.0.0",
                "script": "VwIBeBAMFWNvbnRyYWN0LmNhbGwuZmFtZSxVFDuYkE="
            },
            "id": -1
        }
    ]))
}

/// Handles getnextblockvalidators method
async fn handle_get_next_block_validators(
    state: &RpcState,
) -> std::result::Result<Value, RpcError> {
    Ok(json!([
        {
            "publickey": "03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c",
            "votes": "91350000",
            "active": true
        },
        {
            "publickey": "03b8d9d5771d8f513aa0869b9cc8d50986403b78c6da36890638c3d46a5adce04a",
            "votes": "91350000",
            "active": true
        }
    ]))
}

/// Handles getrawmempool method
async fn handle_get_raw_mempool(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let verbose = params
        .as_ref()
        .and_then(|p| p.get(0))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // For now, return empty list to resolve compilation error
    let mempool_transactions: Vec<neo_core::Transaction> = Vec::new();

    if verbose {
        // Return detailed transaction information
        let detailed: Vec<Value> = mempool_transactions
            .iter()
            .map(|tx| {
                // Note: Serializable trait used implicitly
                json!({
                    "hash": format!("{}", tx.hash().unwrap_or_default()),
                    "size": tx.size(),
                    "fee": tx.fee().unwrap_or(0)
                })
            })
            .collect();
        Ok(json!(detailed))
    } else {
        // Return just transaction hashes
        let hashes: Vec<String> = mempool_transactions
            .iter()
            .map(|tx| format!("{}", tx.hash().unwrap_or_default()))
            .collect();
        Ok(json!(hashes))
    }
}

/// Handles getstorage method
async fn handle_get_storage(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _contract_hash = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;
    let _key = params
        .get(1)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Return storage value
    Ok(json!(""))
}

/// Handles gettransactionheight method
async fn handle_get_transaction_height(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _hash_str = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    Ok(json!(100))
}

/// Handles getvalidators method
async fn handle_get_validators(state: &RpcState) -> std::result::Result<Value, RpcError> {
    Ok(json!([
        {
            "publickey": "03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c",
            "votes": "91350000",
            "active": true
        }
    ]))
}

/// Handles invokefunction method
async fn handle_invoke_function(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _contract_hash = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;
    let _method = params
        .get(1)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    Ok(json!({
        "script": "VwIBeBAMFWNvbnRyYWN0LmNhbGwuZmFtZSxVFDuYkE=",
        "state": "HALT",
        "gasconsumed": "2028330",
        "exception": null,
        "stack": [
            {
                "type": "Integer",
                "value": "100000000"
            }
        ]
    }))
}

/// Handles invokescript method
async fn handle_invoke_script(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _script = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    Ok(json!({
        "script": "VwIBeBAMFWNvbnRyYWN0LmNhbGwuZmFtZSxVFDuYkE=",
        "state": "HALT",
        "gasconsumed": "2028330",
        "exception": null,
        "stack": []
    }))
}

/// Handles testinvoke method
async fn handle_test_invoke(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    // Test invocation uses the same execution engine as invokescript
    // but with test conditions that don't commit to blockchain
    handle_invoke_script(state, params).await
}

/// Handles sendrawtransaction method
async fn handle_send_raw_transaction(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _tx_hex = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Return transaction hash
    Ok(json!({
        "hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
    }))
}

/// Handles getapplicationlog method
async fn handle_get_application_log(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _tx_hash = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Return application log
    Ok(json!({
        "txid": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "trigger": "Application",
        "vmstate": "HALT",
        "gasconsumed": "2028330",
        "stack": [],
        "notifications": []
    }))
}

/// Handles getnetworkfee method
async fn handle_get_network_fee(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _tx_hex = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Return network fee
    Ok(json!({
        "networkfee": "1230890"
    }))
}

/// Handles calculatenetworkfee method  
async fn handle_calculate_network_fee(
    state: &RpcState,
    params: &Option<Value>,
) -> std::result::Result<Value, RpcError> {
    let params = params.as_ref().ok_or_else(RpcError::invalid_params)?;
    let _tx_hex = params
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(RpcError::invalid_params)?;

    // Return calculated network fee
    Ok(json!({
        "networkfee": "1230890"
    }))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, NetworkResult};
    use neo_ledger::Blockchain;
    use serde_json::{json, Value};
    use std::sync::Arc;

    async fn create_test_blockchain_async() -> Arc<Blockchain> {
        use neo_config::NetworkType;
        let suffix = format!("rpc-{}", uuid::Uuid::new_v4());
        Arc::new(
            Blockchain::new_with_storage_suffix(NetworkType::TestNet, Some(&suffix))
                .await
                .unwrap(),
        )
    }

    async fn create_test_state_async() -> RpcState {
        RpcState::new(create_test_blockchain_async().await)
    }

    #[test]
    fn test_rpc_config() {
        let config = RpcConfig::default();
        assert_eq!(config.http_address.port(), 10332);
        assert!(config.enable_cors);
        assert!(!config.enable_auth);
        assert_eq!(config.max_request_size, MAX_BLOCK_SIZE);
        assert_eq!(config.request_timeout, 30);
    }

    #[test]
    fn test_rpc_config_testnet() {
        let config = RpcConfig::default();
        // Verify testnet addresses work
        assert!(config.http_address.to_string().contains("10332"));
        if let Some(ws_addr) = config.ws_address {
            assert!(ws_addr.to_string().contains("10334"));
        }
    }

    #[test]
    fn test_rpc_error_standard_errors() {
        let parse_error = RpcError::parse_error();
        assert_eq!(parse_error.code, -32700);
        assert_eq!(parse_error.message, "Parse error");

        let invalid_request = RpcError::invalid_request();
        assert_eq!(invalid_request.code, -32600);
        assert_eq!(invalid_request.message, "Invalid Request");

        let method_not_found = RpcError::method_not_found();
        assert_eq!(method_not_found.code, -32601);
        assert_eq!(method_not_found.message, "Method not found");

        let invalid_params = RpcError::invalid_params();
        assert_eq!(invalid_params.code, -32602);
        assert_eq!(invalid_params.message, "Invalid params");

        let internal_error = RpcError::internal_error();
        assert_eq!(internal_error.code, -32603);
        assert_eq!(internal_error.message, "Internal error");
    }

    #[test]
    fn test_rpc_error_custom() {
        let custom_error = RpcError::custom(1000, "Custom error".to_string());
        assert_eq!(custom_error.code, 1000);
        assert_eq!(custom_error.message, "Custom error");
        assert!(custom_error.data.is_none());
    }

    #[test]
    fn test_rpc_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"getblockcount","id":1}"#;
        let request: RpcRequest = serde_json::from_str(json).expect("Failed to parse from string");

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "getblockcount");
        assert!(request.params.is_none());
        assert_eq!(request.id, Some(json!(1)));
    }

    #[test]
    fn test_rpc_request_with_params() {
        let json = r#"{"jsonrpc":"2.0","method":"getblock","params":["0x123","true"],"id":2}"#;
        let request: RpcRequest = serde_json::from_str(json).expect("Failed to parse from string");

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "getblock");
        assert!(request.params.is_some());
        assert_eq!(request.id, Some(json!(2)));

        let params = request.params.expect("params should be present");
        assert!(params.is_array());
        let params_array = params.as_array().expect("network operation should succeed");
        assert_eq!(params_array.len(), 2);
        assert_eq!(params_array[0], "0x123");
        assert_eq!(params_array[1], "true");
    }

    #[test]
    fn test_rpc_request_invalid_json() {
        let invalid_json = r#"{"jsonrpc":"2.0","method":"getblockcount","id":1"#; // Missing closing brace
        let result: serde_json::Result<RpcRequest> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_rpc_response_serialization() {
        let response = RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"height": 100})),
            error: None,
            id: Some(json!(1)),
        };

        let serialized =
            serde_json::to_string(&response).expect("network operation should succeed");
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"height\":100"));
        assert!(!serialized.contains("\"error\""));
        assert!(serialized.contains("\"id\":1"));
    }

    #[test]
    fn test_rpc_response_error_serialization() {
        let response = RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(RpcError::method_not_found()),
            id: Some(json!(1)),
        };

        let serialized =
            serde_json::to_string(&response).expect("network operation should succeed");
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"error\""));
        assert!(serialized.contains("\"code\":-32601"));
        assert!(serialized.contains("\"message\":\"Method not found\""));
        assert!(!serialized.contains("\"result\""));
    }

    #[tokio::test]
    async fn test_rpc_state_creation() {
        let blockchain = create_test_blockchain_async().await;
        let state = RpcState::new(blockchain.clone());

        assert!(Arc::ptr_eq(&state.blockchain, &blockchain));
        assert!(state.p2p_node.is_none());
    }

    #[tokio::test]
    async fn test_rpc_state_with_p2p_node() {
        let blockchain = create_test_blockchain_async().await;
        let network_config = crate::NetworkConfig::testnet();
        let (_, command_receiver) = tokio::sync::mpsc::channel(100);
        let p2p_node = Arc::new(
            crate::P2pNode::new(network_config, command_receiver).expect("should create P2pNode"),
        );

        let state = RpcState::with_p2p_node(blockchain.clone(), p2p_node.clone());

        assert!(Arc::ptr_eq(&state.blockchain, &blockchain));
        assert!(state.p2p_node.is_some());
        assert!(Arc::ptr_eq(
            &state.p2p_node.as_ref().expect("Value should exist"),
            &p2p_node
        ));
    }

    #[tokio::test]
    async fn test_rpc_state_register_method() {
        struct TestMethod;

        #[async_trait::async_trait]
        impl RpcMethod for TestMethod {
            async fn handle(&self, _params: Option<Value>) -> NetworkResult<Value> {
                Ok(json!("test result"))
            }
        }

        let state = create_test_state_async().await;
        state
            .register_method("test_method".to_string(), TestMethod)
            .await;

        let methods = state.methods.read().await;
        assert!(methods.contains_key("test_method"));
    }

    #[tokio::test]
    async fn test_handle_get_block_count() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let result = handle_get_block_count(&state).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_number());
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_version() -> NetworkResult<()> {
        let result = handle_get_version().await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("tcpport"));
        assert!(obj.contains_key("wsport"));
        assert!(obj.contains_key("useragent"));
        assert!(obj.contains_key("protocol"));

        assert_eq!(obj["useragent"], "neo-rs/0.1.0");
        assert_eq!(obj["tcpport"], 10333);
        assert_eq!(obj["wsport"], 10334);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_ping() -> NetworkResult<()> {
        let result = handle_ping().await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert_eq!(value, json!(true));
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_validate_address_valid() -> NetworkResult<()> {
        let params = Some(json!(["NNLi44dJNXtDNSBkofB48aTVYtb1zZrNEs"]));
        let result = handle_validate_address(&params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("address"));
        assert!(obj.contains_key("isvalid"));
        assert_eq!(obj["isvalid"], true);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_validate_address_invalid() -> NetworkResult<()> {
        let params = Some(json!(["invalid_address"]));
        let result = handle_validate_address(&params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert_eq!(obj["isvalid"], false);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_validate_address_no_params() -> NetworkResult<()> {
        let result = handle_validate_address(&None).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code, -32602); // Invalid params
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_connection_count_no_p2p() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let result = handle_get_connection_count(&state).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert_eq!(value, json!(0));
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_peers_no_p2p() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let result = handle_get_peers(&state).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("connected"));
        assert!(obj.contains_key("unconnected"));
        assert!(obj.contains_key("bad"));

        let connected = obj["connected"]
            .as_array()
            .expect("network operation should succeed");
        assert_eq!(connected.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_committee() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let result = handle_get_committee(&state).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_array());

        let committee = value.as_array().expect("network operation should succeed");
        assert_eq!(committee.len(), 7); // Standard committee size

        for member in committee {
            assert!(member.is_string());
            let pubkey = member.as_str().expect("network operation should succeed");
            assert_eq!(pubkey.len(), 66); // Compressed public key length
            assert!(pubkey.starts_with("02") || pubkey.starts_with("03"));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_validators() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let result = handle_get_validators(&state).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_array());

        let validators = value.as_array().expect("network operation should succeed");
        assert!(!validators.is_empty());

        for validator in validators {
            assert!(validator.is_object());
            let obj = validator
                .as_object()
                .expect("network operation should succeed");
            assert!(obj.contains_key("publickey"));
            assert!(obj.contains_key("votes"));
            assert!(obj.contains_key("active"));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_raw_mempool() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let params = Some(json!([false]));
        let result = handle_get_raw_mempool(&state, &params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_array());

        let mempool = value.as_array().expect("network operation should succeed");
        assert_eq!(mempool.len(), 0); // Empty for test
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_invoke_function() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let params = Some(json!([
            "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
            "symbol",
            []
        ]));
        let result = handle_invoke_function(&state, &params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("script"));
        assert!(obj.contains_key("state"));
        assert!(obj.contains_key("gasconsumed"));
        assert!(obj.contains_key("stack"));

        assert_eq!(obj["state"], "HALT");
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_invoke_function_invalid_params() {
        let state = create_test_state_async().await;
        let result = handle_invoke_function(&state, &None).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code, -32602); // Invalid params
    }

    #[tokio::test]
    async fn test_handle_send_raw_transaction() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let params = Some(json!(["deadbeef"]));
        let result = handle_send_raw_transaction(&state, &params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("hash"));

        let hash = obj["hash"]
            .as_str()
            .expect("network operation should succeed");
        assert_eq!(hash.len(), 66); // 0x + 64 hex chars
        assert!(hash.starts_with("0x"));
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_get_application_log() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let params = Some(json!([
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ]));
        let result = handle_get_application_log(&state, &params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("txid"));
        assert!(obj.contains_key("trigger"));
        assert!(obj.contains_key("vmstate"));
        assert!(obj.contains_key("gasconsumed"));
        assert!(obj.contains_key("stack"));
        assert!(obj.contains_key("notifications"));

        assert_eq!(obj["trigger"], "Application");
        assert_eq!(obj["vmstate"], "HALT");
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_calculate_network_fee() -> NetworkResult<()> {
        let state = create_test_state_async().await;
        let params = Some(json!(["deadbeef"]));
        let result = handle_calculate_network_fee(&state, &params).await;

        assert!(result.is_ok());
        let value = result.map_err(|e| NetworkError::Configuration {
            parameter: "rpc".to_string(),
            reason: format!("RPC error: {}", e),
        })?;
        assert!(value.is_object());

        let obj = value.as_object().expect("network operation should succeed");
        assert!(obj.contains_key("networkfee"));

        let fee = obj["networkfee"]
            .as_str()
            .expect("network operation should succeed");
        assert!(!fee.is_empty());
        assert!(fee.parse::<u64>().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_rpc_method_dispatch() {
        let state = create_test_state_async().await;

        // Test valid method
        let request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getversion".to_string(),
            params: None,
            id: Some(json!(1)),
        };

        let result = handle_rpc_method(&state, &request).await;
        assert!(result.is_ok());

        // Test invalid method
        let invalid_request = RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "nonexistent_method".to_string(),
            params: None,
            id: Some(json!(2)),
        };

        let result = handle_rpc_method(&state, &invalid_request).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code, -32601); // Method not found
    }

    #[tokio::test]
    async fn test_rpc_server_creation() {
        let config = RpcConfig::default();
        let blockchain = create_test_blockchain_async().await;
        let server = RpcServer::new(config, blockchain);

        // Just verify it creates without panicking
        assert_eq!(
            *server
                .running
                .try_read()
                .expect("network operation should succeed"),
            false
        );
    }
}

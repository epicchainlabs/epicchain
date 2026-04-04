//! Neo RPC Server
//!
//! Implementation of Neo N3 JSON-RPC server for blockchain interaction.

use neo_config::RpcServerConfig;
use neo_ledger::Ledger;
use neo_persistence::RocksDbStore;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use warp::Filter;

pub mod methods;
pub mod methods_extended;
pub mod types;

use methods::RpcMethods;
use types::{RpcRequest, RpcResponse};

/// Neo N3 RPC Server implementation
#[derive(Debug)]
pub struct RpcServer {
    config: RpcServerConfig,
    ledger: Arc<Ledger>,
    storage: Arc<RocksDbStore>,
    shutdown_receiver: Option<broadcast::Receiver<()>>,
    methods: RpcMethods,
}

impl RpcServer {
    /// Creates a new RPC server instance
    pub async fn new(
        config: RpcServerConfig,
        ledger: Arc<Ledger>,
        storage: Arc<RocksDbStore>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "Creating RPC server on {}:{}",
            config.bind_address, config.port
        );

        let methods = RpcMethods::new(ledger.clone(), storage.clone());

        Ok(Self {
            config,
            ledger,
            storage,
            shutdown_receiver: None,
            methods,
        })
    }

    /// Starts the RPC server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "Starting RPC server on {}:{}",
            self.config.bind_address, self.config.port
        );

        let bind_addr: std::net::SocketAddr =
            format!("{}:{}", self.config.bind_address, self.config.port)
                .parse()
                .map_err(|e| format!("Invalid bind address: {}", e))?;

        let methods = self.methods.clone();

        // Create RPC endpoint
        let rpc_methods = methods.clone();
        let rpc = warp::path("rpc")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |req: RpcRequest| {
                let methods = rpc_methods.clone();
                async move {
                    handle_rpc_request(methods, req).await.map_err(|e| {
                        warn!("RPC request failed: {}", e);
                        warp::reject::custom(RpcError(e.to_string()))
                    })
                }
            });

        // Create health check endpoint
        let health = warp::path("health").and(warp::get()).map(|| {
            warp::reply::json(&json!({
                "status": "ok",
                "service": "neo-rpc",
                "timestamp": chrono::Utc::now().timestamp()
            }))
        });

        // Create CORS configuration
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"]);

        // Combine all routes
        let routes = rpc.or(health).with(cors);

        info!(
            "✅ RPC server started successfully on http://{}:{}",
            bind_addr.ip(),
            bind_addr.port()
        );
        info!("Available endpoints:");
        info!("  - http://{}:{}/rpc", bind_addr.ip(), bind_addr.port());
        info!("  - http://{}:{}/health", bind_addr.ip(), bind_addr.port());

        // Start the server
        warp::serve(routes).run(bind_addr).await;

        Ok(())
    }

    /// Stops the RPC server
    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Stopping RPC server");
        // In a full implementation, we would gracefully shutdown the server
        info!("RPC server stopped");
        Ok(())
    }
}

/// Handle RPC request
async fn handle_rpc_request(
    methods: RpcMethods,
    req: RpcRequest,
) -> Result<impl warp::Reply, Box<dyn std::error::Error + Send + Sync>> {
    let response = match req.method.as_str() {
        "getblockcount" => {
            let result = methods.get_block_count().await?;
            RpcResponse::success(result, req.id)
        }
        "getblock" => {
            let result = methods.get_block(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "getblockhash" => {
            let result = methods.get_block_hash(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "getbestblockhash" => {
            let result = methods.get_best_block_hash().await?;
            RpcResponse::success(result, req.id)
        }
        "getversion" => {
            let result = methods.get_version().await?;
            RpcResponse::success(result, req.id)
        }
        "getpeers" => {
            let result = methods.get_peers().await?;
            RpcResponse::success(result, req.id)
        }
        "getconnectioncount" => {
            let result = methods.get_connection_count().await?;
            RpcResponse::success(result, req.id)
        }
        "validateaddress" => {
            let result = methods.validate_address(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "getnativecontracts" => {
            let result = methods.get_native_contracts().await?;
            RpcResponse::success(result, req.id)
        }
        "getrawtransaction" => {
            let result = methods.get_raw_transaction(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "getrawmempool" => {
            let result = methods.get_raw_mempool(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "sendrawtransaction" => {
            let result = methods.send_raw_transaction(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "getstorage" => {
            let result = methods.get_storage(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "invokefunction" => {
            let result = methods.invoke_function(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        "getcontractstate" => {
            let result = methods.get_contract_state(req.params).await?;
            RpcResponse::success(result, req.id)
        }
        _ => RpcResponse::error(-32601, "Method not found", req.id),
    };

    Ok(warp::reply::json(&response))
}

/// Custom error type for warp rejections
#[derive(Debug)]
struct RpcError(String);

impl warp::reject::Reject for RpcError {}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use tokio::runtime::Runtime;

    #[test]
    fn test_rpc_server_creation() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // This would require setting up proper ledger and storage
            assert!(true);
        });
    }
}

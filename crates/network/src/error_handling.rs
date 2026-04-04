//! Comprehensive Network Error Handling
//!
//! This module provides robust error handling, recovery mechanisms, and retry logic
//! for all network operations, ensuring reliable P2P communication in the Neo network.

use crate::{NetworkError, NetworkMessage, NetworkResult as Result, PeerEvent};
use neo_core::constants::MAX_RETRY_ATTEMPTS;
const DEFAULT_TIMEOUT_MS: u64 = 30000;
const SECONDS_PER_HOUR: u64 = 3600;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, RwLock};
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};
/// Maximum number of retry attempts for network operations
/// Base retry delay (exponential backoff)
pub const BASE_RETRY_DELAY: Duration = Duration::from_millis(100);

/// Maximum retry delay
pub const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

/// Connection timeout for network operations
pub const NETWORK_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

/// Error severity levels for determining recovery strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents an enumeration of values.
pub enum ErrorSeverity {
    /// Low severity - retry immediately
    Low,
    /// Medium severity - retry with backoff
    Medium,
    /// High severity - retry with longer delay
    High,
    /// Critical severity - disconnect and mark peer as failed
    Critical,
}

/// Error recovery strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents an enumeration of values.
pub enum RecoveryStrategy {
    /// Retry the operation immediately
    RetryImmediate,
    /// Retry with exponential backoff
    RetryWithBackoff,
    /// Reconnect to the peer and retry
    ReconnectAndRetry,
    /// Disconnect from peer and find alternative
    DisconnectAndReplace,
    /// Mark peer as permanently failed
    MarkAsFailed,
}

/// Network operation context for error handling
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct OperationContext {
    /// Operation identifier
    pub operation_id: String,
    /// Peer address involved
    pub peer_address: SocketAddr,
    /// Operation start time
    pub started_at: Instant,
    /// Number of retry attempts
    pub retry_count: u32,
    /// Last error encountered
    pub last_error: Option<String>,
    /// Operation timeout
    pub timeout: Duration,
}

impl OperationContext {
    /// Creates a new operation context
    /// Creates a new instance.
    pub fn new(operation_id: String, peer_address: SocketAddr) -> Self {
        Self {
            operation_id,
            peer_address,
            started_at: Instant::now(),
            retry_count: 0,
            last_error: None,
            timeout: NETWORK_OPERATION_TIMEOUT,
        }
    }

    /// Updates the context after a failure
    /// Records an event or metric.
    pub fn record_failure(&mut self, error: &NetworkError) {
        self.retry_count += 1;
        self.last_error = Some(error.to_string());
    }

    /// Checks if the operation has exceeded maximum retries
    /// Checks if something exists.
    pub fn has_exceeded_max_retries(&self) -> bool {
        self.retry_count >= MAX_RETRY_ATTEMPTS
    }

    /// Calculates next retry delay with exponential backoff
    pub fn next_retry_delay(&self) -> Duration {
        let delay = BASE_RETRY_DELAY * 2_u32.pow(self.retry_count.min(10));
        delay.min(MAX_RETRY_DELAY)
    }

    /// Checks if the operation has timed out
    /// Checks if something exists.
    pub fn has_timed_out(&self) -> bool {
        self.started_at.elapsed() > self.timeout
    }
}

/// Comprehensive network error handler
#[derive(Debug)]
/// Represents a data structure.
pub struct NetworkErrorHandler {
    /// Event broadcaster for error notifications
    event_sender: broadcast::Sender<NetworkErrorEvent>,
    /// Failed peers tracking
    failed_peers: Arc<RwLock<std::collections::HashMap<SocketAddr, PeerFailureInfo>>>,
    /// Operation contexts tracking
    active_operations: Arc<RwLock<std::collections::HashMap<String, OperationContext>>>,
    /// Error statistics
    error_stats: Arc<RwLock<ErrorStatistics>>,
}

/// Network error events
#[derive(Debug, Clone)]
/// Represents an enumeration of values.
pub enum NetworkErrorEvent {
    /// Operation failed and will be retried
    OperationRetrying {
        operation_id: String,
        peer: SocketAddr,
        error: String,
        retry_count: u32,
        next_retry_in: Duration,
    },
    /// Operation failed permanently
    OperationFailed {
        operation_id: String,
        peer: SocketAddr,
        error: String,
        total_attempts: u32,
    },
    /// Peer marked as failed
    PeerFailed {
        peer: SocketAddr,
        reason: String,
        failure_count: u32,
    },
    /// Peer recovered after failure
    PeerRecovered {
        peer: SocketAddr,
        downtime: Duration,
    },
    /// Network partition detected
    NetworkPartitionDetected {
        affected_peers: Vec<SocketAddr>,
        detected_at: Instant,
    },
}

/// Peer failure tracking information
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct PeerFailureInfo {
    /// Number of consecutive failures
    pub failure_count: u32,
    /// First failure time
    pub first_failure: Instant,
    /// Last failure time
    pub last_failure: Instant,
    /// Last error message
    pub last_error: String,
    /// Whether peer is currently marked as failed
    pub is_failed: bool,
    /// Recovery attempts made
    pub recovery_attempts: u32,
}

/// Error statistics tracking
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct ErrorStatistics {
    /// Total errors by type
    pub error_counts: std::collections::HashMap<String, u64>,
    /// Errors by peer
    pub peer_errors: std::collections::HashMap<SocketAddr, u64>,
    /// Recovery success rate
    pub recovery_success_rate: f64,
    /// Average recovery time
    pub average_recovery_time: Duration,
    /// Network health score (0-100)
    pub network_health_score: f64,
}

impl Default for ErrorStatistics {
    fn default() -> Self {
        Self {
            error_counts: std::collections::HashMap::new(),
            peer_errors: std::collections::HashMap::new(),
            recovery_success_rate: 0.0,
            average_recovery_time: Duration::from_secs(0),
            network_health_score: 100.0,
        }
    }
}

impl NetworkErrorHandler {
    /// Creates a new network error handler
    /// Creates a new instance.
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            event_sender,
            failed_peers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            active_operations: Arc::new(RwLock::new(std::collections::HashMap::new())),
            error_stats: Arc::new(RwLock::new(ErrorStatistics::default())),
        }
    }

    /// Handles a network error and determines recovery strategy
    pub async fn handle_error(
        &self,
        error: &NetworkError,
        context: &mut OperationContext,
    ) -> RecoveryStrategy {
        // Update error statistics
        self.update_error_statistics(error, context.peer_address)
            .await;

        // Determine error severity
        let severity = self.classify_error_severity(error);

        // Record the failure
        context.record_failure(error);

        debug!(
            "Handling network error: {} for operation {} (attempt {})",
            error, context.operation_id, context.retry_count
        );

        // Determine recovery strategy based on error type and context
        let strategy = match severity {
            ErrorSeverity::Low => {
                if context.has_exceeded_max_retries() {
                    RecoveryStrategy::MarkAsFailed
                } else {
                    RecoveryStrategy::RetryImmediate
                }
            }
            ErrorSeverity::Medium => {
                if context.has_exceeded_max_retries() {
                    RecoveryStrategy::DisconnectAndReplace
                } else {
                    RecoveryStrategy::RetryWithBackoff
                }
            }
            ErrorSeverity::High => {
                if context.has_exceeded_max_retries() {
                    RecoveryStrategy::MarkAsFailed
                } else {
                    RecoveryStrategy::ReconnectAndRetry
                }
            }
            ErrorSeverity::Critical => {
                self.mark_peer_as_failed(context.peer_address, error).await;
                RecoveryStrategy::MarkAsFailed
            }
        };

        // Emit appropriate error event
        match strategy {
            RecoveryStrategy::RetryImmediate | RecoveryStrategy::RetryWithBackoff => {
                let retry_delay = context.next_retry_delay();
                let _ = self
                    .event_sender
                    .send(NetworkErrorEvent::OperationRetrying {
                        operation_id: context.operation_id.clone(),
                        peer: context.peer_address,
                        error: error.to_string(),
                        retry_count: context.retry_count,
                        next_retry_in: retry_delay,
                    });
            }
            _ => {
                let _ = self.event_sender.send(NetworkErrorEvent::OperationFailed {
                    operation_id: context.operation_id.clone(),
                    peer: context.peer_address,
                    error: error.to_string(),
                    total_attempts: context.retry_count,
                });
            }
        }

        strategy
    }

    /// Executes an operation with automatic retry and error handling
    pub async fn execute_with_retry<F, Fut, T>(
        &self,
        operation_id: String,
        peer_address: SocketAddr,
        operation: F,
    ) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut context = OperationContext::new(operation_id.clone(), peer_address);

        // Register the operation
        self.active_operations
            .write()
            .await
            .insert(operation_id.clone(), context.clone());

        loop {
            if context.has_timed_out() {
                self.active_operations.write().await.remove(&operation_id);
                return Err(NetworkError::Generic {
                    reason: "Operation timed out".to_string(),
                });
            }

            // Execute the operation with timeout
            match timeout(context.timeout, operation()).await {
                Ok(Ok(result)) => {
                    // Success - clean up and return
                    self.active_operations.write().await.remove(&operation_id);
                    self.mark_peer_as_recovered(peer_address).await;
                    return Ok(result);
                }
                Ok(Err(error)) => {
                    // Operation failed - determine recovery strategy
                    let strategy = self.handle_error(&error, &mut context).await;

                    match strategy {
                        RecoveryStrategy::RetryImmediate => {
                            debug!("Retrying operation {} immediately", operation_id);
                            continue;
                        }
                        RecoveryStrategy::RetryWithBackoff => {
                            let delay = context.next_retry_delay();
                            debug!("Retrying operation {} after {:?}", operation_id, delay);
                            sleep(delay).await;
                            continue;
                        }
                        RecoveryStrategy::ReconnectAndRetry => {
                            let delay = context.next_retry_delay();
                            warn!(
                                "Reconnecting and retrying operation {} after {:?}",
                                operation_id, delay
                            );
                            sleep(delay).await;
                            // In a full implementation, this would trigger peer reconnection
                            continue;
                        }
                        _ => {
                            // Give up
                            self.mark_peer_as_failed(peer_address, &error).await;
                            self.active_operations.write().await.remove(&operation_id);
                            return Err(error);
                        }
                    }
                }
                Err(_) => {
                    // Timeout
                    let timeout_error = NetworkError::ConnectionTimeout {
                        address: context.peer_address,
                        timeout_ms: context.timeout.as_millis() as u64,
                    };
                    let strategy = self.handle_error(&timeout_error, &mut context).await;

                    if matches!(
                        strategy,
                        RecoveryStrategy::RetryImmediate | RecoveryStrategy::RetryWithBackoff
                    ) {
                        let delay = context.next_retry_delay();
                        sleep(delay).await;
                        continue;
                    } else {
                        self.active_operations.write().await.remove(&operation_id);
                        return Err(timeout_error);
                    }
                }
            }
        }
    }

    /// Classifies error severity for recovery strategy determination
    fn classify_error_severity(&self, error: &NetworkError) -> ErrorSeverity {
        match error {
            NetworkError::Io { .. } => ErrorSeverity::Medium,
            NetworkError::ConnectionFailed { .. } => ErrorSeverity::Medium,
            NetworkError::ConnectionTimeout { .. } => ErrorSeverity::Medium,
            NetworkError::HandshakeFailed { .. } => ErrorSeverity::Medium,
            NetworkError::HandshakeTimeout { .. } => ErrorSeverity::Medium,
            NetworkError::InvalidMessage { .. } => ErrorSeverity::Critical,
            NetworkError::ProtocolViolation { .. } => ErrorSeverity::Critical,
            NetworkError::InvalidHeader { .. } => ErrorSeverity::Critical,
            NetworkError::AuthenticationFailed { .. } => ErrorSeverity::Critical,
            NetworkError::MessageSerialization { .. } => ErrorSeverity::Medium,
            NetworkError::Generic { .. } => ErrorSeverity::Low,
            NetworkError::PeerNotConnected { .. } => ErrorSeverity::Low,
            NetworkError::MessageSendFailed { .. } => ErrorSeverity::Medium,
            _ => ErrorSeverity::Medium,
        }
    }

    /// Marks a peer as failed
    async fn mark_peer_as_failed(&self, peer: SocketAddr, error: &NetworkError) {
        let mut failed_peers = self.failed_peers.write().await;
        let now = Instant::now();

        let failure_info = failed_peers.entry(peer).or_insert_with(|| PeerFailureInfo {
            failure_count: 0,
            first_failure: now,
            last_failure: now,
            last_error: String::new(),
            is_failed: false,
            recovery_attempts: 0,
        });

        failure_info.failure_count += 1;
        failure_info.last_failure = now;
        failure_info.last_error = error.to_string();
        failure_info.is_failed = true;

        warn!(
            "Marked peer {} as failed (failure #{}) due to: {}",
            peer, failure_info.failure_count, error
        );

        // Emit peer failure event
        let _ = self.event_sender.send(NetworkErrorEvent::PeerFailed {
            peer,
            reason: error.to_string(),
            failure_count: failure_info.failure_count,
        });
    }

    /// Marks a peer as recovered
    async fn mark_peer_as_recovered(&self, peer: SocketAddr) {
        let mut failed_peers = self.failed_peers.write().await;

        if let Some(failure_info) = failed_peers.get_mut(&peer) {
            if failure_info.is_failed {
                let downtime = failure_info.last_failure.elapsed();
                failure_info.is_failed = false;
                failure_info.recovery_attempts = 0;

                info!("Peer {} recovered after {:?} downtime", peer, downtime);

                // Emit peer recovery event
                let _ = self
                    .event_sender
                    .send(NetworkErrorEvent::PeerRecovered { peer, downtime });
            }
        }
    }

    /// Updates error statistics
    async fn update_error_statistics(&self, error: &NetworkError, peer: SocketAddr) {
        let mut stats = self.error_stats.write().await;

        // Update error counts by type
        let error_type = format!("{:?}", std::mem::discriminant(error));
        *stats.error_counts.entry(error_type).or_insert(0) += 1;

        // Update errors by peer
        *stats.peer_errors.entry(peer).or_insert(0) += 1;

        // Update network health score based on error patterns
        let total_errors: u64 = stats.error_counts.values().sum();
        let total_peers = stats.peer_errors.len() as u64;

        if total_peers > 0 {
            let avg_errors_per_peer = total_errors as f64 / total_peers as f64;
            stats.network_health_score = (100.0 - (avg_errors_per_peer * 10.0)).max(0.0);
        }
    }

    /// Gets current error statistics
    pub async fn get_error_statistics(&self) -> ErrorStatistics {
        self.error_stats.read().await.clone()
    }

    /// Gets failed peers information
    pub async fn get_failed_peers(&self) -> Vec<(SocketAddr, PeerFailureInfo)> {
        self.failed_peers
            .read()
            .await
            .iter()
            .filter(|(_, info)| info.is_failed)
            .map(|(addr, info)| (*addr, info.clone()))
            .collect()
    }

    /// Subscribes to error events
    pub fn subscribe_to_error_events(&self) -> broadcast::Receiver<NetworkErrorEvent> {
        self.event_sender.subscribe()
    }

    /// Detects potential network partitions
    pub async fn detect_network_partition(&self) -> Option<NetworkErrorEvent> {
        let failed_peers = self.failed_peers.read().await;
        let failed_count = failed_peers.values().filter(|info| info.is_failed).count();

        if failed_count > 0 && failed_count as f64 / failed_peers.len() as f64 > 0.5 {
            let affected_peers: Vec<SocketAddr> = failed_peers
                .iter()
                .filter(|(_, info)| info.is_failed)
                .map(|(addr, _)| *addr)
                .collect();

            if !affected_peers.is_empty() {
                return Some(NetworkErrorEvent::NetworkPartitionDetected {
                    affected_peers,
                    detected_at: Instant::now(),
                });
            }
        }

        None
    }

    /// Performs periodic maintenance (cleanup old failures, detect partitions, etc.)
    pub async fn perform_maintenance(&self) {
        // Cleanup old failure information
        let mut failed_peers = self.failed_peers.write().await;
        let cutoff_time = Instant::now() - Duration::from_secs(3600);

        failed_peers.retain(|_, info| info.last_failure > cutoff_time || info.is_failed);

        drop(failed_peers);

        if let Some(partition_event) = self.detect_network_partition().await {
            warn!("Network partition detected: {:?}", partition_event);
            let _ = self.event_sender.send(partition_event);
        }

        // Cleanup old operation contexts
        let mut operations = self.active_operations.write().await;
        let operation_cutoff = Instant::now() - Duration::from_secs(300); // 5 minutes

        operations.retain(|_, context| context.started_at > operation_cutoff);
    }
}

impl Default for NetworkErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{NetworkError, NetworkResult};
    use std::net::SocketAddr;
    use std::time::Duration;
    use tokio::time::timeout;

    #[test]
    fn test_operation_context_creation() {
        let context = OperationContext::new(
            "test_operation".to_string(),
            "127.0.0.1:8080".parse().expect("valid address"),
        );

        assert_eq!(context.operation_id, "test_operation");
        assert_eq!(context.peer_address.to_string(), "127.0.0.1:8080");
        assert_eq!(context.retry_count, 0);
        assert!(!context.has_exceeded_max_retries());
        assert!(context.last_error.is_none());
        assert!(context.started_at.elapsed().as_millis() < 100); // Recent creation
    }

    #[test]
    fn test_operation_context_failure_recording() {
        let mut context = OperationContext::new(
            "test_op".to_string(),
            "127.0.0.1:8080".parse().expect("valid address"),
        );

        let error =
            NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test error");
        context.record_failure(&error);

        assert_eq!(context.retry_count, 1);
        assert!(context.last_error.is_some());
        assert!(context
            .last_error
            .as_ref()
            .expect("Last error should be present")
            .to_string()
            .contains("test error"));
    }

    #[test]
    fn test_operation_context_max_retries() {
        let mut context = OperationContext::new(
            "test_op".to_string(),
            "127.0.0.1:8080".parse().expect("valid address"),
        );

        // Should not exceed max retries initially
        assert!(!context.has_exceeded_max_retries());

        // Record failures up to max retries
        let error =
            NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test");
        for i in 0..MAX_RETRY_ATTEMPTS {
            context.record_failure(&error);
            assert_eq!(context.retry_count, i + 1);
        }

        // Should now exceed max retries
        assert!(context.has_exceeded_max_retries());
    }

    #[test]
    fn test_peer_failure_info() {
        let failure_info = PeerFailureInfo {
            failure_count: 0,
            first_failure: Instant::now(),
            last_failure: Instant::now(),
            last_error: String::new(),
            is_failed: false,
            recovery_attempts: 0,
        };

        assert_eq!(failure_info.failure_count, 0);
        assert_eq!(failure_info.recovery_attempts, 0);
        assert!(!failure_info.is_failed);
        assert!(failure_info.first_failure.elapsed().as_millis() < 100); // Recent creation
    }

    #[test]
    fn test_error_severity_classification() {
        let handler = NetworkErrorHandler::new();

        // Test critical errors
        assert_eq!(
            handler.classify_error_severity(&NetworkError::invalid_message(
                SocketAddr::from(([127, 0, 0, 1], 8080)),
                "test",
                "test"
            )),
            ErrorSeverity::Critical
        );
        assert_eq!(
            handler.classify_error_severity(&NetworkError::InvalidHeader {
                peer: SocketAddr::from(([127, 0, 0, 1], 8080)),
                reason: "test".to_string()
            }),
            ErrorSeverity::Critical
        );
        assert_eq!(
            handler.classify_error_severity(&NetworkError::authentication_failed(
                SocketAddr::from(([127, 0, 0, 1], 8080)),
                "test"
            )),
            ErrorSeverity::Critical
        );

        // Test medium errors
        assert_eq!(
            handler.classify_error_severity(&NetworkError::ConnectionTimeout {
                address: "127.0.0.1:8080".parse().expect("valid address"),
                timeout_ms: DEFAULT_TIMEOUT_MS
            }),
            ErrorSeverity::Medium
        );
        assert_eq!(
            handler.classify_error_severity(&NetworkError::connection_failed(
                SocketAddr::from(([127, 0, 0, 1], 8080)),
                "test"
            )),
            ErrorSeverity::Medium
        );
        assert_eq!(
            handler.classify_error_severity(&NetworkError::handshake_failed(
                SocketAddr::from(([127, 0, 0, 1], 8080)),
                "test"
            )),
            ErrorSeverity::Medium
        );

        // Test low severity errors
        assert_eq!(
            handler.classify_error_severity(&NetworkError::Generic {
                reason: "test".to_string()
            }),
            ErrorSeverity::Low
        );
        assert_eq!(
            handler.classify_error_severity(&NetworkError::peer_not_connected(SocketAddr::from((
                [127, 0, 0, 1],
                8080
            )))),
            ErrorSeverity::Low
        );
    }

    // #[test]
    // fn test_recovery_strategy_selection() {
    //     let handler = NetworkErrorHandler::new();

    //     // Test critical error recovery
    //     assert_eq!(
    //         handler.select_recovery_strategy(&NetworkError::invalid_message(SocketAddr::from(([127, 0, 0, 1], 8080)), "test", "test")),
    //         RecoveryStrategy::MarkAsFailed
    //     );

    //     // Test medium error recovery
    //     assert_eq!(
    //         handler.select_recovery_strategy(&NetworkError::ConnectionTimeout {
    //             address: "localhost:8080".parse().expect("valid address"),
    //             timeout_ms: DEFAULT_TIMEOUT_MS
    //         }),
    //         RecoveryStrategy::RetryWithBackoff
    //     );

    //     // Test low error recovery
    //     assert_eq!(
    //         handler.select_recovery_strategy(&NetworkError::Generic {
    //             reason: "test".to_string()
    //         }),
    //         RecoveryStrategy::RetryImmediate
    //     );
    // }

    #[test]
    fn test_error_statistics_default() {
        let stats = ErrorStatistics::default();

        assert!(stats.error_counts.is_empty());
        assert!(stats.peer_errors.is_empty());
        assert_eq!(stats.network_health_score, 100.0);
    }

    #[tokio::test]
    async fn test_network_error_handler_creation() {
        let handler = NetworkErrorHandler::new();

        // Test initial state
        let stats = handler.get_error_statistics().await;
        assert_eq!(stats.network_health_score, 100.0);
        assert!(stats.error_counts.is_empty());

        let failed_peers = handler.get_failed_peers().await;
        assert!(failed_peers.is_empty());
    }

    #[tokio::test]
    async fn test_error_statistics_update() {
        let handler = NetworkErrorHandler::new();
        let peer = "127.0.0.1:8080".parse().expect("valid address");
        let error =
            NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test");

        handler.update_error_statistics(&error, peer).await;

        let stats = handler.get_error_statistics().await;
        assert!(stats.error_counts.len() > 0);
        assert!(stats.peer_errors.contains_key(&peer));
        assert_eq!(stats.peer_errors[&peer], 1);
    }

    #[tokio::test]
    async fn test_multiple_error_statistics_updates() {
        let handler = NetworkErrorHandler::new();
        let peer1 = "127.0.0.1:8080".parse().expect("valid address");
        let peer2 = "127.0.0.1:8081".parse().expect("valid address");

        handler
            .update_error_statistics(
                &NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test1"),
                peer1,
            )
            .await;
        handler
            .update_error_statistics(
                &NetworkError::ConnectionTimeout {
                    address: peer1,
                    timeout_ms: DEFAULT_TIMEOUT_MS,
                },
                peer1,
            )
            .await;
        handler
            .update_error_statistics(
                &NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test2"),
                peer2,
            )
            .await;

        let stats = handler.get_error_statistics().await;
        assert_eq!(stats.peer_errors[&peer1], 2);
        assert_eq!(stats.peer_errors[&peer2], 1);
        assert!(stats.network_health_score < 100.0); // Should decrease with errors
    }

    // #[tokio::test]
    // async fn test_record_error_event() {
    //     let handler = NetworkErrorHandler::new();
    //     let mut event_receiver = handler.subscribe_to_events();
    //     let peer = "localhost:8080".parse().expect("valid address");
    //     let error = NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test error");
    //
    //     // Record error
    //     handler.record_error(error.clone(), peer).await;
    //
    //     // Check event was emitted
    //     let event = timeout(Duration::from_millis(100), event_receiver.recv()).await;
    //     assert!(event.is_ok());
    //
    //     let network_event = event?.expect("operation should succeed");
    //     match network_event {
    //         NetworkErrorEvent::OperationFailed {
    //             peer: event_peer,
    //             ..
    //         } => {
    //             assert_eq!(event_peer, peer);
    //         }
    //         _ => panic!("Expected OperationFailed event"),
    //     }
    // }

    #[tokio::test]
    async fn test_execute_with_retry_success() {
        let handler = NetworkErrorHandler::new();
        let operation_id = "test_success".to_string();
        let peer = "127.0.0.1:8080".parse().expect("valid address");

        let result = handler
            .execute_with_retry(operation_id, peer, || async { Ok("success".to_string()) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.expect("Result should be Ok"), "success");
    }

    #[tokio::test]
    async fn test_execute_with_retry_eventual_success() {
        let handler = NetworkErrorHandler::new();
        let operation_id = "test_eventual_success".to_string();
        let peer = "127.0.0.1:8080".parse().expect("valid address");
        let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let attempt_count_clone = attempt_count.clone();
        let result = handler
            .execute_with_retry(operation_id, peer, || {
                let count = attempt_count_clone.clone();
                async move {
                    let current = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if current < 2 {
                        Err(NetworkError::ConnectionTimeout {
                            address: peer,
                            timeout_ms: DEFAULT_TIMEOUT_MS,
                        })
                    } else {
                        Ok("success after retries".to_string())
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(
            result.expect("Result should be Ok"),
            "success after retries"
        );
        assert_eq!(attempt_count.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_execute_with_retry_max_attempts_exceeded() {
        let handler = NetworkErrorHandler::new();
        let operation_id = "test_max_attempts".to_string();
        let peer = "127.0.0.1:8080".parse().expect("valid address");

        let result: NetworkResult<String> = handler
            .execute_with_retry(operation_id, peer, || async {
                Err(NetworkError::connection_timeout(
                    SocketAddr::from(([127, 0, 0, 1], 8080)),
                    30000,
                )
                .into())
            })
            .await;

        assert!(result.is_err());
        let failed_peers = handler.get_failed_peers().await;
        assert!(!failed_peers.is_empty());
    }

    #[tokio::test]
    async fn test_mark_peer_as_failed() {
        let handler = NetworkErrorHandler::new();
        let peer = "127.0.0.1:8080".parse().expect("valid address");

        handler
            .mark_peer_as_failed(
                peer,
                &NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test"),
            )
            .await;

        let failed_peers = handler.get_failed_peers().await;
        assert_eq!(failed_peers.len(), 1);
        assert_eq!(failed_peers[0].0, peer);
        assert!(failed_peers[0].1.is_failed);
        assert_eq!(failed_peers[0].1.failure_count, 1);
    }

    #[tokio::test]
    async fn test_mark_peer_as_recovered() {
        let handler = NetworkErrorHandler::new();
        let peer = "127.0.0.1:8080".parse().expect("valid address");

        // First mark as failed
        handler
            .mark_peer_as_failed(
                peer,
                &NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test"),
            )
            .await;

        // Then mark as recovered
        handler.mark_peer_as_recovered(peer).await;

        let failed_peers = handler.get_failed_peers().await;
        if let Some((_, failure_info)) = failed_peers.iter().find(|(p, _)| *p == peer) {
            assert!(!failure_info.is_failed);
            assert_eq!(failure_info.recovery_attempts, 0);
        }
    }

    #[tokio::test]
    async fn test_perform_maintenance() {
        let handler = NetworkErrorHandler::new();
        let peer = "127.0.0.1:8080".parse().expect("valid address");

        // Add some errors and failed peers
        handler
            .mark_peer_as_failed(
                peer,
                &NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test"),
            )
            .await;
        handler
            .update_error_statistics(&NetworkError::connection_timeout(peer, 30000), peer)
            .await;

        // Perform maintenance
        handler.perform_maintenance().await;

        // Should still have the data but maintenance should have run
        let stats = handler.get_error_statistics().await;
        assert!(!stats.error_counts.is_empty());
    }

    // #[tokio::test]
    // async fn test_event_subscription() {
    //     let handler = NetworkErrorHandler::new();
    //     let mut receiver1 = handler.subscribe_to_events();
    //     let mut receiver2 = handler.subscribe_to_events();
    //
    //     let peer = "localhost:8080".parse().expect("valid address");
    //     let error = NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test");
    //
    //     // Record error which should emit event
    //     handler.record_error(error, peer).await;
    //
    //     // Both receivers should get the event
    //     let event1 = timeout(Duration::from_millis(100), receiver1.recv()).await;
    //     let event2 = timeout(Duration::from_millis(100), receiver2.recv()).await;
    //
    //     assert!(event1.is_ok());
    //     assert!(event2.is_ok());
    // }

    #[tokio::test]
    async fn test_network_health_score_calculation() {
        let handler = NetworkErrorHandler::new();
        let peer = "127.0.0.1:8080".parse().expect("valid address");

        // Initial health should be 100%
        let initial_stats = handler.get_error_statistics().await;
        assert_eq!(initial_stats.network_health_score, 100.0);

        // Add errors and check health decreases
        for _ in 0..5 {
            handler
                .update_error_statistics(
                    &NetworkError::connection_failed(
                        SocketAddr::from(([127, 0, 0, 1], 8080)),
                        "test",
                    ),
                    peer,
                )
                .await;
        }

        let updated_stats = handler.get_error_statistics().await;
        assert!(updated_stats.network_health_score < 100.0);
        assert!(updated_stats.network_health_score >= 0.0);
    }

    #[test]
    fn test_network_error_event_types() {
        let peer = "127.0.0.1:8080".parse().expect("valid address");
        let error =
            NetworkError::connection_failed(SocketAddr::from(([127, 0, 0, 1], 8080)), "test");
        let downtime = Duration::from_secs(30);

        let error_event = NetworkErrorEvent::OperationFailed {
            operation_id: "test_op".to_string(),
            peer,
            error: error.to_string(),
            total_attempts: 1,
        };

        let peer_failed_event = NetworkErrorEvent::PeerFailed {
            peer,
            reason: error.to_string(),
            failure_count: 3,
        };

        let peer_recovered_event = NetworkErrorEvent::PeerRecovered { peer, downtime };

        // Just verify they can be created without panicking
        match error_event {
            NetworkErrorEvent::OperationFailed {
                peer: event_peer, ..
            } => {
                assert_eq!(event_peer, peer);
            }
            _ => panic!("Wrong event type"),
        }

        match peer_failed_event {
            NetworkErrorEvent::PeerFailed { failure_count, .. } => {
                assert_eq!(failure_count, 3);
            }
            _ => panic!("Wrong event type"),
        }

        match peer_recovered_event {
            NetworkErrorEvent::PeerRecovered {
                downtime: event_downtime,
                ..
            } => {
                assert_eq!(event_downtime, downtime);
            }
            _ => panic!("Wrong event type"),
        }
    }
}

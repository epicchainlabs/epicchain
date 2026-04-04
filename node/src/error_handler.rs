//! Comprehensive error handling for edge cases
//!
//! This module provides centralized error handling, recovery strategies,
//! and graceful degradation for various edge cases in the Neo-RS node.

use anyhow::{Context, Result};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Error severity levels for categorizing issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Critical errors that require immediate attention or shutdown
    Critical,
    /// High severity errors that may affect node operation
    High,
    /// Medium severity errors that should be monitored
    Medium,
    /// Low severity errors that are mostly informational
    Low,
}

/// Error categories for better organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Network-related errors
    Network,
    /// Storage and database errors
    Storage,
    /// Consensus mechanism errors
    Consensus,
    /// Virtual machine errors
    VirtualMachine,
    /// Transaction processing errors
    Transaction,
    /// RPC server errors
    RpcServer,
    /// Configuration errors
    Configuration,
    /// Resource exhaustion errors
    ResourceExhaustion,
    /// Cryptographic errors
    Cryptography,
    /// Synchronization errors
    Synchronization,
}

/// Error tracking information
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    /// When the error occurred
    pub timestamp: Instant,
    /// Error message
    pub message: String,
    /// Error category
    pub category: ErrorCategory,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Number of times this error has occurred
    pub occurrence_count: u64,
    /// Last occurrence time
    pub last_occurrence: Instant,
}

/// Recovery strategy for handling errors
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Retry the operation with exponential backoff
    RetryWithBackoff {
        max_attempts: u32,
        initial_delay: Duration,
    },
    /// Switch to an alternative method
    Fallback(String),
    /// Log and continue operation
    LogAndContinue,
    /// Restart the affected component
    RestartComponent(String),
    /// Graceful shutdown
    GracefulShutdown,
    /// Circuit breaker pattern
    CircuitBreaker { threshold: u32, timeout: Duration },
}

/// Comprehensive error handler for the Neo-RS node
pub struct ErrorHandler {
    /// Error tracking by category
    error_stats: Arc<DashMap<ErrorCategory, Vec<ErrorInfo>>>,
    /// Circuit breakers for different components
    circuit_breakers: Arc<DashMap<String, CircuitBreaker>>,
    /// Recovery strategies
    recovery_strategies: Arc<DashMap<String, RecoveryStrategy>>,
    /// Global error count
    total_errors: Arc<RwLock<u64>>,
    /// Node start time
    start_time: Instant,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new() -> Self {
        let mut handler = Self {
            error_stats: Arc::new(DashMap::new()),
            circuit_breakers: Arc::new(DashMap::new()),
            recovery_strategies: Arc::new(DashMap::new()),
            total_errors: Arc::new(RwLock::new(0)),
            start_time: Instant::now(),
        };

        // Initialize default recovery strategies
        handler.init_default_strategies();
        handler
    }

    /// Initialize default recovery strategies
    fn init_default_strategies(&mut self) {
        use ErrorCategory::{
            Configuration, Consensus, Network, ResourceExhaustion, Storage, VirtualMachine,
        };

        // Network errors: retry with backoff
        self.recovery_strategies.insert(
            format!("{:?}", Network),
            RecoveryStrategy::RetryWithBackoff {
                max_attempts: 5,
                initial_delay: Duration::from_secs(1),
            },
        );

        // Storage errors: circuit breaker
        self.recovery_strategies.insert(
            format!("{:?}", Storage),
            RecoveryStrategy::CircuitBreaker {
                threshold: 10,
                timeout: Duration::from_secs(60),
            },
        );

        // Consensus errors: restart component
        self.recovery_strategies.insert(
            format!("{:?}", Consensus),
            RecoveryStrategy::RestartComponent("consensus".to_string()),
        );

        // VM errors: fallback to safe mode
        self.recovery_strategies.insert(
            format!("{:?}", VirtualMachine),
            RecoveryStrategy::Fallback("safe_vm_execution".to_string()),
        );

        // Resource exhaustion: graceful shutdown
        self.recovery_strategies.insert(
            format!("{:?}", ResourceExhaustion),
            RecoveryStrategy::GracefulShutdown,
        );
    }

    /// Handle an error with appropriate recovery strategy
    pub async fn handle_error(
        &self,
        error: anyhow::Error,
        category: ErrorCategory,
        severity: ErrorSeverity,
        context: &str,
    ) -> Result<RecoveryAction> {
        // Increment error count
        {
            let mut total = self.total_errors.write().await;
            *total += 1;
        }

        // Track error
        self.track_error(&error, category, severity).await;

        // Log error based on severity
        match severity {
            ErrorSeverity::Critical => {
                error!(
                    "🔴 CRITICAL ERROR in {}: {} - {}",
                    context,
                    category_name(category),
                    error
                );
                error!("   Stack trace: {:?}", error);
            }
            ErrorSeverity::High => {
                error!(
                    "🟠 HIGH SEVERITY ERROR in {}: {} - {}",
                    context,
                    category_name(category),
                    error
                );
            }
            ErrorSeverity::Medium => {
                warn!(
                    "🟡 Medium severity error in {}: {} - {}",
                    context,
                    category_name(category),
                    error
                );
            }
            ErrorSeverity::Low => {
                debug!(
                    "🔵 Low severity error in {}: {} - {}",
                    context,
                    category_name(category),
                    error
                );
            }
        }

        // Get recovery strategy
        let strategy = self.get_recovery_strategy(category, severity).await;

        // Execute recovery
        match strategy {
            RecoveryStrategy::RetryWithBackoff {
                max_attempts,
                initial_delay,
            } => Ok(RecoveryAction::Retry {
                max_attempts,
                delay: initial_delay,
            }),
            RecoveryStrategy::Fallback(method) => {
                info!("Falling back to alternative method: {}", method);
                Ok(RecoveryAction::UseFallback(method))
            }
            RecoveryStrategy::LogAndContinue => Ok(RecoveryAction::Continue),
            RecoveryStrategy::RestartComponent(component) => {
                warn!("Component restart required: {}", component);
                Ok(RecoveryAction::RestartComponent(component))
            }
            RecoveryStrategy::GracefulShutdown => {
                error!("Initiating graceful shutdown due to critical error");
                Ok(RecoveryAction::Shutdown)
            }
            RecoveryStrategy::CircuitBreaker { threshold, timeout } => {
                let breaker_open = self.check_circuit_breaker(context, threshold).await;
                if breaker_open {
                    warn!("Circuit breaker OPEN for {}, failing fast", context);
                    Ok(RecoveryAction::FailFast)
                } else {
                    Ok(RecoveryAction::Continue)
                }
            }
        }
    }

    /// Track error occurrence
    async fn track_error(
        &self,
        error: &anyhow::Error,
        category: ErrorCategory,
        severity: ErrorSeverity,
    ) {
        let mut errors = self.error_stats.entry(category).or_insert_with(Vec::new);

        let error_msg = error.to_string();
        if let Some(existing) = errors.iter_mut().find(|e| e.message == error_msg) {
            existing.occurrence_count += 1;
            existing.last_occurrence = Instant::now();
        } else {
            errors.push(ErrorInfo {
                timestamp: Instant::now(),
                message: error_msg,
                category,
                severity,
                occurrence_count: 1,
                last_occurrence: Instant::now(),
            });
        }

        if errors.len() > 1000 {
            let drain_count = errors.len() - 1000;
            errors.drain(0..drain_count);
        }
    }

    /// Get recovery strategy for error
    async fn get_recovery_strategy(
        &self,
        category: ErrorCategory,
        severity: ErrorSeverity,
    ) -> RecoveryStrategy {
        if let Some(strategy) = self.recovery_strategies.get(&format!("{:?}", category)) {
            return strategy.clone();
        }

        // Default strategies based on severity
        match severity {
            ErrorSeverity::Critical => RecoveryStrategy::GracefulShutdown,
            ErrorSeverity::High => RecoveryStrategy::RestartComponent("affected".to_string()),
            ErrorSeverity::Medium => RecoveryStrategy::RetryWithBackoff {
                max_attempts: 3,
                initial_delay: Duration::from_secs(1),
            },
            ErrorSeverity::Low => RecoveryStrategy::LogAndContinue,
        }
    }

    /// Check circuit breaker status
    async fn check_circuit_breaker(&self, context: &str, threshold: u32) -> bool {
        let mut breakers = self
            .circuit_breakers
            .entry(context.to_string())
            .or_insert_with(|| CircuitBreaker::new(threshold, Duration::from_secs(60)));

        breakers.record_failure();
        breakers.is_open()
    }

    /// Get error statistics
    pub async fn get_error_stats(&self) -> ErrorStatistics {
        let total_errors = *self.total_errors.read().await;
        let uptime = self.start_time.elapsed();

        let mut stats_by_category = Vec::new();
        for entry in self.error_stats.iter() {
            let (category, errors) = entry.pair();
            let total = errors.len() as u64;
            let recent = errors
                .iter()
                .filter(|e| e.last_occurrence.elapsed() < Duration::from_secs(300))
                .count() as u64;

            stats_by_category.push(CategoryStats {
                category: *category,
                total_errors: total,
                recent_errors: recent,
            });
        }

        ErrorStatistics {
            total_errors,
            errors_per_hour: (total_errors as f64 / uptime.as_secs_f64()) * 3600.0,
            uptime,
            stats_by_category,
        }
    }

    /// Clear old error records
    pub async fn cleanup_old_errors(&self, max_age: Duration) {
        let now = Instant::now();

        for mut entry in self.error_stats.iter_mut() {
            let errors = entry.value_mut();
            errors.retain(|e| now.duration_since(e.last_occurrence) < max_age);
        }
    }
}

/// Circuit breaker implementation
#[derive(Debug, Clone)]
struct CircuitBreaker {
    failure_count: u32,
    threshold: u32,
    last_failure: Option<Instant>,
    timeout: Duration,
    state: CircuitState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    fn new(threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_count: 0,
            threshold,
            last_failure: None,
            timeout,
            state: CircuitState::Closed,
        }
    }

    fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());

        if self.failure_count >= self.threshold {
            self.state = CircuitState::Open;
        }
    }

    fn record_success(&mut self) {
        if self.state == CircuitState::HalfOpen {
            self.state = CircuitState::Closed;
            self.failure_count = 0;
        }
    }

    fn is_open(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => false,
            CircuitState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed() > self.timeout {
                        self.state = CircuitState::HalfOpen;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => false,
        }
    }
}

/// Recovery action to take
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    /// Retry the operation
    Retry { max_attempts: u32, delay: Duration },
    /// Use fallback method
    UseFallback(String),
    /// Continue operation
    Continue,
    /// Restart component
    RestartComponent(String),
    /// Shutdown node
    Shutdown,
    /// Fail fast (circuit breaker open)
    FailFast,
}

/// Error statistics
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    pub total_errors: u64,
    pub errors_per_hour: f64,
    pub uptime: Duration,
    pub stats_by_category: Vec<CategoryStats>,
}

#[derive(Debug, Clone)]
pub struct CategoryStats {
    pub category: ErrorCategory,
    pub total_errors: u64,
    pub recent_errors: u64,
}

/// Get human-readable category name
fn category_name(category: ErrorCategory) -> &'static str {
    match category {
        ErrorCategory::Network => "Network",
        ErrorCategory::Storage => "Storage",
        ErrorCategory::Consensus => "Consensus",
        ErrorCategory::VirtualMachine => "VM",
        ErrorCategory::Transaction => "Transaction",
        ErrorCategory::RpcServer => "RPC",
        ErrorCategory::Configuration => "Config",
        ErrorCategory::ResourceExhaustion => "Resources",
        ErrorCategory::Cryptography => "Crypto",
        ErrorCategory::Synchronization => "Sync",
    }
}

/// Retry helper with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_attempts: u32,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;

    if max_attempts == 0 {
        error!("Invalid max_attempts: cannot be 0");
        // Since we can't return an error without knowing the error type,
        return operation();
    }

    for attempt in 1..=max_attempts {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_attempts => {
                warn!(
                    "Attempt {} failed: {}. Retrying in {:?}/* implementation */;",
                    attempt, e, delay
                );
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
            Err(e) => {
                error!("All {} attempts failed. Last error: {}", max_attempts, e);
                return Err(e);
            }
        }
    }

    // This should never be reached since the loop handles all cases,
    error!("Unexpected state in retry_with_backoff - this should not happen");
    operation()
}

/// Async retry helper with exponential backoff
pub async fn retry_with_backoff_async<F, Fut, T, E>(
    mut operation: F,
    max_attempts: u32,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay;

    if max_attempts == 0 {
        error!("Invalid max_attempts: cannot be 0");
        // Since we can't return an error without knowing the error type,
        return operation().await;
    }

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_attempts => {
                warn!(
                    "Attempt {} failed: {}. Retrying in {:?}/* implementation */;",
                    attempt, e, delay
                );
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
            Err(e) => {
                error!("All {} attempts failed. Last error: {}", max_attempts, e);
                return Err(e);
            }
        }
    }

    // This should never be reached since the loop handles all cases,
    error!("Unexpected state in retry_with_backoff_async - this should not happen");
    operation().await
}

/// Helper macro for handling errors with context
#[macro_export]
macro_rules! handle_error {
    ($handler:expr, $result:expr, $category:expr, $severity:expr, $context:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err(e) => {
                let action = $handler
                    .handle_error(e.into(), $category, $severity, $context)
                    .await?;
                match action {
                    RecoveryAction::Continue => Ok(Default::default()),
                    RecoveryAction::Shutdown => Err(anyhow::anyhow!("Shutdown required")),
                    _ => Err(anyhow::anyhow!("Error handling required: {:?}", action)),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_handler_creation() {
        let handler = ErrorHandler::new();
        let stats = handler.get_error_stats().await;
        assert_eq!(stats.total_errors, 0);
    }

    #[tokio::test]
    async fn test_error_tracking() {
        let handler = ErrorHandler::new();

        let error = anyhow::anyhow!("Test network error");
        handler
            .handle_error(
                error,
                ErrorCategory::Network,
                ErrorSeverity::Medium,
                "test_context",
            )
            .await
            .expect("operation should succeed");

        let stats = handler.get_error_stats().await;
        assert_eq!(stats.total_errors, 1);
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(60));

        assert!(!breaker.is_open());

        breaker.record_failure();
        assert!(!breaker.is_open());

        breaker.record_failure();
        // After reaching threshold (3), one more failure will open the breaker in this implementation
        breaker.record_failure();
        assert!(breaker.is_open());
    }

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let mut counter = 0;
        let result = retry_with_backoff(
            || {
                counter += 1;
                if counter < 3 {
                    Err("Still failing")
                } else {
                    Ok("Success")
                }
            },
            5,
            Duration::from_millis(10),
        )
        .await;

        assert_eq!(result.expect("operation should succeed"), "Success");
        assert_eq!(counter, 3);
    }
}

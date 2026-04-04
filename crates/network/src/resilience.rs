//! Network resilience patterns
//!
//! This module provides resilience patterns for handling network failures,
//! including circuit breakers, retry logic, and fallback mechanisms.

use crate::{NetworkError, NetworkResult as Result};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents an enumeration of values.
pub enum CircuitState {
    /// Circuit is closed - normal operation
    Closed,
    /// Circuit is open - requests are blocked
    Open,
    /// Circuit is half-open - testing if service recovered
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
/// Represents a data structure.
pub struct CircuitBreaker {
    /// Current state
    state: Arc<RwLock<CircuitState>>,
    /// Failure count
    failure_count: AtomicU32,
    /// Success count in half-open state
    success_count: AtomicU32,
    /// Last failure time
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    /// Configuration
    config: CircuitBreakerConfig,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
/// Represents a data structure.
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    /// Success threshold to close circuit from half-open
    pub success_threshold: u32,
    /// Timeout before attempting to close circuit
    pub timeout: Duration,
    /// Reset timeout after successful operations
    pub reset_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(30),
            reset_timeout: Duration::from_secs(60),
        }
    }
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    /// Creates a new instance.
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: AtomicU32::new(0),
            success_count: AtomicU32::new(0),
            last_failure_time: Arc::new(RwLock::new(None)),
            config,
        }
    }

    /// Check if request should be allowed
    pub async fn should_allow_request(&self) -> Result<()> {
        let mut state = self.state.write().await;

        match *state {
            CircuitState::Open => {
                // Check if timeout has passed
                if let Some(last_failure) = *self.last_failure_time.read().await {
                    if last_failure.elapsed() >= self.config.timeout {
                        *state = CircuitState::HalfOpen;
                        self.success_count.store(0, Ordering::Relaxed);
                    } else {
                        return Err(NetworkError::CircuitBreakerOpen {
                            reason: "Circuit breaker is open".to_string(),
                        });
                    }
                }
            }
            CircuitState::HalfOpen => {
                // Allow limited requests in half-open state
            }
            CircuitState::Closed => {
                // Normal operation
            }
        }

        Ok(())
    }

    /// Record a successful operation
    pub async fn record_success(&self) {
        let mut state = self.state.write().await;

        match *state {
            CircuitState::HalfOpen => {
                let count = self.success_count.fetch_add(1, Ordering::Relaxed) + 1;
                if count >= self.config.success_threshold {
                    *state = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::Relaxed);
                    self.success_count.store(0, Ordering::Relaxed);
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::Relaxed);
            }
            _ => {}
        }
    }

    /// Record a failed operation
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;

        match *state {
            CircuitState::Closed => {
                let count = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                if count >= self.config.failure_threshold {
                    *state = CircuitState::Open;
                    *self.last_failure_time.write().await = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                // Any failure in half-open state reopens the circuit
                *state = CircuitState::Open;
                *self.last_failure_time.write().await = Some(Instant::now());
                self.failure_count.store(0, Ordering::Relaxed);
            }
            _ => {}
        }
    }

    /// Get current state
    pub async fn get_state(&self) -> CircuitState {
        *self.state.read().await
    }
}

/// Adaptive retry strategy with exponential backoff
/// Represents a data structure.
pub struct AdaptiveRetry {
    /// Base delay between retries
    base_delay: Duration,
    /// Maximum delay between retries
    max_delay: Duration,
    /// Maximum number of retries
    max_retries: u32,
    /// Jitter factor (0.0 to 1.0)
    jitter_factor: f64,
    /// Success rate tracking
    success_rate: Arc<RwLock<f64>>,
    /// Total attempts
    total_attempts: AtomicU64,
    /// Successful attempts
    successful_attempts: AtomicU64,
}

impl AdaptiveRetry {
    /// Create a new adaptive retry strategy
    /// Creates a new instance.
    pub fn new(base_delay: Duration, max_delay: Duration, max_retries: u32) -> Self {
        Self {
            base_delay,
            max_delay,
            max_retries,
            jitter_factor: 0.1,
            success_rate: Arc::new(RwLock::new(1.0)),
            total_attempts: AtomicU64::new(0),
            successful_attempts: AtomicU64::new(0),
        }
    }

    /// Execute operation with retry
    pub async fn execute<F, Fut, T>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut attempt = 0;
        let mut delay = self.base_delay;

        loop {
            self.total_attempts.fetch_add(1, Ordering::Relaxed);

            match f().await {
                Ok(result) => {
                    self.successful_attempts.fetch_add(1, Ordering::Relaxed);
                    self.update_success_rate().await;
                    return Ok(result);
                }
                Err(e) if attempt >= self.max_retries => {
                    self.update_success_rate().await;
                    return Err(e);
                }
                Err(e) if !self.should_retry(&e) => {
                    self.update_success_rate().await;
                    return Err(e);
                }
                Err(_) => {
                    attempt += 1;

                    // Apply exponential backoff with jitter
                    let jitter = self.calculate_jitter(delay);
                    tokio::time::sleep(delay + jitter).await;

                    // Increase delay for next attempt
                    delay = (delay * 2).min(self.max_delay);
                }
            }
        }
    }

    /// Check if error is retryable
    fn should_retry(&self, error: &NetworkError) -> bool {
        match error {
            NetworkError::ConnectionTimeout { .. }
            | NetworkError::ConnectionRefused { .. }
            | NetworkError::TemporaryFailure { .. } => true,
            _ => false,
        }
    }

    /// Calculate jitter for delay
    fn calculate_jitter(&self, delay: Duration) -> Duration {
        use rand::Rng;
        let jitter_ms = (delay.as_millis() as f64 * self.jitter_factor) as u64;
        let random_jitter = rand::thread_rng().gen_range(0..=jitter_ms);
        Duration::from_millis(random_jitter)
    }

    /// Update success rate
    async fn update_success_rate(&self) {
        let total = self.total_attempts.load(Ordering::Relaxed);
        let successful = self.successful_attempts.load(Ordering::Relaxed);

        if total > 0 {
            let rate = successful as f64 / total as f64;
            *self.success_rate.write().await = rate;

            // Adjust retry strategy based on success rate
            if rate < 0.5 && total > 10 {
                // Poor success rate - might want to adjust strategy
                tracing::warn!("Low success rate: {:.2}%", rate * 100.0);
            }
        }
    }

    /// Get current success rate
    pub async fn get_success_rate(&self) -> f64 {
        *self.success_rate.read().await
    }
}

/// Bulkhead pattern for resource isolation
/// Represents a data structure.
pub struct Bulkhead {
    /// Maximum concurrent operations
    max_concurrent: usize,
    /// Current concurrent operations
    current_concurrent: AtomicU32,
    /// Queue for waiting operations
    max_queue_size: usize,
    /// Current queue size
    queue_size: AtomicU32,
}

impl Bulkhead {
    /// Create a new bulkhead
    /// Creates a new instance.
    pub fn new(max_concurrent: usize, max_queue_size: usize) -> Self {
        Self {
            max_concurrent,
            current_concurrent: AtomicU32::new(0),
            max_queue_size,
            queue_size: AtomicU32::new(0),
        }
    }

    /// Acquire a permit to execute
    pub fn try_acquire(&self) -> Result<BulkheadPermit> {
        let current = self.current_concurrent.load(Ordering::Relaxed);

        if current as usize >= self.max_concurrent {
            // Try to queue
            let queue = self.queue_size.load(Ordering::Relaxed);
            if queue as usize >= self.max_queue_size {
                return Err(NetworkError::ResourceExhausted {
                    resource: "bulkhead_queue".to_string(),
                    used: queue as u64,
                    limit: self.max_queue_size as u64,
                });
            }

            self.queue_size.fetch_add(1, Ordering::Relaxed);
            return Err(NetworkError::Queued {
                reason: "Request queued in bulkhead".to_string(),
            });
        }

        self.current_concurrent.fetch_add(1, Ordering::Relaxed);
        Ok(BulkheadPermit { bulkhead: self })
    }

    /// Release a permit
    fn release(&self) {
        self.current_concurrent.fetch_sub(1, Ordering::Relaxed);

        // Process queued item if any
        let queue = self.queue_size.load(Ordering::Relaxed);
        if queue > 0 {
            self.queue_size.fetch_sub(1, Ordering::Relaxed);
            self.current_concurrent.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Permit for bulkhead execution
/// Represents a data structure.
pub struct BulkheadPermit<'a> {
    bulkhead: &'a Bulkhead,
}

impl<'a> Drop for BulkheadPermit<'a> {
    fn drop(&mut self) {
        self.bulkhead.release();
    }
}

/// Health check for network services
/// Represents a data structure.
pub struct HealthChecker {
    /// Check interval
    check_interval: Duration,
    /// Timeout for health checks
    check_timeout: Duration,
    /// Last check time
    last_check: Arc<RwLock<Option<Instant>>>,
    /// Health status
    is_healthy: Arc<RwLock<bool>>,
}

impl HealthChecker {
    /// Create a new health checker
    /// Creates a new instance.
    pub fn new(check_interval: Duration, check_timeout: Duration) -> Self {
        Self {
            check_interval,
            check_timeout,
            last_check: Arc::new(RwLock::new(None)),
            is_healthy: Arc::new(RwLock::new(true)),
        }
    }

    /// Check if service is healthy
    pub async fn is_healthy(&self) -> bool {
        // Check if we need to perform a health check
        let should_check = {
            let last = *self.last_check.read().await;
            match last {
                None => true,
                Some(time) => time.elapsed() >= self.check_interval,
            }
        };

        if should_check {
            self.perform_health_check().await;
        }

        *self.is_healthy.read().await
    }

    /// Perform health check
    async fn perform_health_check(&self) {
        *self.last_check.write().await = Some(Instant::now());

        // Simulate health check (in real implementation, would ping service)
        let check_future = tokio::time::sleep(Duration::from_millis(10));

        match tokio::time::timeout(self.check_timeout, check_future).await {
            Ok(_) => {
                *self.is_healthy.write().await = true;
            }
            Err(_) => {
                *self.is_healthy.write().await = false;
                tracing::warn!("Health check timed out");
            }
        }
    }

    /// Mark service as unhealthy
    pub async fn mark_unhealthy(&self) {
        *self.is_healthy.write().await = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            reset_timeout: Duration::from_millis(200),
        };

        let breaker = CircuitBreaker::new(config);

        // Initially closed
        assert_eq!(breaker.get_state().await, CircuitState::Closed);
        assert!(breaker.should_allow_request().await.is_ok());

        // Record failures to open circuit
        breaker.record_failure().await;
        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Open);

        // Circuit is open
        assert!(breaker.should_allow_request().await.is_err());

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert!(breaker.should_allow_request().await.is_ok());
        assert_eq!(breaker.get_state().await, CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_adaptive_retry() {
        let retry = AdaptiveRetry::new(Duration::from_millis(10), Duration::from_millis(100), 3);

        let attempt = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let attempt_clone = attempt.clone();

        let result = retry
            .execute(move || {
                let attempt_val =
                    attempt_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                async move {
                    if attempt_val < 3 {
                        Err(NetworkError::ConnectionTimeout {
                            address: "127.0.0.1:8333".parse().unwrap(),
                            timeout_ms: 1000,
                        })
                    } else {
                        Ok(42)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt.load(std::sync::atomic::Ordering::Relaxed), 3);
    }

    #[test]
    fn test_bulkhead() {
        let bulkhead = Bulkhead::new(2, 1);

        // First two should succeed
        let permit1 = bulkhead.try_acquire();
        assert!(permit1.is_ok());

        let permit2 = bulkhead.try_acquire();
        assert!(permit2.is_ok());

        // Third should be queued
        let permit3 = bulkhead.try_acquire();
        assert!(permit3.is_err());

        // Drop a permit to free capacity
        drop(permit1);

        // Now should succeed
        let permit4 = bulkhead.try_acquire();
        assert!(permit4.is_ok());
    }
}

//! Graceful Shutdown Handling
//!
//! This module provides comprehensive graceful shutdown handling for the Neo node,
//! ensuring all components shut down cleanly and in the correct order, matching
//! the C# Neo implementation exactly.

use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::{broadcast, Notify, RwLock};
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

/// Maximum time to wait for graceful shutdown
pub const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

/// Time to wait between shutdown stages
pub const SHUTDOWN_STAGE_DELAY: Duration = Duration::from_millis(100);

/// Shutdown-specific errors
#[derive(Error, Debug)]
/// Represents an enumeration of values.
pub enum ShutdownError {
    #[error("Shutdown timeout exceeded")]
    /// Shutdown timed out
    Timeout,

    #[error("Component failed to shutdown: {0}")]
    /// Component error occurred
    ComponentError(String),

    #[error("Shutdown already in progress")]
    /// Shutdown already in progress
    AlreadyInProgress,

    #[error("Shutdown cancelled")]
    /// Shutdown was cancelled
    Cancelled,
}

/// Shutdown stages (matches C# Neo shutdown sequence exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Represents an enumeration of values.
pub enum ShutdownStage {
    /// Initial stage - prepare for shutdown
    Prepare,
    /// Stop accepting new connections
    StopAcceptingConnections,
    /// Stop consensus activities
    StopConsensus,
    /// Stop network services
    StopNetwork,
    /// Stop RPC services
    StopRpc,
    /// Flush pending transactions
    FlushTransactions,
    /// Save blockchain state
    SaveState,
    /// Close database connections
    CloseDatabase,
    /// Final cleanup
    Cleanup,
    /// Shutdown complete
    Complete,
}

impl std::fmt::Display for ShutdownStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShutdownStage::Prepare => write!(f, "Preparing for shutdown"),
            ShutdownStage::StopAcceptingConnections => write!(f, "Stopping new connections"),
            ShutdownStage::StopConsensus => write!(f, "Stopping consensus"),
            ShutdownStage::StopNetwork => write!(f, "Stopping network services"),
            ShutdownStage::StopRpc => write!(f, "Stopping RPC services"),
            ShutdownStage::FlushTransactions => write!(f, "Flushing pending transactions"),
            ShutdownStage::SaveState => write!(f, "Saving blockchain state"),
            ShutdownStage::CloseDatabase => write!(f, "Closing database connections"),
            ShutdownStage::Cleanup => write!(f, "Performing final cleanup"),
            ShutdownStage::Complete => write!(f, "Shutdown complete"),
        }
    }
}

/// Shutdown events for monitoring
#[derive(Debug, Clone)]
/// Represents an enumeration of values.
pub enum ShutdownEvent {
    /// Shutdown initiated
    Initiated {
        /// Reason for shutdown
        reason: String,
        /// Timestamp of event
        timestamp: std::time::SystemTime,
    },
    /// Stage started
    StageStarted {
        /// Current shutdown stage
        stage: ShutdownStage,
        /// Timestamp of event
        timestamp: std::time::SystemTime,
    },
    /// Stage completed
    StageCompleted {
        /// Current shutdown stage
        stage: ShutdownStage,
        /// Duration of operation
        duration: Duration,
        /// Timestamp of event
        timestamp: std::time::SystemTime,
    },
    /// Stage failed
    StageFailed {
        /// Current shutdown stage
        stage: ShutdownStage,
        error: String,
        /// Timestamp of event
        timestamp: std::time::SystemTime,
    },
    /// Shutdown completed
    Completed {
        /// Duration of operation
        total_duration: Duration,
        /// Timestamp of event
        timestamp: std::time::SystemTime,
    },
    /// Shutdown failed
    Failed {
        error: String,
        /// Timestamp of event
        timestamp: std::time::SystemTime,
    },
}

/// Component that can be shut down gracefully
#[async_trait::async_trait]
/// Defines a trait interface.
pub trait Shutdown: Send + Sync {
    /// Component name for logging
    fn name(&self) -> &str;

    /// Shutdown the component gracefully
    async fn shutdown(&self) -> std::result::Result<(), ShutdownError>;

    /// Check if component is ready to shutdown
    async fn can_shutdown(&self) -> bool {
        true
    }

    /// Priority for shutdown order (lower = earlier)
    fn shutdown_priority(&self) -> u32 {
        100
    }
}

/// Graceful shutdown coordinator
/// Represents a data structure.
pub struct ShutdownCoordinator {
    /// Current shutdown stage
    current_stage: Arc<RwLock<Option<ShutdownStage>>>,
    /// Shutdown signal notifier
    shutdown_notify: Arc<Notify>,
    /// Event broadcaster
    event_sender: broadcast::Sender<ShutdownEvent>,
    /// Registered components
    components: Arc<RwLock<Vec<Arc<dyn Shutdown>>>>,
    /// Shutdown in progress flag
    is_shutting_down: Arc<RwLock<bool>>,
    /// Start time of shutdown
    shutdown_start_time: Arc<RwLock<Option<std::time::Instant>>>,
}

impl ShutdownCoordinator {
    /// Creates a new shutdown coordinator
    /// Creates a new instance.
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);

        Self {
            current_stage: Arc::new(RwLock::new(None)),
            shutdown_notify: Arc::new(Notify::new()),
            event_sender,
            components: Arc::new(RwLock::new(Vec::new())),
            is_shutting_down: Arc::new(RwLock::new(false)),
            shutdown_start_time: Arc::new(RwLock::new(None)),
        }
    }

    /// Registers a component for graceful shutdown
    pub async fn register_component(&self, component: Arc<dyn Shutdown>) {
        info!("Registering component for shutdown: {}", component.name());
        let mut components = self.components.write().await;
        components.push(component);

        // Sort by priority
        components.sort_by_key(|c| c.shutdown_priority());
    }

    /// Subscribes to shutdown events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<ShutdownEvent> {
        self.event_sender.subscribe()
    }

    /// Gets a shutdown signal that can be awaited
    /// Gets a value from the internal state.
    pub fn get_shutdown_signal(&self) -> Arc<Notify> {
        Arc::clone(&self.shutdown_notify)
    }

    /// Initiates graceful shutdown
    pub async fn initiate_shutdown(&self, reason: String) -> Result<(), ShutdownError> {
        {
            let mut is_shutting_down = self.is_shutting_down.write().await;
            if *is_shutting_down {
                return Err(ShutdownError::AlreadyInProgress);
            }
            *is_shutting_down = true;
        }

        info!("🛑 Initiating graceful shutdown: {}", reason);

        // Record start time
        *self.shutdown_start_time.write().await = Some(std::time::Instant::now());

        // Emit shutdown initiated event
        let _ = self.event_sender.send(ShutdownEvent::Initiated {
            reason: reason.clone(),
            timestamp: std::time::SystemTime::now(),
        });

        // Notify all waiters
        self.shutdown_notify.notify_waiters();

        // Execute shutdown sequence
        match timeout(GRACEFUL_SHUTDOWN_TIMEOUT, self.execute_shutdown_sequence()).await {
            Ok(Ok(())) => {
                let duration = self
                    .shutdown_start_time
                    .read()
                    .await
                    .map(|start| start.elapsed())
                    .unwrap_or_default();

                info!("✅ Graceful shutdown completed in {:?}", duration);

                let _ = self.event_sender.send(ShutdownEvent::Completed {
                    total_duration: duration,
                    timestamp: std::time::SystemTime::now(),
                });

                Ok(())
            }
            Ok(Err(e)) => {
                error!("❌ Shutdown failed: {}", e);

                let _ = self.event_sender.send(ShutdownEvent::Failed {
                    error: e.to_string(),
                    timestamp: std::time::SystemTime::now(),
                });

                Err(e)
            }
            Err(_) => {
                error!("❌ Shutdown timeout exceeded");

                let _ = self.event_sender.send(ShutdownEvent::Failed {
                    error: "Timeout exceeded".to_string(),
                    timestamp: std::time::SystemTime::now(),
                });

                Err(ShutdownError::Timeout)
            }
        }
    }

    /// Executes the shutdown sequence
    async fn execute_shutdown_sequence(&self) -> Result<(), ShutdownError> {
        let stages = [
            ShutdownStage::Prepare,
            ShutdownStage::StopAcceptingConnections,
            ShutdownStage::StopConsensus,
            ShutdownStage::StopNetwork,
            ShutdownStage::StopRpc,
            ShutdownStage::FlushTransactions,
            ShutdownStage::SaveState,
            ShutdownStage::CloseDatabase,
            ShutdownStage::Cleanup,
            ShutdownStage::Complete,
        ];

        for stage in stages {
            self.execute_shutdown_stage(stage).await?;

            // Small delay between stages
            if stage != ShutdownStage::Complete {
                sleep(SHUTDOWN_STAGE_DELAY).await;
            }
        }

        Ok(())
    }

    /// Executes a specific shutdown stage
    async fn execute_shutdown_stage(&self, stage: ShutdownStage) -> Result<(), ShutdownError> {
        info!("📍 Shutdown stage: {}", stage);

        // Update current stage
        *self.current_stage.write().await = Some(stage);

        // Emit stage started event
        let stage_start = std::time::Instant::now();
        let _ = self.event_sender.send(ShutdownEvent::StageStarted {
            stage,
            timestamp: std::time::SystemTime::now(),
        });

        // Execute stage-specific shutdown logic
        match stage {
            ShutdownStage::Prepare => {
                debug!("Preparing components for shutdown");
            }

            ShutdownStage::StopAcceptingConnections => {
                // Stop accepting new connections
                self.shutdown_components_by_priority(0..20).await?;
            }

            ShutdownStage::StopConsensus => {
                // Stop consensus activities
                self.shutdown_components_by_priority(20..40).await?;
            }

            ShutdownStage::StopNetwork => {
                // Stop network services
                self.shutdown_components_by_priority(40..60).await?;
            }

            ShutdownStage::StopRpc => {
                // Stop RPC services
                self.shutdown_components_by_priority(60..80).await?;
            }

            ShutdownStage::FlushTransactions => {
                // Flush pending transactions
                self.shutdown_components_by_priority(80..100).await?;
            }

            ShutdownStage::SaveState => {
                // Save blockchain state
                self.shutdown_components_by_priority(100..120).await?;
            }

            ShutdownStage::CloseDatabase => {
                // Close database connections
                self.shutdown_components_by_priority(120..140).await?;
            }

            ShutdownStage::Cleanup => {
                // Final cleanup
                self.shutdown_components_by_priority(140..200).await?;
            }

            ShutdownStage::Complete => {
                // Nothing to do
            }
        }

        // Emit stage completed event
        let stage_duration = stage_start.elapsed();
        let _ = self.event_sender.send(ShutdownEvent::StageCompleted {
            stage,
            duration: stage_duration,
            timestamp: std::time::SystemTime::now(),
        });

        Ok(())
    }

    /// Shuts down components within a priority range
    async fn shutdown_components_by_priority(
        &self,
        priority_range: std::ops::Range<u32>,
    ) -> Result<(), ShutdownError> {
        let components = self.components.read().await;

        for component in components.iter() {
            let priority = component.shutdown_priority();
            if priority_range.contains(&priority) {
                debug!(
                    "Shutting down component: {} (priority: {})",
                    component.name(),
                    priority
                );

                if !component.can_shutdown().await {
                    warn!(
                        "Component {} is not ready to shutdown, waiting/* implementation */;",
                        component.name()
                    );

                    // Wait a bit and check again
                    for _ in 0..10 {
                        sleep(Duration::from_millis(100)).await;
                        if component.can_shutdown().await {
                            break;
                        }
                    }
                }

                // Shutdown the component
                match component.shutdown().await {
                    Ok(()) => {
                        debug!("✅ Component {} shutdown successfully", component.name());
                    }
                    Err(e) => {
                        error!(
                            "❌ Failed to shutdown component {}: {}",
                            component.name(),
                            e
                        );
                        return Err(ShutdownError::ComponentError(format!(
                            "{}: {}",
                            component.name(),
                            e
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Gets the current shutdown stage
    pub async fn current_stage(&self) -> Option<ShutdownStage> {
        *self.current_stage.read().await
    }

    /// Checks if shutdown is in progress
    pub async fn is_shutting_down(&self) -> bool {
        *self.is_shutting_down.read().await
    }
}

impl Default for ShutdownCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal handler for graceful shutdown
/// Represents a data structure.
pub struct SignalHandler {
    shutdown_coordinator: Arc<ShutdownCoordinator>,
}

impl SignalHandler {
    /// Creates a new signal handler
    /// Creates a new instance.
    pub fn new(shutdown_coordinator: Arc<ShutdownCoordinator>) -> Self {
        Self {
            shutdown_coordinator,
        }
    }

    /// Starts listening for shutdown signals
    pub async fn start(self) {
        tokio::spawn(async move {
            self.handle_signals().await;
        });
    }

    #[cfg(unix)]
    async fn handle_signals(&self) {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");
        let mut sighup = signal(SignalKind::hangup()).expect("Failed to install SIGHUP handler");

        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM");
                let _ = self.shutdown_coordinator
                    .initiate_shutdown("SIGTERM received".to_string()).await;
            }
            _ = sigint.recv() => {
                info!("Received SIGINT");
                let _ = self.shutdown_coordinator
                    .initiate_shutdown("SIGINT received".to_string()).await;
            }
            _ = sighup.recv() => {
                info!("Received SIGHUP");
                let _ = self.shutdown_coordinator
                    .initiate_shutdown("SIGHUP received".to_string()).await;
            }
        }
    }

    #[cfg(windows)]
    async fn handle_signals(&self) {
        use tokio::signal;

        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C");
                let _ = self
                    .shutdown_coordinator
                    .initiate_shutdown("Ctrl+C received".to_string())
                    .await;
            }
            Err(e) => {
                error!("Failed to listen for Ctrl+C: {}", e);
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    struct TestComponent {
        name: String,
        priority: u32,
        shutdown_delay: Duration,
    }
    #[async_trait::async_trait]
    impl Shutdown for TestComponent {
        fn name(&self) -> &str {
            &self.name
        }
        async fn shutdown(&self) -> std::result::Result<(), ShutdownError> {
            sleep(self.shutdown_delay).await;
            Ok(())
        }
        fn shutdown_priority(&self) -> u32 {
            self.priority
        }
    }
    #[tokio::test]
    async fn test_shutdown_coordinator() {
        let coordinator = ShutdownCoordinator::new();
        // Register test components
        let component1 = Arc::new(TestComponent {
            name: "Network".to_string(),
            priority: 50,
            shutdown_delay: Duration::from_millis(10),
        });
        let component2 = Arc::new(TestComponent {
            name: "Database".to_string(),
            priority: 120,
            shutdown_delay: Duration::from_millis(10),
        });
        coordinator.register_component(component1).await;
        coordinator.register_component(component2).await;
        // Subscribe to events
        let mut events = coordinator.subscribe_to_events();
        // Initiate shutdown
        let result = coordinator
            .initiate_shutdown("Test shutdown".to_string())
            .await;
        assert!(result.is_ok());
        // Verify shutdown completed
        assert!(coordinator.is_shutting_down().await);
        // Check that we received events
        let mut event_count = 0;
        while let Ok(event) = events.try_recv() {
            event_count += 1;
            log::info!("Received event: {event:?}");
        }
        assert!(event_count > 0);
    }
    #[tokio::test]
    async fn test_shutdown_signal() {
        let coordinator = ShutdownCoordinator::new();
        let shutdown_signal = coordinator.get_shutdown_signal();
        let signal_clone = Arc::clone(&shutdown_signal);
        let wait_task = tokio::spawn(async move {
            signal_clone.notified().await;
            true
        });
        // Initiate shutdown
        let _ = coordinator.initiate_shutdown("Test".to_string()).await;
        // Verify signal was received
        let result = wait_task.await.expect("operation should succeed");
        assert!(result);
    }
}

//! Performance monitoring and profiling for Neo node
//!
//! Provides detailed performance tracking and bottleneck detection.

use crate::error_handling::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Performance sample for a specific metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    /// Timestamp of the sample
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    /// Value of the metric
    pub value: f64,
    /// Optional metadata
    pub metadata: Option<HashMap<String, String>>,
}

/// Performance metric with historical data
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    /// Metric name
    _name: String,
    /// Historical samples (limited to max_samples)
    samples: VecDeque<PerformanceSample>,
    /// Maximum number of samples to keep
    max_samples: usize,
    /// Running statistics
    stats: MetricStatistics,
}

/// Statistics for a performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatistics {
    /// Current value
    pub current: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Average value
    pub avg: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// 50th percentile
    pub p50: f64,
    /// 90th percentile
    pub p90: f64,
    /// 99th percentile
    pub p99: f64,
    /// Sample count
    pub count: usize,
}

impl Default for MetricStatistics {
    fn default() -> Self {
        Self {
            current: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            avg: 0.0,
            std_dev: 0.0,
            p50: 0.0,
            p90: 0.0,
            p99: 0.0,
            count: 0,
        }
    }
}

impl PerformanceMetric {
    /// Create new performance metric
    pub fn new(name: String, max_samples: usize) -> Self {
        Self {
            _name: name,
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
            stats: MetricStatistics::default(),
        }
    }

    /// Add a sample to the metric
    pub fn add_sample(&mut self, value: f64, metadata: Option<HashMap<String, String>>) {
        let sample = PerformanceSample {
            timestamp: Utc::now(),
            value,
            metadata,
        };

        // Add sample and maintain max size
        self.samples.push_back(sample);
        if self.samples.len() > self.max_samples {
            self.samples.pop_front();
        }

        // Update statistics
        self.update_statistics();
    }

    /// Update statistics based on current samples
    fn update_statistics(&mut self) {
        if self.samples.is_empty() {
            return;
        }

        let values: Vec<f64> = self.samples.iter().map(|s| s.value).collect();
        let count = values.len();

        self.stats.current = values.last().copied().unwrap_or(0.0);
        self.stats.min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        self.stats.max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        self.stats.avg = values.iter().sum::<f64>() / count as f64;
        self.stats.count = count;

        // Calculate standard deviation
        let variance = values
            .iter()
            .map(|&v| (v - self.stats.avg).powi(2))
            .sum::<f64>()
            / count as f64;
        self.stats.std_dev = variance.sqrt();

        // Calculate percentiles
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.stats.p50 = self.percentile(&sorted, 0.50);
        self.stats.p90 = self.percentile(&sorted, 0.90);
        self.stats.p99 = self.percentile(&sorted, 0.99);
    }

    /// Calculate percentile from sorted values
    fn percentile(&self, sorted: &[f64], p: f64) -> f64 {
        if sorted.is_empty() {
            return 0.0;
        }

        let index = ((sorted.len() - 1) as f64 * p) as usize;
        sorted[index]
    }

    /// Get current statistics
    pub fn get_stats(&self) -> &MetricStatistics {
        &self.stats
    }

    /// Get recent samples
    pub fn get_samples(&self, count: usize) -> Vec<PerformanceSample> {
        self.samples.iter().rev().take(count).cloned().collect()
    }
}

/// Performance monitor for tracking various metrics
pub struct PerformanceMonitor {
    /// Registered metrics
    metrics: Arc<RwLock<HashMap<String, PerformanceMetric>>>,
    /// Performance thresholds
    thresholds: Arc<RwLock<HashMap<String, PerformanceThreshold>>>,
    /// Alert callbacks
    alert_callbacks: Arc<RwLock<Vec<Box<dyn Fn(PerformanceAlert) + Send + Sync>>>>,
}

/// Performance threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThreshold {
    /// Metric name
    pub metric: String,
    /// Warning threshold
    pub warning: f64,
    /// Critical threshold
    pub critical: f64,
    /// Threshold type (min or max)
    pub threshold_type: ThresholdType,
}

/// Threshold type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThresholdType {
    /// Value should not exceed threshold
    Max,
    /// Value should not fall below threshold
    Min,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Metric name
    pub metric: String,
    /// Alert level
    pub level: AlertLevel,
    /// Current value
    pub value: f64,
    /// Threshold value
    pub threshold: f64,
    /// Alert message
    pub message: String,
    /// Timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

/// Alert level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    /// Warning level alert
    Warning,
    /// Critical level alert
    Critical,
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            thresholds: Arc::new(RwLock::new(HashMap::new())),
            alert_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a new metric
    pub async fn register_metric(&self, name: String, max_samples: usize) {
        let mut metrics = self.metrics.write().await;
        metrics.insert(name.clone(), PerformanceMetric::new(name, max_samples));
    }

    /// Record a metric value
    pub async fn record(&self, metric: &str, value: f64) -> Result<()> {
        self.record_with_metadata(metric, value, None).await
    }

    /// Record a metric value with metadata
    pub async fn record_with_metadata(
        &self,
        metric: &str,
        value: f64,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<()> {
        let mut metrics = self.metrics.write().await;

        if let Some(m) = metrics.get_mut(metric) {
            m.add_sample(value, metadata);

            // Check thresholds
            let thresholds = self.thresholds.read().await;
            if let Some(threshold) = thresholds.get(metric) {
                if let Some(alert) = self.check_threshold(metric, value, threshold) {
                    self.trigger_alert(alert).await;
                }
            }
        }

        Ok(())
    }

    /// Set threshold for a metric
    pub async fn set_threshold(&self, threshold: PerformanceThreshold) {
        let mut thresholds = self.thresholds.write().await;
        thresholds.insert(threshold.metric.clone(), threshold);
    }

    /// Register alert callback
    pub async fn register_alert_callback<F>(&self, callback: F)
    where
        F: Fn(PerformanceAlert) + Send + Sync + 'static,
    {
        let mut callbacks = self.alert_callbacks.write().await;
        callbacks.push(Box::new(callback));
    }

    /// Check if value violates threshold
    fn check_threshold(
        &self,
        metric: &str,
        value: f64,
        threshold: &PerformanceThreshold,
    ) -> Option<PerformanceAlert> {
        let (level, threshold_value) = match threshold.threshold_type {
            ThresholdType::Max => {
                if value > threshold.critical {
                    (AlertLevel::Critical, threshold.critical)
                } else if value > threshold.warning {
                    (AlertLevel::Warning, threshold.warning)
                } else {
                    return None;
                }
            }
            ThresholdType::Min => {
                if value < threshold.critical {
                    (AlertLevel::Critical, threshold.critical)
                } else if value < threshold.warning {
                    (AlertLevel::Warning, threshold.warning)
                } else {
                    return None;
                }
            }
        };

        Some(PerformanceAlert {
            metric: metric.to_string(),
            level,
            value,
            threshold: threshold_value,
            message: format!(
                "Metric '{}' value {} exceeds {:?} threshold {}",
                metric, value, level, threshold_value
            ),
            timestamp: Utc::now(),
        })
    }

    /// Trigger alert callbacks
    async fn trigger_alert(&self, alert: PerformanceAlert) {
        let callbacks = self.alert_callbacks.read().await;
        for callback in callbacks.iter() {
            callback(alert.clone());
        }
    }

    /// Get statistics for a metric
    pub async fn get_stats(&self, metric: &str) -> Option<MetricStatistics> {
        let metrics = self.metrics.read().await;
        metrics.get(metric).map(|m| m.get_stats().clone())
    }

    /// Get all metrics statistics
    pub async fn get_all_stats(&self) -> HashMap<String, MetricStatistics> {
        let metrics = self.metrics.read().await;
        metrics
            .iter()
            .map(|(name, metric)| (name.clone(), metric.get_stats().clone()))
            .collect()
    }

    /// Get recent samples for a metric
    pub async fn get_samples(&self, metric: &str, count: usize) -> Vec<PerformanceSample> {
        let metrics = self.metrics.read().await;
        metrics
            .get(metric)
            .map(|m| m.get_samples(count))
            .unwrap_or_default()
    }
}

/// Performance profiler for measuring execution time
pub struct Profiler {
    /// Start time
    start: Instant,
    /// Operation name
    operation: String,
    /// Performance monitor
    monitor: Option<Arc<PerformanceMonitor>>,
}

impl Profiler {
    /// Start profiling an operation
    pub fn start(operation: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.into(),
            monitor: None,
        }
    }

    /// Start profiling with monitor
    pub fn start_with_monitor(
        operation: impl Into<String>,
        monitor: Arc<PerformanceMonitor>,
    ) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.into(),
            monitor: Some(monitor),
        }
    }

    /// Stop profiling and get duration
    pub fn stop(self) -> Duration {
        self.start.elapsed()
    }

    /// Stop profiling and record to monitor
    pub async fn stop_and_record(self) {
        let duration = self.start.elapsed();

        if let Some(monitor) = self.monitor {
            let _ = monitor
                .record(&self.operation, duration.as_secs_f64())
                .await;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metric() {
        let mut metric = PerformanceMetric::new("test".to_string(), 100);

        // Add samples
        for i in 0..10 {
            metric.add_sample(i as f64, None);
        }

        let stats = metric.get_stats();
        assert_eq!(stats.count, 10);
        assert_eq!(stats.min, 0.0);
        assert_eq!(stats.max, 9.0);
        assert_eq!(stats.avg, 4.5);
    }

    #[tokio::test]
    async fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();

        // Register metric
        monitor
            .register_metric("test_metric".to_string(), 100)
            .await;

        // Record values
        for i in 0..5 {
            monitor.record("test_metric", i as f64).await.unwrap();
        }

        // Get statistics
        let stats = monitor.get_stats("test_metric").await.unwrap();
        assert_eq!(stats.count, 5);
        assert_eq!(stats.avg, 2.0);
    }

    #[tokio::test]
    async fn test_threshold_alerts() {
        let monitor = Arc::new(PerformanceMonitor::new());

        // Register metric
        monitor.register_metric("cpu".to_string(), 100).await;

        // Set threshold
        let threshold = PerformanceThreshold {
            metric: "cpu".to_string(),
            warning: 70.0,
            critical: 90.0,
            threshold_type: ThresholdType::Max,
        };
        monitor.set_threshold(threshold).await;

        // Register alert callback
        let alert_triggered = Arc::new(RwLock::new(false));
        let alert_flag = alert_triggered.clone();

        monitor
            .register_alert_callback(move |_alert| {
                let flag = alert_flag.clone();
                tokio::spawn(async move {
                    let mut triggered = flag.write().await;
                    *triggered = true;
                });
            })
            .await;

        // Trigger alert
        monitor.record("cpu", 95.0).await.unwrap();

        // Wait a bit for async callback
        tokio::time::sleep(Duration::from_millis(100)).await;

        let triggered = alert_triggered.read().await;
        assert!(*triggered);
    }
}

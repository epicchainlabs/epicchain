//! Advanced alerting system for Neo-RS monitoring
//!
//! Provides configurable alerts, thresholds, and notification channels.

use crate::error_handling::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Represents an enumeration of values.
pub enum AlertLevel {
    /// Informational alert
    Info,
    /// Warning level alert
    Warning,
    /// Critical level alert requiring immediate attention
    Critical,
    /// Emergency level alert indicating system failure
    Emergency,
}

impl AlertLevel {
    /// Get numeric priority (higher = more urgent)
    pub fn priority(&self) -> u8 {
        match self {
            AlertLevel::Info => 1,
            AlertLevel::Warning => 2,
            AlertLevel::Critical => 3,
            AlertLevel::Emergency => 4,
        }
    }
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct AlertThreshold {
    /// Metric name to monitor
    pub metric: String,
    /// Warning threshold value
    pub warning: f64,
    /// Critical threshold value
    pub critical: f64,
    /// Emergency threshold value
    pub emergency: f64,
    /// Comparison operator
    pub operator: ThresholdOperator,
    /// Time window for evaluation (seconds)
    pub window_seconds: u64,
    /// Minimum occurrences before triggering
    pub min_occurrences: u32,
}

/// Threshold comparison operators
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an enumeration of values.
pub enum ThresholdOperator {
    /// Greater than threshold
    GreaterThan,
    /// Less than threshold
    LessThan,
    /// Equal to threshold
    Equal,
    /// Not equal to threshold
    NotEqual,
}

impl ThresholdOperator {
    /// Evaluate value against threshold
    pub fn evaluate(&self, value: f64, threshold: f64) -> bool {
        match self {
            ThresholdOperator::GreaterThan => value > threshold,
            ThresholdOperator::LessThan => value < threshold,
            ThresholdOperator::Equal => (value - threshold).abs() < f64::EPSILON,
            ThresholdOperator::NotEqual => (value - threshold).abs() >= f64::EPSILON,
        }
    }
}

/// Alert instance
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct Alert {
    /// Unique alert ID
    pub id: String,
    /// Alert level
    pub level: AlertLevel,
    /// Metric that triggered the alert
    pub metric: String,
    /// Current value
    pub value: f64,
    /// Threshold that was exceeded
    pub threshold: f64,
    /// Alert message
    pub message: String,
    /// Timestamp when alert was triggered
    pub timestamp: SystemTime,
    /// Additional context
    pub context: HashMap<String, String>,
    /// Whether alert is acknowledged
    pub acknowledged: bool,
    /// Resolution timestamp
    pub resolved_at: Option<SystemTime>,
}

impl Alert {
    /// Create new alert
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(
        level: AlertLevel,
        metric: String,
        value: f64,
        threshold: f64,
        message: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            level,
            metric,
            value,
            threshold,
            message,
            timestamp: SystemTime::now(),
            context: HashMap::new(),
            acknowledged: false,
            resolved_at: None,
        }
    }

    /// Add context to alert
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }

    /// Acknowledge alert
    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }

    /// Resolve alert
    pub fn resolve(&mut self) {
        self.resolved_at = Some(SystemTime::now());
    }

    /// Check if alert is active
    /// Checks a boolean condition.
    pub fn is_active(&self) -> bool {
        self.resolved_at.is_none()
    }

    /// Get alert age in seconds
    pub fn age_seconds(&self) -> u64 {
        SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or(Duration::ZERO)
            .as_secs()
    }
}

/// Alert rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a data structure.
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Threshold configuration
    pub threshold: AlertThreshold,
    /// Whether rule is enabled
    pub enabled: bool,
    /// Notification channels
    pub channels: Vec<String>,
    /// Cooldown period in seconds
    pub cooldown_seconds: u64,
}

/// Notification channel trait
#[async_trait::async_trait]
/// Defines a trait interface.
pub trait NotificationChannel: Send + Sync {
    /// Send alert notification
    async fn send(&self, alert: &Alert) -> Result<()>;

    /// Get channel name
    fn name(&self) -> &str;
}

/// Log-based notification channel
/// Represents a data structure.
pub struct LogChannel {
    name: String,
}

impl LogChannel {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait::async_trait]
impl NotificationChannel for LogChannel {
    async fn send(&self, alert: &Alert) -> Result<()> {
        match alert.level {
            AlertLevel::Info => info!(
                alert_id = %alert.id,
                metric = %alert.metric,
                value = alert.value,
                "{}",
                alert.message
            ),
            AlertLevel::Warning => warn!(
                alert_id = %alert.id,
                metric = %alert.metric,
                value = alert.value,
                threshold = alert.threshold,
                "{}",
                alert.message
            ),
            AlertLevel::Critical | AlertLevel::Emergency => error!(
                alert_id = %alert.id,
                metric = %alert.metric,
                value = alert.value,
                threshold = alert.threshold,
                "{}",
                alert.message
            ),
        }
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Webhook notification channel
/// Represents a data structure.
pub struct WebhookChannel {
    name: String,
    url: String,
    client: reqwest::Client,
}

impl WebhookChannel {
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl NotificationChannel for WebhookChannel {
    async fn send(&self, alert: &Alert) -> Result<()> {
        let payload = serde_json::json!({
            "alert": alert,
            "timestamp": alert.timestamp,
            "level": alert.level,
            "metric": alert.metric,
            "value": alert.value,
            "threshold": alert.threshold,
            "message": alert.message
        });

        let response = self
            .client
            .post(&self.url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                crate::error_handling::NeoError::Network(
                    crate::error_handling::NetworkError::ConnectionFailed(e.to_string()),
                )
            })?;

        if !response.status().is_success() {
            return Err(crate::error_handling::NeoError::Network(
                crate::error_handling::NetworkError::ProtocolViolation(format!(
                    "Webhook returned status: {}",
                    response.status()
                )),
            ));
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Alert manager for handling alerts and notifications
/// Represents a data structure.
pub struct AlertManager {
    /// Alert rules
    rules: Arc<RwLock<HashMap<String, AlertRule>>>,
    /// Active alerts
    alerts: Arc<RwLock<HashMap<String, Alert>>>,
    /// Notification channels
    channels: Arc<RwLock<HashMap<String, Arc<dyn NotificationChannel>>>>,
    /// Alert cooldowns
    cooldowns: Arc<RwLock<HashMap<String, SystemTime>>>,
}

impl AlertManager {
    /// Create new alert manager
    /// Creates a new instance.
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
            alerts: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
            cooldowns: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add alert rule
    pub async fn add_rule(&self, rule: AlertRule) {
        let mut rules = self.rules.write().await;
        rules.insert(rule.name.clone(), rule);
    }

    /// Remove alert rule
    pub async fn remove_rule(&self, name: &str) {
        let mut rules = self.rules.write().await;
        rules.remove(name);
    }

    /// Add notification channel
    pub async fn add_channel(&self, channel: Arc<dyn NotificationChannel>) {
        let mut channels = self.channels.write().await;
        channels.insert(channel.name().to_string(), channel);
    }

    /// Evaluate metric value against all rules
    pub async fn evaluate(&self, metric: &str, value: f64) -> Result<()> {
        let rules = self.rules.read().await;

        for rule in rules.values() {
            if rule.threshold.metric == metric && rule.enabled {
                self.evaluate_rule(rule, value).await?;
            }
        }

        Ok(())
    }

    /// Evaluate single rule
    async fn evaluate_rule(&self, rule: &AlertRule, value: f64) -> Result<()> {
        // Check cooldown
        if self.is_in_cooldown(&rule.name).await {
            return Ok(());
        }

        let threshold = &rule.threshold;
        let mut alert_level = None;

        // Check emergency threshold
        if threshold.operator.evaluate(value, threshold.emergency) {
            alert_level = Some(AlertLevel::Emergency);
        }
        // Check critical threshold
        else if threshold.operator.evaluate(value, threshold.critical) {
            alert_level = Some(AlertLevel::Critical);
        }
        // Check warning threshold
        else if threshold.operator.evaluate(value, threshold.warning) {
            alert_level = Some(AlertLevel::Warning);
        }

        if let Some(level) = alert_level {
            let threshold_value = match level {
                AlertLevel::Emergency => threshold.emergency,
                AlertLevel::Critical => threshold.critical,
                AlertLevel::Warning => threshold.warning,
                AlertLevel::Info => threshold.warning,
            };

            let alert = Alert::new(
                level,
                threshold.metric.clone(),
                value,
                threshold_value,
                format!(
                    "Metric '{}' {} {} (current: {})",
                    threshold.metric,
                    match threshold.operator {
                        ThresholdOperator::GreaterThan => "exceeded threshold",
                        ThresholdOperator::LessThan => "below threshold",
                        ThresholdOperator::Equal => "equals threshold",
                        ThresholdOperator::NotEqual => "not equal to threshold",
                    },
                    threshold_value,
                    value
                ),
            )
            .with_context("rule".to_string(), rule.name.clone())
            .with_context("operator".to_string(), format!("{:?}", threshold.operator));

            self.trigger_alert(alert, rule).await?;
        }

        Ok(())
    }

    /// Trigger alert and send notifications
    async fn trigger_alert(&self, alert: Alert, rule: &AlertRule) -> Result<()> {
        // Store alert
        {
            let mut alerts = self.alerts.write().await;
            alerts.insert(alert.id.clone(), alert.clone());
        }

        // Set cooldown
        {
            let mut cooldowns = self.cooldowns.write().await;
            cooldowns.insert(
                rule.name.clone(),
                SystemTime::now() + Duration::from_secs(rule.cooldown_seconds),
            );
        }

        // Send notifications
        let channels = self.channels.read().await;
        for channel_name in &rule.channels {
            if let Some(channel) = channels.get(channel_name) {
                if let Err(e) = channel.send(&alert).await {
                    error!(
                        channel = channel_name,
                        alert_id = %alert.id,
                        error = %e,
                        "Failed to send alert notification"
                    );
                }
            }
        }

        info!(
            alert_id = %alert.id,
            level = ?alert.level,
            metric = %alert.metric,
            value = alert.value,
            "Alert triggered"
        );

        Ok(())
    }

    /// Check if rule is in cooldown
    async fn is_in_cooldown(&self, rule_name: &str) -> bool {
        let cooldowns = self.cooldowns.read().await;
        if let Some(&cooldown_until) = cooldowns.get(rule_name) {
            SystemTime::now() < cooldown_until
        } else {
            false
        }
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.read().await;
        alerts.values().filter(|a| a.is_active()).cloned().collect()
    }

    /// Acknowledge alert
    pub async fn acknowledge_alert(&self, alert_id: &str) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        if let Some(alert) = alerts.get_mut(alert_id) {
            alert.acknowledge();
            info!(alert_id = alert_id, "Alert acknowledged");
        }
        Ok(())
    }

    /// Resolve alert
    pub async fn resolve_alert(&self, alert_id: &str) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        if let Some(alert) = alerts.get_mut(alert_id) {
            alert.resolve();
            info!(alert_id = alert_id, "Alert resolved");
        }
        Ok(())
    }

    /// Get alert statistics
    pub async fn get_stats(&self) -> AlertStats {
        let alerts = self.alerts.read().await;
        let total = alerts.len();
        let active = alerts.values().filter(|a| a.is_active()).count();
        let acknowledged = alerts.values().filter(|a| a.acknowledged).count();

        let mut by_level = HashMap::new();
        for alert in alerts.values().filter(|a| a.is_active()) {
            *by_level.entry(alert.level).or_insert(0) += 1;
        }

        AlertStats {
            total_alerts: total,
            active_alerts: active,
            acknowledged_alerts: acknowledged,
            alerts_by_level: by_level,
        }
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Alert statistics
#[derive(Debug, Serialize, Deserialize)]
/// Represents a data structure.
pub struct AlertStats {
    /// Total number of alerts
    pub total_alerts: usize,
    /// Number of active alerts
    pub active_alerts: usize,
    /// Number of acknowledged alerts
    pub acknowledged_alerts: usize,
    /// Alerts grouped by level
    pub alerts_by_level: HashMap<AlertLevel, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alert_creation() {
        let alert = Alert::new(
            AlertLevel::Warning,
            "cpu_usage".to_string(),
            85.0,
            80.0,
            "CPU usage is high".to_string(),
        );

        assert_eq!(alert.level, AlertLevel::Warning);
        assert_eq!(alert.metric, "cpu_usage");
        assert_eq!(alert.value, 85.0);
        assert_eq!(alert.threshold, 80.0);
        assert!(alert.is_active());
        assert!(!alert.acknowledged);
    }

    #[tokio::test]
    async fn test_threshold_evaluation() {
        let operator = ThresholdOperator::GreaterThan;
        assert!(operator.evaluate(85.0, 80.0));
        assert!(!operator.evaluate(75.0, 80.0));
    }

    #[tokio::test]
    async fn test_alert_manager() {
        let manager = AlertManager::new();

        // Add log channel
        let log_channel = Arc::new(LogChannel::new("log".to_string()));
        manager.add_channel(log_channel).await;

        // Add rule
        let rule = AlertRule {
            name: "cpu_high".to_string(),
            description: "CPU usage too high".to_string(),
            threshold: AlertThreshold {
                metric: "cpu_usage".to_string(),
                warning: 70.0,
                critical: 85.0,
                emergency: 95.0,
                operator: ThresholdOperator::GreaterThan,
                window_seconds: 60,
                min_occurrences: 1,
            },
            enabled: true,
            channels: vec!["log".to_string()],
            cooldown_seconds: 300,
        };

        manager.add_rule(rule).await;

        // Test evaluation
        manager.evaluate("cpu_usage", 80.0).await.unwrap();

        let active_alerts = manager.get_active_alerts().await;
        assert_eq!(active_alerts.len(), 1);
        assert_eq!(active_alerts[0].level, AlertLevel::Warning);
    }
}

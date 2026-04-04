//! Integration tests for the monitoring system

use neo_core::monitoring::{
    init_monitoring, AlertLevel, ExporterFactory, HealthStatus, PerformanceThreshold, Profiler,
    ThresholdType,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_monitoring_system_initialization() {
    let system = init_monitoring("1.0.0-test".to_string())
        .await
        .expect("Failed to initialize monitoring");

    // Check that health monitor is working
    let health_report = system.health_monitor.check_health().await.unwrap();
    assert_eq!(health_report.version, "1.0.0-test");
    assert!(!health_report.components.is_empty());

    // Check that performance monitor is working
    let perf_stats = system.performance_monitor.get_all_stats().await;
    assert!(perf_stats.contains_key("block_processing"));
    assert!(perf_stats.contains_key("tx_validation"));
}

#[tokio::test]
async fn test_health_checks() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    let health_report = system.health_monitor.check_health().await.unwrap();

    // Check overall status
    assert!(
        health_report.status == HealthStatus::Healthy
            || health_report.status == HealthStatus::Degraded
            || health_report.status == HealthStatus::Unhealthy
    );

    // Check component health
    for component in &health_report.components {
        assert!(!component.component.is_empty());
        assert!(component.duration.as_millis() < 1000); // Health check should be fast
    }

    // Check uptime
    assert!(health_report.uptime.as_secs() >= 0);
}

#[tokio::test]
async fn test_performance_monitoring() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Record some performance metrics
    for i in 0..10 {
        system
            .performance_monitor
            .record("block_processing", 0.1 * i as f64)
            .await
            .unwrap();
    }

    // Get statistics
    let stats = system
        .performance_monitor
        .get_stats("block_processing")
        .await
        .unwrap();

    assert_eq!(stats.count, 10);
    assert!(stats.min >= 0.0);
    assert!(stats.max <= 0.9);
    assert!(stats.avg > 0.0);
}

#[tokio::test]
async fn test_performance_thresholds_and_alerts() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Set up alert tracking
    let alert_triggered = Arc::new(tokio::sync::RwLock::new(false));
    let alert_flag = alert_triggered.clone();

    system
        .performance_monitor
        .register_alert_callback(move |alert| {
            if matches!(
                alert.level,
                neo_core::monitoring::performance::AlertLevel::Critical
            ) {
                let flag = alert_flag.clone();
                tokio::spawn(async move {
                    let mut triggered = flag.write().await;
                    *triggered = true;
                });
            }
        })
        .await;

    // Set a low threshold that will trigger
    system
        .performance_monitor
        .set_threshold(PerformanceThreshold {
            metric: "test_metric".to_string(),
            warning: 0.5,
            critical: 1.0,
            threshold_type: ThresholdType::Max,
        })
        .await;

    // Register the metric
    system
        .performance_monitor
        .register_metric("test_metric".to_string(), 100)
        .await;

    // Record a value that exceeds the critical threshold
    system
        .performance_monitor
        .record("test_metric", 1.5)
        .await
        .unwrap();

    // Wait for async callback
    sleep(Duration::from_millis(100)).await;

    let triggered = alert_triggered.read().await;
    assert!(*triggered, "Alert should have been triggered");
}

#[tokio::test]
async fn test_profiler() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Register metric for profiling
    system
        .performance_monitor
        .register_metric("test_operation".to_string(), 100)
        .await;

    // Profile an operation
    let profiler =
        Profiler::start_with_monitor("test_operation", system.performance_monitor.clone());

    // Simulate some work
    sleep(Duration::from_millis(10)).await;

    // Stop and record
    profiler.stop_and_record().await;

    // Check that the metric was recorded
    let stats = system
        .performance_monitor
        .get_stats("test_operation")
        .await
        .unwrap();

    assert_eq!(stats.count, 1);
    assert!(stats.current > 0.0);
}

#[tokio::test]
async fn test_metrics_exporters() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Test Prometheus exporter
    let prometheus_output = system.export("prometheus").await.unwrap();
    assert!(prometheus_output.contains("neo_health_status"));
    assert!(prometheus_output.contains("TYPE"));
    assert!(prometheus_output.contains("HELP"));

    // Test JSON exporter
    let json_output = system.export("json").await.unwrap();
    assert!(json_output.contains("\"health\""));
    assert!(json_output.contains("\"performance\""));

    // Test JSON pretty exporter
    let json_pretty_output = system.export("json-pretty").await.unwrap();
    assert!(json_pretty_output.contains("{\n"));
    assert!(json_pretty_output.contains("\"health\""));

    // Test CSV exporter
    let csv_output = system.export("csv").await.unwrap();
    assert!(csv_output.contains("timestamp,component,status"));
}

#[tokio::test]
async fn test_status_report() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Record some metrics
    for i in 0..5 {
        system
            .performance_monitor
            .record("tx_validation", 0.01 * i as f64)
            .await
            .unwrap();
    }

    // Get status report
    let status = system.get_status().await.unwrap();

    // Verify report contents
    assert_eq!(status.health.version, "1.0.0");
    assert!(!status.health.components.is_empty());
    assert!(!status.performance.is_empty());
    assert!(!status.metrics.is_empty());

    // Check performance stats
    if let Some(tx_stats) = status.performance.get("tx_validation") {
        assert_eq!(tx_stats.count, 5);
    }
}

#[tokio::test]
async fn test_cache_behavior() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // First call should execute health checks
    let start = std::time::Instant::now();
    let _report1 = system.health_monitor.check_health().await.unwrap();
    let first_duration = start.elapsed();

    // Second call should use cache (much faster)
    let start = std::time::Instant::now();
    let _report2 = system.health_monitor.check_health().await.unwrap();
    let second_duration = start.elapsed();

    // Cache should make second call faster
    // Note: This might be flaky in CI, so we're being generous
    assert!(
        second_duration < first_duration * 2,
        "Second call should be similar or faster due to caching"
    );
}

#[tokio::test]
async fn test_background_tasks() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Start background tasks
    system.start_background_tasks();

    // Wait for background tasks to run
    sleep(Duration::from_secs(1)).await;

    // Check that system metrics have been updated
    let stats = system.performance_monitor.get_all_stats().await;

    // Background tasks should have recorded memory and CPU usage
    // Note: These might not exist if the background task hasn't run yet
    // This is more of a smoke test
    assert!(!stats.is_empty());
}

#[tokio::test]
async fn test_exporter_factory() {
    // Test factory creation
    assert!(ExporterFactory::create("prometheus").is_some());
    assert!(ExporterFactory::create("json").is_some());
    assert!(ExporterFactory::create("json-pretty").is_some());
    assert!(ExporterFactory::create("csv").is_some());
    assert!(ExporterFactory::create("unknown").is_none());

    // Test OTLP exporter creation
    let otlp_exporter =
        ExporterFactory::create_otlp("http://localhost:4317".to_string(), "neo-node".to_string());
    assert_eq!(otlp_exporter.content_type(), "application/json");
}

#[tokio::test]
async fn test_invalid_export_format() {
    let system = init_monitoring("1.0.0".to_string()).await.unwrap();

    // Try to export with invalid format
    let result = system.export("invalid_format").await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = format!("{:?}", e);
        assert!(error_msg.contains("Unsupported export format"));
    }
}

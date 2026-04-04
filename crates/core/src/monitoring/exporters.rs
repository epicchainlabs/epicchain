//! Metrics exporters for various monitoring systems
//!
//! Supports Prometheus, JSON, and custom export formats.

use crate::error_handling::Result;
use crate::monitoring::{HealthReport, MetricStatistics};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// Status report combining health, performance, and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReport {
    /// Health report
    pub health: HealthReport,
    /// Performance statistics
    pub performance: HashMap<String, MetricStatistics>,
    /// Raw metrics in Prometheus format
    pub metrics: String,
}

/// Trait for exporting metrics
pub trait MetricsExporter: Send + Sync {
    /// Export metrics in the target format
    fn export(&self, report: &StatusReport) -> Result<String>;

    /// Get the content type for HTTP responses
    fn content_type(&self) -> &str;
}

/// Prometheus exporter
pub struct PrometheusExporter;

impl MetricsExporter for PrometheusExporter {
    fn export(&self, report: &StatusReport) -> Result<String> {
        let mut output = String::new();

        // Add raw Prometheus metrics
        output.push_str(&report.metrics);

        // Add health metrics
        writeln!(
            &mut output,
            "# HELP neo_health_status Overall health status (0=unknown, 1=healthy, 2=degraded, 3=unhealthy)"
        )?;
        writeln!(&mut output, "# TYPE neo_health_status gauge")?;

        let health_value = match report.health.status {
            crate::monitoring::HealthStatus::Unknown => 0,
            crate::monitoring::HealthStatus::Healthy => 1,
            crate::monitoring::HealthStatus::Degraded => 2,
            crate::monitoring::HealthStatus::Unhealthy => 3,
        };
        writeln!(&mut output, "neo_health_status {}", health_value)?;

        // Add component health metrics
        for component in &report.health.components {
            let component_value = match component.status {
                crate::monitoring::HealthStatus::Unknown => 0,
                crate::monitoring::HealthStatus::Healthy => 1,
                crate::monitoring::HealthStatus::Degraded => 2,
                crate::monitoring::HealthStatus::Unhealthy => 3,
            };

            writeln!(
                &mut output,
                "neo_component_health{{component=\"{}\"}} {}",
                component.component, component_value
            )?;
        }

        // Add performance metrics
        for (metric_name, stats) in &report.performance {
            writeln!(
                &mut output,
                "# HELP neo_perf_{} Performance metric for {}",
                metric_name, metric_name
            )?;
            writeln!(&mut output, "# TYPE neo_perf_{} summary", metric_name)?;

            writeln!(
                &mut output,
                "neo_perf_{}_current {}",
                metric_name, stats.current
            )?;
            writeln!(&mut output, "neo_perf_{}_min {}", metric_name, stats.min)?;
            writeln!(&mut output, "neo_perf_{}_max {}", metric_name, stats.max)?;
            writeln!(&mut output, "neo_perf_{}_avg {}", metric_name, stats.avg)?;
            writeln!(
                &mut output,
                "neo_perf_{}_count {}",
                metric_name, stats.count
            )?;

            // Add percentiles
            writeln!(
                &mut output,
                "neo_perf_{}{{quantile=\"0.5\"}} {}",
                metric_name, stats.p50
            )?;
            writeln!(
                &mut output,
                "neo_perf_{}{{quantile=\"0.9\"}} {}",
                metric_name, stats.p90
            )?;
            writeln!(
                &mut output,
                "neo_perf_{}{{quantile=\"0.99\"}} {}",
                metric_name, stats.p99
            )?;
        }

        Ok(output)
    }

    fn content_type(&self) -> &str {
        "text/plain; version=0.0.4"
    }
}

/// JSON exporter
pub struct JsonExporter {
    /// Pretty print JSON
    pretty: bool,
}

impl JsonExporter {
    /// Create new JSON exporter
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl MetricsExporter for JsonExporter {
    fn export(&self, report: &StatusReport) -> Result<String> {
        let json = if self.pretty {
            serde_json::to_string_pretty(report)?
        } else {
            serde_json::to_string(report)?
        };

        Ok(json)
    }

    fn content_type(&self) -> &str {
        "application/json"
    }
}

/// OpenTelemetry exporter
pub struct OpenTelemetryExporter {
    /// Endpoint URL
    _endpoint: String,
    /// Service name
    service_name: String,
}

impl OpenTelemetryExporter {
    /// Create new OpenTelemetry exporter
    pub fn new(endpoint: String, service_name: String) -> Self {
        Self {
            _endpoint: endpoint,
            service_name,
        }
    }
}

impl MetricsExporter for OpenTelemetryExporter {
    fn export(&self, report: &StatusReport) -> Result<String> {
        // Create OTLP format
        let otlp = OtlpMetrics {
            resource: Resource {
                attributes: vec![
                    Attribute {
                        key: "service.name".to_string(),
                        value: self.service_name.clone(),
                    },
                    Attribute {
                        key: "service.version".to_string(),
                        value: report.health.version.clone(),
                    },
                ],
            },
            metrics: self.convert_to_otlp_metrics(report),
        };

        Ok(serde_json::to_string(&otlp)?)
    }

    fn content_type(&self) -> &str {
        "application/json"
    }
}

impl OpenTelemetryExporter {
    fn convert_to_otlp_metrics(&self, report: &StatusReport) -> Vec<OtlpMetric> {
        let mut metrics = Vec::new();

        // Convert health metrics
        metrics.push(OtlpMetric {
            name: "health.status".to_string(),
            description: "Overall health status".to_string(),
            unit: "".to_string(),
            data: MetricData::Gauge {
                value: match report.health.status {
                    crate::monitoring::HealthStatus::Unknown => 0.0,
                    crate::monitoring::HealthStatus::Healthy => 1.0,
                    crate::monitoring::HealthStatus::Degraded => 2.0,
                    crate::monitoring::HealthStatus::Unhealthy => 3.0,
                },
            },
        });

        // Convert performance metrics
        for (name, stats) in &report.performance {
            metrics.push(OtlpMetric {
                name: format!("performance.{}", name),
                description: format!("Performance metric for {}", name),
                unit: "ms".to_string(),
                data: MetricData::Summary {
                    count: stats.count as u64,
                    sum: stats.avg * stats.count as f64,
                    quantiles: vec![
                        Quantile {
                            quantile: 0.5,
                            value: stats.p50,
                        },
                        Quantile {
                            quantile: 0.9,
                            value: stats.p90,
                        },
                        Quantile {
                            quantile: 0.99,
                            value: stats.p99,
                        },
                    ],
                },
            });
        }

        metrics
    }
}

/// OTLP metrics format
#[derive(Debug, Serialize, Deserialize)]
struct OtlpMetrics {
    resource: Resource,
    metrics: Vec<OtlpMetric>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attribute {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OtlpMetric {
    name: String,
    description: String,
    unit: String,
    data: MetricData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum MetricData {
    Gauge {
        value: f64,
    },
    Counter {
        value: f64,
    },
    Summary {
        count: u64,
        sum: f64,
        quantiles: Vec<Quantile>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Quantile {
    quantile: f64,
    value: f64,
}

/// CSV exporter for historical data
pub struct CsvExporter;

impl MetricsExporter for CsvExporter {
    fn export(&self, report: &StatusReport) -> Result<String> {
        let mut output = String::new();

        // Header
        writeln!(
            &mut output,
            "timestamp,component,status,metric,value,min,max,avg,p50,p90,p99"
        )?;

        let timestamp = chrono::Utc::now().to_rfc3339();

        // Health data
        for component in &report.health.components {
            writeln!(
                &mut output,
                "{},{},{:?},,,,,,,,",
                timestamp, component.component, component.status
            )?;
        }

        // Performance data
        for (metric_name, stats) in &report.performance {
            writeln!(
                &mut output,
                "{},performance,Healthy,{},{},{},{},{},{},{},{}",
                timestamp,
                metric_name,
                stats.current,
                stats.min,
                stats.max,
                stats.avg,
                stats.p50,
                stats.p90,
                stats.p99
            )?;
        }

        Ok(output)
    }

    fn content_type(&self) -> &str {
        "text/csv"
    }
}

/// Exporter factory
pub struct ExporterFactory;

impl ExporterFactory {
    /// Create exporter by format
    pub fn create(format: &str) -> Option<Box<dyn MetricsExporter>> {
        match format.to_lowercase().as_str() {
            "prometheus" | "prom" => Some(Box::new(PrometheusExporter)),
            "json" => Some(Box::new(JsonExporter::new(false))),
            "json-pretty" => Some(Box::new(JsonExporter::new(true))),
            "csv" => Some(Box::new(CsvExporter)),
            _ => None,
        }
    }

    /// Create OpenTelemetry exporter
    pub fn create_otlp(endpoint: String, service_name: String) -> Box<dyn MetricsExporter> {
        Box::new(OpenTelemetryExporter::new(endpoint, service_name))
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::monitoring::{HealthCheckResult, HealthStatus};
    use chrono::Utc;
    use std::time::Duration;

    fn create_test_report() -> StatusReport {
        let health = HealthReport {
            status: HealthStatus::Healthy,
            components: vec![HealthCheckResult {
                component: "test".to_string(),
                status: HealthStatus::Healthy,
                message: Some("Test component".to_string()),
                details: HashMap::new(),
                timestamp: Utc::now(),
                duration: Duration::from_millis(10),
            }],
            uptime: Duration::from_secs(3600),
            timestamp: Utc::now(),
            version: "1.0.0".to_string(),
        };

        let mut performance = HashMap::new();
        performance.insert(
            "test_metric".to_string(),
            MetricStatistics {
                current: 10.0,
                min: 1.0,
                max: 20.0,
                avg: 10.5,
                std_dev: 3.2,
                p50: 10.0,
                p90: 18.0,
                p99: 19.5,
                count: 100,
            },
        );

        StatusReport {
            health,
            performance,
            metrics: "# Test metrics\n".to_string(),
        }
    }

    #[test]
    fn test_prometheus_exporter() {
        let exporter = PrometheusExporter;
        let report = create_test_report();

        let output = exporter.export(&report).unwrap();
        assert!(output.contains("neo_health_status"));
        assert!(output.contains("neo_component_health"));
        assert!(output.contains("neo_perf_test_metric"));
    }

    #[test]
    fn test_json_exporter() {
        let exporter = JsonExporter::new(false);
        let report = create_test_report();

        let output = exporter.export(&report).unwrap();
        assert!(output.contains("\"health\""));
        assert!(output.contains("\"performance\""));

        // Verify valid JSON
        let _: StatusReport = serde_json::from_str(&output).unwrap();
    }

    #[test]
    fn test_csv_exporter() {
        let exporter = CsvExporter;
        let report = create_test_report();

        let output = exporter.export(&report).unwrap();
        assert!(output.contains("timestamp,component,status"));
        assert!(output.contains("test_metric"));
    }

    #[test]
    fn test_exporter_factory() {
        assert!(ExporterFactory::create("prometheus").is_some());
        assert!(ExporterFactory::create("json").is_some());
        assert!(ExporterFactory::create("csv").is_some());
        assert!(ExporterFactory::create("unknown").is_none());
    }
}

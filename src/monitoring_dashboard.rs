//! Monitoring Dashboard for Neo-RS
//!
//! This module provides a web-based dashboard for monitoring blockchain metrics
//! in real-time using the system monitoring infrastructure.

use neo_core::system_monitoring::{SYSTEM_MONITOR, SystemMetricsSnapshot};
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

/// Dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    /// Port to run the dashboard on
    pub port: u16,
    /// Update interval in seconds
    pub update_interval: u64,
    /// Number of historical data points to keep
    pub history_size: usize,
    /// Enable debug logging
    pub debug: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            update_interval: 1,
            history_size: 60,
            debug: false,
        }
    }
}

/// Dashboard server
pub struct MonitoringDashboard {
    config: DashboardConfig,
    metrics_history: Arc<std::sync::RwLock<MetricsHistory>>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl MonitoringDashboard {
    /// Create a new monitoring dashboard
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            config: config.clone(),
            metrics_history: Arc::new(std::sync::RwLock::new(
                MetricsHistory::new(config.history_size)
            )),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Start the dashboard server
    pub fn start(&self) -> Result<(), DashboardError> {
        if self.running.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(DashboardError::AlreadyRunning);
        }
        
        self.running.store(true, std::sync::atomic::Ordering::Relaxed);
        
        // Start metrics collection thread
        self.start_metrics_collector();
        
        // Start web server
        self.start_web_server()?;
        
        Ok(())
    }
    
    /// Stop the dashboard server
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
    }
    
    /// Start the metrics collection thread
    fn start_metrics_collector(&self) {
        let history = Arc::clone(&self.metrics_history);
        let interval = self.config.update_interval;
        let running = Arc::clone(&self.running);
        
        thread::spawn(move || {
            while running.load(std::sync::atomic::Ordering::Relaxed) {
                // Collect current metrics
                let snapshot = SYSTEM_MONITOR.snapshot();
                
                // Add to history
                if let Ok(mut hist) = history.write() {
                    hist.add_snapshot(snapshot);
                }
                
                // Sleep until next collection
                thread::sleep(Duration::from_secs(interval));
            }
        });
    }
    
    /// Start the web server
    fn start_web_server(&self) -> Result<(), DashboardError> {
        use std::net::{TcpListener, TcpStream};
        use std::io::{Read, Write};
        
        let port = self.config.port;
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .map_err(|e| DashboardError::ServerError(format!("Failed to bind to port {}: {}", port, e)))?;
        
        let metrics_history = Arc::clone(&self.metrics_history);
        let running = Arc::clone(&self.running);
        
        std::thread::spawn(move || {
            while running.load(std::sync::atomic::Ordering::Relaxed) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        if let Err(e) = Self::handle_client(&mut stream, &metrics_history) {
                            eprintln!("Error handling client: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                        std::thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });
        
        println!("Monitoring dashboard started on http://127.0.0.1:{}", port);
        Ok(())
    }
    
    /// Handle individual client connections
    fn handle_client(
        stream: &mut TcpStream,
        metrics_history: &Arc<std::sync::RwLock<MetricsHistory>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        let response = if request.starts_with("GET /api/metrics") {
            let snapshot = SYSTEM_MONITOR.snapshot();
            let json = serde_json::to_string(&snapshot)?;
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                json.len(),
                json
            )
        } else if request.starts_with("GET /api/history") {
            let history_json = if let Ok(history) = metrics_history.read() {
                serde_json::to_string(&*history)?
            } else {
                "{}".to_string()
            };
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                history_json.len(),
                history_json
            )
        } else if request.starts_with("GET /") {
            let html = Self::generate_static_html();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                html.len(),
                html
            )
        } else {
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\n\r\n404 Not Found".to_string()
        };
        
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
    
    /// Generate static HTML (refactored from generate_html)
    fn generate_static_html() -> String {
        include_str!("monitoring_dashboard_template.html").to_string()
    }
    
    /// Get current metrics as JSON
    pub fn get_metrics_json(&self) -> String {
        let snapshot = SYSTEM_MONITOR.snapshot();
        serde_json::to_string_pretty(&snapshot).unwrap_or_default()
    }
    
    /// Get metrics history as JSON
    pub fn get_history_json(&self) -> String {
        if let Ok(history) = self.metrics_history.read() {
            serde_json::to_string_pretty(&*history).unwrap_or_default()
        } else {
            "{}".to_string()
        }
    }
    
    /// Generate HTML dashboard (deprecated - use built-in server)
    pub fn generate_html(&self) -> String {
        Self::generate_static_html()
    }
}

/// Metrics history storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsHistory {
    max_size: usize,
    snapshots: VecDeque<SystemMetricsSnapshot>,
}

impl MetricsHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            snapshots: VecDeque::with_capacity(max_size),
        }
    }
    
    pub fn add_snapshot(&mut self, snapshot: SystemMetricsSnapshot) {
        if self.snapshots.len() >= self.max_size {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snapshot);
    }
    
    pub fn get_latest(&self) -> Option<&SystemMetricsSnapshot> {
        self.snapshots.back()
    }
    
    pub fn get_history(&self) -> &VecDeque<SystemMetricsSnapshot> {
        &self.snapshots
    }
}

/// Dashboard error types
#[derive(Debug)]
pub enum DashboardError {
    AlreadyRunning,
    ServerError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for DashboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyRunning => write!(f, "Dashboard is already running"),
            Self::ServerError(msg) => write!(f, "Server error: {}", msg),
            Self::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for DashboardError {}

impl From<std::io::Error> for DashboardError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
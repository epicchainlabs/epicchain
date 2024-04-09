use std::fs::OpenOptions;
use std::io::Write;
use chrono::{Local, DateTime};

/// Enum representing different log levels.
#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// Function to log a message with a specified log level to a file.
pub fn log_message(level: LogLevel, message: &str, file_path: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open log file");

    let timestamp = Local::now();
    let log_line = match level {
        LogLevel::Info => format!("[{}] [INFO] {}", timestamp, message),
        LogLevel::Warning => format!("[{}] [WARNING] {}", timestamp, message),
        LogLevel::Error => format!("[{}] [ERROR] {}", timestamp, message),
    };

    writeln!(file, "{}", log_line).expect("Failed to write to log file");
}

/// Function to log a message with a specified log level and custom format to a file.
pub fn log_message_custom(level: LogLevel, message: &str, format: &str, file_path: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open log file");

    let timestamp = Local::now();
    let log_line = match level {
        LogLevel::Info => format!(format, timestamp, "INFO", message),
        LogLevel::Warning => format!(format, timestamp, "WARNING", message),
        LogLevel::Error => format!(format, timestamp, "ERROR", message),
    };

    writeln!(file, "{}", log_line).expect("Failed to write to log file");
}

fn main() {
    let log_file_path = "log.txt";

    // Log messages with default format
    log_message(LogLevel::Info, "This is an informational message", log_file_path);
    log_message(LogLevel::Warning, "This is a warning message", log_file_path);
    log_message(LogLevel::Error, "This is an error message", log_file_path);

    // Log messages with custom format
    log_message_custom(LogLevel::Info, "Custom info message", "[{}] [{}] {}", log_file_path);
    log_message_custom(LogLevel::Warning, "Custom warning message", "[{}] [{}] {}", log_file_path);
    log_message_custom(LogLevel::Error, "Custom error message", "[{}] [{}] {}", log_file_path);
}

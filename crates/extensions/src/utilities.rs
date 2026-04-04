//! Utility functions for Neo Extensions

use crate::error::{ExtensionError, ExtensionResult};
use chrono::{DateTime, Utc};
#[cfg(test)]
#[allow(dead_code)]
use neo_core::constants::{
    MAX_BLOCK_SIZE, MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK, SECONDS_PER_BLOCK,
};

// Define constant locally
const SECONDS_PER_HOUR: u64 = 3600;
use std::time::{SystemTime, UNIX_EPOCH};
/// Convert timestamp to DateTime
pub fn timestamp_to_datetime(timestamp: u64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_else(|| Utc::now())
}

/// Convert DateTime to timestamp
pub fn datetime_to_timestamp(datetime: &DateTime<Utc>) -> u64 {
    datetime.timestamp() as u64
}

/// Get current timestamp
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Convert bytes to human readable size
pub fn bytes_to_human_readable(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Generate a random UUID
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Validate UUID format
pub fn is_valid_uuid(uuid_str: &str) -> bool {
    uuid::Uuid::parse_str(uuid_str).is_ok()
}

/// Calculate checksum of data
pub fn calculate_checksum(data: &[u8]) -> u32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish() as u32
}

/// Retry operation with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
    initial_delay_ms: u64,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut delay = initial_delay_ms;

    for attempt in 0..=max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_retries {
                    return Err(e);
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                delay *= 2; // Exponential backoff
            }
        }
    }

    // This should never be reached since the loop handles all cases,
    operation()
}

/// Clamp value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Safe division that returns 0 if divisor is 0
pub fn safe_divide(dividend: f64, divisor: f64) -> f64 {
    if divisor == 0.0 {
        0.0
    } else {
        dividend / divisor
    }
}

/// Format duration in human readable format
pub fn format_duration(duration_secs: u64) -> String {
    let days = duration_secs / 86400;
    let hours = (duration_secs % 86400) / 3600;
    let minutes = (duration_secs % 3600) / 60;
    let seconds = duration_secs % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Truncate string to specified length with ellipsis
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        "/* implementation */;".to_string()
    } else {
        format!("{}/* implementation */;", &s[..max_len - 3])
    }
}

/// Check if string is empty or whitespace only
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

/// Convert snake_case to camelCase
pub fn snake_to_camel_case(snake_str: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in snake_str.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap_or(c));
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert camelCase to snake_case
pub fn camel_to_snake_case(camel_str: &str) -> String {
    let mut result = String::new();

    for (i, c) in camel_str.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
    }

    result
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_conversion() {
        let now = current_timestamp();
        let datetime = timestamp_to_datetime(now);
        let back_to_timestamp = datetime_to_timestamp(&datetime);

        assert!((now as i64 - back_to_timestamp as i64).abs() <= 1);
    }

    #[test]
    fn test_bytes_to_human_readable() {
        assert_eq!(bytes_to_human_readable(0), "0 B");
        assert_eq!(
            bytes_to_human_readable(MAX_TRANSACTIONS_PER_BLOCK.try_into().unwrap()),
            "512 B"
        );
        assert_eq!(
            bytes_to_human_readable(MAX_SCRIPT_SIZE.try_into().unwrap()),
            "64.00 KB"
        );
        assert_eq!(bytes_to_human_readable(1536), "1.50 KB");
        assert_eq!(
            bytes_to_human_readable(MAX_BLOCK_SIZE.try_into().unwrap()),
            "1.00 MB"
        );
    }

    #[test]
    fn test_uuid_operations() {
        let uuid = generate_uuid();
        assert!(is_valid_uuid(&uuid));
        assert!(!is_valid_uuid("invalid-uuid"));
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 1, 10), 5);
        assert_eq!(clamp(0, 1, 10), 1);
        assert_eq!(clamp(SECONDS_PER_BLOCK, 1, 10), 10);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), 5.0);
        assert_eq!(safe_divide(10.0, 0.0), 0.0);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
        assert_eq!(format_duration(90061), "1d 1h 1m 1s");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(
            truncate_string("hello world", 8),
            "hello/* implementation */;"
        );
        assert_eq!(truncate_string("hi", 2), "hi");
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(snake_to_camel_case("hello_world"), "helloWorld");
        assert_eq!(camel_to_snake_case("helloWorld"), "hello_world");
        assert_eq!(snake_to_camel_case("test"), "test");
        assert_eq!(camel_to_snake_case("Test"), "test");
    }

    #[test]
    fn test_empty_or_whitespace() {
        assert!(is_empty_or_whitespace(""));
        assert!(is_empty_or_whitespace("   "));
        assert!(is_empty_or_whitespace("\t\n"));
        assert!(!is_empty_or_whitespace("hello"));
        assert!(!is_empty_or_whitespace(" hello "));
    }

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let mut attempts = 0;
        let result = retry_with_backoff(
            || {
                attempts += 1;
                if attempts < 3 {
                    Err("failed")
                } else {
                    Ok("success")
                }
            },
            5,
            1,
        )
        .await;

        assert_eq!(result, Ok("success"));
        assert_eq!(attempts, 3);
    }
}

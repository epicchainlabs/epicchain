//! Extensions Utilities C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Extensions utility functions.

use neo_extensions::utilities::*;

#[cfg(test)]
#[allow(dead_code)]
mod utilities_tests {
    use super::*;

    /// Test utility functions
    #[test]
    fn test_utility_functions_compatibility() {
        // Test timestamp conversion
        let now = current_timestamp();
        let datetime = timestamp_to_datetime(now);
        let back_to_timestamp = datetime_to_timestamp(&datetime);

        // Should be equal or very close (within 1 second)
        assert!((back_to_timestamp as i64 - now as i64).abs() <= 1);

        // Test bytes to human readable
        assert_eq!(bytes_to_human_readable(0), "0 B");
        assert_eq!(bytes_to_human_readable(1023), "1023 B");
        assert_eq!(bytes_to_human_readable(1024), "1.00 KB");
        assert_eq!(bytes_to_human_readable(1536), "1.50 KB");
        assert_eq!(bytes_to_human_readable(1048576), "1.00 MB");
        assert_eq!(bytes_to_human_readable(1073741824), "1.00 GB");
    }

    /// Test time utilities
    #[test]
    fn test_time_utilities_compatibility() {
        let timestamp = 1234567890u64;
        let datetime = timestamp_to_datetime(timestamp);
        let back_to_timestamp = datetime_to_timestamp(&datetime);
        assert_eq!(back_to_timestamp, timestamp);

        // Test clamp function
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);

        // Test safe divide
        assert_eq!(safe_divide(10.0, 2.0), 5.0);
        assert_eq!(safe_divide(10.0, 0.0), 0.0);
    }
}

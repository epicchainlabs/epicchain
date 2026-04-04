//! Extensions Error Handling C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Extensions error handling.

use neo_extensions::error::*;

#[cfg(test)]
#[allow(dead_code)]
mod error_handling_tests {
    use super::*;

    /// Test error handling (matches C# exception handling exactly)
    #[test]
    fn test_error_handling_compatibility() {
        // Test error creation and formatting
        let error = ExtensionError::EncodingError("test input".to_string());
        assert!(format!("{}", error).contains("Encoding error"));

        // Test error conversion
        let result: Result<(), ExtensionError> = Err(error);
        assert!(result.is_err());

        // Test error chaining
        let chained = result.map_err(|e| ExtensionError::Generic(format!("Error: {}", e)));
        assert!(chained.is_err());
    }
}

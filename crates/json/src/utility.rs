/// Utility functions for JSON processing
/// This matches the C# Neo.Json.Utility class
///
/// Strict UTF-8 encoding/decoding utilities
pub struct StrictUtf8;

impl StrictUtf8 {
    /// Converts a string to UTF-8 bytes
    pub fn get_bytes(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    /// Converts UTF-8 bytes to a string
    /// Returns an error if the bytes are not valid UTF-8
    pub fn get_string(bytes: &[u8]) -> Result<String, std::str::Utf8Error> {
        std::str::from_utf8(bytes).map(|s| s.to_string())
    }

    /// Converts UTF-8 bytes to a string, replacing invalid sequences
    pub fn get_string_lossy(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).to_string()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_utf8_roundtrip() {
        let original = "Hello, 世界! 🌍";
        let bytes = StrictUtf8::get_bytes(original);
        let recovered = StrictUtf8::get_string(&bytes).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn test_strict_utf8_invalid() {
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        assert!(StrictUtf8::get_string(&invalid_bytes).is_err());

        let lossy = StrictUtf8::get_string_lossy(&invalid_bytes);
        assert!(!lossy.is_empty()); // Should contain replacement characters
    }
}

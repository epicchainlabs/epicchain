use crate::error::{JsonError, JsonResult};
use crate::JToken;
use std::fmt;

/// Represents a JSON string
/// This matches the C# JString class
#[derive(Debug, Clone, PartialEq)]
pub struct JString {
    value: String,
}

impl JString {
    /// Creates a new JSON string
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// Creates a new JSON string from a string slice
    pub fn from_string_slice(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    /// Gets the string value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Gets the string value as a String (for compatibility with C# JString.ToString())
    pub fn get_string(&self) -> String {
        self.value.clone()
    }

    /// Sets the string value
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /// Gets the length of the string
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Checks if the string is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Converts the JString to a JToken::String
    pub fn to_jtoken(self) -> JToken {
        JToken::String(self.value)
    }

    /// Creates a JString from a JToken::String
    pub fn from_jtoken(token: JToken) -> JsonResult<Self> {
        match token {
            JToken::String(value) => Ok(Self { value }),
            _ => Err(JsonError::InvalidCast("Token is not a string".to_string())),
        }
    }

    /// Convert JString to boolean (matches C# AsBoolean behavior)
    /// Returns false only for empty strings, true for all other strings
    pub fn as_boolean(&self) -> bool {
        !self.value.is_empty()
    }

    /// Convert JString to number (matches C# AsNumber behavior)
    /// Returns 0.0 for empty strings, parsed number or NaN for invalid
    pub fn as_number(&self) -> f64 {
        if self.value.is_empty() {
            return 0.0;
        }

        self.value.parse::<f64>().unwrap_or(f64::NAN)
    }

    /// Get enum from JString (matches C# GetEnum behavior)
    /// Throws error if conversion fails
    pub fn get_enum<T: std::str::FromStr>(&self) -> Result<T, String> {
        self.value
            .parse()
            .map_err(|_| "Invalid enum value".to_string())
    }

    /// Get enum from JString with default (matches C# AsEnum behavior)
    /// Returns default value if conversion fails
    pub fn as_enum<T: std::str::FromStr>(&self, default: T, _ignore_case: bool) -> T {
        self.value.parse().unwrap_or(default)
    }

    /// Write JSON representation (matches C# Write behavior)
    pub fn write<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "\"{}\"", self.value)
    }

    /// Clone the JString (matches C# Clone behavior)
    pub fn clone_jstring(&self) -> Self {
        self.clone()
    }
}

impl Default for JString {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl From<String> for JString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for JString {
    fn from(value: &str) -> Self {
        Self::from_string_slice(value)
    }
}

impl From<JString> for String {
    fn from(jstring: JString) -> Self {
        jstring.value
    }
}

impl AsRef<str> for JString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for JString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jstring_new() {
        let jstr = JString::new("test".to_string());
        assert_eq!(jstr.value(), "test");
        assert_eq!(jstr.len(), 4);
        assert!(!jstr.is_empty());
    }

    #[test]
    fn test_jstring_from_string_slice() {
        let jstr = JString::from_string_slice("hello");
        assert_eq!(jstr.value(), "hello");
        assert_eq!(jstr.to_string(), "hello".to_string());
    }

    #[test]
    fn test_jstring_empty() {
        let jstr = JString::default();
        assert!(jstr.is_empty());
        assert_eq!(jstr.len(), 0);
    }

    #[test]
    fn test_jstring_set_value() {
        let mut jstr = JString::new("initial".to_string());
        jstr.set_value("updated".to_string());
        assert_eq!(jstr.value(), "updated");
    }

    #[test]
    fn test_jstring_conversions() {
        let jstr = JString::from("test");
        let string: String = jstr.clone().into();
        assert_eq!(string, "test");

        let jstr2: JString = "hello".into();
        assert_eq!(jstr2.value(), "hello");
    }

    #[test]
    fn test_jstring_jtoken_conversion() {
        let jstr = JString::from("test");
        let token = jstr.clone().to_jtoken();

        match token {
            JToken::String(value) => assert_eq!(value, "test"),
            _ => {
                // Test expectation: this should never happen in valid scenarios
                assert!(
                    false,
                    "Expected JToken::String but got different token type"
                );
            }
        }

        let jstr2 = JString::from_jtoken(JToken::String("hello".to_string()))
            .expect("operation should succeed");
        assert_eq!(jstr2.value(), "hello");

        let result = JString::from_jtoken(JToken::Number(42.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_jstring_display() {
        let jstr = JString::from("test");
        assert_eq!(format!("{jstr}"), "test");
    }
}

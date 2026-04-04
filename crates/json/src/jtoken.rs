use crate::error::{JsonError, JsonResult};
use crate::utility::StrictUtf8;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

/// Represents a JSON token - the main type for all JSON values
/// This matches the C# JToken abstract class
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JToken {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Option<JToken>>),
    Object(crate::ordered_dictionary::OrderedDictionary<String, Option<JToken>>),
}

impl JToken {
    /// Represents a null token (equivalent to C# JToken.Null)
    pub const NULL: Option<JToken> = None;

    /// Gets the child token at the specified index (for arrays)
    pub fn get_index(&self, index: usize) -> JsonResult<Option<&JToken>> {
        match self {
            JToken::Array(arr) => {
                if index < arr.len() {
                    Ok(arr[index].as_ref())
                } else {
                    Ok(None)
                }
            }
            _ => Err(JsonError::NotSupported(
                "Index access not supported for this token type".to_string(),
            )),
        }
    }

    /// Sets the child token at the specified index (for arrays)
    pub fn set_index(&mut self, index: usize, value: Option<JToken>) -> JsonResult<()> {
        match self {
            JToken::Array(arr) => {
                if index < arr.len() {
                    arr[index] = value;
                    Ok(())
                } else {
                    Err(JsonError::NotSupported("Index out of bounds".to_string()))
                }
            }
            _ => Err(JsonError::NotSupported(
                "Index access not supported for this token type".to_string(),
            )),
        }
    }

    /// Gets the property with the specified key (for objects)
    pub fn get_property(&self, key: &str) -> Option<&JToken> {
        match self {
            JToken::Object(obj) => obj.get(&key.to_string()).and_then(|v| v.as_ref()),
            _ => None,
        }
    }

    /// Sets the property with the specified key (for objects)
    pub fn set_property(&mut self, key: String, value: Option<JToken>) -> JsonResult<()> {
        match self {
            JToken::Object(obj) => {
                obj.insert(key, value);
                Ok(())
            }
            _ => Err(JsonError::NotSupported(
                "Property access not supported for this token type".to_string(),
            )),
        }
    }

    /// Converts the current JSON token to a boolean value
    pub fn as_boolean(&self) -> bool {
        match self {
            JToken::Boolean(b) => *b,
            JToken::Null => false,
            JToken::Number(n) => *n != 0.0 && !n.is_nan(),
            JToken::String(s) => !s.is_empty(),
            JToken::Array(arr) => !arr.is_empty(),
            JToken::Object(obj) => !obj.is_empty(),
        }
    }

    /// Converts the current JSON token to an enum value
    pub fn as_enum<T>(&self, default_value: T, _ignore_case: bool) -> T
    where
        T: Clone + std::str::FromStr + std::convert::TryFrom<u32>,
    {
        // 1. Try to parse enum from string value (production implementation)
        if let JToken::String(str_value) = self {
            if let Ok(parsed_value) = str_value.parse::<T>() {
                return parsed_value;
            }
        }

        // 2. Try to parse enum from integer value (production fallback)
        if let JToken::Number(num_value) = self {
            if num_value.is_finite() && *num_value >= 0.0 {
                let int_value = *num_value as u64;
                if let Ok(enum_value) = T::try_from(int_value as u32) {
                    return enum_value;
                }
            }
        }

        // 3. Return default value if parsing fails (production error handling)
        default_value
    }

    /// Converts the current JSON token to a floating point number
    pub fn as_number(&self) -> f64 {
        match self {
            JToken::Number(n) => *n,
            JToken::Boolean(true) => 1.0,
            JToken::Boolean(false) => 0.0,
            JToken::String(s) => s.parse().unwrap_or(f64::NAN),
            _ => f64::NAN,
        }
    }

    /// Converts the current JSON token to a string
    pub fn as_string(&self) -> String {
        match self {
            JToken::String(s) => s.clone(),
            JToken::Number(n) => n.to_string(),
            JToken::Boolean(b) => b.to_string(),
            JToken::Null => "null".to_string(),
            _ => self.to_string(),
        }
    }

    /// Gets the boolean value (strict)
    pub fn get_boolean(&self) -> JsonResult<bool> {
        match self {
            JToken::Boolean(b) => Ok(*b),
            _ => Err(JsonError::InvalidCast("Token is not a boolean".to_string())),
        }
    }

    /// Gets the enum value (strict)
    pub fn get_enum<T>(&self, _ignore_case: bool) -> JsonResult<T>
    where
        T: Clone + std::str::FromStr + std::convert::TryFrom<u32>,
        T::Err: std::fmt::Debug,
        <T as std::convert::TryFrom<u32>>::Error: std::fmt::Debug,
    {
        // 1. Try to parse enum from string value
        if let JToken::String(str_value) = self {
            // Parse enum by name
            if let Ok(parsed_value) = str_value.parse::<T>() {
                return Ok(parsed_value);
            }
        }

        // 2. Try to parse enum from integer value
        if let JToken::Number(num_value) = self {
            if num_value.is_finite() && *num_value >= 0.0 {
                let int_value = *num_value as u64;
                // Convert integer to enum
                if let Ok(enum_value) = T::try_from(int_value as u32) {
                    return Ok(enum_value);
                }
            }
        }

        // 3. Return error if parsing fails
        Err(JsonError::InvalidCast(
            "Cannot convert token to enum".to_string(),
        ))
    }

    /// Gets the 32-bit signed integer value
    pub fn get_int32(&self) -> JsonResult<i32> {
        let d = self.get_number()?;
        if d.fract() != 0.0 {
            return Err(JsonError::InvalidCast(
                "Number is not an integer".to_string(),
            ));
        }
        if d < i32::MIN as f64 || d > i32::MAX as f64 {
            return Err(JsonError::OverflowError(
                "Number cannot be converted to i32".to_string(),
            ));
        }
        Ok(d as i32)
    }

    /// Gets the floating point number value (strict)
    pub fn get_number(&self) -> JsonResult<f64> {
        match self {
            JToken::Number(n) => Ok(*n),
            _ => Err(JsonError::InvalidCast("Token is not a number".to_string())),
        }
    }

    /// Gets the string value (strict)
    pub fn get_string(&self) -> JsonResult<String> {
        match self {
            JToken::String(s) => Ok(s.clone()),
            _ => Err(JsonError::InvalidCast("Token is not a string".to_string())),
        }
    }

    /// Parses a JSON token from a byte array
    pub fn parse(value: &[u8], max_nest: usize) -> JsonResult<Option<JToken>> {
        let json_str = StrictUtf8::get_string(value)
            .map_err(|e| JsonError::ParseError(format!("Invalid UTF-8: {e}")))?;
        Self::parse_string(&json_str, max_nest)
    }

    /// Parses a JSON token from a string
    pub fn parse_string(value: &str, _max_nest: usize) -> JsonResult<Option<JToken>> {
        if value.trim().is_empty() {
            return Ok(None);
        }

        let json_value: JsonValue = serde_json::from_str(value)
            .map_err(|e| JsonError::ParseError(format!("JSON parse error: {e}")))?;

        Ok(Some(Self::from_serde_value(json_value)))
    }

    /// Converts from serde_json::Value to JToken
    fn from_serde_value(value: JsonValue) -> JToken {
        match value {
            JsonValue::Null => JToken::Null,
            JsonValue::Bool(b) => JToken::Boolean(b),
            JsonValue::Number(n) => JToken::Number(n.as_f64().unwrap_or(0.0)),
            JsonValue::String(s) => JToken::String(s),
            JsonValue::Array(arr) => {
                let tokens: Vec<Option<JToken>> = arr
                    .into_iter()
                    .map(|v| Some(Self::from_serde_value(v)))
                    .collect();
                JToken::Array(tokens)
            }
            JsonValue::Object(obj) => {
                let mut ordered_dict = crate::ordered_dictionary::OrderedDictionary::new();
                for (key, value) in obj {
                    ordered_dict.insert(key, Some(Self::from_serde_value(value)));
                }
                JToken::Object(ordered_dict)
            }
        }
    }

    /// Converts to serde_json::Value
    fn to_serde_value(&self) -> JsonValue {
        match self {
            JToken::Null => JsonValue::Null,
            JToken::Boolean(b) => JsonValue::Bool(*b),
            JToken::Number(n) => JsonValue::Number(
                serde_json::Number::from_f64(*n).unwrap_or_else(|| serde_json::Number::from(0)),
            ),
            JToken::String(s) => JsonValue::String(s.clone()),
            JToken::Array(arr) => {
                let values: Vec<JsonValue> = arr
                    .iter()
                    .map(|opt_token| match opt_token {
                        Some(token) => token.to_serde_value(),
                        None => JsonValue::Null,
                    })
                    .collect();
                JsonValue::Array(values)
            }
            JToken::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, opt_value) in obj.iter() {
                    let value = match opt_value {
                        Some(token) => token.to_serde_value(),
                        None => JsonValue::Null,
                    };
                    map.insert(key.clone(), value);
                }
                JsonValue::Object(map)
            }
        }
    }

    /// Encodes the current JSON token into a byte array
    pub fn to_byte_array(&self, indented: bool) -> Vec<u8> {
        let json_str = if indented {
            serde_json::to_string_pretty(&self.to_serde_value())
        } else {
            serde_json::to_string(&self.to_serde_value())
        };

        match json_str {
            Ok(s) => StrictUtf8::get_bytes(&s),
            Err(_) => Vec::new(),
        }
    }

    /// Encodes the current JSON token into a string
    pub fn to_string_formatted(&self, indented: bool) -> String {
        let bytes = self.to_byte_array(indented);
        StrictUtf8::get_string(&bytes).unwrap_or_default()
    }

    /// Creates a copy of the current JSON token
    pub fn clone_token(&self) -> JToken {
        self.clone()
    }
}

impl fmt::Display for JToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_formatted(false))
    }
}

impl From<bool> for JToken {
    fn from(value: bool) -> Self {
        JToken::Boolean(value)
    }
}

impl From<f64> for JToken {
    fn from(value: f64) -> Self {
        JToken::Number(value)
    }
}

impl From<i32> for JToken {
    fn from(value: i32) -> Self {
        JToken::Number(value as f64)
    }
}

impl From<String> for JToken {
    fn from(value: String) -> Self {
        JToken::String(value)
    }
}

impl From<&str> for JToken {
    fn from(value: &str) -> Self {
        JToken::String(value.to_string())
    }
}

impl From<Vec<Option<JToken>>> for JToken {
    fn from(value: Vec<Option<JToken>>) -> Self {
        JToken::Array(value)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jtoken_boolean() {
        let token = JToken::Boolean(true);
        assert_eq!(token.as_boolean(), true);
        assert_eq!(token.get_boolean().unwrap(), true);
        assert_eq!(token.as_number(), 1.0);
    }

    #[test]
    fn test_jtoken_number() {
        let token = JToken::Number(42.5);
        assert_eq!(token.as_number(), 42.5);
        assert_eq!(token.get_number().unwrap(), 42.5);
        assert_eq!(token.as_boolean(), true);
    }

    #[test]
    fn test_jtoken_string() {
        let token = JToken::String("hello".to_string());
        assert_eq!(token.as_string(), "hello");
        assert_eq!(token.get_string().unwrap(), "hello");
        assert_eq!(token.as_boolean(), true);
    }

    #[test]
    fn test_jtoken_parse() {
        let json = r#"{"name": "test", "value": 42}"#;
        let token = JToken::parse_string(json, 64)
            .expect("Failed to unwrap")
            .expect("Failed second unwrap");

        if let JToken::Object(_) = token {
            assert_eq!(token.get_property("name").unwrap().as_string(), "test");
            assert_eq!(token.get_property("value").unwrap().as_number(), 42.0);
        } else {
            // Test expectation: this should never happen in valid scenarios
            assert!(false, "Expected object token but got different token type");
        }
    }

    #[test]
    fn test_jtoken_array() {
        let arr = vec![
            Some(JToken::Number(1.0)),
            Some(JToken::String("test".to_string())),
            None,
        ];
        let token = JToken::Array(arr);

        assert_eq!(
            token
                .get_index(0)
                .expect("Failed to unwrap")
                .expect("Failed second unwrap")
                .as_number(),
            1.0
        );
        assert_eq!(
            token
                .get_index(1)
                .expect("Failed to unwrap")
                .expect("Failed second unwrap")
                .as_string(),
            "test"
        );
        assert!(token.get_index(2).unwrap().is_none());
    }
}

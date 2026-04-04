use crate::error::{JsonError, JsonResult};
use crate::JToken;
use std::fmt;

/// Represents a JSON number
/// This matches the C# JNumber class
#[derive(Debug, Clone, PartialEq)]
pub struct JNumber {
    value: f64,
}

impl JNumber {
    /// Creates a new JSON number
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    /// Creates a new JSON number from an integer
    pub fn from_i32(value: i32) -> Self {
        Self {
            value: value as f64,
        }
    }

    /// Creates a new JSON number from a u32
    pub fn from_u32(value: u32) -> Self {
        Self {
            value: value as f64,
        }
    }

    /// Creates a new JSON number from an i64
    pub fn from_i64(value: i64) -> Self {
        Self {
            value: value as f64,
        }
    }

    /// Creates a new JSON number from a u64
    pub fn from_u64(value: u64) -> Self {
        Self {
            value: value as f64,
        }
    }

    /// Gets the numeric value as f64
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the numeric value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Converts to i32 if possible
    pub fn to_i32(&self) -> JsonResult<i32> {
        if self.value.fract() == 0.0
            && self.value >= i32::MIN as f64
            && self.value <= i32::MAX as f64
        {
            Ok(self.value as i32)
        } else {
            Err(JsonError::InvalidCast(format!(
                "Cannot convert {} to i32",
                self.value
            )))
        }
    }

    /// Converts to u32 if possible
    pub fn to_u32(&self) -> JsonResult<u32> {
        if self.value.fract() == 0.0 && self.value >= 0.0 && self.value <= u32::MAX as f64 {
            Ok(self.value as u32)
        } else {
            Err(JsonError::InvalidCast(format!(
                "Cannot convert {} to u32",
                self.value
            )))
        }
    }

    /// Converts to i64 if possible
    pub fn to_i64(&self) -> JsonResult<i64> {
        if self.value.fract() == 0.0
            && self.value >= i64::MIN as f64
            && self.value <= i64::MAX as f64
        {
            Ok(self.value as i64)
        } else {
            Err(JsonError::InvalidCast(format!(
                "Cannot convert {} to i64",
                self.value
            )))
        }
    }

    /// Converts to u64 if possible
    pub fn to_u64(&self) -> JsonResult<u64> {
        if self.value.fract() == 0.0 && self.value >= 0.0 && self.value <= u64::MAX as f64 {
            Ok(self.value as u64)
        } else {
            Err(JsonError::InvalidCast(format!(
                "Cannot convert {} to u64",
                self.value
            )))
        }
    }

    /// Checks if the number is an integer
    pub fn is_integer(&self) -> bool {
        self.value.fract() == 0.0
    }

    /// Checks if the number is finite
    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }

    /// Checks if the number is infinite
    pub fn is_infinite(&self) -> bool {
        self.value.is_infinite()
    }

    /// Checks if the number is NaN
    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }

    /// Converts the JNumber to a JToken::Number
    pub fn to_jtoken(self) -> JToken {
        JToken::Number(self.value)
    }

    /// Creates a JNumber from a JToken::Number
    pub fn from_jtoken(token: JToken) -> JsonResult<Self> {
        match token {
            JToken::Number(value) => Ok(Self { value }),
            _ => Err(JsonError::InvalidCast("Token is not a number".to_string())),
        }
    }
}

impl Default for JNumber {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl From<f64> for JNumber {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<f32> for JNumber {
    fn from(value: f32) -> Self {
        Self::new(value as f64)
    }
}

impl From<i32> for JNumber {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<u32> for JNumber {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<i64> for JNumber {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<u64> for JNumber {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<JNumber> for f64 {
    fn from(jnumber: JNumber) -> Self {
        jnumber.value
    }
}

impl fmt::Display for JNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jnumber_new() {
        let jnum = JNumber::new(42.5);
        assert_eq!(jnum.value(), 42.5);
        assert!(!jnum.is_integer());
        assert!(jnum.is_finite());
    }

    #[test]
    fn test_jnumber_integer() {
        let jnum = JNumber::from_i32(42);
        assert_eq!(jnum.value(), 42.0);
        assert!(jnum.is_integer());
        assert_eq!(jnum.to_i32().unwrap(), 42);
    }

    #[test]
    fn test_jnumber_conversions() {
        let jnum = JNumber::from(123);
        assert_eq!(jnum.to_i32().unwrap(), 123);
        assert_eq!(jnum.to_u32().unwrap(), 123);
        assert_eq!(jnum.to_i64().unwrap(), 123);
        assert_eq!(jnum.to_u64().unwrap(), 123);

        let float_num = JNumber::from(123.5);
        assert!(float_num.to_i32().is_err());
        assert!(float_num.to_u32().is_err());
    }

    #[test]
    fn test_jnumber_edge_cases() {
        let inf = JNumber::new(f64::INFINITY);
        assert!(inf.is_infinite());
        assert!(!inf.is_finite());

        let nan = JNumber::new(f64::NAN);
        assert!(nan.is_nan());
        assert!(!nan.is_finite());
    }

    #[test]
    fn test_jnumber_jtoken_conversion() {
        let jnum = JNumber::from(42.0);
        let token = jnum.clone().to_jtoken();

        match token {
            JToken::Number(value) => assert_eq!(value, 42.0),
            _ => {
                // Test expectation: this should never happen in valid scenarios
                assert!(
                    false,
                    "Expected JToken::Number but got different token type"
                );
            }
        }

        let jnum2 = JNumber::from_jtoken(JToken::Number(123.5)).expect("operation should succeed");
        assert_eq!(jnum2.value(), 123.5);

        let result = JNumber::from_jtoken(JToken::String("not a number".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_jnumber_display() {
        let jnum = JNumber::from(42.5);
        assert_eq!(format!("{jnum}"), "42.5");
    }

    #[test]
    fn test_jnumber_set_value() {
        let mut jnum = JNumber::new(10.0);
        jnum.set_value(20.5);
        assert_eq!(jnum.value(), 20.5);
    }
}

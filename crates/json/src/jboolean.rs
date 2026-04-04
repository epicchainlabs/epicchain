use crate::error::{JsonError, JsonResult};
use crate::JToken;
use std::fmt;

/// Represents a JSON boolean
/// This matches the C# JBoolean class
#[derive(Debug, Clone, PartialEq)]
pub struct JBoolean {
    value: bool,
}

impl JBoolean {
    /// Creates a new JSON boolean
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    /// Creates a JSON boolean representing true
    pub fn true_value() -> Self {
        Self { value: true }
    }

    /// Creates a JSON boolean representing false
    pub fn false_value() -> Self {
        Self { value: false }
    }

    /// Gets the boolean value
    pub fn value(&self) -> bool {
        self.value
    }

    /// Sets the boolean value
    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }

    /// Converts the JBoolean to a JToken::Boolean
    pub fn to_jtoken(self) -> JToken {
        JToken::Boolean(self.value)
    }

    /// Creates a JBoolean from a JToken::Boolean
    pub fn from_jtoken(token: JToken) -> JsonResult<Self> {
        match token {
            JToken::Boolean(value) => Ok(Self { value }),
            _ => Err(JsonError::InvalidCast("Token is not a boolean".to_string())),
        }
    }
}

impl Default for JBoolean {
    fn default() -> Self {
        Self::new(false)
    }
}

impl From<bool> for JBoolean {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<JBoolean> for bool {
    fn from(jboolean: JBoolean) -> Self {
        jboolean.value
    }
}

impl fmt::Display for JBoolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jboolean_new() {
        let jtrue = JBoolean::new(true);
        assert!(jtrue.value());

        let jfalse = JBoolean::new(false);
        assert!(!jfalse.value());
    }

    #[test]
    fn test_jboolean_constructors() {
        let jtrue = JBoolean::true_value();
        assert!(jtrue.value());

        let jfalse = JBoolean::false_value();
        assert!(!jfalse.value());
    }

    #[test]
    fn test_jboolean_default() {
        let jbool = JBoolean::default();
        assert!(!jbool.value());
    }

    #[test]
    fn test_jboolean_set_value() {
        let mut jbool = JBoolean::new(false);
        jbool.set_value(true);
        assert!(jbool.value());
    }

    #[test]
    fn test_jboolean_conversions() {
        let jbool = JBoolean::from(true);
        assert!(jbool.value());

        let bool_val: bool = jbool.into();
        assert!(bool_val);
    }

    #[test]
    fn test_jboolean_jtoken_conversion() {
        let jbool = JBoolean::from(true);
        let token = jbool.clone().to_jtoken();

        match token {
            JToken::Boolean(value) => assert!(value),
            _ => {
                // Test expectation: this should never happen in valid scenarios
                assert!(
                    false,
                    "Expected JToken::Boolean but got different token type"
                );
            }
        }

        let jbool2 =
            JBoolean::from_jtoken(JToken::Boolean(false)).expect("operation should succeed");
        assert!(!jbool2.value());

        let result = JBoolean::from_jtoken(JToken::String("not a boolean".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_jboolean_display() {
        let jtrue = JBoolean::from(true);
        assert_eq!(format!("{jtrue}"), "true");

        let jfalse = JBoolean::from(false);
        assert_eq!(format!("{jfalse}"), "false");
    }
}

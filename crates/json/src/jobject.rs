use crate::{JContainer, JToken, OrderedDictionary};

/// Represents a JSON object
/// This matches the C# JObject class
#[derive(Debug, Clone, PartialEq)]
pub struct JObject {
    properties: OrderedDictionary<String, Option<JToken>>,
}

impl JObject {
    /// Creates a new empty JSON object
    pub fn new() -> Self {
        Self {
            properties: OrderedDictionary::new(),
        }
    }

    /// Gets the properties of the JSON object
    pub fn properties(&self) -> &OrderedDictionary<String, Option<JToken>> {
        &self.properties
    }

    /// Gets a mutable reference to the properties
    pub fn properties_mut(&mut self) -> &mut OrderedDictionary<String, Option<JToken>> {
        &mut self.properties
    }

    /// Gets the property with the specified name
    pub fn get(&self, name: &str) -> Option<&JToken> {
        self.properties
            .get(&name.to_string())
            .and_then(|v| v.as_ref())
    }

    /// Sets the property with the specified name
    pub fn set(&mut self, name: String, value: Option<JToken>) {
        self.properties.insert(name, value);
    }

    /// Determines whether the JSON object contains a property with the specified name
    pub fn contains_property(&self, key: &str) -> bool {
        self.properties.contains_key(&key.to_string())
    }

    /// Clears all properties from the object
    pub fn clear(&mut self) {
        self.properties.clear();
    }
}

impl JContainer for JObject {
    fn clear_container(&mut self) {
        self.properties.clear();
    }

    fn children(&self) -> Vec<Option<&JToken>> {
        self.properties.values().map(|v| v.as_ref()).collect()
    }
}

impl Default for JObject {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_jobject_basic() {
        let mut obj = JObject::new();
        assert!(obj.properties().is_empty());

        obj.set(
            "test".to_string(),
            Some(JToken::String("value".to_string())),
        );
        assert_eq!(obj.properties().len(), 1);
        assert!(obj.contains_property("test"));

        let value = obj.get("test").unwrap();
        assert_eq!(value.as_string(), "value");
    }

    #[test]
    fn test_jobject_clear() {
        let mut obj = JObject::new();
        obj.set("test".to_string(), Some(JToken::Number(42.0)));
        assert_eq!(obj.properties().len(), 1);

        obj.clear();
        assert!(obj.properties().is_empty());
    }
}

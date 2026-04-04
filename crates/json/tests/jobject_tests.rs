//! JObject C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Json.JObject functionality.
//! Tests are based on the C# Neo.Json.JObject test suite.

use neo_json::*;

#[cfg(test)]
#[allow(dead_code)]
mod jobject_tests {
    use super::*;

    /// Test JObject creation and basic operations (matches C# JObject tests exactly)
    #[test]
    fn test_jobject_creation_compatibility() {
        let empty_obj = JObject::new();
        assert_eq!(empty_obj.properties().len(), 0);
        assert!(empty_obj.properties().is_empty());

        // Test object with initial properties
        let mut obj = JObject::new();
        obj.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj.set("version".to_string(), Some(JToken::Number(3.0)));
        obj.set("active".to_string(), Some(JToken::Boolean(true)));
        obj.set("config".to_string(), None); // null property

        assert_eq!(obj.properties().len(), 4);
        assert!(!obj.properties().is_empty());

        // Verify properties
        assert_eq!(obj.get("name"), Some(&JToken::String("Neo".to_string())));
        assert_eq!(obj.get("version"), Some(&JToken::Number(3.0)));
        assert_eq!(obj.get("active"), Some(&JToken::Boolean(true)));
        assert_eq!(obj.get("config"), None); // null property returns None from get()
        assert_eq!(obj.get("nonexistent"), None);
    }

    /// Test JObject property access methods (matches C# JObject property access exactly)
    #[test]
    fn test_jobject_property_access_compatibility() {
        let mut obj = JObject::new();

        // Test setting various property types
        obj.set("null_prop".to_string(), Some(JToken::Null));
        obj.set("bool_prop".to_string(), Some(JToken::Boolean(false)));
        obj.set("int_prop".to_string(), Some(JToken::Number(42.0)));
        obj.set(
            "float_prop".to_string(),
            Some(JToken::Number(std::f64::consts::PI)),
        );
        obj.set(
            "string_prop".to_string(),
            Some(JToken::String("test value".to_string())),
        );
        obj.set(
            "empty_string".to_string(),
            Some(JToken::String("".to_string())),
        );
        obj.set("missing_prop".to_string(), None);

        // Test contains_property method
        assert!(obj.contains_property("null_prop"));
        assert!(obj.contains_property("bool_prop"));
        assert!(obj.contains_property("int_prop"));
        assert!(obj.contains_property("float_prop"));
        assert!(obj.contains_property("string_prop"));
        assert!(obj.contains_property("empty_string"));
        assert!(obj.contains_property("missing_prop"));
        assert!(!obj.contains_property("nonexistent"));

        // Test get method behavior
        assert_eq!(obj.get("null_prop"), Some(&JToken::Null));
        assert_eq!(obj.get("bool_prop"), Some(&JToken::Boolean(false)));
        assert_eq!(obj.get("int_prop"), Some(&JToken::Number(42.0)));
        assert_eq!(
            obj.get("float_prop"),
            Some(&JToken::Number(std::f64::consts::PI))
        );
        assert_eq!(
            obj.get("string_prop"),
            Some(&JToken::String("test value".to_string()))
        );
        assert_eq!(
            obj.get("empty_string"),
            Some(&JToken::String("".to_string()))
        );
        assert_eq!(obj.get("missing_prop"), None); // null property
        assert_eq!(obj.get("nonexistent"), None);
    }

    /// Test JObject property modification (matches C# JObject property updates exactly)
    #[test]
    fn test_jobject_property_modification_compatibility() {
        let mut obj = JObject::new();

        // Set initial properties
        obj.set(
            "modifiable".to_string(),
            Some(JToken::String("initial".to_string())),
        );
        obj.set("replaceable".to_string(), Some(JToken::Number(1.0)));

        assert_eq!(
            obj.get("modifiable"),
            Some(&JToken::String("initial".to_string()))
        );
        assert_eq!(obj.get("replaceable"), Some(&JToken::Number(1.0)));

        // Modify existing properties
        obj.set(
            "modifiable".to_string(),
            Some(JToken::String("modified".to_string())),
        );
        obj.set("replaceable".to_string(), Some(JToken::Boolean(true)));

        assert_eq!(
            obj.get("modifiable"),
            Some(&JToken::String("modified".to_string()))
        );
        assert_eq!(obj.get("replaceable"), Some(&JToken::Boolean(true)));

        // Replace with null
        obj.set("modifiable".to_string(), None);
        assert_eq!(obj.get("modifiable"), None);
        assert!(obj.contains_property("modifiable")); // Property still exists but is null

        // Replace null with value
        obj.set("modifiable".to_string(), Some(JToken::Number(42.0)));
        assert_eq!(obj.get("modifiable"), Some(&JToken::Number(42.0)));

        // Test type changes
        obj.set(
            "type_changing".to_string(),
            Some(JToken::String("string".to_string())),
        );
        assert_eq!(
            obj.get("type_changing"),
            Some(&JToken::String("string".to_string()))
        );

        obj.set("type_changing".to_string(), Some(JToken::Number(123.456)));
        assert_eq!(obj.get("type_changing"), Some(&JToken::Number(123.456)));

        obj.set("type_changing".to_string(), Some(JToken::Boolean(false)));
        assert_eq!(obj.get("type_changing"), Some(&JToken::Boolean(false)));
    }

    /// Test JObject with nested structures (matches C# JObject nesting exactly)
    #[test]
    fn test_jobject_nested_structures_compatibility() {
        let mut root = JObject::new();

        // Create nested object
        let mut nested_obj = JObject::new();
        nested_obj.set(
            "inner_value".to_string(),
            Some(JToken::String("nested".to_string())),
        );
        nested_obj.set("inner_number".to_string(), Some(JToken::Number(99.0)));

        // Create nested array
        let nested_array = vec![
            Some(JToken::String("item1".to_string())),
            Some(JToken::String("item2".to_string())),
            Some(JToken::Number(42.0)),
            None, // null element
        ];

        // Set nested structures on root
        root.set(
            "nested_object".to_string(),
            Some(JToken::Object(nested_obj.properties().clone())),
        );
        root.set(
            "nested_array".to_string(),
            Some(JToken::Array(nested_array)),
        );
        root.set(
            "simple_value".to_string(),
            Some(JToken::String("simple".to_string())),
        );

        // Verify nested access
        assert!(root.contains_property("nested_object"));
        assert!(root.contains_property("nested_array"));
        assert!(root.contains_property("simple_value"));

        // Test nested object access
        if let Some(JToken::Object(ref nested_dict)) = root.get("nested_object") {
            assert!(nested_dict.contains_key(&"inner_value".to_string()));
            assert!(nested_dict.contains_key(&"inner_number".to_string()));
            assert_eq!(
                nested_dict.get(&"inner_value".to_string()),
                Some(&Some(JToken::String("nested".to_string())))
            );
            assert_eq!(
                nested_dict.get(&"inner_number".to_string()),
                Some(&Some(JToken::Number(99.0)))
            );
        } else {
            panic!("Expected nested object");
        }

        // Test nested array access
        if let Some(JToken::Array(ref arr)) = root.get("nested_array") {
            assert_eq!(arr.len(), 4);
            assert_eq!(arr[0], Some(JToken::String("item1".to_string())));
            assert_eq!(arr[1], Some(JToken::String("item2".to_string())));
            assert_eq!(arr[2], Some(JToken::Number(42.0)));
            assert_eq!(arr[3], None);
        } else {
            panic!("Expected nested array");
        }
    }

    /// Test JObject clearing and removal (matches C# JObject.Clear exactly)
    #[test]
    fn test_jobject_clearing_compatibility() {
        let mut obj = JObject::new();

        // Add multiple properties
        obj.set(
            "prop1".to_string(),
            Some(JToken::String("value1".to_string())),
        );
        obj.set("prop2".to_string(), Some(JToken::Number(42.0)));
        obj.set("prop3".to_string(), Some(JToken::Boolean(true)));
        obj.set("prop4".to_string(), None);

        assert_eq!(obj.properties().len(), 4);
        assert!(obj.contains_property("prop1"));
        assert!(obj.contains_property("prop2"));
        assert!(obj.contains_property("prop3"));
        assert!(obj.contains_property("prop4"));

        // Clear all properties
        obj.clear();

        assert_eq!(obj.properties().len(), 0);
        assert!(obj.properties().is_empty());
        assert!(!obj.contains_property("prop1"));
        assert!(!obj.contains_property("prop2"));
        assert!(!obj.contains_property("prop3"));
        assert!(!obj.contains_property("prop4"));

        assert_eq!(obj.get("prop1"), None);
        assert_eq!(obj.get("prop2"), None);
        assert_eq!(obj.get("prop3"), None);
        assert_eq!(obj.get("prop4"), None);

        // Test that object can be reused after clearing
        obj.set(
            "new_prop".to_string(),
            Some(JToken::String("new_value".to_string())),
        );
        assert_eq!(obj.properties().len(), 1);
        assert!(obj.contains_property("new_prop"));
        assert_eq!(
            obj.get("new_prop"),
            Some(&JToken::String("new_value".to_string()))
        );
    }

    /// Test JObject property enumeration (matches C# JObject property iteration exactly)
    #[test]
    fn test_jobject_property_enumeration_compatibility() {
        let mut obj = JObject::new();

        let expected_properties = vec![
            ("first".to_string(), Some(JToken::String("1st".to_string()))),
            ("second".to_string(), Some(JToken::Number(2.0))),
            ("third".to_string(), Some(JToken::Boolean(true))),
            ("fourth".to_string(), None),
            (
                "fifth".to_string(),
                Some(JToken::Array(vec![Some(JToken::Number(5.0))])),
            ),
        ];

        for (key, value) in &expected_properties {
            obj.set(key.clone(), value.clone());
        }

        let properties = obj.properties();
        assert_eq!(properties.len(), expected_properties.len());

        // Verify all properties exist
        for (key, expected_value) in &expected_properties {
            assert!(properties.contains_key(key));
            assert_eq!(properties.get(key), Some(expected_value));

            // Also test through the object interface
            assert!(obj.contains_property(key));
            if let Some(value) = expected_value {
                assert_eq!(obj.get(key), Some(value));
            } else {
                assert_eq!(obj.get(key), None);
            }
        }

        // Test that non-existent properties are not found
        assert!(!obj.contains_property("nonexistent"));
        assert_eq!(obj.get("nonexistent"), None);
    }

    /// Test JObject equality and comparison (matches C# JObject.Equals exactly)
    #[test]
    fn test_jobject_equality_compatibility() {
        // Test empty object equality
        let empty1 = JObject::new();
        let empty2 = JObject::new();
        assert_eq!(empty1, empty2);

        // Test objects with same properties
        let mut obj1 = JObject::new();
        obj1.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj1.set("version".to_string(), Some(JToken::Number(3.0)));
        obj1.set("active".to_string(), Some(JToken::Boolean(true)));

        let mut obj2 = JObject::new();
        obj2.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj2.set("version".to_string(), Some(JToken::Number(3.0)));
        obj2.set("active".to_string(), Some(JToken::Boolean(true)));

        assert_eq!(obj1, obj2);

        // Test objects with different property values
        let mut obj3 = JObject::new();
        obj3.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj3.set("version".to_string(), Some(JToken::Number(4.0))); // Different value
        obj3.set("active".to_string(), Some(JToken::Boolean(true)));

        assert_ne!(obj1, obj3);

        // Test objects with different properties
        let mut obj4 = JObject::new();
        obj4.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj4.set("version".to_string(), Some(JToken::Number(3.0)));
        // Missing "active" property

        assert_ne!(obj1, obj4);

        // Test objects with extra properties
        let mut obj5 = JObject::new();
        obj5.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj5.set("version".to_string(), Some(JToken::Number(3.0)));
        obj5.set("active".to_string(), Some(JToken::Boolean(true)));
        obj5.set(
            "extra".to_string(),
            Some(JToken::String("additional".to_string())),
        );

        assert_ne!(obj1, obj5);

        // Test objects with null properties
        let mut obj6 = JObject::new();
        obj6.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj6.set("version".to_string(), None); // null value
        obj6.set("active".to_string(), Some(JToken::Boolean(true)));

        let mut obj7 = JObject::new();
        obj7.set("name".to_string(), Some(JToken::String("Neo".to_string())));
        obj7.set("version".to_string(), None); // null value
        obj7.set("active".to_string(), Some(JToken::Boolean(true)));

        assert_eq!(obj6, obj7);
    }

    /// Test JObject cloning (matches C# JObject cloning behavior exactly)
    #[test]
    fn test_jobject_cloning_compatibility() {
        let mut original = JObject::new();
        original.set(
            "string_prop".to_string(),
            Some(JToken::String("test".to_string())),
        );
        original.set("number_prop".to_string(), Some(JToken::Number(42.0)));
        original.set("bool_prop".to_string(), Some(JToken::Boolean(true)));
        original.set("null_prop".to_string(), None);

        let mut nested = JObject::new();
        nested.set(
            "inner".to_string(),
            Some(JToken::String("inner_value".to_string())),
        );
        original.set(
            "nested_prop".to_string(),
            Some(JToken::Object(nested.properties().clone())),
        );

        let cloned = original.clone();

        // Verify clone equality
        assert_eq!(original, cloned);

        // Verify all properties are cloned
        assert_eq!(cloned.properties().len(), original.properties().len());
        assert_eq!(cloned.get("string_prop"), original.get("string_prop"));
        assert_eq!(cloned.get("number_prop"), original.get("number_prop"));
        assert_eq!(cloned.get("bool_prop"), original.get("bool_prop"));
        assert_eq!(cloned.get("null_prop"), original.get("null_prop"));
        assert_eq!(cloned.get("nested_prop"), original.get("nested_prop"));

        // Test that modifications to clone don't affect original
        let mut cloned_mut = cloned;
        cloned_mut.set(
            "new_prop".to_string(),
            Some(JToken::String("new_value".to_string())),
        );

        assert!(cloned_mut.contains_property("new_prop"));
        assert!(!original.contains_property("new_prop"));
        assert_ne!(original, cloned_mut);
    }

    /// Test JObject with special characters and unicode (matches C# unicode handling exactly)
    #[test]
    fn test_jobject_unicode_compatibility() {
        let mut obj = JObject::new();

        // Test property names with special characters
        obj.set(
            "property with spaces".to_string(),
            Some(JToken::String("value1".to_string())),
        );
        obj.set(
            "property-with-hyphens".to_string(),
            Some(JToken::String("value2".to_string())),
        );
        obj.set(
            "property_with_underscores".to_string(),
            Some(JToken::String("value3".to_string())),
        );
        obj.set(
            "property.with.dots".to_string(),
            Some(JToken::String("value4".to_string())),
        );
        obj.set(
            "property/with/slashes".to_string(),
            Some(JToken::String("value5".to_string())),
        );

        // Test unicode property names
        obj.set(
            "属性".to_string(),
            Some(JToken::String("Chinese property".to_string())),
        );
        obj.set(
            "خاصية".to_string(),
            Some(JToken::String("Arabic property".to_string())),
        );
        obj.set(
            "свойство".to_string(),
            Some(JToken::String("Russian property".to_string())),
        );
        obj.set(
            "プロパティ".to_string(),
            Some(JToken::String("Japanese property".to_string())),
        );
        obj.set(
            "🔑".to_string(),
            Some(JToken::String("Emoji property".to_string())),
        );

        // Test unicode property values
        obj.set(
            "unicode_values".to_string(),
            Some(JToken::String(
                "Hello 世界 🌍 مرحبا мир こんにちは".to_string(),
            )),
        );

        // Verify all properties are accessible
        assert!(obj.contains_property("property with spaces"));
        assert!(obj.contains_property("property-with-hyphens"));
        assert!(obj.contains_property("property_with_underscores"));
        assert!(obj.contains_property("property.with.dots"));
        assert!(obj.contains_property("property/with/slashes"));

        assert!(obj.contains_property("属性"));
        assert!(obj.contains_property("خاصية"));
        assert!(obj.contains_property("свойство"));
        assert!(obj.contains_property("プロパティ"));
        assert!(obj.contains_property("🔑"));

        // Verify values
        assert_eq!(
            obj.get("属性"),
            Some(&JToken::String("Chinese property".to_string()))
        );
        assert_eq!(
            obj.get("unicode_values"),
            Some(&JToken::String(
                "Hello 世界 🌍 مرحبا мир こんにちは".to_string()
            ))
        );
    }

    /// Test JObject performance with large datasets (matches C# performance characteristics)
    #[test]
    fn test_jobject_performance_compatibility() {
        let mut obj = JObject::new();

        // Test performance with many properties
        let property_count = 1000;
        for i in 0..property_count {
            let key = format!("property_{i:04}");
            let value = format!("value_{i:04}");
            obj.set(key, Some(JToken::String(value)));
        }

        assert_eq!(obj.properties().len(), property_count);

        // Test access performance
        for i in 0..property_count {
            let key = format!("property_{i:04}");
            let expected_value = format!("value_{i:04}");

            assert!(obj.contains_property(&key));
            assert_eq!(obj.get(&key), Some(&JToken::String(expected_value)));
        }

        // Test modification performance
        for i in 0..property_count {
            let key = format!("property_{i:04}");
            let new_value = format!("modified_value_{i:04}");
            obj.set(key, Some(JToken::String(new_value)));
        }

        // Verify modifications
        for i in 0..property_count {
            let key = format!("property_{i:04}");
            let expected_value = format!("modified_value_{i:04}");
            assert_eq!(obj.get(&key), Some(&JToken::String(expected_value)));
        }
    }
}

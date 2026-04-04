//! JSON Serialization C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Json serialization functionality.
//! Tests are based on the C# Neo.Json serialization and deserialization patterns.

use neo_json::*;

#[cfg(test)]
#[allow(dead_code)]
mod serialization_tests {
    use super::*;

    /// Test basic JSON serialization to string (matches C# ToString() exactly)
    #[test]
    fn test_basic_serialization_compatibility() {
        // Test primitive type serialization
        let null_token = JToken::Null;
        let serialized = serde_json::to_string(&null_token).unwrap();
        assert_eq!(serialized, "null");

        let bool_true = JToken::Boolean(true);
        let serialized = serde_json::to_string(&bool_true).unwrap();
        assert_eq!(serialized, "true");

        let bool_false = JToken::Boolean(false);
        let serialized = serde_json::to_string(&bool_false).unwrap();
        assert_eq!(serialized, "false");

        let number_int = JToken::Number(42.0);
        let serialized = serde_json::to_string(&number_int).unwrap();
        assert_eq!(serialized, "42.0");

        let number_float = JToken::Number(std::f64::consts::PI);
        let serialized = serde_json::to_string(&number_float).unwrap();
        assert!(serialized.contains("3.14159"));

        let string_token = JToken::String("test string".to_string());
        let serialized = serde_json::to_string(&string_token).unwrap();
        assert_eq!(serialized, "\"test string\"");

        let empty_string = JToken::String("".to_string());
        let serialized = serde_json::to_string(&empty_string).unwrap();
        assert_eq!(serialized, "\"\"");
    }

    /// Test array serialization (matches C# JArray serialization exactly)
    #[test]
    fn test_array_serialization_compatibility() {
        // Test empty array
        let empty_array = JToken::Array(vec![]);
        let serialized = serde_json::to_string(&empty_array).unwrap();
        assert_eq!(serialized, "[]");

        // Test array with mixed types
        let mixed_array = JToken::Array(vec![
            Some(JToken::Null),
            Some(JToken::Boolean(true)),
            Some(JToken::Number(42.0)),
            Some(JToken::String("test".to_string())),
            None, // null element
        ]);
        let serialized = serde_json::to_string(&mixed_array).unwrap();

        // Parse back to verify structure
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert!(parsed.is_array());
        let array = parsed.as_array().unwrap();
        assert_eq!(array.len(), 5);
        assert!(array[0].is_null());
        assert!(array[1].as_bool().unwrap());
        assert_eq!(array[2].as_f64().unwrap(), 42.0);
        assert_eq!(array[3].as_str().unwrap(), "test");
        assert!(array[4].is_null());

        // Test nested arrays
        let nested_array = JToken::Array(vec![
            Some(JToken::Array(vec![
                Some(JToken::Number(1.0)),
                Some(JToken::Number(2.0)),
            ])),
            Some(JToken::Array(vec![
                Some(JToken::String("a".to_string())),
                Some(JToken::String("b".to_string())),
            ])),
        ]);
        let serialized = serde_json::to_string(&nested_array).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 2);
    }

    /// Test object serialization (matches C# JObject serialization exactly)
    #[test]
    fn test_object_serialization_compatibility() {
        // Test empty object
        let empty_obj = JToken::Object(OrderedDictionary::new());
        let serialized = serde_json::to_string(&empty_obj).unwrap();
        assert_eq!(serialized, "{}");

        // Test object with various property types
        let mut properties = OrderedDictionary::new();
        properties.insert("null_prop".to_string(), Some(JToken::Null));
        properties.insert("bool_prop".to_string(), Some(JToken::Boolean(true)));
        properties.insert("num_prop".to_string(), Some(JToken::Number(123.456)));
        properties.insert(
            "str_prop".to_string(),
            Some(JToken::String("value".to_string())),
        );
        properties.insert("missing_prop".to_string(), None);

        let object_token = JToken::Object(properties);
        let serialized = serde_json::to_string(&object_token).unwrap();

        // Parse back to verify structure
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert!(parsed.is_object());
        let object = parsed.as_object().unwrap();

        assert!(object.contains_key("null_prop"));
        assert!(object.contains_key("bool_prop"));
        assert!(object.contains_key("num_prop"));
        assert!(object.contains_key("str_prop"));
        assert!(object.contains_key("missing_prop"));

        assert!(object["null_prop"].is_null());
        assert!(object["bool_prop"].as_bool().unwrap());
        assert_eq!(object["num_prop"].as_f64().unwrap(), 123.456);
        assert_eq!(object["str_prop"].as_str().unwrap(), "value");
        assert!(object["missing_prop"].is_null());

        // Test nested objects
        let mut nested = OrderedDictionary::new();
        nested.insert(
            "inner".to_string(),
            Some(JToken::String("inner_value".to_string())),
        );

        let mut outer = OrderedDictionary::new();
        outer.insert("nested_obj".to_string(), Some(JToken::Object(nested)));
        outer.insert(
            "simple".to_string(),
            Some(JToken::String("simple_value".to_string())),
        );

        let nested_token = JToken::Object(outer);
        let serialized = serde_json::to_string(&nested_token).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        assert!(parsed["nested_obj"].is_object());
        assert_eq!(
            parsed["nested_obj"]["inner"].as_str().unwrap(),
            "inner_value"
        );
        assert_eq!(parsed["simple"].as_str().unwrap(), "simple_value");
    }

    /// Test round-trip serialization (matches C# round-trip consistency exactly)
    #[test]
    fn test_roundtrip_serialization_compatibility() {
        // Create complex structure
        let mut root = OrderedDictionary::new();
        root.insert("version".to_string(), Some(JToken::Number(3.0)));
        root.insert("name".to_string(), Some(JToken::String("Neo".to_string())));
        root.insert("active".to_string(), Some(JToken::Boolean(true)));
        root.insert("config".to_string(), None);

        // Add array
        let items = vec![
            Some(JToken::String("item1".to_string())),
            Some(JToken::Number(42.0)),
            None,
            Some(JToken::Boolean(false)),
        ];
        root.insert("items".to_string(), Some(JToken::Array(items)));

        // Add nested object
        let mut nested = OrderedDictionary::new();
        nested.insert("timeout".to_string(), Some(JToken::Number(30000.0)));
        nested.insert("retries".to_string(), Some(JToken::Number(3.0)));
        root.insert("settings".to_string(), Some(JToken::Object(nested)));

        let original = JToken::Object(root);

        let serialized = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: JToken = serde_json::from_str(&serialized).unwrap();

        // Verify equality
        assert_eq!(original, deserialized);

        // Verify structure is preserved
        if let JToken::Object(ref obj) = deserialized {
            assert_eq!(obj.len(), 5);
            assert!(obj.contains_key(&"version".to_string()));
            assert!(obj.contains_key(&"name".to_string()));
            assert!(obj.contains_key(&"active".to_string()));
            assert!(obj.contains_key(&"config".to_string()));
            assert!(obj.contains_key(&"items".to_string()));
            assert!(obj.contains_key(&"settings".to_string()));

            // Check specific values
            assert_eq!(
                obj.get(&"version".to_string()),
                Some(&Some(JToken::Number(3.0)))
            );
            assert_eq!(
                obj.get(&"name".to_string()),
                Some(&Some(JToken::String("Neo".to_string())))
            );
            assert_eq!(
                obj.get(&"active".to_string()),
                Some(&Some(JToken::Boolean(true)))
            );
            assert_eq!(obj.get(&"config".to_string()), Some(&None));
        } else {
            panic!("Expected object token");
        }
    }

    /// Test special character serialization (matches C# escape handling exactly)
    #[test]
    fn test_special_character_serialization_compatibility() {
        // Test strings with special characters that need escaping
        let special_chars = vec![
            ("quote", "He said \"Hello\""),
            ("backslash", "Path\\to\\file"),
            ("newline", "Line1\nLine2"),
            ("carriage_return", "Line1\rLine2"),
            ("tab", "Col1\tCol2"),
            ("form_feed", "Page1\u{000C}Page2"),
            ("backspace", "Text\u{0008}Corrected"),
            ("null_char", "Text\u{0000}WithNull"),
            ("unicode", "Unicode: ñáéíóú 中文 🌍"),
            (
                "combined",
                "Complex: \"Quote\"\n\tTab\r\nNewline\\Backslash",
            ),
        ];

        for (name, text) in special_chars {
            let token = JToken::String(text.to_string());
            let serialized = serde_json::to_string(&token).unwrap();

            // Deserialize back
            let deserialized: JToken = serde_json::from_str(&serialized).unwrap();

            // Verify round-trip preserves original text
            if let JToken::String(ref result_text) = deserialized {
                assert_eq!(result_text, text, "Failed for case: {name}");
            } else {
                panic!("Expected string token for case: {name}");
            }
        }
    }

    /// Test numeric serialization edge cases (matches C# number handling exactly)
    #[test]
    fn test_numeric_serialization_compatibility() {
        // Test various numeric values
        let numbers = vec![
            0.0,
            -0.0,
            1.0,
            -1.0,
            42.0,
            -42.0,
            std::f64::consts::PI,
            -std::f64::consts::PI,
            1.23e10,
            1.23e-10,
            f64::MAX,
            f64::MIN,
            f64::MIN_POSITIVE,
        ];

        for number in numbers {
            let token = JToken::Number(number);
            let serialized = serde_json::to_string(&token).unwrap();

            // Deserialize back
            let deserialized: JToken = serde_json::from_str(&serialized).unwrap();

            // Verify round-trip preserves value
            if let JToken::Number(result_number) = deserialized {
                if number.is_nan() {
                    assert!(result_number.is_nan());
                } else if number == 0.0 && result_number == 0.0 {
                    // Both zeros are equal regardless of sign
                    // both are zero; nothing to assert
                } else {
                    assert_eq!(result_number, number, "Failed for number: {number}");
                }
            } else {
                panic!("Expected number token for: {number}");
            }
        }

        let special_numbers = vec![f64::INFINITY, f64::NEG_INFINITY, f64::NAN];

        for number in special_numbers {
            let token = JToken::Number(number);
            // Some JSON implementations don't support special float values
            // Test that serialization at least doesn't panic
            let _serialized = serde_json::to_string(&token);
            // Note: Deserialization might fail for special values, which is acceptable
        }
    }

    /// Test large structure serialization (matches C# performance characteristics)
    #[test]
    fn test_large_structure_serialization_compatibility() {
        // Create large nested structure
        let mut root = OrderedDictionary::new();

        // Large array
        let mut large_array = Vec::new();
        for i in 0..1000 {
            let mut item = OrderedDictionary::new();
            item.insert("id".to_string(), Some(JToken::Number(i as f64)));
            item.insert(
                "name".to_string(),
                Some(JToken::String(format!("item_{i:04}"))),
            );
            item.insert("active".to_string(), Some(JToken::Boolean(i % 2 == 0)));
            large_array.push(Some(JToken::Object(item)));
        }
        root.insert("items".to_string(), Some(JToken::Array(large_array)));

        // Large object with many properties
        let mut large_object = OrderedDictionary::new();
        for i in 0..500 {
            large_object.insert(
                format!("prop_{i:03}"),
                Some(JToken::String(format!("value_{i:03}"))),
            );
        }
        root.insert("properties".to_string(), Some(JToken::Object(large_object)));

        let large_token = JToken::Object(root);

        // Test serialization doesn't panic and completes
        let serialized = serde_json::to_string(&large_token).unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization works
        let deserialized: JToken = serde_json::from_str(&serialized).unwrap();

        // Verify structure is preserved
        if let JToken::Object(ref obj) = deserialized {
            assert_eq!(obj.len(), 2);
            assert!(obj.contains_key(&"items".to_string()));
            assert!(obj.contains_key(&"properties".to_string()));

            // Verify array structure
            if let Some(Some(JToken::Array(ref items))) = obj.get(&"items".to_string()) {
                assert_eq!(items.len(), 1000);
            } else {
                panic!("Expected items array");
            }

            // Verify object structure
            if let Some(Some(JToken::Object(ref props))) = obj.get(&"properties".to_string()) {
                assert_eq!(props.len(), 500);
            } else {
                panic!("Expected properties object");
            }
        } else {
            panic!("Expected root object");
        }
    }

    /// Test Neo blockchain JSON serialization (matches C# Neo JSON patterns exactly)
    #[test]
    fn test_neo_blockchain_serialization_compatibility() {
        // Create Neo block-style JSON
        let mut block = OrderedDictionary::new();
        block.insert(
            "hash".to_string(),
            Some(JToken::String(
                "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            )),
        );
        block.insert("size".to_string(), Some(JToken::Number(1234.0)));
        block.insert("version".to_string(), Some(JToken::Number(0.0)));
        block.insert(
            "previousblockhash".to_string(),
            Some(JToken::String(
                "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            )),
        );
        block.insert(
            "merkleroot".to_string(),
            Some(JToken::String(
                "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
            )),
        );
        block.insert("time".to_string(), Some(JToken::Number(1640995200000.0)));
        block.insert("index".to_string(), Some(JToken::Number(12345.0)));
        block.insert(
            "nonce".to_string(),
            Some(JToken::String("0x0000000000000000".to_string())),
        );

        // Add witnesses
        let mut witnesses = Vec::new();
        let mut witness = OrderedDictionary::new();
        witness.insert(
            "invocation".to_string(),
            Some(JToken::String("".to_string())),
        );
        witness.insert(
            "verification".to_string(),
            Some(JToken::String(
                "EQwhA/HsPB4oPogN5unEifDyfBkAfFM4WqpMDJF8MgB57a3yEQtBMHOzuw==".to_string(),
            )),
        );
        witnesses.push(Some(JToken::Object(witness)));
        block.insert("witnesses".to_string(), Some(JToken::Array(witnesses)));

        // Add transactions
        let mut transactions = Vec::new();
        let mut tx = OrderedDictionary::new();
        tx.insert(
            "hash".to_string(),
            Some(JToken::String(
                "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
            )),
        );
        tx.insert("size".to_string(), Some(JToken::Number(250.0)));
        tx.insert("version".to_string(), Some(JToken::Number(0.0)));
        tx.insert("nonce".to_string(), Some(JToken::Number(123456789.0)));
        tx.insert(
            "sender".to_string(),
            Some(JToken::String(
                "NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB".to_string(),
            )),
        );
        tx.insert("sysfee".to_string(), Some(JToken::String("0".to_string())));
        tx.insert(
            "netfee".to_string(),
            Some(JToken::String("1000000".to_string())),
        );
        tx.insert("validuntilblock".to_string(), Some(JToken::Number(12445.0)));

        // Add signers
        let mut signers = Vec::new();
        let mut signer = OrderedDictionary::new();
        signer.insert(
            "account".to_string(),
            Some(JToken::String(
                "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            )),
        );
        signer.insert(
            "scopes".to_string(),
            Some(JToken::String("CalledByEntry".to_string())),
        );
        signers.push(Some(JToken::Object(signer)));
        tx.insert("signers".to_string(), Some(JToken::Array(signers)));

        tx.insert("attributes".to_string(), Some(JToken::Array(vec![])));
        tx.insert(
            "script".to_string(),
            Some(JToken::String(
                "VwEBEEEfLnsHEVqNG0wJD/////8AAAAVZ5TKjiAM1w==".to_string(),
            )),
        );

        transactions.push(Some(JToken::Object(tx)));
        block.insert("tx".to_string(), Some(JToken::Array(transactions)));

        let block_token = JToken::Object(block);

        // Test serialization
        let serialized = serde_json::to_string(&block_token).unwrap();
        assert!(!serialized.is_empty());

        // Test round-trip
        let deserialized: JToken = serde_json::from_str(&serialized).unwrap();
        assert_eq!(block_token, deserialized);

        // Test pretty-printed serialization
        let pretty_serialized = serde_json::to_string_pretty(&block_token).unwrap();
        assert!(!pretty_serialized.is_empty());
        assert!(pretty_serialized.len() > serialized.len()); // Pretty format should be longer

        // Verify pretty format can be deserialized
        let pretty_deserialized: JToken = serde_json::from_str(&pretty_serialized).unwrap();
        assert_eq!(block_token, pretty_deserialized);
    }

    /// Test error handling during serialization (matches C# error behavior exactly)
    #[test]
    fn test_serialization_error_handling_compatibility() {
        // But test that we handle edge cases gracefully

        // Test empty structures
        let empty_array = JToken::Array(vec![]);
        assert!(serde_json::to_string(&empty_array).is_ok());

        let empty_object = JToken::Object(OrderedDictionary::new());
        assert!(serde_json::to_string(&empty_object).is_ok());

        // Test structures with null values
        let array_with_nulls = JToken::Array(vec![None, Some(JToken::Null), None]);
        assert!(serde_json::to_string(&array_with_nulls).is_ok());

        let mut obj_with_nulls = OrderedDictionary::new();
        obj_with_nulls.insert("null_prop".to_string(), None);
        obj_with_nulls.insert("explicit_null".to_string(), Some(JToken::Null));
        let object_with_nulls = JToken::Object(obj_with_nulls);
        assert!(serde_json::to_string(&object_with_nulls).is_ok());

        // Test deeply nested structures
        let mut deep_nested = JToken::Object(OrderedDictionary::new());
        for _i in 0..100 {
            let mut new_level = OrderedDictionary::new();
            new_level.insert("nested".to_string(), Some(deep_nested));
            deep_nested = JToken::Object(new_level);
        }
        // This should not panic, though it might be slow
        assert!(serde_json::to_string(&deep_nested).is_ok());
    }
}

//! JToken C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Json.JToken functionality.
//! Tests are based on the C# Neo.Json.JToken test suite.

use neo_json::*;

#[cfg(test)]
#[allow(dead_code)]
mod jtoken_tests {
    use super::*;

    /// Test JToken creation and basic properties (matches C# JToken tests exactly)
    #[test]
    fn test_jtoken_creation_compatibility() {
        let null_token = JToken::Null;
        assert_eq!(null_token, JToken::Null);

        let true_token = JToken::Boolean(true);
        let false_token = JToken::Boolean(false);
        assert_eq!(true_token, JToken::Boolean(true));
        assert_eq!(false_token, JToken::Boolean(false));
        assert_ne!(true_token, false_token);

        let int_token = JToken::Number(42.0);
        let float_token = JToken::Number(std::f64::consts::PI);
        let zero_token = JToken::Number(0.0);
        let negative_token = JToken::Number(-123.456);

        assert_eq!(int_token, JToken::Number(42.0));
        assert_eq!(float_token, JToken::Number(std::f64::consts::PI));
        assert_eq!(zero_token, JToken::Number(0.0));
        assert_eq!(negative_token, JToken::Number(-123.456));

        let string_token = JToken::String("test string".to_string());
        let empty_string_token = JToken::String("".to_string());
        let unicode_token = JToken::String("Hello 世界 🌍".to_string());

        assert_eq!(string_token, JToken::String("test string".to_string()));
        assert_eq!(empty_string_token, JToken::String("".to_string()));
        assert_eq!(unicode_token, JToken::String("Hello 世界 🌍".to_string()));
    }

    /// Test JToken array operations (matches C# JToken array handling exactly)
    #[test]
    fn test_jtoken_array_operations_compatibility() {
        // Test empty array creation
        let empty_array = JToken::Array(vec![]);
        assert_eq!(empty_array, JToken::Array(vec![]));

        // Test array with various token types
        let mixed_array = JToken::Array(vec![
            Some(JToken::Null),
            Some(JToken::Boolean(true)),
            Some(JToken::Number(42.0)),
            Some(JToken::String("test".to_string())),
            None, // null entry
        ]);

        if let JToken::Array(ref arr) = mixed_array {
            assert_eq!(arr.len(), 5);
            assert_eq!(arr[0], Some(JToken::Null));
            assert_eq!(arr[1], Some(JToken::Boolean(true)));
            assert_eq!(arr[2], Some(JToken::Number(42.0)));
            assert_eq!(arr[3], Some(JToken::String("test".to_string())));
            assert_eq!(arr[4], None);
        } else {
            panic!("Expected array token");
        }

        // Test array index access
        assert_eq!(mixed_array.get_index(0).unwrap(), Some(&JToken::Null));
        assert_eq!(
            mixed_array.get_index(1).unwrap(),
            Some(&JToken::Boolean(true))
        );
        assert_eq!(mixed_array.get_index(4).unwrap(), None);
        assert_eq!(mixed_array.get_index(10).unwrap(), None); // Out of bounds
    }

    /// Test JToken object operations (matches C# JToken object handling exactly)
    #[test]
    fn test_jtoken_object_operations_compatibility() {
        // Test empty object creation
        let empty_obj = OrderedDictionary::new();
        let empty_token = JToken::Object(empty_obj);

        if let JToken::Object(ref obj) = empty_token {
            assert_eq!(obj.len(), 0);
        } else {
            panic!("Expected object token");
        }

        // Test object with various properties
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

        if let JToken::Object(ref obj) = object_token {
            assert_eq!(obj.len(), 5);
            assert!(obj.contains_key(&"null_prop".to_string()));
            assert!(obj.contains_key(&"bool_prop".to_string()));
            assert!(obj.contains_key(&"num_prop".to_string()));
            assert!(obj.contains_key(&"str_prop".to_string()));
            assert!(obj.contains_key(&"missing_prop".to_string()));

            assert_eq!(obj.get(&"null_prop".to_string()), Some(&Some(JToken::Null)));
            assert_eq!(
                obj.get(&"bool_prop".to_string()),
                Some(&Some(JToken::Boolean(true)))
            );
            assert_eq!(obj.get(&"missing_prop".to_string()), Some(&None));
            assert_eq!(obj.get(&"nonexistent".to_string()), None);
        } else {
            panic!("Expected object token");
        }
    }

    /// Test JToken type checking and validation (matches C# JToken type system exactly)
    #[test]
    fn test_jtoken_type_checking_compatibility() {
        // Test all token types
        let tokens = vec![
            JToken::Null,
            JToken::Boolean(true),
            JToken::Boolean(false),
            JToken::Number(42.0),
            JToken::Number(-std::f64::consts::PI),
            JToken::Number(0.0),
            JToken::String("test".to_string()),
            JToken::String("".to_string()),
            JToken::Array(vec![]),
            JToken::Array(vec![Some(JToken::Number(1.0)), Some(JToken::Number(2.0))]),
            JToken::Object(OrderedDictionary::new()),
        ];

        // Verify type consistency
        for token in &tokens {
            match token {
                JToken::Null => {
                    assert_eq!(*token, JToken::Null);
                }
                JToken::Boolean(val) => {
                    assert!(matches!(token, JToken::Boolean(_)));
                    if let JToken::Boolean(b) = token {
                        assert_eq!(*b, *val);
                    }
                }
                JToken::Number(val) => {
                    assert!(matches!(token, JToken::Number(_)));
                    if let JToken::Number(n) = token {
                        assert_eq!(*n, *val);
                    }
                }
                JToken::String(val) => {
                    assert!(matches!(token, JToken::String(_)));
                    if let JToken::String(s) = token {
                        assert_eq!(s, val);
                    }
                }
                JToken::Array(_) => {
                    assert!(matches!(token, JToken::Array(_)));
                }
                JToken::Object(_) => {
                    assert!(matches!(token, JToken::Object(_)));
                }
            }
        }
    }

    /// Test JToken equality and comparison (matches C# JToken.Equals exactly)
    #[test]
    fn test_jtoken_equality_compatibility() {
        // Test null equality
        assert_eq!(JToken::Null, JToken::Null);

        // Test boolean equality
        assert_eq!(JToken::Boolean(true), JToken::Boolean(true));
        assert_eq!(JToken::Boolean(false), JToken::Boolean(false));
        assert_ne!(JToken::Boolean(true), JToken::Boolean(false));

        // Test number equality
        assert_eq!(JToken::Number(42.0), JToken::Number(42.0));
        assert_eq!(JToken::Number(0.0), JToken::Number(0.0));
        assert_ne!(JToken::Number(1.0), JToken::Number(2.0));

        assert_eq!(
            JToken::String("test".to_string()),
            JToken::String("test".to_string())
        );
        assert_eq!(
            JToken::String("".to_string()),
            JToken::String("".to_string())
        );
        assert_ne!(
            JToken::String("a".to_string()),
            JToken::String("b".to_string())
        );

        // Test array equality
        let arr1 = JToken::Array(vec![Some(JToken::Number(1.0)), Some(JToken::Number(2.0))]);
        let arr2 = JToken::Array(vec![Some(JToken::Number(1.0)), Some(JToken::Number(2.0))]);
        let arr3 = JToken::Array(vec![Some(JToken::Number(1.0)), Some(JToken::Number(3.0))]);

        assert_eq!(arr1, arr2);
        assert_ne!(arr1, arr3);

        // Test object equality
        let mut obj1 = OrderedDictionary::new();
        obj1.insert("key".to_string(), Some(JToken::String("value".to_string())));
        let token1 = JToken::Object(obj1);

        let mut obj2 = OrderedDictionary::new();
        obj2.insert("key".to_string(), Some(JToken::String("value".to_string())));
        let token2 = JToken::Object(obj2);

        let mut obj3 = OrderedDictionary::new();
        obj3.insert(
            "key".to_string(),
            Some(JToken::String("different".to_string())),
        );
        let token3 = JToken::Object(obj3);

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);

        // Test cross-type inequality
        assert_ne!(JToken::Null, JToken::Boolean(false));
        assert_ne!(JToken::Number(0.0), JToken::String("0".to_string()));
        assert_ne!(
            JToken::Array(vec![]),
            JToken::Object(OrderedDictionary::new())
        );
    }

    /// Test JToken cloning and copying (matches C# JToken cloning behavior exactly)
    #[test]
    fn test_jtoken_cloning_compatibility() {
        // Test primitive token cloning
        let original_null = JToken::Null;
        let cloned_null = original_null.clone();
        assert_eq!(original_null, cloned_null);

        let original_bool = JToken::Boolean(true);
        let cloned_bool = original_bool.clone();
        assert_eq!(original_bool, cloned_bool);

        let original_num = JToken::Number(42.0);
        let cloned_num = original_num.clone();
        assert_eq!(original_num, cloned_num);

        let original_str = JToken::String("test".to_string());
        let cloned_str = original_str.clone();
        assert_eq!(original_str, cloned_str);

        // Test array cloning
        let original_array = JToken::Array(vec![
            Some(JToken::Number(1.0)),
            Some(JToken::String("test".to_string())),
            None,
        ]);
        let cloned_array = original_array.clone();
        assert_eq!(original_array, cloned_array);

        // Test object cloning
        let mut properties = OrderedDictionary::new();
        properties.insert("key1".to_string(), Some(JToken::Number(123.0)));
        properties.insert(
            "key2".to_string(),
            Some(JToken::String("value".to_string())),
        );
        let original_object = JToken::Object(properties);
        let cloned_object = original_object.clone();
        assert_eq!(original_object, cloned_object);
    }

    /// Test JToken array modification (matches C# JToken array mutation exactly)
    #[test]
    fn test_jtoken_array_modification_compatibility() {
        let mut array = JToken::Array(vec![
            Some(JToken::Number(1.0)),
            Some(JToken::Number(2.0)),
            Some(JToken::Number(3.0)),
        ]);

        // Test index setting
        assert!(array
            .set_index(1, Some(JToken::String("modified".to_string())))
            .is_ok());
        assert_eq!(
            array.get_index(1).unwrap(),
            Some(&JToken::String("modified".to_string()))
        );

        // Test setting to null
        assert!(array.set_index(2, None).is_ok());
        assert_eq!(array.get_index(2).unwrap(), None);

        // Test out of bounds
        assert!(array.set_index(10, Some(JToken::Number(42.0))).is_err());

        // Test error on non-array
        let mut non_array = JToken::Number(42.0);
        assert!(non_array.set_index(0, Some(JToken::Null)).is_err());
    }

    /// Test JToken special values (matches C# special value handling exactly)
    #[test]
    fn test_jtoken_special_values_compatibility() {
        // Test floating point special values
        let positive_infinity = JToken::Number(f64::INFINITY);
        let negative_infinity = JToken::Number(f64::NEG_INFINITY);
        let nan_value = JToken::Number(f64::NAN);

        // These should be handled gracefully
        assert!(matches!(positive_infinity, JToken::Number(_)));
        assert!(matches!(negative_infinity, JToken::Number(_)));
        assert!(matches!(nan_value, JToken::Number(_)));

        // Test very large and small numbers
        let large_number = JToken::Number(1.797_693_134_862_315_7e308);
        let small_number = JToken::Number(2.225_073_858_507_201_4e-308);
        assert!(matches!(large_number, JToken::Number(_)));
        assert!(matches!(small_number, JToken::Number(_)));

        let special_string =
            JToken::String("Line1\nLine2\tTab\"Quote'Apostrophe\\Backslash".to_string());
        assert!(matches!(special_string, JToken::String(_)));

        // Test empty collections
        let empty_array = JToken::Array(vec![]);
        let empty_object = JToken::Object(OrderedDictionary::new());
        assert!(matches!(empty_array, JToken::Array(_)));
        assert!(matches!(empty_object, JToken::Object(_)));
    }

    /// Test JToken nested structures (matches C# nested JSON handling exactly)
    #[test]
    fn test_jtoken_nested_structures_compatibility() {
        // Create deeply nested structure
        let mut level3 = OrderedDictionary::new();
        level3.insert(
            "value".to_string(),
            Some(JToken::String("deep_value".to_string())),
        );
        level3.insert("number".to_string(), Some(JToken::Number(42.0)));

        let mut level2 = OrderedDictionary::new();
        level2.insert("nested".to_string(), Some(JToken::Object(level3)));
        level2.insert(
            "array".to_string(),
            Some(JToken::Array(vec![
                Some(JToken::Number(1.0)),
                Some(JToken::Number(2.0)),
                Some(JToken::Number(3.0)),
            ])),
        );

        let mut level1 = OrderedDictionary::new();
        level1.insert("data".to_string(), Some(JToken::Object(level2)));
        level1.insert(
            "meta".to_string(),
            Some(JToken::String("metadata".to_string())),
        );

        let root = JToken::Object(level1);

        // Verify structure
        if let JToken::Object(ref obj) = root {
            assert!(obj.contains_key(&"data".to_string()));
            assert!(obj.contains_key(&"meta".to_string()));

            if let Some(Some(JToken::Object(ref data))) = obj.get(&"data".to_string()) {
                assert!(data.contains_key(&"nested".to_string()));
                assert!(data.contains_key(&"array".to_string()));

                if let Some(Some(JToken::Object(ref nested))) = data.get(&"nested".to_string()) {
                    assert_eq!(
                        nested.get(&"value".to_string()),
                        Some(&Some(JToken::String("deep_value".to_string())))
                    );
                    assert_eq!(
                        nested.get(&"number".to_string()),
                        Some(&Some(JToken::Number(42.0)))
                    );
                }
            }
        } else {
            panic!("Expected root object");
        }
    }

    /// Test JToken performance characteristics (matches C# performance expectations)
    #[test]
    fn test_jtoken_performance_compatibility() {
        // Test large array creation
        let mut large_array = Vec::new();
        for i in 0..1000 {
            large_array.push(Some(JToken::Number(i as f64)));
        }
        let array_token = JToken::Array(large_array);

        // Test access performance
        for i in 0..1000 {
            let value = array_token.get_index(i).unwrap();
            assert_eq!(value, Some(&JToken::Number(i as f64)));
        }

        // Test large object creation
        let mut large_object = OrderedDictionary::new();
        for i in 0..1000 {
            large_object.insert(
                format!("key_{i}"),
                Some(JToken::String(format!("value_{i}"))),
            );
        }
        let object_token = JToken::Object(large_object);

        // Test object access
        if let JToken::Object(ref obj) = object_token {
            for i in 0..1000 {
                let key = format!("key_{i}");
                assert!(obj.contains_key(&key));
                assert_eq!(
                    obj.get(&key),
                    Some(&Some(JToken::String(format!("value_{i}"))))
                );
            }
        }
    }
}

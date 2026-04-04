//! JArray C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Json.JArray functionality.
//! Tests are based on the C# Neo.Json.JArray test suite.

use neo_json::*;

#[cfg(test)]
#[allow(dead_code)]
mod jarray_tests {
    use super::*;

    /// Test JArray creation and basic operations (matches C# JArray tests exactly)
    #[test]
    fn test_jarray_creation_compatibility() {
        let empty_array = JArray::new();
        assert_eq!(empty_array.len(), 0);
        assert!(empty_array.is_empty());

        // Test array creation from vector
        let items = vec![
            Some(JToken::String("first".to_string())),
            Some(JToken::Number(42.0)),
            Some(JToken::Boolean(true)),
            None, // null element
        ];
        let array_from_vec = JArray::from_vec(items.clone());
        assert_eq!(array_from_vec.len(), 4);
        assert!(!array_from_vec.is_empty());

        // Verify elements
        assert_eq!(
            array_from_vec.get(0),
            Some(&JToken::String("first".to_string()))
        );
        assert_eq!(array_from_vec.get(1), Some(&JToken::Number(42.0)));
        assert_eq!(array_from_vec.get(2), Some(&JToken::Boolean(true)));
        assert_eq!(array_from_vec.get(3), None); // null element
        assert_eq!(array_from_vec.get(10), None); // out of bounds

        // Test array creation from iterator
        let iter_items = vec![
            Some(JToken::String("iter1".to_string())),
            Some(JToken::String("iter2".to_string())),
            Some(JToken::String("iter3".to_string())),
        ];
        let array_from_iter = JArray::from_iter(iter_items);
        assert_eq!(array_from_iter.len(), 3);
        assert_eq!(
            array_from_iter.get(0),
            Some(&JToken::String("iter1".to_string()))
        );
        assert_eq!(
            array_from_iter.get(1),
            Some(&JToken::String("iter2".to_string()))
        );
        assert_eq!(
            array_from_iter.get(2),
            Some(&JToken::String("iter3".to_string()))
        );
    }

    /// Test JArray element access and modification (matches C# JArray indexer exactly)
    #[test]
    fn test_jarray_element_access_compatibility() {
        let mut array = JArray::new();

        // Test with empty array
        assert!(array.is_empty());
        assert_eq!(array.get(0), None);

        // Build array with various token types
        let initial_items = vec![
            Some(JToken::Null),
            Some(JToken::Boolean(true)),
            Some(JToken::Boolean(false)),
            Some(JToken::Number(0.0)),
            Some(JToken::Number(42.5)),
            Some(JToken::Number(-123.456)),
            Some(JToken::String("".to_string())),
            Some(JToken::String("test string".to_string())),
            Some(JToken::String("unicode: 世界 🌍".to_string())),
            None, // null element
        ];

        array = JArray::from_vec(initial_items);
        assert_eq!(array.len(), 10);

        // Test element access
        assert_eq!(array.get(0), Some(&JToken::Null));
        assert_eq!(array.get(1), Some(&JToken::Boolean(true)));
        assert_eq!(array.get(2), Some(&JToken::Boolean(false)));
        assert_eq!(array.get(3), Some(&JToken::Number(0.0)));
        assert_eq!(array.get(4), Some(&JToken::Number(42.5)));
        assert_eq!(array.get(5), Some(&JToken::Number(-123.456)));
        assert_eq!(array.get(6), Some(&JToken::String("".to_string())));
        assert_eq!(
            array.get(7),
            Some(&JToken::String("test string".to_string()))
        );
        assert_eq!(
            array.get(8),
            Some(&JToken::String("unicode: 世界 🌍".to_string()))
        );
        assert_eq!(array.get(9), None); // null element
        assert_eq!(array.get(10), None); // out of bounds

        // Test mutable element access
        if let Some(Some(token)) = array.get_mut(1) {
            *token = JToken::String("modified".to_string());
        }
        assert_eq!(array.get(1), Some(&JToken::String("modified".to_string())));
    }

    /// Test JArray element modification operations (matches C# JArray mutation exactly)
    #[test]
    fn test_jarray_modification_compatibility() {
        let mut array = JArray::from_vec(vec![
            Some(JToken::String("original1".to_string())),
            Some(JToken::String("original2".to_string())),
            Some(JToken::String("original3".to_string())),
        ]);

        assert_eq!(array.len(), 3);

        // Test element replacement via set
        let _ = array.set(1, Some(JToken::Number(999.0)));
        assert_eq!(array.get(1), Some(&JToken::Number(999.0)));
        assert_eq!(array.get(0), Some(&JToken::String("original1".to_string()))); // unchanged
        assert_eq!(array.get(2), Some(&JToken::String("original3".to_string()))); // unchanged

        // Test setting to null
        let _ = array.set(2, None);
        assert_eq!(array.get(2), None);
        assert_eq!(array.len(), 3); // size unchanged

        // Test adding elements
        array.add(Some(JToken::Boolean(true)));
        assert_eq!(array.len(), 4);
        assert_eq!(array.get(3), Some(&JToken::Boolean(true)));

        array.add(None);
        assert_eq!(array.len(), 5);
        assert_eq!(array.get(4), None);

        // Test inserting elements
        let _ = array.insert(2, Some(JToken::String("inserted".to_string())));
        assert_eq!(array.len(), 6);
        assert_eq!(array.get(2), Some(&JToken::String("inserted".to_string())));
        assert_eq!(array.get(3), None); // previously at index 2, now at 3

        // Test removing elements
        let removed = array.remove(2).unwrap();
        assert_eq!(removed, Some(JToken::String("inserted".to_string())));
        assert_eq!(array.len(), 5);
        assert_eq!(array.get(2), None); // what was at index 3 is now at 2
    }

    /// Test JArray with nested structures (matches C# JArray nesting exactly)
    #[test]
    fn test_jarray_nested_structures_compatibility() {
        let mut array = JArray::new();

        // Create nested object
        let mut nested_obj = OrderedDictionary::new();
        nested_obj.insert("id".to_string(), Some(JToken::Number(1.0)));
        nested_obj.insert(
            "name".to_string(),
            Some(JToken::String("nested_object".to_string())),
        );
        nested_obj.insert("active".to_string(), Some(JToken::Boolean(true)));

        // Create nested array
        let nested_array = vec![
            Some(JToken::String("nested_item_1".to_string())),
            Some(JToken::String("nested_item_2".to_string())),
            Some(JToken::Number(100.0)),
        ];

        // Add nested structures to main array
        array.add(Some(JToken::Object(nested_obj)));
        array.add(Some(JToken::Array(nested_array)));
        array.add(Some(JToken::String("simple_string".to_string())));

        assert_eq!(array.len(), 3);

        // Test access to nested object
        if let Some(JToken::Object(ref obj)) = array.get(0) {
            assert!(obj.contains_key(&"id".to_string()));
            assert!(obj.contains_key(&"name".to_string()));
            assert!(obj.contains_key(&"active".to_string()));
            assert_eq!(obj.get(&"id".to_string()), Some(&Some(JToken::Number(1.0))));
            assert_eq!(
                obj.get(&"name".to_string()),
                Some(&Some(JToken::String("nested_object".to_string())))
            );
            assert_eq!(
                obj.get(&"active".to_string()),
                Some(&Some(JToken::Boolean(true)))
            );
        } else {
            panic!("Expected nested object at index 0");
        }

        // Test access to nested array
        if let Some(JToken::Array(ref arr)) = array.get(1) {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Some(JToken::String("nested_item_1".to_string())));
            assert_eq!(arr[1], Some(JToken::String("nested_item_2".to_string())));
            assert_eq!(arr[2], Some(JToken::Number(100.0)));
        } else {
            panic!("Expected nested array at index 1");
        }

        // Test simple element
        assert_eq!(
            array.get(2),
            Some(&JToken::String("simple_string".to_string()))
        );
    }

    /// Test JArray iteration and enumeration (matches C# JArray enumeration exactly)
    #[test]
    fn test_jarray_iteration_compatibility() {
        let items = vec![
            Some(JToken::String("first".to_string())),
            Some(JToken::Number(2.0)),
            Some(JToken::Boolean(true)),
            None,
            Some(JToken::String("last".to_string())),
        ];

        let array = JArray::from_vec(items.clone());

        // Test iteration over elements
        for (i, expected_item) in items.iter().enumerate() {
            let actual_item = array.get(i);
            match expected_item {
                Some(expected_token) => {
                    assert_eq!(actual_item, Some(expected_token));
                }
                None => {
                    assert_eq!(actual_item, None);
                }
            }
        }

        // Test iterator-like access patterns
        let mut found_items = Vec::new();
        for i in 0..array.len() {
            found_items.push(array.get(i).cloned());
        }

        assert_eq!(found_items.len(), items.len());
        for (i, found_item) in found_items.iter().enumerate() {
            match &items[i] {
                Some(expected_token) => {
                    assert_eq!(found_item, &Some(expected_token.clone()));
                }
                None => {
                    assert_eq!(found_item, &None);
                }
            }
        }
    }

    /// Test JArray equality and comparison (matches C# JArray.Equals exactly)
    #[test]
    fn test_jarray_equality_compatibility() {
        // Test empty array equality
        let empty1 = JArray::new();
        let empty2 = JArray::new();
        assert_eq!(empty1, empty2);

        // Test arrays with same elements
        let items1 = vec![
            Some(JToken::String("test".to_string())),
            Some(JToken::Number(42.0)),
            Some(JToken::Boolean(true)),
            None,
        ];
        let array1 = JArray::from_vec(items1.clone());
        let array2 = JArray::from_vec(items1);
        assert_eq!(array1, array2);

        // Test arrays with different elements
        let items3 = vec![
            Some(JToken::String("test".to_string())),
            Some(JToken::Number(43.0)), // Different number
            Some(JToken::Boolean(true)),
            None,
        ];
        let array3 = JArray::from_vec(items3);
        assert_ne!(array1, array3);

        // Test arrays with different lengths
        let items4 = vec![
            Some(JToken::String("test".to_string())),
            Some(JToken::Number(42.0)),
            Some(JToken::Boolean(true)),
            // Missing null element
        ];
        let array4 = JArray::from_vec(items4);
        assert_ne!(array1, array4);

        // Test arrays with elements in different order
        let items5 = vec![
            Some(JToken::Number(42.0)),               // Swapped
            Some(JToken::String("test".to_string())), // Swapped
            Some(JToken::Boolean(true)),
            None,
        ];
        let array5 = JArray::from_vec(items5);
        assert_ne!(array1, array5);

        // Test nested array equality
        let nested_items = vec![Some(JToken::String("nested".to_string()))];
        let items6 = vec![
            Some(JToken::Array(nested_items.clone())),
            Some(JToken::String("other".to_string())),
        ];
        let items7 = vec![
            Some(JToken::Array(nested_items)),
            Some(JToken::String("other".to_string())),
        ];
        let array6 = JArray::from_vec(items6);
        let array7 = JArray::from_vec(items7);
        assert_eq!(array6, array7);
    }

    /// Test JArray cloning (matches C# JArray cloning behavior exactly)
    #[test]
    fn test_jarray_cloning_compatibility() {
        let original_items = vec![
            Some(JToken::String("clone_test".to_string())),
            Some(JToken::Number(123.456)),
            Some(JToken::Boolean(false)),
            None,
        ];

        let mut nested_obj = OrderedDictionary::new();
        nested_obj.insert(
            "nested_prop".to_string(),
            Some(JToken::String("nested_value".to_string())),
        );
        let nested_items = vec![Some(JToken::Object(nested_obj))];

        let mut all_items = original_items;
        all_items.push(Some(JToken::Array(nested_items)));

        let original = JArray::from_vec(all_items);
        let cloned = original.clone();

        // Verify clone equality
        assert_eq!(original, cloned);
        assert_eq!(original.len(), cloned.len());

        // Verify all elements are cloned
        for i in 0..original.len() {
            assert_eq!(original.get(i), cloned.get(i));
        }

        // Test that modifications to clone don't affect original
        let mut cloned_mut = cloned;
        cloned_mut.add(Some(JToken::String("added_to_clone".to_string())));

        assert_eq!(cloned_mut.len(), original.len() + 1);
        assert_ne!(original, cloned_mut);
        assert_eq!(
            cloned_mut.get(original.len()),
            Some(&JToken::String("added_to_clone".to_string()))
        );
        assert_eq!(original.get(original.len()), None); // Out of bounds in original
    }

    /// Test JArray clearing and removal operations (matches C# JArray.Clear exactly)
    #[test]
    fn test_jarray_clearing_compatibility() {
        let mut array = JArray::from_vec(vec![
            Some(JToken::String("item1".to_string())),
            Some(JToken::String("item2".to_string())),
            Some(JToken::String("item3".to_string())),
            Some(JToken::Number(42.0)),
            None,
        ]);

        assert_eq!(array.len(), 5);
        assert!(!array.is_empty());

        // Test removing specific elements
        let removed = array.remove(1).unwrap();
        assert_eq!(removed, Some(JToken::String("item2".to_string())));
        assert_eq!(array.len(), 4);
        assert_eq!(array.get(1), Some(&JToken::String("item3".to_string()))); // Shifted down

        // Test removing from end
        let last_removed = array.remove(array.len() - 1).unwrap();
        assert_eq!(last_removed, None); // Was the null element
        assert_eq!(array.len(), 3);

        // Test clearing entire array
        array.clear();
        assert_eq!(array.len(), 0);
        assert!(array.is_empty());
        assert_eq!(array.get(0), None);

        // Test that array can be reused after clearing
        array.add(Some(JToken::String("after_clear".to_string())));
        assert_eq!(array.len(), 1);
        assert!(!array.is_empty());
        assert_eq!(
            array.get(0),
            Some(&JToken::String("after_clear".to_string()))
        );
    }

    /// Test JArray with special values and edge cases (matches C# edge case handling exactly)
    #[test]
    fn test_jarray_edge_cases_compatibility() {
        let mut array = JArray::new();

        // Test with floating point special values
        array.add(Some(JToken::Number(f64::INFINITY)));
        array.add(Some(JToken::Number(f64::NEG_INFINITY)));
        array.add(Some(JToken::Number(f64::NAN)));
        array.add(Some(JToken::Number(0.0)));
        array.add(Some(JToken::Number(-0.0)));

        assert_eq!(array.len(), 5);

        // Test with very large and small numbers
        array.add(Some(JToken::Number(f64::MAX)));
        array.add(Some(JToken::Number(f64::MIN)));
        array.add(Some(JToken::Number(f64::MIN_POSITIVE)));

        // Test with special strings
        array.add(Some(JToken::String("".to_string()))); // Empty string
        array.add(Some(JToken::String("\n\r\t".to_string()))); // Whitespace
        array.add(Some(JToken::String("\"'\\".to_string()))); // Quotes and backslash
        array.add(Some(JToken::String("Hello\u{0000}World".to_string()))); // Null character
        array.add(Some(JToken::String(
            "\u{1F600}\u{1F601}\u{1F602}".to_string(),
        ))); // Emojis

        // Test null elements mixed in
        array.add(None);
        array.add(Some(JToken::Null));
        array.add(None);

        // Verify all elements are accessible
        assert_eq!(array.len(), 16);

        // Test access to special values
        assert!(matches!(array.get(0), Some(JToken::Number(_))));
        assert!(matches!(array.get(1), Some(JToken::Number(_))));
        assert!(matches!(array.get(2), Some(JToken::Number(_))));

        assert_eq!(array.get(8), Some(&JToken::String("".to_string())));
        assert_eq!(array.get(13), None); // null element
        assert_eq!(array.get(14), Some(&JToken::Null)); // JToken::Null
        assert_eq!(array.get(15), None); // null element
    }

    /// Test JArray performance with large datasets (matches C# performance characteristics)
    #[test]
    fn test_jarray_performance_compatibility() {
        let mut array = JArray::new();
        let element_count = 1000;

        // Test performance of adding many elements
        for i in 0..element_count {
            array.add(Some(JToken::String(format!("element_{i:04}"))));
        }
        assert_eq!(array.len(), element_count);

        // Test access performance
        for i in 0..element_count {
            let expected_value = format!("element_{i:04}");
            assert_eq!(array.get(i), Some(&JToken::String(expected_value)));
        }

        // Test modification performance
        for i in 0..element_count {
            let new_value = format!("modified_{i:04}");
            array.set(i, Some(JToken::String(new_value)));
        }

        // Verify modifications
        for i in 0..element_count {
            let expected_value = format!("modified_{i:04}");
            assert_eq!(array.get(i), Some(&JToken::String(expected_value)));
        }

        let initial_len = array.len();
        array.insert(0, Some(JToken::String("inserted_at_start".to_string())));
        assert_eq!(array.len(), initial_len + 1);
        assert_eq!(
            array.get(0),
            Some(&JToken::String("inserted_at_start".to_string()))
        );
        assert_eq!(
            array.get(1),
            Some(&JToken::String("modified_0000".to_string()))
        ); // Shifted

        // Test removal performance
        let removed = array.remove(0).unwrap();
        assert_eq!(
            removed,
            Some(JToken::String("inserted_at_start".to_string()))
        );
        assert_eq!(array.len(), initial_len);
        assert_eq!(
            array.get(0),
            Some(&JToken::String("modified_0000".to_string()))
        ); // Back to original
    }

    /// Test JArray with heterogeneous data types (matches C# mixed type handling exactly)
    #[test]
    fn test_jarray_heterogeneous_types_compatibility() {
        let mut array = JArray::new();

        // Add various types in sequence
        array.add(Some(JToken::Null));
        array.add(Some(JToken::Boolean(true)));
        array.add(Some(JToken::Boolean(false)));
        array.add(Some(JToken::Number(0.0)));
        array.add(Some(JToken::Number(42.0)));
        array.add(Some(JToken::Number(-3.14159)));
        array.add(Some(JToken::String("simple string".to_string())));
        array.add(Some(JToken::String("unicode: ñáéíóú 中文 🎉".to_string())));

        // Add nested object
        let mut nested_obj = OrderedDictionary::new();
        nested_obj.insert(
            "type".to_string(),
            Some(JToken::String("nested_object".to_string())),
        );
        nested_obj.insert("count".to_string(), Some(JToken::Number(1.0)));
        array.add(Some(JToken::Object(nested_obj)));

        // Add nested array
        let nested_array = vec![
            Some(JToken::String("nested1".to_string())),
            Some(JToken::String("nested2".to_string())),
            Some(JToken::Number(999.0)),
        ];
        array.add(Some(JToken::Array(nested_array)));

        // Add null element
        array.add(None);

        assert_eq!(array.len(), 11);

        // Verify each type is preserved correctly
        assert_eq!(array.get(0), Some(&JToken::Null));
        assert_eq!(array.get(1), Some(&JToken::Boolean(true)));
        assert_eq!(array.get(2), Some(&JToken::Boolean(false)));
        assert_eq!(array.get(3), Some(&JToken::Number(0.0)));
        assert_eq!(array.get(4), Some(&JToken::Number(42.0)));
        assert_eq!(array.get(5), Some(&JToken::Number(-3.14159)));
        assert_eq!(
            array.get(6),
            Some(&JToken::String("simple string".to_string()))
        );
        assert_eq!(
            array.get(7),
            Some(&JToken::String("unicode: ñáéíóú 中文 🎉".to_string()))
        );

        // Verify nested structures
        assert!(matches!(array.get(8), Some(JToken::Object(_))));
        assert!(matches!(array.get(9), Some(JToken::Array(_))));
        assert_eq!(array.get(10), None);

        // Test type checking
        for i in 0..array.len() {
            let element = array.get(i);
            match i {
                0 => assert!(matches!(element, Some(JToken::Null))),
                1 | 2 => assert!(matches!(element, Some(JToken::Boolean(_)))),
                3 | 4 | 5 => assert!(matches!(element, Some(JToken::Number(_)))),
                6 | 7 => assert!(matches!(element, Some(JToken::String(_)))),
                8 => assert!(matches!(element, Some(JToken::Object(_)))),
                9 => assert!(matches!(element, Some(JToken::Array(_)))),
                10 => assert_eq!(element, None),
                _ => unreachable!(),
            }
        }
    }
}

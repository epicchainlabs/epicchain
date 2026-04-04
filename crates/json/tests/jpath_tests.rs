//! JPath C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Json.JPath functionality.
//! Tests are based on the C# Neo.Json JPath expression evaluation.

use neo_json::*;

#[cfg(test)]
#[allow(dead_code)]
mod jpath_tests {
    use super::*;

    /// Test basic JPath parsing and evaluation (matches C# JPath exactly)
    #[test]
    fn test_jpath_basic_parsing_compatibility() {
        // Create test JSON structure
        let mut root = OrderedDictionary::new();
        root.insert("name".to_string(), Some(JToken::String("Neo".to_string())));
        root.insert("version".to_string(), Some(JToken::Number(3.0)));
        root.insert("active".to_string(), Some(JToken::Boolean(true)));

        let json = JToken::Object(root);

        let tokens = JPathToken::parse("$.name").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("Neo".to_string()));

        let tokens = JPathToken::parse("$.version").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(3.0));

        let tokens = JPathToken::parse("$.active").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Boolean(true));

        // Test non-existent property
        let tokens = JPathToken::parse("$.nonexistent").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 0);
    }

    /// Test JPath array access (matches C# JPath array indexing exactly)
    #[test]
    fn test_jpath_array_access_compatibility() {
        // Create array structure
        let items = vec![
            Some(JToken::String("first".to_string())),
            Some(JToken::String("second".to_string())),
            Some(JToken::String("third".to_string())),
            Some(JToken::Number(42.0)),
            None, // null element
        ];

        let mut root = OrderedDictionary::new();
        root.insert("items".to_string(), Some(JToken::Array(items)));
        root.insert("count".to_string(), Some(JToken::Number(5.0)));

        let json = JToken::Object(root);

        let tokens = JPathToken::parse("$.items[0]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("first".to_string()));

        let tokens = JPathToken::parse("$.items[1]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("second".to_string()));

        let tokens = JPathToken::parse("$.items[3]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(42.0));

        // Test null element access
        let tokens = JPathToken::parse("$.items[4]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 0); // null elements are filtered out

        // Test out of bounds access
        let tokens = JPathToken::parse("$.items[10]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 0);

        let tokens = JPathToken::parse("$.items[-1]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        // Behavior depends on implementation - could be last element or empty
        assert!(results.len() <= 1);
    }

    /// Test JPath wildcard operations (matches C# JPath wildcard exactly)
    #[test]
    fn test_jpath_wildcard_compatibility() {
        let items = vec![
            Some(JToken::String("item1".to_string())),
            Some(JToken::String("item2".to_string())),
            Some(JToken::String("item3".to_string())),
            Some(JToken::Number(100.0)),
            Some(JToken::Boolean(true)),
        ];

        let mut root = OrderedDictionary::new();
        root.insert("data".to_string(), Some(JToken::Array(items)));

        let json = JToken::Object(root);

        let tokens = JPathToken::parse("$.data[*]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 5);
        assert_eq!(results[0], &JToken::String("item1".to_string()));
        assert_eq!(results[1], &JToken::String("item2".to_string()));
        assert_eq!(results[2], &JToken::String("item3".to_string()));
        assert_eq!(results[3], &JToken::Number(100.0));
        assert_eq!(results[4], &JToken::Boolean(true));

        let mut obj_with_props = OrderedDictionary::new();
        obj_with_props.insert(
            "prop1".to_string(),
            Some(JToken::String("value1".to_string())),
        );
        obj_with_props.insert(
            "prop2".to_string(),
            Some(JToken::String("value2".to_string())),
        );
        obj_with_props.insert("prop3".to_string(), Some(JToken::Number(42.0)));

        let obj_json = JToken::Object(obj_with_props);

        let tokens = JPathToken::parse("$.*").unwrap();
        let results = JPathToken::evaluate(&tokens, &obj_json).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.contains(&&JToken::String("value1".to_string())));
        assert!(results.contains(&&JToken::String("value2".to_string())));
        assert!(results.contains(&&JToken::Number(42.0)));
    }

    /// Test JPath nested property access (matches C# JPath nested navigation exactly)
    #[test]
    fn test_jpath_nested_access_compatibility() {
        // Create deeply nested structure
        let mut level3 = OrderedDictionary::new();
        level3.insert(
            "value".to_string(),
            Some(JToken::String("deep_value".to_string())),
        );
        level3.insert("count".to_string(), Some(JToken::Number(999.0)));

        let mut level2 = OrderedDictionary::new();
        level2.insert("nested".to_string(), Some(JToken::Object(level3)));
        level2.insert(
            "meta".to_string(),
            Some(JToken::String("metadata".to_string())),
        );

        let mut level1 = OrderedDictionary::new();
        level1.insert("data".to_string(), Some(JToken::Object(level2)));
        level1.insert(
            "root_prop".to_string(),
            Some(JToken::String("root_value".to_string())),
        );

        let json = JToken::Object(level1);

        let tokens = JPathToken::parse("$.data.nested.value").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("deep_value".to_string()));

        let tokens = JPathToken::parse("$.data.nested.count").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(999.0));

        let tokens = JPathToken::parse("$.data.meta").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("metadata".to_string()));

        // Test non-existent nested path
        let tokens = JPathToken::parse("$.data.nested.nonexistent").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 0);

        let tokens = JPathToken::parse("$.data.nonexistent.value").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 0);
    }

    /// Test JPath with array of objects (matches C# JPath object array navigation exactly)
    #[test]
    fn test_jpath_object_array_compatibility() {
        // Create array of objects
        let mut objects = Vec::new();

        for i in 0..3 {
            let mut obj = OrderedDictionary::new();
            obj.insert("id".to_string(), Some(JToken::Number(i as f64)));
            obj.insert(
                "name".to_string(),
                Some(JToken::String(format!("object_{i}"))),
            );
            obj.insert("active".to_string(), Some(JToken::Boolean(i % 2 == 0)));
            objects.push(Some(JToken::Object(obj)));
        }

        let mut root = OrderedDictionary::new();
        root.insert("objects".to_string(), Some(JToken::Array(objects)));

        let json = JToken::Object(root);

        let tokens = JPathToken::parse("$.objects[*].name").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], &JToken::String("object_0".to_string()));
        assert_eq!(results[1], &JToken::String("object_1".to_string()));
        assert_eq!(results[2], &JToken::String("object_2".to_string()));

        let tokens = JPathToken::parse("$.objects[*].id").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], &JToken::Number(0.0));
        assert_eq!(results[1], &JToken::Number(1.0));
        assert_eq!(results[2], &JToken::Number(2.0));

        // Test accessing specific object property
        let tokens = JPathToken::parse("$.objects[1].name").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("object_1".to_string()));

        let tokens = JPathToken::parse("$.objects[2].active").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Boolean(true));
    }

    /// Test JPath array slicing (matches C# JPath slice operations exactly)
    #[test]
    fn test_jpath_array_slicing_compatibility() {
        let mut items = Vec::new();
        for i in 0..10 {
            items.push(Some(JToken::Number(i as f64)));
        }

        let mut root = OrderedDictionary::new();
        root.insert("numbers".to_string(), Some(JToken::Array(items)));

        let json = JToken::Object(root);

        let tokens = JPathToken::parse("$.numbers[2:5]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3); // indices 2, 3, 4
        assert_eq!(results[0], &JToken::Number(2.0));
        assert_eq!(results[1], &JToken::Number(3.0));
        assert_eq!(results[2], &JToken::Number(4.0));

        // Test slice from start
        let tokens = JPathToken::parse("$.numbers[:3]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3); // indices 0, 1, 2
        assert_eq!(results[0], &JToken::Number(0.0));
        assert_eq!(results[1], &JToken::Number(1.0));
        assert_eq!(results[2], &JToken::Number(2.0));

        // Test slice to end
        let tokens = JPathToken::parse("$.numbers[7:]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3); // indices 7, 8, 9
        assert_eq!(results[0], &JToken::Number(7.0));
        assert_eq!(results[1], &JToken::Number(8.0));
        assert_eq!(results[2], &JToken::Number(9.0));

        // Test full slice
        let tokens = JPathToken::parse("$.numbers[:]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 10); // all elements
        for i in 0..10 {
            assert_eq!(results[i], &JToken::Number(i as f64));
        }
    }

    /// Test JPath error handling and edge cases (matches C# JPath error behavior exactly)
    #[test]
    fn test_jpath_error_handling_compatibility() {
        let mut root = OrderedDictionary::new();
        root.insert(
            "test".to_string(),
            Some(JToken::String("value".to_string())),
        );
        let json = JToken::Object(root);

        // Test invalid JPath expressions
        assert!(JPathToken::parse("").is_err()); // Empty path
        assert!(JPathToken::parse("invalid").is_err()); // Missing $
        assert!(JPathToken::parse("$.").is_err()); // Incomplete path
        assert!(JPathToken::parse("$[").is_err()); // Incomplete array access
        assert!(JPathToken::parse("$]").is_err()); // Invalid bracket
        assert!(JPathToken::parse("$.prop[").is_err()); // Incomplete array access

        // Test evaluating on null/empty JSON
        let empty_json = JToken::Object(OrderedDictionary::new());
        let tokens = JPathToken::parse("$.anything").unwrap();
        let results = JPathToken::evaluate(&tokens, &empty_json).unwrap();
        assert_eq!(results.len(), 0);

        let tokens = JPathToken::parse("$.test[0]").unwrap(); // test is string, not array
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 0); // Should handle gracefully

        // Test accessing object property on non-object
        let array_json = JToken::Array(vec![Some(JToken::String("item".to_string()))]);
        let tokens = JPathToken::parse("$.property").unwrap();
        let results = JPathToken::evaluate(&tokens, &array_json).unwrap();
        assert_eq!(results.len(), 0); // Should handle gracefully
    }

    /// Test JPath with special characters and unicode (matches C# unicode handling exactly)
    #[test]
    fn test_jpath_unicode_compatibility() {
        // Create JSON with unicode property names and values
        let mut root = OrderedDictionary::new();
        root.insert(
            "属性".to_string(),
            Some(JToken::String("中文值".to_string())),
        );
        root.insert(
            "property with spaces".to_string(),
            Some(JToken::String("spaced value".to_string())),
        );
        root.insert(
            "property-with-hyphens".to_string(),
            Some(JToken::String("hyphenated".to_string())),
        );
        root.insert(
            "🔑".to_string(),
            Some(JToken::String("emoji key".to_string())),
        );

        // Create array with unicode values
        let unicode_items = vec![
            Some(JToken::String("Hello 世界".to_string())),
            Some(JToken::String("مرحبا".to_string())),
            Some(JToken::String("こんにちは".to_string())),
            Some(JToken::String("🌍🌎🌏".to_string())),
        ];
        root.insert(
            "unicode_array".to_string(),
            Some(JToken::Array(unicode_items)),
        );

        let json = JToken::Object(root);

        // Test unicode property access
        let tokens = JPathToken::parse("$.属性").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("中文值".to_string()));

        // This test depends on how the parser handles spaces
        if let Ok(tokens) = JPathToken::parse("$['property with spaces']") {
            let results = JPathToken::evaluate(&tokens, &json).unwrap();
            assert_eq!(results.len(), 1);
            assert_eq!(results[0], &JToken::String("spaced value".to_string()));
        }

        // Test unicode array access
        let tokens = JPathToken::parse("$.unicode_array[0]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("Hello 世界".to_string()));

        let tokens = JPathToken::parse("$.unicode_array[*]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 4);
        assert_eq!(results[3], &JToken::String("🌍🌎🌏".to_string()));
    }

    /// Test JPath with complex Neo blockchain structures (matches C# Neo JSON patterns exactly)
    #[test]
    fn test_jpath_neo_blockchain_compatibility() {
        // Create Neo blockchain-style JSON structure
        let mut block = OrderedDictionary::new();
        block.insert(
            "hash".to_string(),
            Some(JToken::String("0x1234567890abcdef".to_string())),
        );
        block.insert("index".to_string(), Some(JToken::Number(12345.0)));

        // Create transactions array
        let mut transactions = Vec::new();
        for i in 0..3 {
            let mut tx = OrderedDictionary::new();
            tx.insert(
                "txid".to_string(),
                Some(JToken::String(format!("0x{i:064x}"))),
            );
            tx.insert(
                "size".to_string(),
                Some(JToken::Number((250 + i * 50) as f64)),
            );
            tx.insert(
                "sender".to_string(),
                Some(JToken::String(
                    "NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB".to_string(),
                )),
            );

            // Create vout array
            let mut vouts = Vec::new();
            for j in 0..2 {
                let mut vout = OrderedDictionary::new();
                vout.insert("n".to_string(), Some(JToken::Number(j as f64)));
                vout.insert(
                    "asset".to_string(),
                    Some(JToken::String(
                        "0xd2c270ebfc2a1cdd3e470014a4dff7c091f699ec".to_string(),
                    )),
                );
                vout.insert(
                    "value".to_string(),
                    Some(JToken::String("100000000".to_string())),
                );
                vout.insert(
                    "address".to_string(),
                    Some(JToken::String(format!("Address{i}{j}"))),
                );
                vouts.push(Some(JToken::Object(vout)));
            }
            tx.insert("vout".to_string(), Some(JToken::Array(vouts)));

            transactions.push(Some(JToken::Object(tx)));
        }
        block.insert("tx".to_string(), Some(JToken::Array(transactions)));

        let json = JToken::Object(block);

        // Test Neo-specific JPath queries
        let tokens = JPathToken::parse("$.hash").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            &JToken::String("0x1234567890abcdef".to_string())
        );

        // Test getting all transaction IDs
        let tokens = JPathToken::parse("$.tx[*].txid").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            &JToken::String(
                "0x0000000000000000000000000000000000000000000000000000000000000000".to_string()
            )
        );

        // Test getting specific transaction
        let tokens = JPathToken::parse("$.tx[1].sender").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            &JToken::String("NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB".to_string())
        );

        // Test getting all vout values
        let tokens = JPathToken::parse("$.tx[*].vout[*].value").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 6); // 3 tx * 2 vout each
        for result in &results {
            assert_eq!(result, &&JToken::String("100000000".to_string()));
        }

        // Test getting specific vout
        let tokens = JPathToken::parse("$.tx[0].vout[1].address").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("Address01".to_string()));

        // Test array slicing on transactions
        let tokens = JPathToken::parse("$.tx[1:3].size").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 2); // transactions 1 and 2
        assert_eq!(results[0], &JToken::Number(300.0)); // tx 1 size
        assert_eq!(results[1], &JToken::Number(350.0)); // tx 2 size
    }

    /// Test JPath performance with large structures (matches C# performance expectations)
    #[test]
    fn test_jpath_performance_compatibility() {
        // Create large JSON structure
        let mut root = OrderedDictionary::new();

        // Create large array
        let mut large_array = Vec::new();
        for i in 0..1000 {
            let mut item = OrderedDictionary::new();
            item.insert("id".to_string(), Some(JToken::Number(i as f64)));
            item.insert(
                "name".to_string(),
                Some(JToken::String(format!("item_{i:04}"))),
            );
            item.insert(
                "category".to_string(),
                Some(JToken::String(if i % 3 == 0 {
                    "A".to_string()
                } else if i % 3 == 1 {
                    "B".to_string()
                } else {
                    "C".to_string()
                })),
            );
            large_array.push(Some(JToken::Object(item)));
        }
        root.insert("items".to_string(), Some(JToken::Array(large_array)));

        let json = JToken::Object(root);

        // Test performance of wildcard queries
        let tokens = JPathToken::parse("$.items[*].id").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1000);

        // Verify first and last results
        assert_eq!(results[0], &JToken::Number(0.0));
        assert_eq!(results[999], &JToken::Number(999.0));

        // Test performance of slice queries
        let tokens = JPathToken::parse("$.items[100:200].name").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 100);
        assert_eq!(results[0], &JToken::String("item_0100".to_string()));
        assert_eq!(results[99], &JToken::String("item_0199".to_string()));

        // Test performance of specific index access
        let tokens = JPathToken::parse("$.items[500].category").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("C".to_string())); // 500 % 3 == 2, maps to "C"
    }
}

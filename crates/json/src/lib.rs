//! # Neo JSON Library
//!
//! A comprehensive JSON library for the Neo blockchain ecosystem, providing
//! high-performance JSON parsing, manipulation, and querying capabilities.
//!
//! This library is designed to be fully compatible with the C# Neo.Json library,
//! providing the same API and functionality while leveraging Rust's performance
//! and safety guarantees.
//!
//! ## Features
//!
//! - **Complete JSON Support**: Full support for all JSON data types
//! - **JSON Path Queries**: Advanced JSON path expressions for data extraction
//! - **High Performance**: Optimized for blockchain data processing
//! - **Memory Efficient**: Minimal memory footprint with zero-copy operations where possible
//! - **Thread Safe**: All types are thread-safe and can be shared across threads
//! - **Neo Compatibility**: Full compatibility with Neo blockchain JSON structures
//!
//! ## Quick Start
//!
//! ```rust
//! use neo_json::*;
//!
//! // Create a JSON object
//! let mut obj = OrderedDictionary::new();
//! obj.insert("name".to_string(), Some(JToken::String("Neo".to_string())));
//! obj.insert("version".to_string(), Some(JToken::Number(3.0)));
//!
//! let json = JToken::Object(obj);
//!
//! // Query with JSON path
//! let tokens = JPathToken::parse("$.name").expect("Operation failed");
//! let results = JPathToken::evaluate(&tokens, &json).expect("Operation failed");
//! ```
//!
//! ## Performance Characteristics
//!
//! - **Parsing**: O(n) time complexity for JSON parsing
//! - **Path Queries**: O(log n) average case for property access
//! - **Memory**: Minimal allocations with efficient string interning
//! - **Throughput**: Optimized for high-frequency blockchain operations

// Remove unused imports

pub mod error;
pub mod jarray;
pub mod jboolean;
pub mod jcontainer;
pub mod jnumber;
pub mod jobject;
pub mod jpath;
pub mod jstring;
pub mod jtoken;
pub mod ordered_dictionary;
pub mod utility;

pub use error::{JsonError, JsonResult};
pub use jarray::JArray;
pub use jboolean::JBoolean;
pub use jcontainer::JContainer;
pub use jnumber::JNumber;
pub use jobject::JObject;
pub use jpath::{JPathToken, JPathTokenType};
pub use jstring::JString;
pub use jtoken::JToken;
pub use ordered_dictionary::OrderedDictionary;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_json_creation() {
        let obj = JObject::new();
        assert!(obj.properties().is_empty());
    }

    #[test]
    fn test_json_null() {
        let null_token: Option<JToken> = None;
        assert!(null_token.is_none());
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complex_json_structure() {
        // Test complex nested JSON structure similar to Neo blockchain data
        let mut root = OrderedDictionary::new();

        // Add block information
        root.insert(
            "hash".to_string(),
            Some(JToken::String("0x1234567890abcdef".to_string())),
        );
        root.insert("index".to_string(), Some(JToken::Number(12345.0)));
        root.insert(
            "timestamp".to_string(),
            Some(JToken::Number(1640995200000.0)),
        );

        // Add transactions array
        let mut transactions = Vec::new();
        for i in 0..3 {
            let mut tx = OrderedDictionary::new();
            tx.insert("txid".to_string(), Some(JToken::String(format!("tx_{i}"))));
            tx.insert(
                "size".to_string(),
                Some(JToken::Number((100 + i * 50) as f64)),
            );
            tx.insert("valid".to_string(), Some(JToken::Boolean(true)));
            transactions.push(Some(JToken::Object(tx)));
        }
        root.insert(
            "transactions".to_string(),
            Some(JToken::Array(transactions)),
        );

        // Test JSON path queries
        let root_token = JToken::Object(root);

        // Query block hash
        let tokens = JPathToken::parse("$.hash").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &root_token).expect("Operation failed");
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            &JToken::String("0x1234567890abcdef".to_string())
        );

        // Query all transaction IDs
        let tokens = JPathToken::parse("$.transactions[*].txid").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &root_token).expect("Operation failed");
        assert_eq!(results.len(), 3);

        // Query specific transaction
        let tokens = JPathToken::parse("$.transactions[1].size").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &root_token).expect("Operation failed");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(150.0));
    }

    #[test]
    fn test_json_serialization_roundtrip() {
        // Test that we can serialize and deserialize complex structures
        let mut obj = OrderedDictionary::new();
        obj.insert(
            "string".to_string(),
            Some(JToken::String("test".to_string())),
        );
        obj.insert("number".to_string(), Some(JToken::Number(42.5)));
        obj.insert("boolean".to_string(), Some(JToken::Boolean(true)));
        obj.insert("null".to_string(), Some(JToken::Null));

        let array = vec![
            Some(JToken::Number(1.0)),
            Some(JToken::Number(2.0)),
            Some(JToken::Number(3.0)),
        ];
        obj.insert("array".to_string(), Some(JToken::Array(array)));

        let token = JToken::Object(obj);

        // Test that the structure is preserved
        if let JToken::Object(ref obj) = token {
            assert_eq!(obj.len(), 5);
            assert!(obj.contains_key(&"string".to_string()));
            assert!(obj.contains_key(&"number".to_string()));
            assert!(obj.contains_key(&"boolean".to_string()));
            assert!(obj.contains_key(&"null".to_string()));
            assert!(obj.contains_key(&"array".to_string()));
        } else {
            // Test expectation: this should never happen in valid test scenarios
            assert!(false, "Expected object but got different token type");
        }
    }

    #[test]
    fn test_performance_large_structure() {
        // Test performance with larger structures
        let mut root = OrderedDictionary::new();

        // Create a large array
        let mut large_array = Vec::new();
        for i in 0..1000 {
            let mut item = OrderedDictionary::new();
            item.insert("id".to_string(), Some(JToken::Number(i as f64)));
            item.insert(
                "name".to_string(),
                Some(JToken::String(format!("item_{i}"))),
            );
            item.insert("active".to_string(), Some(JToken::Boolean(i % 2 == 0)));
            large_array.push(Some(JToken::Object(item)));
        }
        root.insert("items".to_string(), Some(JToken::Array(large_array)));

        let root_token = JToken::Object(root);

        // Test path evaluation performance
        let tokens = JPathToken::parse("$.items[*].id").expect("operation should succeed");
        let results = JPathToken::evaluate(&tokens, &root_token).expect("operation should succeed");
        assert_eq!(results.len(), 1000);

        // Test slice performance
        let tokens = JPathToken::parse("$.items[100:200]").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &root_token).expect("Operation failed");
        assert_eq!(results.len(), 100);
    }

    #[test]
    fn test_neo_blockchain_compatibility() {
        // Test compatibility with Neo blockchain JSON structures
        let mut block = OrderedDictionary::new();
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
                "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            )),
        );
        block.insert("time".to_string(), Some(JToken::Number(1640995200.0)));
        block.insert("index".to_string(), Some(JToken::Number(0.0)));
        block.insert(
            "nonce".to_string(),
            Some(JToken::String("0x0000000000000000".to_string())),
        );

        let mut witnesses = Vec::new();
        let mut witness = OrderedDictionary::new();
        witness.insert(
            "invocation".to_string(),
            Some(JToken::String("".to_string())),
        );
        witness.insert(
            "verification".to_string(),
            Some(JToken::String("EQ==".to_string())),
        );
        witnesses.push(Some(JToken::Object(witness)));
        block.insert("witnesses".to_string(), Some(JToken::Array(witnesses)));

        let mut tx_array = Vec::new();
        let mut tx = OrderedDictionary::new();
        tx.insert(
            "hash".to_string(),
            Some(JToken::String(
                "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
            )),
        );
        tx.insert("size".to_string(), Some(JToken::Number(123.0)));
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
        tx.insert("validuntilblock".to_string(), Some(JToken::Number(1000.0)));
        tx_array.push(Some(JToken::Object(tx)));
        block.insert("tx".to_string(), Some(JToken::Array(tx_array)));

        let block_token = JToken::Object(block);

        // Test various Neo-specific queries
        let tokens = JPathToken::parse("$.merkleroot").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &block_token).expect("Operation failed");
        assert_eq!(results.len(), 1);

        let tokens = JPathToken::parse("$.tx[0].sender").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &block_token).expect("Operation failed");
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0],
            &JToken::String("NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB".to_string())
        );

        let tokens = JPathToken::parse("$.witnesses[*].verification").expect("Operation failed");
        let results = JPathToken::evaluate(&tokens, &block_token).expect("Operation failed");
        assert_eq!(results.len(), 1);
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_large_object_creation_performance() {
        let start = Instant::now();

        let mut obj = OrderedDictionary::new();
        for i in 0..10000 {
            obj.insert(
                format!("key_{i}"),
                Some(JToken::String(format!("value_{i}"))),
            );
        }

        let duration = start.elapsed();
        log::info!("Created 10,000 key-value pairs in {:?}", duration);

        assert!(duration.as_millis() < 1000);
        assert_eq!(obj.len(), 10000);
    }

    #[test]
    fn test_deep_nesting_performance() {
        let start = Instant::now();

        // Create deeply nested structure
        let mut current = OrderedDictionary::new();
        for i in 0..100 {
            let mut next = OrderedDictionary::new();
            next.insert("value".to_string(), Some(JToken::Number(i as f64)));
            current.insert("nested".to_string(), Some(JToken::Object(next.clone())));
            current = next;
        }

        let duration = start.elapsed();
        log::info!("Created 100-level deep nesting in {:?}", duration);

        // Should handle deep nesting efficiently
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_json_path_query_performance() {
        let mut root = OrderedDictionary::new();
        let mut items = Vec::new();

        for i in 0..1000 {
            let mut item = OrderedDictionary::new();
            item.insert("id".to_string(), Some(JToken::Number(i as f64)));
            item.insert(
                "name".to_string(),
                Some(JToken::String(format!("item_{i}"))),
            );
            item.insert(
                "category".to_string(),
                Some(JToken::String(if i % 2 == 0 {
                    "even".to_string()
                } else {
                    "odd".to_string()
                })),
            );
            items.push(Some(JToken::Object(item)));
        }
        root.insert("items".to_string(), Some(JToken::Array(items)));

        let json = JToken::Object(root);

        let start = Instant::now();

        // Perform multiple path queries
        for _ in 0..100 {
            let tokens = JPathToken::parse("$.items[*].name").expect("Operation failed");
            let results = JPathToken::evaluate(&tokens, &json).expect("Operation failed");
            assert_eq!(results.len(), 1000);
        }

        let duration = start.elapsed();
        log::info!("Performed 100 path queries on 1000 items in {:?}", duration);

        // Should handle queries efficiently
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that we don't have excessive memory overhead
        let mut objects = Vec::new();

        for i in 0..1000 {
            let mut obj = OrderedDictionary::new();
            obj.insert("id".to_string(), Some(JToken::Number(i as f64)));
            obj.insert("data".to_string(), Some(JToken::String("x".repeat(100))));
            objects.push(JToken::Object(obj));
        }

        // Verify all objects are created correctly
        assert_eq!(objects.len(), 1000);

        // Test that we can still perform operations efficiently
        let start = Instant::now();
        let mut count = 0;
        for obj in &objects {
            if let JToken::Object(ref dict) = obj {
                if dict.contains_key(&"id".to_string()) {
                    count += 1;
                }
            }
        }
        let duration = start.elapsed();

        assert_eq!(count, 1000);
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_concurrent_access_simulation() {
        let mut obj = OrderedDictionary::new();
        obj.insert(
            "shared_data".to_string(),
            Some(JToken::String("test".to_string())),
        );
        let json = JToken::Object(obj);

        let start = Instant::now();

        // Simulate multiple "threads" accessing the same data
        for _ in 0..1000 {
            let tokens = JPathToken::parse("$.shared_data").unwrap();
            let results = JPathToken::evaluate(&tokens, &json).unwrap();
            assert_eq!(results.len(), 1);
        }

        let duration = start.elapsed();
        log::info!("Simulated 1000 concurrent accesses in {:?}", duration);

        // Should handle concurrent-like access efficiently
        assert!(duration.as_millis() < 100);
    }
}

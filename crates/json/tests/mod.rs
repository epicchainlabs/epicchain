//! JSON Module C# Compatibility Test Suite
//!
//! This module contains comprehensive tests that ensure full compatibility
//! with C# Neo.Json functionality including JToken, JObject, JArray, and
//! JSON path operations.

mod jarray_tests;
mod jobject_tests;
mod jpath_tests;
mod jstring_comprehensive_tests;
mod jtoken_tests;
mod serialization_tests;

mod integration_tests {
    use neo_json::*;

    /// Test complete JSON workflow (matches C# Neo.Json usage patterns exactly)
    #[test]
    fn test_complete_json_workflow() {
        // Simulate complete JSON workflow that matches C# Neo.Json usage

        // 1. Create JSON object
        let mut root = OrderedDictionary::new();
        root.insert("version".to_string(), Some(JToken::Number(3.0)));
        root.insert(
            "network".to_string(),
            Some(JToken::String("mainnet".to_string())),
        );
        root.insert("active".to_string(), Some(JToken::Boolean(true)));

        // 2. Create nested object
        let mut config = OrderedDictionary::new();
        config.insert("timeout".to_string(), Some(JToken::Number(30000.0)));
        config.insert("retries".to_string(), Some(JToken::Number(3.0)));
        root.insert("config".to_string(), Some(JToken::Object(config)));

        // 3. Create array
        let nodes = vec![
            Some(JToken::String("node1.neo.org".to_string())),
            Some(JToken::String("node2.neo.org".to_string())),
            Some(JToken::String("node3.neo.org".to_string())),
        ];
        root.insert("nodes".to_string(), Some(JToken::Array(nodes)));

        let json = JToken::Object(root);

        // 4. Test JSON path queries
        let tokens = JPathToken::parse("$.version").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(3.0));

        // 5. Test nested property access
        let tokens = JPathToken::parse("$.config.timeout").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(30000.0));

        // 6. Test array access
        let tokens = JPathToken::parse("$.nodes[1]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::String("node2.neo.org".to_string()));

        // 7. Test wildcard access
        let tokens = JPathToken::parse("$.nodes[*]").unwrap();
        let results = JPathToken::evaluate(&tokens, &json).unwrap();
        assert_eq!(results.len(), 3);
    }

    /// Test Neo blockchain JSON structure compatibility (matches C# Neo blockchain JSON exactly)
    #[test]
    fn test_neo_blockchain_json_compatibility() {
        // Create Neo blockchain-style JSON structure
        let mut block = OrderedDictionary::new();

        // Block header
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
                "0x0000000000000000000000000000000000000000".to_string(),
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
        block.insert("speaker".to_string(), Some(JToken::Number(0.0)));

        // Consensus data
        let mut consensus = OrderedDictionary::new();
        consensus.insert("primary".to_string(), Some(JToken::Number(0.0)));
        consensus.insert(
            "nonce".to_string(),
            Some(JToken::String("0x1234567890abcdef".to_string())),
        );
        block.insert("consensus".to_string(), Some(JToken::Object(consensus)));

        // Witnesses array
        let mut witnesses = Vec::new();
        for i in 0..7 {
            let mut witness = OrderedDictionary::new();
            witness.insert(
                "invocation".to_string(),
                Some(JToken::String(format!("DEA1{i:02x}"))),
            );
            witness.insert(
                "verification".to_string(),
                Some(JToken::String(
                    "EQwhA/HsPB4oPogN5unEifDyfBkAfFM4WqpMDJF8MgB57a3yEQtBMHOzuw==".to_string(),
                )),
            );
            witnesses.push(Some(JToken::Object(witness)));
        }
        block.insert("witnesses".to_string(), Some(JToken::Array(witnesses)));

        // Transactions array
        let mut transactions = Vec::new();
        for i in 0..3 {
            let mut tx = OrderedDictionary::new();
            tx.insert(
                "hash".to_string(),
                Some(JToken::String(format!("0x{i:064x}"))),
            );
            tx.insert(
                "size".to_string(),
                Some(JToken::Number((250 + i * 50) as f64)),
            );
            tx.insert("version".to_string(), Some(JToken::Number(0.0)));
            tx.insert(
                "nonce".to_string(),
                Some(JToken::Number((123456789 + i * 1000) as f64)),
            );
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
            tx.insert(
                "validuntilblock".to_string(),
                Some(JToken::Number((12345 + 100) as f64)),
            );

            // Signers array
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

            // Script
            tx.insert(
                "script".to_string(),
                Some(JToken::String(
                    "VwEBEEEfLnsHEVqNG0wJD/////8AAAAVZ5TKjiAM1w==".to_string(),
                )),
            );

            transactions.push(Some(JToken::Object(tx)));
        }
        block.insert("tx".to_string(), Some(JToken::Array(transactions)));

        let block_token = JToken::Object(block);

        // Test various Neo blockchain queries
        let tokens = JPathToken::parse("$.hash").unwrap();
        let results = JPathToken::evaluate(&tokens, &block_token).unwrap();
        assert_eq!(results.len(), 1);

        let tokens = JPathToken::parse("$.tx[*].sender").unwrap();
        let results = JPathToken::evaluate(&tokens, &block_token).unwrap();
        assert_eq!(results.len(), 3);

        let tokens = JPathToken::parse("$.witnesses[*].verification").unwrap();
        let results = JPathToken::evaluate(&tokens, &block_token).unwrap();
        assert_eq!(results.len(), 7);

        let tokens = JPathToken::parse("$.consensus.primary").unwrap();
        let results = JPathToken::evaluate(&tokens, &block_token).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &JToken::Number(0.0));
    }

    /// Test RPC response JSON compatibility (matches C# Neo RPC responses exactly)
    #[test]
    fn test_rpc_response_json_compatibility() {
        // Create JSON-RPC response structure
        let mut response = OrderedDictionary::new();
        response.insert(
            "jsonrpc".to_string(),
            Some(JToken::String("2.0".to_string())),
        );
        response.insert("id".to_string(), Some(JToken::Number(1.0)));

        let mut result = OrderedDictionary::new();
        result.insert(
            "blockhash".to_string(),
            Some(JToken::String(
                "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            )),
        );
        result.insert("confirmations".to_string(), Some(JToken::Number(100.0)));
        result.insert("blocktime".to_string(), Some(JToken::Number(1640995200.0)));

        // VIn array
        let mut vin = Vec::new();
        let mut vin_item = OrderedDictionary::new();
        vin_item.insert(
            "txid".to_string(),
            Some(JToken::String(
                "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
            )),
        );
        vin_item.insert("vout".to_string(), Some(JToken::Number(0.0)));
        vin.push(Some(JToken::Object(vin_item)));
        result.insert("vin".to_string(), Some(JToken::Array(vin)));

        // VOut array
        let mut vout = Vec::new();
        for i in 0..2 {
            let mut vout_item = OrderedDictionary::new();
            vout_item.insert("n".to_string(), Some(JToken::Number(i as f64)));
            vout_item.insert(
                "asset".to_string(),
                Some(JToken::String(
                    "0xd2c270ebfc2a1cdd3e470014a4dff7c091f699ec".to_string(),
                )),
            );
            vout_item.insert(
                "value".to_string(),
                Some(JToken::String("100000000".to_string())),
            );
            vout_item.insert(
                "address".to_string(),
                Some(JToken::String(
                    "NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB".to_string(),
                )),
            );
            vout.push(Some(JToken::Object(vout_item)));
        }
        result.insert("vout".to_string(), Some(JToken::Array(vout)));

        response.insert("result".to_string(), Some(JToken::Object(result)));

        let response_token = JToken::Object(response);

        // Test RPC response queries
        let tokens = JPathToken::parse("$.jsonrpc").unwrap();
        let results = JPathToken::evaluate(&tokens, &response_token).unwrap();
        assert_eq!(results[0], &JToken::String("2.0".to_string()));

        let tokens = JPathToken::parse("$.result.blockhash").unwrap();
        let results = JPathToken::evaluate(&tokens, &response_token).unwrap();
        assert_eq!(results.len(), 1);

        let tokens = JPathToken::parse("$.result.vout[*].value").unwrap();
        let results = JPathToken::evaluate(&tokens, &response_token).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], &JToken::String("100000000".to_string()));
    }

    /// Test error response JSON compatibility (matches C# error handling exactly)
    #[test]
    fn test_error_response_json_compatibility() {
        // Create JSON-RPC error response
        let mut error_response = OrderedDictionary::new();
        error_response.insert(
            "jsonrpc".to_string(),
            Some(JToken::String("2.0".to_string())),
        );
        error_response.insert("id".to_string(), Some(JToken::Number(1.0)));

        let mut error = OrderedDictionary::new();
        error.insert("code".to_string(), Some(JToken::Number(-32602.0)));
        error.insert(
            "message".to_string(),
            Some(JToken::String("Invalid params".to_string())),
        );

        let mut error_data = OrderedDictionary::new();
        error_data.insert(
            "parameter".to_string(),
            Some(JToken::String("block_index".to_string())),
        );
        error_data.insert(
            "expected".to_string(),
            Some(JToken::String("number".to_string())),
        );
        error_data.insert(
            "actual".to_string(),
            Some(JToken::String("string".to_string())),
        );
        error.insert("data".to_string(), Some(JToken::Object(error_data)));

        error_response.insert("error".to_string(), Some(JToken::Object(error)));

        let error_token = JToken::Object(error_response);

        // Test error response queries
        let tokens = JPathToken::parse("$.error.code").unwrap();
        let results = JPathToken::evaluate(&tokens, &error_token).unwrap();
        assert_eq!(results[0], &JToken::Number(-32602.0));

        let tokens = JPathToken::parse("$.error.data.parameter").unwrap();
        let results = JPathToken::evaluate(&tokens, &error_token).unwrap();
        assert_eq!(results[0], &JToken::String("block_index".to_string()));
    }
}

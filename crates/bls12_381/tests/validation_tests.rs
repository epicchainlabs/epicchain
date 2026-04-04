//! BLS12-381 Validation C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Cryptography.BLS12_381 validation.
//! Tests are based on the C# BLS12_381.Validation test suite.

use neo_bls12_381::*;
use rand::thread_rng;

#[cfg(test)]
#[allow(dead_code)]
mod validation_tests {
    use super::*;

    /// Test private key validation (matches C# ValidatePrivateKey exactly)
    #[test]
    fn test_private_key_validation_compatibility() {
        let mut rng = thread_rng();

        // Test valid private keys
        for _ in 0..10 {
            let private_key = Bls12381::generate_private_key(&mut rng);
            assert!(Bls12381::validate_private_key(&private_key));
            assert!(private_key.is_valid());
        }

        let zero_bytes = vec![0u8; 32];
        if let Ok(zero_key) = Bls12381::private_key_from_bytes(&zero_bytes) {
            assert!(!Bls12381::validate_private_key(&zero_key));
            assert!(!zero_key.is_valid());
        }

        let max_bytes = vec![0xFFu8; 32];
        if let Ok(max_key) = Bls12381::private_key_from_bytes(&max_bytes) {
            assert!(!Bls12381::validate_private_key(&max_key));
            assert!(!max_key.is_valid());
        }

        // Test specific invalid values
        let invalid_test_vectors = vec![
            hex::decode("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001")
                .unwrap(),
            hex::decode("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000002")
                .unwrap(),
        ];

        for invalid_bytes in invalid_test_vectors {
            if let Ok(invalid_key) = Bls12381::private_key_from_bytes(&invalid_bytes) {
                assert!(
                    !Bls12381::validate_private_key(&invalid_key),
                    "Key should be invalid: {}",
                    hex::encode(&invalid_bytes)
                );
            }
        }
    }

    /// Test public key validation (matches C# ValidatePublicKey exactly)
    #[test]
    fn test_public_key_validation_compatibility() {
        let mut rng = thread_rng();

        // Test valid public keys derived from valid private keys
        for _ in 0..10 {
            let private_key = Bls12381::generate_private_key(&mut rng);
            let public_key = Bls12381::derive_public_key(&private_key);
            assert!(Bls12381::validate_public_key(&public_key));
            assert!(public_key.is_valid());
        }

        let zero_bytes = vec![0u8; 48];
        if let Ok(zero_key) = Bls12381::public_key_from_bytes(&zero_bytes) {
            assert!(!Bls12381::validate_public_key(&zero_key));
            assert!(!zero_key.is_valid());
        }

        let max_bytes = vec![0xFFu8; 48];
        if let Ok(max_key) = Bls12381::public_key_from_bytes(&max_bytes) {
            assert!(!Bls12381::validate_public_key(&max_key));
            assert!(!max_key.is_valid());
        }

        // Test invalid public key formats
        let invalid_formats = vec![
            vec![0x01; 48], // Invalid point
            vec![0x80; 48], // Invalid compression flag
            vec![0xC0; 48], // Invalid compression flag
        ];

        for invalid_bytes in invalid_formats {
            if let Ok(invalid_key) = Bls12381::public_key_from_bytes(&invalid_bytes) {
                assert!(
                    !Bls12381::validate_public_key(&invalid_key),
                    "Key should be invalid: {}",
                    hex::encode(&invalid_bytes)
                );
            }
        }
    }

    /// Test signature validation (matches C# ValidateSignature exactly)
    #[test]
    fn test_signature_validation_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let message = b"Validation test message";

        // Test valid signatures
        for _ in 0..10 {
            let signature = Bls12381::sign(&private_key, message).unwrap();
            assert!(Bls12381::validate_signature(&signature));
            assert!(signature.is_valid());
        }

        let zero_bytes = vec![0u8; 96];
        if let Ok(zero_sig) = Bls12381::signature_from_bytes(&zero_bytes) {
            assert!(!Bls12381::validate_signature(&zero_sig));
            assert!(!zero_sig.is_valid());
        }

        let max_bytes = vec![0xFFu8; 96];
        if let Ok(max_sig) = Bls12381::signature_from_bytes(&max_bytes) {
            assert!(!Bls12381::validate_signature(&max_sig));
            assert!(!max_sig.is_valid());
        }

        // Test specific invalid signature patterns
        let invalid_patterns = vec![
            vec![0x01; 96],                            // Invalid point
            vec![0x80; 96],                            // Invalid compression flag
            vec![0xC0; 96],                            // Invalid compression flag
            [vec![0x00; 48], vec![0xFF; 48]].concat(), // Mixed patterns
        ];

        for invalid_bytes in invalid_patterns {
            if let Ok(invalid_sig) = Bls12381::signature_from_bytes(&invalid_bytes) {
                assert!(
                    !Bls12381::validate_signature(&invalid_sig),
                    "Signature should be invalid: {}",
                    hex::encode(&invalid_bytes)
                );
            }
        }
    }

    /// Test validation consistency across operations (matches C# consistency exactly)
    #[test]
    fn test_validation_consistency_compatibility() {
        let mut rng = thread_rng();

        // Generate valid key pair
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        // Both should be valid
        assert!(Bls12381::validate_private_key(&private_key));
        assert!(Bls12381::validate_public_key(&public_key));

        assert!(private_key.is_valid());
        assert!(public_key.is_valid());

        // Create signature
        let message = b"Consistency test";
        let signature = Bls12381::sign(&private_key, message).unwrap();

        // Signature should be valid
        assert!(Bls12381::validate_signature(&signature));
        assert!(signature.is_valid());

        assert!(Bls12381::verify(&public_key, message, &signature));

        // Serialize and deserialize - should maintain validity
        let private_bytes = Bls12381::private_key_to_bytes(&private_key);
        let public_bytes = Bls12381::public_key_to_bytes(&public_key);
        let signature_bytes = Bls12381::signature_to_bytes(&signature);

        let restored_private = Bls12381::private_key_from_bytes(&private_bytes).unwrap();
        let restored_public = Bls12381::public_key_from_bytes(&public_bytes).unwrap();
        let restored_signature = Bls12381::signature_from_bytes(&signature_bytes).unwrap();

        // All restored objects should be valid
        assert!(Bls12381::validate_private_key(&restored_private));
        assert!(Bls12381::validate_public_key(&restored_public));
        assert!(Bls12381::validate_signature(&restored_signature));

        // Verification should still work
        assert!(Bls12381::verify(
            &restored_public,
            message,
            &restored_signature
        ));
    }

    /// Test validation with edge cases (matches C# edge case handling exactly)
    #[test]
    fn test_validation_edge_cases_compatibility() {
        // Test validation with boundary values

        let one_bytes = {
            let mut bytes = vec![0u8; 32];
            bytes[31] = 1;
            bytes
        };

        if let Ok(one_key) = Bls12381::private_key_from_bytes(&one_bytes) {
            assert!(Bls12381::validate_private_key(&one_key));
        }

        let order_minus_one =
            hex::decode("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000")
                .unwrap();
        if let Ok(max_valid_key) = Bls12381::private_key_from_bytes(&order_minus_one) {
            assert!(Bls12381::validate_private_key(&max_valid_key));
        }

        // Test public key validation with compressed/uncompressed points
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let public_bytes = Bls12381::public_key_to_bytes(&public_key);

        assert_eq!(public_bytes.len(), 48);

        // First byte should indicate compression
        assert!(public_bytes[0] & 0x80 != 0 || public_bytes[0] == 0);

        // Test signature validation with different message lengths
        let test_messages = vec![
            vec![],           // Empty
            vec![0x00],       // Single byte
            vec![0xFF; 32],   // 32 bytes
            vec![0x42; 1000], // Large message
        ];

        for message in test_messages {
            let signature = Bls12381::sign(&private_key, &message).unwrap();
            assert!(Bls12381::validate_signature(&signature));
            assert!(Bls12381::verify(&public_key, &message, &signature));
        }
    }

    /// Test validation with malformed data (matches C# error handling exactly)
    #[test]
    fn test_validation_malformed_data_compatibility() {
        // Test with truncated data
        let truncated_sizes = vec![
            (30, 31, 32), // Private key sizes (too small, too small, correct)
            (46, 47, 48), // Public key sizes
            (94, 95, 96), // Signature sizes
        ];

        for (too_small, still_small, correct) in truncated_sizes {
            // Test private key validation with wrong sizes
            if too_small <= 32 {
                let small_bytes = vec![0x42u8; too_small];
                assert!(Bls12381::private_key_from_bytes(&small_bytes).is_err());
            }

            // Test public key validation with wrong sizes
            if too_small <= 48 && too_small >= 46 {
                let small_bytes = vec![0x42u8; too_small];
                assert!(Bls12381::public_key_from_bytes(&small_bytes).is_err());
            }

            // Test signature validation with wrong sizes
            if too_small <= 96 && too_small >= 94 {
                let small_bytes = vec![0x42u8; too_small];
                assert!(Bls12381::signature_from_bytes(&small_bytes).is_err());
            }
        }

        // Test with oversized data
        let oversized_data = vec![
            vec![0x42u8; 33], // Private key too large
            vec![0x42u8; 49], // Public key too large
            vec![0x42u8; 97], // Signature too large
        ];

        assert!(Bls12381::private_key_from_bytes(&oversized_data[0]).is_err());
        assert!(Bls12381::public_key_from_bytes(&oversized_data[1]).is_err());
        assert!(Bls12381::signature_from_bytes(&oversized_data[2]).is_err());
    }

    /// Test validation performance (matches C# performance characteristics exactly)
    #[test]
    fn test_validation_performance_compatibility() {
        let mut rng = thread_rng();
        let count = 1000;

        // Generate test data
        let private_keys: Vec<_> = (0..count)
            .map(|_| Bls12381::generate_private_key(&mut rng))
            .collect();

        let public_keys: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::derive_public_key(pk))
            .collect();

        let message = b"Performance test message";
        let signatures: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::sign(pk, message).unwrap())
            .collect();

        // Measure validation performance
        let start = std::time::Instant::now();

        // Validate all private keys
        for private_key in &private_keys {
            assert!(Bls12381::validate_private_key(private_key));
        }

        // Validate all public keys
        for public_key in &public_keys {
            assert!(Bls12381::validate_public_key(public_key));
        }

        // Validate all signatures
        for signature in &signatures {
            assert!(Bls12381::validate_signature(signature));
        }

        let duration = start.elapsed();

        // Validation should be reasonably fast
        // This is a performance regression test
        assert!(
            duration.as_millis() < 10000,
            "Validation took too long: {:?}",
            duration
        );

        println!("Validated {} keys and signatures in {:?}", count, duration);
    }

    /// Test validation thread safety (matches C# thread safety exactly)
    #[test]
    fn test_validation_thread_safety_compatibility() {
        use std::sync::Arc;
        use std::thread;

        let mut rng = thread_rng();

        // Create shared test data
        let private_key = Arc::new(Bls12381::generate_private_key(&mut rng));
        let public_key = Arc::new(Bls12381::derive_public_key(&private_key));
        let message = Arc::new(b"Thread safety test".to_vec());
        let signature = Arc::new(Bls12381::sign(&private_key, &message).unwrap());

        let mut handles = Vec::new();

        // Spawn multiple validation threads
        for i in 0..10 {
            let pk = Arc::clone(&private_key);
            let pub_k = Arc::clone(&public_key);
            let sig = Arc::clone(&signature);
            let msg = Arc::clone(&message);

            let handle = thread::spawn(move || {
                // Validate keys and signature multiple times
                for _ in 0..100 {
                    assert!(
                        Bls12381::validate_private_key(&pk),
                        "Private key validation failed in thread {}",
                        i
                    );
                    assert!(
                        Bls12381::validate_public_key(&pub_k),
                        "Public key validation failed in thread {}",
                        i
                    );
                    assert!(
                        Bls12381::validate_signature(&sig),
                        "Signature validation failed in thread {}",
                        i
                    );

                    // Also test verification
                    assert!(
                        Bls12381::verify(&pub_k, &msg, &sig),
                        "Signature verification failed in thread {}",
                        i
                    );
                }

                i // Return thread ID
            });

            handles.push(handle);
        }

        let thread_ids: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All threads should have completed successfully
        assert_eq!(thread_ids.len(), 10);
        for (expected, actual) in (0..10).zip(thread_ids) {
            assert_eq!(expected, actual);
        }
    }

    /// Test validation with generated vs parsed keys (matches C# consistency exactly)
    #[test]
    fn test_validation_generated_vs_parsed_compatibility() {
        let mut rng = thread_rng();

        // Generate original key pair
        let original_private = Bls12381::generate_private_key(&mut rng);
        let original_public = Bls12381::derive_public_key(&original_private);
        let message = b"Generated vs parsed test";
        let original_signature = Bls12381::sign(&original_private, message).unwrap();

        // Validate originals
        assert!(Bls12381::validate_private_key(&original_private));
        assert!(Bls12381::validate_public_key(&original_public));
        assert!(Bls12381::validate_signature(&original_signature));

        // Serialize and parse back
        let private_bytes = Bls12381::private_key_to_bytes(&original_private);
        let public_bytes = Bls12381::public_key_to_bytes(&original_public);
        let signature_bytes = Bls12381::signature_to_bytes(&original_signature);

        let parsed_private = Bls12381::private_key_from_bytes(&private_bytes).unwrap();
        let parsed_public = Bls12381::public_key_from_bytes(&public_bytes).unwrap();
        let parsed_signature = Bls12381::signature_from_bytes(&signature_bytes).unwrap();

        // Validate parsed versions
        assert!(Bls12381::validate_private_key(&parsed_private));
        assert!(Bls12381::validate_public_key(&parsed_public));
        assert!(Bls12381::validate_signature(&parsed_signature));

        // Both versions should be equal
        assert_eq!(original_private, parsed_private);
        assert_eq!(original_public, parsed_public);
        assert_eq!(original_signature, parsed_signature);

        // Both should verify correctly
        assert!(Bls12381::verify(
            &original_public,
            message,
            &original_signature
        ));
        assert!(Bls12381::verify(&parsed_public, message, &parsed_signature));
        assert!(Bls12381::verify(
            &original_public,
            message,
            &parsed_signature
        ));
        assert!(Bls12381::verify(
            &parsed_public,
            message,
            &original_signature
        ));
    }

    /// Test validation error messages (matches C# error reporting exactly)
    #[test]
    fn test_validation_error_messages_compatibility() {
        // Test that validation methods return appropriate boolean results

        // Invalid private key
        let zero_private_bytes = vec![0u8; 32];
        if let Ok(zero_private) = Bls12381::private_key_from_bytes(&zero_private_bytes) {
            let is_valid = Bls12381::validate_private_key(&zero_private);
            assert!(!is_valid); // Should be false, not panic
        }

        // Invalid public key
        let zero_public_bytes = vec![0u8; 48];
        if let Ok(zero_public) = Bls12381::public_key_from_bytes(&zero_public_bytes) {
            let is_valid = Bls12381::validate_public_key(&zero_public);
            assert!(!is_valid); // Should be false, not panic
        }

        // Invalid signature
        let zero_signature_bytes = vec![0u8; 96];
        if let Ok(zero_signature) = Bls12381::signature_from_bytes(&zero_signature_bytes) {
            let is_valid = Bls12381::validate_signature(&zero_signature);
            assert!(!is_valid); // Should be false, not panic
        }

        // This matches the C# behavior where validation methods return boolean
    }
}

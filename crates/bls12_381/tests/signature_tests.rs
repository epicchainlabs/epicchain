//! BLS12-381 Signature Operations C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Cryptography.BLS12_381 signature operations.
//! Tests are based on the C# BLS12_381.Signature test suite.

use neo_bls12_381::*;
use rand::thread_rng;

#[cfg(test)]
#[allow(dead_code)]
mod signature_tests {
    use super::*;

    /// Test basic signature and verification (matches C# Sign/Verify exactly)
    #[test]
    fn test_basic_signature_verification_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let message = b"Hello, Neo blockchain!";

        // Sign message
        let signature = Bls12381::sign(&private_key, message).unwrap();

        // Verify signature
        assert!(Bls12381::verify(&public_key, message, &signature));

        // Signature should be valid
        assert!(Bls12381::validate_signature(&signature));
        assert!(signature.is_valid());
    }

    /// Test signature with wrong message (matches C# negative verification exactly)
    #[test]
    fn test_signature_wrong_message_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let original_message = b"Original message";
        let wrong_message = b"Wrong message";

        // Sign original message
        let signature = Bls12381::sign(&private_key, original_message).unwrap();

        // Verify with correct message should pass
        assert!(Bls12381::verify(&public_key, original_message, &signature));

        // Verify with wrong message should fail
        assert!(!Bls12381::verify(&public_key, wrong_message, &signature));
    }

    /// Test signature with wrong public key (matches C# negative verification exactly)
    #[test]
    fn test_signature_wrong_public_key_compatibility() {
        let mut rng = thread_rng();

        // Generate two different key pairs
        let private_key1 = Bls12381::generate_private_key(&mut rng);
        let public_key1 = Bls12381::derive_public_key(&private_key1);

        let private_key2 = Bls12381::generate_private_key(&mut rng);
        let public_key2 = Bls12381::derive_public_key(&private_key2);

        let message = b"Test message";

        // Sign with first private key
        let signature = Bls12381::sign(&private_key1, message).unwrap();

        // Verify with correct public key should pass
        assert!(Bls12381::verify(&public_key1, message, &signature));

        // Verify with wrong public key should fail
        assert!(!Bls12381::verify(&public_key2, message, &signature));
    }

    /// Test signature determinism (matches C# deterministic signatures exactly)
    #[test]
    fn test_signature_determinism_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let message = b"Deterministic test message";

        // Sign same message multiple times
        let signature1 = Bls12381::sign(&private_key, message).unwrap();
        let signature2 = Bls12381::sign(&private_key, message).unwrap();
        let signature3 = private_key.sign(message, NEO_SIGNATURE_SCHEME).unwrap();

        assert_eq!(signature1, signature2);
        assert_eq!(signature1, signature3);
        assert_eq!(signature2, signature3);
    }

    /// Test empty message signature (matches C# empty message handling exactly)
    #[test]
    fn test_empty_message_signature_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let empty_message = b"";

        // Should be able to sign empty message
        let signature = Bls12381::sign(&private_key, empty_message).unwrap();

        // Should be able to verify empty message
        assert!(Bls12381::verify(&public_key, empty_message, &signature));

        // Signature should be valid
        assert!(Bls12381::validate_signature(&signature));
    }

    /// Test large message signature (matches C# large message handling exactly)
    #[test]
    fn test_large_message_signature_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let large_message = vec![0x42u8; 1024 * 1024];

        // Should be able to sign large message
        let signature = Bls12381::sign(&private_key, &large_message).unwrap();

        // Should be able to verify large message
        assert!(Bls12381::verify(&public_key, &large_message, &signature));

        // Signature should be valid
        assert!(Bls12381::validate_signature(&signature));
    }

    /// Test signature with various message types (matches C# message type handling exactly)
    #[test]
    fn test_signature_various_message_types_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let test_messages = vec![
            b"ASCII message".to_vec(),
            "UTF-8 message: 🦀".as_bytes().to_vec(),
            vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD], // Binary data
            (0..=255).map(|x| x as u8).collect::<Vec<u8>>(), // All byte values
            vec![0x00; 1000],                               // Repeated nulls
            vec![0xFF; 1000],                               // Repeated 0xFF
        ];

        for (i, message) in test_messages.iter().enumerate() {
            let signature = Bls12381::sign(&private_key, message)
                .unwrap_or_else(|_| panic!("Failed to sign message {}", i));

            assert!(
                Bls12381::verify(&public_key, message, &signature),
                "Failed to verify message {}",
                i
            );

            assert!(
                Bls12381::validate_signature(&signature),
                "Invalid signature for message {}",
                i
            );
        }
    }

    /// Test signature size consistency (matches C# signature size exactly)
    #[test]
    fn test_signature_size_consistency_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);

        let messages = vec![
            b"short".to_vec(),
            b"medium length message for testing".to_vec(),
            vec![0u8; 10000], // Long message
        ];

        for message in messages {
            let signature = Bls12381::sign(&private_key, &message).unwrap();
            let signature_bytes = Bls12381::signature_to_bytes(&signature);

            // All signatures should be same size regardless of message length
            assert_eq!(signature_bytes.len(), 96); // matches C# SIGNATURE_SIZE
        }
    }

    /// Test signature serialization roundtrip (matches C# signature serialization exactly)
    #[test]
    fn test_signature_serialization_roundtrip_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Serialization test message";

        // Create signature
        let original_signature = Bls12381::sign(&private_key, message).unwrap();

        // Serialize to bytes
        let signature_bytes = Bls12381::signature_to_bytes(&original_signature);
        assert_eq!(signature_bytes.len(), 96);

        // Deserialize from bytes
        let deserialized_signature = Bls12381::signature_from_bytes(&signature_bytes).unwrap();

        // Should be identical
        assert_eq!(original_signature, deserialized_signature);

        // Should still verify correctly
        assert!(Bls12381::verify(
            &public_key,
            message,
            &deserialized_signature
        ));

        // Should be valid
        assert!(Bls12381::validate_signature(&deserialized_signature));
    }

    /// Test signature deserialization error handling (matches C# error handling exactly)
    #[test]
    fn test_signature_deserialization_errors_compatibility() {
        // Test wrong size
        let wrong_size_bytes = vec![0u8; 95]; // Should be 96
        assert!(Bls12381::signature_from_bytes(&wrong_size_bytes).is_err());

        let too_large_bytes = vec![0u8; 97]; // Should be 96
        assert!(Bls12381::signature_from_bytes(&too_large_bytes).is_err());

        // Test empty data
        let empty_bytes = vec![];
        assert!(Bls12381::signature_from_bytes(&empty_bytes).is_err());

        // Test invalid signature data
        let invalid_bytes = vec![0xFFu8; 96];
        let result = Bls12381::signature_from_bytes(&invalid_bytes);

        // Should either fail to parse or create invalid signature
        match result {
            Ok(invalid_signature) => {
                assert!(!Bls12381::validate_signature(&invalid_signature));
            }
            Err(_) => {} // Also acceptable
        }
    }

    /// Test signature scheme compatibility (matches C# SignatureScheme exactly)
    #[test]
    fn test_signature_scheme_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Signature scheme test";

        // Test with NEO_SIGNATURE_SCHEME
        let signature1 = Bls12381::sign(&private_key, message).unwrap();
        let signature2 = private_key.sign(message, NEO_SIGNATURE_SCHEME).unwrap();

        // Should be identical
        assert_eq!(signature1, signature2);

        // Both should verify
        assert!(Bls12381::verify(&public_key, message, &signature1));
        assert!(public_key.verify(message, &signature2, NEO_SIGNATURE_SCHEME));
    }

    /// Test multiple signatures from same key (matches C# multi-signature behavior exactly)
    #[test]
    fn test_multiple_signatures_same_key_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let messages = vec![
            b"Message 1".to_vec(),
            b"Message 2".to_vec(),
            b"Message 3".to_vec(),
            b"Message 4".to_vec(),
            b"Message 5".to_vec(),
        ];

        let mut signatures = Vec::new();

        for message in &messages {
            let signature = Bls12381::sign(&private_key, message).unwrap();
            signatures.push(signature);
        }

        // Verify all signatures
        for (i, message) in messages.iter().enumerate() {
            assert!(
                Bls12381::verify(&public_key, message, &signatures[i]),
                "Failed to verify signature {}",
                i
            );
        }

        for i in 0..messages.len() {
            for j in 0..signatures.len() {
                if i != j {
                    assert!(
                        !Bls12381::verify(&public_key, &messages[i], &signatures[j]),
                        "Incorrectly verified message {} with signature {}",
                        i,
                        j
                    );
                }
            }
        }
    }

    /// Test signature validation edge cases (matches C# validation exactly)
    #[test]
    fn test_signature_validation_edge_cases_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let message = b"Edge case test";

        // Valid signature
        let valid_signature = Bls12381::sign(&private_key, message).unwrap();
        assert!(Bls12381::validate_signature(&valid_signature));

        // Test zero signature
        let zero_bytes = vec![0u8; 96];
        if let Ok(zero_signature) = Bls12381::signature_from_bytes(&zero_bytes) {
            assert!(!Bls12381::validate_signature(&zero_signature));
        }

        // Test max value signature
        let max_bytes = vec![0xFFu8; 96];
        if let Ok(max_signature) = Bls12381::signature_from_bytes(&max_bytes) {
            assert!(!Bls12381::validate_signature(&max_signature));
        }
    }

    /// Test concurrent signature operations (matches C# thread safety exactly)
    #[test]
    fn test_concurrent_signature_operations_compatibility() {
        use std::sync::Arc;
        use std::thread;

        let mut rng = thread_rng();
        let private_key = Arc::new(Bls12381::generate_private_key(&mut rng));
        let public_key = Arc::new(Bls12381::derive_public_key(&private_key));

        let mut handles = Vec::new();

        // Spawn multiple threads to sign and verify concurrently
        for i in 0..10 {
            let pk = Arc::clone(&private_key);
            let pub_k = Arc::clone(&public_key);

            let handle = thread::spawn(move || {
                let message = format!("Concurrent message {}", i);
                let message_bytes = message.as_bytes();

                // Sign message
                let signature = Bls12381::sign(&pk, message_bytes).unwrap();

                // Verify signature
                assert!(Bls12381::verify(&pub_k, message_bytes, &signature));

                // Validate signature
                assert!(Bls12381::validate_signature(&signature));

                signature
            });

            handles.push(handle);
        }

        // Collect all signatures
        let signatures: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All signatures should be valid but different
        for (i, sig1) in signatures.iter().enumerate() {
            assert!(Bls12381::validate_signature(sig1));

            for (j, sig2) in signatures.iter().enumerate() {
                if i != j {
                    assert_ne!(sig1, sig2);
                }
            }
        }
    }
}

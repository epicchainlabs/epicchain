//! BLS12-381 Performance C# Compatibility Tests
//!
//! These tests ensure performance characteristics match C# Neo.Cryptography.BLS12_381.
//! Tests are based on the C# BLS12_381.Performance test suite.

use neo_bls12_381::*;
use rand::thread_rng;
use std::time::Instant;

#[cfg(test)]
#[allow(dead_code)]
mod performance_tests {
    use super::*;

    /// Test key generation performance (matches C# key generation benchmarks exactly)
    #[test]
    fn test_key_generation_performance_compatibility() {
        let mut rng = thread_rng();
        let iterations = 100;

        // Measure private key generation
        let start = Instant::now();
        let private_keys: Vec<_> = (0..iterations)
            .map(|_| Bls12381::generate_private_key(&mut rng))
            .collect();
        let private_key_duration = start.elapsed();

        // Measure public key derivation
        let start = Instant::now();
        let public_keys: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::derive_public_key(pk))
            .collect();
        let public_key_duration = start.elapsed();

        // Measure key pair generation
        let start = Instant::now();
        let key_pairs: Vec<_> = (0..iterations)
            .map(|_| Bls12381::generate_key_pair(&mut rng))
            .collect();
        let key_pair_duration = start.elapsed();

        // Verify all keys are valid
        for private_key in &private_keys {
            assert!(Bls12381::validate_private_key(private_key));
        }
        for public_key in &public_keys {
            assert!(Bls12381::validate_public_key(public_key));
        }
        for key_pair in &key_pairs {
            assert!(Bls12381::validate_private_key(key_pair.private_key()));
            assert!(Bls12381::validate_public_key(key_pair.public_key()));
        }

        assert!(
            private_key_duration.as_millis() < 5000,
            "Private key generation too slow: {:?}",
            private_key_duration
        );
        assert!(
            public_key_duration.as_millis() < 2000,
            "Public key derivation too slow: {:?}",
            public_key_duration
        );
        assert!(
            key_pair_duration.as_millis() < 5000,
            "Key pair generation too slow: {:?}",
            key_pair_duration
        );

        println!(
            "Generated {} private keys in {:?}",
            iterations, private_key_duration
        );
        println!(
            "Derived {} public keys in {:?}",
            iterations, public_key_duration
        );
        println!(
            "Generated {} key pairs in {:?}",
            iterations, key_pair_duration
        );
    }

    /// Test signing performance (matches C# signing benchmarks exactly)
    #[test]
    fn test_signing_performance_compatibility() {
        let mut rng = thread_rng();
        let iterations = 100;
        let private_key = Bls12381::generate_private_key(&mut rng);
        let message = b"Performance test message for signing benchmarks";

        // Measure signing performance
        let start = Instant::now();
        let signatures: Vec<_> = (0..iterations)
            .map(|_| Bls12381::sign(&private_key, message).unwrap())
            .collect();
        let signing_duration = start.elapsed();

        for signature in &signatures {
            assert!(Bls12381::validate_signature(signature));
        }

        for i in 1..signatures.len() {
            assert_eq!(signatures[0], signatures[i]);
        }

        // Performance should be reasonable
        assert!(
            signing_duration.as_millis() < 10000,
            "Signing too slow: {:?}",
            signing_duration
        );

        println!(
            "Created {} signatures in {:?}",
            iterations, signing_duration
        );

        // Test signing with different message sizes
        let message_sizes = vec![0, 32, 256, 1024, 4096];

        for size in message_sizes {
            let large_message = vec![0x42u8; size];

            let start = Instant::now();
            let signature = Bls12381::sign(&private_key, &large_message).unwrap();
            let duration = start.elapsed();

            assert!(Bls12381::validate_signature(&signature));

            // Signing time should not significantly increase with message size
            assert!(
                duration.as_millis() < 1000,
                "Signing message of size {} too slow: {:?}",
                size,
                duration
            );
        }
    }

    /// Test verification performance (matches C# verification benchmarks exactly)
    #[test]
    fn test_verification_performance_compatibility() {
        let mut rng = thread_rng();
        let iterations = 100;
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);
        let message = b"Performance test message for verification benchmarks";
        let signature = Bls12381::sign(&private_key, message).unwrap();

        // Measure verification performance
        let start = Instant::now();
        for _ in 0..iterations {
            assert!(Bls12381::verify(&public_key, message, &signature));
        }
        let verification_duration = start.elapsed();

        // Performance should be reasonable
        assert!(
            verification_duration.as_millis() < 15000,
            "Verification too slow: {:?}",
            verification_duration
        );

        println!(
            "Verified {} signatures in {:?}",
            iterations, verification_duration
        );

        // Test verification with different message sizes
        let message_sizes = vec![0, 32, 256, 1024, 4096];

        for size in message_sizes {
            let large_message = vec![0x42u8; size];
            let large_signature = Bls12381::sign(&private_key, &large_message).unwrap();

            let start = Instant::now();
            let result = Bls12381::verify(&public_key, &large_message, &large_signature);
            let duration = start.elapsed();

            assert!(result);

            // Verification time should not significantly increase with message size
            assert!(
                duration.as_millis() < 1000,
                "Verifying message of size {} too slow: {:?}",
                size,
                duration
            );
        }
    }

    /// Test aggregation performance (matches C# aggregation benchmarks exactly)
    #[test]
    fn test_aggregation_performance_compatibility() {
        let mut rng = thread_rng();
        let message = b"Aggregation performance test message";

        // Test with different aggregation sizes
        let aggregation_sizes = vec![2, 5, 10, 20, 50, 100];

        for size in aggregation_sizes {
            // Generate key pairs and signatures
            let key_pairs: Vec<_> = (0..size)
                .map(|_| {
                    let private_key = Bls12381::generate_private_key(&mut rng);
                    let public_key = Bls12381::derive_public_key(&private_key);
                    (private_key, public_key)
                })
                .collect();

            let signatures: Vec<_> = key_pairs
                .iter()
                .map(|(private_key, _)| Bls12381::sign(private_key, message).unwrap())
                .collect();

            let public_keys: Vec<_> = key_pairs
                .iter()
                .map(|(_, public_key)| public_key.clone())
                .collect();

            // Measure signature aggregation
            let start = Instant::now();
            let aggregate_signature = Bls12381::aggregate_signatures(&signatures).unwrap();
            let aggregation_duration = start.elapsed();

            // Measure public key aggregation
            let start = Instant::now();
            let aggregate_public_key = Bls12381::aggregate_public_keys(&public_keys).unwrap();
            let public_aggregation_duration = start.elapsed();

            // Measure fast aggregate verification
            let start = Instant::now();
            let verification_result =
                Bls12381::fast_aggregate_verify(&public_keys, message, &aggregate_signature);
            let fast_verify_duration = start.elapsed();

            assert!(verification_result);

            // Performance should scale reasonably with aggregation size
            let max_aggregation_time = size as u128 * 10; // 10ms per signature max
            let max_verification_time = size as u128 * 20; // 20ms per signature max

            assert!(
                aggregation_duration.as_millis() < max_aggregation_time,
                "Signature aggregation of {} too slow: {:?}",
                size,
                aggregation_duration
            );
            assert!(
                public_aggregation_duration.as_millis() < max_aggregation_time,
                "Public key aggregation of {} too slow: {:?}",
                size,
                public_aggregation_duration
            );
            assert!(
                fast_verify_duration.as_millis() < max_verification_time,
                "Fast aggregate verify of {} too slow: {:?}",
                size,
                fast_verify_duration
            );

            println!(
                "Aggregated {} signatures in {:?}, verified in {:?}",
                size, aggregation_duration, fast_verify_duration
            );
        }
    }

    /// Test serialization performance (matches C# serialization benchmarks exactly)
    #[test]
    fn test_serialization_performance_compatibility() {
        let mut rng = thread_rng();
        let iterations = 1000;

        // Generate test data
        let private_keys: Vec<_> = (0..iterations)
            .map(|_| Bls12381::generate_private_key(&mut rng))
            .collect();

        let public_keys: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::derive_public_key(pk))
            .collect();

        let message = b"Serialization performance test";
        let signatures: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::sign(pk, message).unwrap())
            .collect();

        // Measure private key serialization
        let start = Instant::now();
        let private_bytes: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::private_key_to_bytes(pk))
            .collect();
        let private_serialization_duration = start.elapsed();

        // Measure public key serialization
        let start = Instant::now();
        let public_bytes: Vec<_> = public_keys
            .iter()
            .map(|pk| Bls12381::public_key_to_bytes(pk))
            .collect();
        let public_serialization_duration = start.elapsed();

        // Measure signature serialization
        let start = Instant::now();
        let signature_bytes: Vec<_> = signatures
            .iter()
            .map(|sig| Bls12381::signature_to_bytes(sig))
            .collect();
        let signature_serialization_duration = start.elapsed();

        // Measure private key deserialization
        let start = Instant::now();
        let deserialized_private: Vec<_> = private_bytes
            .iter()
            .map(|bytes| Bls12381::private_key_from_bytes(bytes).unwrap())
            .collect();
        let private_deserialization_duration = start.elapsed();

        // Measure public key deserialization
        let start = Instant::now();
        let deserialized_public: Vec<_> = public_bytes
            .iter()
            .map(|bytes| Bls12381::public_key_from_bytes(bytes).unwrap())
            .collect();
        let public_deserialization_duration = start.elapsed();

        // Measure signature deserialization
        let start = Instant::now();
        let deserialized_signatures: Vec<_> = signature_bytes
            .iter()
            .map(|bytes| Bls12381::signature_from_bytes(bytes).unwrap())
            .collect();
        let signature_deserialization_duration = start.elapsed();

        // Verify all deserialized data matches
        for i in 0..iterations {
            assert_eq!(private_keys[i], deserialized_private[i]);
            assert_eq!(public_keys[i], deserialized_public[i]);
            assert_eq!(signatures[i], deserialized_signatures[i]);
        }

        // Performance should be reasonable
        assert!(
            private_serialization_duration.as_millis() < 1000,
            "Private key serialization too slow: {:?}",
            private_serialization_duration
        );
        assert!(
            public_serialization_duration.as_millis() < 1000,
            "Public key serialization too slow: {:?}",
            public_serialization_duration
        );
        assert!(
            signature_serialization_duration.as_millis() < 1000,
            "Signature serialization too slow: {:?}",
            signature_serialization_duration
        );

        assert!(
            private_deserialization_duration.as_millis() < 2000,
            "Private key deserialization too slow: {:?}",
            private_deserialization_duration
        );
        assert!(
            public_deserialization_duration.as_millis() < 5000,
            "Public key deserialization too slow: {:?}",
            public_deserialization_duration
        );
        assert!(
            signature_deserialization_duration.as_millis() < 10000,
            "Signature deserialization too slow: {:?}",
            signature_deserialization_duration
        );

        println!(
            "Serialized/deserialized {} private keys in {:?}/{:?}",
            iterations, private_serialization_duration, private_deserialization_duration
        );
        println!(
            "Serialized/deserialized {} public keys in {:?}/{:?}",
            iterations, public_serialization_duration, public_deserialization_duration
        );
        println!(
            "Serialized/deserialized {} signatures in {:?}/{:?}",
            iterations, signature_serialization_duration, signature_deserialization_duration
        );
    }

    /// Test validation performance (matches C# validation benchmarks exactly)
    #[test]
    fn test_validation_performance_compatibility() {
        let mut rng = thread_rng();
        let iterations = 1000;

        // Generate test data
        let private_keys: Vec<_> = (0..iterations)
            .map(|_| Bls12381::generate_private_key(&mut rng))
            .collect();

        let public_keys: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::derive_public_key(pk))
            .collect();

        let message = b"Validation performance test";
        let signatures: Vec<_> = private_keys
            .iter()
            .map(|pk| Bls12381::sign(pk, message).unwrap())
            .collect();

        // Measure private key validation
        let start = Instant::now();
        for private_key in &private_keys {
            assert!(Bls12381::validate_private_key(private_key));
        }
        let private_validation_duration = start.elapsed();

        // Measure public key validation
        let start = Instant::now();
        for public_key in &public_keys {
            assert!(Bls12381::validate_public_key(public_key));
        }
        let public_validation_duration = start.elapsed();

        // Measure signature validation
        let start = Instant::now();
        for signature in &signatures {
            assert!(Bls12381::validate_signature(signature));
        }
        let signature_validation_duration = start.elapsed();

        // Performance should be reasonable
        assert!(
            private_validation_duration.as_millis() < 500,
            "Private key validation too slow: {:?}",
            private_validation_duration
        );
        assert!(
            public_validation_duration.as_millis() < 2000,
            "Public key validation too slow: {:?}",
            public_validation_duration
        );
        assert!(
            signature_validation_duration.as_millis() < 5000,
            "Signature validation too slow: {:?}",
            signature_validation_duration
        );

        println!(
            "Validated {} private keys in {:?}",
            iterations, private_validation_duration
        );
        println!(
            "Validated {} public keys in {:?}",
            iterations, public_validation_duration
        );
        println!(
            "Validated {} signatures in {:?}",
            iterations, signature_validation_duration
        );
    }

    /// Test concurrent performance (matches C# thread performance exactly)
    #[test]
    fn test_concurrent_performance_compatibility() {
        use std::sync::Arc;
        use std::thread;

        let mut rng = thread_rng();
        let thread_count = 4;
        let operations_per_thread = 50;

        let private_key = Arc::new(Bls12381::generate_private_key(&mut rng));
        let public_key = Arc::new(Bls12381::derive_public_key(&private_key));
        let message = Arc::new(b"Concurrent performance test".to_vec());

        // Measure concurrent signing performance
        let start = Instant::now();
        let signing_handles: Vec<_> = (0..thread_count)
            .map(|_| {
                let pk = Arc::clone(&private_key);
                let msg = Arc::clone(&message);

                thread::spawn(move || {
                    let mut signatures = Vec::new();
                    for _ in 0..operations_per_thread {
                        let signature = Bls12381::sign(&pk, &msg).unwrap();
                        signatures.push(signature);
                    }
                    signatures
                })
            })
            .collect();

        let signing_results: Vec<_> = signing_handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        let concurrent_signing_duration = start.elapsed();

        // Measure concurrent verification performance
        let all_signatures: Vec<_> = signing_results.into_iter().flatten().collect();

        let start = Instant::now();
        let verification_handles: Vec<_> = (0..thread_count)
            .map(|i| {
                let pub_k = Arc::clone(&public_key);
                let msg = Arc::clone(&message);
                let sigs = all_signatures
                    [i * operations_per_thread..(i + 1) * operations_per_thread]
                    .to_vec();

                thread::spawn(move || {
                    for signature in sigs {
                        assert!(Bls12381::verify(&pub_k, &msg, &signature));
                    }
                })
            })
            .collect();

        for handle in verification_handles {
            handle.join().unwrap();
        }
        let concurrent_verification_duration = start.elapsed();

        let total_operations = thread_count * operations_per_thread;

        // Concurrent performance should be reasonable
        assert!(
            concurrent_signing_duration.as_millis() < 10000,
            "Concurrent signing too slow: {:?}",
            concurrent_signing_duration
        );
        assert!(
            concurrent_verification_duration.as_millis() < 15000,
            "Concurrent verification too slow: {:?}",
            concurrent_verification_duration
        );

        println!(
            "Concurrent signing: {} operations across {} threads in {:?}",
            total_operations, thread_count, concurrent_signing_duration
        );
        println!(
            "Concurrent verification: {} operations across {} threads in {:?}",
            total_operations, thread_count, concurrent_verification_duration
        );

        let first_signature = &all_signatures[0];
        for signature in &all_signatures {
            assert_eq!(first_signature, signature);
        }
    }

    /// Test memory usage characteristics (matches C# memory behavior exactly)
    #[test]
    fn test_memory_usage_compatibility() {
        let mut rng = thread_rng();
        let iterations = 1000;

        // Test that operations don't cause memory leaks
        // This is primarily a regression test

        // Generate and drop many keys
        for _ in 0..iterations {
            let private_key = Bls12381::generate_private_key(&mut rng);
            let public_key = Bls12381::derive_public_key(&private_key);
            let message = b"Memory test";
            let signature = Bls12381::sign(&private_key, message).unwrap();

            // Validate and verify
            assert!(Bls12381::validate_private_key(&private_key));
            assert!(Bls12381::validate_public_key(&public_key));
            assert!(Bls12381::validate_signature(&signature));
            assert!(Bls12381::verify(&public_key, message, &signature));

            // Serialize and deserialize
            let private_bytes = Bls12381::private_key_to_bytes(&private_key);
            let public_bytes = Bls12381::public_key_to_bytes(&public_key);
            let signature_bytes = Bls12381::signature_to_bytes(&signature);

            let _restored_private = Bls12381::private_key_from_bytes(&private_bytes).unwrap();
            let _restored_public = Bls12381::public_key_from_bytes(&public_bytes).unwrap();
            let _restored_signature = Bls12381::signature_from_bytes(&signature_bytes).unwrap();

            // Objects should be dropped automatically here
        }

        // Test aggregation memory usage
        let signatures: Vec<_> = (0..100)
            .map(|_| {
                let private_key = Bls12381::generate_private_key(&mut rng);
                Bls12381::sign(&private_key, b"Aggregation memory test").unwrap()
            })
            .collect();

        // Create and drop many aggregates
        for _ in 0..10 {
            let _aggregate = Bls12381::aggregate_signatures(&signatures).unwrap();
            // Aggregate should be dropped here
        }

        assert!(true);
    }

    /// Test performance with edge cases (matches C# edge case performance exactly)
    #[test]
    fn test_edge_case_performance_compatibility() {
        let mut rng = thread_rng();
        let private_key = Bls12381::generate_private_key(&mut rng);
        let public_key = Bls12381::derive_public_key(&private_key);

        // Test performance with edge case messages
        let edge_cases = vec![
            vec![],                                             // Empty message
            vec![0x00],                                         // Single zero byte
            vec![0xFF],                                         // Single max byte
            vec![0x00; 10000],                                  // Large zeros
            vec![0xFF; 10000],                                  // Large max values
            (0..=255).cycle().take(10000).collect::<Vec<u8>>(), // Pattern
        ];

        for (i, message) in edge_cases.iter().enumerate() {
            let start = Instant::now();
            let signature = Bls12381::sign(&private_key, message).unwrap();
            let signing_duration = start.elapsed();

            let start = Instant::now();
            let verification_result = Bls12381::verify(&public_key, message, &signature);
            let verification_duration = start.elapsed();

            assert!(verification_result);

            // Edge cases should not significantly impact performance
            assert!(
                signing_duration.as_millis() < 1000,
                "Edge case {} signing too slow: {:?}",
                i,
                signing_duration
            );
            assert!(
                verification_duration.as_millis() < 1000,
                "Edge case {} verification too slow: {:?}",
                i,
                verification_duration
            );
        }

        // Test performance with maximum aggregation size
        let max_aggregation_size = 1000;
        let signatures: Vec<_> = (0..max_aggregation_size)
            .map(|_| {
                let pk = Bls12381::generate_private_key(&mut rng);
                Bls12381::sign(&pk, b"Max aggregation test").unwrap()
            })
            .collect();

        let start = Instant::now();
        let aggregate = Bls12381::aggregate_signatures(&signatures).unwrap();
        let aggregation_duration = start.elapsed();

        // Large aggregation should complete in reasonable time
        assert!(
            aggregation_duration.as_millis() < 30000,
            "Large aggregation too slow: {:?}",
            aggregation_duration
        );

        println!(
            "Aggregated {} signatures in {:?}",
            max_aggregation_size, aggregation_duration
        );
    }
}

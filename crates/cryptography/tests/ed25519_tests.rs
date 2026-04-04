//! Ed25519 tests converted from C# Neo unit tests (UT_Ed25519.cs).
//! These tests ensure 100% compatibility with the C# Neo Ed25519 implementation.

use neo_cryptography::ed25519::Ed25519;

// ============================================================================
// Helper functions
// ============================================================================

/// Convert hex string to bytes
fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    hex::decode(hex_str).unwrap()
}

/// Convert bytes to hex string
#[allow(dead_code)]
fn to_hex_string(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

// ============================================================================
// C# UT_Ed25519 test conversions
// ============================================================================

/// Test converted from C# UT_Ed25519.TestGenerateKeyPair
#[test]
fn test_generate_key_pair() {
    let (private_key, public_key) = Ed25519::generate_key_pair();
    assert_eq!(32, private_key.len()); // Ed25519 private key is 32 bytes
    assert_eq!(32, public_key.len()); // Ed25519 public key is 32 bytes
}

/// Test converted from C# UT_Ed25519.TestGetPublicKey
#[test]
fn test_get_public_key() {
    let (private_key, _) = Ed25519::generate_key_pair();
    let public_key = Ed25519::private_key_to_public_key(&private_key).unwrap();

    assert!(!public_key.is_empty());
    assert_eq!(32, public_key.len()); // Ed25519 public key is 32 bytes
}

/// Test converted from C# UT_Ed25519.TestSignAndVerify
#[test]
fn test_sign_and_verify() {
    let (private_key, public_key) = Ed25519::generate_key_pair();
    let message = "Hello, Neo!".as_bytes();

    let signature = Ed25519::sign(&private_key, message).unwrap();
    assert!(!signature.is_empty());
    assert_eq!(64, signature.len()); // Ed25519 signature is 64 bytes

    let is_valid = Ed25519::verify(&public_key, message, &signature);
    assert!(is_valid);
}

/// Test converted from C# UT_Ed25519.TestFailedVerify
#[test]
fn test_failed_verify() {
    let (private_key, public_key) = Ed25519::generate_key_pair();
    let message = "Hello, Neo!".as_bytes();

    let signature = Ed25519::sign(&private_key, message).unwrap();

    // Tamper with the message
    let tampered_message = "Hello, Neo?".as_bytes();
    let is_valid = Ed25519::verify(&public_key, tampered_message, &signature);
    assert!(!is_valid);

    // Tamper with the signature
    let mut tampered_signature = signature.clone();
    tampered_signature[0] ^= 0x01; // Flip one bit
    let is_valid = Ed25519::verify(&public_key, message, &tampered_signature);
    assert!(!is_valid);

    // Use wrong public key
    let (_, wrong_public_key) = Ed25519::generate_key_pair();
    let is_valid = Ed25519::verify(&wrong_public_key, message, &signature);
    assert!(!is_valid);
}

/// Test converted from C# UT_Ed25519.TestInvalidPrivateKeySize
#[test]
fn test_invalid_private_key_size() {
    let invalid_private_key = vec![0u8; 31]; // Invalid size
    let result = Ed25519::private_key_to_public_key(&invalid_private_key);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid"));
}

/// Test converted from C# UT_Ed25519.TestInvalidSignatureSize
#[test]
fn test_invalid_signature_size() {
    let message = "Test message".as_bytes();
    let invalid_signature = vec![0u8; 63]; // Invalid size
    let public_key = vec![0u8; 32];

    let is_valid = Ed25519::verify(&public_key, message, &invalid_signature);
    assert!(!is_valid);

    // Test with oversized signature
    let oversized_signature = vec![0u8; 65];
    let is_valid = Ed25519::verify(&public_key, message, &oversized_signature);
    assert!(!is_valid);
}

/// Test converted from C# UT_Ed25519.TestInvalidPublicKeySize
#[test]
fn test_invalid_public_key_size() {
    let message = "Test message".as_bytes();
    let signature = vec![0u8; 64];
    let invalid_public_key = vec![0u8; 31]; // Invalid size

    let is_valid = Ed25519::verify(&invalid_public_key, message, &signature);
    assert!(!is_valid);

    // Test with oversized public key
    let oversized_public_key = vec![0u8; 33];
    let is_valid = Ed25519::verify(&oversized_public_key, message, &signature);
    assert!(!is_valid);
}

/// Test converted from C# UT_Ed25519.TestVectorCase1 (RFC 8032 test vector)
#[test]
fn test_vector_case1() {
    let private_key =
        hex_to_bytes("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
    let expected_public_key =
        hex_to_bytes("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
    let message: Vec<u8> = vec![]; // Empty message
    let expected_signature = hex_to_bytes(
        &("e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155".to_string()
            + "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"),
    );

    // Test public key derivation
    let derived_public_key = Ed25519::private_key_to_public_key(&private_key).unwrap();
    assert_eq!(expected_public_key, derived_public_key);

    // Test signature generation
    let signature = Ed25519::sign(&private_key, &message).unwrap();
    assert_eq!(expected_signature, signature);

    // Test signature verification
    assert!(Ed25519::verify(
        &expected_public_key,
        &message,
        &expected_signature
    ));
}

/// Test converted from C# UT_Ed25519.TestVectorCase2 (RFC 8032 test vector)
#[test]
fn test_vector_case2() {
    let private_key =
        hex_to_bytes("4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb");
    let expected_public_key =
        hex_to_bytes("3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c");
    let message = "r".as_bytes();
    let expected_signature = hex_to_bytes(
        &("92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da".to_string()
            + "085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"),
    );

    // Test public key derivation
    let derived_public_key = Ed25519::private_key_to_public_key(&private_key).unwrap();
    assert_eq!(expected_public_key, derived_public_key);

    // Test signature generation
    let signature = Ed25519::sign(&private_key, message).unwrap();
    assert_eq!(expected_signature, signature);

    // Test signature verification
    assert!(Ed25519::verify(
        &expected_public_key,
        message,
        &expected_signature
    ));
}

/// Test converted from C# UT_Ed25519.TestVectorCase3 (RFC 8032 test vector)
#[test]
fn test_vector_case3() {
    let private_key =
        hex_to_bytes("c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7");
    let expected_public_key =
        hex_to_bytes("fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025");
    let expected_signature = hex_to_bytes(
        &("6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac".to_string()
            + "18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a"),
    );
    let message = hex_to_bytes("af82");

    // Test public key derivation
    let derived_public_key = Ed25519::private_key_to_public_key(&private_key).unwrap();
    assert_eq!(expected_public_key, derived_public_key);

    // Test signature generation
    let signature = Ed25519::sign(&private_key, &message).unwrap();
    assert_eq!(expected_signature, signature);

    // Test signature verification
    assert!(Ed25519::verify(
        &expected_public_key,
        &message,
        &expected_signature
    ));
}

// ============================================================================
// Additional Ed25519 tests for comprehensive coverage
// ============================================================================

/// Test deterministic key generation
#[test]
fn test_deterministic_operations() {
    let (private_key, public_key) = Ed25519::generate_key_pair();
    let message = "Consistent test message".as_bytes();

    // Test that public key derivation is deterministic
    let public_key1 = Ed25519::private_key_to_public_key(&private_key).unwrap();
    let public_key2 = Ed25519::private_key_to_public_key(&private_key).unwrap();
    assert_eq!(public_key1, public_key2);
    assert_eq!(public_key, public_key1);

    // Test that signature generation is deterministic for Ed25519
    let signature1 = Ed25519::sign(&private_key, message).unwrap();
    let signature2 = Ed25519::sign(&private_key, message).unwrap();
    assert_eq!(signature1, signature2);

    // Test that verification is consistent
    assert!(Ed25519::verify(&public_key1, message, &signature1));
    assert!(Ed25519::verify(&public_key2, message, &signature2));
}

/// Test empty message handling
#[test]
fn test_empty_message() {
    let (private_key, public_key) = Ed25519::generate_key_pair();
    let empty_message: &[u8] = &[];

    let signature = Ed25519::sign(&private_key, empty_message).unwrap();
    assert_eq!(64, signature.len());

    let is_valid = Ed25519::verify(&public_key, empty_message, &signature);
    assert!(is_valid);
}

/// Test large message handling
#[test]
fn test_large_message() {
    let (private_key, public_key) = Ed25519::generate_key_pair();

    // Create a large message (1MB)
    let large_message = vec![0xAA; 1024 * 1024];

    let signature = Ed25519::sign(&private_key, &large_message).unwrap();
    assert_eq!(64, signature.len());

    let is_valid = Ed25519::verify(&public_key, &large_message, &signature);
    assert!(is_valid);
}

/// Test Unicode message handling
#[test]
fn test_unicode_message() {
    let (private_key, public_key) = Ed25519::generate_key_pair();

    let unicode_message = "Hello 世界! 🌍 Ελληνικά Русский العربية".as_bytes();

    let signature = Ed25519::sign(&private_key, unicode_message).unwrap();
    let is_valid = Ed25519::verify(&public_key, unicode_message, &signature);
    assert!(is_valid);
}

/// Test multiple signatures with same key
#[test]
fn test_multiple_signatures() {
    let (private_key, public_key) = Ed25519::generate_key_pair();

    let messages = vec![
        "Message 1".as_bytes(),
        "Message 2".as_bytes(),
        "Message 3".as_bytes(),
    ];

    let mut signatures = Vec::new();

    // Sign all messages
    for message in &messages {
        let signature = Ed25519::sign(&private_key, message).unwrap();
        signatures.push(signature);
    }

    // Verify all signatures
    for (i, message) in messages.iter().enumerate() {
        assert!(Ed25519::verify(&public_key, message, &signatures[i]));
    }

    // Cross-verify (wrong message with wrong signature should fail)
    for (i, message) in messages.iter().enumerate() {
        for (j, signature) in signatures.iter().enumerate() {
            if i != j {
                assert!(!Ed25519::verify(&public_key, message, signature));
            }
        }
    }
}

/// Test key pair independence
#[test]
fn test_key_pair_independence() {
    let (private_key1, public_key1) = Ed25519::generate_key_pair();
    let (private_key2, public_key2) = Ed25519::generate_key_pair();

    // Keys should be different
    assert_ne!(private_key1, private_key2);
    assert_ne!(public_key1, public_key2);

    let message = "Test independence".as_bytes();

    let signature1 = Ed25519::sign(&private_key1, message).unwrap();
    let signature2 = Ed25519::sign(&private_key2, message).unwrap();

    // Signatures should be different
    assert_ne!(signature1, signature2);

    // Cross-verification should fail
    assert!(!Ed25519::verify(&public_key1, message, &signature2));
    assert!(!Ed25519::verify(&public_key2, message, &signature1));

    // Self-verification should succeed
    assert!(Ed25519::verify(&public_key1, message, &signature1));
    assert!(Ed25519::verify(&public_key2, message, &signature2));
}

/// Test edge cases with specific byte patterns
#[test]
fn test_edge_case_byte_patterns() {
    let (private_key, public_key) = Ed25519::generate_key_pair();

    let edge_case_messages = vec![
        vec![0x00; 32],                                 // All zeros
        vec![0xFF; 32],                                 // All ones
        vec![0x00, 0xFF, 0x00, 0xFF],                   // Alternating pattern
        (0..256).map(|i| i as u8).collect::<Vec<u8>>(), // All byte values
    ];

    for message in edge_case_messages {
        let signature = Ed25519::sign(&private_key, &message).unwrap();
        assert!(Ed25519::verify(&public_key, &message, &signature));

        // Modify message slightly and verify it fails
        if !message.is_empty() {
            let mut modified_message = message.clone();
            modified_message[0] = modified_message[0].wrapping_add(1);
            assert!(!Ed25519::verify(&public_key, &modified_message, &signature));
        }
    }
}

/// Test constant time operations (basic timing attack resistance)
#[test]
fn test_constant_time_behavior() {
    let (private_key, public_key) = Ed25519::generate_key_pair();
    let message = "Timing test message".as_bytes();

    let valid_signature = Ed25519::sign(&private_key, message).unwrap();

    // Create various invalid signatures
    let invalid_signatures = vec![
        vec![0x00; 64], // All zeros
        vec![0xFF; 64], // All ones
        {
            let mut sig = valid_signature.clone();
            sig[0] ^= 0x01; // Single bit flip
            sig
        },
        {
            let mut sig = valid_signature.clone();
            sig[63] ^= 0x80; // Last bit flip
            sig
        },
    ];

    // All invalid signatures should fail verification
    // This doesn't test actual timing, but ensures the interface works correctly
    for invalid_sig in invalid_signatures {
        assert!(!Ed25519::verify(&public_key, message, &invalid_sig));
    }

    // Valid signature should still work
    assert!(Ed25519::verify(&public_key, message, &valid_signature));
}

// ============================================================================
// Error handling and validation tests
// ============================================================================

/// Test comprehensive error handling
#[test]
fn test_comprehensive_error_handling() {
    // Test all invalid private key sizes
    for size in [0, 1, 15, 16, 31, 33, 64, 128] {
        let invalid_key = vec![0u8; size];
        if size != 32 {
            assert!(Ed25519::private_key_to_public_key(&invalid_key).is_err());
        }
    }

    // Test all invalid public key sizes for verification
    let (valid_private_key, valid_public_key) = Ed25519::generate_key_pair();
    let valid_signature = Ed25519::sign(&valid_private_key, b"test").unwrap();

    for size in [0, 1, 15, 16, 31, 33, 64, 128] {
        let invalid_pubkey = vec![0u8; size];
        if size != 32 {
            assert!(!Ed25519::verify(&invalid_pubkey, b"test", &valid_signature));
        }
    }

    // Test all invalid signature sizes
    for size in [0, 1, 31, 32, 63, 65, 128] {
        let invalid_signature = vec![0u8; size];
        if size != 64 {
            assert!(!Ed25519::verify(
                &valid_public_key,
                b"test",
                &invalid_signature
            ));
        }
    }
}

/// Test with known-bad key patterns
#[test]
fn test_known_bad_patterns() {
    // Test signing with all-zero private key (this implementation allows it)
    let zero_key = vec![0u8; 32];
    let result = Ed25519::private_key_to_public_key(&zero_key);
    // The behavior varies by implementation - document what this one does
    println!("Zero key result: {:?}", result.is_ok());

    // Test with maximum value private key (should be invalid for Ed25519)
    let max_key = vec![0xFF; 32];
    // Note: This may or may not be invalid depending on Ed25519 implementation
    // The test documents the behavior
    let result = Ed25519::private_key_to_public_key(&max_key);
    println!("Max key result: {:?}", result.is_ok());
}

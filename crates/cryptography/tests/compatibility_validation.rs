use hex;
use neo_cryptography::{base58, ecdsa::ECDsa, hash};

#[test]
fn test_hash_compatibility() {
    println!("=== Testing Hash Function Compatibility ===\n");

    // SHA256 test vectors (from Bitcoin/C# Neo)
    let sha256_cases = vec![
        (
            "",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            "abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            "Neo",
            "effee861f3433baac2d48e5b422c771dfb3762fb096a4aa9a8ba49eb6e7d7c27",
        ),
    ];

    for (input, expected) in sha256_cases {
        let result = hash::sha256(input.as_bytes());
        let result_hex = hex::encode(result);
        println!("SHA256('{}') = {}", input, result_hex);
        println!("Expected:      {}", expected);
        assert_eq!(
            result_hex, expected,
            "SHA256 test failed for input '{}'",
            input
        );
        println!("✅ PASS\n");
    }

    // RIPEMD160 test vectors
    let ripemd_cases = vec![
        ("", "9c1185a5c5e9fc54612808977ee8f548b2258d31"),
        ("abc", "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc"),
    ];

    for (input, expected) in ripemd_cases {
        let result = hash::ripemd160(input.as_bytes());
        let result_hex = hex::encode(result);
        println!("RIPEMD160('{}') = {}", input, result_hex);
        println!("Expected:        {}", expected);
        assert_eq!(
            result_hex, expected,
            "RIPEMD160 test failed for input '{}'",
            input
        );
        println!("✅ PASS\n");
    }

    // Hash160 and Hash256 determinism tests
    let test_data = b"Neo blockchain";
    let hash160_1 = hash::hash160(test_data);
    let hash160_2 = hash::hash160(test_data);
    assert_eq!(hash160_1, hash160_2, "Hash160 is not deterministic");
    println!("✅ Hash160 determinism: PASS");

    let hash256_1 = hash::hash256(test_data);
    let hash256_2 = hash::hash256(test_data);
    assert_eq!(hash256_1, hash256_2, "Hash256 is not deterministic");
    println!("✅ Hash256 determinism: PASS\n");
}

#[test]
fn test_ecdsa_compatibility() {
    println!("=== Testing ECDSA Compatibility ===\n");

    // Test key generation and derivation
    let private_key = ECDsa::generate_private_key();
    assert_eq!(private_key.len(), 32, "Private key should be 32 bytes");
    println!("✅ Private key generation: PASS");

    let public_key = ECDsa::derive_public_key(&private_key).unwrap();
    assert_eq!(
        public_key.len(),
        65,
        "Uncompressed public key should be 65 bytes"
    );
    assert_eq!(
        public_key[0], 0x04,
        "Uncompressed key should start with 0x04"
    );
    println!("✅ Public key derivation: PASS");

    let compressed_key = ECDsa::derive_compressed_public_key(&private_key).unwrap();
    assert_eq!(
        compressed_key.len(),
        33,
        "Compressed public key should be 33 bytes"
    );
    assert!(
        compressed_key[0] == 0x02 || compressed_key[0] == 0x03,
        "Compressed key should start with 0x02 or 0x03"
    );
    println!("✅ Key compression: PASS");

    // Test key validation
    assert!(
        ECDsa::validate_private_key(&private_key),
        "Generated private key should be valid"
    );
    assert!(
        ECDsa::validate_public_key(&public_key),
        "Derived public key should be valid"
    );
    assert!(
        ECDsa::validate_public_key(&compressed_key),
        "Compressed key should be valid"
    );
    println!("✅ Key validation: PASS");

    // Test compression/decompression round-trip
    let compressed_from_uncompressed = ECDsa::compress_public_key(&public_key).unwrap();
    let decompressed = ECDsa::decompress_public_key(&compressed_from_uncompressed).unwrap();
    assert_eq!(public_key, decompressed, "Compression round-trip failed");
    println!("✅ Compression round-trip: PASS");

    // Test signing and verification
    let message = b"Neo blockchain test message";
    let signature = ECDsa::sign_neo_format(message, &private_key).unwrap();
    assert_eq!(signature.len(), 64, "Signature should be 64 bytes");
    println!("✅ Signature generation: PASS");

    let is_valid = ECDsa::verify_neo_format(message, &signature, &public_key).unwrap();
    assert!(is_valid, "Signature verification should succeed");
    println!("✅ Signature verification: PASS");

    // Test with wrong message
    let wrong_message = b"Wrong message";
    let is_invalid = ECDsa::verify_neo_format(wrong_message, &signature, &public_key).unwrap();
    assert!(
        !is_invalid,
        "Signature verification should fail for wrong message"
    );
    println!("✅ Invalid signature rejection: PASS\n");
}

#[test]
fn test_base58_compatibility() {
    println!("=== Testing Base58 Compatibility ===\n");

    // Base58 test vectors
    let test_cases = vec![
        (vec![], ""),
        (vec![0], "1"),
        (vec![0, 0], "11"),
        (vec![1, 2, 3], "Ldp"),
        (vec![255], "5Q"),
    ];

    for (input, expected) in test_cases {
        let encoded = base58::encode(&input);
        println!("Base58({:?}) = '{}'", input, encoded);
        assert_eq!(encoded, expected, "Base58 encoding failed");
        println!("✅ PASS");

        // Test round-trip if not empty
        if !encoded.is_empty() {
            let decoded = base58::decode(&encoded).unwrap();
            assert_eq!(input, decoded, "Base58 round-trip failed");
            println!("✅ Round-trip: PASS");
        }
        println!();
    }

    // Base58Check test
    let test_data = vec![1, 2, 3, 4, 5];
    let encoded_check = base58::encode_check(&test_data);
    assert!(
        !encoded_check.is_empty(),
        "Base58Check encoding should not be empty"
    );
    println!("✅ Base58Check encoding: PASS");

    let decoded_check = base58::decode_check(&encoded_check).unwrap();
    assert_eq!(test_data, decoded_check, "Base58Check round-trip failed");
    println!("✅ Base58Check round-trip: PASS\n");
}

#[test]
fn test_performance_characteristics() {
    println!("=== Testing Performance Characteristics ===\n");

    let test_data = b"Neo blockchain performance test data".repeat(1000);

    // Hash performance
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _hash = hash::sha256(&test_data);
    }
    let sha256_time = start.elapsed();
    println!("SHA256 (1000 iterations): {:?}", sha256_time);

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _hash = hash::ripemd160(&test_data);
    }
    let ripemd_time = start.elapsed();
    println!("RIPEMD160 (1000 iterations): {:?}", ripemd_time);

    // ECDSA performance
    let private_key = ECDsa::generate_private_key();
    let public_key = ECDsa::derive_public_key(&private_key).unwrap();
    let message = b"Performance test message";

    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _signature = ECDsa::sign_neo_format(message, &private_key).unwrap();
    }
    let sign_time = start.elapsed();
    println!("ECDSA Sign (100 iterations): {:?}", sign_time);

    let signature = ECDsa::sign_neo_format(message, &private_key).unwrap();
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _valid = ECDsa::verify_neo_format(message, &signature, &public_key).unwrap();
    }
    let verify_time = start.elapsed();
    println!("ECDSA Verify (100 iterations): {:?}", verify_time);

    println!("✅ Performance test completed\n");
}

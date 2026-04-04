//! Core compatibility tests for wallet module
//!
//! These tests ensure that our core types (UInt160, UInt256, etc.) work correctly
//! with the wallet functionality and match the C# implementation behavior.

use hex;
use neo_core::UInt160;
use neo_wallets::{
    Contract, ContractParameterType, Error, KeyPair, StandardWalletAccount, Version, WalletAccount,
};

use neo_wallets::ContractParameterType::*;

#[test]
fn test_uint160_address_conversion() {
    let test_data = [0x01u8];
    let script_hash = UInt160::from_script(&test_data);

    // Convert to address
    let address = script_hash.to_address();
    assert!(!address.is_empty());

    // Convert back from address
    let restored_hash = UInt160::from_address(&address).unwrap();
    assert_eq!(script_hash, restored_hash);
}

#[test]
fn test_uint160_from_script() {
    // Test script hash generation
    let script = vec![0x0c, 0x21, 0x03]; // Sample script bytes
    let script_hash = UInt160::from_script(&script);

    // Should be deterministic
    let script_hash2 = UInt160::from_script(&script);
    assert_eq!(script_hash, script_hash2);

    // Different script should produce different hash
    let different_script = vec![0x0c, 0x21, 0x04];
    let different_hash = UInt160::from_script(&different_script);
    assert_ne!(script_hash, different_hash);
}

#[test]
fn test_uint160_zero() {
    // Test zero UInt160
    let zero = UInt160::new();
    let address = zero.to_address();

    // Should be able to convert zero hash to address and back
    let restored = UInt160::from_address(&address).unwrap();
    assert_eq!(zero, restored);
}

#[test]
fn test_invalid_address_format() {
    // Test invalid address formats
    let invalid_addresses = vec![
        "invalid_address",
        "3vQB7B6MrGQZaxCuFg4oh",                 // Too short
        "NdtB8RXRmJ7Nhw1FPTm7E6HoDZGnDw37nf123", // Too long
        "",                                      // Empty
    ];

    for invalid_address in invalid_addresses {
        let result = UInt160::from_address(invalid_address);
        assert!(
            result.is_err(),
            "Address '{}' should be invalid",
            invalid_address
        );
    }
}

#[test]
fn test_key_pair_script_hash_consistency() {
    // Test that KeyPair script hash matches UInt160::from_script
    let key_pair = KeyPair::generate().unwrap();
    let verification_script = key_pair.get_verification_script();

    // Calculate script hash using UInt160::from_script
    let calculated_hash = UInt160::from_script(&verification_script);

    let key_pair_hash = key_pair.get_script_hash();
    assert_eq!(calculated_hash, key_pair_hash);
}

#[test]
fn test_wallet_account_address_consistency() {
    // Test that wallet account addresses are consistent
    let key_pair = KeyPair::generate().unwrap();
    let account = StandardWalletAccount::new_with_key(key_pair.clone(), None);

    let account_address = account.address();
    let script_hash_address = account.script_hash().to_address();
    assert_eq!(account_address, script_hash_address);

    let key_pair_address = key_pair.get_script_hash().to_address();
    assert_eq!(account_address, key_pair_address);
}

#[test]
fn test_contract_script_hash() {
    // Test contract script hash generation
    let key_pair = KeyPair::generate().unwrap();
    let contract =
        Contract::create_signature_contract(&key_pair.get_public_key_point().unwrap()).unwrap();

    assert_eq!(contract.script_hash(), key_pair.get_script_hash());
}

#[test]
fn test_witness_creation() {
    // Test witness creation with proper script hash
    let key_pair = KeyPair::generate().unwrap();
    let data = b"test data for signing";

    // Sign data
    let signature = key_pair.sign(data).unwrap();

    // Create witness
    let invocation_script = vec![0x0c, 0x40]; // PUSHDATA1 64 bytes
    let verification_script = key_pair.get_verification_script();
    let witness = neo_core::Witness::new_with_scripts(invocation_script, verification_script);

    // Witness should be valid
    assert!(!witness.invocation_script().is_empty());
    assert!(!witness.verification_script().is_empty());
}

#[test]
fn test_transaction_hash_data() {
    // Test transaction hash data generation
    let mut transaction = neo_core::Transaction::new();
    transaction.set_version(0);
    transaction.set_nonce(12345);
    transaction.set_system_fee(1000);
    transaction.set_network_fee(500);
    transaction.set_valid_until_block(100);

    // Get hash data
    let hash_data = transaction.get_hash_data();
    assert!(!hash_data.is_empty());

    // Hash data should be deterministic
    let hash_data2 = transaction.get_hash_data();
    assert_eq!(hash_data, hash_data2);
}

#[test]
fn test_version_compatibility() {
    // Test version parsing and formatting
    let version_str = "1.2.3";
    let version = Version::parse(version_str).unwrap();

    assert_eq!(1, version.major);
    assert_eq!(2, version.minor);
    assert_eq!(3, version.patch);

    assert_eq!(version_str, version.to_string());
}

#[test]
fn test_version_default() {
    // Test default version
    let default_version = Version::default();
    assert_eq!(0, default_version.major);
    assert_eq!(1, default_version.minor);
    assert_eq!(0, default_version.patch);
}

#[test]
fn test_error_conversions() {
    // Test error type conversions
    let crypto_error = neo_cryptography::Error::InvalidKey("test error".to_string());
    let wallet_error: Error = crypto_error.into();

    match wallet_error {
        Error::Cryptography(_) => {
            // Expected conversion
        }
        _ => panic!("Expected Cryptography error"),
    }
}

#[test]
fn test_contract_parameter_types() {
    // Test contract parameter type conversions
    let types = vec![
        (Any, 0x00),
        (Boolean, 0x10),
        (Integer, 0x11),
        (ByteArray, 0x12),
        (String, 0x13),
        (Hash160, 0x14),
        (Hash256, 0x15),
        (PublicKey, 0x16),
        (Signature, 0x17),
        (Array, 0x20),
        (Map, 0x22),
        (InteropInterface, 0x30),
        (Void, 0xff),
    ];

    for (param_type, expected_value) in types {
        // Test conversion to u8
        let value = param_type as u8;
        assert_eq!(expected_value, value);

        // Test conversion from u8
        let restored_type = ContractParameterType::try_from(value).unwrap();
        assert_eq!(param_type, restored_type);

        // Test display
        let display_str = param_type.to_string();
        assert!(!display_str.is_empty());
    }
}

#[test]
fn test_invalid_contract_parameter_type() {
    // Test invalid contract parameter type
    let invalid_value = 0x99;
    let result = ContractParameterType::try_from(invalid_value);
    assert!(result.is_err());
}

#[test]
fn test_uint160_parsing() {
    let hex_str = "0xff00000000000000000000000000000000000001";
    let uint160 =
        UInt160::from_bytes(&hex::decode(hex_str.trim_start_matches("0x")).unwrap()).unwrap();

    assert_eq!(hex_str, uint160.to_string());
}

#[test]
fn test_uint160_zero_initialization() {
    let uint160 = UInt160::new();
    assert_eq!(
        "0x0000000000000000000000000000000000000000",
        uint160.to_string()
    );
}

#[test]
fn test_uint160_from_bytes() {
    let bytes = [0u8; 20];
    let uint160 = UInt160::from_bytes(&bytes).unwrap();
    assert_eq!(UInt160::new(), uint160);
}

#[test]
fn test_uint160_invalid_length() {
    let invalid_bytes = [0u8; 21]; // Too long
    let result = UInt160::from_bytes(&invalid_bytes);
    assert!(result.is_err());
}

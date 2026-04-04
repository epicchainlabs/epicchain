//! Basic tests for the smart contract module.

use neo_core::UInt160;
use neo_smart_contract::native::NativeContract;
use neo_smart_contract::*;

#[test]
fn test_basic_compilation() {
    // This test just ensures the module compiles and basic types work
    let hash = UInt160::zero();
    assert_eq!(hash, UInt160::zero());
}

#[test]
fn test_contract_manifest_creation() {
    let manifest = manifest::ContractManifest::new("TestContract".to_string());
    assert_eq!(manifest.name, "TestContract");
    assert!(!manifest.permissions.is_empty());
    assert!(manifest.validate().is_ok());
}

#[test]
fn test_storage_key_operations() {
    let contract_hash = UInt160::zero();
    let key = storage::StorageKey::from_string(contract_hash, "test_key");

    assert_eq!(key.contract, contract_hash);
    assert_eq!(key.as_string(), Some("test_key".to_string()));

    let hex_key = key.to_hex_string();
    let from_hex = storage::StorageKey::from_hex_string(contract_hash, &hex_key).unwrap();
    assert_eq!(key, from_hex);
}

#[test]
fn test_storage_item_operations() {
    let item = storage::StorageItem::from_string("test_value");
    assert_eq!(item.as_string(), Some("test_value".to_string()));
    assert!(!item.is_constant);

    let constant_item = storage::StorageItem::new_constant(b"constant".to_vec());
    assert!(constant_item.is_constant);
}

#[test]
fn test_native_contract_registry() {
    let registry = native::NativeRegistry::new();

    // Check that standard contracts are registered
    let neo_hash = native::NeoToken::new().hash();
    let gas_hash = native::GasToken::new().hash();

    assert!(registry.is_native(&neo_hash));
    assert!(registry.is_native(&gas_hash));
    assert!(registry.get(&neo_hash).is_some());
    assert!(registry.get(&gas_hash).is_some());
}

#[test]
fn test_neo_token_basic_operations() {
    let neo = native::NeoToken::new();
    assert_eq!(neo.name(), "NeoToken");

    // Test that the contract has methods
    assert!(!neo.methods().is_empty());

    // Test hash is valid
    let hash = neo.hash();
    assert_ne!(hash, UInt160::zero());
}

#[test]
fn test_gas_token_basic_operations() {
    let gas = native::GasToken::new();
    assert_eq!(gas.name(), "GasToken");

    // Test that the contract has methods
    assert!(!gas.methods().is_empty());

    // Test hash is valid
    let hash = gas.hash();
    assert_ne!(hash, UInt160::zero());
}

#[test]
fn test_contract_state_creation() {
    let hash = UInt160::zero();
    let nef = contract_state::NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]); // RET opcode
    let manifest = manifest::ContractManifest::default();

    let state = contract_state::ContractState::new(1, hash, nef, manifest);
    assert_eq!(state.id, 1);
    assert_eq!(state.update_counter, 0);
    assert_eq!(state.hash, hash);
}

#[test]
fn test_policy_contract() {
    let policy = native::PolicyContract::new();
    assert_eq!(policy.name(), "PolicyContract");

    // Test that the contract has methods
    assert!(!policy.methods().is_empty());

    // Test hash is valid
    let hash = policy.hash();
    assert_ne!(hash, UInt160::zero());
}

#[test]
fn test_oracle_contract() {
    let oracle = native::OracleContract::new();
    assert_eq!(oracle.name(), "OracleContract");

    // Test that the contract has methods
    assert!(!oracle.methods().is_empty());

    // Test hash is valid
    let hash = oracle.hash();
    assert_ne!(hash, UInt160::zero());
}

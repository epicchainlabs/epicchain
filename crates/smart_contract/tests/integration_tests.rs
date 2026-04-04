//! Integration tests for the smart contract module.

use neo_core::{UInt160, UInt256};
use neo_smart_contract::manifest::{
    ContractManifest, ContractPermission, ContractPermissionDescriptor, WildcardContainer,
};
use neo_smart_contract::*;
use neo_vm::TriggerType;

#[test]
fn test_application_engine_creation() {
    let engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);
    assert_eq!(engine.gas_limit(), 10_000_000);
    assert_eq!(engine.gas_consumed(), 0);
    assert_eq!(engine.trigger(), TriggerType::Application);
}

#[test]
fn test_application_engine_gas_operations() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);
    let result = engine.consume_gas(1000);
    assert!(result.is_ok());
    assert_eq!(engine.gas_consumed(), 1000);
}

#[test]
fn test_wildcard_container_integration() {
    let wildcard: WildcardContainer<String> = WildcardContainer::create_wildcard();
    assert!(wildcard.is_wildcard());
    assert_eq!(wildcard.count(), 0);

    let specific = WildcardContainer::create(vec!["method1".to_string()]);
    assert!(!specific.is_wildcard());
    assert_eq!(specific.count(), 1);
    assert!(specific.contains(&"method1".to_string()));
}

#[test]
fn test_contract_permission_integration() {
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::Hash(UInt160::zero()),
        methods: WildcardContainer::create_wildcard(),
    };

    assert!(permission.allows_contract(&UInt160::zero()));
    assert!(permission.allows_method("any_method"));
    assert!(permission.validate().is_ok());
}

#[test]
fn test_contract_manifest_integration() {
    let mut manifest = ContractManifest::default();
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::Hash(UInt160::zero()),
        methods: WildcardContainer::create_wildcard(),
    };
    manifest.permissions.push(permission);
    assert!(manifest.can_call(&UInt160::zero(), "test"));
}

#[test]
fn test_uint256_operations() {
    let hash1 = UInt256::zero();
    let hash2 = UInt256::zero();
    assert_eq!(hash1, hash2);
    let hash_str = hash1.to_string();
    assert_eq!(hash_str.len(), 66); // "0x" + 64 hex chars
}

#[test]
fn test_error_handling() {
    let mut engine = ApplicationEngine::new(TriggerType::Application, 1000);
    let result = engine.consume_gas(2000);
    assert!(result.is_err());
}

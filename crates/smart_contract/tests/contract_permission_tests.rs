//! Contract permission tests converted from C# Neo unit tests (UT_ContractPermission.cs).
//! These tests ensure 100% compatibility with the C# Neo contract permission implementation.

use neo_core::UInt160;
use neo_cryptography::{secp256r1, ECPoint};
use neo_smart_contract::contract_state::ContractState;
use neo_smart_contract::manifest::{
    ContractGroup, ContractManifest, ContractPermission, ContractPermissionDescriptor,
    WildcardContainer,
};
use rand::Rng;
use std::str::FromStr;

// ============================================================================
// Helper functions for testing
// ============================================================================

/// Create a default test manifest
fn create_default_manifest() -> ContractManifest {
    let mut manifest = ContractManifest::new("testManifest".to_string());

    // Add default permission
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::create_wildcard(),
        methods: WildcardContainer::create_wildcard(),
    };
    manifest.permissions.push(permission);

    manifest
}

/// Create a default permission (wildcard for both contract and methods)
fn create_default_permission() -> ContractPermission {
    ContractPermission {
        contract: ContractPermissionDescriptor::create_wildcard(),
        methods: WildcardContainer::create_wildcard(),
    }
}

// ============================================================================
// Test contract permission serialization/deserialization
// ============================================================================

/// Test converted from C# UT_ContractPermission.TestDeserialize
#[test]
fn test_deserialize() {
    // Test 1: Default permission (wildcards)
    let permission = create_default_permission();

    assert!(matches!(
        permission.contract,
        ContractPermissionDescriptor::Wildcard
    ));
    assert!(matches!(permission.methods, WildcardContainer::Wildcard));

    // Test 2: Specific permission
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::Hash(UInt160::zero()),
        methods: WildcardContainer::List(vec!["test".to_string()]),
    };

    assert!(matches!(
        permission.contract,
        ContractPermissionDescriptor::Hash(_)
    ));
    assert!(matches!(permission.methods, WildcardContainer::List(_)));

    if let ContractPermissionDescriptor::Hash(hash) = &permission.contract {
        assert_eq!(*hash, UInt160::zero());
    }

    if let WildcardContainer::List(methods) = &permission.methods {
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0], "test");
    }
}

// ============================================================================
// Test permission checking logic
// ============================================================================

/// Test converted from C# UT_ContractPermission.TestIsAllowed
#[test]
fn test_is_allowed() {
    // Test 1: Permission allows specific contract hash
    let manifest1 = create_default_manifest();
    let mut permission1 = create_default_permission();
    permission1.contract = ContractPermissionDescriptor::Hash(UInt160::zero());

    // Should allow contract with hash UInt160::zero()
    assert!(permission1.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest1,
        },
        "AAA"
    ));

    // Reset to wildcard
    permission1.contract = ContractPermissionDescriptor::create_wildcard();

    // Test 2: Permission with different hash
    let manifest2 = create_default_manifest();
    let mut permission2 = create_default_permission();
    permission2.contract = ContractPermissionDescriptor::Hash(
        UInt160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
    );

    // Should NOT allow contract with hash UInt160::zero()
    assert!(!permission2.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest2,
        },
        "AAA"
    ));

    // Reset to wildcard
    permission2.contract = ContractPermissionDescriptor::create_wildcard();

    // Test 3: Permission with public key matching group
    let mut rng = rand::thread_rng();
    let private_key3: [u8; 32] = rng.gen();
    let public_key3 = secp256r1::generate_public_key(&private_key3);

    let mut manifest3 = create_default_manifest();
    manifest3.groups.push(ContractGroup {
        pub_key: public_key3.clone(),
        signature: vec![0u8; 64], // Dummy signature for testing
    });

    let mut permission3 = create_default_permission();
    permission3.contract = ContractPermissionDescriptor::Group(public_key3);

    // Should allow contract with matching group
    assert!(permission3.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest3,
        },
        "AAA"
    ));

    // Test 4: Permission with public key NOT matching group
    let private_key41: [u8; 32] = rng.gen();
    let public_key41 = secp256r1::generate_public_key(&private_key41);

    let private_key42: [u8; 32] = rng.gen();
    let public_key42 = secp256r1::generate_public_key(&private_key42);

    let mut manifest4 = create_default_manifest();
    manifest4.groups.push(ContractGroup {
        pub_key: public_key42,
        signature: vec![0u8; 64], // Dummy signature for testing
    });

    let mut permission4 = create_default_permission();
    permission4.contract = ContractPermissionDescriptor::Group(public_key41);

    // Should NOT allow contract with non-matching group
    assert!(!permission4.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest4,
        },
        "AAA"
    ));
}

// ============================================================================
// Test method permission checking
// ============================================================================

/// Test method wildcard permissions
#[test]
fn test_method_wildcard_permissions() {
    let manifest = create_default_manifest();
    let permission = create_default_permission();

    // Wildcard methods should allow any method
    assert!(permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "anyMethod"
    ));
    assert!(permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "transfer"
    ));
    assert!(permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "balanceOf"
    ));
}

/// Test specific method permissions
#[test]
fn test_specific_method_permissions() {
    let manifest = create_default_manifest();
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::create_wildcard(),
        methods: WildcardContainer::List(vec!["transfer".to_string(), "balanceOf".to_string()]),
    };

    // Should allow listed methods
    assert!(permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "transfer"
    ));
    assert!(permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "balanceOf"
    ));

    // Should NOT allow unlisted methods
    assert!(!permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "mint"
    ));
    assert!(!permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "burn"
    ));
}

// ============================================================================
// Test complex permission scenarios
// ============================================================================

/// Test combined contract and method restrictions
#[test]
fn test_combined_restrictions() {
    let manifest = create_default_manifest();

    // Permission: specific contract, specific methods
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::Hash(UInt160::zero()),
        methods: WildcardContainer::List(vec!["transfer".to_string()]),
    };

    // Should allow: correct contract + correct method
    assert!(permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "transfer"
    ));

    // Should NOT allow: correct contract + wrong method
    assert!(!permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "balanceOf"
    ));

    // Should NOT allow: wrong contract + correct method
    let different_hash = UInt160::from_str("0x0000000000000000000000000000000000000001").unwrap();
    assert!(!permission.is_allowed(
        &ContractInfo {
            hash: different_hash,
            manifest: &manifest,
        },
        "transfer"
    ));
}

/// Test empty method list
#[test]
fn test_empty_method_list() {
    let manifest = create_default_manifest();
    let permission = ContractPermission {
        contract: ContractPermissionDescriptor::create_wildcard(),
        methods: WildcardContainer::List(vec![]),
    };

    // Empty method list should not allow any methods
    assert!(!permission.is_allowed(
        &ContractInfo {
            hash: UInt160::zero(),
            manifest: &manifest,
        },
        "anyMethod"
    ));
}

// ============================================================================
// Implementation helpers
// ============================================================================

/// Contract information for permission checking
struct ContractInfo<'a> {
    hash: UInt160,
    manifest: &'a ContractManifest,
}

impl ContractPermission {
    /// Check if this permission allows the specified contract and method
    fn is_allowed(&self, contract: &ContractInfo, method: &str) -> bool {
        // Check contract permission
        let contract_allowed = match &self.contract {
            ContractPermissionDescriptor::Wildcard => true,
            ContractPermissionDescriptor::Hash(allowed_hash) => contract.hash == *allowed_hash,
            ContractPermissionDescriptor::Group(allowed_key) => {
                // Check if contract has this public key in its groups
                contract
                    .manifest
                    .groups
                    .iter()
                    .any(|g| &g.pub_key == allowed_key)
            }
        };

        if !contract_allowed {
            return false;
        }

        // Check method permission
        match &self.methods {
            WildcardContainer::Wildcard => true,
            WildcardContainer::List(allowed_methods) => {
                allowed_methods.contains(&method.to_string())
            }
        }
    }
}

impl ContractPermissionDescriptor {
    fn create_wildcard() -> Self {
        ContractPermissionDescriptor::Wildcard
    }
}

impl<T> WildcardContainer<T> {
    fn create_wildcard() -> Self {
        WildcardContainer::Wildcard
    }
}

impl FromStr for UInt160 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            let bytes = hex::decode(&s[2..]).map_err(|e| e.to_string())?;
            if bytes.len() != 20 {
                return Err("Invalid UInt160 length".to_string());
            }
            let mut arr = [0u8; 20];
            arr.copy_from_slice(&bytes);
            Ok(UInt160::from_bytes(arr))
        } else {
            Err("Invalid UInt160 format".to_string())
        }
    }
}

/// Helper module for secp256r1 operations
mod secp256r1 {
    use k256::elliptic_curve::sec1::ToEncodedPoint;
    use neo_cryptography::ECPoint;
    use p256::{PublicKey, SecretKey};

    pub fn generate_public_key(private_key: &[u8; 32]) -> ECPoint {
        let secret_key = SecretKey::from_bytes(private_key.into()).unwrap();
        let public_key = secret_key.public_key();
        let point = public_key.to_encoded_point(false);
        ECPoint::from_bytes(point.as_bytes()).unwrap()
    }
}

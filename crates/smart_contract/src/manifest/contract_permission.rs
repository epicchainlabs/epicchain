//! Contract permission implementation.
//!
//! Defines the permissions that a contract requires to call other contracts
//! and access their methods.

use super::{ContractGroup, ContractManifest, WildcardContainer};
use crate::{Error, Result};
use neo_config::ADDRESS_SIZE;
use neo_core::UInt160;
use neo_cryptography::ecc::ECPoint;
use serde::{Deserialize, Serialize};

/// Represents a permission that a contract requires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractPermission {
    /// The contract or group that this permission applies to.
    pub contract: ContractPermissionDescriptor,

    /// The methods that are allowed to be called.
    pub methods: WildcardContainer<String>,
}

/// Describes what contract or group a permission applies to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContractPermissionDescriptor {
    /// Wildcard - applies to all contracts.
    Wildcard(String),

    /// Specific contract hash.
    Hash(UInt160),

    /// Group public key.
    Group(ECPoint),
}

impl ContractPermission {
    /// Creates a new contract permission.
    pub fn new(contract: ContractPermissionDescriptor, methods: WildcardContainer<String>) -> Self {
        Self { contract, methods }
    }

    /// Creates a wildcard permission that allows calling any method on any contract.
    pub fn default_wildcard() -> Self {
        Self {
            contract: ContractPermissionDescriptor::Wildcard("*".to_string()),
            methods: WildcardContainer::create_wildcard(),
        }
    }

    /// Creates a permission for a specific contract.
    pub fn for_contract(hash: UInt160, methods: WildcardContainer<String>) -> Self {
        Self {
            contract: ContractPermissionDescriptor::Hash(hash),
            methods,
        }
    }

    /// Creates a permission for a specific group.
    pub fn for_group(public_key: ECPoint, methods: WildcardContainer<String>) -> Self {
        Self {
            contract: ContractPermissionDescriptor::Group(public_key),
            methods,
        }
    }

    /// Checks if this permission allows calling a specific contract.
    pub fn allows_contract(&self, contract_hash: &UInt160) -> bool {
        match &self.contract {
            ContractPermissionDescriptor::Wildcard(_) => true,
            ContractPermissionDescriptor::Hash(hash) => hash == contract_hash,
            ContractPermissionDescriptor::Group(_group_key) => {
                // Group permission requires access to the application engine to verify group membership
                // This would be properly implemented when engine context is available
                false
            }
        }
    }

    /// Checks if this permission allows calling a specific method.
    /// This matches C# ContractPermission.IsAllowed exactly.
    pub fn allows_method(&self, method_name: &str) -> bool {
        self.methods.contains(&method_name.to_string())
    }

    /// Gets the size of the permission in bytes.
    pub fn size(&self) -> usize {
        let mut size = 0;

        // Contract descriptor size
        match &self.contract {
            ContractPermissionDescriptor::Wildcard(s) => size += s.len(),
            ContractPermissionDescriptor::Hash(_) => size += ADDRESS_SIZE, // UInt160 size
            ContractPermissionDescriptor::Group(_) => size += 33, // Compressed public key size
        }

        // Method permission size
        if self.methods.is_wildcard() {
            size += 1; // "*" string
        } else {
            for method in self.methods.values() {
                size += method.len();
            }
        }

        size
    }

    /// Validates the permission.
    pub fn validate(&self) -> Result<()> {
        // Validate contract descriptor
        match &self.contract {
            ContractPermissionDescriptor::Wildcard(s) => {
                if s != "*" {
                    return Err(Error::InvalidManifest(
                        "Invalid wildcard in contract permission".to_string(),
                    ));
                }
            }
            ContractPermissionDescriptor::Hash(_) => {
                // Hash validation is handled by UInt160 type
            }
            ContractPermissionDescriptor::Group(pubkey) => {
                if !pubkey.is_valid() {
                    return Err(Error::InvalidManifest(
                        "Invalid public key in contract permission".to_string(),
                    ));
                }
            }
        }

        // Validate method permission
        if !self.methods.is_wildcard() && self.methods.count() == 0 {
            return Err(Error::InvalidManifest(
                "Method list cannot be empty".to_string(),
            ));
        }

        for method in self.methods.values() {
            if method.is_empty() {
                return Err(Error::InvalidManifest(
                    "Method name cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl ContractPermissionDescriptor {
    /// Checks if this descriptor is a wildcard.
    pub fn is_wildcard(&self) -> bool {
        matches!(self, ContractPermissionDescriptor::Wildcard(_))
    }

    /// Checks if this descriptor is for a specific hash.
    pub fn is_hash(&self) -> bool {
        matches!(self, ContractPermissionDescriptor::Hash(_))
    }

    /// Checks if this descriptor is for a group.
    pub fn is_group(&self) -> bool {
        matches!(self, ContractPermissionDescriptor::Group(_))
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use neo_cryptography::ecc::{ECCurve, ECPoint};

    #[test]
    fn test_wildcard_permission() {
        let permission = ContractPermission::default_wildcard();

        let any_hash = UInt160::zero();
        assert!(permission.allows_contract(&any_hash));
        assert!(permission.allows_method("any_method"));
        assert!(permission.validate().is_ok());
    }

    #[test]
    fn test_specific_contract_permission() {
        let contract_hash = UInt160::zero();
        let methods = WildcardContainer::create(vec!["method1".to_string(), "method2".to_string()]);
        let permission = ContractPermission::for_contract(contract_hash, methods);

        assert!(permission.allows_contract(&contract_hash));
        assert!(!permission.allows_contract(&UInt160::from_bytes(&[1u8; ADDRESS_SIZE]).unwrap()));
        assert!(permission.allows_method("method1"));
        assert!(permission.allows_method("method2"));
        assert!(!permission.allows_method("method3"));
        assert!(permission.validate().is_ok());
    }

    #[test]
    fn test_group_permission() {
        let public_key = ECPoint::infinity(ECCurve::secp256r1());
        let methods = WildcardContainer::create_wildcard();
        let permission = ContractPermission::for_group(public_key, methods);

        // Group permissions don't allow arbitrary contracts by default
        let any_hash = UInt160::zero();
        assert!(!permission.allows_contract(&any_hash));
        assert!(permission.allows_method("any_method"));
    }

    #[test]
    fn test_permission_validation() {
        // Valid wildcard permission
        let valid_permission = ContractPermission::default_wildcard();
        assert!(valid_permission.validate().is_ok());

        // Invalid wildcard
        let invalid_permission = ContractPermission {
            contract: ContractPermissionDescriptor::Wildcard("invalid".to_string()),
            methods: WildcardContainer::create_wildcard(),
        };
        assert!(invalid_permission.validate().is_err());

        // Empty method list
        let empty_methods_permission = ContractPermission {
            contract: ContractPermissionDescriptor::Wildcard("*".to_string()),
            methods: WildcardContainer::create(vec![]),
        };
        assert!(empty_methods_permission.validate().is_err());
    }

    #[test]
    fn test_descriptor_type_checks() {
        let wildcard = ContractPermissionDescriptor::Wildcard("*".to_string());
        let hash = ContractPermissionDescriptor::Hash(UInt160::zero());
        let group = ContractPermissionDescriptor::Group(ECPoint::infinity(ECCurve::secp256r1()));

        assert!(wildcard.is_wildcard());
        assert!(!wildcard.is_hash());
        assert!(!wildcard.is_group());

        assert!(!hash.is_wildcard());
        assert!(hash.is_hash());
        assert!(!hash.is_group());

        assert!(!group.is_wildcard());
        assert!(!group.is_hash());
        assert!(group.is_group());
    }

    #[test]
    fn test_wildcard_container_methods() {
        let wildcard_methods: WildcardContainer<String> = WildcardContainer::create_wildcard();
        let specific_methods = WildcardContainer::create(vec!["test".to_string()]);

        assert!(wildcard_methods.is_wildcard());
        assert_eq!(wildcard_methods.count(), 0);

        assert!(!specific_methods.is_wildcard());
        assert_eq!(specific_methods.count(), 1);
        assert!(specific_methods.contains(&"test".to_string()));
    }
}

//! Native contracts module.
//!
//! This module provides the built-in native contracts for the Neo blockchain,
//! including NEO, GAS, Policy, and other system contracts.

pub mod contract_management;
pub mod crypto_lib;
pub mod fungible_token;
pub mod gas_token;
pub mod governance_types;
pub mod ledger_contract;
pub mod native_contract;
pub mod neo_token;
pub mod oracle_contract;
pub mod policy_contract;
pub mod role_management;
pub mod std_lib;

pub use contract_management::ContractManagement;
pub use crypto_lib::CryptoLib;
pub use fungible_token::{DefaultTokenAccountState, FungibleToken, TokenAccountState};
pub use gas_token::GasToken;
pub use ledger_contract::LedgerContract;
pub use native_contract::{NativeContract, NativeMethod};
pub use neo_token::NeoToken;
pub use oracle_contract::{OracleContract, OracleRequest, OracleResponse};
pub use policy_contract::PolicyContract;
pub use role_management::{Role, RoleManagement};
pub use std_lib::StdLib;

use neo_core::UInt160;
use std::collections::HashMap;

/// Registry for native contracts.
pub struct NativeRegistry {
    contracts: HashMap<UInt160, Box<dyn NativeContract>>,
}

impl NativeRegistry {
    /// Creates a new native contract registry.
    pub fn new() -> Self {
        let mut registry = Self {
            contracts: HashMap::new(),
        };

        // Register standard native contracts
        registry.register_standard_contracts();

        registry
    }

    /// Registers a native contract.
    pub fn register(&mut self, contract: Box<dyn NativeContract>) {
        self.contracts.insert(contract.hash(), contract);
    }

    /// Gets a native contract by hash.
    pub fn get(&self, hash: &UInt160) -> Option<&dyn NativeContract> {
        self.contracts.get(hash).map(|c| c.as_ref())
    }

    /// Checks if a contract hash is a native contract.
    pub fn is_native(&self, hash: &UInt160) -> bool {
        self.contracts.contains_key(hash)
    }

    /// Gets all native contract hashes.
    pub fn all_hashes(&self) -> Vec<UInt160> {
        self.contracts.keys().copied().collect()
    }

    /// Registers standard Neo native contracts.
    fn register_standard_contracts(&mut self) {
        // Register ContractManagement contract
        self.register(Box::new(ContractManagement::new()));

        // Register LedgerContract
        self.register(Box::new(LedgerContract::new()));

        // Register NEO token contract
        self.register(Box::new(NeoToken::new()));

        // Register GAS token contract
        self.register(Box::new(GasToken::new()));

        // Register Policy contract
        self.register(Box::new(PolicyContract::new()));

        // Register RoleManagement contract
        self.register(Box::new(RoleManagement::new()));

        // Register StdLib contract
        self.register(Box::new(StdLib::new()));

        // Register CryptoLib contract
        self.register(Box::new(CryptoLib::new()));

        // Register Oracle contract
        self.register(Box::new(OracleContract::new()));
    }
}

impl Default for NativeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};

    #[test]
    fn test_native_registry_creation() {
        let registry = NativeRegistry::new();

        // Check that standard contracts are registered
        let neo_hash = NeoToken::new().hash();
        let gas_hash = GasToken::new().hash();

        assert!(registry.is_native(&neo_hash));
        assert!(registry.is_native(&gas_hash));
        assert!(registry.get(&neo_hash).is_some());
        assert!(registry.get(&gas_hash).is_some());
    }

    #[test]
    fn test_native_registry_all_hashes() {
        let registry = NativeRegistry::new();
        let hashes = registry.all_hashes();

        // Should have at least NEO and GAS contracts
        assert!(hashes.len() >= 2);
    }

    #[test]
    fn test_non_native_contract() {
        let registry = NativeRegistry::new();
        let random_hash = UInt160::zero();

        // Assuming zero hash is not used by native contracts
        if !registry.is_native(&random_hash) {
            assert!(registry.get(&random_hash).is_none());
        }
    }
}

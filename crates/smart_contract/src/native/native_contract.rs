//! Base native contract trait and types.

use crate::application_engine::ApplicationEngine;
use crate::{Error, Result};
use neo_core::UInt160;
use serde::{Deserialize, Serialize};

/// Trait for native contract implementations.
pub trait NativeContract: Send + Sync {
    /// Gets the hash of the native contract.
    fn hash(&self) -> UInt160;

    /// Gets the name of the native contract.
    fn name(&self) -> &str;

    /// Gets the supported methods of the native contract.
    fn methods(&self) -> &[NativeMethod];

    /// Invokes a method on the native contract.
    fn invoke(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>>;

    /// Called when the contract is initialized.
    fn initialize(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        Ok(())
    }

    /// Called on each block persistence.
    fn on_persist(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        Ok(())
    }

    /// Called after block persistence.
    fn post_persist(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        Ok(())
    }
}

/// Represents a method in a native contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeMethod {
    /// The name of the method.
    pub name: String,

    /// The gas cost of the method.
    pub gas_cost: i64,

    /// Whether the method is safe (read-only).
    pub safe: bool,

    /// The required call flags for this method.
    pub required_call_flags: u8,
}

impl NativeMethod {
    /// Creates a new native method.
    pub fn new(name: String, gas_cost: i64, safe: bool, required_call_flags: u8) -> Self {
        Self {
            name,
            gas_cost,
            safe,
            required_call_flags,
        }
    }

    /// Creates a new safe (read-only) method.
    pub fn safe(name: String, gas_cost: i64) -> Self {
        Self::new(name, gas_cost, true, 0)
    }

    /// Creates a new unsafe (state-changing) method.
    pub fn unsafe_method(name: String, gas_cost: i64, required_call_flags: u8) -> Self {
        Self::new(name, gas_cost, false, required_call_flags)
    }
}

/// Base implementation for native contracts.
pub struct BaseNativeContract {
    /// The hash of the contract.
    pub hash: UInt160,

    /// The name of the contract.
    pub name: String,

    /// The supported methods.
    pub methods: Vec<NativeMethod>,
}

impl BaseNativeContract {
    /// Creates a new base native contract.
    pub fn new(hash: UInt160, name: String, methods: Vec<NativeMethod>) -> Self {
        Self {
            hash,
            name,
            methods,
        }
    }

    /// Finds a method by name.
    pub fn find_method(&self, name: &str) -> Option<&NativeMethod> {
        self.methods.iter().find(|m| m.name == name)
    }

    /// Validates that a method exists and can be called.
    pub fn validate_method_call(&self, method: &str) -> Result<&NativeMethod> {
        self.find_method(method)
            .ok_or_else(|| Error::NativeContractError(format!("Method not found: {}", method)))
    }
}

/// Macro to help implement native contracts.
#[macro_export]
macro_rules! impl_native_contract {
    ($contract:ty, $hash:expr, $name:expr, $methods:expr) => {
        impl NativeContract for $contract {
            fn hash(&self) -> UInt160 {
                $hash
            }

            fn name(&self) -> &str {
                $name
            }

            fn methods(&self) -> &[NativeMethod] {
                &$methods
            }

            fn invoke(
                &self,
                engine: &mut ApplicationEngine,
                method: &str,
                args: &[Vec<u8>],
            ) -> Result<Vec<u8>> {
                self.invoke_method(engine, method, args)
            }
        }
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_native_method_creation() {
        let method = NativeMethod::new("test".to_string(), 1000, true, 0);
        assert_eq!(method.name, "test");
        assert_eq!(method.gas_cost, 1000);
        assert!(method.safe);
        assert_eq!(method.required_call_flags, 0);
    }

    #[test]
    fn test_native_method_safe() {
        let method = NativeMethod::safe("test".to_string(), 1000);
        assert!(method.safe);
        assert_eq!(method.required_call_flags, 0);
    }

    #[test]
    fn test_native_method_unsafe() {
        let method = NativeMethod::unsafe_method("test".to_string(), 1000, 0x01);
        assert!(!method.safe);
        assert_eq!(method.required_call_flags, 0x01);
    }

    #[test]
    fn test_base_native_contract() {
        let hash = UInt160::zero();
        let name = "TestContract".to_string();
        let methods = vec![
            NativeMethod::safe("get".to_string(), 100),
            NativeMethod::unsafe_method("set".to_string(), 1000, 0x01),
        ];

        let contract = BaseNativeContract::new(hash, name.clone(), methods);

        assert_eq!(contract.hash, hash);
        assert_eq!(contract.name, name);
        assert_eq!(contract.methods.len(), 2);

        assert!(contract.find_method("get").is_some());
        assert!(contract.find_method("set").is_some());
        assert!(contract.find_method("nonexistent").is_none());

        assert!(contract.validate_method_call("get").is_ok());
        assert!(contract.validate_method_call("nonexistent").is_err());
    }
}

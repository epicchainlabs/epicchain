//! Interop services for smart contracts.
//!
//! This module provides interoperability services that smart contracts
//! can use to interact with the blockchain and external systems.

pub mod contract;
pub mod crypto;
pub mod runtime;
pub mod storage;

pub use contract::ContractService;
pub use crypto::CryptoService;
pub use runtime::RuntimeService;
pub use storage::StorageService;

use crate::application_engine::ApplicationEngine;
use crate::{Error, Result};

/// Trait for interop service implementations.
pub trait InteropService {
    /// Gets the name of the interop service.
    fn name(&self) -> &str;

    /// Gets the gas cost for this service.
    fn gas_cost(&self) -> i64;

    /// Executes the interop service.
    fn execute(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>>;
}

/// Registry for interop services.
pub struct InteropRegistry {
    services: std::collections::HashMap<String, Box<dyn InteropService>>,
}

impl InteropRegistry {
    /// Creates a new interop registry.
    pub fn new() -> Self {
        let mut registry = Self {
            services: std::collections::HashMap::new(),
        };

        // Register standard services
        registry.register_standard_services();

        registry
    }

    /// Registers a new interop service.
    pub fn register(&mut self, service: Box<dyn InteropService>) {
        self.services.insert(service.name().to_string(), service);
    }

    /// Gets an interop service by name.
    pub fn get(&self, name: &str) -> Option<&dyn InteropService> {
        self.services.get(name).map(|s| s.as_ref())
    }

    /// Executes an interop service.
    pub fn execute(
        &self,
        name: &str,
        engine: &mut ApplicationEngine,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        let service = self
            .get(name)
            .ok_or_else(|| Error::InteropServiceError(format!("Service not found: {}", name)))?;

        // Check and consume gas
        engine.consume_gas(service.gas_cost())?;

        // Execute the service
        service.execute(engine, args)
    }

    /// Registers standard Neo interop services.
    fn register_standard_services(&mut self) {
        // Runtime services
        self.register(Box::new(runtime::LogService));
        self.register(Box::new(runtime::NotifyService));
        self.register(Box::new(runtime::GetTimeService));

        // Storage services
        self.register(Box::new(storage::GetService));
        self.register(Box::new(storage::PutService));
        self.register(Box::new(storage::DeleteService));

        // Contract services
        self.register(Box::new(contract::CallService));
        self.register(Box::new(contract::GetContractService));

        // Crypto services
        self.register(Box::new(crypto::CheckSigService));
        self.register(Box::new(crypto::CheckMultiSigService));
    }
}

impl Default for InteropRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};

    #[test]
    fn test_interop_registry_creation() {
        let registry = InteropRegistry::new();

        // Check that standard services are registered
        assert!(registry.get("System.Runtime.Log").is_some());
        assert!(registry.get("System.Storage.Get").is_some());
        assert!(registry.get("System.Contract.Call").is_some());
        assert!(registry.get("System.Crypto.CheckSig").is_some());
    }

    #[test]
    fn test_interop_service_execution() {
        let registry = InteropRegistry::new();
        let mut engine = ApplicationEngine::new(neo_vm::TriggerType::Application, 10_000_000);

        engine.set_current_script_hash(Some(neo_core::UInt160::zero()));

        // Test log service
        let result = registry.execute(
            "System.Runtime.Log",
            &mut engine,
            &[b"test message".to_vec()],
        );
        assert!(result.is_ok());
    }
}

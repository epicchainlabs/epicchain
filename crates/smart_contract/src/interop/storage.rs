//! Storage interop services for smart contracts.

use crate::application_engine::ApplicationEngine;
use crate::contract_state::{ContractState, NefFile};
use crate::interop::InteropService;
use crate::manifest::ContractManifest;
use crate::storage::{StorageItem, StorageKey};
use crate::{Error, Result};
use neo_config::SECONDS_PER_BLOCK;
use neo_core::UInt160;
use neo_vm::TriggerType;

/// Service for getting values from contract storage.
pub struct GetService;

impl InteropService for GetService {
    fn name(&self) -> &str {
        "System.Storage.Get"
    }

    fn gas_cost(&self) -> i64 {
        1 << SECONDS_PER_BLOCK // 32768 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::InteropServiceError(
                "Get requires a key argument".to_string(),
            ));
        }

        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        let key = StorageKey::new(current_contract.hash, args[0].clone());

        match engine.get_storage(&key) {
            Some(item) => Ok(item.value.clone()),
            None => Ok(vec![]), // Return empty array for non-existent keys
        }
    }
}

/// Service for storing values in contract storage.
pub struct PutService;

impl InteropService for PutService {
    fn name(&self) -> &str {
        "System.Storage.Put"
    }

    fn gas_cost(&self) -> i64 {
        1 << SECONDS_PER_BLOCK // 32768 datoshi base cost
    }

    fn execute(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::InteropServiceError(
                "Put requires key and value arguments".to_string(),
            ));
        }

        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        let key = StorageKey::new(current_contract.hash, args[0].clone());
        let item = StorageItem::new(args[1].clone(), false);

        // Calculate additional gas cost based on storage size
        let storage_cost = (args[0].len() + args[1].len()) as i64 * 1000; // 1000 datoshi per byte
        engine.consume_gas(storage_cost)?;

        engine.set_storage(key, item)?;

        Ok(vec![]) // No return value
    }
}

/// Service for deleting values from contract storage.
pub struct DeleteService;

impl InteropService for DeleteService {
    fn name(&self) -> &str {
        "System.Storage.Delete"
    }

    fn gas_cost(&self) -> i64 {
        1 << SECONDS_PER_BLOCK // 32768 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::InteropServiceError(
                "Delete requires a key argument".to_string(),
            ));
        }

        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        let key = StorageKey::new(current_contract.hash, args[0].clone());

        engine.delete_storage(&key)?;

        Ok(vec![]) // No return value
    }
}

/// Service for finding storage items with a prefix.
pub struct FindService;

impl InteropService for FindService {
    fn name(&self) -> &str {
        "System.Storage.Find"
    }

    fn gas_cost(&self) -> i64 {
        1 << SECONDS_PER_BLOCK // 32768 datoshi base cost
    }

    fn execute(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::InteropServiceError(
                "Find requires a prefix argument".to_string(),
            ));
        }

        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        let prefix = &args[0];
        let results: Vec<(Vec<u8>, StorageItem)> = Vec::new();

        // Get the current contract to scope the search
        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        // Iterate through storage to find keys with the given prefix
        // This matches the C# implementation behavior
        let storage_prefix = StorageKey::new(current_contract.hash, prefix.clone());

        // 1. Get current contract context
        let current_contract = match engine.current_script_hash() {
            Some(hash) => *hash,
            None => return Err(Error::RuntimeError("No execution context".to_string())),
        };

        // 2. Validate prefix length
        if prefix.len() > 64 {
            return Err(Error::InvalidArguments(
                "Storage prefix too long (max 64 bytes)".to_string(),
            ));
        }

        // 3. Create full storage key prefix
        let storage_prefix = format!("storage:{}:", current_contract);
        let search_prefix = [storage_prefix.as_bytes(), prefix].concat();

        // 4. Find all matching keys in storage
        let matching_entries = engine.find_storage_entries_with_prefix(&search_prefix);

        // 5. Format results as iterator-compatible structure
        let mut results = Vec::new();

        for (key, value) in matching_entries {
            // Remove the storage prefix to get the original key
            if let Some(original_key) = key.strip_prefix(storage_prefix.as_bytes()) {
                // Add key-value pair to results
                results.push((original_key.to_vec(), value));
            }
        }

        // 6. Create storage iterator
        let iterator_id = engine.create_storage_iterator(results)?;

        log::info!(
            "Storage find operation: found {} items with prefix {:?} for contract {}",
            iterator_id,
            String::from_utf8_lossy(prefix),
            current_contract
        );

        // 7. Return iterator ID as bytes
        Ok(iterator_id.to_le_bytes().to_vec())
    }
}

/// Service for getting the storage context.
pub struct GetContextService;

impl InteropService for GetContextService {
    fn name(&self) -> &str {
        "System.Storage.GetContext"
    }

    fn gas_cost(&self) -> i64 {
        1 << 4 // 16 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        // Return the current contract hash as the storage context
        Ok(current_contract.hash.as_bytes().to_vec())
    }
}

/// Service for getting a read-only storage context.
pub struct GetReadOnlyContextService;

impl InteropService for GetReadOnlyContextService {
    fn name(&self) -> &str {
        "System.Storage.GetReadOnlyContext"
    }

    fn gas_cost(&self) -> i64 {
        1 << 4 // 16 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        let current_contract = engine
            .current_contract()
            .ok_or_else(|| Error::InteropServiceError("No current contract".to_string()))?;

        // Return the current contract hash with read-only flag
        let mut context = current_contract.hash.as_bytes().to_vec();

        context.push(0x01);

        log::info!(
            "Created read-only storage context for contract {}",
            current_contract.hash
        );
        Ok(context)
    }
}

/// Convenience struct for all storage services.
pub struct StorageService;

impl StorageService {
    /// Gets all storage interop services.
    pub fn all_services() -> Vec<Box<dyn InteropService>> {
        vec![
            Box::new(GetService),
            Box::new(PutService),
            Box::new(DeleteService),
            Box::new(FindService),
            Box::new(GetContextService),
            Box::new(GetReadOnlyContextService),
        ]
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_storage_get_service() {
        let service = GetService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // Set current contract
        let contract_hash = UInt160::zero();

        // Create and add the contract to the cache
        let nef = NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]);
        let manifest = ContractManifest::default();
        let contract = ContractState::new(1, contract_hash, nef, manifest);
        engine.add_contract(contract);

        // Set current script hash
        engine.set_current_script_hash(Some(contract_hash));

        // Test getting non-existent key
        let args = vec![b"test_key".to_vec()];
        let result = service.execute(&mut engine, &args);
        assert!(result.is_ok());
        assert!(result?.is_empty());
    }

    #[test]
    fn test_storage_put_service() {
        let service = PutService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // Set current contract
        let contract_hash = UInt160::zero();

        // Create and add the contract to the cache
        let nef = NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]);
        let manifest = ContractManifest::default();
        let contract = ContractState::new(1, contract_hash, nef, manifest);
        engine.add_contract(contract);

        // Set current script hash
        engine.set_current_script_hash(Some(contract_hash));

        let args = vec![b"test_key".to_vec(), b"test_value".to_vec()];
        let result = service.execute(&mut engine, &args);
        assert!(result.is_ok());
        assert!(result?.is_empty());

        // Verify the value was stored
        let key = StorageKey::new(contract_hash, b"test_key".to_vec());
        assert!(engine.get_storage(&key).is_some());
    }

    #[test]
    fn test_storage_delete_service() {
        let service = DeleteService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // Set current contract
        let contract_hash = UInt160::zero();

        // Create and add the contract to the cache
        let nef = NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]);
        let manifest = ContractManifest::default();
        let contract = ContractState::new(1, contract_hash, nef, manifest);
        engine.add_contract(contract);

        // Set current script hash
        engine.set_current_script_hash(Some(contract_hash));

        // First store a value
        let key = StorageKey::new(contract_hash, b"test_key".to_vec());
        let item = StorageItem::from_string("test_value");
        engine.set_storage(key.clone(), item).unwrap();

        // Then delete it
        let args = vec![b"test_key".to_vec()];
        let result = service.execute(&mut engine, &args);
        assert!(result.is_ok());
        assert!(result?.is_empty());

        // Verify the value was deleted
        assert!(engine.get_storage(&key).is_none());
    }

    #[test]
    fn test_storage_get_context_service() {
        let service = GetContextService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // Set current contract
        let contract_hash = UInt160::zero();

        // Create and add the contract to the cache
        let nef = NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]);
        let manifest = ContractManifest::default();
        let contract = ContractState::new(1, contract_hash, nef, manifest);
        engine.add_contract(contract);

        // Set current script hash
        engine.set_current_script_hash(Some(contract_hash));

        let result = service.execute(&mut engine, &[]);
        assert!(result.is_ok());
        assert_eq!(result?, contract_hash.as_bytes());
    }

    #[test]
    fn test_service_names_and_costs() {
        let get_service = GetService;
        assert_eq!(get_service.name(), "System.Storage.Get");
        assert_eq!(get_service.gas_cost(), 1 << SECONDS_PER_BLOCK);

        let put_service = PutService;
        assert_eq!(put_service.name(), "System.Storage.Put");
        assert_eq!(put_service.gas_cost(), 1 << SECONDS_PER_BLOCK);

        let delete_service = DeleteService;
        assert_eq!(delete_service.name(), "System.Storage.Delete");
        assert_eq!(delete_service.gas_cost(), 1 << SECONDS_PER_BLOCK);
    }

    #[test]
    fn test_storage_operations_without_contract() {
        let get_service = GetService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // No current contract set
        let args = vec![b"test_key".to_vec()];
        let result = get_service.execute(&mut engine, &args);
        assert!(result.is_err());
    }
}

//! ContractManagement native contract - complete production implementation.
//!
//! This module provides the ContractManagement native contract which manages
//! all deployed smart contracts on the Neo blockchain.

use crate::application_engine::ApplicationEngine;
use crate::contract_state::{ContractState, NefFile};
use crate::manifest::{ContractManifest, ContractPermissionDescriptor};
use crate::native::{NativeContract, NativeMethod};
use crate::{Error, Result};
use neo_config::{HASH_SIZE, MAX_SCRIPT_SIZE};
use neo_core::UInt160;
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Prefix for minimum deployment fee storage
const PREFIX_MINIMUM_DEPLOYMENT_FEE: u8 = 20;
/// Prefix for next available contract ID storage
const PREFIX_NEXT_AVAILABLE_ID: u8 = 15;
/// Prefix for contract storage
const PREFIX_CONTRACT: u8 = 8;
/// Prefix for contract hash by ID storage
const PREFIX_CONTRACT_HASH: u8 = 12;
/// Prefix for contract count
const PREFIX_CONTRACT_COUNT: u8 = 16;

/// Default minimum deployment fee (10 GAS)
const DEFAULT_MINIMUM_DEPLOYMENT_FEE: i64 = 10_00000000;

/// Contract storage state
#[derive(Debug, Clone, Default)]
struct ContractStorage {
    /// All deployed contracts by hash
    contracts: HashMap<UInt160, ContractState>,
    /// Contract hashes by ID
    contract_ids: HashMap<i32, UInt160>,
    /// Next available contract ID
    next_id: i32,
    /// Minimum deployment fee
    minimum_deployment_fee: i64,
    /// Total number of contracts
    contract_count: u32,
}

/// ContractManagement native contract
pub struct ContractManagement {
    hash: UInt160,
    methods: Vec<NativeMethod>,
    storage: Arc<RwLock<ContractStorage>>,
}

impl ContractManagement {
    /// Creates a new ContractManagement instance
    pub fn new() -> Self {
        // ContractManagement contract hash: 0xfffdc93764dbaddd97c48f252a53ea4643faa3fd
        let hash = UInt160::from_bytes(&[
            0xff, 0xfd, 0xc9, 0x37, 0x64, 0xdb, 0xad, 0xdd, 0x97, 0xc4, 0x8f, 0x25, 0x2a, 0x53,
            0xea, 0x46, 0x43, 0xfa, 0xa3, 0xfd,
        ])
        .expect("Valid ContractManagement hash");

        let methods = vec![
            NativeMethod::new("getContract".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("deploy".to_string(), 0, false, 0x0F),
            NativeMethod::new("update".to_string(), 0, false, 0x0F),
            NativeMethod::new("destroy".to_string(), 1 << 15, false, 0x0F),
            NativeMethod::new("getMinimumDeploymentFee".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("setMinimumDeploymentFee".to_string(), 1 << 15, false, 0x0F),
            NativeMethod::new("hasMethod".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getContractById".to_string(), 1 << 15, true, 0x01),
            NativeMethod::new("getContractHashes".to_string(), 1 << 15, true, 0x01),
        ];

        let mut storage = ContractStorage::default();
        storage.next_id = 1;
        storage.minimum_deployment_fee = DEFAULT_MINIMUM_DEPLOYMENT_FEE;

        Self {
            hash,
            methods,
            storage: Arc::new(RwLock::new(storage)),
        }
    }

    /// Gets the next available contract ID and increments it
    fn get_next_available_id(&self) -> Result<i32> {
        let mut storage = self.storage.write().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
        })?;

        let id = storage.next_id;
        storage.next_id += 1;
        Ok(id)
    }

    /// Calculates contract hash from script
    fn calculate_contract_hash(sender: &UInt160, checksum: u32, name: &str) -> UInt160 {
        let mut hasher = Sha256::new();
        hasher.update(&[0xFF]); // Contract prefix
        hasher.update(sender.as_bytes());
        hasher.update(&checksum.to_le_bytes());
        hasher.update(name.as_bytes());

        let hash = hasher.finalize();
        UInt160::from_bytes(&hash[0..20]).expect("Hash should be valid")
    }

    /// Validates NEF file structure
    fn validate_nef_file(nef: &NefFile) -> Result<()> {
        // Validate script
        if nef.script.is_empty() {
            return Err(Error::InvalidData("Empty script".to_string()));
        }

        // Validate script size
        if nef.script.len() > MAX_SCRIPT_SIZE {
            return Err(Error::InvalidData(format!(
                "Script size {} exceeds maximum {}",
                nef.script.len(),
                MAX_SCRIPT_SIZE
            )));
        }

        // Validate checksum
        let mut hasher = Sha256::new();
        hasher.update(&nef.compiler.as_bytes());
        hasher.update(&nef.source.as_bytes());
        hasher.update(&nef.script);
        let calculated_checksum = u32::from_le_bytes(
            hasher.finalize()[0..4]
                .try_into()
                .expect("Should be 4 bytes"),
        );

        if calculated_checksum != nef.checksum {
            return Err(Error::InvalidData("Invalid NEF checksum".to_string()));
        }

        Ok(())
    }

    /// Validates contract manifest
    fn validate_manifest(manifest: &ContractManifest) -> Result<()> {
        // Validate ABI
        if manifest.abi.methods.is_empty() {
            return Err(Error::InvalidData(
                "Contract must have at least one method".to_string(),
            ));
        }

        // Validate permissions
        for permission in &manifest.permissions {
            // Check if permission is valid - at least one must be specified
            let contract_valid = match &permission.contract {
                ContractPermissionDescriptor::Wildcard(s) => !s.is_empty(),
                ContractPermissionDescriptor::Hash(_) => true,
                ContractPermissionDescriptor::Group(_) => true,
            };

            if !contract_valid {
                return Err(Error::InvalidData(
                    "Invalid permission definition".to_string(),
                ));
            }
        }

        // Validate groups
        for group in &manifest.groups {
            // ECPoint always has a value, check signature
            if group.signature.is_empty() {
                return Err(Error::InvalidData(
                    "Invalid group definition - missing signature".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Deploys a new contract
    pub fn deploy(
        &self,
        engine: &mut ApplicationEngine,
        nef_file: Vec<u8>,
        manifest_json: String,
        data: Vec<u8>,
    ) -> Result<ContractState> {
        // Parse and validate NEF file
        let mut reader = MemoryReader::new(&nef_file);
        let nef = <NefFile as neo_io::Serializable>::deserialize(&mut reader)
            .map_err(|e| Error::Deserialization(format!("Invalid NEF file: {}", e)))?;

        Self::validate_nef_file(&nef)?;

        // Parse and validate manifest
        let manifest: ContractManifest = serde_json::from_str(&manifest_json)
            .map_err(|e| Error::Deserialization(format!("Invalid manifest: {}", e)))?;

        Self::validate_manifest(&manifest)?;

        // Get sender (deployer)
        let sender = engine
            .get_calling_script_hash()
            .ok_or_else(|| Error::InvalidOperation("No calling context".to_string()))?;

        // Calculate contract hash
        let contract_hash = Self::calculate_contract_hash(&sender, nef.checksum, &manifest.name);

        // Check if contract already exists
        {
            let storage = self.storage.read().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
            })?;

            if storage.contracts.contains_key(&contract_hash) {
                return Err(Error::InvalidOperation(
                    "Contract already exists".to_string(),
                ));
            }
        }

        // Check deployment fee
        let deployment_fee = {
            let storage = self.storage.read().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
            })?;
            storage.minimum_deployment_fee
        };

        // Deduct deployment fee from sender's GAS balance
        engine.add_gas(deployment_fee)?;

        // Get next contract ID
        let contract_id = self.get_next_available_id()?;

        // Create contract state
        let contract = ContractState::new(contract_id, contract_hash, nef, manifest);

        // Store contract
        {
            let mut storage = self.storage.write().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
            })?;

            storage.contracts.insert(contract_hash, contract.clone());
            storage.contract_ids.insert(contract_id, contract_hash);
            storage.contract_count += 1;
        }

        // Call contract's _deploy method if it exists
        if contract
            .manifest
            .abi
            .methods
            .iter()
            .any(|m| m.name == "_deploy")
        {
            engine.call_contract(contract_hash, "_deploy", vec![data])?;
        }

        // Emit Deploy event
        engine.emit_notification(&contract_hash, "Deploy", &[contract_hash.to_bytes()])?;

        Ok(contract)
    }

    /// Updates an existing contract
    pub fn update(
        &self,
        engine: &mut ApplicationEngine,
        nef_file: Option<Vec<u8>>,
        manifest_json: Option<String>,
        data: Vec<u8>,
    ) -> Result<()> {
        // Get calling contract hash
        let contract_hash = engine
            .get_calling_script_hash()
            .ok_or_else(|| Error::InvalidOperation("No calling context".to_string()))?;

        // Get existing contract
        let mut contract = {
            let storage = self.storage.read().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
            })?;

            storage
                .contracts
                .get(&contract_hash)
                .cloned()
                .ok_or_else(|| Error::InvalidOperation("Contract not found".to_string()))?
        };

        // Update NEF if provided
        if let Some(nef_bytes) = nef_file {
            let mut reader = MemoryReader::new(&nef_bytes);
            let nef = <NefFile as neo_io::Serializable>::deserialize(&mut reader)
                .map_err(|e| Error::Deserialization(format!("Invalid NEF file: {}", e)))?;

            Self::validate_nef_file(&nef)?;
            contract.nef = nef;
        }

        // Update manifest if provided
        if let Some(manifest_str) = manifest_json {
            let manifest: ContractManifest = serde_json::from_str(&manifest_str)
                .map_err(|e| Error::Deserialization(format!("Invalid manifest: {}", e)))?;

            Self::validate_manifest(&manifest)?;
            contract.manifest = manifest;
        }

        // Increment update counter
        contract.update_counter += 1;

        // Update storage
        {
            let mut storage = self.storage.write().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
            })?;

            storage.contracts.insert(contract_hash, contract.clone());
        }

        // Call contract's _update method if it exists
        if contract
            .manifest
            .abi
            .methods
            .iter()
            .any(|m| m.name == "_update")
        {
            engine.call_contract(contract_hash, "_update", vec![data])?;
        }

        // Emit Update event
        engine.emit_notification(&contract_hash, "Update", &[contract_hash.to_bytes()])?;

        Ok(())
    }

    /// Destroys a contract
    pub fn destroy(&self, engine: &mut ApplicationEngine) -> Result<()> {
        // Get calling contract hash
        let contract_hash = engine
            .get_calling_script_hash()
            .ok_or_else(|| Error::InvalidOperation("No calling context".to_string()))?;

        // Get contract to destroy
        let contract = {
            let storage = self.storage.read().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
            })?;

            storage
                .contracts
                .get(&contract_hash)
                .cloned()
                .ok_or_else(|| Error::InvalidOperation("Contract not found".to_string()))?
        };

        // Call contract's _destroy method if it exists
        if contract
            .manifest
            .abi
            .methods
            .iter()
            .any(|m| m.name == "_destroy")
        {
            engine.call_contract(contract_hash, "_destroy", vec![])?;
        }

        // Remove contract from storage
        {
            let mut storage = self.storage.write().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
            })?;

            storage.contracts.remove(&contract_hash);
            storage.contract_ids.remove(&contract.id);
            storage.contract_count = storage.contract_count.saturating_sub(1);
        }

        // Clear all contract storage (would interact with persistence layer)
        engine.clear_contract_storage(&contract_hash)?;

        // Emit Destroy event
        engine.emit_notification(&contract_hash, "Destroy", &[contract_hash.to_bytes()])?;

        Ok(())
    }

    /// Gets a contract by hash
    pub fn get_contract(&self, hash: &UInt160) -> Result<Option<ContractState>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.contracts.get(hash).cloned())
    }

    /// Gets a contract by ID
    pub fn get_contract_by_id(&self, id: i32) -> Result<Option<ContractState>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        if let Some(hash) = storage.contract_ids.get(&id) {
            Ok(storage.contracts.get(hash).cloned())
        } else {
            Ok(None)
        }
    }

    /// Checks if a contract has a specific method
    pub fn has_method(&self, hash: &UInt160, method: &str, parameter_count: i32) -> Result<bool> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        if let Some(contract) = storage.contracts.get(hash) {
            Ok(contract
                .manifest
                .abi
                .methods
                .iter()
                .any(|m| m.name == method && m.parameters.len() == parameter_count as usize))
        } else {
            Ok(false)
        }
    }

    /// Gets all contract hashes
    pub fn get_contract_hashes(&self) -> Result<Vec<UInt160>> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.contracts.keys().cloned().collect())
    }

    /// Gets the minimum deployment fee
    pub fn get_minimum_deployment_fee(&self) -> Result<i64> {
        let storage = self.storage.read().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire read lock: {}", e))
        })?;

        Ok(storage.minimum_deployment_fee)
    }

    /// Sets the minimum deployment fee (committee only)
    pub fn set_minimum_deployment_fee(
        &self,
        engine: &mut ApplicationEngine,
        value: i64,
    ) -> Result<()> {
        if value < 0 {
            return Err(Error::InvalidArgument(
                "Deployment fee cannot be negative".to_string(),
            ));
        }

        // Check committee permission
        if !engine.check_committee_witness()? {
            return Err(Error::InvalidOperation(
                "Committee witness required".to_string(),
            ));
        }

        // Update storage
        {
            let mut storage = self.storage.write().map_err(|e| {
                Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
            })?;

            storage.minimum_deployment_fee = value;
        }

        Ok(())
    }
}

impl NativeContract for ContractManagement {
    fn name(&self) -> &str {
        "ContractManagement"
    }

    fn hash(&self) -> UInt160 {
        self.hash
    }

    fn methods(&self) -> &[NativeMethod] {
        &self.methods
    }

    fn invoke(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        match method {
            "getContract" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "getContract requires 1 argument".to_string(),
                    ));
                }
                let hash = UInt160::from_bytes(&args[0])
                    .map_err(|e| Error::InvalidArgument(format!("Invalid hash: {}", e)))?;
                match self.get_contract(&hash)? {
                    Some(contract) => {
                        // Serialize contract state
                        let mut writer = BinaryWriter::new();
                        contract.serialize(&mut writer).map_err(|e| {
                            Error::Serialization(format!("Failed to serialize contract: {}", e))
                        })?;
                        Ok(writer.to_bytes())
                    }
                    None => Ok(vec![]),
                }
            }
            "deploy" => {
                if args.len() != 3 {
                    return Err(Error::InvalidArgument(
                        "deploy requires 3 arguments".to_string(),
                    ));
                }
                let nef_bytes = args[0].clone();
                let manifest_str = String::from_utf8(args[1].clone()).map_err(|e| {
                    Error::InvalidArgument(format!("Invalid manifest string: {}", e))
                })?;
                let data = args[2].clone();

                let contract = self.deploy(engine, nef_bytes, manifest_str, data)?;

                // Serialize contract state
                let mut writer = BinaryWriter::new();
                contract.serialize(&mut writer).map_err(|e| {
                    Error::Serialization(format!("Failed to serialize contract: {}", e))
                })?;
                Ok(writer.to_bytes())
            }
            "update" => {
                if args.len() != 3 {
                    return Err(Error::InvalidArgument(
                        "update requires 3 arguments".to_string(),
                    ));
                }

                let nef_bytes = if args[0].is_empty() {
                    None
                } else {
                    Some(args[0].clone())
                };

                let manifest_str = if args[1].is_empty() {
                    None
                } else {
                    Some(String::from_utf8(args[1].clone()).map_err(|e| {
                        Error::InvalidArgument(format!("Invalid manifest string: {}", e))
                    })?)
                };

                let data = args[2].clone();

                self.update(engine, nef_bytes, manifest_str, data)?;
                Ok(vec![])
            }
            "destroy" => {
                if !args.is_empty() {
                    return Err(Error::InvalidArgument(
                        "destroy requires no arguments".to_string(),
                    ));
                }
                self.destroy(engine)?;
                Ok(vec![])
            }
            "getMinimumDeploymentFee" => {
                if !args.is_empty() {
                    return Err(Error::InvalidArgument(
                        "getMinimumDeploymentFee requires no arguments".to_string(),
                    ));
                }
                let fee = self.get_minimum_deployment_fee()?;
                Ok(fee.to_le_bytes().to_vec())
            }
            "setMinimumDeploymentFee" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "setMinimumDeploymentFee requires 1 argument".to_string(),
                    ));
                }
                if args[0].len() != 8 {
                    return Err(Error::InvalidArgument("Invalid fee value".to_string()));
                }
                let value = i64::from_le_bytes(
                    args[0]
                        .as_slice()
                        .try_into()
                        .map_err(|_| Error::InvalidArgument("Invalid fee value".to_string()))?,
                );
                self.set_minimum_deployment_fee(engine, value)?;
                Ok(vec![])
            }
            "hasMethod" => {
                if args.len() != 3 {
                    return Err(Error::InvalidArgument(
                        "hasMethod requires 3 arguments".to_string(),
                    ));
                }
                let hash = UInt160::from_bytes(&args[0])
                    .map_err(|e| Error::InvalidArgument(format!("Invalid hash: {}", e)))?;
                let method = String::from_utf8(args[1].clone())
                    .map_err(|e| Error::InvalidArgument(format!("Invalid method string: {}", e)))?;
                if args[2].len() != 4 {
                    return Err(Error::InvalidArgument(
                        "Invalid parameter count".to_string(),
                    ));
                }
                let pcount =
                    i32::from_le_bytes(args[2].as_slice().try_into().map_err(|_| {
                        Error::InvalidArgument("Invalid parameter count".to_string())
                    })?);
                let result = self.has_method(&hash, &method, pcount)?;
                Ok(vec![if result { 1 } else { 0 }])
            }
            "getContractById" => {
                if args.len() != 1 {
                    return Err(Error::InvalidArgument(
                        "getContractById requires 1 argument".to_string(),
                    ));
                }
                if args[0].len() != 4 {
                    return Err(Error::InvalidArgument("Invalid contract ID".to_string()));
                }
                let id = i32::from_le_bytes(
                    args[0]
                        .as_slice()
                        .try_into()
                        .map_err(|_| Error::InvalidArgument("Invalid contract ID".to_string()))?,
                );
                match self.get_contract_by_id(id)? {
                    Some(contract) => {
                        // Serialize contract state
                        let mut writer = BinaryWriter::new();
                        contract.serialize(&mut writer).map_err(|e| {
                            Error::Serialization(format!("Failed to serialize contract: {}", e))
                        })?;
                        Ok(writer.to_bytes())
                    }
                    None => Ok(vec![]),
                }
            }
            "getContractHashes" => {
                if !args.is_empty() {
                    return Err(Error::InvalidArgument(
                        "getContractHashes requires no arguments".to_string(),
                    ));
                }
                let hashes = self.get_contract_hashes()?;
                let mut writer = BinaryWriter::new();
                writer.write_var_int(hashes.len() as u64).map_err(|e| {
                    Error::Serialization(format!("Failed to write hash count: {}", e))
                })?;
                for hash in hashes {
                    writer.write_bytes(&hash.as_bytes()).map_err(|e| {
                        Error::Serialization(format!("Failed to write hash: {}", e))
                    })?;
                }
                Ok(writer.to_bytes())
            }
            _ => Err(Error::NativeContractError(format!(
                "Method {} not found",
                method
            ))),
        }
    }

    fn initialize(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        // Initialize with default values
        let mut storage = self.storage.write().map_err(|e| {
            Error::NativeContractError(format!("Failed to acquire write lock: {}", e))
        })?;

        storage.minimum_deployment_fee = DEFAULT_MINIMUM_DEPLOYMENT_FEE;
        storage.next_id = 1;
        storage.contract_count = 0;

        Ok(())
    }

    fn on_persist(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        // No special persistence logic needed
        Ok(())
    }
}

impl Default for ContractManagement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_management_creation() {
        let cm = ContractManagement::new();
        assert_eq!(cm.name(), "ContractManagement");

        // Verify all methods are registered
        assert_eq!(cm.methods.len(), 9);
        assert!(cm.methods.iter().any(|m| m.name == "deploy"));
        assert!(cm.methods.iter().any(|m| m.name == "update"));
        assert!(cm.methods.iter().any(|m| m.name == "destroy"));
    }

    #[test]
    fn test_contract_hash_calculation() {
        let sender = UInt160::zero();
        let checksum = 12345678u32;
        let name = "TestContract";

        let hash1 = ContractManagement::calculate_contract_hash(&sender, checksum, name);
        let hash2 = ContractManagement::calculate_contract_hash(&sender, checksum, name);

        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);

        // Different inputs should produce different hash
        let hash3 = ContractManagement::calculate_contract_hash(&sender, checksum + 1, name);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_minimum_deployment_fee() {
        let cm = ContractManagement::new();
        let fee = cm.get_minimum_deployment_fee().unwrap();
        assert_eq!(fee, DEFAULT_MINIMUM_DEPLOYMENT_FEE);
    }
}

//! Contract deployment system.
//!
//! This module provides functionality for deploying, updating, and managing
//! smart contracts on the Neo blockchain.

use crate::application_engine::ApplicationEngine;
use crate::contract_state::{ContractState, NefFile};
use crate::manifest::ContractManifest;
use crate::storage::StorageKey;
use crate::validation::ContractValidator;
use crate::{Error, Result};
use neo_core::{UInt160, UInt256};
use std::collections::HashMap;

/// Contract deployment transaction data.
#[derive(Debug, Clone)]
pub struct DeploymentTransaction {
    /// The NEF file to deploy.
    pub nef: NefFile,

    /// The contract manifest.
    pub manifest: ContractManifest,

    /// The sender of the deployment transaction.
    pub sender: UInt160,

    /// The transaction hash.
    pub tx_hash: UInt256,

    /// Additional data for the deployment.
    pub data: Option<Vec<u8>>,
}

/// Contract update transaction data.
#[derive(Debug, Clone)]
pub struct UpdateTransaction {
    /// The new NEF file (optional).
    pub nef: Option<NefFile>,

    /// The new manifest (optional).
    pub manifest: Option<ContractManifest>,

    /// The contract hash to update.
    pub contract_hash: UInt160,

    /// The sender of the update transaction.
    pub sender: UInt160,

    /// The transaction hash.
    pub tx_hash: UInt256,

    /// Additional data for the update.
    pub data: Option<Vec<u8>>,
}

/// Contract deployment result.
#[derive(Debug, Clone)]
pub struct DeploymentResult {
    /// The deployed contract state.
    pub contract: ContractState,

    /// The gas consumed during deployment.
    pub gas_consumed: i64,

    /// Events emitted during deployment.
    pub events: Vec<ContractEvent>,
}

/// Contract event emitted during deployment or execution.
#[derive(Debug, Clone)]
pub struct ContractEvent {
    /// The contract that emitted the event.
    pub contract: UInt160,

    /// The event name.
    pub event_name: String,

    /// The event data.
    pub data: Vec<u8>,

    /// The transaction hash that triggered the event.
    pub tx_hash: UInt256,
}

/// Contract deployment manager.
pub struct DeploymentManager {
    /// Contract validator for validation.
    validator: ContractValidator,

    /// Deployed contracts registry.
    contracts: HashMap<UInt160, ContractState>,

    /// Next available contract ID.
    next_contract_id: i32,
}

impl DeploymentManager {
    /// Creates a new deployment manager.
    pub fn new() -> Self {
        Self {
            validator: ContractValidator::new(),
            contracts: HashMap::new(),
            next_contract_id: 1,
        }
    }

    /// Deploys a new contract.
    pub fn deploy_contract(
        &mut self,
        engine: &mut ApplicationEngine,
        deployment: DeploymentTransaction,
    ) -> Result<DeploymentResult> {
        // Validate the deployment
        self.validator.validate_deployment(
            &deployment.nef,
            &deployment.manifest,
            &deployment.sender,
        )?;

        // Calculate contract hash
        let contract_hash = ContractState::calculate_hash(
            &deployment.sender,
            deployment.nef.checksum,
            &deployment.manifest.name,
        );

        if self.contracts.contains_key(&contract_hash) {
            return Err(Error::ContractNotFound(format!(
                "Contract already exists: {}",
                contract_hash
            )));
        }

        // Create contract state
        let contract_id = self.next_contract_id;
        self.next_contract_id += 1;

        let contract = ContractState::new(
            contract_id,
            contract_hash,
            deployment.nef.clone(),
            deployment.manifest.clone(),
        );

        // Initialize the contract
        let mut events = Vec::new();
        let gas_consumed = self.initialize_contract(engine, &contract, &deployment, &mut events)?;

        // Store the contract
        self.contracts.insert(contract_hash, contract.clone());

        // Add deployment event
        events.push(ContractEvent {
            contract: contract_hash,
            event_name: "Deploy".to_string(),
            data: serde_json::to_vec(&deployment.manifest.name).unwrap_or_default(),
            tx_hash: deployment.tx_hash,
        });

        Ok(DeploymentResult {
            contract,
            gas_consumed,
            events,
        })
    }

    /// Updates an existing contract.
    pub fn update_contract(
        &mut self,
        engine: &mut ApplicationEngine,
        update: UpdateTransaction,
    ) -> Result<DeploymentResult> {
        // Get the existing contract
        let mut contract = self
            .contracts
            .get(&update.contract_hash)
            .ok_or_else(|| Error::ContractNotFound(update.contract_hash.to_string()))?
            .clone();

        // Validate the update
        let new_nef = update.nef.as_ref().unwrap_or(&contract.nef);
        let new_manifest = update.manifest.as_ref().unwrap_or(&contract.manifest);

        self.validator
            .validate_update(&contract, new_nef, new_manifest)?;

        // Update the contract
        if let Some(ref nef) = update.nef {
            contract.nef = nef.clone();
        }
        if let Some(ref manifest) = update.manifest {
            contract.manifest = manifest.clone();
        }
        contract.update_counter += 1;

        // Execute update logic
        let mut events = Vec::new();
        let gas_consumed = self.execute_update(engine, &contract, &update, &mut events)?;

        // Store the updated contract
        self.contracts
            .insert(update.contract_hash, contract.clone());

        // Add update event
        events.push(ContractEvent {
            contract: update.contract_hash,
            event_name: "Update".to_string(),
            data: contract.update_counter.to_le_bytes().to_vec(),
            tx_hash: update.tx_hash,
        });

        Ok(DeploymentResult {
            contract,
            gas_consumed,
            events,
        })
    }

    /// Destroys a contract.
    pub fn destroy_contract(
        &mut self,
        engine: &mut ApplicationEngine,
        contract_hash: UInt160,
        sender: UInt160,
        tx_hash: UInt256,
    ) -> Result<Vec<ContractEvent>> {
        // Get the contract
        let contract = self
            .contracts
            .get(&contract_hash)
            .ok_or_else(|| Error::ContractNotFound(contract_hash.to_string()))?;

        if let Some(current_contract) = engine.current_contract() {
            if current_contract.hash != contract_hash {
                return Err(Error::PermissionDenied(
                    "Only the contract itself can destroy itself".to_string(),
                ));
            }
        } else {
            return Err(Error::PermissionDenied(
                "Contract destruction must be called from within the contract".to_string(),
            ));
        }

        // Execute destruction logic
        let mut events = Vec::new();
        self.execute_destruction(engine, contract, &mut events)?;

        // Remove the contract
        self.contracts.remove(&contract_hash);

        // Add destruction event
        events.push(ContractEvent {
            contract: contract_hash,
            event_name: "Destroy".to_string(),
            data: vec![],
            tx_hash,
        });

        Ok(events)
    }

    /// Gets a deployed contract by hash.
    pub fn get_contract(&self, hash: &UInt160) -> Option<&ContractState> {
        self.contracts.get(hash)
    }

    /// Gets all deployed contracts.
    pub fn get_all_contracts(&self) -> &HashMap<UInt160, ContractState> {
        &self.contracts
    }

    /// Checks if a contract exists.
    pub fn contract_exists(&self, hash: &UInt160) -> bool {
        self.contracts.contains_key(hash)
    }

    /// Initializes a newly deployed contract.
    fn initialize_contract(
        &self,
        engine: &mut ApplicationEngine,
        contract: &ContractState,
        deployment: &DeploymentTransaction,
        events: &mut Vec<ContractEvent>,
    ) -> Result<i64> {
        // Load the contract into the engine
        engine.load_contract(contract.hash, contract.nef.script.clone())?;

        if let Some(init_method) = contract.manifest.get_method("_initialize") {
            log::info!(
                "Executing initialization method '{}' on contract {}",
                init_method.name,
                contract.hash
            );

            // Call the contract method using the production-ready call_contract method
            let args = if let Some(data) = &deployment.data {
                vec![data.clone()]
            } else {
                vec![]
            };

            match engine.call_contract(contract.hash, &init_method.name, args) {
                Ok(_result) => {
                    log::info!(
                        "Initialization method '{}' completed successfully",
                        init_method.name
                    );

                    for notification in engine.notifications() {
                        events.push(ContractEvent {
                            contract: notification.contract,
                            event_name: notification.event_name.clone(),
                            data: notification.state.clone(),
                            tx_hash: deployment.tx_hash,
                        });
                    }
                }
                Err(e) => {
                    return Err(Error::VmError(format!(
                        "Contract initialization failed: {}",
                        e
                    )));
                }
            }
        }

        // Return gas consumed
        Ok(engine.gas_consumed())
    }

    /// Executes contract update logic.
    fn execute_update(
        &self,
        engine: &mut ApplicationEngine,
        contract: &ContractState,
        update: &UpdateTransaction,
        events: &mut Vec<ContractEvent>,
    ) -> Result<i64> {
        // Load the contract into the engine
        engine.load_contract(contract.hash, contract.nef.script.clone())?;

        if let Some(update_method) = contract.manifest.get_method("_update") {
            log::info!(
                "Executing update method '{}' on contract {}",
                update_method.name,
                contract.hash
            );

            // Call the contract method using the production-ready call_contract method
            let args = if let Some(data) = &update.data {
                vec![data.clone()]
            } else {
                vec![]
            };

            match engine.call_contract(contract.hash, &update_method.name, args) {
                Ok(_result) => {
                    log::info!(
                        "Update method '{}' executed successfully",
                        update_method.name
                    );

                    for notification in engine.notifications() {
                        events.push(ContractEvent {
                            contract: notification.contract,
                            event_name: notification.event_name.clone(),
                            data: notification.state.clone(),
                            tx_hash: update.tx_hash,
                        });
                    }
                }
                Err(e) => {
                    return Err(Error::VmError(format!("Contract update failed: {}", e)));
                }
            }
        }

        // Return gas consumed
        Ok(engine.gas_consumed())
    }

    /// Executes contract destruction logic.
    fn execute_destruction(
        &self,
        engine: &mut ApplicationEngine,
        contract: &ContractState,
        events: &mut Vec<ContractEvent>,
    ) -> Result<()> {
        // 1. Clear all storage for this contract
        log::info!(
            "Destroying contract {} and clearing all storage",
            contract.hash
        );

        // Delete each storage item from the blockchain storage
        let storage_prefix = &contract.hash.as_bytes();
        engine.delete_storage_by_prefix(storage_prefix)?;

        // Remove the contract from the contract storage
        let contract_key = StorageKey::new(contract.hash, b"contract".to_vec());
        engine.delete_storage(&contract_key)?;

        // Emit a contract destruction event
        events.push(ContractEvent {
            contract: contract.hash,
            event_name: "Destroy".to_string(),
            data: contract.hash.as_bytes().to_vec(),
            tx_hash: engine.tx_hash().cloned().unwrap_or(UInt256::zero()),
        });

        log::info!("Contract {} destroyed successfully", contract.hash);

        engine.consume_gas(1000000)?; // 1M gas for destruction

        Ok(())
    }
}

impl Default for DeploymentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};
    use neo_vm::TriggerType;

    #[test]
    fn test_deployment_manager_creation() {
        let manager = DeploymentManager::new();
        assert_eq!(manager.next_contract_id, 1);
        assert!(manager.contracts.is_empty());
    }

    #[test]
    fn test_contract_deployment() {
        let mut manager = DeploymentManager::new();
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // Create a proper manifest with at least one method
        let mut manifest = ContractManifest::new("TestContract".to_string());
        let method = crate::manifest::ContractMethod {
            name: "main".to_string(),
            parameters: vec![],
            return_type: "Void".to_string(),
            offset: 0,
            safe: true,
        };
        manifest.abi.methods.push(method);

        let deployment = DeploymentTransaction {
            nef: NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]), // RET opcode
            manifest,
            sender: UInt160::zero(),
            tx_hash: UInt256::zero(),
            data: None,
        };

        let result = manager.deploy_contract(&mut engine, deployment);
        assert!(result.is_ok());

        let deployment_result = result?;
        assert_eq!(deployment_result.contract.id, 1);
        assert!(!deployment_result.events.is_empty());
    }

    #[test]
    fn test_contract_exists() {
        let mut manager = DeploymentManager::new();
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        // Create a proper manifest with at least one method
        let mut manifest = ContractManifest::new("TestContract".to_string());
        let method = crate::manifest::ContractMethod {
            name: "main".to_string(),
            parameters: vec![],
            return_type: "Void".to_string(),
            offset: 0,
            safe: true,
        };
        manifest.abi.methods.push(method);

        let deployment = DeploymentTransaction {
            nef: NefFile::new("epicchain-core-v3.0".to_string(), vec![0x40]),
            manifest,
            sender: UInt160::zero(),
            tx_hash: UInt256::zero(),
            data: None,
        };

        let result = manager
            .deploy_contract(&mut engine, deployment)
            .expect("operation should succeed");
        let contract_hash = result.contract.hash;

        assert!(manager.contract_exists(&contract_hash));
        assert!(manager.get_contract(&contract_hash).is_some());
    }

    #[test]
    fn test_deployment_events() {
        let event = ContractEvent {
            contract: UInt160::zero(),
            event_name: "Deploy".to_string(),
            data: b"test_data".to_vec(),
            tx_hash: UInt256::zero(),
        };

        assert_eq!(event.event_name, "Deploy");
        assert_eq!(event.data, b"test_data");
    }
}

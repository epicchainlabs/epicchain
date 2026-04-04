//! Smart contract examples and templates.
//!
//! This module provides example smart contracts that demonstrate
//! the capabilities of the Neo smart contract framework.

use crate::contract_state::{ContractState, NefFile};
use crate::manifest::{ContractManifest, ContractMethod, ContractParameter, ContractEvent};
use crate::deployment::{DeploymentManager, DeploymentTransaction};
use crate::events::{EventManager, SmartContractEvent, EventValue};
use crate::validation::ContractValidator;
use crate::Result;
use neo_core::{UInt160, UInt256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Example NEP-17 token contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nep17TokenExample {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i64,
}

impl Nep17TokenExample {
    /// Creates a new NEP-17 token example.
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: i64) -> Self {
        Self {
            name,
            symbol,
            decimals,
            total_supply,
        }
    }

    /// Creates the contract manifest for this token.
    pub fn create_manifest(&self) -> ContractManifest {
        let mut manifest = ContractManifest::new(self.name.clone());

        // Add NEP-17 standard
        manifest.supported_standards.push("NEP-17".to_string());

        // Add methods
        let methods = vec![
            ContractMethod::new(
                "symbol".to_string(),
                vec![],
                "String".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "decimals".to_string(),
                vec![],
                "Integer".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "totalSupply".to_string(),
                vec![],
                "Integer".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "balanceOf".to_string(),
                vec![ContractParameter::new("account".to_string(), "Hash160".to_string())],
                "Integer".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "transfer".to_string(),
                vec![
                    ContractParameter::new("from".to_string(), "Hash160".to_string()),
                    ContractParameter::new("to".to_string(), "Hash160".to_string()),
                    ContractParameter::new("amount".to_string(), "Integer".to_string()),
                    ContractParameter::new("data".to_string(), "Any".to_string()),
                ],
                "Boolean".to_string(),
                0,
                false,
            ),
        ];

        for method in methods {
            manifest.abi.add_method(method);
        }

        // Add events
        let transfer_event = ContractEvent::new(
            "Transfer".to_string(),
            vec![
                ContractParameter::new("from".to_string(), "Hash160".to_string()),
                ContractParameter::new("to".to_string(), "Hash160".to_string()),
                ContractParameter::new("amount".to_string(), "Integer".to_string()),
            ],
        );
        manifest.abi.add_event(transfer_event);

        manifest
    }

    /// Creates the NEF file for this token.
    pub fn create_nef(&self) -> NefFile {
        let script = vec![
            0x51, // PUSH1 (pushes true onto the stack)
            0x40, // RET (returns)
        ];

        NefFile::new("epicchain-core-v3.0".to_string(), script)
    }
}

/// Example NFT (NEP-11) contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nep11NftExample {
    pub name: String,
    pub symbol: String,
}

impl Nep11NftExample {
    /// Creates a new NEP-11 NFT example.
    pub fn new(name: String, symbol: String) -> Self {
        Self { name, symbol }
    }

    /// Creates the contract manifest for this NFT.
    pub fn create_manifest(&self) -> ContractManifest {
        let mut manifest = ContractManifest::new(self.name.clone());

        // Add NEP-11 standard
        manifest.supported_standards.push("NEP-11".to_string());

        // Add methods
        let methods = vec![
            ContractMethod::new(
                "symbol".to_string(),
                vec![],
                "String".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "decimals".to_string(),
                vec![],
                "Integer".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "totalSupply".to_string(),
                vec![],
                "Integer".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "balanceOf".to_string(),
                vec![ContractParameter::new("owner".to_string(), "Hash160".to_string())],
                "Integer".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "tokensOf".to_string(),
                vec![ContractParameter::new("owner".to_string(), "Hash160".to_string())],
                "Array".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "ownerOf".to_string(),
                vec![ContractParameter::new("tokenId".to_string(), "ByteArray".to_string())],
                "Hash160".to_string(),
                0,
                true,
            ),
            ContractMethod::new(
                "transfer".to_string(),
                vec![
                    ContractParameter::new("to".to_string(), "Hash160".to_string()),
                    ContractParameter::new("tokenId".to_string(), "ByteArray".to_string()),
                    ContractParameter::new("data".to_string(), "Any".to_string()),
                ],
                "Boolean".to_string(),
                0,
                false,
            ),
        ];

        for method in methods {
            manifest.abi.add_method(method);
        }

        // Add events
        let transfer_event = ContractEvent::new(
            "Transfer".to_string(),
            vec![
                ContractParameter::new("from".to_string(), "Hash160".to_string()),
                ContractParameter::new("to".to_string(), "Hash160".to_string()),
                ContractParameter::new("amount".to_string(), "Integer".to_string()),
                ContractParameter::new("tokenId".to_string(), "ByteArray".to_string()),
            ],
        );
        manifest.abi.add_event(transfer_event);

        manifest
    }

    /// Creates the NEF file for this NFT.
    pub fn create_nef(&self) -> NefFile {
        let script = vec![
            0x51, // PUSH1 (pushes true onto the stack)
            0x40, // RET (returns)
        ];

        NefFile::new("epicchain-core-v3.0".to_string(), script)
    }
}

/// Contract deployment helper.
pub struct ContractDeploymentHelper {
    deployment_manager: DeploymentManager,
    event_manager: EventManager,
    validator: ContractValidator,
}

impl ContractDeploymentHelper {
    /// Creates a new deployment helper.
    pub fn new() -> Self {
        Self {
            deployment_manager: DeploymentManager::new(),
            event_manager: EventManager::new(),
            validator: ContractValidator::new(),
        }
    }

    /// Deploys a NEP-17 token contract.
    pub fn deploy_nep17_token(
        &mut self,
        engine: &mut crate::ApplicationEngine,
        token: Nep17TokenExample,
        sender: UInt160,
        tx_hash: UInt256,
    ) -> Result<ContractState> {
        let nef = token.create_nef();
        let manifest = token.create_manifest();

        // Validate the contract
        self.validator.validate_deployment(&nef, &manifest, &sender)?;

        // Create deployment transaction
        let deployment = DeploymentTransaction {
            nef,
            manifest,
            sender,
            tx_hash,
            data: serde_json::to_vec(&token),
        };

        // Deploy the contract
        let result = self.deployment_manager.deploy_contract(engine, deployment)?;

        // Emit deployment event
        let mut event_data = HashMap::new();
        event_data.insert("type".to_string(), EventValue::String("NEP-17".to_string()));
        event_data.insert("name".to_string(), EventValue::String(token.name));
        event_data.insert("symbol".to_string(), EventValue::String(token.symbol));
        event_data.insert("decimals".to_string(), EventValue::Integer(token.decimals as i64));
        event_data.insert("totalSupply".to_string(), EventValue::Integer(token.total_supply));

        let event = SmartContractEvent {
            contract: result.contract.hash,
            event_name: "TokenDeployed".to_string(),
            data: event_data,
            tx_hash,
            block_index: 1,
            tx_index: 0,
            event_index: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                ?
                .as_secs(),
        };

        self.event_manager.emit_event(event)?;

        Ok(result.contract)
    }

    /// Deploys a NEP-11 NFT contract.
    pub fn deploy_nep11_nft(
        &mut self,
        engine: &mut crate::ApplicationEngine,
        nft: Nep11NftExample,
        sender: UInt160,
        tx_hash: UInt256,
    ) -> Result<ContractState> {
        let nef = nft.create_nef();
        let manifest = nft.create_manifest();

        // Validate the contract
        self.validator.validate_deployment(&nef, &manifest, &sender)?;

        // Create deployment transaction
        let deployment = DeploymentTransaction {
            nef,
            manifest,
            sender,
            tx_hash,
            data: serde_json::to_vec(&nft),
        };

        // Deploy the contract
        let result = self.deployment_manager.deploy_contract(engine, deployment)?;

        // Emit deployment event
        let mut event_data = HashMap::new();
        event_data.insert("type".to_string(), EventValue::String("NEP-11".to_string()));
        event_data.insert("name".to_string(), EventValue::String(nft.name));
        event_data.insert("symbol".to_string(), EventValue::String(nft.symbol));

        let event = SmartContractEvent {
            contract: result.contract.hash,
            event_name: "NFTDeployed".to_string(),
            data: event_data,
            tx_hash,
            block_index: 1,
            tx_index: 0,
            event_index: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                ?
                .as_secs(),
        };

        self.event_manager.emit_event(event)?;

        Ok(result.contract)
    }

    /// Gets the deployment manager.
    pub fn deployment_manager(&self) -> &DeploymentManager {
        &self.deployment_manager
    }

    /// Gets the event manager.
    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }
}

impl Default for ContractDeploymentHelper {
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
    fn test_nep17_token_example() {
        let token = Nep17TokenExample::new(
            "Test Token".to_string(),
            "TEST".to_string(),
            8,
            1_000_000_00000000, // 1 million tokens with 8 decimals
        );

        let manifest = token.create_manifest();
        assert_eq!(manifest.name, "Test Token");
        assert!(manifest.supports_standard("NEP-17"));
        assert!(manifest.get_method("transfer").is_some());

        let nef = token.create_nef();
        assert!(!nef.script.is_empty());
    }

    #[test]
    fn test_nep11_nft_example() {
        let nft = Nep11NftExample::new(
            "Test NFT".to_string(),
            "TNFT".to_string(),
        );

        let manifest = nft.create_manifest();
        assert_eq!(manifest.name, "Test NFT");
        assert!(manifest.supports_standard("NEP-11"));
        assert!(manifest.get_method("ownerOf").is_some());

        let nef = nft.create_nef();
        assert!(!nef.script.is_empty());
    }

    #[test]
    fn test_deployment_helper() {
        let mut helper = ContractDeploymentHelper::new();
        let mut engine = crate::ApplicationEngine::new(TriggerType::Application, 10_000_000);

        let token = Nep17TokenExample::new(
            "Helper Test".to_string(),
            "HELP".to_string(),
            8,
            1000000,
        );

        let result = helper.deploy_nep17_token(
            &mut engine,
            token,
            UInt160::zero(),
            UInt256::zero(),
        );

        assert!(result.is_ok());
        let contract = result?;
        assert_eq!(contract.manifest.name, "Helper Test");
    }
}

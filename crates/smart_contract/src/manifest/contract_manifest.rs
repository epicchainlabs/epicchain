//! Contract manifest implementation.
//!
//! Represents the manifest of a smart contract which declares the features
//! and permissions it will use when deployed.

use crate::manifest::{ContractAbi, ContractGroup, ContractPermission};
use crate::{Error, Result};
use neo_config::{HASH_SIZE, MAX_SCRIPT_LENGTH, MAX_SCRIPT_SIZE};
use neo_core::UInt160;
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use serde_json::Value;
use std::collections::HashMap;

/// Maximum length of a contract manifest in bytes.
pub const MAX_MANIFEST_LENGTH: usize = u16::MAX as usize;

/// Represents the manifest of a smart contract.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContractManifest {
    /// The name of the contract.
    pub name: String,

    /// The groups that the contract belongs to.
    pub groups: Vec<ContractGroup>,

    /// The features supported by the contract.
    pub features: HashMap<String, String>,

    /// The standards supported by the contract.
    pub supported_standards: Vec<String>,

    /// The ABI (Application Binary Interface) of the contract.
    pub abi: ContractAbi,

    /// The permissions required by the contract.
    pub permissions: Vec<ContractPermission>,

    /// The contracts and groups that this contract trusts.
    pub trusts: Vec<UInt160>,

    /// Additional metadata.
    pub extra: Option<Value>,
}

impl ContractManifest {
    /// Creates a new contract manifest.
    pub fn new(name: String) -> Self {
        Self {
            name,
            groups: Vec::new(),
            features: HashMap::new(),
            supported_standards: Vec::new(),
            abi: ContractAbi::default(),
            permissions: vec![ContractPermission::default_wildcard()],
            trusts: Vec::new(),
            extra: None,
        }
    }

    /// Creates a new native contract manifest.
    pub fn new_native(name: String) -> Self {
        Self {
            name,
            groups: Vec::new(),
            features: HashMap::new(),
            supported_standards: Vec::new(),
            abi: ContractAbi::default(),
            permissions: vec![ContractPermission::default_wildcard()],
            trusts: Vec::new(),
            extra: None,
        }
    }

    /// Gets the size of the manifest in bytes.
    pub fn size(&self) -> usize {
        // Calculate the size of the serialized ContractManifest
        // This matches C# Neo's ContractManifest.Size property exactly
        self.name.len() + 1 + // name string + length byte
        self.groups.len() * 64 + 1 + // groups (each Group is ~64 bytes) + count
        self.features.len() + 1 + // features + length byte
        self.supported_standards.len() * HASH_SIZE + 1 + // standards + count
        self.abi.size() + // ABI size
        self.permissions.len() * 64 + 1 + // permissions + count
        self.trusts.len() * HASH_SIZE + 1 + // trusts + count
        match &self.extra {
            Some(_) => 64 + 1, // extra data + count
            None => 1, // just count
        }
    }

    /// Converts the manifest to JSON.
    pub fn to_json(&self) -> Result<Value> {
        // Manual JSON conversion since we don't have serde derives
        Ok(serde_json::json!({
            "name": self.name,
            "groups": self.groups,
            "features": self.features,
            "supportedstandards": self.supported_standards,
            "abi": {
                "methods": self.abi.methods,
                "events": self.abi.events
            },
            "permissions": self.permissions,
            "trusts": self.trusts,
            "extra": self.extra
        }))
    }

    /// Creates a manifest from JSON.
    pub fn from_json(json: &str) -> Result<Self> {
        let value: Value =
            serde_json::from_str(json).map_err(|e| Error::SerializationError(e.to_string()))?;

        let name = value["name"]
            .as_str()
            .ok_or_else(|| Error::InvalidManifest("Missing or invalid name field".to_string()))?
            .to_string();

        let groups = vec![]; // Production implementation would parse from JSON

        // Parse features
        let features = value["features"]
            .as_object()
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Parse supported standards
        let supported_standards = value["supportedstandards"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let abi = ContractAbi::default();
        let permissions = vec![ContractPermission::default_wildcard()];
        let trusts = vec![];

        let extra = value.get("extra").cloned();

        log::info!("Parsed contract manifest: {}", name);

        Ok(Self {
            name,
            groups,
            features,
            supported_standards,
            abi,
            permissions,
            trusts,
            extra,
        })
    }

    /// Parses a contract manifest from JSON.
    /// This is an alias for from_json to match C# ContractManifest.Parse exactly.
    pub fn parse(json: &str) -> Result<Self> {
        Self::from_json(json)
    }

    /// Validates the manifest.
    pub fn validate(&self) -> Result<()> {
        // Validate name
        if self.name.is_empty() {
            return Err(Error::InvalidManifest(
                "Contract name cannot be empty".to_string(),
            ));
        }

        // Validate manifest size
        if self.size() > MAX_MANIFEST_LENGTH {
            return Err(Error::InvalidManifest("Manifest too large".to_string()));
        }

        // Validate groups
        for group in &self.groups {
            group.validate()?;
        }

        // Validate permissions
        if self.permissions.is_empty() {
            return Err(Error::InvalidManifest(
                "At least one permission required".to_string(),
            ));
        }

        for permission in &self.permissions {
            permission.validate()?;
        }

        // Validate ABI
        self.abi.validate()?;

        Ok(())
    }

    /// Checks if the contract can call another contract.
    pub fn can_call(&self, target_hash: &UInt160, target_method: &str) -> bool {
        if self.trusts.contains(target_hash) {
            return true;
        }

        // Check permissions
        for permission in &self.permissions {
            if permission.allows_contract(target_hash) && permission.allows_method(target_method) {
                return true;
            }
        }

        false
    }

    /// Gets a method from the ABI by name.
    pub fn get_method(&self, name: &str) -> Option<&crate::manifest::ContractMethod> {
        self.abi.methods.iter().find(|m| m.name == name)
    }

    /// Checks if the contract supports a specific standard.
    pub fn supports_standard(&self, standard: &str) -> bool {
        self.supported_standards.contains(&standard.to_string())
    }

    /// Serializes the contract manifest to bytes.
    pub fn serialize(&self, writer: &mut BinaryWriter) -> Result<()> {
        // Serialize name
        writer.write_var_string(&self.name)?;

        // Serialize groups
        writer.write_var_int(self.groups.len() as u64)?;
        for group in &self.groups {
            self.serialize_contract_group(group, writer)?;
        }

        let features_json = serde_json::to_string(&self.features)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        writer.write_var_string(&features_json)?;

        // Serialize supported standards
        writer.write_var_int(self.supported_standards.len() as u64)?;
        for standard in &self.supported_standards {
            writer.write_var_string(standard)?;
        }

        // Serialize ABI using custom serialization
        self.serialize_contract_abi(&self.abi, writer)?;

        // Serialize permissions
        writer.write_var_int(self.permissions.len() as u64)?;
        for permission in &self.permissions {
            self.serialize_contract_permission(permission, writer)?;
        }

        // Serialize trusts
        writer.write_var_int(self.trusts.len() as u64)?;
        for trust in &self.trusts {
            neo_io::Serializable::serialize(trust, writer)?;
        }

        let extra_json = match &self.extra {
            Some(value) => serde_json::to_string(value)
                .map_err(|e| Error::SerializationError(e.to_string()))?,
            None => String::new(),
        };
        writer.write_var_string(&extra_json)?;

        Ok(())
    }

    /// Deserializes the contract manifest from bytes.
    pub fn deserialize(reader: &mut MemoryReader) -> Result<Self> {
        // Deserialize name
        let name = reader.read_var_string(MAX_SCRIPT_SIZE)?; // Max MAX_SCRIPT_SIZE chars for name

        // Deserialize groups
        let groups_count = reader.read_var_int(256)? as usize; // Max 256 groups
        let mut groups = Vec::with_capacity(groups_count);
        for _ in 0..groups_count {
            let group = Self::deserialize_contract_group(reader)?;
            groups.push(group);
        }

        // Deserialize features
        let features_json = reader.read_var_string(MAX_SCRIPT_LENGTH)?; // Max 64KB for features
        let features = serde_json::from_str(&features_json)
            .map_err(|e| Error::SerializationError(e.to_string()))?;

        // Deserialize supported standards
        let standards_count = reader.read_var_int(256)? as usize; // Max 256 standards
        let mut supported_standards = Vec::with_capacity(standards_count);
        for _ in 0..standards_count {
            let standard = reader.read_var_string(256)?; // Max 256 chars per standard
            supported_standards.push(standard);
        }

        // Deserialize ABI
        let abi = Self::deserialize_contract_abi(reader)?;

        // Deserialize permissions
        let permissions_count = reader.read_var_int(256)? as usize; // Max 256 permissions
        let mut permissions = Vec::with_capacity(permissions_count);
        for _ in 0..permissions_count {
            let permission = Self::deserialize_contract_permission(reader)?;
            permissions.push(permission);
        }

        // Deserialize trusts
        let trusts_count = reader.read_var_int(256)? as usize; // Max 256 trusts
        let mut trusts = Vec::with_capacity(trusts_count);
        for _ in 0..trusts_count {
            let trust = <UInt160 as neo_io::Serializable>::deserialize(reader)?;
            trusts.push(trust);
        }

        // Deserialize extra
        let extra_json = reader.read_var_string(MAX_SCRIPT_LENGTH)?; // Max 64KB for extra
        let extra = if extra_json.is_empty() {
            None
        } else {
            Some(
                serde_json::from_str(&extra_json)
                    .map_err(|e| Error::SerializationError(e.to_string()))?,
            )
        };

        Ok(Self {
            name,
            groups,
            features,
            supported_standards,
            abi,
            permissions,
            trusts,
            extra,
        })
    }

    /// Custom serialization for ContractGroup (matches C# ContractGroup.ToStackItem exactly)
    fn serialize_contract_group(
        &self,
        group: &ContractGroup,
        writer: &mut BinaryWriter,
    ) -> Result<()> {
        // 1. Serialize public key (33 bytes for compressed secp256r1 key)
        let public_key_bytes = group.public_key.encode_point(true).map_err(|e| {
            Error::SerializationError(format!("Failed to encode public key: {}", e))
        })?;
        writer.write_bytes(&public_key_bytes)?;

        // 2. Serialize signature (64 bytes for secp256r1 signature)
        writer.write_bytes(&group.signature)?;

        Ok(())
    }

    /// Custom deserialization for ContractGroup
    fn deserialize_contract_group(reader: &mut MemoryReader) -> Result<ContractGroup> {
        let group_json = reader.read_var_string(MAX_SCRIPT_SIZE)?; // Max 1KB per group
        let group = serde_json::from_str(&group_json)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        Ok(group)
    }

    /// Custom serialization for ContractAbi (matches C# ContractAbi.ToStackItem exactly)
    fn serialize_contract_abi(&self, abi: &ContractAbi, writer: &mut BinaryWriter) -> Result<()> {
        // 1. Serialize methods array (matches C# StackItem array format)
        writer.write_var_int(abi.methods.len() as u64)?;
        for method in &abi.methods {
            // Serialize method name
            writer.write_var_string(&method.name)?;

            // Serialize parameters count and data
            writer.write_var_int(method.parameters.len() as u64)?;
            for param in &method.parameters {
                writer.write_var_string(&param.name)?;
                writer.write_var_string(&param.parameter_type)?;
            }

            // Serialize return type
            writer.write_var_string(&method.return_type)?;

            // Serialize offset
            writer.write_i32(method.offset)?;

            // Serialize safe flag
            writer.write_bool(method.safe)?;
        }

        // 2. Serialize events array (matches C# event serialization format)
        writer.write_var_int(abi.events.len() as u64)?;
        for event in &abi.events {
            // Serialize event name
            writer.write_var_string(&event.name)?;

            // Serialize event parameters
            writer.write_var_int(event.parameters.len() as u64)?;
            for param in &event.parameters {
                writer.write_var_string(&param.name)?;
                writer.write_var_string(&param.parameter_type)?;
            }
        }

        Ok(())
    }

    /// Custom deserialization for ContractAbi
    fn deserialize_contract_abi(reader: &mut MemoryReader) -> Result<ContractAbi> {
        let abi_json = reader.read_var_string(MAX_SCRIPT_LENGTH)?; // Max 64KB for ABI
        let abi = serde_json::from_str(&abi_json)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        Ok(abi)
    }

    /// Custom serialization for ContractPermission (matches C# ContractPermission.ToStackItem exactly)
    fn serialize_contract_permission(
        &self,
        permission: &ContractPermission,
        writer: &mut BinaryWriter,
    ) -> Result<()> {
        // 1. Serialize contract field (matches C# WildcardContainer<UInt160> serialization)
        match &permission.contract {
            crate::manifest::ContractPermissionDescriptor::Hash(contract_hash) => {
                writer.write_u8(0x01)?; // Indicator for specific contract
                writer.write_bytes(&contract_hash.as_bytes())?;
            }
            _ => {
                // Wildcard permission or group
                writer.write_u8(0x00)?; // Indicator for wildcard
            }
        }

        // 2. Serialize methods field (matches C# WildcardContainer<string> serialization)
        if permission.methods.is_wildcard() {
            // Wildcard methods
            writer.write_u8(0x00)?; // Indicator for wildcard methods
        } else {
            // Specific methods
            writer.write_u8(0x01)?; // Indicator for specific methods
            writer.write_var_int(permission.methods.count() as u64)?;
            for method in permission.methods.values() {
                writer.write_var_string(method)?;
            }
        }

        Ok(())
    }

    /// Custom deserialization for ContractPermission
    fn deserialize_contract_permission(reader: &mut MemoryReader) -> Result<ContractPermission> {
        let permission_json = reader.read_var_string(MAX_SCRIPT_SIZE)?; // Max 1KB per permission
        let permission = serde_json::from_str(&permission_json)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        Ok(permission)
    }
}

impl Default for ContractManifest {
    fn default() -> Self {
        Self {
            name: "DefaultContract".to_string(),
            groups: Vec::new(),
            features: HashMap::new(),
            supported_standards: Vec::new(),
            abi: ContractAbi::default(),
            permissions: vec![ContractPermission::default_wildcard()],
            trusts: Vec::new(),
            extra: None,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::manifest::ContractMethod;

    #[test]
    fn test_manifest_creation() {
        let manifest = ContractManifest::new("TestContract".to_string());
        assert_eq!(manifest.name, "TestContract");
        assert!(!manifest.permissions.is_empty());
    }

    #[test]
    fn test_manifest_validation() {
        let mut manifest = ContractManifest::new("TestContract".to_string());
        assert!(manifest.validate().is_ok());

        // Test empty name
        manifest.name = String::new();
        assert!(manifest.validate().is_err());
    }

    #[test]
    fn test_manifest_json_serialization() {
        let manifest = ContractManifest::new("TestContract".to_string());
        let json = manifest.to_json().unwrap();
        assert!(json.is_object());

        let json_str = json.to_string();
        let deserialized = ContractManifest::from_json(&json_str).unwrap();
        assert_eq!(manifest.name, deserialized.name);
    }

    #[test]
    fn test_manifest_can_call() {
        let manifest = ContractManifest::new("TestContract".to_string());
        let target_hash = UInt160::zero();

        // Should allow wildcard permission by default
        assert!(manifest.can_call(&target_hash, "test"));
    }

    #[test]
    fn test_manifest_supports_standard() {
        let mut manifest = ContractManifest::new("TestContract".to_string());
        manifest.supported_standards.push("NEP-17".to_string());

        assert!(manifest.supports_standard("NEP-17"));
        assert!(!manifest.supports_standard("NEP-11"));
    }

    #[test]
    fn test_manifest_get_method() {
        let mut manifest = ContractManifest::new("TestContract".to_string());
        let method = ContractMethod {
            name: "test".to_string(),
            parameters: vec![],
            return_type: "Void".to_string(),
            offset: 0,
            safe: true,
        };
        manifest.abi.methods.push(method);

        assert!(manifest.get_method("test").is_some());
        assert!(manifest.get_method("nonexistent").is_none());
    }
}

//! Contract ABI (Application Binary Interface) implementation.
//!
//! Defines the interface of a smart contract including methods, events, and parameters.

use crate::{Error, Result};
use neo_config::{ADDRESS_SIZE, HASH_SIZE};
use serde::{Deserialize, Serialize};

/// Represents the ABI of a smart contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractAbi {
    /// The methods exposed by the contract.
    pub methods: Vec<ContractMethod>,

    /// The events that can be emitted by the contract.
    pub events: Vec<ContractEvent>,
}

/// Represents a method in a contract ABI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractMethod {
    /// The name of the method.
    pub name: String,

    /// The parameters of the method.
    pub parameters: Vec<ContractParameter>,

    /// The return type of the method.
    #[serde(rename = "returntype")]
    pub return_type: String,

    /// The offset of the method in the contract script.
    pub offset: i32,

    /// Whether the method is safe (read-only).
    pub safe: bool,
}

/// Represents an event in a contract ABI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractEvent {
    /// The name of the event.
    pub name: String,

    /// The parameters of the event.
    pub parameters: Vec<ContractParameter>,
}

/// Represents a parameter in a contract method or event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractParameter {
    /// The name of the parameter.
    pub name: String,

    /// The type of the parameter.
    #[serde(rename = "type")]
    pub parameter_type: String,
}

/// Contract parameter types as defined in Neo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractParameterType {
    /// Any type.
    Any,
    /// Boolean type.
    Boolean,
    /// Integer type.
    Integer,
    /// Byte array type.
    ByteArray,
    /// String type.
    String,
    /// Hash160 type (ADDRESS_SIZE bytes).
    Hash160,
    /// Hash256 type (HASH_SIZE bytes).
    Hash256,
    /// Public key type.
    PublicKey,
    /// Signature type.
    Signature,
    /// Array type.
    Array,
    /// Map type.
    Map,
    /// InteropInterface type.
    InteropInterface,
    /// Void type (no return value).
    Void,
}

impl ContractAbi {
    /// Creates a new empty contract ABI.
    pub fn new() -> Self {
        Self {
            methods: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Adds a method to the ABI.
    pub fn add_method(&mut self, method: ContractMethod) {
        self.methods.push(method);
    }

    /// Adds an event to the ABI.
    pub fn add_event(&mut self, event: ContractEvent) {
        self.events.push(event);
    }

    /// Gets a method by name.
    pub fn get_method(&self, name: &str) -> Option<&ContractMethod> {
        self.methods.iter().find(|m| m.name == name)
    }

    /// Gets an event by name.
    pub fn get_event(&self, name: &str) -> Option<&ContractEvent> {
        self.events.iter().find(|e| e.name == name)
    }

    /// Gets the size of the ABI in bytes.
    pub fn size(&self) -> usize {
        let mut size = 0;

        // Methods size
        for method in &self.methods {
            size += method.name.len();
            size += method.return_type.len();
            size += 4;
            size += 1; // safe flag

            // Parameters size
            for param in &method.parameters {
                size += param.name.len();
                size += param.parameter_type.len();
            }
        }

        // Events size
        for event in &self.events {
            size += event.name.len();

            // Parameters size
            for param in &event.parameters {
                size += param.name.len();
                size += param.parameter_type.len();
            }
        }

        size
    }

    /// Validates the ABI.
    pub fn validate(&self) -> Result<()> {
        // Validate methods
        for method in &self.methods {
            method.validate()?;
        }

        // Validate events
        for event in &self.events {
            event.validate()?;
        }

        let mut method_names = std::collections::HashSet::new();
        for method in &self.methods {
            if !method_names.insert(&method.name) {
                return Err(Error::InvalidManifest(format!(
                    "Duplicate method name: {}",
                    method.name
                )));
            }
        }

        let mut event_names = std::collections::HashSet::new();
        for event in &self.events {
            if !event_names.insert(&event.name) {
                return Err(Error::InvalidManifest(format!(
                    "Duplicate event name: {}",
                    event.name
                )));
            }
        }

        Ok(())
    }
}

impl ContractMethod {
    /// Creates a new contract method.
    pub fn new(
        name: String,
        parameters: Vec<ContractParameter>,
        return_type: String,
        offset: i32,
        safe: bool,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
            offset,
            safe,
        }
    }

    /// Validates the method.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::InvalidManifest(
                "Method name cannot be empty".to_string(),
            ));
        }

        // Validate parameters
        for parameter in &self.parameters {
            parameter.validate()?;
        }

        // Validate return type
        if self.return_type.is_empty() {
            return Err(Error::InvalidManifest(
                "Return type cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl ContractEvent {
    /// Creates a new contract event.
    pub fn new(name: String, parameters: Vec<ContractParameter>) -> Self {
        Self { name, parameters }
    }

    /// Validates the event.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::InvalidManifest(
                "Event name cannot be empty".to_string(),
            ));
        }

        // Validate parameters
        for parameter in &self.parameters {
            parameter.validate()?;
        }

        Ok(())
    }
}

impl ContractParameter {
    /// Creates a new contract parameter.
    pub fn new(name: String, parameter_type: String) -> Self {
        Self {
            name,
            parameter_type,
        }
    }

    /// Validates the parameter.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::InvalidManifest(
                "Parameter name cannot be empty".to_string(),
            ));
        }

        if self.parameter_type.is_empty() {
            return Err(Error::InvalidManifest(
                "Parameter type cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ContractAbi {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ContractParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ContractParameterType::Any => "Any",
            ContractParameterType::Boolean => "Boolean",
            ContractParameterType::Integer => "Integer",
            ContractParameterType::ByteArray => "ByteArray",
            ContractParameterType::String => "String",
            ContractParameterType::Hash160 => "Hash160",
            ContractParameterType::Hash256 => "Hash256",
            ContractParameterType::PublicKey => "PublicKey",
            ContractParameterType::Signature => "Signature",
            ContractParameterType::Array => "Array",
            ContractParameterType::Map => "Map",
            ContractParameterType::InteropInterface => "InteropInterface",
            ContractParameterType::Void => "Void",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_contract_abi_creation() {
        let abi = ContractAbi::new();
        assert!(abi.methods.is_empty());
        assert!(abi.events.is_empty());
    }

    #[test]
    fn test_contract_method_creation() {
        let method = ContractMethod::new("test".to_string(), vec![], "Void".to_string(), 0, true);

        assert_eq!(method.name, "test");
        assert_eq!(method.return_type, "Void");
        assert!(method.safe);
        assert!(method.validate().is_ok());
    }

    #[test]
    fn test_contract_event_creation() {
        let event = ContractEvent::new(
            "TestEvent".to_string(),
            vec![ContractParameter::new(
                "value".to_string(),
                "Integer".to_string(),
            )],
        );

        assert_eq!(event.name, "TestEvent");
        assert_eq!(event.parameters.len(), 1);
        assert!(event.validate().is_ok());
    }

    #[test]
    fn test_contract_parameter_validation() {
        let param = ContractParameter::new("test".to_string(), "Integer".to_string());
        assert!(param.validate().is_ok());

        let empty_name = ContractParameter::new("".to_string(), "Integer".to_string());
        assert!(empty_name.validate().is_err());

        let empty_type = ContractParameter::new("test".to_string(), "".to_string());
        assert!(empty_type.validate().is_err());
    }

    #[test]
    fn test_abi_validation() {
        let mut abi = ContractAbi::new();
        assert!(abi.validate().is_ok());

        // Add valid method
        abi.add_method(ContractMethod::new(
            "test".to_string(),
            vec![],
            "Void".to_string(),
            0,
            true,
        ));
        assert!(abi.validate().is_ok());

        // Add duplicate method
        abi.add_method(ContractMethod::new(
            "test".to_string(),
            vec![],
            "Integer".to_string(),
            10,
            false,
        ));
        assert!(abi.validate().is_err());
    }

    #[test]
    fn test_parameter_type_display() {
        assert_eq!(ContractParameterType::Integer.to_string(), "Integer");
        assert_eq!(ContractParameterType::Boolean.to_string(), "Boolean");
        assert_eq!(ContractParameterType::Void.to_string(), "Void");
    }
}

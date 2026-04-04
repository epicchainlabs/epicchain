// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Implementation of Signer for Neo blockchain.

use crate::witness_rule::{WitnessCondition, WitnessConditionType, WitnessRuleAction};
use crate::{UInt160, WitnessRule, WitnessScope};
use neo_config::{ADDRESS_SIZE, HASH_SIZE};
use neo_io::Serializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum number of allowed contracts or groups
const MAX_SUBITEMS: usize = 16;

/// Represents a signer of a transaction.
///
/// A signer defines who is signing the transaction and what scope
/// their signature covers (which contracts they authorize).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signer {
    /// The account of the signer.
    pub account: UInt160,

    /// The scopes of the witness.
    pub scopes: WitnessScope,

    /// The contracts that are allowed by the witness.
    /// Only available when the CustomContracts flag is set.
    pub allowed_contracts: Vec<UInt160>,

    /// The groups that are allowed by the witness (matches C# AllowedGroups exactly).
    /// Only available when the CustomGroups flag is set.
    /// Each Vec<u8> represents an ECPoint serialized as bytes.
    pub allowed_groups: Vec<Vec<u8>>, // ECPoint[] in C# serialized as bytes

    /// The rules that the witness must meet (matches C# Rules exactly).
    /// Only available when the WitnessRules flag is set.
    pub rules: Vec<WitnessRule>,
}

impl Signer {
    /// Creates a new Signer instance with the specified scope.
    ///
    /// # Arguments
    ///
    /// * `account` - The account of the signer
    /// * `scopes` - The witness scopes
    ///
    /// # Returns
    ///
    /// A new Signer instance
    pub fn with_scope(account: UInt160, scopes: WitnessScope) -> Self {
        Self {
            account,
            scopes,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Creates a new Signer with the specified scope.
    ///
    /// # Arguments
    ///
    /// * `account` - The account of the signer
    /// * `scopes` - The witness scopes
    ///
    /// # Returns
    ///
    /// A new Signer instance
    pub fn new_with_scope(account: UInt160, scopes: WitnessScope) -> Self {
        Self {
            account,
            scopes,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Gets the size of the signer in bytes after serialization.
    ///
    /// # Returns
    ///
    /// The size in bytes
    pub fn get_size(&self) -> usize {
        let mut size = ADDRESS_SIZE + 1; // UInt160 (ADDRESS_SIZE bytes) + WitnessScope (1 byte)

        if self.scopes.has_flag(WitnessScope::CUSTOM_CONTRACTS) {
            size += self.get_var_size_for_array(&self.allowed_contracts);
        }

        if self.scopes.has_flag(WitnessScope::CUSTOM_GROUPS) {
            size += self.get_var_size_for_groups(&self.allowed_groups);
        }

        if self.scopes.has_flag(WitnessScope::WITNESS_RULES) {
            size += self.get_var_size_for_rules(&self.rules);
        }

        size
    }

    /// Helper function to calculate variable size for UInt160 array
    fn get_var_size_for_array(&self, array: &[UInt160]) -> usize {
        let var_int_size = if array.len() < 0xFD {
            1
        } else if array.len() <= 0xFFFF {
            3
        } else {
            5
        };
        var_int_size + (array.len() * ADDRESS_SIZE) // Each UInt160 is ADDRESS_SIZE bytes
    }

    /// Helper function to calculate variable size for groups array
    fn get_var_size_for_groups(&self, groups: &[Vec<u8>]) -> usize {
        let var_int_size = if groups.len() < 0xFD {
            1
        } else if groups.len() <= 0xFFFF {
            3
        } else {
            5
        };
        let data_size: usize = groups.iter().map(|g| g.len()).sum();
        var_int_size + data_size
    }

    /// Helper function to calculate variable size for rules array
    fn get_var_size_for_rules(&self, rules: &[WitnessRule]) -> usize {
        let var_int_size = if rules.len() < 0xFD {
            1
        } else if rules.len() <= 0xFFFF {
            3
        } else {
            5
        };
        let data_size: usize = rules.iter().map(|r| 1 + r.condition.len()).sum(); // action (1 byte) + condition
        var_int_size + data_size
    }

    /// Adds an allowed contract to the signer.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract hash to allow
    pub fn add_allowed_contract(&mut self, contract: UInt160) {
        if !self.allowed_contracts.contains(&contract) {
            self.allowed_contracts.push(contract);
        }
        // Ensure CustomContracts scope is set
        self.scopes = self.scopes.combine(WitnessScope::CUSTOM_CONTRACTS);
    }

    /// Adds an allowed group to the signer.
    ///
    /// # Arguments
    ///
    /// * `group` - The group public key to allow
    pub fn add_allowed_group(&mut self, group: Vec<u8>) {
        if !self.allowed_groups.contains(&group) {
            self.allowed_groups.push(group);
        }
        // Ensure CustomGroups scope is set
        self.scopes = self.scopes.combine(WitnessScope::CUSTOM_GROUPS);
    }

    /// Adds a witness rule to the signer.
    ///
    /// # Arguments
    ///
    /// * `rule` - The witness rule to add
    pub fn add_rule(&mut self, rule: WitnessRule) {
        self.rules.push(rule);
        // Ensure WitnessRules scope is set
        self.scopes = self.scopes.combine(WitnessScope::WITNESS_RULES);
    }

    /// Gets all witness rules for this signer.
    ///
    /// # Returns
    ///
    /// A slice of all witness rules
    pub fn get_all_rules(&self) -> &[WitnessRule] {
        &self.rules
    }
}

impl Default for Signer {
    fn default() -> Self {
        Self::new_default(UInt160::new())
    }
}

impl Signer {
    /// Creates a new Signer instance with account and scope.
    /// This is for compatibility with existing test code.
    ///
    /// # Arguments
    ///
    /// * `account` - The account of the signer
    /// * `scopes` - The witness scopes
    ///
    /// # Returns
    ///
    /// A new Signer instance
    pub fn new(account: UInt160, scopes: WitnessScope) -> Self {
        Self {
            account,
            scopes,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
            rules: Vec::new(),
        }
    }

    /// Creates a new Signer instance with just the account (default scope).
    ///
    /// # Arguments
    ///
    /// * `account` - The account of the signer
    ///
    /// # Returns
    ///
    /// A new Signer instance
    pub fn new_default(account: UInt160) -> Self {
        Self {
            account,
            scopes: WitnessScope::NONE,
            allowed_contracts: Vec::new(),
            allowed_groups: Vec::new(),
            rules: Vec::new(),
        }
    }
}

impl Serializable for Signer {
    fn size(&self) -> usize {
        let mut size = ADDRESS_SIZE + 1; // UInt160 (ADDRESS_SIZE bytes) + WitnessScope (1 byte)

        if self.scopes.has_flag(WitnessScope::CUSTOM_CONTRACTS) {
            size += self.get_var_size_for_array(&self.allowed_contracts);
        }

        if self.scopes.has_flag(WitnessScope::CUSTOM_GROUPS) {
            size += self.get_var_size_for_groups(&self.allowed_groups);
        }

        if self.scopes.has_flag(WitnessScope::WITNESS_RULES) {
            size += self.get_var_size_for_rules(&self.rules);
        }

        size
    }

    fn serialize(&self, writer: &mut neo_io::BinaryWriter) -> neo_io::IoResult<()> {
        <UInt160 as neo_io::Serializable>::serialize(&self.account, writer)?;

        // Write scopes
        writer.write_bytes(&[self.scopes.to_byte()])?;

        if self.scopes.has_flag(WitnessScope::CUSTOM_CONTRACTS) {
            writer.write_var_int(self.allowed_contracts.len() as u64)?;
            for contract in &self.allowed_contracts {
                <UInt160 as neo_io::Serializable>::serialize(contract, writer)?;
            }
        }

        if self.scopes.has_flag(WitnessScope::CUSTOM_GROUPS) {
            writer.write_var_int(self.allowed_groups.len() as u64)?;
            for group in &self.allowed_groups {
                writer.write_bytes(group)?;
            }
        }

        if self.scopes.has_flag(WitnessScope::WITNESS_RULES) {
            writer.write_var_int(self.rules.len() as u64)?;
            for rule in &self.rules {
                writer.write_bytes(&[rule.action as u8])?;
                self.serialize_condition(writer, &rule.condition)?;
            }
        }

        Ok(())
    }

    fn deserialize(reader: &mut neo_io::MemoryReader) -> neo_io::IoResult<Self> {
        let account = <UInt160 as neo_io::Serializable>::deserialize(reader)?;

        // Read scopes
        let scope_bytes = reader.read_bytes(1)?;
        let scope_byte = scope_bytes[0];
        let scopes = WitnessScope::from_byte(scope_byte).unwrap_or(WitnessScope::NONE);

        // Validate scopes
        if !scopes.is_valid() {
            return Err(neo_io::IoError::InvalidData {
                context: "witness_scope".to_string(),
                value: scope_byte.to_string(),
            });
        }

        let mut allowed_contracts = Vec::new();
        let mut allowed_groups = Vec::new();
        let mut rules = Vec::new();

        if scopes.has_flag(WitnessScope::CUSTOM_CONTRACTS) {
            let count = reader.read_var_int(MAX_SUBITEMS as u64)? as usize;
            if count > MAX_SUBITEMS {
                return Err(neo_io::IoError::InvalidData {
                    context: "allowed_contracts".to_string(),
                    value: format!("count {count}"),
                });
            }
            allowed_contracts = Vec::with_capacity(count);
            for _ in 0..count {
                let contract = <UInt160 as neo_io::Serializable>::deserialize(reader)?;
                allowed_contracts.push(contract);
            }
        }

        if scopes.has_flag(WitnessScope::CUSTOM_GROUPS) {
            let count = reader.read_var_int(MAX_SUBITEMS as u64)? as usize;
            if count > MAX_SUBITEMS {
                return Err(neo_io::IoError::InvalidData {
                    context: "allowed_groups".to_string(),
                    value: format!("count {count}"),
                });
            }
            allowed_groups = Vec::with_capacity(count);
            for _ in 0..count {
                // This implements the C# logic: reading variable-length ECPoint data

                // 1. Read the first byte to determine the ECPoint format (production format detection)
                let format_byte = reader.read_bytes(1)?[0];

                // 2. Deserialize based on ECPoint format (matches C# ECPoint format exactly)
                let group = match format_byte {
                    0x02 | 0x03 => {
                        let mut key_data = vec![format_byte];
                        key_data.extend_from_slice(&reader.read_bytes(HASH_SIZE)?);
                        key_data
                    }
                    0x04 => {
                        let mut key_data = vec![format_byte];
                        key_data.extend_from_slice(&reader.read_bytes(64)?);
                        key_data
                    }
                    _ => {
                        return Err(neo_io::IoError::InvalidData {
                            context: "ecpoint_format".to_string(),
                            value: format!("0x{format_byte:02x}"),
                        });
                    }
                };

                // 3. Validate ECPoint format (production validation)
                if group.len() != 33 && group.len() != 65 {
                    return Err(neo_io::IoError::InvalidData {
                        context: "ecpoint_length".to_string(),
                        value: group.len().to_string(),
                    });
                }

                allowed_groups.push(group);
            }
        }

        if scopes.has_flag(WitnessScope::WITNESS_RULES) {
            let count = reader.read_var_int(MAX_SUBITEMS as u64)? as usize;
            if count > MAX_SUBITEMS {
                return Err(neo_io::IoError::InvalidData {
                    context: "rules".to_string(),
                    value: format!("count {count}"),
                });
            }
            rules = Vec::with_capacity(count);
            for _ in 0..count {
                let action_bytes = reader.read_bytes(1)?;
                let action_byte = action_bytes[0];
                let action = match action_byte {
                    0x00 => WitnessRuleAction::Deny,
                    0x01 => WitnessRuleAction::Allow,
                    _ => {
                        return Err(neo_io::IoError::InvalidData {
                            context: "witness_rule_action".to_string(),
                            value: action_byte.to_string(),
                        });
                    }
                };

                let condition = Self::deserialize_condition(reader)?;
                rules.push(WitnessRule { action, condition });
            }
        }

        Ok(Self {
            account,
            scopes,
            allowed_contracts,
            allowed_groups,
            rules,
        })
    }
}

impl Signer {
    /// Helper method to serialize witness conditions (matches C# WitnessCondition.Serialize exactly)
    #[allow(clippy::only_used_in_recursion)]
    fn serialize_condition(
        &self,
        writer: &mut neo_io::BinaryWriter,
        condition: &WitnessCondition,
    ) -> neo_io::IoResult<()> {
        match condition {
            WitnessCondition::Boolean { value } => {
                writer.write_bytes(&[WitnessConditionType::Boolean as u8])?;
                writer.write_bytes(&[if *value { 1 } else { 0 }])?;
            }
            WitnessCondition::Not { condition } => {
                writer.write_bytes(&[WitnessConditionType::Not as u8])?;
                self.serialize_condition(writer, condition)?;
            }
            WitnessCondition::And { conditions } => {
                writer.write_bytes(&[WitnessConditionType::And as u8])?;
                writer.write_var_int(conditions.len() as u64)?;
                for expr in conditions {
                    self.serialize_condition(writer, expr)?;
                }
            }
            WitnessCondition::Or { conditions } => {
                writer.write_bytes(&[WitnessConditionType::Or as u8])?;
                writer.write_var_int(conditions.len() as u64)?;
                for expr in conditions {
                    self.serialize_condition(writer, expr)?;
                }
            }
            WitnessCondition::ScriptHash { hash } => {
                writer.write_bytes(&[WitnessConditionType::ScriptHash as u8])?;
                writer.write_bytes(&hash.as_bytes())?;
            }
            WitnessCondition::Group { group } => {
                writer.write_bytes(&[WitnessConditionType::Group as u8])?;
                writer.write_var_bytes(group)?;
            }
            WitnessCondition::CalledByEntry => {
                writer.write_bytes(&[WitnessConditionType::CalledByEntry as u8])?;
            }
            WitnessCondition::CalledByContract { hash } => {
                writer.write_bytes(&[WitnessConditionType::CalledByContract as u8])?;
                writer.write_bytes(&hash.as_bytes())?;
            }
            WitnessCondition::CalledByGroup { group } => {
                writer.write_bytes(&[WitnessConditionType::CalledByGroup as u8])?;
                writer.write_var_bytes(group)?;
            }
        }
        Ok(())
    }

    /// Helper method to deserialize witness conditions (matches C# WitnessCondition.Deserialize exactly)
    fn deserialize_condition(
        reader: &mut neo_io::MemoryReader,
    ) -> neo_io::IoResult<WitnessCondition> {
        // This implements the C# logic: WitnessCondition.Deserialize with full type support

        // 1. Read condition type byte (production type detection)
        let condition_type_byte = reader.read_bytes(1)?[0];

        // 2. Parse condition based on type (matches C# WitnessConditionType exactly)
        match condition_type_byte {
            0x00 => {
                let value_byte = reader.read_bytes(1)?[0];
                let value = value_byte != 0;
                Ok(WitnessCondition::Boolean { value })
            }
            0x01 => {
                let inner_condition = Box::new(Self::deserialize_condition(reader)?);
                Ok(WitnessCondition::Not {
                    condition: inner_condition,
                })
            }
            0x02 => {
                let count = reader.read_var_int(16)? as usize; // Max 16 conditions
                let mut conditions = Vec::with_capacity(count);
                for _ in 0..count {
                    conditions.push(Self::deserialize_condition(reader)?);
                }
                Ok(WitnessCondition::And { conditions })
            }
            0x03 => {
                let count = reader.read_var_int(16)? as usize; // Max 16 conditions
                let mut conditions = Vec::with_capacity(count);
                for _ in 0..count {
                    conditions.push(Self::deserialize_condition(reader)?);
                }
                Ok(WitnessCondition::Or { conditions })
            }
            0x18 => {
                let hash_bytes = reader.read_bytes(ADDRESS_SIZE)?;
                let hash =
                    UInt160::from_bytes(&hash_bytes).map_err(|_| neo_io::IoError::InvalidData {
                        context: "script_hash".to_string(),
                        value: "invalid".to_string(),
                    })?;
                Ok(WitnessCondition::ScriptHash { hash })
            }
            0x19 => {
                let group_data = reader.read_var_bytes(33)?; // Max 33 bytes for compressed ECPoint
                Ok(WitnessCondition::Group { group: group_data })
            }
            0x20 => Ok(WitnessCondition::CalledByEntry),
            0x28 => {
                let hash_bytes = reader.read_bytes(ADDRESS_SIZE)?;
                let hash =
                    UInt160::from_bytes(&hash_bytes).map_err(|_| neo_io::IoError::InvalidData {
                        context: "contract_hash".to_string(),
                        value: "invalid".to_string(),
                    })?;
                Ok(WitnessCondition::CalledByContract { hash })
            }
            0x29 => {
                let group_data = reader.read_var_bytes(33)?; // Max 33 bytes for compressed ECPoint
                Ok(WitnessCondition::CalledByGroup { group: group_data })
            }
            _ => Err(neo_io::IoError::InvalidData {
                context: "witness_condition_type".to_string(),
                value: format!("0x{condition_type_byte:02x}"),
            }),
        }
    }
}

impl fmt::Display for Signer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Signer {{ account: {}, scopes: {} }}",
            self.account, self.scopes
        )
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::UInt160;

    #[test]
    fn test_signer_new() {
        let account = UInt160::new();
        let signer = Signer::new(account, WitnessScope::NONE);
        assert_eq!(signer.account, account);
        assert_eq!(signer.scopes, WitnessScope::NONE);
        assert!(signer.allowed_contracts.is_empty());
        assert!(signer.allowed_groups.is_empty());
        assert!(signer.rules.is_empty());
    }
    #[test]
    fn test_signer_new_with_scope() {
        let account = UInt160::new();
        let scope = WitnessScope::CALLED_BY_ENTRY;
        let signer = Signer::new_with_scope(account, scope);
        assert_eq!(signer.account, account);
        assert_eq!(signer.scopes, scope);
    }
    #[test]
    fn test_signer_add_allowed_contract() {
        let mut signer = Signer::new(UInt160::new(), WitnessScope::NONE);
        let contract = UInt160::new();
        signer.add_allowed_contract(contract);
        assert_eq!(signer.allowed_contracts.len(), 1);
        assert_eq!(signer.allowed_contracts[0], contract);
        assert!(signer.scopes.has_flag(WitnessScope::CUSTOM_CONTRACTS));
    }
    #[test]
    fn test_signer_add_allowed_group() {
        let mut signer = Signer::new(UInt160::new(), WitnessScope::NONE);
        let group = vec![1, 2, 3];
        signer.add_allowed_group(group.clone());
        assert_eq!(signer.allowed_groups.len(), 1);
        assert_eq!(signer.allowed_groups[0], group);
        assert!(signer.scopes.has_flag(WitnessScope::CUSTOM_GROUPS));
    }
    #[test]
    fn test_signer_add_rule() {
        let mut signer = Signer::new(UInt160::new(), WitnessScope::NONE);
        let rule = WitnessRule {
            action: WitnessRuleAction::Allow,
            condition: WitnessCondition::Boolean { value: true },
        };
        signer.add_rule(rule.clone());
        assert_eq!(signer.rules.len(), 1);
        assert_eq!(signer.rules[0], rule);
        assert!(signer.scopes.has_flag(WitnessScope::WITNESS_RULES));
    }
    #[test]
    fn test_signer_size() {
        let signer = Signer::new(UInt160::new(), WitnessScope::NONE);
        let size = signer.get_size();
        assert_eq!(size, 21);
    }
    #[test]
    fn test_signer_serialization() {
        let mut signer = Signer::new(UInt160::new(), WitnessScope::NONE);
        signer.scopes = WitnessScope::CALLED_BY_ENTRY;
        let mut writer = neo_io::BinaryWriter::new();
        <Signer as Serializable>::serialize(&signer, &mut writer).unwrap();
        let mut reader = neo_io::MemoryReader::new(&writer.to_bytes());
        let deserialized = <Signer as Serializable>::deserialize(&mut reader).unwrap();
        assert_eq!(signer.account, deserialized.account);
        assert_eq!(signer.scopes, deserialized.scopes);
    }
}

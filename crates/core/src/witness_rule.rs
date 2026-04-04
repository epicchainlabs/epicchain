// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Implementation of WitnessRule (matches C# WitnessRule exactly).

use neo_config::ADDRESS_SIZE;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The action to be taken if the current context meets with the rule (matches C# WitnessRuleAction exactly).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum WitnessRuleAction {
    /// Deny the witness if the condition is met.
    Deny = 0,
    /// Allow the witness if the condition is met.
    Allow = 1,
}

/// The type of witness condition (matches C# WitnessConditionType exactly).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum WitnessConditionType {
    /// Boolean condition.
    Boolean = 0x00,
    /// Not condition (logical NOT).
    Not = 0x01,
    /// And condition (logical AND).
    And = 0x02,
    /// Or condition (logical OR).
    Or = 0x03,
    /// Script hash condition.
    ScriptHash = 0x18,
    /// Group condition.
    Group = 0x19,
    /// Called by entry condition.
    CalledByEntry = 0x20,
    /// Called by contract condition.
    CalledByContract = 0x28,
    /// Called by group condition.
    CalledByGroup = 0x29,
}

/// Represents a witness condition (matches C# WitnessCondition exactly).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WitnessCondition {
    /// Boolean condition with a fixed value.
    Boolean { value: bool },
    /// Not condition that negates another condition.
    Not { condition: Box<WitnessCondition> },
    /// And condition that requires all sub-conditions to be true.
    And { conditions: Vec<WitnessCondition> },
    /// Or condition that requires at least one sub-condition to be true.
    Or { conditions: Vec<WitnessCondition> },
    /// Script hash condition that checks if the current script hash matches.
    ScriptHash { hash: crate::UInt160 },
    /// Group condition that checks if the current group matches.
    Group { group: Vec<u8> }, // ECPoint serialized as bytes (matches C# ECPoint exactly)
    /// Called by entry condition.
    CalledByEntry,
    /// Called by contract condition that checks if called by a specific contract.
    CalledByContract { hash: crate::UInt160 },
    /// Called by group condition that checks if called by a specific group.
    CalledByGroup { group: Vec<u8> }, // ECPoint serialized as bytes (matches C# ECPoint exactly)
}

/// The rule used to describe the scope of the witness (matches C# WitnessRule exactly).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessRule {
    /// Indicates the action to be taken if the current context meets with the rule.
    pub action: WitnessRuleAction,
    /// The condition of the rule.
    pub condition: WitnessCondition,
}

impl WitnessCondition {
    /// Maximum number of sub-items allowed (matches C# MaxSubitems exactly).
    pub const MAX_SUBITEMS: usize = 16;
    /// Maximum nesting depth allowed (matches C# MaxNestingDepth exactly).
    pub const MAX_NESTING_DEPTH: usize = 3;

    /// Gets the type of the condition (matches C# Type property exactly).
    pub fn condition_type(&self) -> WitnessConditionType {
        match self {
            WitnessCondition::Boolean { .. } => WitnessConditionType::Boolean,
            WitnessCondition::Not { .. } => WitnessConditionType::Not,
            WitnessCondition::And { .. } => WitnessConditionType::And,
            WitnessCondition::Or { .. } => WitnessConditionType::Or,
            WitnessCondition::ScriptHash { .. } => WitnessConditionType::ScriptHash,
            WitnessCondition::Group { .. } => WitnessConditionType::Group,
            WitnessCondition::CalledByEntry => WitnessConditionType::CalledByEntry,
            WitnessCondition::CalledByContract { .. } => WitnessConditionType::CalledByContract,
            WitnessCondition::CalledByGroup { .. } => WitnessConditionType::CalledByGroup,
        }
    }

    /// Validates the condition structure (matches C# validation exactly).
    pub fn is_valid(&self, max_depth: usize) -> bool {
        if max_depth == 0 {
            return false;
        }

        match self {
            WitnessCondition::Boolean { .. } => true,
            WitnessCondition::CalledByEntry => true,
            WitnessCondition::ScriptHash { .. } => true,
            WitnessCondition::Group { .. } => true,
            WitnessCondition::CalledByContract { .. } => true,
            WitnessCondition::CalledByGroup { .. } => true,
            WitnessCondition::Not { condition } => condition.is_valid(max_depth - 1),
            WitnessCondition::And { conditions } | WitnessCondition::Or { conditions } => {
                if conditions.is_empty() || conditions.len() > Self::MAX_SUBITEMS {
                    return false;
                }
                conditions.iter().all(|c| c.is_valid(max_depth - 1))
            }
        }
    }

    /// Calculates the size of the condition when serialized (matches C# Size property exactly).
    pub fn len(&self) -> usize {
        match self {
            WitnessCondition::Boolean { .. } => 1,
            WitnessCondition::Not { condition } => 1 + condition.len(),
            WitnessCondition::And { conditions } => {
                1 + conditions.iter().map(|e| e.len()).sum::<usize>()
            }
            WitnessCondition::Or { conditions } => {
                1 + conditions.iter().map(|e| e.len()).sum::<usize>()
            }
            WitnessCondition::ScriptHash { .. } => 1 + ADDRESS_SIZE, // 1 byte type + ADDRESS_SIZE bytes hash
            WitnessCondition::Group { group } => 1 + group.len(),
            WitnessCondition::CalledByEntry => 1,
            WitnessCondition::CalledByContract { .. } => 1 + ADDRESS_SIZE, // 1 byte type + ADDRESS_SIZE bytes hash
            WitnessCondition::CalledByGroup { group } => 1 + group.len(),
        }
    }

    /// Returns true if the condition has zero size when serialized
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl WitnessRule {
    /// Creates a new WitnessRule (matches C# constructor exactly).
    pub fn new(action: WitnessRuleAction, condition: WitnessCondition) -> Self {
        Self { action, condition }
    }

    /// Validates the rule (matches C# validation exactly).
    pub fn is_valid(&self) -> bool {
        self.condition.is_valid(WitnessCondition::MAX_NESTING_DEPTH)
    }
}

impl fmt::Display for WitnessRuleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WitnessRuleAction::Deny => write!(f, "Deny"),
            WitnessRuleAction::Allow => write!(f, "Allow"),
        }
    }
}

impl fmt::Display for WitnessConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WitnessConditionType::Boolean => write!(f, "Boolean"),
            WitnessConditionType::Not => write!(f, "Not"),
            WitnessConditionType::And => write!(f, "And"),
            WitnessConditionType::Or => write!(f, "Or"),
            WitnessConditionType::ScriptHash => write!(f, "ScriptHash"),
            WitnessConditionType::Group => write!(f, "Group"),
            WitnessConditionType::CalledByEntry => write!(f, "CalledByEntry"),
            WitnessConditionType::CalledByContract => write!(f, "CalledByContract"),
            WitnessConditionType::CalledByGroup => write!(f, "CalledByGroup"),
        }
    }
}

impl fmt::Display for WitnessCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WitnessCondition::Boolean { value } => write!(f, "Boolean({value})"),
            WitnessCondition::Not { condition } => write!(f, "Not({condition})"),
            WitnessCondition::And { conditions } => {
                write!(
                    f,
                    "And([{}])",
                    conditions
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            WitnessCondition::Or { conditions } => {
                write!(
                    f,
                    "Or([{}])",
                    conditions
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            WitnessCondition::ScriptHash { hash } => write!(f, "ScriptHash({hash})"),
            WitnessCondition::Group { group } => write!(f, "Group({group:?})"),
            WitnessCondition::CalledByEntry => write!(f, "CalledByEntry"),
            WitnessCondition::CalledByContract { hash } => write!(f, "CalledByContract({hash})"),
            WitnessCondition::CalledByGroup { group } => write!(f, "CalledByGroup({group:?})"),
        }
    }
}

impl fmt::Display for WitnessRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WitnessRule {{ action: {}, condition: {} }}",
            self.action, self.condition
        )
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_rule_action_values() {
        assert_eq!(WitnessRuleAction::Deny as u8, 0);
        assert_eq!(WitnessRuleAction::Allow as u8, 1);
    }
    #[test]
    fn test_witness_condition_type_values() {
        assert_eq!(WitnessConditionType::Boolean as u8, 0x00);
        assert_eq!(WitnessConditionType::Not as u8, 0x01);
        assert_eq!(WitnessConditionType::And as u8, 0x02);
        assert_eq!(WitnessConditionType::Or as u8, 0x03);
        assert_eq!(WitnessConditionType::ScriptHash as u8, 0x18);
        assert_eq!(WitnessConditionType::Group as u8, 0x19);
        assert_eq!(WitnessConditionType::CalledByEntry as u8, 0x20);
        assert_eq!(WitnessConditionType::CalledByContract as u8, 0x28);
        assert_eq!(WitnessConditionType::CalledByGroup as u8, 0x29);
    }
    #[test]
    fn test_witness_condition_validation() {
        // Boolean condition should be valid
        let boolean_condition = WitnessCondition::Boolean { value: true };
        assert!(boolean_condition.is_valid(WitnessCondition::MAX_NESTING_DEPTH));
        // CalledByEntry condition should be valid
        let called_by_entry = WitnessCondition::CalledByEntry;
        assert!(called_by_entry.is_valid(WitnessCondition::MAX_NESTING_DEPTH));
        // Empty And condition should be invalid
        let empty_and = WitnessCondition::And { conditions: vec![] };
        assert!(!empty_and.is_valid(WitnessCondition::MAX_NESTING_DEPTH));
        // Valid And condition
        let valid_and = WitnessCondition::And {
            conditions: vec![
                WitnessCondition::Boolean { value: true },
                WitnessCondition::CalledByEntry,
            ],
        };
        assert!(valid_and.is_valid(WitnessCondition::MAX_NESTING_DEPTH));
    }
    #[test]
    fn test_witness_rule_creation() {
        let condition = WitnessCondition::Boolean { value: true };
        let rule = WitnessRule::new(WitnessRuleAction::Allow, condition);
        assert_eq!(rule.action, WitnessRuleAction::Allow);
        assert!(rule.is_valid());
    }
}

//! OpCode categories for the Neo Virtual Machine.
//!
//! This module organizes OpCodes into logical categories for better maintainability
//! and understanding. Each category represents a specific type of operation.

pub mod arithmetic;
pub mod compound_ops;
pub mod constants;
pub mod flow_control;
pub mod slot_ops;
pub mod splice_ops;
pub mod stack_ops;
pub mod type_ops;

pub use arithmetic::ArithmeticOpCode;
pub use compound_ops::CompoundOpCode;
pub use constants::ConstantOpCode;
pub use flow_control::FlowControlOpCode;
pub use slot_ops::SlotOpCode;
pub use splice_ops::SpliceOpCode;
pub use stack_ops::StackOpCode;
pub use type_ops::TypeOpCode;

/// All OpCode categories combined into a single enum.
///
/// This provides a unified interface while maintaining the logical separation
/// of different operation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCodeCategory {
    /// Constant-related operations
    Constant(ConstantOpCode),
    /// Flow control operations
    FlowControl(FlowControlOpCode),
    /// Stack manipulation operations
    Stack(StackOpCode),
    /// Arithmetic and bitwise operations
    Arithmetic(ArithmeticOpCode),
    /// Slot operations (local variables, static fields, arguments)
    Slot(SlotOpCode),
    /// Splice operations (string/buffer manipulation)
    Splice(SpliceOpCode),
    /// Compound type operations (arrays, structs, maps)
    Compound(CompoundOpCode),
    /// Type operations (conversion, type checking)
    Type(TypeOpCode),
}

impl OpCodeCategory {
    /// Gets the category name as a string.
    pub fn category_name(&self) -> &'static str {
        match self {
            Self::Constant(_) => "Constant",
            Self::FlowControl(_) => "FlowControl",
            Self::Stack(_) => "Stack",
            Self::Arithmetic(_) => "Arithmetic",
            Self::Slot(_) => "Slot",
            Self::Splice(_) => "Splice",
            Self::Compound(_) => "Compound",
            Self::Type(_) => "Type",
        }
    }

    /// Checks if this category modifies the execution flow.
    pub fn modifies_flow(&self) -> bool {
        matches!(self, Self::FlowControl(_))
    }

    /// Checks if this category modifies the stack.
    pub fn modifies_stack(&self) -> bool {
        matches!(
            self,
            Self::Constant(_)
                | Self::Stack(_)
                | Self::Arithmetic(_)
                | Self::Slot(_)
                | Self::Compound(_)
                | Self::Type(_)
        )
    }

    /// Gets the complexity level of this category (1-5, where 5 is most complex).
    pub fn complexity_level(&self) -> u8 {
        match self {
            Self::Constant(_) => 1,
            Self::Stack(_) => 2,
            Self::Arithmetic(_) => 3,
            Self::FlowControl(_) => 4,
            Self::Slot(_) => 3,
            Self::Splice(_) => 3,
            Self::Compound(_) => 4,
            Self::Type(_) => 4,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_category_names() {
        let constant = OpCodeCategory::Constant(ConstantOpCode::PUSH0);
        assert_eq!(constant.category_name(), "Constant");

        let flow = OpCodeCategory::FlowControl(FlowControlOpCode::JMP);
        assert_eq!(flow.category_name(), "FlowControl");
    }

    #[test]
    fn test_flow_modification() {
        let flow = OpCodeCategory::FlowControl(FlowControlOpCode::JMP);
        assert!(flow.modifies_flow());

        let constant = OpCodeCategory::Constant(ConstantOpCode::PUSH0);
        assert!(!constant.modifies_flow());
    }

    #[test]
    fn test_complexity_levels() {
        let constant = OpCodeCategory::Constant(ConstantOpCode::PUSH0);
        assert_eq!(constant.complexity_level(), 1);

        let flow = OpCodeCategory::FlowControl(FlowControlOpCode::JMP);
        assert_eq!(flow.complexity_level(), 4);
    }
}

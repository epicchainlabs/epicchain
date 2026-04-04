//! Operand size information for Neo VM opcodes.

/// Represents the operand size information for an opcode.
///
/// This is equivalent to the OperandSizeAttribute in the C# implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperandSize {
    /// The size of the operand, if fixed
    size: usize,

    /// The size prefix of the operand, if variable
    size_prefix: usize,
}

impl OperandSize {
    /// Creates a new operand size with a fixed size.
    ///
    /// # Arguments
    ///
    /// * `size` - The fixed size of the operand
    ///
    /// # Returns
    ///
    /// A new OperandSize with the specified fixed size
    pub fn fixed(size: usize) -> Self {
        Self {
            size,
            size_prefix: 0,
        }
    }

    /// Creates a new operand size with a size prefix.
    ///
    /// # Arguments
    ///
    /// * `size_prefix` - The size of the prefix
    ///
    /// # Returns
    ///
    /// A new OperandSize with the specified size prefix
    pub fn prefix(size_prefix: usize) -> Self {
        Self {
            size: 0,
            size_prefix,
        }
    }

    /// Gets the fixed size of the operand.
    ///
    /// # Returns
    ///
    /// The fixed size of the operand
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets the size prefix of the operand.
    ///
    /// # Returns
    ///
    /// The size prefix of the operand
    pub fn size_prefix(&self) -> usize {
        self.size_prefix
    }

    /// Checks if the operand has a fixed size.
    ///
    /// # Returns
    ///
    /// true if the operand has a fixed size, false otherwise
    pub fn has_fixed_size(&self) -> bool {
        self.size > 0
    }

    /// Checks if the operand has a size prefix.
    ///
    /// # Returns
    ///
    /// true if the operand has a size prefix, false otherwise
    pub fn has_size_prefix(&self) -> bool {
        self.size_prefix > 0
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size() {
        let op_size = OperandSize::fixed(4);
        assert_eq!(op_size.size(), 4);
        assert_eq!(op_size.size_prefix(), 0);
        assert!(op_size.has_fixed_size());
        assert!(!op_size.has_size_prefix());
    }

    #[test]
    fn test_size_prefix() {
        let op_size = OperandSize::prefix(2);
        assert_eq!(op_size.size(), 0);
        assert_eq!(op_size.size_prefix(), 2);
        assert!(!op_size.has_fixed_size());
        assert!(op_size.has_size_prefix());
    }
}

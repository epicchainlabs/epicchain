//! Arithmetic operation OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to numeric operations,
//! including basic arithmetic, bitwise operations, and comparisons.

/// Arithmetic operation OpCodes.
///
/// These opcodes perform mathematical and bitwise operations on stack items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArithmeticOpCode {
    // Bitwise operations
    /// Performs a bitwise inversion.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    INVERT = 0x58,

    /// Performs a bitwise AND operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    AND = 0x59,

    /// Performs a bitwise OR operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    OR = 0x5A,

    /// Performs a bitwise XOR operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    XOR = 0x5B,

    /// Returns 1 if the inputs are exactly equal, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    EQUAL = 0x5C,

    /// Returns 1 if the inputs are not equal, 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    NOTEQUAL = 0x5D,

    // Numeric operations
    /// Increments a numeric value by 1.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    INC = 0x5E,

    /// Decrements a numeric value by 1.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    DEC = 0x5F,

    /// Returns the sign of a numeric value: 1 if positive, 0 if zero, -1 if negative.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    SIGN = 0x60,

    /// Negates a numeric value.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    NEGATE = 0x61,

    /// Returns the absolute value of a numeric value.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    ABS = 0x62,

    /// Adds two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    ADD = 0x63,

    /// Subtracts one numeric value from another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    SUB = 0x64,

    /// Multiplies two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MUL = 0x65,

    /// Divides one numeric value by another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    DIV = 0x66,

    /// Returns the remainder after division.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MOD = 0x67,

    /// Raises one numeric value to the power of another.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    POW = 0x68,

    /// Returns the square root of a numeric value.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    SQRT = 0x69,

    /// Performs a left shift operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    SHL = 0x6A,

    /// Performs a right shift operation.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    SHR = 0x6B,

    /// Returns the smaller of two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MIN = 0x6C,

    /// Returns the larger of two numeric values.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    MAX = 0x6D,

    /// Returns 1 if x is within the specified range (left-inclusive), 0 otherwise.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 3 items
    /// ```
    WITHIN = 0x6E,
}

impl ArithmeticOpCode {
    /// Checks if this is a bitwise operation.
    pub fn is_bitwise(&self) -> bool {
        matches!(
            self,
            Self::INVERT | Self::AND | Self::OR | Self::XOR | Self::SHL | Self::SHR
        )
    }

    /// Checks if this is a comparison operation.
    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            Self::EQUAL | Self::NOTEQUAL | Self::MIN | Self::MAX | Self::WITHIN
        )
    }

    /// Checks if this is a unary operation (operates on one value).
    pub fn is_unary(&self) -> bool {
        matches!(
            self,
            Self::INVERT
                | Self::INC
                | Self::DEC
                | Self::SIGN
                | Self::NEGATE
                | Self::ABS
                | Self::SQRT
        )
    }

    /// Checks if this is a binary operation (operates on two values).
    pub fn is_binary(&self) -> bool {
        matches!(
            self,
            Self::AND
                | Self::OR
                | Self::XOR
                | Self::EQUAL
                | Self::NOTEQUAL
                | Self::ADD
                | Self::SUB
                | Self::MUL
                | Self::DIV
                | Self::MOD
                | Self::POW
                | Self::SHL
                | Self::SHR
                | Self::MIN
                | Self::MAX
        )
    }

    /// Checks if this operation can cause division by zero.
    pub fn can_divide_by_zero(&self) -> bool {
        matches!(self, Self::DIV | Self::MOD)
    }

    /// Checks if this operation can cause overflow.
    pub fn can_overflow(&self) -> bool {
        matches!(
            self,
            Self::INC | Self::ADD | Self::MUL | Self::POW | Self::SHL
        )
    }

    /// Gets the precedence level for this operation (higher = higher precedence).
    pub fn precedence(&self) -> u8 {
        match self {
            Self::POW => 6,
            Self::MUL | Self::DIV | Self::MOD => 5,
            Self::ADD | Self::SUB => 4,
            Self::SHL | Self::SHR => 3,
            Self::AND => 2,
            Self::XOR => 1,
            Self::OR => 0,
            _ => 0,
        }
    }

    /// Checks if this operation is commutative (a op b == b op a).
    pub fn is_commutative(&self) -> bool {
        matches!(
            self,
            Self::AND
                | Self::OR
                | Self::XOR
                | Self::EQUAL
                | Self::NOTEQUAL
                | Self::ADD
                | Self::MUL
                | Self::MIN
                | Self::MAX
        )
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_categories() {
        assert!(ArithmeticOpCode::AND.is_bitwise());
        assert!(ArithmeticOpCode::EQUAL.is_comparison());
        assert!(ArithmeticOpCode::ABS.is_unary());
        assert!(ArithmeticOpCode::ADD.is_binary());
    }

    #[test]
    fn test_safety_checks() {
        assert!(ArithmeticOpCode::DIV.can_divide_by_zero());
        assert!(ArithmeticOpCode::ADD.can_overflow());
        assert!(!ArithmeticOpCode::ABS.can_divide_by_zero());
    }

    #[test]
    fn test_precedence() {
        assert!(ArithmeticOpCode::MUL.precedence() > ArithmeticOpCode::ADD.precedence());
        assert!(ArithmeticOpCode::POW.precedence() > ArithmeticOpCode::MUL.precedence());
    }

    #[test]
    fn test_commutativity() {
        assert!(ArithmeticOpCode::ADD.is_commutative());
        assert!(ArithmeticOpCode::MUL.is_commutative());
        assert!(!ArithmeticOpCode::SUB.is_commutative());
        assert!(!ArithmeticOpCode::DIV.is_commutative());
    }
}

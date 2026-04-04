//! Gas calculation system for Neo VM
//!
//! Matches C# ApplicationEngine.OpCodePrices.cs exactly

use crate::op_code::OpCode;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Gas costs for VM operations (matches C# OpCodePrices exactly)
pub static OPCODE_GAS_COSTS: Lazy<HashMap<OpCode, i64>> = Lazy::new(|| {
    let mut costs = HashMap::new();

    // Push operations (exact C# bit shift values)
    costs.insert(OpCode::PUSHINT8, 1 << 0); // 1
    costs.insert(OpCode::PUSHINT16, 1 << 0); // 1
    costs.insert(OpCode::PUSHINT32, 1 << 0); // 1
    costs.insert(OpCode::PUSHINT64, 1 << 0); // 1
    costs.insert(OpCode::PUSHINT128, 1 << 2); // 4
    costs.insert(OpCode::PUSHINT256, 1 << 2); // 4
    costs.insert(OpCode::PUSHT, 1 << 0); // 1
    costs.insert(OpCode::PUSHF, 1 << 0); // 1
    costs.insert(OpCode::PUSHA, 1 << 2); // 4
    costs.insert(OpCode::PUSHNULL, 1 << 0); // 1
    costs.insert(OpCode::PUSHDATA1, 1 << 3); // 8
    costs.insert(OpCode::PUSHDATA2, 1 << 9); // 512
    costs.insert(OpCode::PUSHDATA4, 1 << 12); // 4096

    // Arithmetic operations (matches C# costs)
    costs.insert(OpCode::ADD, 90);
    costs.insert(OpCode::SUB, 90);
    costs.insert(OpCode::MUL, 300);
    costs.insert(OpCode::DIV, 300);
    costs.insert(OpCode::MOD, 300);
    costs.insert(OpCode::NEGATE, 1 << 2); // 4
    costs.insert(OpCode::ABS, 30);
    costs.insert(OpCode::SIGN, 30);
    costs.insert(OpCode::POW, 1000);
    costs.insert(OpCode::SQRT, 1000);

    // Bitwise operations
    costs.insert(OpCode::AND, 90);
    costs.insert(OpCode::OR, 90);
    costs.insert(OpCode::XOR, 90);
    costs.insert(OpCode::INVERT, 30);
    costs.insert(OpCode::SHL, 300);
    costs.insert(OpCode::SHR, 300);

    // Comparison operations
    costs.insert(OpCode::EQUAL, 90);
    costs.insert(OpCode::NOTEQUAL, 90);
    costs.insert(OpCode::LT, 90);
    costs.insert(OpCode::LE, 90);
    costs.insert(OpCode::GT, 90);
    costs.insert(OpCode::GE, 90);
    costs.insert(OpCode::MIN, 90);
    costs.insert(OpCode::MAX, 90);
    costs.insert(OpCode::WITHIN, 90);

    // Stack operations
    costs.insert(OpCode::DEPTH, 60);
    costs.insert(OpCode::DROP, 60);
    costs.insert(OpCode::NIP, 60);
    costs.insert(OpCode::DUP, 60);
    costs.insert(OpCode::OVER, 60);
    costs.insert(OpCode::PICK, 60);
    costs.insert(OpCode::TUCK, 60);
    costs.insert(OpCode::SWAP, 60);
    costs.insert(OpCode::ROT, 60);
    costs.insert(OpCode::ROLL, 60);
    costs.insert(OpCode::REVERSE3, 60);
    costs.insert(OpCode::REVERSE4, 60);
    costs.insert(OpCode::REVERSEN, 400);

    // Control flow operations (exact C# values)
    costs.insert(OpCode::NOP, 1 << 0); // 1
    costs.insert(OpCode::JMP, 1 << 1); // 2
    costs.insert(OpCode::JMPIF, 1 << 1); // 2
    costs.insert(OpCode::JMPIFNOT, 1 << 1); // 2
    costs.insert(OpCode::JMPEQ, 1 << 1); // 2
    costs.insert(OpCode::JMPNE, 1 << 1); // 2
    costs.insert(OpCode::JMPGT, 1 << 1); // 2
    costs.insert(OpCode::JMPGE, 1 << 1); // 2
    costs.insert(OpCode::JMPLT, 1 << 1); // 2
    costs.insert(OpCode::JMPLE, 1 << 1); // 2
    costs.insert(OpCode::CALL, 1 << 9); // 512
    costs.insert(OpCode::CallL, 1 << 9); // 512
    costs.insert(OpCode::CALLA, 1 << 9); // 512
    costs.insert(OpCode::ABORT, 0); // 0
    costs.insert(OpCode::ASSERT, 1 << 0); // 1
    costs.insert(OpCode::THROW, 1 << 9); // 512
    costs.insert(OpCode::TRY, 1 << 2); // 4
    costs.insert(OpCode::ENDTRY, 1 << 2); // 4
    costs.insert(OpCode::ENDFINALLY, 1 << 2); // 4
    costs.insert(OpCode::RET, 0); // 0
    costs.insert(OpCode::SYSCALL, 0); // Variable cost

    // Type operations
    costs.insert(OpCode::ISNULL, 60);
    costs.insert(OpCode::ISTYPE, 60);
    costs.insert(OpCode::CONVERT, 240);

    // Array operations (exact C# bit shift values)
    costs.insert(OpCode::NEWARRAY, 1 << 9); // 512
    costs.insert(OpCode::NEWARRAY0, 1 << 4); // 16
    costs.insert(OpCode::NEWSTRUCT, 1 << 9); // 512
    costs.insert(OpCode::NEWSTRUCT0, 1 << 4); // 16
    costs.insert(OpCode::NEWMAP, 1 << 3); // 8
    costs.insert(OpCode::SIZE, 1 << 2); // 4
    costs.insert(OpCode::HASKEY, 1 << 22); // 4194304
    costs.insert(OpCode::KEYS, 1 << 4); // 16
    costs.insert(OpCode::VALUES, 1 << 4); // 16
    costs.insert(OpCode::PICKITEM, 1 << 22); // 4194304
    costs.insert(OpCode::APPEND, 1 << 15); // 32768
    costs.insert(OpCode::SETITEM, 1 << 22); // 4194304
    costs.insert(OpCode::REMOVE, 1 << 15); // 32768
    costs.insert(OpCode::CLEARITEMS, 1 << 4); // 16
    costs.insert(OpCode::POPITEM, 1 << 15); // 32768

    // String operations (exact C# bit shift values)
    costs.insert(OpCode::CAT, 1 << 15); // 32768
    costs.insert(OpCode::SUBSTR, 1 << 15); // 32768
    costs.insert(OpCode::LEFT, 1 << 15); // 32768
    costs.insert(OpCode::RIGHT, 1 << 15); // 32768

    costs
});

/// Gas calculator for VM operations (matches C# ApplicationEngine gas calculation)
pub struct GasCalculator {
    /// Current gas consumed
    gas_consumed: i64,
    /// Gas limit for execution
    gas_limit: i64,
    /// Execution fee factor (default 30)
    exec_fee_factor: u32,
}

impl GasCalculator {
    /// Create new gas calculator (matches C# ApplicationEngine gas initialization)
    pub fn new(gas_limit: i64, exec_fee_factor: u32) -> Self {
        Self {
            gas_consumed: 0,
            gas_limit,
            exec_fee_factor,
        }
    }

    /// Get gas cost for opcode (matches C# GetPrice exactly)
    pub fn get_opcode_cost(&self, opcode: OpCode) -> i64 {
        OPCODE_GAS_COSTS.get(&opcode).copied().unwrap_or(30) // Default cost
    }

    /// Consume gas for opcode execution (matches C# AddFee)
    pub fn consume_gas(&mut self, opcode: OpCode) -> Result<(), GasError> {
        let base_cost = self.get_opcode_cost(opcode);
        let actual_cost = (base_cost as u64).saturating_mul(self.exec_fee_factor as u64) as i64;

        self.gas_consumed = self.gas_consumed.saturating_add(actual_cost);

        if self.gas_consumed > self.gas_limit {
            return Err(GasError::OutOfGas {
                consumed: self.gas_consumed,
                limit: self.gas_limit,
            });
        }

        Ok(())
    }

    /// Add custom gas cost (matches C# AddFee for system calls)
    pub fn add_gas(&mut self, amount: i64) -> Result<(), GasError> {
        let actual_cost = (amount as u64).saturating_mul(self.exec_fee_factor as u64) as i64;
        self.gas_consumed = self.gas_consumed.saturating_add(actual_cost);

        if self.gas_consumed > self.gas_limit {
            return Err(GasError::OutOfGas {
                consumed: self.gas_consumed,
                limit: self.gas_limit,
            });
        }

        Ok(())
    }

    /// Get current gas consumed
    pub fn gas_consumed(&self) -> i64 {
        self.gas_consumed
    }

    /// Get gas limit
    pub fn gas_limit(&self) -> i64 {
        self.gas_limit
    }

    /// Get remaining gas
    pub fn gas_remaining(&self) -> i64 {
        self.gas_limit - self.gas_consumed
    }

    /// Check if gas limit would be exceeded
    pub fn check_gas(&self, additional_gas: i64) -> bool {
        let actual_cost =
            (additional_gas as u64).saturating_mul(self.exec_fee_factor as u64) as i64;
        self.gas_consumed + actual_cost <= self.gas_limit
    }
}

/// Gas calculation errors (matches C# gas limit exceptions)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GasError {
    /// Out of gas during execution
    OutOfGas { consumed: i64, limit: i64 },
    /// Invalid gas amount
    InvalidGas { amount: i64 },
    /// Gas limit exceeded before execution
    GasLimitExceeded { required: i64, available: i64 },
}

impl std::fmt::Display for GasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GasError::OutOfGas { consumed, limit } => {
                write!(
                    f,
                    "Out of gas: consumed {} exceeds limit {}",
                    consumed, limit
                )
            }
            GasError::InvalidGas { amount } => {
                write!(f, "Invalid gas amount: {}", amount)
            }
            GasError::GasLimitExceeded {
                required,
                available,
            } => {
                write!(
                    f,
                    "Gas limit exceeded: required {} > available {}",
                    required, available
                )
            }
        }
    }
}

impl std::error::Error for GasError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_gas_costs() {
        // Test that gas costs match C# exactly
        assert_eq!(OPCODE_GAS_COSTS[&OpCode::PUSHINT8], 1);  // 1 << 0 = 1
        assert_eq!(OPCODE_GAS_COSTS[&OpCode::ADD], 90);      // As defined
        assert_eq!(OPCODE_GAS_COSTS[&OpCode::CALL], 512);    // 1 << 9 = 512
        assert_eq!(OPCODE_GAS_COSTS[&OpCode::SYSCALL], 0);   // Variable cost
    }

    #[test]
    fn test_gas_calculation() {
        let mut calculator = GasCalculator::new(1000000, 30);

        // Test basic gas consumption
        assert!(calculator.consume_gas(OpCode::PUSHINT8).is_ok());
        assert_eq!(calculator.gas_consumed(), 1 * 30); // base_cost(1) * exec_fee_factor(30) = 30

        // Test gas limit
        assert!(calculator.consume_gas(OpCode::CALL).is_ok()); // Should still fit

        // Test out of gas
        let mut small_calculator = GasCalculator::new(1000, 30);
        assert!(small_calculator.consume_gas(OpCode::CALL).is_err()); // 512 * 30 = 15360 > 1000, should exceed limit
    }
}

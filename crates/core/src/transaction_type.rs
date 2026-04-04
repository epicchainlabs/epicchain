// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Transaction type definitions for the Neo blockchain.

use std::fmt;

/// Represents the result of checking if a transaction exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainsTransactionType {
    /// The transaction does not exist.
    NotExist,

    /// The transaction exists in the memory pool.
    ExistsInPool,

    /// The transaction exists in the blockchain ledger.
    ExistsInLedger,
}

impl fmt::Display for ContainsTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContainsTransactionType::NotExist => write!(f, "NotExist"),
            ContainsTransactionType::ExistsInPool => write!(f, "ExistsInPool"),
            ContainsTransactionType::ExistsInLedger => write!(f, "ExistsInLedger"),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_transaction_type_display() {
        assert_eq!(ContainsTransactionType::NotExist.to_string(), "NotExist");
        assert_eq!(
            ContainsTransactionType::ExistsInPool.to_string(),
            "ExistsInPool"
        );
        assert_eq!(
            ContainsTransactionType::ExistsInLedger.to_string(),
            "ExistsInLedger"
        );
    }
}

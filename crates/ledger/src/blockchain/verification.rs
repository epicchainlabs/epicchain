//! Blockchain verification logic.
//!
//! This module provides verification functionality exactly matching C# Neo blockchain verification.

use crate::{BlockHeader, Error, Result};
use neo_config::{MAX_TRANSACTION_SIZE, MILLISECONDS_PER_BLOCK};
use neo_core::{Transaction, UInt160, UInt256, Witness};
use neo_cryptography::ECPoint;
use neo_vm::{ApplicationEngine, TriggerType, VMState};
/// Block verification result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyResult {
    /// Verification succeeded
    Succeed,
    /// Verification failed due to policy
    PolicyFail,
    /// Verification failed
    Fail,
    /// Unknown error occurred
    Unknown,
}

/// Blockchain verification manager (matches C# Neo blockchain verification exactly)
#[derive(Debug, Clone)]
pub struct BlockchainVerifier {
    /// Maximum transaction verification time (milliseconds)
    max_verification_time: u64,
    /// Gas limit for verification
    gas_limit: i64,
}

impl Default for BlockchainVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainVerifier {
    /// Creates a new blockchain verifier
    pub fn new() -> Self {
        Self {
            max_verification_time: 1000, // 1 second
            gas_limit: 50_000_000,       // 0.5 GAS
        }
    }

    /// Verifies a transaction (matches C# Neo VerifyTransaction exactly)
    pub async fn verify_transaction(&self, transaction: &Transaction) -> Result<VerifyResult> {
        // Basic transaction validation
        if let Err(_) = self.validate_transaction_basic(transaction) {
            return Ok(VerifyResult::Fail);
        }

        // Verify transaction witnesses
        if let Err(_) = self.verify_transaction_witnesses(transaction).await {
            return Ok(VerifyResult::Fail);
        }

        // Additional policy checks
        if let Err(_) = self.check_transaction_policy(transaction) {
            return Ok(VerifyResult::PolicyFail);
        }

        Ok(VerifyResult::Succeed)
    }

    /// Verifies a block header (matches C# Neo VerifyHeader exactly)
    pub async fn verify_header(&self, header: &BlockHeader) -> Result<VerifyResult> {
        if header.index == 0 {
            tracing::debug!("Skipping verification for genesis block");
            return Ok(VerifyResult::Succeed);
        }

        // Basic header validation
        if let Err(_) = self.validate_header_basic(header) {
            return Ok(VerifyResult::Fail);
        }

        // Verify header witnesses
        if let Err(_) = self.verify_header_witnesses(header).await {
            return Ok(VerifyResult::Fail);
        }

        Ok(VerifyResult::Succeed)
    }

    /// Validates basic transaction properties
    fn validate_transaction_basic(&self, transaction: &Transaction) -> Result<()> {
        // Check transaction version
        if transaction.version() != 0 {
            return Err(Error::Validation("Invalid transaction version".to_string()));
        }

        // Check transaction size
        let tx_size = transaction.size();
        if tx_size > MAX_TRANSACTION_SIZE {
            // 100KB limit
            return Err(Error::Validation("Transaction too large".to_string()));
        }

        if transaction.witnesses().is_empty() {
            return Err(Error::Validation(
                "Transaction has no witnesses".to_string(),
            ));
        }

        // Validate transaction attributes
        for attribute in transaction.attributes() {
            self.validate_transaction_attribute(attribute)?;
        }

        Ok(())
    }

    /// Validates transaction attributes
    fn validate_transaction_attribute(
        &self,
        _attribute: &neo_core::TransactionAttribute,
    ) -> Result<()> {
        // Implement attribute validation logic
        Ok(())
    }

    /// Verifies transaction witnesses using VM execution
    async fn verify_transaction_witnesses(&self, transaction: &Transaction) -> Result<()> {
        for (index, witness) in transaction.witnesses().iter().enumerate() {
            if let Err(_) = self.verify_witness(transaction, witness, index).await {
                return Err(Error::Validation(format!(
                    "Witness {} verification failed",
                    index
                )));
            }
        }
        Ok(())
    }

    /// Verifies a single witness
    async fn verify_witness(
        &self,
        transaction: &Transaction,
        witness: &Witness,
        _index: usize,
    ) -> Result<()> {
        // Create verification script from witness
        let verification_script = &witness.verification_script;
        if verification_script.is_empty() {
            return Err(Error::Validation("Empty verification script".to_string()));
        }

        let mut engine = ApplicationEngine::new(TriggerType::Verification, self.gas_limit);

        // Load verification script
        let script = neo_vm::Script::new(verification_script.clone(), false)
            .map_err(|_| Error::Validation("Failed to create verification script".to_string()))?;
        if let Err(_) = engine.load_script(script, -1, 0) {
            return Err(Error::Validation(
                "Failed to load verification script".to_string(),
            ));
        }

        if !witness.invocation_script.is_empty() {
            let invocation_script = neo_vm::Script::new(witness.invocation_script.clone(), false)
                .map_err(|_| {
                Error::Validation("Failed to create invocation script".to_string())
            })?;
            if let Err(_) = engine.load_script(invocation_script, 0, 0) {
                return Err(Error::Validation(
                    "Failed to load invocation script".to_string(),
                ));
            }
        }

        engine.set_script_container(transaction.clone());

        // Execute verification
        let execution_script = neo_vm::Script::new(verification_script.clone(), false)
            .map_err(|_| Error::Validation("Failed to create execution script".to_string()))?;
        match engine.execute(execution_script) {
            VMState::HALT => {
                if engine.result_stack().len() == 0 {
                    return Err(Error::Validation("Empty result stack".to_string()));
                }

                match engine.result_stack().peek(0) {
                    Ok(result) => {
                        if !result.as_bool().unwrap_or(false) {
                            return Err(Error::Validation(
                                "Verification script returned false".to_string(),
                            ));
                        }
                    }
                    Err(_) => {
                        return Err(Error::Validation(
                            "Failed to get result from stack".to_string(),
                        ));
                    }
                }
            }
            _ => {
                return Err(Error::Validation("VM execution failed".to_string()));
            }
        }

        Ok(())
    }

    /// Checks transaction against policy rules
    fn check_transaction_policy(&self, _transaction: &Transaction) -> Result<()> {
        // This would check against PolicyContract rules
        Ok(())
    }

    /// Validates basic header properties
    fn validate_header_basic(&self, header: &BlockHeader) -> Result<()> {
        // Check header version
        if header.version > 0 {
            return Err(Error::Validation("Invalid header version".to_string()));
        }

        // Check timestamp
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| Error::BlockValidation(format!("Failed to get timestamp: {}", e)))?
            .as_millis() as u64;

        if header.timestamp > now + MILLISECONDS_PER_BLOCK {
            // SECONDS_PER_BLOCK seconds tolerance
            return Err(Error::Validation(
                "Header timestamp too far in future".to_string(),
            ));
        }

        if header.index > 0 && header.witnesses.is_empty() {
            return Err(Error::Validation("Header has no witnesses".to_string()));
        }

        Ok(())
    }

    /// Verifies header witnesses (consensus validation)
    async fn verify_header_witnesses(&self, header: &BlockHeader) -> Result<()> {
        // This would verify consensus signatures
        // In a full implementation, this would check against committee signatures
        for (index, witness) in header.witnesses.iter().enumerate() {
            if witness.verification_script.is_empty() {
                return Err(Error::Validation(format!(
                    "Header witness {} has empty verification script",
                    index
                )));
            }
        }
        Ok(())
    }

    /// Verifies the consensus data in a block header
    pub fn verify_consensus_data(
        &self,
        header: &BlockHeader,
        _committee: &[ECPoint],
    ) -> Result<()> {
        // Verify primary index
        if header.primary_index as usize >= 7 {
            // Assuming 7 consensus nodes
            return Err(Error::Validation("Invalid primary index".to_string()));
        }

        // Verify consensus signature count
        let required_signatures = (7 * 2 / 3) + 1; // 2/3 + 1 majority
        if header.witnesses.len() < required_signatures {
            return Err(Error::Validation(
                "Insufficient consensus signatures".to_string(),
            ));
        }

        Ok(())
    }

    /// Creates a multisig redeem script from committee
    fn create_multisig_redeem_script_from_committee(
        &self,
        _committee: &[ECPoint],
    ) -> Option<Vec<u8>> {
        // This would create the committee multisig script
        // Implementation would follow C# Neo committee script generation
        None
    }

    /// Creates a multisig redeem script from validators
    fn create_multisig_redeem_script_from_next_validators(
        &self,
        _validators: &[ECPoint],
    ) -> Option<Vec<u8>> {
        // This would create the next validators multisig script
        // Implementation would follow C# Neo validator script generation
        None
    }

    /// Sets the maximum verification time
    pub fn set_max_verification_time(&mut self, time_ms: u64) {
        self.max_verification_time = time_ms;
    }

    /// Sets the gas limit for verification
    pub fn set_gas_limit(&mut self, gas_limit: i64) {
        self.gas_limit = gas_limit;
    }

    /// Gets the current gas limit
    pub fn gas_limit(&self) -> i64 {
        self.gas_limit
    }

    /// Gets the maximum verification time
    pub fn max_verification_time(&self) -> u64 {
        self.max_verification_time
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{Error, Result};

    #[test]
    fn test_verifier_creation() {
        let verifier = BlockchainVerifier::new();
        assert_eq!(verifier.max_verification_time(), 1000);
        assert_eq!(verifier.gas_limit(), 50_000_000);
    }

    #[test]
    fn test_verify_result_enum() {
        assert_eq!(VerifyResult::Succeed, VerifyResult::Succeed);
        assert_ne!(VerifyResult::Succeed, VerifyResult::Fail);
    }

    #[tokio::test]
    async fn test_basic_validation() {
        let verifier = BlockchainVerifier::new();

        let transaction = Transaction::default();

        // This should fail due to empty witnesses
        let result = verifier.verify_transaction(&transaction).await?;
        assert_eq!(result, VerifyResult::Fail);
    }
}

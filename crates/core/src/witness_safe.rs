//! Safe implementation of Witness operations without unwrap()
//!
//! This module demonstrates how to refactor existing code to use safe error handling.

use crate::error::CoreError;
use crate::safe_result::{SafeOption, SafeResult};
use crate::witness::Witness;
use neo_io::{BinaryWriter, MemoryReader, Serializable};

/// Safe witness serialization and deserialization
pub struct SafeWitnessOperations;

impl SafeWitnessOperations {
    /// Safely serialize a witness to bytes
    pub fn serialize_witness(witness: &Witness) -> Result<Vec<u8>, CoreError> {
        let mut writer = BinaryWriter::new();

        <Witness as Serializable>::serialize(witness, &mut writer)
            .with_context("Failed to serialize witness")?;

        Ok(writer.to_bytes())
    }

    /// Safely deserialize a witness from bytes
    pub fn deserialize_witness(bytes: &[u8]) -> Result<Witness, CoreError> {
        let mut reader = MemoryReader::new(bytes);

        <Witness as Serializable>::deserialize(&mut reader)
            .with_context("Failed to deserialize witness")
    }

    /// Safe round-trip test for witness serialization
    pub fn test_witness_round_trip(witness: &Witness) -> Result<bool, CoreError> {
        // Serialize the witness
        let serialized = Self::serialize_witness(witness)?;

        // Deserialize it back
        let deserialized = Self::deserialize_witness(&serialized)?;

        // Compare the results
        Ok(witness.invocation_script == deserialized.invocation_script
            && witness.verification_script == deserialized.verification_script)
    }

    /// Safe batch processing of witnesses
    pub fn process_witnesses(witnesses: &[Witness]) -> Result<Vec<Vec<u8>>, CoreError> {
        witnesses
            .iter()
            .enumerate()
            .map(|(index, witness)| {
                Self::serialize_witness(witness)
                    .with_context(&format!("Failed to process witness at index {}", index))
            })
            .collect()
    }

    /// Safe witness validation
    pub fn validate_witness(witness: &Witness) -> Result<(), CoreError> {
        if witness.invocation_script.is_empty() {
            return Err(CoreError::InvalidData {
                message: "Invocation script cannot be empty".to_string(),
            });
        }

        if witness.verification_script.is_empty() {
            return Err(CoreError::InvalidData {
                message: "Verification script cannot be empty".to_string(),
            });
        }

        if witness.invocation_script.len() > 65536 {
            return Err(CoreError::InvalidData {
                message: format!(
                    "Invocation script too large: {} bytes (max: 65536)",
                    witness.invocation_script.len()
                ),
            });
        }

        if witness.verification_script.len() > 65536 {
            return Err(CoreError::InvalidData {
                message: format!(
                    "Verification script too large: {} bytes (max: 65536)",
                    witness.verification_script.len()
                ),
            });
        }

        Ok(())
    }
}

/// Safe builder pattern for Witness construction
pub struct SafeWitnessBuilder {
    invocation_script: Option<Vec<u8>>,
    verification_script: Option<Vec<u8>>,
}

impl SafeWitnessBuilder {
    /// Create a new witness builder
    pub fn new() -> Self {
        Self {
            invocation_script: None,
            verification_script: None,
        }
    }

    /// Set the invocation script
    pub fn with_invocation_script(mut self, script: Vec<u8>) -> Self {
        self.invocation_script = Some(script);
        self
    }

    /// Set the verification script
    pub fn with_verification_script(mut self, script: Vec<u8>) -> Self {
        self.verification_script = Some(script);
        self
    }

    /// Build the witness with validation
    pub fn build(self) -> Result<Witness, CoreError> {
        let invocation_script = self
            .invocation_script
            .ok_or_context("Invocation script is required")?;

        let verification_script = self
            .verification_script
            .ok_or_context("Verification script is required")?;

        let witness = Witness::new_with_scripts(invocation_script, verification_script);

        // Validate before returning
        SafeWitnessOperations::validate_witness(&witness)?;

        Ok(witness)
    }
}

impl Default for SafeWitnessBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_witness_serialization() {
        let witness = Witness::new_with_scripts(vec![0x01, 0x02, 0x03], vec![0x04, 0x05, 0x06]);

        let result = SafeWitnessOperations::serialize_witness(&witness);
        assert!(result.is_ok());

        let serialized = result.unwrap();
        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_safe_witness_deserialization() {
        let witness = Witness::new_with_scripts(vec![0x01, 0x02, 0x03], vec![0x04, 0x05, 0x06]);

        let serialized = SafeWitnessOperations::serialize_witness(&witness).unwrap();
        let deserialized = SafeWitnessOperations::deserialize_witness(&serialized);

        assert!(deserialized.is_ok());
        let deserialized = deserialized.unwrap();
        assert_eq!(witness.invocation_script, deserialized.invocation_script);
        assert_eq!(
            witness.verification_script,
            deserialized.verification_script
        );
    }

    #[test]
    fn test_safe_witness_round_trip() {
        let witness = Witness::new_with_scripts(vec![0x01, 0x02, 0x03], vec![0x04, 0x05, 0x06]);

        let result = SafeWitnessOperations::test_witness_round_trip(&witness);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_witness_validation() {
        // Valid witness
        let witness = Witness::new_with_scripts(vec![0x01, 0x02, 0x03], vec![0x04, 0x05, 0x06]);
        assert!(SafeWitnessOperations::validate_witness(&witness).is_ok());

        // Empty invocation script
        let witness = Witness::new_with_scripts(vec![], vec![0x04, 0x05, 0x06]);
        let result = SafeWitnessOperations::validate_witness(&witness);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invocation script cannot be empty"));

        // Empty verification script
        let witness = Witness::new_with_scripts(vec![0x01, 0x02, 0x03], vec![]);
        let result = SafeWitnessOperations::validate_witness(&witness);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Verification script cannot be empty"));
    }

    #[test]
    fn test_safe_witness_builder() {
        // Successful build
        let witness = SafeWitnessBuilder::new()
            .with_invocation_script(vec![0x01, 0x02, 0x03])
            .with_verification_script(vec![0x04, 0x05, 0x06])
            .build();

        assert!(witness.is_ok());
        let witness = witness.unwrap();
        assert_eq!(witness.invocation_script, vec![0x01, 0x02, 0x03]);
        assert_eq!(witness.verification_script, vec![0x04, 0x05, 0x06]);

        // Missing invocation script
        let result = SafeWitnessBuilder::new()
            .with_verification_script(vec![0x04, 0x05, 0x06])
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invocation script is required"));

        // Missing verification script
        let result = SafeWitnessBuilder::new()
            .with_invocation_script(vec![0x01, 0x02, 0x03])
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Verification script is required"));
    }

    #[test]
    fn test_batch_witness_processing() {
        let witnesses = vec![
            Witness::new_with_scripts(vec![0x01], vec![0x02]),
            Witness::new_with_scripts(vec![0x03], vec![0x04]),
            Witness::new_with_scripts(vec![0x05], vec![0x06]),
        ];

        let result = SafeWitnessOperations::process_witnesses(&witnesses);
        assert!(result.is_ok());

        let processed = result.unwrap();
        assert_eq!(processed.len(), 3);
    }
}

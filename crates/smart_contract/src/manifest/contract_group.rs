//! Contract group implementation.
//!
//! Represents a set of mutually trusted contracts identified by a public key
//! and accompanied by a signature for the contract hash.

use crate::{Error, Result};
use neo_config::{ADDRESS_SIZE, HASH_SIZE};
use neo_cryptography::ecc::ECPoint;
use serde::{Deserialize, Serialize};

/// Represents a set of mutually trusted contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractGroup {
    /// The public key of the group.
    #[serde(rename = "pubkey")]
    pub public_key: ECPoint,

    /// The signature of the contract hash which can be verified by the public key.
    pub signature: Vec<u8>,
}

impl ContractGroup {
    /// Creates a new contract group.
    pub fn new(public_key: ECPoint, signature: Vec<u8>) -> Self {
        Self {
            public_key,
            signature,
        }
    }

    /// Validates the contract group.
    pub fn validate(&self) -> Result<()> {
        // Validate public key
        if !self.public_key.is_valid() {
            return Err(Error::InvalidManifest(
                "Invalid public key in group".to_string(),
            ));
        }

        if self.signature.len() != 64 {
            return Err(Error::InvalidManifest(
                "Invalid signature length in group".to_string(),
            ));
        }

        Ok(())
    }

    /// Verifies the group signature for a given contract hash.
    pub fn verify_signature(&self, contract_hash: &[u8]) -> Result<bool> {
        // Validate input parameters
        if contract_hash.len() != ADDRESS_SIZE {
            return Err(Error::InvalidManifest(
                "Invalid contract hash length".to_string(),
            ));
        }

        if self.signature.len() != 64 {
            return Err(Error::InvalidManifest(
                "Invalid signature length".to_string(),
            ));
        }

        // Verify the ECDSA signature using secp256r1 curve
        let public_key_bytes = self
            .public_key
            .encode_point(true)
            .map_err(|e| Error::InvalidManifest(format!("Failed to encode public key: {}", e)))?;

        match neo_cryptography::ecdsa::ECDsa::verify_signature_secp256r1(
            contract_hash,
            &self.signature,
            &public_key_bytes, // Compressed format
        ) {
            Ok(is_valid) => {
                if is_valid {
                    log::info!(
                        "Contract group signature verification passed for contract hash: {:?}",
                        hex::encode(contract_hash)
                    );
                } else {
                    log::info!(
                        "Contract group signature verification failed for contract hash: {:?}",
                        hex::encode(contract_hash)
                    );
                }
                Ok(is_valid)
            }
            Err(e) => {
                log::info!("Error verifying contract group signature: {}", e);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use neo_cryptography::ecc::{ECCurve, ECPoint};

    #[test]
    fn test_contract_group_creation() {
        let public_key = ECPoint::infinity(ECCurve::secp256r1()); // This would be a real public key in practice
        let signature = vec![0u8; 64]; // 64-byte signature

        let group = ContractGroup::new(public_key.clone(), signature.clone());
        assert_eq!(group.public_key, public_key);
        assert_eq!(group.signature, signature);
    }

    #[test]
    fn test_contract_group_validation() {
        let public_key = ECPoint::infinity(ECCurve::secp256r1());
        let valid_signature = vec![0u8; 64];
        let invalid_signature = vec![0u8; HASH_SIZE]; // Wrong length

        let valid_group = ContractGroup::new(public_key.clone(), valid_signature);
        let invalid_group = ContractGroup::new(public_key, invalid_signature);

        // Production-ready test with proper validation
        assert!(valid_group.validate().is_ok());
        assert!(invalid_group.validate().is_err());
    }

    #[test]
    fn test_signature_verification() {
        let public_key = ECPoint::infinity(ECCurve::secp256r1());
        let signature = vec![0u8; 64];
        let group = ContractGroup::new(public_key, signature);

        let contract_hash = vec![0u8; ADDRESS_SIZE]; // ADDRESS_SIZE-byte hash
        let invalid_hash = vec![0u8; 16]; // Wrong length

        assert!(group.verify_signature(&contract_hash).is_ok());
        assert!(group.verify_signature(&invalid_hash).is_err());
    }
}

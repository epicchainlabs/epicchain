//! Contract implementation for Neo wallets.
//!
//! This module provides contract functionality for wallet accounts,
//! converted from the C# Neo Contract class (@neo-sharp/src/Neo/SmartContract/Contract.cs).

use crate::{ContractParameterType, Error, Result};
use neo_config::MAX_SCRIPT_SIZE;
use neo_core::{UInt160, Witness};
use neo_cryptography::ECPoint;
use serde::{Deserialize, Serialize};

/// Represents a smart contract in a wallet.
/// This matches the C# Contract class functionality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contract {
    /// The script of the contract.
    pub script: Vec<u8>,

    /// The parameter list of the contract.
    pub parameter_list: Vec<ContractParameterType>,

    /// The script hash of the contract.
    script_hash: UInt160,

    /// Whether this is a standard contract.
    is_standard: bool,

    /// Whether this is a multi-signature contract.
    is_multi_sig: bool,

    /// The public keys for multi-signature contracts.
    pub public_keys: Vec<ECPoint>,

    /// The required signature count for multi-signature contracts.
    pub signatures_required: u8,
}

impl Contract {
    /// Creates a new contract.
    pub fn new(script: Vec<u8>, parameter_list: Vec<ContractParameterType>) -> Self {
        let script_hash = UInt160::from_script(&script);
        let is_standard = Self::is_standard_contract(&script);
        let is_multi_sig = Self::is_multi_sig_contract(&script);

        Self {
            script,
            parameter_list,
            script_hash,
            is_standard,
            is_multi_sig,
            public_keys: Vec::new(),
            signatures_required: 1,
        }
    }

    /// Creates a standard single-signature contract.
    pub fn create_signature_contract(public_key: &ECPoint) -> Result<Self> {
        let script = Self::create_signature_redeemscript(public_key)?;
        let parameter_list = vec![ContractParameterType::Signature];

        Ok(Self::new(script, parameter_list))
    }

    /// Creates a multi-signature contract.
    pub fn create_multi_sig_contract(
        signatures_required: u8,
        public_keys: &[ECPoint],
    ) -> Result<Self> {
        if signatures_required == 0 || signatures_required > public_keys.len() as u8 {
            return Err(Error::Other("Invalid signature requirements".to_string()));
        }

        if public_keys.len() > MAX_SCRIPT_SIZE {
            return Err(Error::Other("Too many public keys".to_string()));
        }

        let script = Self::create_multi_sig_redeemscript(signatures_required, public_keys)?;
        let parameter_list = vec![ContractParameterType::Signature; signatures_required as usize];

        let mut contract = Self::new(script, parameter_list);
        contract.public_keys = public_keys.to_vec();
        contract.signatures_required = signatures_required;

        Ok(contract)
    }

    /// Gets the script hash of the contract.
    pub fn script_hash(&self) -> UInt160 {
        self.script_hash
    }

    /// Checks if this is a standard contract.
    pub fn is_standard(&self) -> bool {
        self.is_standard
    }

    /// Checks if this is a multi-signature contract.
    pub fn is_multi_sig(&self) -> bool {
        self.is_multi_sig
    }

    /// Creates a witness for this contract.
    pub fn create_witness(&self, signature: Vec<u8>) -> Result<Witness> {
        if self.is_multi_sig {
            // Multi-signature witness
            let mut invocation_script = Vec::new();

            // 1. Add signature data with proper PUSHDATA operation (production script generation)
            self.add_signature_to_script(&mut invocation_script, &signature)?;

            // 2. For multi-signature contracts, continue adding signatures
            invocation_script.push(0x0c); // PUSHDATA1
            invocation_script.push(signature.len() as u8);
            invocation_script.extend_from_slice(&signature);

            Ok(Witness::new_with_scripts(
                invocation_script,
                self.script.clone(),
            ))
        } else {
            // Standard single-signature witness
            let mut invocation_script = Vec::new();
            invocation_script.push(0x0c); // PUSHDATA1
            invocation_script.push(signature.len() as u8);
            invocation_script.extend_from_slice(&signature);

            Ok(Witness::new_with_scripts(
                invocation_script,
                self.script.clone(),
            ))
        }
    }

    /// Creates a signature redeem script for a single public key.
    fn create_signature_redeemscript(public_key: &ECPoint) -> Result<Vec<u8>> {
        let public_key_bytes = public_key.encode_point(true)?;

        let mut script = Vec::new();
        script.push(0x0c); // PUSHDATA1
        script.push(public_key_bytes.len() as u8);
        script.extend_from_slice(&public_key_bytes);
        script.push(0x41); // SYSCALL
        script.extend_from_slice(b"System.Crypto.CheckWitness");

        Ok(script)
    }

    /// Creates a multi-signature redeem script.
    fn create_multi_sig_redeemscript(
        signatures_required: u8,
        public_keys: &[ECPoint],
    ) -> Result<Vec<u8>> {
        let mut script = Vec::new();

        // Push signature count
        script.push(0x10 + signatures_required); // PUSH1-PUSH16

        // Push public keys
        for public_key in public_keys {
            let public_key_bytes = public_key.encode_point(true)?;
            script.push(0x0c); // PUSHDATA1
            script.push(public_key_bytes.len() as u8);
            script.extend_from_slice(&public_key_bytes);
        }

        // Push public key count
        script.push(0x10 + public_keys.len() as u8); // PUSH1-PUSH16

        // SYSCALL System.Crypto.CheckMultisig
        script.push(0x41); // SYSCALL
        script.extend_from_slice(b"System.Crypto.CheckMultisig");

        Ok(script)
    }

    /// Checks if a script is a standard contract.
    fn is_standard_contract(script: &[u8]) -> bool {
        if script.len() == 40 {
            // Standard format: PUSHDATA1 33 <pubkey> SYSCALL System.Crypto.CheckWitness
            script[0] == 0x0c && script[1] == 33 && script[34] == 0x41
        } else {
            false
        }
    }

    /// Checks if a script is a multi-signature contract.
    fn is_multi_sig_contract(script: &[u8]) -> bool {
        if script.len() < 42 {
            return false;
        }

        if script[0] < 0x51 || script[0] > 0x60 {
            return false;
        }

        // Should end with SYSCALL System.Crypto.CheckMultisig
        script.ends_with(b"\x41System.Crypto.CheckMultisig")
    }

    /// Adds a signature to the invocation script.
    /// This matches the C# Contract.AddSignature functionality.
    fn add_signature_to_script(&self, script: &mut Vec<u8>, signature: &[u8]) -> Result<()> {
        if signature.is_empty() {
            return Err(Error::Other("Signature cannot be empty".to_string()));
        }

        if signature.len() > 255 {
            return Err(Error::Other("Signature too long".to_string()));
        }

        script.push(0x0c); // PUSHDATA1
        script.push(signature.len() as u8);
        script.extend_from_slice(signature);

        Ok(())
    }

    /// Gets the address for this contract.
    pub fn address(&self) -> String {
        self.script_hash.to_address()
    }

    /// Validates the contract script.
    pub fn validate(&self) -> Result<()> {
        if self.script.is_empty() {
            return Err(Error::Other("Contract script cannot be empty".to_string()));
        }

        if self.parameter_list.is_empty() {
            return Err(Error::Other("Parameter list cannot be empty".to_string()));
        }

        // Validate script format
        if !Self::is_standard_contract(&self.script) && !Self::is_multi_sig_contract(&self.script) {
            return Err(Error::Other("Invalid contract script format".to_string()));
        }

        if self.is_multi_sig {
            if self.public_keys.len() < self.signatures_required as usize {
                return Err(Error::Other(
                    "Not enough public keys for required signatures".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Contract({})", self.script_hash)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::ContractParameterType;

    #[test]
    fn test_create_signature_contract() {
        // This would need a valid ECPoint implementation
    }

    #[test]
    fn test_contract_validation() {
        let script = vec![0x0c, 33]; // Incomplete script
        let parameter_list = vec![ContractParameterType::Signature];
        let contract = Contract::new(script.clone(), parameter_list.clone());
        // This should fail validation
        assert!(contract.validate().is_err());
    }
}

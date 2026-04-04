// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Builder for transaction signers.

use crate::{Signer, UInt160, WitnessRule, WitnessScope};

/// Builder for transaction signers (matches C# SignerBuilder exactly).
#[derive(Debug)]
pub struct SignerBuilder {
    signer: Signer,
}

impl SignerBuilder {
    /// Creates an empty SignerBuilder (matches C# SignerBuilder.CreateEmpty exactly).
    ///
    /// # Returns
    ///
    /// A new SignerBuilder instance with default signer.
    pub fn create_empty() -> Self {
        Self {
            signer: Signer {
                account: UInt160::zero(),
                scopes: WitnessScope::NONE,
                allowed_contracts: Vec::new(),
                allowed_groups: Vec::new(),
                rules: Vec::new(),
            },
        }
    }

    /// Sets the account for the signer (matches C# SignerBuilder.Account exactly).
    ///
    /// # Arguments
    ///
    /// * `script_hash` - The script hash of the account
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn account(mut self, script_hash: UInt160) -> Self {
        self.signer.account = script_hash;
        self
    }

    /// Allows a specific contract (matches C# SignerBuilder.AllowContract exactly).
    ///
    /// # Arguments
    ///
    /// * `contract_hash` - The hash of the contract to allow
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn allow_contract(mut self, contract_hash: UInt160) -> Self {
        self.signer.allowed_contracts.push(contract_hash);
        self
    }

    /// Allows a specific group (matches C# SignerBuilder.AllowGroup exactly).
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key of the group to allow (as serialized bytes)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn allow_group(mut self, public_key: Vec<u8>) -> Self {
        self.signer.allowed_groups.push(public_key);
        self
    }

    /// Adds a witness scope (matches C# SignerBuilder.AddWitnessScope exactly).
    ///
    /// # Arguments
    ///
    /// * `scope` - The witness scope to add
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn add_witness_scope(mut self, scope: WitnessScope) -> Self {
        self.signer.scopes = self.signer.scopes.combine(scope);
        self
    }

    /// Adds a witness rule (matches C# SignerBuilder.AddWitnessRule exactly).
    ///
    /// # Arguments
    ///
    /// * `rule` - The witness rule to add
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn add_witness_rule(mut self, rule: WitnessRule) -> Self {
        self.signer.rules.push(rule);
        self
    }

    /// Builds the signer (matches C# SignerBuilder.Build exactly).
    ///
    /// # Returns
    ///
    /// The built signer
    pub fn build(self) -> Signer {
        self.signer
    }
}

impl Default for SignerBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

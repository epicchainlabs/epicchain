// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Builder for Neo transactions.

/// Builder for Neo transactions (matches C# TransactionBuilder exactly).
#[derive(Debug, Clone)]
pub struct TransactionBuilder {
    tx: crate::Transaction,
}

impl TransactionBuilder {
    /// Creates an empty TransactionBuilder (matches C# TransactionBuilder.CreateEmpty exactly).
    ///
    /// # Returns
    ///
    /// A new TransactionBuilder instance with default transaction.
    pub fn create_empty() -> Self {
        let mut tx = crate::Transaction::new();
        tx.set_script(vec![0x40]); // OpCode.RET (matches C# default)
        Self { tx }
    }

    /// Sets the transaction version (matches C# TransactionBuilder.Version exactly).
    pub fn version(mut self, version: u8) -> Self {
        self.tx.set_version(version);
        self
    }

    /// Sets the transaction nonce (matches C# TransactionBuilder.Nonce exactly).
    pub fn nonce(mut self, nonce: u32) -> Self {
        self.tx.set_nonce(nonce);
        self
    }

    /// Sets the system fee (matches C# TransactionBuilder.SystemFee exactly).
    pub fn system_fee(mut self, system_fee: i64) -> Self {
        self.tx.set_system_fee(system_fee);
        self
    }

    /// Sets the network fee (matches C# TransactionBuilder.NetworkFee exactly).
    pub fn network_fee(mut self, network_fee: i64) -> Self {
        self.tx.set_network_fee(network_fee);
        self
    }

    /// Sets the valid until block (matches C# TransactionBuilder.ValidUntil exactly).
    pub fn valid_until(mut self, block_index: u32) -> Self {
        self.tx.set_valid_until_block(block_index);
        self
    }

    /// Attaches a script to the transaction (matches C# TransactionBuilder.AttachSystem exactly).
    pub fn attach_system(mut self, script: Vec<u8>) -> Self {
        self.tx.set_script(script);
        self
    }

    /// Adds a signer to the transaction (matches C# TransactionBuilder.AddSigner exactly).
    pub fn add_signer(mut self, signer: crate::Signer) -> Self {
        self.tx.add_signer(signer);
        self
    }

    /// Adds a witness to the transaction (matches C# TransactionBuilder.AddWitness exactly).
    pub fn add_witness(mut self, witness: crate::Witness) -> Self {
        self.tx.add_witness(witness);
        self
    }

    /// Builds the transaction (matches C# TransactionBuilder.Build exactly).
    ///
    /// # Returns
    ///
    /// The built transaction.
    pub fn build(self) -> crate::Transaction {
        self.tx
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Builder for transaction witnesses.

use crate::error::{CoreError, CoreResult};
use crate::Witness;

/// Builder for transaction witnesses (matches C# WitnessBuilder exactly).
#[derive(Debug)]
pub struct WitnessBuilder {
    invocation_script: Vec<u8>,
    verification_script: Vec<u8>,
}

impl WitnessBuilder {
    /// Creates an empty WitnessBuilder (matches C# WitnessBuilder.CreateEmpty exactly).
    ///
    /// # Returns
    ///
    /// A new WitnessBuilder instance with empty scripts.
    pub fn create_empty() -> Self {
        Self {
            invocation_script: Vec::new(),
            verification_script: Vec::new(),
        }
    }

    /// Adds an invocation script (matches C# WitnessBuilder.AddInvocation exactly).
    ///
    /// # Arguments
    ///
    /// * `bytes` - The invocation script bytes
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Errors
    ///
    /// Returns an error if invocation script already exists
    pub fn add_invocation(mut self, bytes: Vec<u8>) -> CoreResult<Self> {
        if !self.invocation_script.is_empty() {
            return Err(CoreError::InvalidOperation {
                message: "Invocation script already exists".to_string(),
            });
        }
        self.invocation_script = bytes;
        Ok(self)
    }

    /// Adds a verification script (matches C# WitnessBuilder.AddVerification exactly).
    ///
    /// # Arguments
    ///
    /// * `bytes` - The verification script bytes
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Errors
    ///
    /// Returns an error if verification script already exists
    pub fn add_verification(mut self, bytes: Vec<u8>) -> CoreResult<Self> {
        if !self.verification_script.is_empty() {
            return Err(CoreError::InvalidOperation {
                message: "Verification script already exists".to_string(),
            });
        }
        self.verification_script = bytes;
        Ok(self)
    }

    /// Builds the witness (matches C# WitnessBuilder.Build exactly).
    ///
    /// # Returns
    ///
    /// The built witness
    pub fn build(self) -> Witness {
        Witness::new_with_scripts(self.invocation_script, self.verification_script)
    }
}

impl Default for WitnessBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

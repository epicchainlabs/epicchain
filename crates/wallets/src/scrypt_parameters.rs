//! Scrypt parameters for wallet encryption.
//!
//! This module provides scrypt parameter management for wallet encryption,
//! converted from the C# Neo ScryptParameters class (@neo-sharp/src/Neo/Wallets/ScryptParameters.cs).

use crate::{Error, Result};
use neo_config::{MAX_BLOCK_SIZE, MAX_SCRIPT_SIZE};
use serde::{Deserialize, Serialize};

const ONE_MEGABYTE: usize = 1024 * 1024;
/// Scrypt parameters for key derivation.
/// This matches the C# ScryptParameters class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScryptParameters {
    /// CPU/memory cost parameter (N).
    #[serde(rename = "n")]
    pub n: u32,

    /// Block size parameter (r).
    #[serde(rename = "r")]
    pub r: u32,

    /// Parallelization parameter (p).
    #[serde(rename = "p")]
    pub p: u32,

    /// Derived key length.
    #[serde(rename = "dklen", skip_serializing_if = "Option::is_none")]
    pub dklen: Option<u32>,
}

impl ScryptParameters {
    /// Creates new scrypt parameters.
    pub fn new(n: u32, r: u32, p: u32) -> Result<Self> {
        let params = Self {
            n,
            r,
            p,
            dklen: None,
        };
        params.validate()?;
        Ok(params)
    }

    /// Creates new scrypt parameters with derived key length.
    pub fn new_with_dklen(n: u32, r: u32, p: u32, dklen: u32) -> Result<Self> {
        let params = Self {
            n,
            r,
            p,
            dklen: Some(dklen),
        };
        params.validate()?;
        Ok(params)
    }

    /// Gets the default scrypt parameters for NEP-2.
    pub fn default_nep2() -> Self {
        Self {
            n: 16384,
            r: 8,
            p: 8,
            dklen: Some(64),
        }
    }

    /// Gets the default scrypt parameters for NEP-6.
    pub fn default_nep6() -> Self {
        Self {
            n: 16384,
            r: 8,
            p: 8,
            dklen: Some(64),
        }
    }

    /// Gets fast scrypt parameters (for testing).
    pub fn fast() -> Self {
        Self {
            n: MAX_SCRIPT_SIZE as u32,
            r: 1,
            p: 1,
            dklen: Some(64),
        }
    }

    /// Gets slow scrypt parameters (high security).
    pub fn slow() -> Self {
        Self {
            n: MAX_BLOCK_SIZE as u32, // 2^ADDRESS_SIZE
            r: 8,
            p: 8,
            dklen: Some(64),
        }
    }

    /// Validates the scrypt parameters.
    pub fn validate(&self) -> Result<()> {
        // N must be a power of 2 and greater than 1
        if self.n <= 1 || (self.n & (self.n - 1)) != 0 {
            return Err(Error::Other(
                "N must be a power of 2 greater than 1".to_string(),
            ));
        }

        // R must be greater than 0
        if self.r == 0 {
            return Err(Error::Other("R must be greater than 0".to_string()));
        }

        // P must be greater than 0
        if self.p == 0 {
            return Err(Error::Other("P must be greater than 0".to_string()));
        }

        if self.r > u32::MAX / 128 {
            return Err(Error::Other("R parameter is too large".to_string()));
        }

        if self.p > (u32::MAX - 1) / (128 * self.r) {
            return Err(Error::Other("P parameter is too large".to_string()));
        }

        if self.n > u32::MAX / (128 * self.r) {
            return Err(Error::Other("N parameter is too large".to_string()));
        }

        // Check derived key length
        if let Some(dklen) = self.dklen {
            if dklen == 0 {
                return Err(Error::Other(
                    "Derived key length must be greater than 0".to_string(),
                ));
            }

            if (dklen as u64) > ((MAX_SCRIPT_SIZE as u64) * (MAX_SCRIPT_SIZE as u64)) {
                // 1MB limit
                return Err(Error::Other("Derived key length is too large".to_string()));
            }
        }

        Ok(())
    }

    /// Gets the memory usage in bytes.
    pub fn memory_usage(&self) -> u64 {
        128 * self.r as u64 * self.n as u64
    }

    /// Gets the estimated computation time in milliseconds.
    pub fn estimated_time_ms(&self) -> u64 {
        // Rough estimation based on typical hardware
        let operations = self.n as u64 * self.r as u64 * self.p as u64;
        operations / 1000 // Very rough estimate
    }

    /// Checks if the parameters are considered secure.
    pub fn is_secure(&self) -> bool {
        // Minimum security recommendations
        self.n >= 16384 && self.r >= 8 && self.p >= 1
    }

    /// Checks if the parameters are suitable for production use.
    pub fn is_production_ready(&self) -> bool {
        self.is_secure()
            && self.memory_usage() <= 128 * (MAX_SCRIPT_SIZE as u64) * (MAX_SCRIPT_SIZE as u64)
        // 128MB limit
    }

    /// Gets the log2 of N parameter.
    pub fn log_n(&self) -> u32 {
        self.n.trailing_zeros()
    }

    /// Creates parameters from log2(N).
    pub fn from_log_n(log_n: u8, r: u32, p: u32) -> Result<Self> {
        if log_n > 31 {
            return Err(Error::Other("log_n is too large".to_string()));
        }

        let n = 1u32 << log_n;
        Self::new(n, r, p)
    }

    /// Converts to scrypt crate parameters.
    pub fn to_scrypt_params(&self) -> Result<scrypt::Params> {
        let log_n = self.log_n() as u8;
        let dklen = self.dklen.unwrap_or(64) as usize;

        scrypt::Params::new(log_n, self.r, self.p, dklen).map_err(|e| Error::Scrypt(e.to_string()))
    }

    /// Creates from scrypt crate parameters.
    pub fn from_scrypt_params(params: &scrypt::Params) -> Self {
        Self {
            n: 1u32 << params.log_n(),
            r: params.r(),
            p: params.p(),
            dklen: Some(64), // Standard NEP-2 derived key length
        }
    }
}

impl Default for ScryptParameters {
    fn default() -> Self {
        Self::default_nep6()
    }
}

impl std::fmt::Display for ScryptParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScryptParameters(N={}, r={}, p={}, dklen={:?})",
            self.n, self.r, self.p, self.dklen
        )
    }
}

/// Scrypt parameter presets for different use cases.
pub struct ScryptPresets;

impl ScryptPresets {
    /// Interactive parameters (fast, for real-time use).
    pub fn interactive() -> ScryptParameters {
        ScryptParameters {
            n: 32768, // 2^SECONDS_PER_BLOCK
            r: 8,
            p: 1,
            dklen: Some(64),
        }
    }

    /// Sensitive parameters (balanced security/performance).
    pub fn sensitive() -> ScryptParameters {
        ScryptParameters::default_nep6()
    }

    /// Paranoid parameters (maximum security).
    pub fn paranoid() -> ScryptParameters {
        ScryptParameters {
            n: MAX_BLOCK_SIZE as u32, // 2^ADDRESS_SIZE
            r: 8,
            p: 8,
            dklen: Some(64),
        }
    }

    /// Test parameters (very fast, for testing only).
    pub fn test() -> ScryptParameters {
        ScryptParameters {
            n: 16,
            r: 1,
            p: 1,
            dklen: Some(64),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::ScryptParameters;

    #[test]
    fn test_scrypt_parameters_validation() {
        assert!(ScryptParameters::new(16384, 8, 8).is_ok());
        assert!(ScryptParameters::new(1, 8, 8).is_err()); // N must be > 1
        assert!(ScryptParameters::new(16383, 8, 8).is_err()); // N must be power of 2
        assert!(ScryptParameters::new(16384, 0, 8).is_err()); // R must be > 0
        assert!(ScryptParameters::new(16384, 8, 0).is_err()); // P must be > 0
    }

    #[test]
    fn test_default_parameters() {
        let nep2 = ScryptParameters::default_nep2();
        assert!(nep2.validate().is_ok());
        assert!(nep2.is_secure());

        let nep6 = ScryptParameters::default_nep6();
        assert!(nep6.validate().is_ok());
        assert!(nep6.is_secure());
    }

    #[test]
    fn test_log_n() {
        let params = ScryptParameters::new(16384, 8, 8).unwrap();
        assert_eq!(params.log_n(), 14);

        let from_log = ScryptParameters::from_log_n(14, 8, 8).unwrap();
        assert_eq!(params.n, from_log.n);
    }

    #[test]
    fn test_memory_usage() {
        let params = ScryptParameters::new(16384, 8, 8).unwrap();
        let memory = params.memory_usage();
        assert_eq!(memory, 128 * 8 * 16384);
    }

    #[test]
    fn test_presets() {
        assert!(ScryptPresets::interactive().validate().is_ok());
        assert!(ScryptPresets::sensitive().validate().is_ok());
        assert!(ScryptPresets::paranoid().validate().is_ok());
        assert!(ScryptPresets::test().validate().is_ok());
    }

    #[test]
    fn test_serialization() {
        let params = ScryptParameters::default_nep6();
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: ScryptParameters =
            serde_json::from_str(&json).expect("Failed to parse from string");
        assert_eq!(params, deserialized);
    }
}

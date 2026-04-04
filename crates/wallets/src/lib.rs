//! Neo Wallets Library
//!
//! This crate provides wallet functionality for the Neo blockchain, including:
//! - Wallet creation and management
//! - Account management
//! - NEP-6 wallet standard implementation
//! - Key management and encryption
//! - Transaction signing
//!
//! This implementation is converted from the C# Neo implementation (@neo-sharp/)
//! to ensure exact compatibility with the Neo N3 protocol.

pub mod contract;
pub mod key_pair;
pub mod nep6;
pub mod scrypt_parameters;
pub mod wallet;
pub mod wallet_account;
pub mod wallet_factory;

// Re-export main types
pub use contract::Contract;
pub use key_pair::KeyPair;
pub use nep6::{Nep6Account, Nep6Contract, Nep6Wallet};
pub use scrypt_parameters::ScryptParameters;
pub use wallet::{Wallet, WalletError, WalletResult};
pub use wallet_account::{StandardWalletAccount, WalletAccount};
pub use wallet_factory::{IWalletFactory, WalletFactory};

use neo_core::{UInt160, UInt256};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Result type for wallet operations
pub type Result<T> = std::result::Result<T, Error>;

/// Wallet-related errors
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid password")]
    InvalidPassword,

    #[error("Account not found: {0}")]
    AccountNotFound(UInt160),

    #[error("Wallet file not found: {0}")]
    WalletFileNotFound(String),

    #[error("Invalid wallet format")]
    InvalidWalletFormat,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid private key")]
    InvalidPrivateKey,

    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Invalid WIF format")]
    InvalidWif,

    #[error("Invalid NEP-2 key")]
    InvalidNep2Key,

    #[error("Wallet is locked")]
    WalletLocked,

    #[error("Account is locked")]
    AccountLocked,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cryptography error: {0}")]
    Cryptography(#[from] neo_cryptography::Error),

    #[error("Core error: {0}")]
    Core(#[from] neo_core::CoreError),

    #[error("Scrypt error: {0}")]
    Scrypt(String),

    #[error("AES error: {0}")]
    Aes(String),

    #[error("Base58 decode error: {0}")]
    Base58Decode(String),

    #[error("Hex decode error: {0}")]
    HexDecode(#[from] hex::FromHexError),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("ECC error: {0}")]
    ECC(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<Error> for crate::wallet::WalletError {
    fn from(err: Error) -> Self {
        crate::wallet::WalletError::Other(err.to_string())
    }
}

impl From<neo_cryptography::ecc::ECCError> for Error {
    fn from(err: neo_cryptography::ecc::ECCError) -> Self {
        Error::ECC(err.to_string())
    }
}

/// Contract parameter types for smart contracts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractParameterType {
    Any = 0x00,
    Boolean = 0x10,
    Integer = 0x11,
    ByteArray = 0x12,
    String = 0x13,
    Hash160 = 0x14,
    Hash256 = 0x15,
    PublicKey = 0x16,
    Signature = 0x17,
    Array = 0x20,
    Map = 0x22,
    InteropInterface = 0x30,
    Void = 0xff,
}

impl fmt::Display for ContractParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractParameterType::Any => write!(f, "Any"),
            ContractParameterType::Boolean => write!(f, "Boolean"),
            ContractParameterType::Integer => write!(f, "Integer"),
            ContractParameterType::ByteArray => write!(f, "ByteArray"),
            ContractParameterType::String => write!(f, "String"),
            ContractParameterType::Hash160 => write!(f, "Hash160"),
            ContractParameterType::Hash256 => write!(f, "Hash256"),
            ContractParameterType::PublicKey => write!(f, "PublicKey"),
            ContractParameterType::Signature => write!(f, "Signature"),
            ContractParameterType::Array => write!(f, "Array"),
            ContractParameterType::Map => write!(f, "Map"),
            ContractParameterType::InteropInterface => write!(f, "InteropInterface"),
            ContractParameterType::Void => write!(f, "Void"),
        }
    }
}

impl TryFrom<u8> for ContractParameterType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(ContractParameterType::Any),
            0x10 => Ok(ContractParameterType::Boolean),
            0x11 => Ok(ContractParameterType::Integer),
            0x12 => Ok(ContractParameterType::ByteArray),
            0x13 => Ok(ContractParameterType::String),
            0x14 => Ok(ContractParameterType::Hash160),
            0x15 => Ok(ContractParameterType::Hash256),
            0x16 => Ok(ContractParameterType::PublicKey),
            0x17 => Ok(ContractParameterType::Signature),
            0x20 => Ok(ContractParameterType::Array),
            0x22 => Ok(ContractParameterType::Map),
            0x30 => Ok(ContractParameterType::InteropInterface),
            0xff => Ok(ContractParameterType::Void),
            _ => Err(Error::Other(format!(
                "Invalid contract parameter type: {}",
                value
            ))),
        }
    }
}

/// Version information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(version_str: &str) -> Result<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return Err(Error::Other("Invalid version format".to_string()));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| Error::Other("Invalid major version".to_string()))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| Error::Other("Invalid minor version".to_string()))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| Error::Other("Invalid patch version".to_string()))?;

        Ok(Self::new(major, minor, patch))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new(0, 1, 0)
    }
}

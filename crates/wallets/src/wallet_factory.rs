//! Wallet factory implementation.
//!
//! This module provides wallet factory functionality for creating and opening wallets,
//! converted from the C# Neo WalletFactory classes (@neo-sharp/src/Neo/Wallets/).

use crate::{
    wallet::{Wallet, WalletResult},
    Error, Result,
};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use neo_config::MAX_SCRIPT_SIZE;
use std::path::Path;

/// Trait for wallet factories.
/// This matches the C# IWalletFactory interface.
#[async_trait]
pub trait IWalletFactory: Send + Sync {
    /// Gets the file extension supported by this factory.
    fn file_extension(&self) -> &'static str;

    /// Checks if this factory can handle the specified file.
    fn can_handle(&self, path: &str) -> bool {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case(self.file_extension()))
            .unwrap_or(false)
    }

    /// Creates a new wallet.
    async fn create_wallet(
        &self,
        name: &str,
        path: &str,
        password: &str,
    ) -> WalletResult<Box<dyn Wallet>>;

    /// Opens an existing wallet.
    async fn open_wallet(&self, path: &str, password: &str) -> WalletResult<Box<dyn Wallet>>;

    /// Gets the factory name.
    fn name(&self) -> &'static str;

    /// Gets the factory description.
    fn description(&self) -> &'static str;
}

/// Base wallet factory implementation.
/// This provides common functionality for wallet factories.
pub struct WalletFactory;

impl WalletFactory {
    /// Validates a wallet path.
    pub fn validate_path(path: &str) -> Result<()> {
        if path.is_empty() {
            return Err(Error::Other("Path cannot be empty".to_string()));
        }

        let path_obj = Path::new(path);

        if let Some(parent) = path_obj.parent() {
            if !parent.exists() {
                return Err(Error::Other(format!(
                    "Directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        Ok(())
    }

    /// Validates a wallet name.
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(Error::Other("Name cannot be empty".to_string()));
        }

        if name.len() > 255 {
            return Err(Error::Other("Name is too long".to_string()));
        }

        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(Error::Other("Name contains invalid characters".to_string()));
        }

        Ok(())
    }

    /// Validates a password.
    pub fn validate_password(password: &str) -> Result<()> {
        if password.is_empty() {
            return Err(Error::Other("Password cannot be empty".to_string()));
        }

        if password.len() < 8 {
            return Err(Error::Other(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        // Password should have reasonable length limit (not script size limit)
        if password.len() > 256 {
            return Err(Error::Other("Password is too long".to_string()));
        }

        Ok(())
    }

    /// Checks if a file exists.
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Gets the file size.
    pub fn get_file_size(path: &str) -> Result<u64> {
        let metadata = std::fs::metadata(path).map_err(|e| Error::Io(e))?;
        Ok(metadata.len())
    }

    /// Creates a backup of a wallet file.
    pub fn create_backup(path: &str) -> Result<String> {
        let backup_path = format!("{}.backup", path);
        std::fs::copy(path, &backup_path).map_err(|e| Error::Io(e))?;
        Ok(backup_path)
    }

    /// Generates a unique file name.
    pub fn generate_unique_filename(base_path: &str, extension: &str) -> String {
        let mut counter = 1;
        loop {
            let filename = if counter == 1 {
                format!("{}.{}", base_path, extension)
            } else {
                format!("{}_{}.{}", base_path, counter, extension)
            };

            if !Path::new(&filename).exists() {
                return filename;
            }

            counter += 1;
        }
    }

    /// Securely deletes a file.
    pub fn secure_delete(path: &str) -> Result<()> {
        if Path::new(path).exists() {
            // 1. Get file size for secure overwriting
            let file_size = std::fs::metadata(path).map_err(|e| Error::Io(e))?.len();

            // 2. Overwrite file with random data multiple times for security
            use rand::RngCore;
            let mut rng = rand::thread_rng();

            for pass in 0..3 {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(false)
                    .open(path)
                    .map_err(|e| Error::Io(e))?;

                // Generate random data
                let mut random_data = vec![0u8; file_size as usize];
                rng.fill_bytes(&mut random_data);

                // Overwrite file with random data
                use std::io::Write;
                file.write_all(&random_data).map_err(|e| Error::Io(e))?;
                file.flush().map_err(|e| Error::Io(e))?;

                log::info!("Secure deletion pass {} completed for {}", pass + 1, path);
            }

            // 3. Finally delete the file
            std::fs::remove_file(path).map_err(|e| Error::Io(e))?;
            log::info!("Wallet file {} securely deleted", path);
        }
        Ok(())
    }

    /// Gets the wallet type from file extension.
    pub fn get_wallet_type(path: &str) -> Option<&'static str> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext.to_lowercase().as_str() {
                "json" => "NEP-6",
                "db3" => "NEP-6 (SQLite)",
                "wallet" => "Legacy",
                _ => "Unknown",
            })
    }

    /// Validates wallet file format.
    pub fn validate_wallet_file(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            return Err(Error::WalletFileNotFound(path.to_string()));
        }

        let file_size = Self::get_file_size(path)?;
        if file_size == 0 {
            return Err(Error::InvalidWalletFormat);
        }

        if file_size > 100 * (MAX_SCRIPT_SIZE as u64) * (MAX_SCRIPT_SIZE as u64) {
            // 100MB limit
            return Err(Error::Other("Wallet file is too large".to_string()));
        }

        Ok(())
    }

    /// Gets wallet information without opening it.
    pub fn get_wallet_info(path: &str) -> Result<WalletInfo> {
        Self::validate_wallet_file(path)?;

        let file_size = Self::get_file_size(path)?;
        let wallet_type = Self::get_wallet_type(path).unwrap_or("Unknown");

        let metadata = std::fs::metadata(path).map_err(|e| Error::Io(e))?;

        Ok(WalletInfo {
            path: path.to_string(),
            wallet_type: wallet_type.to_string(),
            file_size,
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
        })
    }
}

/// Information about a wallet file.
#[derive(Debug, Clone)]
pub struct WalletInfo {
    pub path: String,
    pub wallet_type: String,
    pub file_size: u64,
    pub created: Option<std::time::SystemTime>,
    pub modified: Option<std::time::SystemTime>,
}

impl std::fmt::Display for WalletInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wallet: {} (Type: {}, Size: {} bytes)",
            self.path, self.wallet_type, self.file_size
        )
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::WalletFactory;

    #[test]
    fn test_validate_name() {
        assert!(WalletFactory::validate_name("valid_name").is_ok());
        assert!(WalletFactory::validate_name("").is_err());
        assert!(WalletFactory::validate_name("name/with/slash").is_err());
        assert!(WalletFactory::validate_name(&"a".repeat(300)).is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(WalletFactory::validate_password("validpassword123").is_ok());
        assert!(WalletFactory::validate_password("").is_err());
        assert!(WalletFactory::validate_password("short").is_err());
        assert!(WalletFactory::validate_password(&"a".repeat(2000)).is_err());
    }

    #[test]
    fn test_get_wallet_type() {
        assert_eq!(WalletFactory::get_wallet_type("wallet.json"), Some("NEP-6"));
        assert_eq!(
            WalletFactory::get_wallet_type("wallet.db3"),
            Some("NEP-6 (SQLite)")
        );
        assert_eq!(
            WalletFactory::get_wallet_type("wallet.wallet"),
            Some("Legacy")
        );
        assert_eq!(
            WalletFactory::get_wallet_type("wallet.unknown"),
            Some("Unknown")
        );
    }

    #[test]
    fn test_generate_unique_filename() {
        let filename = WalletFactory::generate_unique_filename("/tmp/test", "json");
        assert!(filename.ends_with(".json"));
        assert!(filename.contains("/tmp/test"));
    }
}

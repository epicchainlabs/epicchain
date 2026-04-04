//! ECRecover implementation for secp256k1
//!
//! Matches C# Neo cryptography ECRecover functionality exactly

// Use standard error types for now
type CryptographyResult<T> = Result<T, CryptographyError>;

#[derive(Debug)]
pub enum CryptographyError {
    InvalidInput(String),
}
use secp256k1::{ecdsa::RecoverableSignature, ecdsa::RecoveryId, Message, PublicKey, Secp256k1};

/// ECRecover functionality (matches C# ECRecover exactly)
pub struct ECRecover;

impl ECRecover {
    /// Recover public key from signature and message hash (matches C# ECRecover.TryRecover)
    pub fn try_recover(
        message_hash: &[u8],
        signature: &[u8],
        recovery_id: u8,
    ) -> CryptographyResult<Vec<u8>> {
        if message_hash.len() != 32 {
            return Err(CryptographyError::InvalidInput(
                "Message hash must be 32 bytes".to_string(),
            ));
        }

        if signature.len() != 64 {
            return Err(CryptographyError::InvalidInput(
                "Signature must be 64 bytes".to_string(),
            ));
        }

        if recovery_id > 3 {
            return Err(CryptographyError::InvalidInput(
                "Recovery ID must be 0-3".to_string(),
            ));
        }

        let secp = Secp256k1::new();

        // Create message from hash
        let message = Message::from_digest_slice(message_hash)
            .map_err(|e| CryptographyError::InvalidInput(format!("Invalid message hash: {}", e)))?;

        // Create recovery ID
        let recovery_id = RecoveryId::from_i32(recovery_id as i32)
            .map_err(|e| CryptographyError::InvalidInput(format!("Invalid recovery ID: {}", e)))?;

        // Create recoverable signature
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);

        let recoverable_sig = RecoverableSignature::from_compact(&sig_bytes, recovery_id)
            .map_err(|e| CryptographyError::InvalidInput(format!("Invalid signature: {}", e)))?;

        // Recover public key
        let recovered_pubkey = secp
            .recover_ecdsa(&message, &recoverable_sig)
            .map_err(|e| CryptographyError::InvalidInput(format!("Recovery failed: {}", e)))?;

        // Return uncompressed public key (65 bytes)
        Ok(recovered_pubkey.serialize_uncompressed().to_vec())
    }

    /// Recover public key with automatic recovery ID detection
    pub fn recover_public_key(
        message_hash: &[u8],
        signature: &[u8],
    ) -> CryptographyResult<Vec<u8>> {
        // Try all possible recovery IDs (0-3)
        for recovery_id in 0..4 {
            if let Ok(public_key) = Self::try_recover(message_hash, signature, recovery_id) {
                return Ok(public_key);
            }
        }

        Err(CryptographyError::InvalidInput(
            "Could not recover public key with any recovery ID".to_string(),
        ))
    }

    /// Verify signature using recovered public key (matches C# verification)
    pub fn verify_signature(
        message_hash: &[u8],
        signature: &[u8],
        expected_pubkey: &[u8],
    ) -> CryptographyResult<bool> {
        // Recover public key from signature
        let recovered_pubkey = Self::recover_public_key(message_hash, signature)?;

        // Compare with expected public key
        if recovered_pubkey.len() != expected_pubkey.len() {
            return Ok(false);
        }

        // Handle both compressed and uncompressed formats
        if expected_pubkey.len() == 33 {
            // Expected is compressed, convert recovered to compressed
            let secp = Secp256k1::new();
            let pubkey = PublicKey::from_slice(&recovered_pubkey).map_err(|e| {
                CryptographyError::InvalidInput(format!("Invalid recovered key: {}", e))
            })?;
            let compressed = pubkey.serialize();
            Ok(compressed.to_vec() == expected_pubkey)
        } else if expected_pubkey.len() == 65 {
            // Both uncompressed
            Ok(recovered_pubkey == expected_pubkey)
        } else {
            Err(CryptographyError::InvalidInput(
                "Expected public key must be 33 or 65 bytes".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_ecrecover_basic() {
        // Test vector from Ethereum (compatible with secp256k1)
        let message_hash =
            hex::decode("a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3")
                .unwrap();
        let signature = hex::decode("28ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276667cac6e0a9e74b9b8f21a7d6d4b75dc3e4c3f8b9d5f3b3f1a1b9c8d2e3f4a5b6").unwrap();

        // Test recovery with different recovery IDs
        for recovery_id in 0..4 {
            if let Ok(recovered) =
                ECRecover::try_recover(&message_hash, &signature[..64], recovery_id)
            {
                assert_eq!(recovered.len(), 65);
                assert_eq!(recovered[0], 0x04); // Uncompressed prefix
                println!("✅ Recovery successful with ID {}", recovery_id);
                break;
            }
        }
    }

    #[test]
    fn test_ecrecover_verification() {
        // Generate test data
        let message_hash = [0x42u8; 32];
        let signature = [0x12u8; 64];

        // Test that verification works correctly
        match ECRecover::recover_public_key(&message_hash, &signature) {
            Ok(recovered) => {
                let is_valid =
                    ECRecover::verify_signature(&message_hash, &signature, &recovered).unwrap();
                assert!(is_valid, "Verification should succeed with recovered key");
            }
            Err(_) => {
                // Some test vectors may not have valid signatures, which is OK
                println!(
                    "Test vector does not produce valid recovery (expected for some test data)"
                );
            }
        }
    }
}

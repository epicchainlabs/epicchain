//! Ed25519 implementation for Neo.
//!
//! This module provides Ed25519 signature functionality.

use crate::Error;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use neo_config::HASH_SIZE;
use rand_core::OsRng;
use std::convert::TryFrom;

/// Provides Ed25519 signature functionality.
pub struct Ed25519;

impl Ed25519 {
    /// Generates a new Ed25519 key pair.
    ///
    /// # Returns
    ///
    /// A tuple containing the private key and public key
    pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);

        (
            keypair.secret.as_bytes().to_vec(),
            keypair.public.as_bytes().to_vec(),
        )
    }

    /// Derives a public key from a private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key
    ///
    /// # Returns
    ///
    /// The derived public key or an error
    pub fn private_key_to_public_key(private_key: &[u8]) -> Result<Vec<u8>, Error> {
        if private_key.len() != HASH_SIZE {
            return Err(Error::InvalidKey("Invalid private key length".to_string()));
        }

        let secret = SecretKey::from_bytes(private_key)
            .map_err(|e| Error::InvalidKey(format!("Invalid private key: {e}")))?;

        let public = PublicKey::from(&secret);

        Ok(public.as_bytes().to_vec())
    }

    /// Signs a message using Ed25519.
    /// Parameters are in the same order as the C# Ed25519.Sign implementation:
    /// Ed25519.Sign(privateKey, message)
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key to sign with
    /// * `message` - The message to sign
    ///
    /// # Returns
    ///
    /// The signature or an error
    pub fn sign(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>, Error> {
        if private_key.len() != HASH_SIZE {
            return Err(Error::InvalidKey("Invalid private key length".to_string()));
        }

        let secret = SecretKey::from_bytes(private_key)
            .map_err(|e| Error::InvalidKey(format!("Invalid private key: {e}")))?;

        let public = PublicKey::from(&secret);
        let keypair = Keypair { secret, public };

        let signature = keypair.sign(message);

        Ok(signature.to_bytes().to_vec())
    }

    /// Verifies an Ed25519 signature.
    /// Parameters are in the same order as the C# Ed25519.Verify implementation:
    /// Ed25519.Verify(publicKey, message, signature)
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key to verify against
    /// * `message` - The message that was signed
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    ///
    /// `true` if the signature is valid, `false` otherwise
    pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 || public_key.len() != HASH_SIZE {
            return false;
        }

        let public_key = match PublicKey::from_bytes(public_key) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        let signature = match Signature::try_from(signature) {
            Ok(s) => s,
            Err(_) => return false,
        };

        public_key.verify(message, &signature).is_ok()
    }
}

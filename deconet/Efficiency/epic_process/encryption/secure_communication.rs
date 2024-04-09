use aes::Aes256;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockCipher, NewBlockCipher};
use rand::Rng;
use std::error::Error;

/// Encrypts a message using AES-256 in CBC mode.
pub fn encrypt_message(message: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; 16];
    rng.fill(&mut iv);

    let cipher = Aes256::new(GenericArray::from_slice(key));
    let ciphertext = cipher.encrypt_block(GenericArray::from_slice(message));

    let mut encrypted_message = iv.to_vec();
    encrypted_message.extend_from_slice(&ciphertext);
    Ok(encrypted_message)
}

/// Decrypts a message using AES-256 in CBC mode.
pub fn decrypt_message(encrypted_message: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let iv = &encrypted_message[..16];
    let ciphertext = &encrypted_message[16..];

    let cipher = Aes256::new(GenericArray::from_slice(key));
    let decrypted_message = cipher.decrypt_block(GenericArray::from_slice(ciphertext));

    Ok(decrypted_message.to_vec())
}

/// Generates a random AES-256 key.
pub fn generate_aes_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    key
}

/// Generates a random initialization vector (IV) for AES encryption.
pub fn generate_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);
    iv
}

use sha2::{Digest, Sha256};

/// Computes the SHA-256 hash of a message.
pub fn hash_message(message: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(message);
    hasher.finalize().to_vec()
}

/// Encodes a message to Base64.
pub fn encode_base64(message: &[u8]) -> String {
    base64::encode(message)
}

/// Decodes a Base64-encoded message.
pub fn decode_base64(encoded_message: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(encoded_message)
}

/// Pads a message to ensure it is a multiple of the AES block size.
pub fn pad_message(message: &[u8]) -> Vec<u8> {
    let mut padded_message = message.to_vec();
    let padding_len = 16 - (message.len() % 16);
    padded_message.extend(vec![padding_len as u8; padding_len]);
    padded_message
}

fn main() {
    let key = generate_aes_key();
    let message = b"Hello, world!";
    let encrypted_message = encrypt_message(message, &key).unwrap();
    println!("Encrypted message: {:?}", encrypted_message);

    let decrypted_message = decrypt_message(&encrypted_message, &key).unwrap();
    println!("Decrypted message: {:?}", decrypted_message);
}

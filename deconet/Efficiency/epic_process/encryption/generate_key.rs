use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::error::Error;

/// Generates a symmetric key for encryption/decryption using ChaCha20.
pub fn generate_key(key_length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    if key_length % 8 != 0 {
        return Err("Key length must be a multiple of 8".into());
    }

    let mut rng = ChaCha20Rng::from_entropy();
    let key: Vec<u8> = (0..key_length).map(|_| rng.gen::<u8>()).collect();
    Ok(key)
}

/// Generates a hexadecimal representation of the key.
pub fn generate_hex_key(key_length: usize) -> Result<String, Box<dyn Error>> {
    let key = generate_key(key_length)?;
    let hex_key: String = key.iter().map(|&b| format!("{:02X}", b)).collect();
    Ok(hex_key)
}

fn main() {
    let key_length = 32; // Key length in bytes (ChaCha20 requires 256-bit keys)
    match generate_hex_key(key_length) {
        Ok(hex_key) => println!("Generated hex key: {}", hex_key),
        Err(e) => eprintln!("Error: {}", e),
    }
}

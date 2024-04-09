use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::{self, Rng};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Loads a symmetric key from a file.
pub fn load_key_from_file(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut key = Vec::new();
    file.read_to_end(&mut key)?;
    Ok(key)
}

/// Saves a symmetric key to a file.
pub fn save_key_to_file(key: &[u8], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(key)?;
    Ok(())
}

/// Generates a new symmetric key.
pub fn generate_key(key_length: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key: Vec<u8> = (0..key_length).map(|_| rand::random::<u8>()).collect();
    Ok(key)
}

/// Derives a symmetric key from a password using Argon2id.
pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536, // 64 MB of memory
        time_cost: 10,    // 10 iterations
        lanes: 4,         // Use 4 lanes
        thread_mode: ThreadMode::Sequential,
        secret: &[],
        ad: &[],
        hash_length: 32,  // Output a 256-bit key
    };

    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config)?;
    Ok(hash.as_bytes().to_vec())
}

/// Rotates a symmetric key by deriving a new key from the old key and a new password.
pub fn rotate_key(old_key: &[u8], new_password: &str, salt: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let old_key_str = std::str::from_utf8(old_key)?;
    let new_key = derive_key_from_password(new_password, salt)?;
    let combined_key: Vec<u8> = old_key_str.bytes().zip(new_key).map(|(a, b)| a ^ b).collect();
    Ok(combined_key)
}

/// Stretches a symmetric key to a specified length using a key derivation function.
pub fn stretch_key(key: &[u8], target_length: usize) -> Vec<u8> {
    let mut stretched_key = key.to_vec();
    while stretched_key.len() < target_length {
        let additional_bytes = argon2::hash_encoded(&stretched_key, b"", &Config::default())
            .unwrap_or_else(|_| String::new())
            .as_bytes()
            .to_vec();
        stretched_key.extend_from_slice(&additional_bytes);
    }
    stretched_key.truncate(target_length);
    stretched_key
}

fn main() {
    let password = "my_password";
    let salt = b"mysalt";
    let key = derive_key_from_password(password, salt).unwrap();
    println!("Derived key: {:?}", key);

    let new_password = "my_new_password";
    let rotated_key = rotate_key(&key, new_password, salt).unwrap();
    println!("Rotated key: {:?}", rotated_key);

    let stretched_key = stretch_key(&key, 64);
    println!("Stretched key: {:?}", stretched_key);
}

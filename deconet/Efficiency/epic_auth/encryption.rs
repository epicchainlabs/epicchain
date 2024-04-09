use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{Rng, thread_rng};
use std::convert::TryInto;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// Generate a random 128-bit key
fn generate_key() -> [u8; 16] {
    let mut rng = thread_rng();
    let mut key = [0u8; 16];
    rng.fill(&mut key);
    key
}

// Encrypt data using AES-128 CBC mode
fn encrypt_data(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let iv = [0u8; 16]; // Initialization vector (IV) is set to all zeros for simplicity
    let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data);
    ciphertext
}

// Decrypt data using AES-128 CBC mode
fn decrypt_data(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let iv = [0u8; 16]; // Initialization vector (IV) is set to all zeros for simplicity
    let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();
    let plaintext = cipher.decrypt_vec(data).unwrap();
    plaintext
}

fn main() {
    let key = generate_key();
    let plaintext = b"Hello, world!";
    let ciphertext = encrypt_data(plaintext, &key);

    println!("Plaintext: {:?}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);

    let decrypted = decrypt_data(&ciphertext, &key);
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_key();
        let plaintext = b"Testing encryption and decryption";
        let ciphertext = encrypt_data(plaintext, &key);
        let decrypted = decrypt_data(&ciphertext, &key);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_generate_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        assert_ne!(key1, key2);
    }
}

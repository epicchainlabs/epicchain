//! Neo MPT Trie Library - Rust implementation of Neo.Cryptography.MPTTrie
//!
//! This crate provides a complete implementation of the Neo Merkle Patricia Trie,
//! matching the C# Neo.Cryptography.MPTTrie API exactly for compatibility.

pub mod cache;
pub mod error;
pub mod helper;
pub mod node;
pub mod node_type;
pub mod proof;
pub mod trie;

// Re-export main types
pub use cache::{Cache, CacheStats, MemoryStorage, Storage};
pub use error::{MptError, MptResult};
pub use helper::{common_prefix_length, from_nibbles, to_nibbles};
pub use node::Node;
pub use node_type::NodeType;
pub use proof::{ProofNode, ProofVerifier};
pub use trie::Trie;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::NodeType;
    #[test]
    fn test_basic_trie_creation() {
        // Basic test to ensure the module compiles
        assert_eq!(NodeType::Empty as u8, 0x04);
    }
}

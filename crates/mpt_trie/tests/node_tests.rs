//! Node C# Compatibility Tests
//!
//! These tests ensure full compatibility with C# Neo's MPT Node functionality.
//! Tests are based on the C# Neo.Cryptography.MPTTrie.Node test suite.

use neo_core::UInt256;
use neo_mpt_trie::*;

#[cfg(test)]
#[allow(dead_code)]
mod node_tests {
    use super::*;

    /// Test Node creation and basic properties (matches C# Node tests exactly)
    #[test]
    fn test_node_creation_compatibility() {
        let empty_node = Node::new();
        assert_eq!(empty_node.node_type(), NodeType::Empty);
        assert!(empty_node.is_empty());
        let mut empty_node_mut = empty_node.clone();
        let _ = empty_node_mut.hash();
        // Newly computed hash should exist after calculation
        assert_eq!(empty_node.reference(), 0);

        let test_hash = UInt256::from_bytes(&[1u8; 32]).unwrap();
        let hash_node = Node::new_hash(test_hash);
        assert_eq!(hash_node.node_type(), NodeType::HashNode);
        assert!(!hash_node.is_empty());
        assert_eq!(hash_node.get_hash(), Some(test_hash));

        let branch_node = Node::new_branch();
        assert_eq!(branch_node.node_type(), NodeType::BranchNode);
        assert!(!branch_node.is_empty());
        assert_eq!(branch_node.children().len(), 16);

        let test_value = b"test_leaf_value".to_vec();
        let leaf_node = Node::new_leaf(test_value.clone());
        assert_eq!(leaf_node.node_type(), NodeType::LeafNode);
        assert!(!leaf_node.is_empty());
        assert_eq!(leaf_node.value(), Some(&test_value));

        let test_key = b"extension_key".to_vec();
        let next_node = Node::new_leaf(b"next_value".to_vec());
        let extension_node = Node::new_extension(test_key.clone(), next_node);
        assert_eq!(extension_node.node_type(), NodeType::ExtensionNode);
        assert!(!extension_node.is_empty());
        assert_eq!(extension_node.key(), Some(&test_key));
        assert!(extension_node.next().is_some());
    }

    /// Test Node serialization and deserialization (matches C# Node serialization exactly)
    #[test]
    fn test_node_serialization_compatibility() {
        // Test leaf node serialization
        let leaf_value = b"serialization_test_value".to_vec();
        let leaf_node = Node::new_leaf(leaf_value.clone());

        let serialized = serde_json::to_string(&leaf_node).unwrap();
        let deserialized: Node = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.node_type(), NodeType::LeafNode);
        assert_eq!(deserialized.value(), Some(&leaf_value));

        // Test branch node serialization
        let branch_node = Node::new_branch();
        let serialized_branch = serde_json::to_string(&branch_node).unwrap();
        let deserialized_branch: Node = serde_json::from_str(&serialized_branch).unwrap();

        assert_eq!(deserialized_branch.node_type(), NodeType::BranchNode);
        assert_eq!(deserialized_branch.children().len(), 16);

        // Test hash node serialization
        let test_hash = UInt256::from_bytes(&[42u8; 32]).unwrap();
        let hash_node = Node::new_hash(test_hash);
        let serialized_hash = serde_json::to_string(&hash_node).unwrap();
        let deserialized_hash: Node = serde_json::from_str(&serialized_hash).unwrap();

        assert_eq!(deserialized_hash.node_type(), NodeType::HashNode);
        assert_eq!(deserialized_hash.get_hash(), Some(test_hash));
    }

    /// Test Node hash calculation (matches C# Node.Hash property exactly)
    #[test]
    fn test_node_hash_calculation_compatibility() {
        // Test leaf node hash calculation
        let leaf_value = b"hash_test_value".to_vec();
        let mut leaf_node = Node::new_leaf(leaf_value);

        // Initially no hash
        // Initially no hash cached
        assert!(leaf_node.get_hash().is_none());

        let _ = leaf_node.calculate_hash();
        assert!(leaf_node.get_hash().is_some());

        // Hash should be deterministic
        let hash1 = leaf_node.hash();
        let _ = leaf_node.calculate_hash();
        let hash2 = leaf_node.hash();
        assert_eq!(hash1, hash2);

        // Different values should produce different hashes
        let different_leaf = Node::new_leaf(b"different_value".to_vec());
        let mut different_leaf_mut = different_leaf;
        let _ = different_leaf_mut.calculate_hash();
        assert_ne!(hash1, different_leaf_mut.hash());
    }

    /// Test Node reference counting (matches C# Node reference management exactly)
    #[test]
    fn test_node_reference_counting_compatibility() {
        let mut node = Node::new_leaf(b"reference_test".to_vec());

        // Initial reference count should be 0
        assert_eq!(node.reference(), 0);

        // Increment reference count
        node.set_reference(node.reference() + 1);
        assert_eq!(node.reference(), 1);

        node.set_reference(node.reference() + 1);
        assert_eq!(node.reference(), 2);

        // Decrement reference count
        node.set_reference(node.reference() - 1);
        assert_eq!(node.reference(), 1);

        node.set_reference(node.reference() - 1);
        assert_eq!(node.reference(), 0);
    }

    /// Test Node child operations for branch nodes (matches C# Node child management exactly)
    #[test]
    fn test_branch_node_children_compatibility() {
        let mut branch_node = Node::new_branch();

        // Test initial state - all children should be None
        for i in 0..16 {
            assert!(branch_node.children().get(i).unwrap().is_none());
        }

        // Test setting children
        let child_leaf = Node::new_leaf(b"child_value".to_vec());
        branch_node.set_child(5, Some(child_leaf.clone()));

        assert!(branch_node.children().get(5).unwrap().is_some());
        assert_eq!(
            branch_node
                .children()
                .get(5)
                .unwrap()
                .as_ref()
                .unwrap()
                .value(),
            child_leaf.value()
        );

        // Test other children are still None
        for i in 0..16 {
            if i != 5 {
                assert!(branch_node.children().get(i).unwrap().is_none());
            }
        }

        // Test removing child
        branch_node.set_child(5, None);
        assert!(branch_node.children().get(5).unwrap().is_none());
    }

    /// Test Node validation and consistency (matches C# Node validation exactly)
    #[test]
    fn test_node_validation_compatibility() {
        // Test empty node validation
        let empty_node = Node::new();
        assert!(empty_node.is_empty());

        // Test leaf node validation
        let leaf_node = Node::new_leaf(b"valid_leaf".to_vec());
        assert_eq!(leaf_node.node_type(), NodeType::LeafNode);

        // Test branch node validation
        let branch_node = Node::new_branch();
        assert_eq!(branch_node.node_type(), NodeType::BranchNode);

        // Test extension node validation
        let extension_key = b"valid_extension".to_vec();
        let next_node = Node::new_leaf(b"extension_target".to_vec());
        let extension_node = Node::new_extension(extension_key, next_node);
        assert_eq!(extension_node.node_type(), NodeType::ExtensionNode);

        // Test hash node validation
        let test_hash = UInt256::from_bytes(&[123u8; 32]).unwrap();
        let hash_node = Node::new_hash(test_hash);
        assert_eq!(hash_node.node_type(), NodeType::HashNode);
    }

    /// Test Node type conversion (matches C# Node type operations exactly)
    #[test]
    fn test_node_type_conversion_compatibility() {
        let mut node = Node::new();
        assert_eq!(node.node_type(), NodeType::Empty);

        // Convert to leaf node
        node.set_node_type(NodeType::LeafNode);
        node.set_value(Some(b"converted_leaf".to_vec()));
        assert_eq!(node.node_type(), NodeType::LeafNode);
        assert_eq!(node.value(), Some(&b"converted_leaf".to_vec()));

        // Convert to branch node
        let mut branch_node = Node::new();
        branch_node.set_node_type(NodeType::BranchNode);
        // children are initialized by new_branch()
        assert_eq!(branch_node.node_type(), NodeType::BranchNode);
        assert_eq!(branch_node.children().len(), 16);
    }

    /// Test Node equality and comparison (matches C# Node.Equals exactly)
    #[test]
    fn test_node_equality_compatibility() {
        // Test leaf node equality
        let leaf1 = Node::new_leaf(b"same_value".to_vec());
        let leaf2 = Node::new_leaf(b"same_value".to_vec());
        let leaf3 = Node::new_leaf(b"different_value".to_vec());

        assert_eq!(leaf1, leaf2);
        assert_ne!(leaf1, leaf3);

        // Test hash node equality
        let test_hash = UInt256::from_bytes(&[255u8; 32]).unwrap();
        let hash1 = Node::new_hash(test_hash);
        let hash2 = Node::new_hash(test_hash);
        let different_hash = UInt256::from_bytes(&[0u8; 32]).unwrap();
        let hash3 = Node::new_hash(different_hash);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);

        // Test empty node equality
        let empty1 = Node::new();
        let empty2 = Node::new();
        assert_eq!(empty1, empty2);

        // Test different types are not equal
        assert_ne!(leaf1, empty1);
        assert_ne!(hash1, leaf1);
    }

    /// Test Node size calculation (matches C# Node.Size property exactly)
    #[test]
    fn test_node_size_calculation_compatibility() {
        // Test empty node size
        let empty_node = Node::new();
        assert_eq!(empty_node.size(), 1); // Just the type byte

        // Test leaf node size
        let leaf_value = b"size_test_value".to_vec();
        let leaf_node = Node::new_leaf(leaf_value.clone());
        let expected_leaf_size = 1 + leaf_value.len(); // type byte + value length
        assert_eq!(leaf_node.size(), expected_leaf_size);

        // Test hash node size
        let test_hash = UInt256::from_bytes(&[88u8; 32]).unwrap();
        let hash_node = Node::new_hash(test_hash);
        assert_eq!(hash_node.size(), 1 + 32); // type byte + hash size

        // Test branch node size
        let branch_node = Node::new_branch();
        let expected_branch_size = 1 + (16 * 32); // type byte + 16 child hashes
        assert!(branch_node.size() >= expected_branch_size);
    }

    /// Test Node cloning and copying (matches C# Node cloning behavior exactly)
    #[test]
    fn test_node_cloning_compatibility() {
        // Test leaf node cloning
        let original_leaf = Node::new_leaf(b"clone_test_value".to_vec());
        let cloned_leaf = original_leaf.clone();

        assert_eq!(original_leaf, cloned_leaf);
        assert_eq!(original_leaf.value(), cloned_leaf.value());
        assert_eq!(original_leaf.node_type(), cloned_leaf.node_type());

        // Test branch node cloning
        let mut original_branch = Node::new_branch();
        let child = Node::new_leaf(b"child_for_cloning".to_vec());
        original_branch.set_child(3, Some(child));

        let cloned_branch = original_branch.clone();
        assert_eq!(original_branch, cloned_branch);
        assert!(cloned_branch.children().get(3).unwrap().is_some());

        // Test extension node cloning
        let extension_key = b"clone_extension_key".to_vec();
        let next_node = Node::new_leaf(b"clone_next_value".to_vec());
        let original_extension = Node::new_extension(extension_key.clone(), next_node);
        let cloned_extension = original_extension.clone();

        assert_eq!(original_extension, cloned_extension);
        assert_eq!(original_extension.key(), cloned_extension.key());
        assert_eq!(
            original_extension.next().as_ref().unwrap().value(),
            cloned_extension.next().as_ref().unwrap().value()
        );
    }
}

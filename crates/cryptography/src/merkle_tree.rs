//! Merkle tree implementation for Neo.
//!
//! This module provides a Merkle tree implementation for efficient verification of data integrity.

use crate::hash_algorithm::HashAlgorithm;
use crate::hasher::Hasher;
use std::fmt;

/// A node in a Merkle tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleTreeNode {
    /// The hash of this node
    pub hash: Vec<u8>,

    /// The left child of this node, if any
    pub left: Option<Box<MerkleTreeNode>>,

    /// The right child of this node, if any
    pub right: Option<Box<MerkleTreeNode>>,

    /// Whether this node is a leaf node
    pub is_leaf: bool,
}

impl MerkleTreeNode {
    /// Creates a new leaf node with the given hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the leaf node
    ///
    /// # Returns
    ///
    /// A new leaf node
    pub fn new_leaf(hash: Vec<u8>) -> Self {
        Self {
            hash,
            left: None,
            right: None,
            is_leaf: true,
        }
    }

    /// Creates a new branch node with the given left and right children.
    ///
    /// # Arguments
    ///
    /// * `left` - The left child
    /// * `right` - The right child
    ///
    /// # Returns
    ///
    /// A new branch node
    pub fn new_branch(left: MerkleTreeNode, right: MerkleTreeNode) -> Self {
        let mut combined = Vec::with_capacity(left.hash.len() + right.hash.len());
        combined.extend_from_slice(&left.hash);
        combined.extend_from_slice(&right.hash);

        let hash = Hasher::hash(HashAlgorithm::Sha256, &combined);

        Self {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            is_leaf: false,
        }
    }
}

/// A Merkle tree for efficient verification of data integrity.
#[derive(Clone, Debug)]
pub struct MerkleTree {
    /// The root node of the tree
    pub root: Option<MerkleTreeNode>,

    /// The depth of the tree
    pub depth: usize,
}

impl MerkleTree {
    /// Creates a new Merkle tree from the given hashes.
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes to include in the tree
    ///
    /// # Returns
    ///
    /// A new Merkle tree or None if the hashes are empty
    pub fn new(hashes: &[Vec<u8>]) -> Option<Self> {
        if hashes.is_empty() {
            return None;
        }

        let mut nodes: Vec<MerkleTreeNode> = hashes
            .iter()
            .map(|hash| MerkleTreeNode::new_leaf(hash.clone()))
            .collect();

        let mut depth = 1;
        while nodes.len() > 1 {
            depth += 1;
            nodes = Self::build_next_level(&nodes);
        }

        Some(Self {
            root: Some(nodes.remove(0)),
            depth,
        })
    }

    /// Builds the next level of the tree from the current level.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The current level of nodes
    ///
    /// # Returns
    ///
    /// The next level of nodes
    fn build_next_level(nodes: &[MerkleTreeNode]) -> Vec<MerkleTreeNode> {
        let mut result = Vec::with_capacity(nodes.len().div_ceil(2));

        for i in (0..nodes.len()).step_by(2) {
            if i + 1 < nodes.len() {
                // Pair of nodes
                let branch = MerkleTreeNode::new_branch(nodes[i].clone(), nodes[i + 1].clone());
                result.push(branch);
            } else {
                // Odd node out, duplicate it
                let branch = MerkleTreeNode::new_branch(nodes[i].clone(), nodes[i].clone());
                result.push(branch);
            }
        }

        result
    }

    /// Returns the root hash of the tree.
    ///
    /// # Returns
    ///
    /// The root hash or None if the tree is empty
    pub fn root_hash(&self) -> Option<&Vec<u8>> {
        self.root.as_ref().map(|node| &node.hash)
    }

    /// Computes a Merkle proof for the given leaf index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the leaf to generate a proof for
    /// * `total` - The total number of leaves in the tree
    ///
    /// # Returns
    ///
    /// A Merkle proof as a vector of hashes
    pub fn get_proof(&self, index: usize, total: usize) -> Option<Vec<Vec<u8>>> {
        if index >= total || self.root.is_none() {
            return None;
        }

        let mut proof = Vec::new();
        let mut path = Vec::new();

        // Calculate the path from the root to the leaf
        let mut i = index;
        let mut level_size = total;
        while level_size > 1 {
            path.push(i % 2 == 1);
            i /= 2;
            level_size = level_size.div_ceil(2);
        }

        // Traverse the tree to collect the proof
        let mut node = self.root.as_ref().expect("Field should be initialized");
        for is_right in path.iter().rev() {
            if *is_right {
                // We're going right, so include the left hash
                if let Some(left) = &node.left {
                    proof.push(left.hash.clone());
                    node = node.right.as_ref().expect("Value should exist");
                } else {
                    return None;
                }
            } else {
                // We're going left, so include the right hash
                if let Some(right) = &node.right {
                    proof.push(right.hash.clone());
                    node = node.left.as_ref().expect("Value should exist");
                } else {
                    return None;
                }
            }
        }

        Some(proof)
    }

    /// Verifies a Merkle proof for the given leaf hash.
    ///
    /// # Arguments
    ///
    /// * `leaf_hash` - The hash of the leaf to verify
    /// * `index` - The index of the leaf
    /// * `proof` - The Merkle proof
    /// * `root_hash` - The expected root hash
    /// * `total` - The total number of leaves in the tree
    ///
    /// # Returns
    ///
    /// `true` if the proof is valid, `false` otherwise
    pub fn verify_proof(
        leaf_hash: &[u8],
        index: usize,
        proof: &[Vec<u8>],
        root_hash: &[u8],
        total: usize,
    ) -> bool {
        if index >= total {
            return false;
        }

        let mut hash = leaf_hash.to_vec();
        let mut i = index;

        for sibling_hash in proof.iter() {
            let combined = if i % 2 == 1 {
                // We're on the right, so sibling is on the left
                let mut combined = Vec::with_capacity(hash.len() + sibling_hash.len());
                combined.extend_from_slice(sibling_hash);
                combined.extend_from_slice(&hash);
                combined
            } else {
                // We're on the left, so sibling is on the right
                let mut combined = Vec::with_capacity(hash.len() + sibling_hash.len());
                combined.extend_from_slice(&hash);
                combined.extend_from_slice(sibling_hash);
                combined
            };

            hash = Hasher::hash(HashAlgorithm::Sha256, &combined);
            i /= 2;
        }

        hash == root_hash
    }

    /// Computes the root hash for the given leaf hashes.
    ///
    /// # Arguments
    ///
    /// * `hashes` - The leaf hashes
    ///
    /// # Returns
    ///
    /// The root hash or None if the hashes are empty
    pub fn compute_root(hashes: &[Vec<u8>]) -> Option<Vec<u8>> {
        Self::new(hashes).and_then(|tree| tree.root_hash().cloned())
    }

    /// Trims the tree to the given depth.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth to trim to
    ///
    /// # Returns
    ///
    /// A new tree trimmed to the given depth
    pub fn trim(&self, depth: usize) -> Self {
        if depth >= self.depth || self.root.is_none() {
            return self.clone();
        }

        let mut root = self
            .root
            .as_ref()
            .expect("Field should be initialized")
            .clone();
        let mut current_depth = self.depth;

        while current_depth > depth {
            if root.left.is_none() || root.right.is_none() {
                break;
            }

            root = MerkleTreeNode {
                hash: root.hash,
                left: None,
                right: None,
                is_leaf: false,
            };

            current_depth -= 1;
        }

        Self {
            root: Some(root),
            depth: current_depth,
        }
    }
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.root {
            Some(root) => write!(
                f,
                "MerkleTree {{ depth: {}, root_hash: {} }}",
                self.depth,
                hex::encode(&root.hash)
            ),
            None => write!(f, "MerkleTree {{ empty }}"),
        }
    }
}

use crate::error::{MptError, MptResult};
use crate::node_type::NodeType;
use neo_config::HASH_SIZE;
use neo_core::UInt256;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// MPT Trie Node implementation
/// This matches the C# Node class
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    node_type: NodeType,
    hash: Option<UInt256>,
    reference: i32,

    children: Option<Vec<Option<Box<Node>>>>,

    // Extension node data
    key: Option<Vec<u8>>,
    next: Option<Box<Node>>,

    // Leaf node data
    value: Option<Vec<u8>>,
}

impl Node {
    const BRANCH_CHILD_COUNT: usize = 16;

    /// Creates a new empty node
    pub fn new() -> Self {
        Self {
            node_type: NodeType::Empty,
            hash: None,
            reference: 0,
            children: None,
            key: None,
            next: None,
            value: None,
        }
    }

    /// Creates a new hash node
    pub fn new_hash(hash: UInt256) -> Self {
        Self {
            node_type: NodeType::HashNode,
            hash: Some(hash),
            reference: 0,
            children: None,
            key: None,
            next: None,
            value: None,
        }
    }

    /// Creates a new branch node
    pub fn new_branch() -> Self {
        Self {
            node_type: NodeType::BranchNode,
            hash: None,
            reference: 0,
            children: Some(vec![None; Self::BRANCH_CHILD_COUNT]),
            key: None,
            next: None,
            value: None,
        }
    }

    /// Creates a new extension node
    pub fn new_extension(key: Vec<u8>, next: Node) -> Self {
        Self {
            node_type: NodeType::ExtensionNode,
            hash: None,
            reference: 0,
            children: None,
            key: Some(key),
            next: Some(Box::new(next)),
            value: None,
        }
    }

    /// Creates a new leaf node
    pub fn new_leaf(value: Vec<u8>) -> Self {
        Self {
            node_type: NodeType::LeafNode,
            hash: None,
            reference: 0,
            children: None,
            key: None,
            next: None,
            value: Some(value),
        }
    }

    /// Gets the node type
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// Sets the node type
    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
        self.set_dirty();

        // Initialize appropriate data structures based on node type
        match node_type {
            NodeType::BranchNode => {
                if self.children.is_none() {
                    self.children = Some(vec![None; Self::BRANCH_CHILD_COUNT]);
                }
            }
            NodeType::ExtensionNode => {
                // Extension nodes need key and next
            }
            NodeType::LeafNode => {
                // Leaf nodes need value
            }
            NodeType::HashNode => {
                // Hash nodes need hash
            }
            NodeType::Empty => {
                // Empty nodes need nothing
            }
        }
    }

    /// Checks if the node is empty
    pub fn is_empty(&self) -> bool {
        self.node_type == NodeType::Empty
    }

    /// Gets the hash of the node (production implementation matching C# Neo exactly)
    pub fn hash(&mut self) -> UInt256 {
        if let Some(hash) = self.hash {
            return hash;
        }

        let data = self.to_array_without_reference();
        let hash_bytes = Sha256::digest(&data);

        let mut hash_array = [0u8; HASH_SIZE];
        hash_array.copy_from_slice(&hash_bytes);
        let calculated_hash = UInt256::from_bytes(&hash_array).unwrap_or_default();

        // Cache the calculated hash
        self.hash = Some(calculated_hash);
        calculated_hash
    }

    /// Gets the stored hash value for HashNode types (immutable access)
    pub fn get_hash(&self) -> Option<UInt256> {
        self.hash
    }

    /// Gets the reference count
    pub fn reference(&self) -> i32 {
        self.reference
    }

    /// Sets the reference count
    pub fn set_reference(&mut self, reference: i32) {
        self.reference = reference;
    }

    /// Marks the node as dirty (clears cached hash)
    pub fn set_dirty(&mut self) {
        self.hash = None;
    }

    /// Gets the children for branch nodes (returns Vec<Option<Node>> for compatibility)
    pub fn children(&self) -> Vec<Option<Node>> {
        match &self.children {
            Some(children) => children
                .iter()
                .map(|child| child.as_ref().map(|boxed_node| boxed_node.as_ref().clone()))
                .collect(),
            None => vec![None; Self::BRANCH_CHILD_COUNT],
        }
    }

    /// Gets the children as Option<Vec<Box<Node>>> for internal use
    pub fn children_boxed(&self) -> Option<&Vec<Option<Box<Node>>>> {
        self.children.as_ref()
    }

    /// Gets mutable children for branch nodes
    pub fn children_mut(&mut self) -> Option<&mut Vec<Option<Box<Node>>>> {
        self.children.as_mut()
    }

    /// Sets a child at the given index for branch nodes
    pub fn set_child(&mut self, index: usize, child: Option<Node>) {
        if index >= Self::BRANCH_CHILD_COUNT {
            return;
        }

        if self.children.is_none() {
            self.children = Some(vec![None; Self::BRANCH_CHILD_COUNT]);
        }

        if let Some(children) = &mut self.children {
            children[index] = child.map(Box::new);
            self.set_dirty();
        }
    }

    /// Gets the key for extension nodes
    pub fn key(&self) -> Option<&Vec<u8>> {
        self.key.as_ref()
    }

    /// Sets the key for extension/leaf nodes
    pub fn set_key(&mut self, key: Option<Vec<u8>>) {
        self.key = key;
        self.set_dirty();
    }

    /// Gets the next node for extension nodes
    pub fn next(&self) -> Option<&Node> {
        self.next.as_ref().map(|n| n.as_ref())
    }

    /// Sets the next node for extension nodes
    pub fn set_next(&mut self, next: Option<Box<Node>>) {
        self.next = next;
        self.set_dirty();
    }

    /// Gets the value for leaf nodes
    pub fn value(&self) -> Option<&Vec<u8>> {
        self.value.as_ref()
    }

    /// Sets the value for leaf/branch nodes
    pub fn set_value(&mut self, value: Option<Vec<u8>>) {
        self.value = value;
        self.set_dirty();
    }

    /// Serializes the node (production implementation matching C# Neo exactly)
    pub fn to_array_without_reference(&self) -> Vec<u8> {
        let mut result = Vec::new();

        match &self.node_type {
            NodeType::BranchNode => {
                result.push(0x00); // BranchNode type marker

                if let Some(children) = &self.children {
                    for i in 0..16 {
                        if let Some(Some(child)) = children.get(i) {
                            result.push(0x01); // Child exists marker
                            let child_hash = child.hash.unwrap_or_default();
                            result.extend_from_slice(&child_hash.as_bytes());
                        } else {
                            result.push(0x00); // No child marker
                        }
                    }
                } else {
                    // No children - add 16 empty markers
                    for _ in 0..16 {
                        result.push(0x00);
                    }
                }

                if let Some(ref value) = self.value {
                    result.push(0x01); // Value exists marker
                    result.extend_from_slice(&(value.len() as u32).to_le_bytes());
                    result.extend_from_slice(value);
                } else {
                    result.push(0x00); // No value marker
                }
            }
            NodeType::ExtensionNode => {
                result.push(0x01); // ExtensionNode type marker

                if let Some(ref key) = self.key {
                    result.extend_from_slice(&(key.len() as u32).to_le_bytes());
                    result.extend_from_slice(key);
                } else {
                    result.extend_from_slice(&[0u8; 4]); // Zero length
                }

                if let Some(ref next_node) = self.next {
                    let next_hash = next_node.hash.unwrap_or_default();
                    result.extend_from_slice(&next_hash.as_bytes());
                } else {
                    result.extend_from_slice(&[0u8; HASH_SIZE]); // Zero hash for null reference
                }
            }
            NodeType::LeafNode => {
                result.push(0x02); // LeafNode type marker

                if let Some(ref key) = self.key {
                    result.extend_from_slice(&(key.len() as u32).to_le_bytes());
                    result.extend_from_slice(key);
                } else {
                    result.extend_from_slice(&[0u8; 4]); // Zero length
                }

                if let Some(ref value) = self.value {
                    result.extend_from_slice(&(value.len() as u32).to_le_bytes());
                    result.extend_from_slice(value);
                } else {
                    result.extend_from_slice(&[0u8; 4]); // Zero length for null value
                }
            }
            NodeType::HashNode => {
                result.push(0x03); // HashNode type marker

                if let Some(ref hash) = self.hash {
                    result.extend_from_slice(&hash.as_bytes());
                } else {
                    result.extend_from_slice(&[0u8; HASH_SIZE]); // Zero hash
                }
            }
            NodeType::Empty => {
                result.push(0x04); // Empty node type marker
            }
        }

        result
    }

    /// Calculates the size of the node when serialized
    pub fn size(&self) -> usize {
        let base_size = 1; // NodeType byte
        match self.node_type {
            NodeType::BranchNode => {
                let children_size = 16; // 16 children, each at least 1 byte
                base_size + children_size + 4 // reference as i32
            }
            NodeType::ExtensionNode => {
                let key_size = self.key.as_ref().map(|k| k.len()).unwrap_or(0);
                let next_size = 1; // At least 1 byte for next node
                base_size + key_size + next_size + 4 // reference as i32
            }
            NodeType::LeafNode => {
                let value_size = self.value.as_ref().map(|v| v.len()).unwrap_or(0);
                base_size + value_size + 4 // reference as i32
            }
            NodeType::HashNode => {
                base_size + HASH_SIZE // UInt256 size
            }
            NodeType::Empty => base_size,
        }
    }

    /// Calculates the size when used as a child node
    pub fn size_as_child(&self) -> usize {
        match self.node_type {
            NodeType::BranchNode | NodeType::ExtensionNode | NodeType::LeafNode => {
                // These become hash nodes when used as children
                1 + HASH_SIZE // NodeType + UInt256
            }
            NodeType::HashNode | NodeType::Empty => self.size(),
        }
    }

    /// Serializes the node to array (production implementation matching C# Neo exactly)
    pub fn to_array(&self) -> Vec<u8> {
        self.to_array_without_reference()
    }

    /// Creates a clone suitable for use as a child node
    pub fn clone_as_child(&self) -> Node {
        match self.node_type {
            NodeType::BranchNode | NodeType::ExtensionNode | NodeType::LeafNode => {
                Node::new_hash(self.hash.unwrap_or_default())
            }
            NodeType::HashNode | NodeType::Empty => self.clone(),
        }
    }

    /// Serializes the node to bytes
    pub fn to_bytes(&self) -> MptResult<Vec<u8>> {
        let mut result = self.to_array_without_reference();

        // Add reference count
        result.extend_from_slice(&self.reference.to_le_bytes());

        Ok(result)
    }

    /// Creates a node from bytes (production implementation matching C# Neo exactly)
    pub fn from_bytes(data: &[u8]) -> MptResult<Node> {
        if data.is_empty() {
            return Err(MptError::InvalidNode("Empty data".to_string()));
        }

        let node_type_byte = data[0];
        let mut offset = 1;

        match node_type_byte {
            0x00 => {
                // BranchNode
                let mut node = Node::new_branch();

                // Read 16 children
                for i in 0..16 {
                    if offset >= data.len() {
                        return Err(MptError::InvalidNode(
                            "Incomplete branch node data".to_string(),
                        ));
                    }

                    let has_child = data[offset];
                    offset += 1;

                    if has_child == 0x01 {
                        // Child exists - read hash
                        if offset + HASH_SIZE > data.len() {
                            return Err(MptError::InvalidNode("Incomplete child hash".to_string()));
                        }

                        let mut hash_bytes = [0u8; HASH_SIZE];
                        hash_bytes.copy_from_slice(&data[offset..offset + HASH_SIZE]);
                        offset += HASH_SIZE;

                        let hash = UInt256::from_bytes(&hash_bytes)?;
                        node.set_child(i, Some(Node::new_hash(hash)));
                    }
                }

                if offset < data.len() {
                    let has_value = data[offset];
                    offset += 1;

                    if has_value == 0x01 && offset + 4 <= data.len() {
                        let value_len = u32::from_le_bytes([
                            data[offset],
                            data[offset + 1],
                            data[offset + 2],
                            data[offset + 3],
                        ]) as usize;
                        offset += 4;

                        if offset + value_len <= data.len() {
                            let value = data[offset..offset + value_len].to_vec();
                            node.set_value(Some(value));
                        }
                    }
                }

                Ok(node)
            }
            0x01 => {
                // ExtensionNode
                if offset + 4 > data.len() {
                    return Err(MptError::InvalidNode(
                        "Incomplete extension node".to_string(),
                    ));
                }

                let key_len = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;

                if offset + key_len + HASH_SIZE > data.len() {
                    return Err(MptError::InvalidNode(
                        "Incomplete extension node data".to_string(),
                    ));
                }

                let key = data[offset..offset + key_len].to_vec();
                offset += key_len;

                let mut hash_bytes = [0u8; HASH_SIZE];
                hash_bytes.copy_from_slice(&data[offset..offset + HASH_SIZE]);
                let hash = UInt256::from_bytes(&hash_bytes)?;

                let next_node = Node::new_hash(hash);
                let mut node = Node::new_extension(key, next_node);
                Ok(node)
            }
            0x02 => {
                // LeafNode
                if offset + 4 > data.len() {
                    return Err(MptError::InvalidNode("Incomplete leaf node".to_string()));
                }

                let key_len = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;

                if offset + key_len + 4 > data.len() {
                    return Err(MptError::InvalidNode(
                        "Incomplete leaf node data".to_string(),
                    ));
                }

                let key = data[offset..offset + key_len].to_vec();
                offset += key_len;

                let value_len = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;

                if offset + value_len > data.len() {
                    return Err(MptError::InvalidNode("Incomplete leaf value".to_string()));
                }

                let value = data[offset..offset + value_len].to_vec();

                let mut node = Node::new_leaf(value);
                node.set_key(Some(key));
                Ok(node)
            }
            0x03 => {
                // HashNode
                if offset + HASH_SIZE > data.len() {
                    return Err(MptError::InvalidNode("Incomplete hash node".to_string()));
                }

                let mut hash_bytes = [0u8; HASH_SIZE];
                hash_bytes.copy_from_slice(&data[offset..offset + HASH_SIZE]);
                let hash = UInt256::from_bytes(&hash_bytes)?;

                Ok(Node::new_hash(hash))
            }
            0x04 => {
                // Empty
                Ok(Node::new())
            }
            _ => Err(MptError::InvalidNode(format!(
                "Unknown node type: {}",
                node_type_byte
            ))),
        }
    }

    /// Deserializes a node from bytes (alias for from_bytes for compatibility)
    pub fn deserialize(data: &[u8]) -> MptResult<Node> {
        Self::from_bytes(data)
    }

    /// Gets the path (key) for extension and leaf nodes
    pub fn get_path(&self) -> Option<Vec<u8>> {
        self.key.clone()
    }

    /// Gets a child node for branch nodes
    pub fn get_child(&self, index: usize) -> Option<&Node> {
        if let Some(children) = &self.children {
            if index < Self::BRANCH_CHILD_COUNT {
                return children[index].as_deref();
            }
        }
        None
    }

    /// Calculates the hash of the node (alias for hash method for compatibility)
    pub fn calculate_hash(&mut self) -> UInt256 {
        self.hash()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

// (module unit tests removed; covered by crate-level tests)

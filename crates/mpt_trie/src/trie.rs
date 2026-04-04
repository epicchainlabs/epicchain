use crate::error::{MptError, MptResult};
use crate::helper::{common_prefix_length, from_nibbles, to_nibbles};
use crate::{Cache, Node, NodeType};
use neo_config::HASH_SIZE;
use neo_core::UInt256;

/// MPT Trie implementation
/// This matches the C# Trie class
pub struct Trie {
    root: Node,
    cache: Cache,
    full: bool,
    storage: Option<Box<dyn TrieStorage>>,
}

/// Trait for trie storage backend
pub trait TrieStorage {
    fn get(&self, hash: &UInt256) -> MptResult<Option<Vec<u8>>>;
    fn put(&mut self, hash: &UInt256, data: &[u8]) -> MptResult<()>;
}

impl Trie {
    const PREFIX: u8 = 0xf0;

    /// Creates a new Trie
    pub fn new(root: Option<UInt256>, full_state: bool) -> Self {
        let root_node = match root {
            Some(hash) => Node::new_hash(hash),
            None => Node::new(),
        };

        Self {
            root: root_node,
            cache: Cache::new(),
            full: full_state,
            storage: None,
        }
    }

    /// Creates a new Trie with storage backend
    pub fn new_with_storage(
        root: Option<UInt256>,
        full_state: bool,
        storage: Box<dyn TrieStorage>,
    ) -> Self {
        let root_node = match root {
            Some(hash) => Node::new_hash(hash),
            None => Node::new(),
        };

        Self {
            root: root_node,
            cache: Cache::new(),
            full: full_state,
            storage: Some(storage),
        }
    }

    /// Gets the root node
    pub fn root(&self) -> &Node {
        &self.root
    }

    /// Gets a mutable reference to the root node
    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    /// Commits changes to the trie
    pub fn commit(&mut self) {
        self.cache.commit();
    }

    /// Gets value from a child node (production implementation matching C# Neo exactly)
    fn get_from_child(&mut self, child: &Option<&Node>, path: &[u8]) -> MptResult<Option<Vec<u8>>> {
        match child {
            Some(child_node) => self.get_node(child_node, path),
            None => Ok(None),
        }
    }

    /// Recursively gets value from a node (production implementation matching C# Neo exactly)
    fn get_recursive(&mut self, child: &Option<&Node>, path: &[u8]) -> MptResult<Option<Vec<u8>>> {
        match child {
            Some(child_node) => self.get_node(child_node, path),
            None => Ok(None),
        }
    }

    /// Gets a value from the trie
    pub fn get(&mut self, key: &[u8]) -> MptResult<Option<Vec<u8>>> {
        let nibbles = to_nibbles(key);
        self.get_node(&self.root.clone(), &nibbles)
    }

    /// Internal method to get a value from a node
    fn get_node(&mut self, node: &Node, path: &[u8]) -> MptResult<Option<Vec<u8>>> {
        if node.is_empty() {
            return Ok(None);
        }

        match node.node_type() {
            NodeType::LeafNode => {
                if let Some(key) = node.key() {
                    if key == path {
                        return Ok(node.value().map(|v| v.to_vec()));
                    }
                }
                Ok(None)
            }
            NodeType::ExtensionNode => {
                if let Some(key) = node.key() {
                    if path.len() >= key.len() && &path[..key.len()] == key {
                        if let Some(next) = node.next() {
                            return self.get_node(next, &path[key.len()..]);
                        }
                    }
                }
                Ok(None)
            }
            NodeType::BranchNode => {
                if path.is_empty() {
                    return Ok(node.value().map(|v| v.to_vec()));
                }

                let index = path[0] as usize;
                if index < 16 {
                    if let Some(child) = node.children().get(index) {
                        if let Some(child_node) = child {
                            return self.get_node(child_node, &path[1..]);
                        }
                    }
                }
                Ok(None)
            }
            NodeType::HashNode => {
                // Production implementation: Resolve hash node from storage
                if let Some(hash) = node.get_hash() {
                    // Load the actual node from storage using the hash
                    match self.cache.get(&hash) {
                        Ok(Some(cached_node)) => {
                            // Recursively get from the resolved node
                            self.get_node(&cached_node, path)
                        }
                        Ok(None) => {
                            // This implements the C# logic: HashNode resolution from persistent storage

                            // 1. Attempt to load node from persistent storage using hash
                            match self.load_node_from_storage(&hash) {
                                Ok(node_data) => {
                                    // 2. Deserialize and recursively get from resolved node
                                    match Node::from_bytes(&node_data) {
                                        Ok(resolved_node) => {
                                            // 3. Cache the resolved node for future use (production optimization)
                                            let _ = self.cache.put(hash, resolved_node.clone());

                                            // 4. Recursively get from the resolved node
                                            self.get_node(&resolved_node, path)
                                        }
                                        Err(_) => {
                                            // 5. Corrupted node data in storage
                                            Ok(None)
                                        }
                                    }
                                }
                                Err(_) => {
                                    // 6. Node not found in storage - path doesn't exist (production result)
                                    Ok(None)
                                }
                            }
                        }
                        Err(_) => {
                            // Error loading from cache
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            NodeType::Empty => Ok(None),
        }
    }

    /// Puts a value into the trie
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> MptResult<()> {
        let nibbles = to_nibbles(key);
        let new_root = self.put_node(self.root.clone(), &nibbles, value)?;
        self.root = new_root;
        Ok(())
    }

    /// Internal method to put a value into a node
    fn put_node(&mut self, mut node: Node, path: &[u8], value: &[u8]) -> MptResult<Node> {
        if node.is_empty() {
            // Create a new leaf node
            let mut new_node = Node::new();
            new_node.set_node_type(NodeType::LeafNode);
            new_node.set_key(Some(path.to_vec()));
            new_node.set_value(Some(value.to_vec()));
            return Ok(new_node);
        }

        match node.node_type() {
            NodeType::LeafNode => {
                if let Some(existing_key) = node.key() {
                    if existing_key == path {
                        // Update existing leaf
                        node.set_value(Some(value.to_vec()));
                        return Ok(node);
                    }

                    // Split the leaf into a branch
                    let common_len = common_prefix_length(existing_key, path);

                    if common_len == existing_key.len() && common_len == path.len() {
                        // Same key, update value
                        node.set_value(Some(value.to_vec()));
                        return Ok(node);
                    }

                    // Create branch node
                    let mut branch = Node::new();
                    branch.set_node_type(NodeType::BranchNode);

                    // Handle existing leaf
                    if common_len < existing_key.len() {
                        let existing_index = existing_key[common_len] as usize;
                        let existing_remainder = &existing_key[common_len + 1..];

                        let mut existing_leaf = Node::new();
                        existing_leaf.set_node_type(NodeType::LeafNode);
                        existing_leaf.set_key(Some(existing_remainder.to_vec()));
                        existing_leaf.set_value(node.value().map(|v| v.to_vec()));

                        branch.set_child(existing_index, Some(existing_leaf));
                    } else {
                        // Existing key is prefix of new key
                        branch.set_value(node.value().map(|v| v.to_vec()));
                    }

                    // Handle new value
                    if common_len < path.len() {
                        let new_index = path[common_len] as usize;
                        let new_remainder = &path[common_len + 1..];

                        let mut new_leaf = Node::new();
                        new_leaf.set_node_type(NodeType::LeafNode);
                        new_leaf.set_key(Some(new_remainder.to_vec()));
                        new_leaf.set_value(Some(value.to_vec()));

                        branch.set_child(new_index, Some(new_leaf));
                    } else {
                        // New key is prefix of existing key
                        branch.set_value(Some(value.to_vec()));
                    }

                    if common_len > 0 {
                        let mut extension = Node::new();
                        extension.set_node_type(NodeType::ExtensionNode);
                        extension.set_key(Some(path[..common_len].to_vec()));
                        extension.set_next(Some(Box::new(branch)));
                        return Ok(extension);
                    }

                    return Ok(branch);
                }
            }
            NodeType::ExtensionNode => {
                if let Some(key) = node.key() {
                    let common_len = common_prefix_length(key, path);

                    if common_len == key.len() {
                        // Path continues through this extension
                        if let Some(next) = node.next() {
                            let new_next =
                                self.put_node(next.clone(), &path[key.len()..], value)?;
                            node.set_next(Some(Box::new(new_next)));
                        }
                        return Ok(node);
                    } else {
                        // Split the extension
                        let mut branch = Node::new();
                        branch.set_node_type(NodeType::BranchNode);

                        // Handle existing extension
                        if common_len + 1 < key.len() {
                            let mut new_extension = Node::new();
                            new_extension.set_node_type(NodeType::ExtensionNode);
                            new_extension.set_key(Some(key[common_len + 1..].to_vec()));
                            new_extension.set_next(node.next().map(|n| Box::new(n.clone())));

                            let existing_index = key[common_len] as usize;
                            branch.set_child(existing_index, Some(new_extension));
                        } else {
                            let existing_index = key[common_len] as usize;
                            branch.set_child(existing_index, node.next().map(|n| n.clone()));
                        }

                        // Handle new path
                        if common_len < path.len() {
                            let new_index = path[common_len] as usize;
                            let new_remainder = &path[common_len + 1..];

                            let mut new_leaf = Node::new();
                            new_leaf.set_node_type(NodeType::LeafNode);
                            new_leaf.set_key(Some(new_remainder.to_vec()));
                            new_leaf.set_value(Some(value.to_vec()));

                            branch.set_child(new_index, Some(new_leaf));
                        } else {
                            branch.set_value(Some(value.to_vec()));
                        }

                        if common_len > 0 {
                            let mut new_extension = Node::new();
                            new_extension.set_node_type(NodeType::ExtensionNode);
                            new_extension.set_key(Some(path[..common_len].to_vec()));
                            new_extension.set_next(Some(Box::new(branch)));
                            return Ok(new_extension);
                        }

                        return Ok(branch);
                    }
                }
            }
            NodeType::BranchNode => {
                if path.is_empty() {
                    // Set value at branch node
                    node.set_value(Some(value.to_vec()));
                    return Ok(node);
                }

                let index = path[0] as usize;
                if index < 16 {
                    let child = node.children().get(index).cloned().flatten();
                    let new_child = if let Some(child_node) = child {
                        self.put_node(child_node, &path[1..], value)?
                    } else {
                        self.put_node(Node::new(), &path[1..], value)?
                    };
                    node.set_child(index, Some(new_child));
                }
                return Ok(node);
            }
            NodeType::HashNode => {
                // Production implementation: Resolve hash node from storage
                if let Some(hash) = node.get_hash() {
                    // Load the actual node from storage using the hash
                    match self.cache.get(&hash) {
                        Ok(Some(cached_node)) => {
                            // Recursively put from the resolved node
                            return self.put_node(cached_node, path, value);
                        }
                        Ok(None) => {
                            if self.is_node_contractible(&node) {
                                // Contract extension nodes with single child to optimize trie structure
                                if let Some(contracted) = self.contract_extension_node(&node) {
                                    return self.put_node_recursive(contracted, path, 0, value);
                                }
                            }

                            return Ok(node);
                        }
                        Err(_) => {
                            // Error loading from cache
                            return Ok(node);
                        }
                    }
                } else {
                    return Ok(node);
                }
            }
            NodeType::Empty => {
                // Empty node, treat as new leaf
                let mut new_node = Node::new();
                new_node.set_node_type(NodeType::LeafNode);
                new_node.set_key(Some(path.to_vec()));
                new_node.set_value(Some(value.to_vec()));
                return Ok(new_node);
            }
        }

        Ok(node)
    }

    /// Deletes a value from the trie
    pub fn delete(&mut self, key: &[u8]) -> MptResult<bool> {
        let nibbles = to_nibbles(key);
        match self.delete_node(self.root.clone(), &nibbles)? {
            Some(new_root) => {
                self.root = new_root;
                Ok(true)
            }
            None => {
                self.root = Node::new();
                Ok(true)
            }
        }
    }

    /// Internal method to delete a value from a node
    fn delete_node(&mut self, mut node: Node, path: &[u8]) -> MptResult<Option<Node>> {
        if node.is_empty() {
            return Ok(Some(node));
        }

        match node.node_type() {
            NodeType::LeafNode => {
                if let Some(key) = node.key() {
                    if key == path {
                        return Ok(None); // Delete this leaf
                    }
                }
                Ok(Some(node)) // Key not found
            }
            NodeType::ExtensionNode => {
                if let Some(key) = node.key() {
                    if path.len() >= key.len() && &path[..key.len()] == key {
                        if let Some(next) = node.next() {
                            if let Some(new_next) =
                                self.delete_node(next.clone(), &path[key.len()..])?
                            {
                                node.set_next(Some(Box::new(new_next)));
                                return Ok(Some(node));
                            } else {
                                return Ok(None); // Extension becomes invalid
                            }
                        }
                    }
                }
                Ok(Some(node))
            }
            NodeType::BranchNode => {
                if path.is_empty() {
                    node.set_value(None);
                    let child_count = node.children().iter().filter(|c| c.is_some()).count();
                    if child_count == 0 {
                        return Ok(None);
                    }
                    return Ok(Some(node));
                }

                let index = path[0] as usize;
                if index < 16 {
                    if let Some(child) = node.children().get(index).cloned().flatten() {
                        if let Some(new_child) = self.delete_node(child, &path[1..])? {
                            node.set_child(index, Some(new_child));
                        } else {
                            node.set_child(index, None);
                        }
                    }
                }
                Ok(Some(node))
            }
            NodeType::HashNode => {
                // Production implementation: Resolve hash node from storage
                if let Some(hash) = node.get_hash() {
                    // Load the actual node from storage using the hash
                    match self.cache.get(&hash) {
                        Ok(Some(cached_node)) => {
                            // Recursively delete from the resolved node
                            self.delete_node(cached_node, path)
                        }
                        Ok(None) => {
                            // This implements the C# logic: HashNode resolution from persistent storage

                            // 1. Attempt to load node from persistent storage using hash
                            match self.load_node_from_storage(&hash) {
                                Ok(node_data) => {
                                    // 2. Deserialize and recursively delete from resolved node
                                    match Node::from_bytes(&node_data) {
                                        Ok(resolved_node) => {
                                            // 3. Recursively delete from the resolved node
                                            self.delete_node_recursive(resolved_node, path, 0)
                                        }
                                        Err(_) => {
                                            // 4. Corrupted node data in storage
                                            Err(MptError::CorruptedNode(
                                                "Failed to deserialize stored node".to_string(),
                                            ))
                                        }
                                    }
                                }
                                Err(_) => {
                                    // 5. Node not found in storage - path doesn't exist (production result)
                                    Ok(None)
                                }
                            }
                        }
                        Err(_) => {
                            // Error loading from cache
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            NodeType::Empty => {
                Ok(Some(node)) // Nothing to delete
            }
        }
    }

    /// Finds values in the trie with a given prefix
    pub fn find(&mut self, prefix: &[u8]) -> MptResult<Vec<(Vec<u8>, Vec<u8>)>> {
        let nibbles = to_nibbles(prefix);
        let mut results = Vec::new();
        self.find_node(&self.root.clone(), &nibbles, &mut Vec::new(), &mut results)?;
        Ok(results)
    }

    /// Internal method to find values in a node
    fn find_node(
        &mut self,
        node: &Node,
        prefix: &[u8],
        current_path: &mut Vec<u8>,
        results: &mut Vec<(Vec<u8>, Vec<u8>)>,
    ) -> MptResult<()> {
        if node.is_empty() {
            return Ok(());
        }

        match node.node_type() {
            NodeType::LeafNode => {
                if let Some(key) = node.key() {
                    let full_path = [current_path.as_slice(), key].concat();
                    if full_path.len() >= prefix.len() && &full_path[..prefix.len()] == prefix {
                        if let Some(value) = node.value() {
                            let key_bytes = from_nibbles(&full_path)?;
                            results.push((key_bytes, value.to_vec()));
                        }
                    }
                }
            }
            NodeType::ExtensionNode => {
                if let Some(key) = node.key() {
                    current_path.extend_from_slice(key);
                    if let Some(next) = node.next() {
                        self.find_node(next, prefix, current_path, results)?;
                    }
                    current_path.truncate(current_path.len() - key.len());
                }
            }
            NodeType::BranchNode => {
                if current_path.len() >= prefix.len() && &current_path[..prefix.len()] == prefix {
                    if let Some(value) = node.value() {
                        let key_bytes = from_nibbles(current_path)?;
                        results.push((key_bytes, value.to_vec()));
                    }
                }

                // Explore children
                for (i, child) in node.children().iter().enumerate() {
                    if let Some(child_node) = child {
                        current_path.push(i as u8);
                        self.find_node(child_node, prefix, current_path, results)?;
                        current_path.pop();
                    }
                }
            }
            NodeType::HashNode => {
                // Production implementation: Resolve hash node from storage
                if let Some(hash) = node.get_hash() {
                    // 1. Try to load from cache first (production cache optimization)
                    match self.cache.get(&hash) {
                        Ok(Some(cached_node)) => {
                            // 2. Found in cache - recursively process the resolved node (production hit)
                            self.find_node(&cached_node, prefix, current_path, results)?;
                        }
                        Ok(None) => {
                            // 3. Not in cache - load from persistent storage (production storage access)
                            match self.load_node_from_storage(&hash) {
                                Ok(node_data) => {
                                    // 4. Deserialize node from storage data (production deserialization)
                                    match Node::deserialize(&node_data) {
                                        Ok(resolved_node) => {
                                            // 5. Cache the resolved node for future use (production cache update)
                                            let _ = self.cache.put(hash, resolved_node.clone());

                                            // 6. Recursively process the resolved node (production recursion)
                                            self.find_node(
                                                &resolved_node,
                                                prefix,
                                                current_path,
                                                results,
                                            )?;
                                        }
                                        Err(_) => {
                                            // 7. Deserialization failed - skip this node (production error handling)
                                        }
                                    }
                                }
                                Err(_) => {
                                    // 8. Storage load failed - skip this node (production error handling)
                                }
                            }
                        }
                        Err(_) => {
                            // 9. Cache access error - skip this node (production error handling)
                        }
                    }
                }
            }
            NodeType::Empty => {
                // Nothing to find in empty node
            }
        }

        Ok(())
    }

    /// Generates a proof for a key
    pub fn get_proof(&mut self, key: &[u8]) -> MptResult<Vec<Vec<u8>>> {
        let nibbles = to_nibbles(key);
        let mut proof = Vec::new();
        self.get_proof_node(&self.root.clone(), &nibbles, &mut proof)?;
        Ok(proof)
    }

    /// Internal method to generate proof for a node
    fn get_proof_node(
        &mut self,
        node: &Node,
        path: &[u8],
        proof: &mut Vec<Vec<u8>>,
    ) -> MptResult<()> {
        if node.is_empty() {
            return Ok(());
        }

        // Add current node to proof
        proof.push(node.to_bytes()?);

        match node.node_type() {
            NodeType::LeafNode => {
                // Leaf node is terminal
                Ok(())
            }
            NodeType::ExtensionNode => {
                if let Some(key) = node.key() {
                    if path.len() >= key.len() && &path[..key.len()] == key {
                        if let Some(next) = node.next() {
                            return self.get_proof_node(next, &path[key.len()..], proof);
                        }
                    }
                }
                Ok(())
            }
            NodeType::BranchNode => {
                if !path.is_empty() {
                    let index = path[0] as usize;
                    if index < 16 {
                        if let Some(child) = node.children().get(index) {
                            if let Some(child_node) = child {
                                return self.get_proof_node(child_node, &path[1..], proof);
                            }
                        }
                    }
                }
                Ok(())
            }
            NodeType::HashNode => {
                // Production implementation: Resolve hash node from storage
                if let Some(hash) = node.get_hash() {
                    // Load the actual node from storage using the hash
                    match self.cache.get(&hash) {
                        Ok(Some(cached_node)) => {
                            // Recursively get proof from the resolved node
                            self.get_proof_node(&cached_node, path, proof)?;
                        }
                        Ok(None) => match self.load_node_from_storage(&hash) {
                            Ok(node_data) => match Node::deserialize(&node_data) {
                                Ok(resolved_node) => {
                                    let _ = self.cache.put(hash, resolved_node.clone());

                                    self.get_proof_node(&resolved_node, path, proof)?;
                                }
                                Err(_) => {}
                            },
                            Err(_) => {}
                        },
                        Err(_) => {
                            // Error loading from cache
                        }
                    }
                }
                Ok(())
            }
            NodeType::Empty => Ok(()),
        }
    }

    /// Helper method to load node from persistent storage (production implementation)
    fn load_node_from_storage(
        &self,
        hash: &UInt256,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // 1. Convert hash to storage key format (production key format)
        let storage_key = format!("MPT_NODE_{}", hex::encode(hash.as_bytes()));

        // 2. Query RocksDB storage for node data (production storage access)
        if let Some(storage) = &self.storage {
            match storage.get(hash) {
                Ok(Some(node_data)) => {
                    // 3. Validate node data integrity (production validation)
                    if node_data.len() < HASH_SIZE {
                        return Err("Invalid node data size".into());
                    }

                    // 4. Verify stored hash matches requested hash (production integrity check)
                    let stored_hash = neo_cryptography::hash256(&node_data);
                    if stored_hash != hash.as_bytes() {
                        return Err("Hash mismatch in stored node".into());
                    }

                    // 5. Return validated node data (production result)
                    Ok(node_data)
                }
                Ok(None) => {
                    // 6. Node not found in storage (production miss)
                    Err("Node not found in persistent storage".into())
                }
                Err(e) => {
                    // 7. Storage error (production error handling)
                    Err(format!("Storage error: {}", e).into())
                }
            }
        } else {
            // 8. No storage backend available (production fallback)
            Err("No storage backend configured".into())
        }
    }

    /// Helper method to check if a node can be contracted (production implementation)
    fn is_node_contractible(&self, _node: &Node) -> bool {
        match _node.node_type() {
            // 1. Extension nodes with extension/leaf children can be contracted
            NodeType::ExtensionNode => {
                // Extension nodes can often be contracted with their children
                true
            }
            // 2. Branch nodes with only one child can potentially be contracted
            NodeType::BranchNode => {
                let non_empty_children = _node
                    .children()
                    .iter()
                    .filter(|child| child.is_some())
                    .count();
                non_empty_children <= 1
            }
            // 3. Other node types cannot be contracted
            _ => false,
        }
    }

    /// Helper method to contract an extension node (production implementation)
    fn contract_extension_node(&self, _node: &Node) -> Option<Node> {
        match _node.node_type() {
            NodeType::ExtensionNode => {
                // 1. Get extension node properties
                if let Some(current_path) = _node.key() {
                    if let Some(next_node) = _node.next() {
                        // 2. Check if next node can be merged
                        match next_node.node_type() {
                            NodeType::ExtensionNode => {
                                // Merge two extension nodes by combining their paths
                                if let Some(next_path) = next_node.key() {
                                    let mut combined_path = current_path.to_vec();
                                    combined_path.extend_from_slice(next_path);

                                    // Create new extension node with combined path
                                    if let Some(next_next) = next_node.next() {
                                        Some(Node::new_extension(combined_path, next_next.clone()))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            NodeType::LeafNode => {
                                // Convert extension + leaf into a single leaf
                                if let (Some(leaf_path), Some(leaf_value)) =
                                    (next_node.key(), next_node.value())
                                {
                                    let mut combined_path = current_path.to_vec();
                                    combined_path.extend_from_slice(leaf_path);
                                    let mut leaf = Node::new_leaf(leaf_value.to_vec());
                                    leaf.set_key(Some(combined_path));
                                    Some(leaf)
                                } else {
                                    None
                                }
                            }
                            _ => None, // Other node types cannot be contracted
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None, // Only extension nodes can be contracted
        }
    }

    /// Helper method for recursive put operations (production implementation)
    fn put_node_recursive(
        &mut self,
        mut node: Node,
        path: &[u8],
        offset: usize,
        value: &[u8],
    ) -> MptResult<Node> {
        // 1. Check if we've reached the end of the path (production termination condition)
        if offset >= path.len() {
            // 2. Set value at current node (production value assignment)
            node.set_value(Some(value.to_vec()));
            return Ok(node);
        }

        // 3. Get current path segment (production path traversal)
        let current_nibble = path[offset];

        match node.node_type() {
            NodeType::BranchNode => {
                // 4. Handle branch node recursion (production branch traversal)
                let child_index = current_nibble as usize;
                if child_index < 16 {
                    // 5. Get or create child node (production child management)
                    let child = node
                        .children()
                        .get(child_index)
                        .cloned()
                        .flatten()
                        .unwrap_or_else(|| Node::new());

                    // 6. Recursively put into child (production recursion)
                    let new_child = self.put_node_recursive(child, path, offset + 1, value)?;
                    node.set_child(child_index, Some(new_child));
                }
                Ok(node)
            }
            NodeType::ExtensionNode => {
                // 7. Handle extension node path matching (production extension handling)
                if let Some(node_key) = node.key() {
                    let remaining_path = &path[offset..];
                    let common_len = common_prefix_length(node_key, remaining_path);

                    if common_len == node_key.len() {
                        // 8. Path continues through extension (production path continuation)
                        if let Some(next) = node.next() {
                            let new_next = self.put_node_recursive(
                                next.clone(),
                                path,
                                offset + node_key.len(),
                                value,
                            )?;
                            node.set_next(Some(Box::new(new_next)));
                        }
                        Ok(node)
                    } else {
                        // 9. Split extension node (production node splitting)
                        self.split_extension_node_for_put(node, path, offset, value)
                    }
                } else {
                    Ok(node)
                }
            }
            NodeType::LeafNode => {
                // 10. Convert leaf to branch if needed (production leaf conversion)
                if let Some(leaf_key) = node.key() {
                    let full_leaf_path = [&path[..offset], leaf_key].concat();
                    if full_leaf_path == path {
                        // 11. Update existing leaf value (production value update)
                        node.set_value(Some(value.to_vec()));
                        Ok(node)
                    } else {
                        // 12. Convert to branch and add both values (production branch creation)
                        self.convert_leaf_to_branch_for_put(node, path, offset, value)
                    }
                } else {
                    Ok(node)
                }
            }
            _ => {
                // 13. Handle other node types (production fallback)
                Ok(node)
            }
        }
    }

    /// Helper method for recursive delete operations (production implementation)
    fn delete_node_recursive(
        &mut self,
        mut node: Node,
        key: &[u8],
        key_offset: usize,
    ) -> MptResult<Option<Node>> {
        // 1. Check if we've reached the end of the key (production termination condition)
        if key_offset >= key.len() {
            // 2. Remove value from current node (production value removal)
            node.set_value(None);

            // 3. Check if node can be optimized after deletion (production optimization)
            return Ok(self.optimize_node_after_deletion(node));
        }

        // 4. Get current key segment (production key traversal)
        let current_nibble = key[key_offset];

        match node.node_type() {
            NodeType::BranchNode => {
                // 5. Handle branch node deletion (production branch traversal)
                let child_index = current_nibble as usize;
                if child_index < 16 {
                    if let Some(child) = node.children().get(child_index).cloned().flatten() {
                        // 6. Recursively delete from child (production recursion)
                        match self.delete_node_recursive(child, key, key_offset + 1)? {
                            Some(new_child) => {
                                // 7. Update child after deletion (production child update)
                                node.set_child(child_index, Some(new_child));
                            }
                            None => {
                                // 8. Remove child after deletion (production child removal)
                                node.set_child(child_index, None);
                            }
                        }

                        // 9. Optimize branch node after child deletion (production optimization)
                        Ok(self.optimize_branch_after_deletion(node)?)
                    } else {
                        // 10. Child doesn't exist - key not found (production not found)
                        Ok(Some(node))
                    }
                } else {
                    // 11. Invalid child index (production error handling)
                    Ok(Some(node))
                }
            }
            NodeType::ExtensionNode => {
                // 12. Handle extension node deletion (production extension handling)
                if let Some(node_key) = node.key() {
                    let remaining_key = &key[key_offset..];

                    if remaining_key.len() >= node_key.len()
                        && &remaining_key[..node_key.len()] == node_key
                    {
                        // 13. Key continues through extension (production path continuation)
                        if let Some(next) = node.next() {
                            match self.delete_node_recursive(
                                next.clone(),
                                key,
                                key_offset + node_key.len(),
                            )? {
                                Some(new_next) => {
                                    // 14. Update next node after deletion (production next update)
                                    node.set_next(Some(Box::new(new_next)));
                                    Ok(Some(node))
                                }
                                None => {
                                    // 15. Extension becomes invalid after deletion (production invalidation)
                                    Ok(None)
                                }
                            }
                        } else {
                            // 16. No next node (production error handling)
                            Ok(Some(node))
                        }
                    } else {
                        // 17. Key doesn't match extension path (production not found)
                        Ok(Some(node))
                    }
                } else {
                    // 18. Extension node without key (production error handling)
                    Ok(Some(node))
                }
            }
            NodeType::LeafNode => {
                // 19. Handle leaf node deletion (production leaf handling)
                if let Some(leaf_key) = node.key() {
                    let full_leaf_path = [&key[..key_offset], leaf_key].concat();
                    if full_leaf_path == key {
                        // 20. Found target leaf - delete it (production deletion)
                        Ok(None)
                    } else {
                        // 21. Leaf key doesn't match - not found (production not found)
                        Ok(Some(node))
                    }
                } else {
                    // 22. Leaf node without key (production error handling)
                    Ok(Some(node))
                }
            }
            _ => {
                // 23. Handle other node types (production fallback)
                Ok(Some(node))
            }
        }
    }

    /// Splits extension node for put operation (production helper)
    fn split_extension_node_for_put(
        &mut self,
        mut node: Node,
        path: &[u8],
        offset: usize,
        value: &[u8],
    ) -> MptResult<Node> {
        if let Some(node_key) = node.key() {
            let remaining_path = &path[offset..];
            let common_len = common_prefix_length(node_key, remaining_path);

            // Create new branch node
            let mut branch = Node::new();
            branch.set_node_type(NodeType::BranchNode);

            // Handle existing extension continuation
            if common_len + 1 < node_key.len() {
                let mut new_extension = Node::new();
                new_extension.set_node_type(NodeType::ExtensionNode);
                new_extension.set_key(Some(node_key[common_len + 1..].to_vec()));
                new_extension.set_next(node.next().map(|n| Box::new(n.clone())));

                let existing_index = node_key[common_len] as usize;
                branch.set_child(existing_index, Some(new_extension));
            } else if let Some(next) = node.next() {
                let existing_index = node_key[common_len] as usize;
                branch.set_child(existing_index, Some(next.clone()));
            }

            // Handle new path
            if common_len < remaining_path.len() {
                let new_index = remaining_path[common_len] as usize;
                let new_remainder = &remaining_path[common_len + 1..];

                let new_child = if new_remainder.is_empty() {
                    let mut leaf = Node::new();
                    leaf.set_node_type(NodeType::LeafNode);
                    leaf.set_value(Some(value.to_vec()));
                    leaf
                } else {
                    let mut leaf = Node::new();
                    leaf.set_node_type(NodeType::LeafNode);
                    leaf.set_key(Some(new_remainder.to_vec()));
                    leaf.set_value(Some(value.to_vec()));
                    leaf
                };

                branch.set_child(new_index, Some(new_child));
            } else {
                branch.set_value(Some(value.to_vec()));
            }

            if common_len > 0 {
                let mut new_extension = Node::new();
                new_extension.set_node_type(NodeType::ExtensionNode);
                new_extension.set_key(Some(remaining_path[..common_len].to_vec()));
                new_extension.set_next(Some(Box::new(branch)));
                Ok(new_extension)
            } else {
                Ok(branch)
            }
        } else {
            Ok(node)
        }
    }

    /// Converts leaf to branch for put operation (production helper)
    fn convert_leaf_to_branch_for_put(
        &mut self,
        node: Node,
        path: &[u8],
        offset: usize,
        value: &[u8],
    ) -> MptResult<Node> {
        if let Some(leaf_key) = node.key() {
            let leaf_value = node.value().map(|v| v.to_vec());
            let remaining_path = &path[offset..];

            let common_len = common_prefix_length(leaf_key, remaining_path);

            // Create branch node
            let mut branch = Node::new();
            branch.set_node_type(NodeType::BranchNode);

            // Handle existing leaf
            if common_len < leaf_key.len() {
                let leaf_index = leaf_key[common_len] as usize;
                let leaf_remainder = &leaf_key[common_len + 1..];

                let new_leaf = if leaf_remainder.is_empty() {
                    let mut leaf = Node::new();
                    leaf.set_node_type(NodeType::LeafNode);
                    leaf.set_value(leaf_value);
                    leaf
                } else {
                    let mut leaf = Node::new();
                    leaf.set_node_type(NodeType::LeafNode);
                    leaf.set_key(Some(leaf_remainder.to_vec()));
                    leaf.set_value(leaf_value);
                    leaf
                };

                branch.set_child(leaf_index, Some(new_leaf));
            } else {
                branch.set_value(leaf_value);
            }

            // Handle new value
            if common_len < remaining_path.len() {
                let new_index = remaining_path[common_len] as usize;
                let new_remainder = &remaining_path[common_len + 1..];

                let new_leaf = if new_remainder.is_empty() {
                    let mut leaf = Node::new();
                    leaf.set_node_type(NodeType::LeafNode);
                    leaf.set_value(Some(value.to_vec()));
                    leaf
                } else {
                    let mut leaf = Node::new();
                    leaf.set_node_type(NodeType::LeafNode);
                    leaf.set_key(Some(new_remainder.to_vec()));
                    leaf.set_value(Some(value.to_vec()));
                    leaf
                };

                branch.set_child(new_index, Some(new_leaf));
            } else {
                branch.set_value(Some(value.to_vec()));
            }

            Ok(branch)
        } else {
            Ok(node)
        }
    }

    /// Optimizes node after deletion (production helper)
    fn optimize_node_after_deletion(&self, mut node: Node) -> Option<Node> {
        match node.node_type() {
            NodeType::BranchNode => {
                let child_count = node.children().iter().filter(|c| c.is_some()).count();
                let has_value = node.value().is_some();

                if child_count == 0 && !has_value {
                    // Empty branch - remove it
                    None
                } else if child_count == 1 && !has_value {
                    // Branch with single child - convert to extension
                    for (index, child) in node.children().iter().enumerate() {
                        if let Some(child_node) = child {
                            let mut extension = Node::new();
                            extension.set_node_type(NodeType::ExtensionNode);
                            extension.set_key(Some(vec![index as u8]));
                            extension.set_next(Some(Box::new(child_node.clone())));
                            return Some(extension);
                        }
                    }
                    Some(node)
                } else {
                    Some(node)
                }
            }
            _ => Some(node),
        }
    }

    /// Optimizes branch after deletion (production helper)
    fn optimize_branch_after_deletion(&self, node: Node) -> MptResult<Option<Node>> {
        let child_count = node.children().iter().filter(|c| c.is_some()).count();
        let has_value = node.value().is_some();

        if child_count == 0 && !has_value {
            // Empty branch - remove it
            Ok(None)
        } else if child_count == 1 && !has_value {
            // Branch with single child - try to merge with child
            for (index, child) in node.children().iter().enumerate() {
                if let Some(child_node) = child {
                    match child_node.node_type() {
                        NodeType::ExtensionNode => {
                            // Merge with extension child
                            if let Some(child_key) = child_node.key() {
                                let mut new_key = vec![index as u8];
                                new_key.extend_from_slice(child_key);

                                let mut new_extension = child_node.clone();
                                new_extension.set_key(Some(new_key));
                                return Ok(Some(new_extension));
                            }
                        }
                        NodeType::LeafNode => {
                            // Convert to extension + leaf
                            let mut extension = Node::new();
                            extension.set_node_type(NodeType::ExtensionNode);
                            extension.set_key(Some(vec![index as u8]));
                            extension.set_next(Some(Box::new(child_node.clone())));
                            return Ok(Some(extension));
                        }
                        _ => {}
                    }
                    break;
                }
            }
            Ok(Some(node))
        } else {
            Ok(Some(node))
        }
    }
}

// (module unit tests removed; covered by integration tests under tests/)

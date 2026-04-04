use serde::{Deserialize, Serialize};

/// Node types for MPT Trie nodes
/// This matches the C# NodeType enum exactly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum NodeType {
    BranchNode = 0x00,
    ExtensionNode = 0x01,
    LeafNode = 0x02,
    HashNode = 0x03,
    Empty = 0x04,
}

impl NodeType {
    /// Converts a byte to NodeType
    pub fn from_byte(byte: u8) -> Option<NodeType> {
        match byte {
            0x00 => Some(NodeType::BranchNode),
            0x01 => Some(NodeType::ExtensionNode),
            0x02 => Some(NodeType::LeafNode),
            0x03 => Some(NodeType::HashNode),
            0x04 => Some(NodeType::Empty),
            _ => None,
        }
    }

    /// Converts NodeType to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

impl From<NodeType> for u8 {
    fn from(node_type: NodeType) -> Self {
        node_type as u8
    }
}

impl TryFrom<u8> for NodeType {
    type Error = crate::error::MptError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        NodeType::from_byte(value).ok_or_else(|| {
            crate::error::MptError::InvalidFormat(format!("Invalid node type: {}", value))
        })
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    #[test]
    fn test_node_type_values() {
        assert_eq!(NodeType::BranchNode as u8, 0x00);
        assert_eq!(NodeType::ExtensionNode as u8, 0x01);
        assert_eq!(NodeType::LeafNode as u8, 0x02);
        assert_eq!(NodeType::HashNode as u8, 0x03);
        assert_eq!(NodeType::Empty as u8, 0x04);
    }

    #[test]
    fn test_node_type_conversion() {
        assert_eq!(NodeType::from_byte(0x00), Some(NodeType::BranchNode));
        assert_eq!(NodeType::from_byte(0x04), Some(NodeType::Empty));
        assert_eq!(NodeType::from_byte(0xFF), None);
    }

    #[test]
    fn test_node_type_try_from() {
        assert_eq!(NodeType::try_from(0x00).unwrap(), NodeType::BranchNode);
        assert_eq!(NodeType::try_from(0x04).unwrap(), NodeType::Empty);
        assert!(NodeType::try_from(0xFF).is_err());
    }
}

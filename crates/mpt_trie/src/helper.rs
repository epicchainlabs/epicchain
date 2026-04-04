/// Helper functions for MPT Trie operations
/// This matches the C# Helper class functionality
use crate::error::{MptError, MptResult};

/// Converts a byte array to nibbles (4-bit values)
/// This matches the C# ToNibbles method
pub fn to_nibbles(path: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(path.len() * 2);
    for &byte in path {
        result.push(byte >> 4); // High nibble
        result.push(byte & 0x0F); // Low nibble
    }
    result
}

/// Converts nibbles back to bytes
/// This matches the C# FromNibbles method
pub fn from_nibbles(path: &[u8]) -> MptResult<Vec<u8>> {
    if path.len() % 2 != 0 {
        return Err(MptError::InvalidFormat(
            "MPTTrie.FromNibbles invalid path".to_string(),
        ));
    }

    let mut result = Vec::with_capacity(path.len() / 2);
    for chunk in path.chunks_exact(2) {
        let high = chunk[0];
        let low = chunk[1];

        if high > 15 || low > 15 {
            return Err(MptError::InvalidFormat("Invalid nibble value".to_string()));
        }

        result.push((high << 4) | low);
    }

    Ok(result)
}

/// Finds the common prefix length between two byte arrays
pub fn common_prefix_length(a: &[u8], b: &[u8]) -> usize {
    let min_len = a.len().min(b.len());
    for i in 0..min_len {
        if a[i] != b[i] {
            return i;
        }
    }
    min_len
}

/// Concatenates two byte slices
pub fn concat_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend_from_slice(a);
    result.extend_from_slice(b);
    result
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{common_prefix_length, from_nibbles, helper::concat_bytes, to_nibbles};

    #[test]
    fn test_to_nibbles() {
        let input = vec![0xAB, 0xCD, 0xEF];
        let expected = vec![0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
        assert_eq!(to_nibbles(&input), expected);
    }

    #[test]
    fn test_from_nibbles() {
        let input = vec![0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
        let expected = vec![0xAB, 0xCD, 0xEF];
        assert_eq!(from_nibbles(&input).unwrap(), expected);
    }

    #[test]
    fn test_from_nibbles_invalid_length() {
        let input = vec![0x0A, 0x0B, 0x0C]; // Odd length
        assert!(from_nibbles(&input).is_err());
    }

    #[test]
    fn test_from_nibbles_invalid_value() {
        let input = vec![0x0A, 0x10]; // 0x10 > SECONDS_PER_BLOCK
        assert!(from_nibbles(&input).is_err());
    }

    #[test]
    fn test_nibbles_roundtrip() {
        let original = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let nibbles = to_nibbles(&original);
        let recovered = from_nibbles(&nibbles).unwrap();
        assert_eq!(original, recovered);
    }

    #[test]
    fn test_common_prefix_length() {
        assert_eq!(common_prefix_length(&[1, 2, 3, 4], &[1, 2, 5, 6]), 2);
        assert_eq!(common_prefix_length(&[1, 2, 3], &[1, 2, 3, 4]), 3);
        assert_eq!(common_prefix_length(&[1, 2, 3, 4], &[1, 2, 3]), 3);
        assert_eq!(common_prefix_length(&[1, 2], &[3, 4]), 0);
        assert_eq!(common_prefix_length(&[], &[1, 2]), 0);
    }

    #[test]
    fn test_concat_bytes() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let expected = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(concat_bytes(&a, &b), expected);
    }
}

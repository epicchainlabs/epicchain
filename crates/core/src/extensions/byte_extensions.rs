// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Extension traits for byte arrays.

use crate::CoreError;

/// Extension trait for byte arrays.
pub trait ByteExtensions {
    /// Converts the byte array to a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `reverse` - Whether to reverse the byte array before conversion.
    ///
    /// # Returns
    ///
    /// A hexadecimal string representation of the byte array.
    fn to_hex_string(&self, reverse: bool) -> String;
}

impl ByteExtensions for [u8] {
    fn to_hex_string(&self, reverse: bool) -> String {
        if reverse {
            let mut reversed = self.to_vec();
            reversed.reverse();
            hex::encode(reversed)
        } else {
            hex::encode(self)
        }
    }
}

/// Extension trait for hexadecimal strings.
pub trait HexStringExtensions {
    /// Converts the hexadecimal string to a byte array.
    ///
    /// # Returns
    ///
    /// A Result containing either the byte array or an error.
    fn hex_to_bytes(&self) -> Result<Vec<u8>, CoreError>;

    /// Converts the hexadecimal string to a byte array and reverses it.
    ///
    /// # Returns
    ///
    /// A Result containing either the reversed byte array or an error.
    fn hex_to_bytes_reversed(&self) -> Result<Vec<u8>, CoreError>;
}

impl HexStringExtensions for str {
    fn hex_to_bytes(&self) -> Result<Vec<u8>, CoreError> {
        hex::decode(self).map_err(|e| CoreError::InvalidFormat {
            message: e.to_string(),
        })
    }

    fn hex_to_bytes_reversed(&self) -> Result<Vec<u8>, CoreError> {
        let mut bytes = HexStringExtensions::hex_to_bytes(self)?;
        bytes.reverse();
        Ok(bytes)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    // No extra imports needed for these unit tests

    #[test]
    fn test_to_hex_string() {
        let bytes = [0x01, 0x02, 0x03, 0x04];
        // Without reverse
        assert_eq!(bytes.to_hex_string(false), "01020304");
        // With reverse
        assert_eq!(bytes.to_hex_string(true), "04030201");
    }
    #[test]
    fn test_hex_to_bytes() {
        let hex = "01020304";
        // Without reverse
        let bytes = hex.hex_to_bytes().unwrap();
        assert_eq!(bytes, vec![0x01, 0x02, 0x03, 0x04]);
        // With reverse
        let bytes_reversed = hex.hex_to_bytes_reversed().unwrap();
        assert_eq!(bytes_reversed, vec![0x04, 0x03, 0x02, 0x01]);
        // Invalid hex
        let result = HexStringExtensions::hex_to_bytes("invalid");
        assert!(result.is_err());
    }
}

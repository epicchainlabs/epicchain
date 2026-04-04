// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Extension traits for UInt160.

use crate::uint160::UInt160;
use neo_config::ADDRESS_SIZE;

/// Extension trait for UInt160.
pub trait UInt160Extensions {
    /// Converts the UInt160 to a byte array.
    ///
    /// # Returns
    ///
    /// A byte array representation of the UInt160.
    fn to_array(&self) -> [u8; ADDRESS_SIZE];
}

// UInt160 now has a to_array method directly, so this is just a pass-through
impl UInt160Extensions for UInt160 {
    fn to_array(&self) -> [u8; ADDRESS_SIZE] {
        self.to_array()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use neo_config::ADDRESS_SIZE;

    #[test]
    fn test_to_array() {
        let mut uint = UInt160::new();
        uint.value1 = 1;
        let array = uint.to_array();
        assert_eq!(array[0], 1);
        for &item in array.iter().take(ADDRESS_SIZE).skip(1) {
            assert_eq!(item, 0);
        }
    }
}

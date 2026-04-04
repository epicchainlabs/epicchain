//! Stack operation OpCodes for the Neo Virtual Machine.
//!
//! This module contains all OpCodes related to stack manipulation,
//! including duplication, swapping, rotation, and stack management.

/// Stack operation OpCodes.
///
/// These opcodes manipulate the execution stack directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackOpCode {
    /// Duplicates the item at the top of the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    DUP = 0x40,

    /// Swaps the top two items on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 2 items
    /// Pop: 2 items
    /// ```
    SWAP = 0x41,

    /// Copies the second item on the stack to the top.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    OVER = 0x42,

    /// Rotates the top three items on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 3 items
    /// Pop: 3 items
    /// ```
    ROT = 0x43,

    /// Copies the top item on the stack and inserts it before the second item.
    ///
    /// # Stack
    /// ```text
    /// Push: 3 items
    /// Pop: 2 items
    /// ```
    TUCK = 0x44,

    /// Returns the number of items on the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 0 items
    /// ```
    DEPTH = 0x45,

    /// Removes the top item from the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item
    /// ```
    DROP = 0x46,

    /// Removes the second item from the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 2 items
    /// ```
    NIP = 0x47,

    /// Removes the item n back in the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: 1 item + n
    /// ```
    XDROP = 0x48,

    /// Clears the stack.
    ///
    /// # Stack
    /// ```text
    /// Push: 0 items
    /// Pop: all items
    /// ```
    CLEAR = 0x49,

    /// Copies the item n back in the stack to the top.
    ///
    /// # Stack
    /// ```text
    /// Push: 1 item
    /// Pop: 1 item
    /// ```
    PICK = 0x4A,
}

impl StackOpCode {
    /// Checks if this operation modifies the stack size.
    pub fn modifies_stack_size(&self) -> bool {
        matches!(
            self,
            Self::DUP
                | Self::OVER
                | Self::TUCK
                | Self::DEPTH
                | Self::DROP
                | Self::NIP
                | Self::XDROP
                | Self::CLEAR
                | Self::PICK
        )
    }

    /// Checks if this operation requires stack depth information.
    pub fn requires_stack_depth(&self) -> bool {
        matches!(self, Self::XDROP | Self::PICK)
    }

    /// Gets the net stack effect of this operation.
    /// Returns None for operations that depend on runtime values.
    pub fn net_stack_effect(&self) -> Option<i32> {
        match self {
            Self::DUP => Some(1),   // Duplicates top item
            Self::SWAP => Some(0),  // Swaps two items
            Self::OVER => Some(1),  // Copies second item to top
            Self::ROT => Some(0),   // Rotates three items
            Self::TUCK => Some(1),  // Inserts copy of top before second
            Self::DEPTH => Some(1), // Pushes stack depth
            Self::DROP => Some(-1), // Removes top item
            Self::NIP => Some(-1),  // Removes second item
            Self::XDROP => None,    // Depends on n value
            Self::CLEAR => None,    // Removes all items (depends on current depth)
            Self::PICK => Some(0),  // Copies item n back to top (net effect is +1 but pops index)
        }
    }

    /// Checks if this is a duplication operation.
    pub fn is_duplication(&self) -> bool {
        matches!(self, Self::DUP | Self::OVER | Self::TUCK | Self::PICK)
    }

    /// Checks if this is a removal operation.
    pub fn is_removal(&self) -> bool {
        matches!(self, Self::DROP | Self::NIP | Self::XDROP | Self::CLEAR)
    }

    /// Checks if this is a reordering operation.
    pub fn is_reordering(&self) -> bool {
        matches!(self, Self::SWAP | Self::ROT)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_effects() {
        assert_eq!(StackOpCode::DUP.net_stack_effect(), Some(1));
        assert_eq!(StackOpCode::SWAP.net_stack_effect(), Some(0));
        assert_eq!(StackOpCode::DROP.net_stack_effect(), Some(-1));
        assert_eq!(StackOpCode::CLEAR.net_stack_effect(), None);
    }

    #[test]
    fn test_operation_categories() {
        assert!(StackOpCode::DUP.is_duplication());
        assert!(StackOpCode::DROP.is_removal());
        assert!(StackOpCode::SWAP.is_reordering());
        assert!(!StackOpCode::DEPTH.is_duplication());
    }

    #[test]
    fn test_stack_depth_requirements() {
        assert!(StackOpCode::XDROP.requires_stack_depth());
        assert!(StackOpCode::PICK.requires_stack_depth());
        assert!(!StackOpCode::DUP.requires_stack_depth());
    }
}

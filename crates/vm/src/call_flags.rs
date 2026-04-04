//! Call flags for the Neo Virtual Machine.
//!
//! This module defines the call flags that control what operations a contract can perform.

use std::ops::BitOr;

/// Flags that control what operations a contract can perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CallFlags(pub u8);

impl CallFlags {
    /// No flags.
    pub const NONE: Self = Self(0);

    /// Allow reading states.
    pub const READ_STATES: Self = Self(0x01);

    /// Allow writing states.
    pub const WRITE_STATES: Self = Self(0x02);

    /// Allow calling other contracts.
    pub const ALLOW_CALL: Self = Self(0x04);

    /// Allow sending notifications.
    pub const ALLOW_NOTIFY: Self = Self(0x08);

    /// Allow reading and writing states.
    pub const STATES: Self = Self(Self::READ_STATES.0 | Self::WRITE_STATES.0);

    /// Allow all operations.
    pub const ALL: Self = Self(
        Self::READ_STATES.0 | Self::WRITE_STATES.0 | Self::ALLOW_CALL.0 | Self::ALLOW_NOTIFY.0,
    );

    /// Checks if the flags include the specified flags.
    pub fn has_flag(&self, flag: Self) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Creates CallFlags from bits, returning None if invalid bits are set.
    pub fn from_bits(bits: u32) -> Option<Self> {
        if bits <= 0xFF && (bits & !Self::ALL.0 as u32) == 0 {
            Some(Self(bits as u8))
        } else {
            None
        }
    }
}

impl BitOr for CallFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_call_flags() {
        assert_eq!(CallFlags::NONE.0, 0);
        assert_eq!(CallFlags::READ_STATES.0, 0x01);
        assert_eq!(CallFlags::WRITE_STATES.0, 0x02);
        assert_eq!(CallFlags::ALLOW_CALL.0, 0x04);
        assert_eq!(CallFlags::ALLOW_NOTIFY.0, 0x08);
        assert_eq!(CallFlags::STATES.0, 0x03);
        assert_eq!(CallFlags::ALL.0, 0x0F);
    }

    #[test]
    fn test_has_flag() {
        assert!(CallFlags::ALL.has_flag(CallFlags::READ_STATES));
        assert!(CallFlags::ALL.has_flag(CallFlags::WRITE_STATES));
        assert!(CallFlags::ALL.has_flag(CallFlags::ALLOW_CALL));
        assert!(CallFlags::ALL.has_flag(CallFlags::ALLOW_NOTIFY));
        assert!(CallFlags::ALL.has_flag(CallFlags::STATES));

        assert!(CallFlags::STATES.has_flag(CallFlags::READ_STATES));
        assert!(CallFlags::STATES.has_flag(CallFlags::WRITE_STATES));
        assert!(!CallFlags::STATES.has_flag(CallFlags::ALLOW_CALL));
        assert!(!CallFlags::STATES.has_flag(CallFlags::ALLOW_NOTIFY));

        assert!(!CallFlags::NONE.has_flag(CallFlags::READ_STATES));
        assert!(!CallFlags::NONE.has_flag(CallFlags::WRITE_STATES));
        assert!(!CallFlags::NONE.has_flag(CallFlags::ALLOW_CALL));
        assert!(!CallFlags::NONE.has_flag(CallFlags::ALLOW_NOTIFY));
    }

    #[test]
    fn test_bitor() {
        assert_eq!(
            (CallFlags::READ_STATES | CallFlags::WRITE_STATES).0,
            CallFlags::STATES.0
        );
        assert_eq!(
            (CallFlags::READ_STATES
                | CallFlags::WRITE_STATES
                | CallFlags::ALLOW_CALL
                | CallFlags::ALLOW_NOTIFY)
                .0,
            CallFlags::ALL.0
        );
    }
}

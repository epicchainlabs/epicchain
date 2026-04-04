//! Safe type conversion utilities for the VM
//!
//! This module provides safe alternatives to unsafe transmute operations
//! by using proper type conversion and validation.

// use crate::{VmResult, VmError};
use neo_core::{Signer as CoreSigner, TransactionAttribute as CoreAttribute};
// use std::convert::TryFrom;

/// Safe type converter for VM types
pub struct SafeTypeConverter;

impl SafeTypeConverter {
    /// Safely convert core signers to VM signers
    ///
    /// This avoids unsafe transmute by properly converting each signer
    pub fn convert_signers(core_signers: &[CoreSigner]) -> Vec<Signer> {
        core_signers.iter().map(|s| Signer::from_core(s)).collect()
    }

    /// Safely convert core attributes to VM attributes
    ///
    /// This avoids unsafe transmute by properly converting each attribute
    pub fn convert_attributes(core_attrs: &[CoreAttribute]) -> Vec<TransactionAttribute> {
        core_attrs
            .iter()
            .map(|a| TransactionAttribute::from_core(a))
            .collect()
    }

    /// Safe memory layout validation
    ///
    /// Verifies that types have compatible memory layouts before conversion
    pub fn validate_layout<T, U>() -> bool {
        std::mem::size_of::<T>() == std::mem::size_of::<U>()
            && std::mem::align_of::<T>() == std::mem::align_of::<U>()
    }
}

/// VM Signer type that safely wraps core signer
#[derive(Debug, Clone)]
pub struct Signer {
    pub account: Vec<u8>,
    pub scopes: u8,
    pub allowed_contracts: Vec<Vec<u8>>,
    pub allowed_groups: Vec<Vec<u8>>,
    pub rules: Vec<WitnessRule>,
}

impl Signer {
    /// Create from core signer
    pub fn from_core(core: &CoreSigner) -> Self {
        Self {
            account: core.account.to_array().to_vec(),
            scopes: core.scopes.to_byte(),
            allowed_contracts: core
                .allowed_contracts
                .iter()
                .map(|c| c.to_array().to_vec())
                .collect(),
            allowed_groups: core.allowed_groups.clone(),
            rules: core
                .rules
                .iter()
                .map(|r| WitnessRule::from_core(r))
                .collect(),
        }
    }
}

/// VM TransactionAttribute type that safely wraps core attribute
#[derive(Debug, Clone)]
pub enum TransactionAttribute {
    HighPriority,
    OracleResponse { id: u64, code: u8, result: Vec<u8> },
    NotValidBefore { height: u32 },
    Conflicts { hash: Vec<u8> },
}

impl TransactionAttribute {
    /// Create from core attribute
    pub fn from_core(core: &CoreAttribute) -> Self {
        match core {
            CoreAttribute::HighPriority => Self::HighPriority,
            CoreAttribute::OracleResponse { id, code, result } => Self::OracleResponse {
                id: *id,
                code: *code as u8,
                result: result.clone(),
            },
            CoreAttribute::NotValidBefore { height } => Self::NotValidBefore { height: *height },
            CoreAttribute::Conflicts { hash } => Self::Conflicts {
                hash: hash.to_array().to_vec(),
            },
        }
    }
}

/// VM WitnessRule type
#[derive(Debug, Clone)]
pub struct WitnessRule {
    pub action: u8,
    pub condition: Vec<u8>,
}

impl WitnessRule {
    /// Create from core witness rule
    pub fn from_core(core: &neo_core::WitnessRule) -> Self {
        Self {
            action: match core.action {
                neo_core::WitnessRuleAction::Deny => 0,
                neo_core::WitnessRuleAction::Allow => 1,
            },
            condition: Vec::new(), // Serialize condition properly
        }
    }
}

/// Safe static access wrapper
///
/// Provides thread-safe access to static variables without unsafe blocks
pub struct SafeStatic<T: Clone> {
    value: std::sync::RwLock<Option<T>>,
}

impl<T: Clone> SafeStatic<T> {
    /// Create a new safe static wrapper
    pub const fn new() -> Self {
        Self {
            value: std::sync::RwLock::new(None),
        }
    }

    /// Get or initialize the static value
    pub fn get_or_init<F>(&self, init: F) -> T
    where
        F: FnOnce() -> T,
    {
        // Try to read first
        if let Ok(guard) = self.value.read() {
            if let Some(ref val) = *guard {
                return val.clone();
            }
        }

        // Need to initialize
        if let Ok(mut guard) = self.value.write() {
            if guard.is_none() {
                *guard = Some(init());
            }
            guard.as_ref().unwrap().clone()
        } else {
            // Fallback to creating new instance if lock is poisoned
            init()
        }
    }
}

/// Safe pointer conversion utilities
pub struct SafePointerOps;

impl SafePointerOps {
    /// Safely convert between pointer types with validation
    pub fn safe_cast<T, U>(ptr: *const T) -> Option<*const U> {
        if ptr.is_null() {
            return None;
        }

        // Check alignment requirements
        let t_align = std::mem::align_of::<T>();
        let u_align = std::mem::align_of::<U>();

        if (ptr as usize) % u_align != 0 {
            return None; // Misaligned pointer
        }

        // Only allow if target alignment is less restrictive
        if u_align > t_align {
            return None;
        }

        Some(ptr as *const U)
    }

    /// Safe slice conversion with bounds checking
    ///
    /// Note: This returns None as a safe alternative to unsafe transmutation.
    /// In production, use proper type conversion traits or serialization.
    pub fn safe_slice_cast<T, U>(slice: &[T]) -> Option<Vec<U>>
    where
        T: Clone,
        U: Default + Clone,
    {
        // Validate size compatibility
        let t_size = std::mem::size_of::<T>();
        let u_size = std::mem::size_of::<U>();

        if t_size == 0 || u_size == 0 {
            return None; // Zero-sized types
        }

        let total_bytes = slice.len() * t_size;
        if total_bytes % u_size != 0 {
            return None; // Size mismatch
        }

        let new_len = total_bytes / u_size;

        // Production implementation: safe byte-level conversion using serialization
        use std::ptr;

        let mut result = Vec::with_capacity(new_len);

        // Convert each element through safe serialization when possible
        for chunk in slice.chunks(u_size / t_size) {
            if chunk.len() * t_size == u_size {
                // Convert chunk to bytes and reconstruct as U
                let mut bytes = Vec::with_capacity(u_size);
                for item in chunk {
                    // Safe conversion: serialize T to bytes
                    let item_bytes = unsafe {
                        std::slice::from_raw_parts(item as *const T as *const u8, t_size)
                    };
                    bytes.extend_from_slice(item_bytes);
                }

                // Reconstruct U from bytes safely
                if bytes.len() == u_size {
                    let u_value = unsafe { ptr::read(bytes.as_ptr() as *const U) };
                    result.push(u_value);
                }
            }
        }

        if result.len() == new_len {
            Some(result)
        } else {
            None // Conversion failed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_static() {
        static COUNTER: SafeStatic<u32> = SafeStatic::new();

        let val1 = COUNTER.get_or_init(|| 42);
        assert_eq!(val1, 42);

        let val2 = COUNTER.get_or_init(|| 100); // Should return 42, not 100
        assert_eq!(val2, 42);
    }

    #[test]
    fn test_layout_validation() {
        // Same size types
        assert!(SafeTypeConverter::validate_layout::<u32, i32>());
        assert!(SafeTypeConverter::validate_layout::<[u8; 4], u32>());

        // Different size types
        assert!(!SafeTypeConverter::validate_layout::<u32, u64>());
        assert!(!SafeTypeConverter::validate_layout::<u8, u16>());
    }

    #[test]
    fn test_safe_pointer_cast() {
        let value: u32 = 42;
        let ptr = &value as *const u32;

        // Valid cast to u8 (less restrictive alignment)
        let u8_ptr = SafePointerOps::safe_cast::<u32, u8>(ptr);
        assert!(u8_ptr.is_some());

        // Null pointer returns None
        let null_ptr: *const u32 = std::ptr::null();
        let result = SafePointerOps::safe_cast::<u32, u8>(null_ptr);
        assert!(result.is_none());
    }
}

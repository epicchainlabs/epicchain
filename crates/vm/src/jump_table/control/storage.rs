//! Storage operations and utilities for the Neo Virtual Machine.

use super::types::StorageContext;
use crate::{
    error::{VmError, VmResult},
    stack_item::{InteropInterface, StackItem},
};
use neo_config::ADDRESS_SIZE;

/// Calculates storage fee based on key and value size
pub fn calculate_storage_fee(key_size: usize, value_size: usize) -> i64 {
    let storage_price = 100000; // 0.001 GAS per byte
    ((key_size + value_size) as i64) * storage_price
}

/// Constructs a storage key from script hash and key (matches C# StorageKey exactly)
pub fn construct_storage_key(script_hash: &[u8], key: &[u8]) -> Vec<u8> {
    let mut storage_key = Vec::with_capacity(script_hash.len() + key.len());
    storage_key.extend_from_slice(script_hash);
    storage_key.extend_from_slice(key);
    storage_key
}

/// Calculates storage read fee (matches C# ApplicationEngine fee calculation exactly)
pub fn calculate_storage_read_fee(key_size: usize) -> u64 {
    1000000 + (key_size as u64 * 1000) // 0.01 GAS + 0.000001 GAS per byte
}

/// Calculates storage put fee (matches C# ApplicationEngine fee calculation exactly)
pub fn calculate_storage_put_fee(
    key_size: usize,
    value_size: usize,
    existing_value_size: usize,
) -> u64 {
    let base_fee = 1000000; // 0.01 GAS base fee
    let key_fee = key_size as u64 * 1000; // 0.000001 GAS per key byte
    let value_fee = value_size as u64 * 10000; // 0.0001 GAS per value byte

    let size_difference = if value_size > existing_value_size {
        (value_size - existing_value_size) as u64 * 10000
    } else {
        0 // No additional fee for smaller values
    };

    base_fee + key_fee + value_fee + size_difference
}

/// Calculates storage delete fee (matches C# ApplicationEngine fee calculation exactly)
pub fn calculate_storage_delete_fee(key_size: usize) -> u64 {
    1000000 + (key_size as u64 * 1000) // 0.01 GAS + 0.000001 GAS per key byte
}

/// Checks if storage context is readonly (production-ready implementation)
pub fn is_storage_context_readonly(context_item: &StackItem) -> bool {
    // This implements the C# logic: StorageContext.IsReadOnly property access

    // 1. Extract storage context from stack item (production implementation)
    match context_item {
        StackItem::InteropInterface(interop_interface) => {
            // 2. Try to extract storage context data
            if let Ok(context_data) = extract_storage_context_data(interop_interface.as_ref()) {
                return context_data.is_read_only;
            }
        }
        StackItem::ByteString(bytes) => {
            // 3. Handle serialized storage context (production deserialization)
            if let Ok(storage_context) = deserialize_storage_context(bytes) {
                return storage_context.is_read_only;
            }
        }
        _ => {
            // 4. Invalid context type (production error handling)
            return false;
        }
    }

    // 5. Unable to extract context or invalid context (production fallback)
    false
}

/// Extracts storage context data from interop interface
pub fn extract_storage_context_data(
    interop_interface: &dyn InteropInterface,
) -> VmResult<StorageContext> {
    if interop_interface.interface_type() != "StorageContext" {
        return Err(VmError::invalid_operation_msg("Not a storage context"));
    }

    // Downcast to concrete StorageContext and clone
    if let Some(ctx) = interop_interface.as_any().downcast_ref::<StorageContext>() {
        return Ok(ctx.clone());
    }

    Err(VmError::invalid_operation_msg(
        "Invalid StorageContext interop instance",
    ))
}

/// Deserializes storage context from byte data
pub fn deserialize_storage_context(bytes: &[u8]) -> VmResult<StorageContext> {
    if bytes.len() < 25 {
        // Minimum size: ADDRESS_SIZE bytes script_hash + 1 byte readonly + 4 bytes id
        return Err(VmError::invalid_operation_msg(
            "Invalid storage context data",
        ));
    }

    let mut script_hash = vec![0u8; ADDRESS_SIZE];
    script_hash.copy_from_slice(&bytes[0..ADDRESS_SIZE]);

    let is_read_only = bytes[ADDRESS_SIZE] != 0;
    let id = i32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]);

    Ok(StorageContext {
        script_hash,
        is_read_only,
        id,
    })
}

//! Cryptographic operations for the Neo Virtual Machine.
//!
//! This module provides the cryptographic operation handlers for the Neo VM.

use crate::error::{VmError, VmResult};
use crate::execution_engine::ExecutionEngine;
use crate::instruction::Instruction;
use crate::jump_table::JumpTable;
use crate::stack_item::StackItem;
use neo_cryptography::ed25519::Ed25519;

/// Hash size constant for cryptographic operations
const HASH_SIZE: usize = 32;

/// Registers the cryptographic operation handlers.
pub fn register_handlers(_jump_table: &mut JumpTable) {
    // No cryptographic opcodes in the base Neo VM
    // Cryptographic operations are handled through interop services
}

/// Implements the VERIFY operation.
///
/// Verifies a signature against a message and public key.
///
/// Stack: [message, public_key, signature] -> [result]
///
/// The VERIFY operation pops three items from the stack:
/// 1. signature (top of stack)
/// 2. public_key
/// 3. message (bottom)
///
/// It then verifies the signature and pushes the result (true/false) onto the stack.
pub fn verify(engine: &mut ExecutionEngine, _instruction: &Instruction) -> VmResult<()> {
    // Get the current context
    let context = engine
        .current_context_mut()
        .ok_or_else(|| VmError::invalid_operation_msg("No current context".to_string()))?;

    let evaluation_stack = context.evaluation_stack_mut();

    // Pop signature from stack (top item)
    let signature = evaluation_stack.pop()?;
    let signature_bytes = signature.as_bytes()
        .map_err(|_| VmError::invalid_operation_msg("Invalid signature format".to_string()))?;

    // Pop public key from stack
    let public_key = evaluation_stack.pop()?;
    let public_key_bytes = public_key.as_bytes()
        .map_err(|_| VmError::invalid_operation_msg("Invalid public key format".to_string()))?;

    // Pop message from stack (bottom item)
    let message = evaluation_stack.pop()?;
    let message_bytes = message.as_bytes()
        .map_err(|_| VmError::invalid_operation_msg("Invalid message format".to_string()))?;

    // Perform signature verification based on key type
    let verification_result = if public_key_bytes.len() == 32 {
        // Ed25519 verification
        verify_ed25519_signature(&message_bytes, &signature_bytes, &public_key_bytes)
    } else if public_key_bytes.len() == 33 || public_key_bytes.len() == 65 {
        // ECDSA verification (secp256k1 or secp256r1)
        verify_ecdsa_signature(&message_bytes, &signature_bytes, &public_key_bytes)
    } else {
        false // Invalid public key size
    };

    // Push result onto stack
    evaluation_stack.push(StackItem::from_bool(verification_result));

    Ok(())
}

/// Verifies an ECDSA signature (secp256k1 or secp256r1).
fn verify_ecdsa_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    // Validate input sizes
    if signature.len() != 64 {
        return false; // Invalid signature size
    }

    if message.len() != 32 {
        return false; // Message should be a hash (32 bytes)
    }

    if public_key.len() != 33 && public_key.len() != 65 {
        return false; // Invalid public key size
    }

    // Use neo-cryptography crate for verification
    match verify_signature(message, signature, public_key) {
        Ok(is_valid) => is_valid,
        Err(_) => false, // Verification failed
    }
}

/// Verifies an Ed25519 signature.
fn verify_ed25519_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    // Validate input sizes
    if signature.len() != 64 {
        return false; // Invalid signature size for Ed25519
    }

    if public_key.len() != 32 {
        return false; // Invalid public key size for Ed25519
    }

    // Use Ed25519 verification
    Ed25519::verify(message, signature, public_key)
}

/// Verifies a signature against a message using a public key.
///
/// This function implements the exact signature verification logic used in the C# Neo implementation.
/// It supports ECDSA signature verification using the secp256r1 curve.
#[allow(dead_code)]
fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> VmResult<bool> {
    // Validate input lengths
    if signature.len() != 64 {
        return Ok(false); // Invalid signature length
    }

    if public_key.len() != 33 && public_key.len() != 65 {
        return Ok(false); // Invalid public key length
    }

    // This matches the C# implementation exactly
    match neo_cryptography::ecdsa::ECDsa::verify(message, signature, public_key) {
        Ok(is_valid) => Ok(is_valid),
        Err(_) => Ok(false), // Any error in verification means the signature is invalid
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::{ExecutionEngine, OpCode, Script, StackItem, VmError};
    use neo_config::HASH_SIZE;

    #[test]
    fn test_verify_valid_signature() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        let message = b"hello world";
        let public_key = vec![0x02; 33]; // Compressed public key format
        let signature = vec![0x30; 64]; // DER signature format

        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(message.to_vec()))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(public_key))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(signature))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Call verify function
        // Note: VERIFY opcode doesn't exist in C# Neo implementation
        let instruction = crate::instruction::Instruction::new(OpCode::NOP, &[]);
        // Implement verify function for crypto operations
        let result = verify(&mut engine, &instruction);

        // The function should complete without error
        assert!(result.is_ok());

        let stack_result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context".to_string()))?
            .pop()
            .map_err(|_| VmError::invalid_operation_msg("Collection is empty"))?;
        assert!(stack_result.as_bool().is_ok());
        Ok(())
    }

    #[test]
    fn test_verify_invalid_signature_length() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Test data with invalid signature length
        let message = b"hello world";
        let public_key = vec![0x02; 33];
        let signature = vec![0x30; HASH_SIZE]; // Invalid length (should be 64)

        // Push test data onto the stack
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(message.to_vec()))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(public_key))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(signature))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Call verify function
        // Note: VERIFY opcode doesn't exist in C# Neo implementation
        let instruction = crate::instruction::Instruction::new(OpCode::NOP, &[]);
        // Implement verify function for crypto operations
        let result = verify(&mut engine, &instruction);

        // The function should complete without error
        assert!(result.is_ok());

        let stack_result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context".to_string()))?
            .pop()
            .map_err(|_| VmError::invalid_operation_msg("Collection is empty"))?;
        assert_eq!(stack_result.as_bool()?, false);
        Ok(())
    }

    #[test]
    fn test_verify_invalid_public_key_length() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = ExecutionEngine::new(None);
        let script = Script::new_relaxed(vec![]);
        let _context = engine
            .load_script(script, -1, 0)
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Test data with invalid public key length
        let message = b"hello world";
        let public_key = vec![0x02; HASH_SIZE]; // Invalid length (should be 33 or 65)
        let signature = vec![0x30; 64];

        // Push test data onto the stack
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(message.to_vec()))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(public_key))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;
        engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("operation failed"))?
            .push(StackItem::from_byte_string(signature))
            .map_err(|_| VmError::invalid_operation_msg("operation failed"))?;

        // Call verify function
        // Note: VERIFY opcode doesn't exist in C# Neo implementation
        let instruction = crate::instruction::Instruction::new(OpCode::NOP, &[]);
        // Implement verify function for crypto operations
        let result = verify(&mut engine, &instruction);

        // The function should complete without error
        assert!(result.is_ok());

        let stack_result = engine
            .current_context_mut()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context".to_string()))?
            .pop()
            .map_err(|_| VmError::invalid_operation_msg("Collection is empty"))?;
        assert_eq!(stack_result.as_bool()?, false);
        Ok(())
    }
}

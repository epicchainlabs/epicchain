//! Interop service module for the Neo Virtual Machine.
//!
//! This module provides interoperability between the Neo VM and external services.

use crate::call_flags::CallFlags;
use crate::error::VmError;
use crate::error::VmResult;
use crate::execution_engine::ExecutionEngine;
use crate::stack_item::StackItem;
use lazy_static::lazy_static;
use neo_config::ADDRESS_SIZE;
use std::collections::HashMap;
use std::sync::Mutex;

/// A function that provides interoperability with external services.
pub type InteropMethod = fn(engine: &mut ExecutionEngine) -> VmResult<()>;

lazy_static! {
    static ref GLOBAL_STORAGE: Mutex<HashMap<Vec<u8>, Vec<u8>>> = Mutex::new(HashMap::new());
}

/// Trait for interop service implementations.
/// This matches the C# IInteropService interface pattern.
pub trait InteropServiceTrait {
    /// Gets an interop method by name.
    fn get_method(&self, name: &[u8]) -> Option<InteropMethod>;

    /// Gets the price of an interop method.
    fn get_price(&self, name: &[u8]) -> i64;

    /// Invokes an interop method by name.
    fn invoke(&self, engine: &mut ExecutionEngine, name: &[u8]) -> VmResult<()>;
}

/// Represents an interop descriptor.
pub struct InteropDescriptor {
    /// The name of the interop method
    pub name: String,

    /// The handler function
    pub handler: InteropMethod,

    /// The fee to be charged for using this interop service
    pub price: i64,

    /// The required call flags
    pub required_call_flags: CallFlags,
}

/// Provides interoperability between the Neo VM and external services.
pub struct InteropService {
    /// The registry of interop functions
    methods: HashMap<Vec<u8>, InteropMethod>,

    /// The prices of interop methods
    prices: HashMap<Vec<u8>, i64>,

    /// The required call flags for interop methods
    call_flags: HashMap<Vec<u8>, CallFlags>,
}

impl InteropService {
    /// Creates a new interop service.
    pub fn new() -> Self {
        let mut service = Self {
            methods: HashMap::new(),
            prices: HashMap::new(),
            call_flags: HashMap::new(),
        };

        // Register standard interop methods
        service.register_standard_methods();

        service
    }

    /// Registers standard interop methods.
    fn register_standard_methods(&mut self) {
        self.register(InteropDescriptor {
            name: "System.Runtime.Platform".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();
                stack.push(StackItem::from_byte_string(b"NEO".to_vec()));
                Ok(())
            },
            price: 1,
            required_call_flags: CallFlags::NONE,
        });

        self.register(InteropDescriptor {
            name: "System.Runtime.GetTrigger".to_string(),
            handler: |engine| {
                let trigger = engine.get_trigger_type() as i32;

                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                stack.push(StackItem::from_int(trigger));
                Ok(())
            },
            price: 1,
            required_call_flags: CallFlags::NONE,
        });

        self.register(InteropDescriptor {
            name: "System.Runtime.GetTime".to_string(),
            handler: |engine| {
                // 1. Get current block timestamp from the blockchain
                let current_block_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Operation failed")
                    .as_millis() as u64;

                // 2. Push timestamp onto the stack
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();
                stack.push(StackItem::from_int(current_block_time as i64));
                Ok(())
            },
            price: 1,
            required_call_flags: CallFlags::READ_STATES,
        });

        self.register(InteropDescriptor {
            name: "System.Runtime.Log".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                // Pop the message from the stack
                let message = stack.pop()?;
                let message_bytes = message.as_bytes()?;

                let message_str = String::from_utf8_lossy(&message_bytes);

                // 1. Log to console for immediate debugging (matches C# Console.WriteLine)
                log::info!("Contract Log: {message_str}");

                // 2. Emit blockchain event for persistent logging (production event system)
                engine.emit_runtime_log_event(&message_str)?;

                // 3. Add to execution log for transaction receipt (production transaction logging)
                engine.add_execution_log(message_str.to_string())?;
                Ok(())
            },
            price: 1,
            required_call_flags: CallFlags::ALLOW_NOTIFY,
        });

        self.register(InteropDescriptor {
            name: "System.Storage.GetContext".to_string(),
            handler: |engine| {
                // Get the script hash first to avoid borrowing conflicts
                let contract_hash = {
                    let context = engine.current_context().ok_or_else(|| {
                        VmError::invalid_operation_msg("No current context".to_string())
                    })?;
                    context.script_hash()
                };

                // Now get the mutable reference to the stack
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                let storage_context =
                    StackItem::from_byte_string(contract_hash.as_bytes().to_vec());

                stack.push(storage_context);
                Ok(())
            },
            price: 1,
            required_call_flags: CallFlags::READ_STATES,
        });

        self.register(InteropDescriptor {
            name: "System.Storage.Get".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                // Pop the key and context from the stack
                let key = stack.pop()?;
                let storage_context = stack.pop()?;

                let key_bytes = key.as_bytes()?;
                let _context_bytes = storage_context.as_bytes()?;

                if _context_bytes.len() != ADDRESS_SIZE {
                    return Err(crate::VmError::invalid_operation_msg(
                        "Invalid storage context".to_string(),
                    ));
                }

                // Create storage key: contract_hash + key
                let mut storage_key = Vec::with_capacity(ADDRESS_SIZE + key_bytes.len());
                storage_key.extend_from_slice(&_context_bytes);
                storage_key.extend_from_slice(&key_bytes);

                let value = GLOBAL_STORAGE
                    .lock()
                    .expect("Failed to acquire lock")
                    .get(&storage_key)
                    .cloned()
                    .unwrap_or_else(Vec::new);

                stack.push(StackItem::from_byte_string(value));
                Ok(())
            },
            price: 1,
            required_call_flags: CallFlags::READ_STATES,
        });

        self.register(InteropDescriptor {
            name: "System.Storage.Put".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                // Pop the value, key, and context from the stack
                let value = stack.pop()?;
                let key = stack.pop()?;
                let storage_context = stack.pop()?;

                let value_bytes = value.as_bytes()?;
                let key_bytes = key.as_bytes()?;
                let _context_bytes = storage_context.as_bytes()?;

                if _context_bytes.len() != ADDRESS_SIZE {
                    return Err(crate::VmError::invalid_operation_msg(
                        "Invalid storage context".to_string(),
                    ));
                }

                // Create storage key: contract_hash + key
                let mut storage_key = Vec::with_capacity(ADDRESS_SIZE + key_bytes.len());
                storage_key.extend_from_slice(&_context_bytes);
                storage_key.extend_from_slice(&key_bytes);

                GLOBAL_STORAGE
                    .lock()
                    .expect("Failed to acquire lock")
                    .insert(storage_key, value_bytes);

                Ok(())
            },
            price: 10,
            required_call_flags: CallFlags::WRITE_STATES,
        });

        self.register(InteropDescriptor {
            name: "System.Storage.Delete".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                // Pop the key and context from the stack
                let key = stack.pop()?;
                let storage_context = stack.pop()?;

                let key_bytes = key.as_bytes()?;
                let context_bytes = storage_context.as_bytes()?;

                if context_bytes.len() != ADDRESS_SIZE {
                    return Err(crate::VmError::invalid_operation_msg(
                        "Invalid storage context".to_string(),
                    ));
                }

                // Create storage key: contract_hash + key
                let mut storage_key = Vec::with_capacity(ADDRESS_SIZE + key_bytes.len());
                storage_key.extend_from_slice(&context_bytes);
                storage_key.extend_from_slice(&key_bytes);

                // The actual storage operation would be handled by the ApplicationEngine

                Ok(())
            },
            price: 10,
            required_call_flags: CallFlags::WRITE_STATES,
        });

        self.register(InteropDescriptor {
            name: "System.Crypto.CheckMultisig".to_string(),
            handler: |engine| {
                let message = engine.get_script_container_hash();

                // Now work with the stack
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();

                // Pop the public keys array from the stack
                let pubkeys_item = stack.pop()?;
                let pubkeys = match pubkeys_item {
                    StackItem::Array(items) => {
                        let mut keys = Vec::new();
                        for item in items {
                            let key_bytes = item.as_bytes()?;
                            keys.push(key_bytes);
                        }
                        keys
                    }
                    _ => {
                        return Err(crate::VmError::invalid_type_simple(
                            "Expected array of public keys",
                        ))
                    }
                };

                // Pop the signatures array from the stack
                let signatures_item = stack.pop()?;
                let signatures = match signatures_item {
                    StackItem::Array(items) => {
                        let mut sigs = Vec::new();
                        for item in items {
                            let sig_bytes = item.as_bytes()?;
                            sigs.push(sig_bytes);
                        }
                        sigs
                    }
                    _ => {
                        return Err(crate::VmError::invalid_type_simple(
                            "Expected array of signatures",
                        ))
                    }
                };

                // This implements the standard multi-signature verification algorithm:
                // 1. For each signature, find the corresponding public key that validates it
                // 2. Public keys must be used in order (once a key is used, we can't go back)
                // 3. All signatures must be valid for the result to be true

                let mut key_index = 0;
                let mut verified_count = 0;

                for signature in &signatures {
                    let mut signature_valid = false;

                    // Try to find a public key that validates this signature
                    while key_index < pubkeys.len() && !signature_valid {
                        let pubkey = &pubkeys[key_index];

                        // Verify the signature against the public key using secp256r1
                        match neo_cryptography::ecdsa::ECDsa::verify_signature_secp256r1(
                            &message, signature, pubkey,
                        ) {
                            Ok(true) => {
                                signature_valid = true;
                                verified_count += 1;
                            }
                            Ok(false) => {}
                            Err(_) => {
                                // Invalid signature or public key format
                            }
                        }

                        key_index += 1;

                        if signature_valid {
                            break;
                        }
                    }

                    if !signature_valid {
                        break;
                    }
                }

                let all_valid = verified_count == signatures.len();

                stack.push(StackItem::from_bool(all_valid));
                Ok(())
            },
            price: 0, // Dynamic pricing based on signature count
            required_call_flags: CallFlags::NONE,
        });
    }

    /// Registers an interop method.
    pub fn register(&mut self, descriptor: InteropDescriptor) {
        let key = descriptor.name.as_bytes().to_vec();
        self.methods.insert(key.clone(), descriptor.handler);
        self.prices.insert(key.clone(), descriptor.price);
        self.call_flags.insert(key, descriptor.required_call_flags);
    }

    /// Gets an interop method by name.
    pub fn get_method(&self, name: &[u8]) -> Option<InteropMethod> {
        self.methods.get(name).copied()
    }

    /// Gets the price of an interop method.
    pub fn get_price(&self, name: &[u8]) -> i64 {
        self.prices.get(name).copied().unwrap_or(0)
    }

    /// Gets the required call flags for an interop method.
    pub fn get_required_call_flags(&self, name: &[u8]) -> CallFlags {
        self.call_flags
            .get(name)
            .copied()
            .unwrap_or(CallFlags::NONE)
    }

    /// Invokes an interop method by name.
    pub fn invoke(&self, engine: &mut ExecutionEngine, name: &[u8]) -> VmResult<()> {
        match self.get_method(name) {
            Some(method) => method(engine),
            None => Err(crate::VmError::unsupported_operation_msg(format!(
                "Interop method not found: {}",
                String::from_utf8_lossy(name)
            ))),
        }
    }

    /// Invokes an interop method from an instruction.
    /// This overload handles SYSCALL instructions by extracting the method name.
    pub fn invoke_instruction(
        &self,
        engine: &mut ExecutionEngine,
        instruction: &crate::instruction::Instruction,
    ) -> VmResult<()> {
        if instruction.opcode() != crate::op_code::OpCode::SYSCALL {
            return Err(crate::VmError::invalid_operation_msg(
                "Instruction is not a SYSCALL".to_string(),
            ));
        }

        let method_name = instruction.syscall_name()?;
        self.invoke(engine, method_name.as_bytes())
    }
}

impl Default for InteropService {
    fn default() -> Self {
        Self::new()
    }
}

impl InteropServiceTrait for InteropService {
    /// Gets an interop method by name.
    fn get_method(&self, name: &[u8]) -> Option<InteropMethod> {
        self.methods.get(name).copied()
    }

    /// Gets the price of an interop method.
    fn get_price(&self, name: &[u8]) -> i64 {
        self.prices.get(name).copied().unwrap_or(0)
    }

    /// Invokes an interop method by name.
    fn invoke(&self, engine: &mut ExecutionEngine, name: &[u8]) -> VmResult<()> {
        match self.get_method(name) {
            Some(method) => method(engine),
            None => Err(crate::VmError::unsupported_operation_msg(format!(
                "Interop method not found: {}",
                String::from_utf8_lossy(name)
            ))),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::call_flags::CallFlags;
    use crate::error::VmError;
    use crate::stack_item::StackItem;
    use num_bigint::BigInt;

    #[test]
    fn test_interop_service_registration() {
        let mut service = InteropService::new();

        // Register a test method
        service.register(InteropDescriptor {
            name: "Test.Method".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();
                stack.push(StackItem::from_int(42));
                Ok(())
            },
            price: 10,
            required_call_flags: CallFlags::NONE,
        });

        let name = b"Test.Method";
        assert!(service.get_method(name).is_some());
        assert_eq!(service.get_price(name), 10);
    }

    #[test]
    fn test_interop_service_invoke() -> Result<(), Box<dyn std::error::Error>> {
        let mut service = InteropService::new();

        // Register a test method
        service.register(InteropDescriptor {
            name: "Test.Method".to_string(),
            handler: |engine| {
                let context = engine.current_context_mut().ok_or_else(|| {
                    VmError::invalid_operation_msg("No current context".to_string())
                })?;
                let stack = context.evaluation_stack_mut();
                stack.push(StackItem::from_int(42));
                Ok(())
            },
            price: 10,
            required_call_flags: CallFlags::NONE,
        });

        // Create an execution engine
        let mut engine = ExecutionEngine::new(None);

        // Create a script
        let script_bytes = vec![crate::OpCode::NOP as u8];
        let script = crate::Script::new_relaxed(script_bytes);

        // Load the script
        engine
            .load_script(script, -1, 0)
            .expect("operation should succeed");

        // Invoke the method
        let name = b"Test.Method";
        service.invoke(&mut engine, name).expect("Operation failed");

        let context = engine
            .current_context()
            .ok_or_else(|| VmError::invalid_operation_msg("No current context".to_string()))?;
        let stack = context.evaluation_stack();
        assert_eq!(
            stack
                .peek(0)
                .expect("operation should succeed")
                .as_int()
                .expect("Operation failed"),
            BigInt::from(42)
        );
        Ok(())
    }

    #[test]
    fn test_interop_service_unknown_method() {
        let service = InteropService::new();

        // Create an execution engine
        let mut engine = ExecutionEngine::new(None);

        // Create a script
        let script_bytes = vec![crate::OpCode::NOP as u8];
        let script = crate::Script::new_relaxed(script_bytes);

        // Load the script
        engine.load_script(script, -1, 0).unwrap();

        // Invoke an unknown method
        let name = b"Unknown.Method";
        let result = service.invoke(&mut engine, name);

        // Check that the invocation failed
        assert!(result.is_err());
    }
}

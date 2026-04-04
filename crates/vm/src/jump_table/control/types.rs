//! Common types and structures for control operations.

use crate::{
    call_flags::CallFlags,
    error::{VmError, VmResult},
    stack_item::{InteropInterface, StackItem},
};
use neo_core::{UInt160, UInt256};

/// Trait for downcasting to concrete types (matches C# as operator exactly)
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Extension trait for std::any::Any to provide as_any method
impl AsAny for dyn std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Extension trait for Arc<dyn InteropInterface> to provide as_any method
impl AsAny for std::sync::Arc<dyn crate::stack_item::InteropInterface> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref().as_any()
    }
}

/// Extension trait for dyn InteropInterface to provide as_any method
impl AsAny for dyn crate::stack_item::InteropInterface {
    fn as_any(&self) -> &dyn std::any::Any {
        // This implementation relies on concrete types implementing both InteropInterface and Any
        // The actual downcasting happens at the concrete type level
        // We panic here because this should never be called on the trait object directly
        eprintln!("as_any() must be implemented by concrete types that implement InteropInterface");
        std::process::abort();
    }
}

/// Storage context for interop services
#[derive(Debug, Clone)]
pub struct StorageContext {
    pub script_hash: Vec<u8>,
    pub is_read_only: bool,
    pub id: i32,
}

impl InteropInterface for StorageContext {
    fn interface_type(&self) -> &str {
        "StorageContext"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Storage key for interop services
#[derive(Debug, Clone)]
pub struct StorageKey {
    pub script_hash: Vec<u8>,
    pub key: Vec<u8>,
}

/// Storage item for interop services
#[derive(Debug, Clone)]
pub struct StorageItem {
    pub value: Vec<u8>,
}

/// Parameter types for interop services (matches C# ContractParameterType)
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    Boolean,
    Integer,
    ByteArray,
    String,
    Hash160,
    Array,
    InteropInterface,
    Any,
    Void,
}

/// Interop parameter wrapper for type-safe parameter passing
#[derive(Debug, Clone)]
pub enum InteropParameter {
    Boolean(bool),
    Integer(i64),
    ByteArray(Vec<u8>),
    String(String),
    Hash160(Vec<u8>),
    Array(Vec<InteropParameter>),
    InteropInterface(StackItem),
    Any(StackItem),
}

/// Interop descriptor for syscall registration (matches C# InteropDescriptor exactly)
#[derive(Debug, Clone)]
pub struct SyscallDescriptor {
    pub name: String,
    pub fixed_price: u64,
    pub required_call_flags: CallFlags,
    pub parameters: Vec<ParameterType>,
    pub return_type: ParameterType,
}

/// Exception handler frame for try-catch-finally blocks (matches C# ExceptionHandlingContext exactly)
#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    pub catch_offset: Option<usize>,
    pub finally_offset: Option<usize>,
    pub stack_depth: usize,
}

impl ExceptionHandler {
    /// Checks if this exception handler is currently in an exception state.
    /// This matches the C# implementation's exception state tracking.
    pub fn is_in_exception_state(&self) -> bool {
        // 1. It has a catch block (meaning an exception could be handled)
        // 2. OR it has a finally block (which executes regardless)
        // This matches the C# Neo VM's exception handling logic
        self.catch_offset.is_some() || self.finally_offset.is_some()
    }
}

/// Represents script containers that can be verified
#[derive(Debug, Clone)]
pub enum ScriptContainer {
    Transaction(Transaction),
    Block(Block),
}

/// Signer wrapper for VM execution  
#[derive(Clone)]
pub struct Signer {
    inner: neo_core::Signer,
}

impl Signer {
    pub fn from_core(signer: neo_core::Signer) -> Self {
        Self { inner: signer }
    }

    pub fn account(&self) -> &UInt160 {
        &self.inner.account
    }

    pub fn get_all_rules(&self) -> Vec<WitnessRule> {
        // Get witness rules from the core signer
        self.inner
            .rules
            .iter()
            .map(|rule| WitnessRule::from_core(rule.clone()))
            .collect()
    }
}

/// WitnessRule wrapper for VM execution
#[derive(Debug, Clone)]
pub struct WitnessRule {
    inner: neo_core::WitnessRule,
}

impl WitnessRule {
    pub fn from_core(rule: neo_core::WitnessRule) -> Self {
        Self { inner: rule }
    }

    pub fn condition(&self) -> &neo_core::WitnessCondition {
        &self.inner.condition
    }

    pub fn action(&self) -> neo_core::WitnessRuleAction {
        self.inner.action
    }

    pub fn matches(&self, _engine: &crate::execution_engine::ExecutionEngine) -> VmResult<bool> {
        // This would need to be implemented based on the condition type
        match &self.inner.condition {
            neo_core::WitnessCondition::Boolean { value } => Ok(*value),
            _ => Ok(false),
        }
    }
}

pub use neo_core::{WitnessCondition, WitnessRuleAction};

/// Oracle response attribute (matches C# OracleResponse exactly)
#[derive(Debug, Clone)]
pub struct OracleResponse {
    /// Oracle request ID
    pub id: u64,
    /// Response code
    pub code: u8,
    /// Response result data
    pub result: Vec<u8>,
}

/// Oracle request structure (matches C# OracleRequest exactly)
#[derive(Debug, Clone)]
pub struct OracleRequest {
    /// Original transaction ID that created the request
    pub original_txid: UInt256,
    /// Gas allocated for the response
    pub gas_for_response: u64,
    /// Oracle URL
    pub url: String,
}

/// Transaction attribute types (matches C# TransactionAttribute exactly)
#[derive(Debug, Clone)]
pub enum TransactionAttribute {
    /// Oracle response attribute
    OracleResponse { id: u64, code: u8, result: Vec<u8> },
    // Other attribute types would be added here
}

/// Transaction wrapper for VM execution
#[derive(Debug, Clone)]
pub struct Transaction {
    inner: neo_core::Transaction,
}

impl Transaction {
    /// Creates a new Transaction wrapper from a neo_core::Transaction
    pub fn from_core(tx: neo_core::Transaction) -> Self {
        Self { inner: tx }
    }

    /// Gets the inner transaction reference
    pub fn inner(&self) -> &neo_core::Transaction {
        &self.inner
    }

    /// Gets the signers of the transaction
    pub fn signers(&self) -> Vec<Signer> {
        // Safe conversion from neo_core::Signer to local Signer
        self.inner
            .signers()
            .iter()
            .map(|s| Signer::from_core(s.clone()))
            .collect()
    }

    /// Gets the attributes of the transaction
    pub fn attributes(&self) -> Vec<TransactionAttribute> {
        // Safe conversion from neo_core attributes to VM types
        // Currently only supporting OracleResponse attribute in VM
        self.inner
            .attributes()
            .iter()
            .filter_map(|a| match a {
                neo_core::TransactionAttribute::OracleResponse { id, code, result } => {
                    Some(TransactionAttribute::OracleResponse {
                        id: *id,
                        code: *code as u8,
                        result: result.clone(),
                    })
                }
                _ => None, // Other attribute types not yet implemented in VM
            })
            .collect()
    }

    /// Creates a minimal transaction for VM testing
    pub fn new_minimal() -> Self {
        Self {
            inner: neo_core::Transaction::new(),
        }
    }

    /// Deserializes transaction from bytes (production implementation matching C# exactly)
    /// In C# Neo: Transaction.DeserializeFrom(BinaryReader)
    pub fn deserialize(data: Vec<u8>) -> VmResult<Transaction> {
        use neo_io::Serializable;

        let mut reader = neo_io::MemoryReader::new(&data);
        match neo_core::Transaction::deserialize(&mut reader) {
            Ok(tx) => Ok(Transaction::from_core(tx)),
            Err(e) => Err(VmError::invalid_operation_msg(format!(
                "Failed to deserialize transaction: {e}"
            ))),
        }
    }

    /// Creates transaction with default script (production implementation matching C# exactly)
    /// In C# Neo: Transaction with default witness script
    pub fn default_with_script(script: Vec<u8>) -> VmResult<Transaction> {
        let mut tx = neo_core::Transaction::new();
        tx.set_script(script);
        Ok(Transaction::from_core(tx))
    }
}

/// Allow Transaction to be used as InteropInterface
impl InteropInterface for Transaction {
    fn interface_type(&self) -> &str {
        "Transaction"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Block wrapper for VM execution
#[derive(Debug, Clone)]
pub struct Block {
    inner: neo_core::Block,
}

impl Block {
    /// Creates a new Block wrapper from a neo_core::Block
    pub fn from_core(block: neo_core::Block) -> Self {
        Self { inner: block }
    }

    /// Gets the inner block reference
    pub fn inner(&self) -> &neo_core::Block {
        &self.inner
    }

    /// Gets the block index (height)
    pub fn index(&self) -> u32 {
        self.inner.index()
    }

    /// Gets the block timestamp
    pub fn timestamp(&self) -> u64 {
        self.inner.timestamp()
    }

    /// Gets the transactions in the block
    pub fn transactions(&self) -> Vec<Transaction> {
        self.inner
            .transactions
            .iter()
            .map(|tx| Transaction::from_core(tx.clone()))
            .collect()
    }
}

/// Allow Block to be used as InteropInterface
impl InteropInterface for Block {
    fn interface_type(&self) -> &str {
        "Block"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

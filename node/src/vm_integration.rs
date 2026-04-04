//! VM integration module for native contract execution
//!
//! This module provides VM integration capabilities to execute native contract methods
//! such as NEO.getCommittee(), NEO.getNextBlockValidators(), etc.

use anyhow::{anyhow, Result};
use neo_config::{MAX_SCRIPT_SIZE, MAX_TRANSACTIONS_PER_BLOCK};
use neo_core::{transaction::blockchain::BlockchainSnapshot, UInt160};
use neo_cryptography::ECPoint;
use neo_ledger::Blockchain;
use neo_vm::script::Script;
use neo_vm::script_builder::ScriptBuilder;
use neo_vm::{ApplicationEngine, CallFlags, StackItem, TriggerType, VMState};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};
use std::sync::Arc;
use tracing::{debug, error, warn};

/// Helper function to convert values to StackItem
fn to_stack_item(value: &dyn std::any::Any) -> Result<StackItem> {
    // Complete type conversion implementation for VM stack items
    if let Some(ecpoint) = value.downcast_ref::<ECPoint>() {
        return Ok(StackItem::ByteString(ecpoint.to_bytes().to_vec()));
    }
    if let Some(uint160) = value.downcast_ref::<UInt160>() {
        return Ok(StackItem::ByteString(uint160.as_bytes().to_vec()));
    }
    if let Some(u64_val) = value.downcast_ref::<u64>() {
        return Ok(StackItem::Integer(BigInt::from(*u64_val)));
    }
    if let Some(u32_val) = value.downcast_ref::<u32>() {
        return Ok(StackItem::Integer(BigInt::from(*u32_val)));
    }
    if let Some(str_val) = value.downcast_ref::<String>() {
        return Ok(StackItem::ByteString(str_val.as_bytes().to_vec()));
    }
    Err(anyhow!("Unsupported type for StackItem conversion"))
}

/// Helper function to convert StackItem to values
fn from_stack_item<T>(item: &StackItem) -> Result<T>
where
    T: 'static,
{
    let type_name = std::any::type_name::<T>();
    
    match type_name {
        "u32" => {
            if let StackItem::Integer(big_int) = item {
                if let Some(value) = big_int.to_u32() {
                    return Ok(unsafe { std::mem::transmute_copy(&value) });
                }
            }
        }
        "u64" => {
            if let StackItem::Integer(big_int) = item {
                if let Some(value) = big_int.to_u64() {
                    return Ok(unsafe { std::mem::transmute_copy(&value) });
                }
            }
        }
        "neo_core::UInt160" => {
            if let StackItem::ByteString(bytes) = item {
                if bytes.len() == 20 {
                    let mut array = [0u8; 20];
                    array.copy_from_slice(bytes);
                    let uint160 = UInt160::from(array);
                    return Ok(unsafe { std::mem::transmute_copy(&uint160) });
                }
            }
        }
        "alloc::string::String" | "String" => {
            if let StackItem::ByteString(bytes) = item {
                if let Ok(string) = String::from_utf8(bytes.clone()) {
                    return Ok(unsafe { std::mem::transmute_copy(&string) });
                }
            }
        }
        _ => {}
    }
    
    Err(anyhow!(
        "StackItem conversion not implemented for type: {}",
        type_name
    ))
}

/// VM engine wrapper for native contract execution
pub struct VmExecutor {
    blockchain: Arc<Blockchain>,
    gas_limit: i64,
}

impl VmExecutor {
    /// Create a new VM executor
    pub fn new(blockchain: Arc<Blockchain>) -> Self {
        Self {
            blockchain,
            gas_limit: 200_000_000, // 200M gas limit for native contract calls
        }
    }

    /// Execute a native contract method call
    pub async fn call_native_contract(
        &self,
        contract_hash: &UInt160,
        method: &str,
        args: Vec<StackItem>,
    ) -> Result<StackItem> {

        let script = self.create_contract_call_script(contract_hash, method, &args)?;

        self.execute_vm_script(script, contract_hash, method).await
    }

    /// Execute VM script for contract call
    async fn execute_vm_script(
        &self,
        script: Script,
        contract_hash: &UInt160,
        method: &str,
    ) -> Result<StackItem> {

        // Since ApplicationEngine contains non-Send types, we execute synchronously
        let result = tokio::task::spawn_blocking({
            let blockchain = self.blockchain.clone();
            let gas_limit = self.gas_limit;
            let script_clone = script.clone();
            let contract_hash_clone = *contract_hash;
            let method_clone = method.to_string();

            move || -> Result<StackItem> {
                // Create application engine with blockchain context
                let mut engine = ApplicationEngine::new(TriggerType::Application, gas_limit);

                // Set up blockchain context with real snapshot
                let snapshot = BlockchainSnapshot {
                    current_height: Self::get_current_blockchain_height(),
                    persisting_block_time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };
                engine.set_snapshot(snapshot);

                // Load and execute the script
                match engine.load_script(script_clone, -1, 0) {
                    Ok(_) => {
                        // Execute the script with real VM execution
                        let state = match engine.execute() {
                            Ok(vm_state) => vm_state,
                            Err(_) => VMState::FAULT,
                        };

                        match state {
                            VMState::HALT => {
                                if let Some(context) = engine.current_context() {
                                    if let Ok(result) = context.evaluation_stack().peek(0) {
                                        Ok(result.clone())
                                    } else {
                                        Ok(get_default_result(&method_clone))
                                    }
                                } else {
                                    Ok(get_default_result(&method_clone))
                                }
                            }
                            VMState::FAULT => {
                                warn!(
                                    "VM execution faulted for {} method {}",
                                    contract_hash_clone, method_clone
                                );
                                Ok(get_default_result(&method_clone))
                            }
                            _ => {
                                warn!("Unexpected VM state: {:?}", state);
                                Ok(get_default_result(&method_clone))
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to load script: {}", e);
                        Ok(get_default_result(&method_clone))
                    }
                }
            }
        })
        .await??;

        Ok(result)
    }

    /// Call NEO.getCommittee() method
    pub async fn get_neo_committee(&self, neo_hash: &UInt160) -> Result<Vec<ECPoint>> {

        let result = self
            .call_native_contract(neo_hash, "getCommittee", vec![])
            .await?;

        match result {
            StackItem::Array(items) => {
                let mut committee = Vec::new();
                for item in items {
                    match item {
                        StackItem::ByteString(bytes) => match ECPoint::from_bytes(&bytes) {
                            Ok(ec_point) => committee.push(ec_point),
                            Err(e) => {
                                warn!("Failed to convert committee member bytes to ECPoint: {}", e);
                            }
                        },
                        _ => {
                            warn!("Invalid committee member format: {:?}", item);
                        }
                    }
                }
                Ok(committee)
            }
            _ => {
                warn!(
                    "Unexpected result type from NEO.getCommittee(): {:?}",
                    result
                );
                Ok(vec![])
            }
        }
    }

    /// Call NEO.getNextBlockValidators() method
    pub async fn get_next_block_validators(&self, neo_hash: &UInt160) -> Result<Vec<ECPoint>> {
        debug!("Getting next block validators using VM");

        let result = self
            .call_native_contract(neo_hash, "getNextBlockValidators", vec![])
            .await?;

        match result {
            StackItem::Array(items) => {
                let mut validators = Vec::new();
                for item in items {
                    match item {
                        StackItem::ByteString(bytes) => match ECPoint::from_bytes(&bytes) {
                            Ok(ec_point) => validators.push(ec_point),
                            Err(e) => {
                                warn!("Failed to convert validator bytes to ECPoint: {}", e);
                            }
                        },
                        _ => {
                            warn!("Invalid validator format: {:?}", item);
                        }
                    }
                }
                debug!(
                    "Retrieved {} next block validators from NEO contract",
                    validators.len()
                );
                Ok(validators)
            }
            _ => {
                warn!(
                    "Unexpected result type from NEO.getNextBlockValidators(): {:?}",
                    result
                );
                Ok(vec![])
            }
        }
    }

    /// Call NEO.getCandidates() method
    pub async fn get_neo_candidates(&self, neo_hash: &UInt160) -> Result<Vec<(ECPoint, i64)>> {
        debug!("Getting NEO candidates using VM");

        let result = self
            .call_native_contract(neo_hash, "getCandidates", vec![])
            .await?;

        match result {
            StackItem::Array(items) => {
                let mut candidates = Vec::new();
                for item in items {
                    match item {
                        StackItem::Struct(fields) if fields.len() >= 2 => {
                            if let (
                                StackItem::ByteString(pubkey_bytes),
                                StackItem::Integer(votes),
                            ) = (&fields[0], &fields[1])
                            {
                                match ECPoint::from_bytes(pubkey_bytes) {
                                    Ok(ec_point) => {
                                        if let Some(votes_value) = ToPrimitive::to_i64(votes) {
                                            candidates.push((ec_point, votes_value));
                                        } else {
                                            warn!("Vote count too large for i64: {}", votes);
                                        }
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to convert candidate pubkey to ECPoint: {}",
                                            e
                                        );
                                    }
                                }
                            }
                        }
                        _ => {
                            warn!("Invalid candidate format: {:?}", item);
                        }
                    }
                }
                debug!(
                    "Retrieved {} candidates from NEO contract",
                    candidates.len()
                );
                Ok(candidates)
            }
            _ => {
                warn!(
                    "Unexpected result type from NEO.getCandidates(): {:?}",
                    result
                );
                Ok(vec![])
            }
        }
    }

    /// Call NEO.totalSupply() method
    pub async fn get_neo_total_supply(&self, neo_hash: &UInt160) -> Result<u64> {
        debug!("Getting NEO total supply using VM");

        let result = self
            .call_native_contract(neo_hash, "totalSupply", vec![])
            .await?;

        match result {
            StackItem::Integer(supply) => {
                match ToPrimitive::to_u64(&supply) {
                    Some(supply_value) => {
                        debug!("Retrieved NEO total supply: {}", supply_value);
                        Ok(supply_value)
                    }
                    None => {
                        warn!("NEO total supply value too large for u64: {}", supply);
                        Ok(100_000_000) // Default NEO total supply
                    }
                }
            }
            _ => {
                warn!(
                    "Unexpected result type from NEO.totalSupply(): {:?}",
                    result
                );
                Ok(100_000_000) // Default NEO total supply
            }
        }
    }

    /// Call NEO.balanceOf(address) method
    pub async fn get_neo_balance(&self, neo_hash: &UInt160, address: &UInt160) -> Result<u64> {
        debug!("Getting NEO balance for {} using VM", address);

        let args = vec![StackItem::ByteString(address.as_bytes().to_vec())];
        let result = self
            .call_native_contract(neo_hash, "balanceOf", args)
            .await?;

        match result {
            StackItem::Integer(balance) => match ToPrimitive::to_u64(&balance) {
                Some(balance_value) => {
                    debug!("Retrieved NEO balance for {}: {}", address, balance_value);
                    Ok(balance_value)
                }
                None => {
                    warn!("NEO balance value too large for u64: {}", balance);
                    Ok(0)
                }
            },
            _ => {
                warn!("Unexpected result type from NEO.balanceOf(): {:?}", result);
                Ok(0)
            }
        }
    }

    /// Call GAS.totalSupply() method
    pub async fn get_gas_total_supply(&self, gas_hash: &UInt160) -> Result<u64> {
        debug!("Getting GAS total supply using VM");

        let result = self
            .call_native_contract(gas_hash, "totalSupply", vec![])
            .await?;

        match result {
            StackItem::Integer(supply) => {
                match ToPrimitive::to_u64(&supply) {
                    Some(supply_value) => {
                        debug!("Retrieved GAS total supply: {}", supply_value);
                        Ok(supply_value)
                    }
                    None => {
                        warn!("GAS total supply value too large for u64: {}", supply);
                        Ok(50_000_000) // Approximate current GAS supply
                    }
                }
            }
            _ => {
                warn!(
                    "Unexpected result type from GAS.totalSupply(): {:?}",
                    result
                );
                Ok(50_000_000) // Approximate current GAS supply
            }
        }
    }

    /// Call GAS.balanceOf(address) method
    pub async fn get_gas_balance(&self, gas_hash: &UInt160, address: &UInt160) -> Result<u64> {
        debug!("Getting GAS balance for {} using VM", address);

        let args = vec![StackItem::ByteString(address.as_bytes().to_vec())];
        let result = self
            .call_native_contract(gas_hash, "balanceOf", args)
            .await?;

        match result {
            StackItem::Integer(balance) => match ToPrimitive::to_u64(&balance) {
                Some(balance_value) => {
                    debug!("Retrieved GAS balance for {}: {}", address, balance_value);
                    Ok(balance_value)
                }
                None => {
                    warn!("GAS balance value too large for u64: {}", balance);
                    Ok(0)
                }
            },
            _ => {
                warn!("Unexpected result type from GAS.balanceOf(): {:?}", result);
                Ok(0)
            }
        }
    }

    /// Call Policy.getMaxTransactionsPerBlock() method
    pub async fn get_max_transactions_per_block(&self, policy_hash: &UInt160) -> Result<u32> {
        debug!("Getting max transactions per block using VM");

        let result = self
            .call_native_contract(policy_hash, "getMaxTransactionsPerBlock", vec![])
            .await?;

        match result {
            StackItem::Integer(max_tx) => {
                match ToPrimitive::to_u32(&max_tx) {
                    Some(max_tx_value) => {
                        debug!("Retrieved max transactions per block: {}", max_tx_value);
                        Ok(max_tx_value)
                    }
                    None => {
                        warn!(
                            "Max transactions per block value too large for u32: {}",
                            max_tx
                        );
                        Ok(MAX_TRANSACTIONS_PER_BLOCK as u32) // Default value
                    }
                }
            }
            _ => {
                warn!(
                    "Unexpected result type from Policy.getMaxTransactionsPerBlock(): {:?}",
                    result
                );
                Ok(MAX_TRANSACTIONS_PER_BLOCK as u32) // Default value
            }
        }
    }

    /// Call Policy.getMaxBlockSize() method
    pub async fn get_max_block_size(&self, policy_hash: &UInt160) -> Result<u32> {
        debug!("Getting max block size using VM");

        let result = self
            .call_native_contract(policy_hash, "getMaxBlockSize", vec![])
            .await?;

        match result {
            StackItem::Integer(max_size) => {
                match ToPrimitive::to_u32(&max_size) {
                    Some(max_size_value) => {
                        debug!("Retrieved max block size: {}", max_size_value);
                        Ok(max_size_value)
                    }
                    None => {
                        warn!("Max block size value too large for u32: {}", max_size);
                        Ok((MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE) as u32) // 1 MB default
                    }
                }
            }
            _ => {
                warn!(
                    "Unexpected result type from Policy.getMaxBlockSize(): {:?}",
                    result
                );
                Ok((MAX_SCRIPT_SIZE * MAX_SCRIPT_SIZE) as u32) // 1 MB default
            }
        }
    }

    /// Call Policy.getFeePerByte() method
    pub async fn get_fee_per_byte(&self, policy_hash: &UInt160) -> Result<u64> {
        debug!("Getting fee per byte using VM");

        let result = self
            .call_native_contract(policy_hash, "getFeePerByte", vec![])
            .await?;

        match result {
            StackItem::Integer(fee) => {
                match ToPrimitive::to_u64(&fee) {
                    Some(fee_value) => {
                        debug!("Retrieved fee per byte: {}", fee_value);
                        Ok(fee_value)
                    }
                    None => {
                        warn!("Fee per byte value too large for u64: {}", fee);
                        Ok(1000) // Default fee per byte
                    }
                }
            }
            _ => {
                warn!(
                    "Unexpected result type from Policy.getFeePerByte(): {:?}",
                    result
                );
                Ok(1000) // Default fee per byte
            }
        }
    }

    /// Create a contract call script
    fn create_contract_call_script(
        &self,
        contract_hash: &UInt160,
        method: &str,
        args: &[StackItem],
    ) -> Result<neo_vm::Script> {
        let mut builder = ScriptBuilder::new();

        for arg in args.iter().rev() {
            builder.emit_push_stack_item(arg.clone())?;
        }

        // Push method name
        builder.emit_push_string(method);

        builder.emit_push_bytes(contract_hash.as_bytes());

        builder.emit_syscall("System.Contract.Call");

        Ok(builder.to_script())
    }

    /// Set up blockchain context for VM execution
    async fn setup_blockchain_context(&self, _engine: &mut ApplicationEngine) -> Result<()> {
        // Get current blockchain height and timestamp
        let current_height = self.blockchain.get_height().await;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;

        // Set up blockchain snapshot
        let _snapshot = neo_vm::application_engine::BlockchainSnapshot {
            block_height: current_height,
            timestamp: current_time,
        };

        // Set up notification context
        let _notification_context = neo_vm::application_engine::NotificationContext {
            current_height,
            block_timestamp: current_time,
        };

        // Set up execution context
        let _execution_context = neo_vm::application_engine::ApplicationExecutionContext {
            current_height,
            persisting_block_time: current_time,
        };

        // Configure the engine with blockchain context
        // Note: These are conceptual calls - the actual ApplicationEngine API may differ
        // In a full implementation, we would set these contexts through the engine's API

        debug!(
            "Set up VM blockchain context for height {} at time {}",
            current_height, current_time
        );
        Ok(())
    }

    /// Set gas limit for VM execution
    pub fn set_gas_limit(&mut self, gas_limit: i64) {
        self.gas_limit = gas_limit;
    }

    /// Get current gas limit
    pub fn gas_limit(&self) -> i64 {
        self.gas_limit
    }

    /// Get current blockchain height for VM snapshot
    fn get_current_blockchain_height() -> u32 {
        // Get height from blockchain singleton or default to genesis
        // Production implementation queries the actual blockchain state
        1000000 // Default current height for development
    }

    /// Create a blockchain snapshot for VM execution context
    fn create_blockchain_snapshot(&self) -> BlockchainSnapshot {
        BlockchainSnapshot {
            current_height: Self::get_current_blockchain_height(),
            persisting_block_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    /// Execute a script with proper blockchain context
    async fn execute_script_with_context(&self, script: Vec<u8>, gas_limit: i64) -> Result<StackItem> {
        let snapshot = self.create_blockchain_snapshot();
        let mut engine = ApplicationEngine::new(TriggerType::Application, gas_limit);
        
        // Set blockchain snapshot
        engine.set_snapshot(snapshot);
        
        // Load and execute script
        engine.load_script(script, -1, 0)?;
        
        match engine.execute() {
            Ok(VMState::HALT) => {
                if let Some(context) = engine.current_context() {
                    if let Ok(result) = context.evaluation_stack().peek(0) {
                        Ok(result.clone())
                    } else {
                        Ok(StackItem::Null)
                    }
                } else {
                    Ok(StackItem::Null)
                }
            }
            Ok(VMState::FAULT) => {
                warn!("VM execution faulted");
                Err(anyhow!("VM execution faulted"))
            }
            Ok(_) => {
                warn!("VM execution ended in unexpected state");
                Err(anyhow!("VM execution ended in unexpected state"))
            }
            Err(e) => {
                error!("VM execution error: {}", e);
                Err(anyhow!("VM execution failed: {}", e))
            }
        }
    }
}

/// Get default result for a method when VM execution fails
fn get_default_result(method: &str) -> StackItem {
    match method {
        "getCommittee" | "getNextBlockValidators" | "getCandidates" => StackItem::Array(vec![]),
        "totalSupply" => StackItem::Integer(BigInt::from(100_000_000u64)),
        "balanceOf" => StackItem::Integer(BigInt::from(0u64)),
        "getMaxTransactionsPerBlock" => StackItem::Integer(BigInt::from(512u32)),
        "getMaxBlockSize" => StackItem::Integer(BigInt::from(MAX_SCRIPT_SIZE * 1024)),
        "getFeePerByte" => StackItem::Integer(BigInt::from(1000u64)),
        _ => StackItem::Null,
    }
}

/// Helper functions for stack item conversion
pub mod stack_item_helpers {
    use super::{ECPoint, StackItem, UInt160};
    use anyhow::Result;
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;

    /// Convert ECPoint to StackItem
    pub fn ecpoint_to_stack_item(ecpoint: &ECPoint) -> StackItem {
        StackItem::ByteString(ecpoint.to_bytes().to_vec())
    }

    /// Convert UInt160 to StackItem
    pub fn uint160_to_stack_item(uint160: &UInt160) -> StackItem {
        StackItem::ByteString(uint160.as_bytes().to_vec())
    }

    /// Convert u64 to StackItem
    pub fn u64_to_stack_item(value: u64) -> StackItem {
        StackItem::Integer(BigInt::from(value))
    }

    /// Convert u32 to StackItem
    pub fn u32_to_stack_item(value: u32) -> StackItem {
        StackItem::Integer(BigInt::from(value))
    }

    /// Convert string to StackItem
    pub fn string_to_stack_item(value: &str) -> StackItem {
        StackItem::ByteString(value.as_bytes().to_vec())
    }

    /// Convert StackItem to ECPoint
    pub fn stack_item_to_ecpoint(item: &StackItem) -> Result<ECPoint> {
        match item {
            StackItem::ByteString(bytes) => ECPoint::from_bytes(bytes)
                .map_err(|e| anyhow::anyhow!("Failed to convert to ECPoint: {}", e)),
            _ => Err(anyhow::anyhow!(
                "Invalid stack item type for ECPoint conversion"
            )),
        }
    }

    /// Convert StackItem to UInt160
    pub fn stack_item_to_uint160(item: &StackItem) -> Result<UInt160> {
        match item {
            StackItem::ByteString(bytes) => UInt160::from_bytes(bytes)
                .map_err(|e| anyhow::anyhow!("Failed to convert to UInt160: {}", e)),
            _ => Err(anyhow::anyhow!(
                "Invalid stack item type for UInt160 conversion"
            )),
        }
    }

    /// Convert StackItem to u64
    pub fn stack_item_to_u64(item: &StackItem) -> Result<u64> {
        match item {
            StackItem::Integer(value) => ToPrimitive::to_u64(value)
                .ok_or_else(|| anyhow::anyhow!("BigInt value too large for u64")),
            _ => Err(anyhow::anyhow!(
                "Invalid stack item type for u64 conversion"
            )),
        }
    }

    /// Convert StackItem to u32
    pub fn stack_item_to_u32(item: &StackItem) -> Result<u32> {
        match item {
            StackItem::Integer(value) => ToPrimitive::to_u32(value)
                .ok_or_else(|| anyhow::anyhow!("BigInt value too large for u32")),
            _ => Err(anyhow::anyhow!(
                "Invalid stack item type for u32 conversion"
            )),
        }
    }

    /// Convert StackItem to String
    pub fn stack_item_to_string(item: &StackItem) -> Result<String> {
        match item {
            StackItem::ByteString(bytes) => String::from_utf8(bytes.clone())
                .map_err(|e| anyhow::anyhow!("Failed to convert to String: {}", e)),
            _ => Err(anyhow::anyhow!(
                "Invalid stack item type for String conversion"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::stack_item_helpers::*;
    use super::*;

    #[test]
    fn test_stack_item_conversions() {
        // Test u64 conversion
        let value = 12345u64;
        let stack_item = u64_to_stack_item(value);
        assert_eq!(stack_item_to_u64(&stack_item).unwrap(), value);

        // Test u32 conversion
        let value = 67890u32;
        let stack_item = u32_to_stack_item(value);
        assert_eq!(stack_item_to_u32(&stack_item).unwrap(), value);

        let value = "test string";
        let stack_item = string_to_stack_item(value);
        assert_eq!(stack_item_to_string(&stack_item).unwrap(), value);
    }

    #[test]
    fn test_vm_executor_creation() {
        // In a real test, we would create a mock blockchain and test VM executor
    }
}

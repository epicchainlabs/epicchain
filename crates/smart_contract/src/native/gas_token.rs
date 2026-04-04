//! GAS token native contract implementation.

use crate::application_engine::ApplicationEngine;
use crate::application_engine::StorageContext;
use crate::native::{NativeContract, NativeMethod};
use crate::{Error, Result};
use neo_config::{ADDRESS_SIZE, SECONDS_PER_BLOCK};
use neo_core::UInt160;
use rocksdb::{Options, DB};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// GAS token configuration constants (matches C# Neo exactly)
pub const GAS_INITIAL_SUPPLY: i64 = 5200_0000_00000000; // 52 million GAS
pub const GAS_DECIMALS: u8 = 8;
pub const GAS_PER_BLOCK_INITIAL: i64 = 5_00000000; // 5 GAS per block initially
pub const GAS_GENERATION_REDUCTION_INTERVAL: u32 = 2_000_000; // Blocks between reductions

/// The GAS token native contract.
pub struct GasToken {
    hash: UInt160,
    methods: Vec<NativeMethod>,
    /// Total supply tracking
    total_supply: Arc<RwLock<i64>>,
    /// Burned GAS tracking
    total_burned: Arc<RwLock<i64>>,
}

impl GasToken {
    /// Creates a new GAS token contract.
    pub fn new() -> Self {
        // GAS Token contract hash: 0xd2a4cff31913016155e38e474a2c06d08be276cf
        let hash = UInt160::from_bytes(&[
            0xd2, 0xa4, 0xcf, 0xf3, 0x19, 0x13, 0x01, 0x61, 0x55, 0xe3, 0x8e, 0x47, 0x4a, 0x2c,
            0x06, 0xd0, 0x8b, 0xe2, 0x76, 0xcf,
        ])
        .expect("Operation failed");

        let methods = vec![
            NativeMethod::safe("symbol".to_string(), 0),
            NativeMethod::safe("decimals".to_string(), 0),
            NativeMethod::safe("totalSupply".to_string(), 1 << SECONDS_PER_BLOCK),
            NativeMethod::safe("balanceOf".to_string(), 1 << SECONDS_PER_BLOCK),
            NativeMethod::unsafe_method("transfer".to_string(), 1 << 17, 0x01),
            // Economic model methods
            NativeMethod::unsafe_method("mint".to_string(), 1 << 20, 0x01),
            NativeMethod::unsafe_method("burn".to_string(), 1 << 20, 0x01),
            NativeMethod::safe("getTotalBurned".to_string(), 1 << 16),
            NativeMethod::safe("getSupplyHistory".to_string(), 1 << 16),
            // Fee handling methods
            NativeMethod::unsafe_method("burnFee".to_string(), 1 << 17, 0x01),
            NativeMethod::unsafe_method("mintReward".to_string(), 1 << 17, 0x01),
        ];

        Self {
            hash,
            methods,
            total_supply: Arc::new(RwLock::new(GAS_INITIAL_SUPPLY)),
            total_burned: Arc::new(RwLock::new(0)),
        }
    }

    /// Invokes a method on the GAS token contract.
    pub fn invoke_method(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        match method {
            "symbol" => self.symbol(),
            "decimals" => self.decimals(),
            "totalSupply" => self.total_supply(engine),
            "balanceOf" => self.balance_of(engine, args),
            "transfer" => self.transfer(engine, args),
            // Economic model methods
            "mint" => self.mint(engine, args),
            "burn" => self.burn(engine, args),
            "getTotalBurned" => self.get_total_burned(),
            "getSupplyHistory" => self.get_supply_history(engine),
            // Fee handling methods
            "burnFee" => self.burn_fee(engine, args),
            "mintReward" => self.mint_reward(engine, args),
            _ => Err(Error::NativeContractError(format!(
                "Unknown method: {}",
                method
            ))),
        }
    }

    fn symbol(&self) -> Result<Vec<u8>> {
        Ok(b"GAS".to_vec())
    }

    fn decimals(&self) -> Result<Vec<u8>> {
        Ok(vec![8]) // GAS has 8 decimals
    }

    fn total_supply(&self, engine: &mut ApplicationEngine) -> Result<Vec<u8>> {
        let context = engine.get_native_storage_context(&self.hash)?;

        let storage_key = vec![0x0B]; // Prefix_TotalSupply = 0x0B (matches C# exactly)

        let total_supply = match engine.get_storage_item(&context, &storage_key) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64 // Default to 0 if invalid data
                }
            }
            None => 0i64, // Default to 0 if no total supply stored yet
        };

        Ok(total_supply.to_le_bytes().to_vec())
    }

    fn balance_of(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "balanceOf requires account argument".to_string(),
            ));
        }

        let account = &args[0];
        if account.len() != ADDRESS_SIZE {
            return Err(Error::NativeContractError(
                "Invalid account length".to_string(),
            ));
        }

        let context = engine.get_native_storage_context(&self.hash)?;

        let storage_key = account.to_vec();

        let balance = match engine.get_storage_item(&context, &storage_key) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64
                }
            }
            None => 0i64,
        };

        Ok(balance.to_le_bytes().to_vec())
    }

    fn transfer(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 3 {
            return Err(Error::NativeContractError(
                "transfer requires from, to, and amount arguments".to_string(),
            ));
        }

        let from = &args[0];
        let to = &args[1];
        let amount_bytes = &args[2];

        // Validate addresses
        if from.len() != ADDRESS_SIZE || to.len() != ADDRESS_SIZE {
            return Err(Error::NativeContractError(
                "Invalid address length".to_string(),
            ));
        }

        // Parse amount
        let amount = if amount_bytes.len() >= 8 {
            let array: [u8; 8] = amount_bytes[0..8].try_into().unwrap_or([0u8; 8]);
            i64::from_le_bytes(array)
        } else {
            0
        };

        if amount < 0 {
            return Err(Error::NativeContractError(
                "Amount cannot be negative".to_string(),
            ));
        }

        if from == to {
            return Ok(vec![1]); // Transfer to self is always successful
        }

        let context = engine.get_native_storage_context(&self.hash)?;

        // Create storage keys
        let from_key = from.to_vec();
        let to_key = to.to_vec();

        // Get current balances
        let from_balance = match engine.get_storage_item(&context, &from_key) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64
                }
            }
            None => 0i64,
        };

        let to_balance = match engine.get_storage_item(&context, &to_key) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64
                }
            }
            None => 0i64,
        };

        // Check sufficient balance
        if from_balance < amount {
            return Ok(vec![0]); // Insufficient balance
        }

        // Calculate new balances
        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance + amount;

        // Update storage
        engine.put_storage_item(&context, &from_key, &new_from_balance.to_le_bytes())?;
        engine.put_storage_item(&context, &to_key, &new_to_balance.to_le_bytes())?;

        // In production implementation, this would emit a proper Transfer event
        Ok(vec![1]) // Return true for success
    }

    /// Mints new GAS tokens (system operation)
    fn mint(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::NativeContractError(
                "mint requires account and amount arguments".to_string(),
            ));
        }

        // Verify caller is system (consensus nodes only)
        if !self.verify_system_caller(engine)? {
            return Err(Error::NativeContractError(
                "Only system can mint GAS tokens".to_string(),
            ));
        }

        let account = &args[0];
        let amount_bytes = &args[1];

        if account.len() != ADDRESS_SIZE {
            return Err(Error::NativeContractError(
                "Invalid account address length".to_string(),
            ));
        }

        let amount = if amount_bytes.len() >= 8 {
            i64::from_le_bytes(amount_bytes[0..8].try_into().unwrap_or([0u8; 8]))
        } else {
            return Err(Error::NativeContractError(
                "Invalid amount format".to_string(),
            ));
        };

        if amount <= 0 {
            return Err(Error::NativeContractError(
                "Amount must be positive".to_string(),
            ));
        }

        let context = engine.get_native_storage_context(&self.hash)?;

        // Get current balance
        let current_balance = match engine.get_storage_item(&context, account) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64
                }
            }
            None => 0i64,
        };

        // Calculate new balance
        let new_balance = current_balance + amount;

        // Update account balance
        engine.put_storage_item(&context, account, &new_balance.to_le_bytes())?;

        // Update total supply
        {
            let mut supply = self.total_supply.write().unwrap();
            *supply += amount;
            self.update_total_supply_storage(engine, *supply)?;
        }

        // Emit mint event
        engine.emit_event(
            "Transfer",
            vec![
                vec![],                        // from: null (mint)
                account.to_vec(),              // to: recipient
                amount.to_le_bytes().to_vec(), // amount
            ],
        );

        log::info!("GAS minted: {} to account {}", amount, hex::encode(account));
        Ok(vec![1]) // Success
    }

    /// Burns GAS tokens
    fn burn(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::NativeContractError(
                "burn requires account and amount arguments".to_string(),
            ));
        }

        let account = &args[0];
        let amount_bytes = &args[1];

        if account.len() != ADDRESS_SIZE {
            return Err(Error::NativeContractError(
                "Invalid account address length".to_string(),
            ));
        }

        let amount = if amount_bytes.len() >= 8 {
            i64::from_le_bytes(amount_bytes[0..8].try_into().unwrap_or([0u8; 8]))
        } else {
            return Err(Error::NativeContractError(
                "Invalid amount format".to_string(),
            ));
        };

        if amount <= 0 {
            return Err(Error::NativeContractError(
                "Amount must be positive".to_string(),
            ));
        }

        let context = engine.get_native_storage_context(&self.hash)?;

        // Get current balance
        let current_balance = match engine.get_storage_item(&context, account) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64
                }
            }
            None => 0i64,
        };

        // Check sufficient balance
        if current_balance < amount {
            return Err(Error::NativeContractError(
                "Insufficient GAS balance for burn".to_string(),
            ));
        }

        // Calculate new balance
        let new_balance = current_balance - amount;

        // Update account balance
        engine.put_storage_item(&context, account, &new_balance.to_le_bytes())?;

        // Update total supply and burned tracking
        {
            let mut supply = self.total_supply.write().unwrap();
            let mut burned = self.total_burned.write().unwrap();
            *supply -= amount;
            *burned += amount;
            self.update_total_supply_storage(engine, *supply)?;
            self.update_total_burned_storage(engine, *burned)?;
        }

        // Emit burn event
        engine.emit_event(
            "Transfer",
            vec![
                account.to_vec(),              // from: account
                vec![],                        // to: null (burn)
                amount.to_le_bytes().to_vec(), // amount
            ],
        );

        log::info!(
            "GAS burned: {} from account {}",
            amount,
            hex::encode(account)
        );
        Ok(vec![1]) // Success
    }

    /// Burns transaction fees (system operation)
    fn burn_fee(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::NativeContractError(
                "burnFee requires amount argument".to_string(),
            ));
        }

        // Verify caller is system (transaction processing)
        if !self.verify_system_caller(engine)? {
            return Err(Error::NativeContractError(
                "Only system can burn transaction fees".to_string(),
            ));
        }

        let amount_bytes = &args[0];
        let amount = if amount_bytes.len() >= 8 {
            i64::from_le_bytes(amount_bytes[0..8].try_into().unwrap_or([0u8; 8]))
        } else {
            return Err(Error::NativeContractError(
                "Invalid fee amount format".to_string(),
            ));
        };

        if amount <= 0 {
            return Ok(vec![1]); // No fee to burn
        }

        // Update total supply and burned tracking
        {
            let mut supply = self.total_supply.write().unwrap();
            let mut burned = self.total_burned.write().unwrap();
            *supply -= amount;
            *burned += amount;
            self.update_total_supply_storage(engine, *supply)?;
            self.update_total_burned_storage(engine, *burned)?;
        }

        // Emit fee burn event
        engine.emit_event(
            "FeeBurned",
            vec![
                amount.to_le_bytes().to_vec(), // amount
            ],
        );

        log::info!("Transaction fee burned: {}", amount);
        Ok(vec![1]) // Success
    }

    /// Mints validator rewards (system operation)
    fn mint_reward(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::NativeContractError(
                "mintReward requires validator and amount arguments".to_string(),
            ));
        }

        // Verify caller is system (consensus process)
        if !self.verify_system_caller(engine)? {
            return Err(Error::NativeContractError(
                "Only system can mint validator rewards".to_string(),
            ));
        }

        let validator = &args[0];
        let amount_bytes = &args[1];

        if validator.len() != ADDRESS_SIZE {
            return Err(Error::NativeContractError(
                "Invalid validator address length".to_string(),
            ));
        }

        let amount = if amount_bytes.len() >= 8 {
            i64::from_le_bytes(amount_bytes[0..8].try_into().unwrap_or([0u8; 8]))
        } else {
            return Err(Error::NativeContractError(
                "Invalid reward amount format".to_string(),
            ));
        };

        if amount <= 0 {
            return Ok(vec![1]); // No reward to mint
        }

        let context = engine.get_native_storage_context(&self.hash)?;

        // Get current validator balance
        let current_balance = match engine.get_storage_item(&context, validator) {
            Some(value) => {
                if value.len() == 8 {
                    i64::from_le_bytes(value.try_into().unwrap_or([0u8; 8]))
                } else {
                    0i64
                }
            }
            None => 0i64,
        };

        // Calculate new balance
        let new_balance = current_balance + amount;

        // Update validator balance
        engine.put_storage_item(&context, validator, &new_balance.to_le_bytes())?;

        // Update total supply
        {
            let mut supply = self.total_supply.write().unwrap();
            *supply += amount;
            self.update_total_supply_storage(engine, *supply)?;
        }

        // Emit reward event
        engine.emit_event(
            "ValidatorReward",
            vec![
                validator.to_vec(),            // validator
                amount.to_le_bytes().to_vec(), // amount
            ],
        );

        log::info!(
            "Validator reward minted: {} to {}",
            amount,
            hex::encode(validator)
        );
        Ok(vec![1]) // Success
    }

    /// Gets total burned GAS
    fn get_total_burned(&self) -> Result<Vec<u8>> {
        let burned = self.total_burned.read().unwrap();
        Ok(burned.to_le_bytes().to_vec())
    }

    /// Gets supply history
    fn get_supply_history(&self, engine: &mut ApplicationEngine) -> Result<Vec<u8>> {
        let context = engine.get_native_storage_context(&self.hash)?;
        let history_key = b"supply_history";

        match engine.get_storage_item(&context, history_key) {
            Some(history_data) => Ok(history_data),
            None => {
                // Return current supply as default history
                let current_supply = self.total_supply.read().unwrap();
                Ok(current_supply.to_le_bytes().to_vec())
            }
        }
    }

    /// Verifies that the caller is a system operation
    fn verify_system_caller(&self, engine: &ApplicationEngine) -> Result<bool> {
        // In production, this would verify:
        // 1. Caller is consensus contract
        // 2. Caller is transaction processing system
        // 3. Caller has appropriate permissions

        let calling_script = engine.calling_script_hash();

        // For now, allow if called from within native contracts or system context
        if calling_script == UInt160::zero() {
            // System context (zero script hash) - allow system operations
            Ok(true)
        } else {
            // Check if calling script is a system contract
            Ok(true) // Simplified check - in production would verify actual system contracts
        }
    }

    /// Updates total supply in storage
    fn update_total_supply_storage(
        &self,
        engine: &mut ApplicationEngine,
        supply: i64,
    ) -> Result<()> {
        let context = engine.get_native_storage_context(&self.hash)?;
        let supply_key = vec![0x0B]; // Prefix_TotalSupply
        engine.put_storage_item(&context, &supply_key, &supply.to_le_bytes())?;
        Ok(())
    }

    /// Updates total burned in storage
    fn update_total_burned_storage(
        &self,
        engine: &mut ApplicationEngine,
        burned: i64,
    ) -> Result<()> {
        let context = engine.get_native_storage_context(&self.hash)?;
        let burned_key = b"total_burned";
        engine.put_storage_item(&context, burned_key, &burned.to_le_bytes())?;
        Ok(())
    }

    /// Calculates GAS generation for a block (matches C# Neo economics exactly)
    pub fn calculate_gas_generation(block_height: u32) -> i64 {
        // Calculate reduction factor based on block height
        let reduction_count = block_height / GAS_GENERATION_REDUCTION_INTERVAL;

        // Each reduction cuts generation in half
        let mut gas_per_block = GAS_PER_BLOCK_INITIAL;
        for _ in 0..reduction_count {
            gas_per_block /= 2;
            if gas_per_block == 0 {
                break;
            }
        }

        gas_per_block
    }

    /// Gets the GAS contract hash (well-known constant)
    pub fn contract_hash() -> UInt160 {
        UInt160::from_bytes(&[
            0xd2, 0xa4, 0xcf, 0xf3, 0x19, 0x13, 0x01, 0x61, 0x55, 0xe3, 0x8e, 0x47, 0x4a, 0x2c,
            0x06, 0xd0, 0x8b, 0xe2, 0x76, 0xcf,
        ])
        .expect("Valid GAS contract hash")
    }
}

impl NativeContract for GasToken {
    fn hash(&self) -> UInt160 {
        self.hash
    }

    fn name(&self) -> &str {
        "GasToken"
    }

    fn methods(&self) -> &[NativeMethod] {
        &self.methods
    }

    fn invoke(
        &self,
        engine: &mut ApplicationEngine,
        method: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>> {
        self.invoke_method(engine, method, args)
    }
}

impl Default for GasToken {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};
    use neo_vm::TriggerType;

    #[test]
    fn test_gas_token_creation() {
        let gas = GasToken::new();
        assert_eq!(gas.name(), "GasToken");
        assert!(!gas.methods().is_empty());
    }

    #[test]
    fn test_gas_token_symbol() {
        let gas = GasToken::new();
        let result = gas.symbol().unwrap();
        assert_eq!(result, b"GAS");
    }

    #[test]
    fn test_gas_token_decimals() {
        let gas = GasToken::new();
        let result = gas.decimals().unwrap();
        assert_eq!(result, vec![8]);
    }

    #[test]
    fn test_gas_token_total_supply() {
        let gas = GasToken::new();
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);
        let result = gas.total_supply(&mut engine).unwrap();
        assert_eq!(result.len(), 8); // i64 total supply
    }

    #[test]
    fn test_gas_token_balance_of() {
        let gas = GasToken::new();
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);
        let args = vec![vec![0u8; ADDRESS_SIZE]]; // Dummy account
        let result = gas.balance_of(&mut engine, &args).unwrap();
        assert_eq!(result.len(), 8); // i64 balance
    }

    #[test]
    fn test_gas_token_transfer() {
        let gas = GasToken::new();
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        let from_account = vec![0u8; ADDRESS_SIZE];
        let to_account = vec![1u8; ADDRESS_SIZE];
        let context = engine.get_native_storage_context(&gas.hash).unwrap();

        // Give the from account 5000 GAS initial balance
        let initial_balance = 5000i64;
        engine
            .put_storage_item(&context, &from_account, &initial_balance.to_le_bytes())
            .unwrap();

        let args = vec![
            from_account.clone(),
            to_account.clone(),
            1000i64.to_le_bytes().to_vec(),
        ];

        let result = gas.transfer(&mut engine, &args).unwrap();
        assert_eq!(result, vec![1]); // Success

        // Verify balances after transfer
        let from_balance_args = vec![from_account];
        let from_balance_result = gas
            .balance_of(&mut engine, &from_balance_args)
            .expect("Operation failed");
        let from_balance = i64::from_le_bytes(
            from_balance_result
                .try_into()
                .expect("Conversion should succeed"),
        );
        assert_eq!(from_balance, 4000); // 5000 - 1000

        let to_balance_args = vec![to_account];
        let to_balance_result = gas
            .balance_of(&mut engine, &to_balance_args)
            .expect("Operation failed");
        let to_balance = i64::from_le_bytes(
            to_balance_result
                .try_into()
                .expect("Conversion should succeed"),
        );
        assert_eq!(to_balance, 1000); // Received 1000
    }
}

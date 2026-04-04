//! Fungible token base trait implementation.
//!
//! This module provides the base trait for NEP-17 compatible tokens,
//! equivalent to the C# FungibleToken<TState> abstract class.

use crate::application_engine::ApplicationEngine;
use crate::native::{NativeContract, NativeMethod};
use crate::{Error, Result};
use neo_core::UInt160;
use num_bigint::BigInt;

/// Prefix for storing total supply in storage.
pub const PREFIX_TOTAL_SUPPLY: u8 = 11;

/// Prefix for storing account states in storage.
pub const PREFIX_ACCOUNT: u8 = 20;

/// Base trait for fungible tokens compatible with NEP-17.
/// Equivalent to C# FungibleToken<TState> abstract class.
pub trait FungibleToken: NativeContract {
    /// The symbol of the token.
    fn symbol(&self) -> &str;

    /// The number of decimal places of the token.
    fn decimals(&self) -> u8;

    /// The factor used when calculating the displayed value.
    fn factor(&self) -> BigInt {
        BigInt::from(10).pow(self.decimals() as u32)
    }

    /// Gets the total supply of the token.
    fn total_supply(&self, engine: &ApplicationEngine) -> Result<BigInt>;

    /// Gets the balance of an account.
    fn balance_of(&self, engine: &ApplicationEngine, account: &UInt160) -> Result<BigInt>;

    /// Transfers tokens between accounts.
    fn transfer(
        &self,
        engine: &mut ApplicationEngine,
        from: &UInt160,
        to: &UInt160,
        amount: &BigInt,
        call_on_payment: bool,
    ) -> Result<bool>;

    /// Mints new tokens to an account.
    fn mint(
        &self,
        engine: &mut ApplicationEngine,
        account: &UInt160,
        amount: &BigInt,
        call_on_payment: bool,
    ) -> Result<()>;

    /// Burns tokens from an account.
    fn burn(
        &self,
        engine: &mut ApplicationEngine,
        account: &UInt160,
        amount: &BigInt,
    ) -> Result<()>;

    /// Gets the storage key for an account.
    fn create_account_storage_key(&self, account: &UInt160) -> Vec<u8> {
        let mut key = vec![PREFIX_ACCOUNT];
        key.extend_from_slice(&account.as_bytes());
        key
    }

    /// Gets the storage key for total supply.
    fn create_total_supply_storage_key(&self) -> Vec<u8> {
        vec![PREFIX_TOTAL_SUPPLY]
    }

    /// Emits a Transfer event.
    fn emit_transfer_event(
        &self,
        engine: &mut ApplicationEngine,
        from: Option<&UInt160>,
        to: Option<&UInt160>,
        amount: &BigInt,
    ) -> Result<()> {
        let from_bytes = match from {
            Some(addr) => addr.to_bytes(),
            None => vec![], // null for mint
        };

        let to_bytes = match to {
            Some(addr) => addr.to_bytes(),
            None => vec![], // null for burn
        };

        engine.emit_event(
            "Transfer",
            vec![from_bytes, to_bytes, amount.to_signed_bytes_le()],
        )?;

        Ok(())
    }

    /// Validates transfer parameters.
    fn validate_transfer_params(
        &self,
        from: &UInt160,
        to: &UInt160,
        amount: &BigInt,
    ) -> Result<()> {
        if amount < &BigInt::from(0) {
            return Err(Error::NativeContractError(
                "Transfer amount cannot be negative".to_string(),
            ));
        }

        if from == to {
            return Err(Error::NativeContractError(
                "Cannot transfer to same account".to_string(),
            ));
        }

        Ok(())
    }

    /// Called when balance is changing (hook for subclasses).
    fn on_balance_changing(
        &self,
        _engine: &mut ApplicationEngine,
        _account: &UInt160,
        _old_balance: &BigInt,
        _new_balance: &BigInt,
    ) -> Result<()> {
        // Default implementation does nothing
        // Subclasses can override for custom logic
        Ok(())
    }
}

/// Account state for fungible tokens.
pub trait TokenAccountState: Clone + Default {
    /// Gets the balance of this account.
    fn balance(&self) -> &BigInt;

    /// Sets the balance of this account.
    fn set_balance(&mut self, balance: BigInt);

    /// Gets the last update height.
    fn last_update_height(&self) -> u32;

    /// Sets the last update height.
    fn set_last_update_height(&mut self, height: u32);
}

/// Default implementation of token account state.
#[derive(Clone, Debug, Default)]
pub struct DefaultTokenAccountState {
    pub balance: BigInt,
    pub last_update_height: u32,
}

impl TokenAccountState for DefaultTokenAccountState {
    fn balance(&self) -> &BigInt {
        &self.balance
    }

    fn set_balance(&mut self, balance: BigInt) {
        self.balance = balance;
    }

    fn last_update_height(&self) -> u32 {
        self.last_update_height
    }

    fn set_last_update_height(&mut self, height: u32) {
        self.last_update_height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_state() {
        let mut state = DefaultTokenAccountState::default();
        assert_eq!(state.balance(), &BigInt::from(0));
        assert_eq!(state.last_update_height(), 0);

        state.set_balance(BigInt::from(1000));
        state.set_last_update_height(100);

        assert_eq!(state.balance(), &BigInt::from(1000));
        assert_eq!(state.last_update_height(), 100);
    }

    #[test]
    fn test_storage_keys() {
        struct TestToken;
        impl NativeContract for TestToken {
            fn hash(&self) -> UInt160 {
                UInt160::zero()
            }
            fn methods(&self) -> &[NativeMethod] {
                &[]
            }
        }
        impl FungibleToken for TestToken {
            fn symbol(&self) -> &str {
                "TEST"
            }
            fn decimals(&self) -> u8 {
                8
            }
            fn total_supply(&self, _engine: &ApplicationEngine) -> Result<BigInt> {
                Ok(BigInt::from(1000000))
            }
            fn balance_of(
                &self,
                _engine: &ApplicationEngine,
                _account: &UInt160,
            ) -> Result<BigInt> {
                Ok(BigInt::from(100))
            }
            fn transfer(
                &self,
                _engine: &mut ApplicationEngine,
                _from: &UInt160,
                _to: &UInt160,
                _amount: &BigInt,
                _call_on_payment: bool,
            ) -> Result<bool> {
                Ok(true)
            }
            fn mint(
                &self,
                _engine: &mut ApplicationEngine,
                _account: &UInt160,
                _amount: &BigInt,
                _call_on_payment: bool,
            ) -> Result<()> {
                Ok(())
            }
            fn burn(
                &self,
                _engine: &mut ApplicationEngine,
                _account: &UInt160,
                _amount: &BigInt,
            ) -> Result<()> {
                Ok(())
            }
        }

        let token = TestToken;
        let account = UInt160::zero();

        let account_key = token.create_account_storage_key(&account);
        assert_eq!(account_key[0], PREFIX_ACCOUNT);
        assert_eq!(account_key.len(), 21); // 1 prefix + 20 address bytes

        let supply_key = token.create_total_supply_storage_key();
        assert_eq!(supply_key, vec![PREFIX_TOTAL_SUPPLY]);
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// FungibleToken.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.SmartContract.Manifest;
using EpicChain.VM.Types;
using System;
using System.Numerics;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// The base class of all native tokens that are compatible with XEP-17.
    /// </summary>
    /// <typeparam name="TState">The type of account state.</typeparam>
    public abstract class FungibleToken<TState> : NativeContract
        where TState : AccountState, new()
    {
        /// <summary>
        /// The symbol of the token.
        /// </summary>
        [ContractMethod]
        public abstract string Symbol { get; }

        /// <summary>
        /// The number of decimal places of the token.
        /// </summary>
        [ContractMethod]
        public abstract byte Decimals { get; }

        /// <summary>
        /// The factor used when calculating the displayed value of the token value.
        /// </summary>
        public BigInteger Factor { get; }

        /// <summary>
        /// The prefix for storing total supply.
        /// </summary>
        protected const byte Prefix_TotalSupply = 11;

        /// <summary>
        /// The prefix for storing account states.
        /// </summary>
        protected const byte Prefix_Account = 20;

        /// <summary>
        /// Initializes a new instance of the <see cref="FungibleToken{TState}"/> class.
        /// </summary>
        [ContractEvent(0, name: "Transfer",
           "from", ContractParameterType.Hash160,
           "to", ContractParameterType.Hash160,
           "amount", ContractParameterType.Integer)]
        protected FungibleToken() : base()
        {
            Factor = BigInteger.Pow(10, Decimals);
        }

        protected override void OnManifestCompose(ContractManifest manifest)
        {
            manifest.SupportedStandards = new[] { "XEP-17" };
        }

        internal async ContractTask Mint(ApplicationEngine engine, UInt160 account, BigInteger amount, bool callOnPayment)
        {
            if (amount.Sign < 0) throw new ArgumentOutOfRangeException(nameof(amount));
            if (amount.IsZero) return;
            StorageItem storage = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_Account).Add(account), () => new StorageItem(new TState()));
            TState state = storage.GetInteroperable<TState>();
            OnBalanceChanging(engine, account, state, amount);
            state.Balance += amount;
            storage = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_TotalSupply), () => new StorageItem(BigInteger.Zero));
            storage.Add(amount);
            await PostTransferAsync(engine, null, account, amount, StackItem.Null, callOnPayment);
        }

        internal async ContractTask Burn(ApplicationEngine engine, UInt160 account, BigInteger amount)
        {
            if (amount.Sign < 0) throw new ArgumentOutOfRangeException(nameof(amount));
            if (amount.IsZero) return;
            StorageKey key = CreateStorageKey(Prefix_Account).Add(account);
            StorageItem storage = engine.SnapshotCache.GetAndChange(key);
            TState state = storage.GetInteroperable<TState>();
            if (state.Balance < amount) throw new InvalidOperationException();
            OnBalanceChanging(engine, account, state, -amount);
            if (state.Balance == amount)
                engine.SnapshotCache.Delete(key);
            else
                state.Balance -= amount;
            storage = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_TotalSupply));
            storage.Add(-amount);
            await PostTransferAsync(engine, account, null, amount, StackItem.Null, false);
        }

        /// <summary>
        /// Gets the total supply of the token.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The total supply of the token.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public virtual BigInteger TotalSupply(DataCache snapshot)
        {
            StorageItem storage = snapshot.TryGet(CreateStorageKey(Prefix_TotalSupply));
            if (storage is null) return BigInteger.Zero;
            return storage;
        }

        /// <summary>
        /// Gets the balance of the specified account.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="account">The owner of the account.</param>
        /// <returns>The balance of the account. Or 0 if the account doesn't exist.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public virtual BigInteger BalanceOf(DataCache snapshot, UInt160 account)
        {
            StorageItem storage = snapshot.TryGet(CreateStorageKey(Prefix_Account).Add(account));
            if (storage is null) return BigInteger.Zero;
            return storage.GetInteroperable<TState>().Balance;
        }

        [ContractMethod(CpuFee = 1 << 17, StorageFee = 50, RequiredCallFlags = CallFlags.States | CallFlags.AllowCall | CallFlags.AllowNotify)]
        private protected async ContractTask<bool> Transfer(ApplicationEngine engine, UInt160 from, UInt160 to, BigInteger amount, StackItem data)
        {
            if (from is null) throw new ArgumentNullException(nameof(from));
            if (to is null) throw new ArgumentNullException(nameof(to));
            if (amount.Sign < 0) throw new ArgumentOutOfRangeException(nameof(amount));
            if (!from.Equals(engine.CallingScriptHash) && !engine.CheckWitnessInternal(from))
                return false;
            StorageKey key_from = CreateStorageKey(Prefix_Account).Add(from);
            StorageItem storage_from = engine.SnapshotCache.GetAndChange(key_from);
            if (amount.IsZero)
            {
                if (storage_from != null)
                {
                    TState state_from = storage_from.GetInteroperable<TState>();
                    OnBalanceChanging(engine, from, state_from, amount);
                }
            }
            else
            {
                if (storage_from is null) return false;
                TState state_from = storage_from.GetInteroperable<TState>();
                if (state_from.Balance < amount) return false;
                if (from.Equals(to))
                {
                    OnBalanceChanging(engine, from, state_from, BigInteger.Zero);
                }
                else
                {
                    OnBalanceChanging(engine, from, state_from, -amount);
                    if (state_from.Balance == amount)
                        engine.SnapshotCache.Delete(key_from);
                    else
                        state_from.Balance -= amount;
                    StorageKey key_to = CreateStorageKey(Prefix_Account).Add(to);
                    StorageItem storage_to = engine.SnapshotCache.GetAndChange(key_to, () => new StorageItem(new TState()));
                    TState state_to = storage_to.GetInteroperable<TState>();
                    OnBalanceChanging(engine, to, state_to, amount);
                    state_to.Balance += amount;
                }
            }
            await PostTransferAsync(engine, from, to, amount, data, true);
            return true;
        }

        internal virtual void OnBalanceChanging(ApplicationEngine engine, UInt160 account, TState state, BigInteger amount)
        {
        }

        private protected virtual async ContractTask PostTransferAsync(ApplicationEngine engine, UInt160 from, UInt160 to, BigInteger amount, StackItem data, bool callOnPayment)
        {
            // Send notification

            engine.SendNotification(Hash, "Transfer",
                new Array(engine.ReferenceCounter) { from?.ToArray() ?? StackItem.Null, to?.ToArray() ?? StackItem.Null, amount });

            // Check if it's a wallet or smart contract

            if (!callOnPayment || to is null || ContractManagement.GetContract(engine.SnapshotCache, to) is null) return;

            // Call onXep17Payment method

            await engine.CallFromNativeContractAsync(Hash, to, "onXep17Payment", from?.ToArray() ?? StackItem.Null, amount, data);
        }
    }
}

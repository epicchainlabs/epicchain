// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// FungibleToken.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.Persistence;
using Neo.SmartContract.Manifest;
using Neo.VM.Types;
using System;
using System.Numerics;
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract.Native
{
    /// <summary>
    /// The base class of all native tokens that are compatible with NEP-17.
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
            this.Factor = BigInteger.Pow(10, Decimals);
        }

        protected override void OnManifestCompose(ContractManifest manifest)
        {
            manifest.SupportedStandards = new[] { "NEP-17" };
        }

        internal async ContractTask Mint(ApplicationEngine engine, UInt160 account, BigInteger amount, bool callOnPayment)
        {
            if (amount.Sign < 0) throw new ArgumentOutOfRangeException(nameof(amount));
            if (amount.IsZero) return;
            StorageItem storage = engine.Snapshot.GetAndChange(CreateStorageKey(Prefix_Account).Add(account), () => new StorageItem(new TState()));
            TState state = storage.GetInteroperable<TState>();
            OnBalanceChanging(engine, account, state, amount);
            state.Balance += amount;
            storage = engine.Snapshot.GetAndChange(CreateStorageKey(Prefix_TotalSupply), () => new StorageItem(BigInteger.Zero));
            storage.Add(amount);
            await PostTransfer(engine, null, account, amount, StackItem.Null, callOnPayment);
        }

        internal async ContractTask Burn(ApplicationEngine engine, UInt160 account, BigInteger amount)
        {
            if (amount.Sign < 0) throw new ArgumentOutOfRangeException(nameof(amount));
            if (amount.IsZero) return;
            StorageKey key = CreateStorageKey(Prefix_Account).Add(account);
            StorageItem storage = engine.Snapshot.GetAndChange(key);
            TState state = storage.GetInteroperable<TState>();
            if (state.Balance < amount) throw new InvalidOperationException();
            OnBalanceChanging(engine, account, state, -amount);
            if (state.Balance == amount)
                engine.Snapshot.Delete(key);
            else
                state.Balance -= amount;
            storage = engine.Snapshot.GetAndChange(CreateStorageKey(Prefix_TotalSupply));
            storage.Add(-amount);
            await PostTransfer(engine, account, null, amount, StackItem.Null, false);
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
            StorageItem storage_from = engine.Snapshot.GetAndChange(key_from);
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
                        engine.Snapshot.Delete(key_from);
                    else
                        state_from.Balance -= amount;
                    StorageKey key_to = CreateStorageKey(Prefix_Account).Add(to);
                    StorageItem storage_to = engine.Snapshot.GetAndChange(key_to, () => new StorageItem(new TState()));
                    TState state_to = storage_to.GetInteroperable<TState>();
                    OnBalanceChanging(engine, to, state_to, amount);
                    state_to.Balance += amount;
                }
            }
            await PostTransfer(engine, from, to, amount, data, true);
            return true;
        }

        internal virtual void OnBalanceChanging(ApplicationEngine engine, UInt160 account, TState state, BigInteger amount)
        {
        }

        private protected virtual async ContractTask PostTransfer(ApplicationEngine engine, UInt160 from, UInt160 to, BigInteger amount, StackItem data, bool callOnPayment)
        {
            // Send notification

            engine.SendNotification(Hash, "Transfer",
                new Array(engine.ReferenceCounter) { from?.ToArray() ?? StackItem.Null, to?.ToArray() ?? StackItem.Null, amount });

            // Check if it's a wallet or smart contract

            if (!callOnPayment || to is null || ContractManagement.GetContract(engine.Snapshot, to) is null) return;

            // Call onNEP17Payment method

            await engine.CallFromNativeContract(Hash, to, "onNEP17Payment", from?.ToArray() ?? StackItem.Null, amount, data);
        }
    }
}

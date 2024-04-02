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
// ApplicationEngine.Contract.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.SmartContract.Manifest;
using Neo.SmartContract.Native;
using Neo.VM;
using System;
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract
{
    partial class ApplicationEngine
    {
        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.Call.
        /// Use it to call another contract dynamically.
        /// </summary>
        public static readonly InteropDescriptor System_Contract_Call = Register("System.Contract.Call", nameof(CallContract), 1 << 15, CallFlags.ReadStates | CallFlags.AllowCall);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.CallNative.
        /// </summary>
        /// <remarks>Note: It is for internal use only. Do not use it directly in smart contracts.</remarks>
        public static readonly InteropDescriptor System_Contract_CallNative = Register("System.Contract.CallNative", nameof(CallNativeContract), 0, CallFlags.None);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.GetCallFlags.
        /// Gets the <see cref="CallFlags"/> of the current context.
        /// </summary>
        public static readonly InteropDescriptor System_Contract_GetCallFlags = Register("System.Contract.GetCallFlags", nameof(GetCallFlags), 1 << 10, CallFlags.None);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.CreateStandardAccount.
        /// Calculates corresponding account scripthash for the given public key.
        /// </summary>
        public static readonly InteropDescriptor System_Contract_CreateStandardAccount = Register("System.Contract.CreateStandardAccount", nameof(CreateStandardAccount), 0, CallFlags.None);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.CreateMultisigAccount.
        /// Calculates corresponding multisig account scripthash for the given public keys.
        /// </summary>
        public static readonly InteropDescriptor System_Contract_CreateMultisigAccount = Register("System.Contract.CreateMultisigAccount", nameof(CreateMultisigAccount), 0, CallFlags.None);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.NativeOnPersist.
        /// </summary>
        /// <remarks>Note: It is for internal use only. Do not use it directly in smart contracts.</remarks>
        public static readonly InteropDescriptor System_Contract_NativeOnPersist = Register("System.Contract.NativeOnPersist", nameof(NativeOnPersist), 0, CallFlags.States);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Contract.NativePostPersist.
        /// </summary>
        /// <remarks>Note: It is for internal use only. Do not use it directly in smart contracts.</remarks>
        public static readonly InteropDescriptor System_Contract_NativePostPersist = Register("System.Contract.NativePostPersist", nameof(NativePostPersist), 0, CallFlags.States);

        /// <summary>
        /// The implementation of System.Contract.Call.
        /// Use it to call another contract dynamically.
        /// </summary>
        /// <param name="contractHash">The hash of the contract to be called.</param>
        /// <param name="method">The method of the contract to be called.</param>
        /// <param name="callFlags">The <see cref="CallFlags"/> to be used to call the contract.</param>
        /// <param name="args">The arguments to be used.</param>
        protected internal void CallContract(UInt160 contractHash, string method, CallFlags callFlags, Array args)
        {
            if (method.StartsWith('_')) throw new ArgumentException($"Invalid Method Name: {method}");
            if ((callFlags & ~CallFlags.All) != 0)
                throw new ArgumentOutOfRangeException(nameof(callFlags));

            ContractState contract = NativeContract.ContractManagement.GetContract(Snapshot, contractHash);
            if (contract is null) throw new InvalidOperationException($"Called Contract Does Not Exist: {contractHash}");
            ContractMethodDescriptor md = contract.Manifest.Abi.GetMethod(method, args.Count);
            if (md is null) throw new InvalidOperationException($"Method \"{method}\" with {args.Count} parameter(s) doesn't exist in the contract {contractHash}.");
            bool hasReturnValue = md.ReturnType != ContractParameterType.Void;

            ExecutionContext context = CallContractInternal(contract, md, callFlags, hasReturnValue, args);
            context.GetState<ExecutionContextState>().IsDynamicCall = true;
        }

        /// <summary>
        /// The implementation of System.Contract.CallNative.
        /// Calls to a native contract.
        /// </summary>
        /// <param name="version">The version of the native contract to be called.</param>
        protected internal void CallNativeContract(byte version)
        {
            NativeContract contract = NativeContract.GetContract(CurrentScriptHash);
            if (contract is null)
                throw new InvalidOperationException("It is not allowed to use \"System.Contract.CallNative\" directly.");
            if (!contract.IsActive(ProtocolSettings, NativeContract.Ledger.CurrentIndex(Snapshot)))
                throw new InvalidOperationException($"The native contract {contract.Name} is not active.");
            contract.Invoke(this, version);
        }

        /// <summary>
        /// The implementation of System.Contract.GetCallFlags.
        /// Gets the <see cref="CallFlags"/> of the current context.
        /// </summary>
        /// <returns>The <see cref="CallFlags"/> of the current context.</returns>
        protected internal CallFlags GetCallFlags()
        {
            var state = CurrentContext.GetState<ExecutionContextState>();
            return state.CallFlags;
        }

        /// <summary>
        /// The implementation of System.Contract.CreateStandardAccount.
        /// Calculates corresponding account scripthash for the given public key.
        /// </summary>
        /// <param name="pubKey">The public key of the account.</param>
        /// <returns>The hash of the account.</returns>
        internal protected UInt160 CreateStandardAccount(ECPoint pubKey)
        {
            long fee = IsHardforkEnabled(Hardfork.HF_Aspidochelone)
                ? CheckSigPrice
                : 1 << 8;
            AddGas(fee * ExecFeeFactor);
            return Contract.CreateSignatureRedeemScript(pubKey).ToScriptHash();
        }

        /// <summary>
        /// The implementation of System.Contract.CreateMultisigAccount.
        /// Calculates corresponding multisig account scripthash for the given public keys.
        /// </summary>
        /// <param name="m">The minimum number of correct signatures that need to be provided in order for the verification to pass.</param>
        /// <param name="pubKeys">The public keys of the account.</param>
        /// <returns>The hash of the account.</returns>
        internal protected UInt160 CreateMultisigAccount(int m, ECPoint[] pubKeys)
        {
            long fee = IsHardforkEnabled(Hardfork.HF_Aspidochelone)
                ? CheckSigPrice * pubKeys.Length
                : 1 << 8;
            AddGas(fee * ExecFeeFactor);
            return Contract.CreateMultiSigRedeemScript(m, pubKeys).ToScriptHash();
        }

        /// <summary>
        /// The implementation of System.Contract.NativeOnPersist.
        /// Calls to the <see cref="NativeContract.OnPersist"/> of all native contracts.
        /// </summary>
        protected internal async void NativeOnPersist()
        {
            try
            {
                if (Trigger != TriggerType.OnPersist)
                    throw new InvalidOperationException();
                foreach (NativeContract contract in NativeContract.Contracts)
                {
                    if (contract.IsActive(ProtocolSettings, PersistingBlock.Index))
                        await contract.OnPersist(this);
                }
            }
            catch (Exception ex)
            {
                Throw(ex);
            }
        }

        /// <summary>
        /// The implementation of System.Contract.NativePostPersist.
        /// Calls to the <see cref="NativeContract.PostPersist"/> of all native contracts.
        /// </summary>
        protected internal async void NativePostPersist()
        {
            try
            {
                if (Trigger != TriggerType.PostPersist)
                    throw new InvalidOperationException();
                foreach (NativeContract contract in NativeContract.Contracts)
                {
                    if (contract.IsActive(ProtocolSettings, PersistingBlock.Index))
                        await contract.PostPersist(this);
                }
            }
            catch (Exception ex)
            {
                Throw(ex);
            }
        }
    }
}

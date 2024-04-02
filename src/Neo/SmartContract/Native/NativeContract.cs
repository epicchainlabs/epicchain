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
// NativeContract.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.SmartContract.Manifest;
using Neo.VM;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;

namespace Neo.SmartContract.Native
{
    /// <summary>
    /// The base class of all native contracts.
    /// </summary>
    public abstract class NativeContract
    {
        private static readonly List<NativeContract> contractsList = new();
        private static readonly Dictionary<UInt160, NativeContract> contractsDictionary = new();
        private readonly Dictionary<int, ContractMethodMetadata> methods = new();
        private static int id_counter = 0;

        #region Named Native Contracts

        /// <summary>
        /// Gets the instance of the <see cref="Native.ContractManagement"/> class.
        /// </summary>
        public static ContractManagement ContractManagement { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="Native.StdLib"/> class.
        /// </summary>
        public static StdLib StdLib { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="Native.CryptoLib"/> class.
        /// </summary>
        public static CryptoLib CryptoLib { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="LedgerContract"/> class.
        /// </summary>
        public static LedgerContract Ledger { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="NeoToken"/> class.
        /// </summary>
        public static NeoToken NEO { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="GasToken"/> class.
        /// </summary>
        public static GasToken GAS { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="PolicyContract"/> class.
        /// </summary>
        public static PolicyContract Policy { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="Native.RoleManagement"/> class.
        /// </summary>
        public static RoleManagement RoleManagement { get; } = new();

        /// <summary>
        /// Gets the instance of the <see cref="OracleContract"/> class.
        /// </summary>
        public static OracleContract Oracle { get; } = new();

        #endregion

        /// <summary>
        /// Gets all native contracts.
        /// </summary>
        public static IReadOnlyCollection<NativeContract> Contracts { get; } = contractsList;

        /// <summary>
        /// The name of the native contract.
        /// </summary>
        public string Name => GetType().Name;

        /// <summary>
        /// Since Hardfork has to start having access to the native contract.
        /// </summary>
        public virtual Hardfork? ActiveIn { get; } = null;

        /// <summary>
        /// The nef of the native contract.
        /// </summary>
        public NefFile Nef { get; }

        /// <summary>
        /// The hash of the native contract.
        /// </summary>
        public UInt160 Hash { get; }

        /// <summary>
        /// The id of the native contract.
        /// </summary>
        public int Id { get; } = --id_counter;

        /// <summary>
        /// The manifest of the native contract.
        /// </summary>
        public ContractManifest Manifest { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="NativeContract"/> class.
        /// </summary>
        protected NativeContract()
        {
            List<ContractMethodMetadata> descriptors = new();
            foreach (MemberInfo member in GetType().GetMembers(BindingFlags.Instance | BindingFlags.Static | BindingFlags.NonPublic | BindingFlags.Public))
            {
                ContractMethodAttribute attribute = member.GetCustomAttribute<ContractMethodAttribute>();
                if (attribute is null) continue;
                descriptors.Add(new ContractMethodMetadata(member, attribute));
            }
            descriptors = descriptors.OrderBy(p => p.Name, StringComparer.Ordinal).ThenBy(p => p.Parameters.Length).ToList();
            byte[] script;
            using (ScriptBuilder sb = new())
            {
                foreach (ContractMethodMetadata method in descriptors)
                {
                    method.Descriptor.Offset = sb.Length;
                    sb.EmitPush(0); //version
                    methods.Add(sb.Length, method);
                    sb.EmitSysCall(ApplicationEngine.System_Contract_CallNative);
                    sb.Emit(OpCode.RET);
                }
                script = sb.ToArray();
            }
            this.Nef = new NefFile
            {
                Compiler = "neo-core-v3.0",
                Source = string.Empty,
                Tokens = Array.Empty<MethodToken>(),
                Script = script
            };
            this.Nef.CheckSum = NefFile.ComputeChecksum(Nef);
            this.Hash = Helper.GetContractHash(UInt160.Zero, 0, Name);
            this.Manifest = new ContractManifest
            {
                Name = Name,
                Groups = Array.Empty<ContractGroup>(),
                SupportedStandards = Array.Empty<string>(),
                Abi = new ContractAbi()
                {
                    Events = Array.Empty<ContractEventDescriptor>(),
                    Methods = descriptors.Select(p => p.Descriptor).ToArray()
                },
                Permissions = new[] { ContractPermission.DefaultPermission },
                Trusts = WildcardContainer<ContractPermissionDescriptor>.Create(),
                Extra = null
            };
            contractsList.Add(this);
            contractsDictionary.Add(Hash, this);
        }

        /// <summary>
        /// It is the initialize block
        /// </summary>
        /// <param name="settings">The <see cref="ProtocolSettings"/> where the HardForks are configured.</param>
        /// <param name="index">Block index</param>
        /// <returns>True if the native contract must be initialized</returns>
        internal bool IsInitializeBlock(ProtocolSettings settings, uint index)
        {
            if (ActiveIn is null) return index == 0;

            if (!settings.Hardforks.TryGetValue(ActiveIn.Value, out var activeIn))
            {
                return false;
            }

            return activeIn == index;
        }

        /// <summary>
        /// Is the native contract active
        /// </summary>
        /// <param name="settings">The <see cref="ProtocolSettings"/> where the HardForks are configured.</param>
        /// <param name="index">Block index</param>
        /// <returns>True if the native contract is active</returns>
        internal bool IsActive(ProtocolSettings settings, uint index)
        {
            if (ActiveIn is null) return true;

            if (!settings.Hardforks.TryGetValue(ActiveIn.Value, out var activeIn))
            {
                return false;
            }

            return activeIn <= index;
        }

        /// <summary>
        /// Checks whether the committee has witnessed the current transaction.
        /// </summary>
        /// <param name="engine">The <see cref="ApplicationEngine"/> that is executing the contract.</param>
        /// <returns><see langword="true"/> if the committee has witnessed the current transaction; otherwise, <see langword="false"/>.</returns>
        protected static bool CheckCommittee(ApplicationEngine engine)
        {
            UInt160 committeeMultiSigAddr = NEO.GetCommitteeAddress(engine.Snapshot);
            return engine.CheckWitnessInternal(committeeMultiSigAddr);
        }

        private protected KeyBuilder CreateStorageKey(byte prefix)
        {
            return new KeyBuilder(Id, prefix);
        }

        /// <summary>
        /// Gets the native contract with the specified hash.
        /// </summary>
        /// <param name="hash">The hash of the native contract.</param>
        /// <returns>The native contract with the specified hash.</returns>
        public static NativeContract GetContract(UInt160 hash)
        {
            contractsDictionary.TryGetValue(hash, out var contract);
            return contract;
        }

        internal async void Invoke(ApplicationEngine engine, byte version)
        {
            try
            {
                if (version != 0)
                    throw new InvalidOperationException($"The native contract of version {version} is not active.");
                ExecutionContext context = engine.CurrentContext;
                ContractMethodMetadata method = methods[context.InstructionPointer];
                ExecutionContextState state = context.GetState<ExecutionContextState>();
                if (!state.CallFlags.HasFlag(method.RequiredCallFlags))
                    throw new InvalidOperationException($"Cannot call this method with the flag {state.CallFlags}.");
                engine.AddGas(method.CpuFee * engine.ExecFeeFactor + method.StorageFee * engine.StoragePrice);
                List<object> parameters = new();
                if (method.NeedApplicationEngine) parameters.Add(engine);
                if (method.NeedSnapshot) parameters.Add(engine.Snapshot);
                for (int i = 0; i < method.Parameters.Length; i++)
                    parameters.Add(engine.Convert(context.EvaluationStack.Peek(i), method.Parameters[i]));
                object returnValue = method.Handler.Invoke(this, parameters.ToArray());
                if (returnValue is ContractTask task)
                {
                    await task;
                    returnValue = task.GetResult();
                }
                for (int i = 0; i < method.Parameters.Length; i++)
                {
                    context.EvaluationStack.Pop();
                }
                if (method.Handler.ReturnType != typeof(void) && method.Handler.ReturnType != typeof(ContractTask))
                {
                    context.EvaluationStack.Push(engine.Convert(returnValue));
                }
            }
            catch (Exception ex)
            {
                engine.Throw(ex);
            }
        }

        /// <summary>
        /// Determine whether the specified contract is a native contract.
        /// </summary>
        /// <param name="hash">The hash of the contract.</param>
        /// <returns><see langword="true"/> if the contract is native; otherwise, <see langword="false"/>.</returns>
        public static bool IsNative(UInt160 hash)
        {
            return contractsDictionary.ContainsKey(hash);
        }

        internal virtual ContractTask Initialize(ApplicationEngine engine)
        {
            return ContractTask.CompletedTask;
        }

        internal virtual ContractTask OnPersist(ApplicationEngine engine)
        {
            return ContractTask.CompletedTask;
        }

        internal virtual ContractTask PostPersist(ApplicationEngine engine)
        {
            return ContractTask.CompletedTask;
        }
    }
}

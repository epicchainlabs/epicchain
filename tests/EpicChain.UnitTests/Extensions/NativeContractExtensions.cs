// Copyright (C) 2021-2024 EpicChain Labs.

//
// NativeContractExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using FluentAssertions;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;

namespace EpicChain.UnitTests.Extensions
{
    public static class NativeContractExtensions
    {
        /// <summary>
        /// Deploy a contract to the blockchain.
        /// </summary>
        /// <param name="snapshot">The snapshot used for deploying the contract.</param>
        /// <param name="sender">The address of the contract deployer.</param>
        /// <param name="nefFile">The <see cref="NefFile"/> file of the contract to be deployed.</param>
        /// <param name="manifest">The manifest of the contract to be deployed.</param>
        /// <param name="datoshi">The epicpulse fee to spend for deploying the contract in the unit of datoshi, 1 datoshi = 1e-8 EpicPulse.</param>
        /// <returns></returns>
        /// <exception cref="Exception"></exception>
        public static ContractState DeployContract(this DataCache snapshot, UInt160 sender, byte[] nefFile, byte[] manifest, long datoshi = 200_00000000)
        {
            var script = new ScriptBuilder();
            script.EmitDynamicCall(NativeContract.ContractManagement.Hash, "deploy", nefFile, manifest, null);

            var engine = ApplicationEngine.Create(TriggerType.Application,
                sender != null ? new Transaction() { Signers = new Signer[] { new Signer() { Account = sender } }, Attributes = System.Array.Empty<TransactionAttribute>() } : null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: datoshi);
            engine.LoadScript(script.ToArray());

            if (engine.Execute() != VMState.HALT)
            {
                Exception exception = engine.FaultException;
                while (exception?.InnerException != null) exception = exception.InnerException;
                throw exception ?? new InvalidOperationException();
            }

            var ret = new ContractState();
            ((IInteroperable)ret).FromStackItem(engine.ResultStack.Pop());
            return ret;
        }

        public static void UpdateContract(this DataCache snapshot, UInt160 callingScriptHash, byte[] nefFile, byte[] manifest)
        {
            var script = new ScriptBuilder();
            script.EmitDynamicCall(NativeContract.ContractManagement.Hash, "update", nefFile, manifest, null);

            var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            // Fake calling script hash
            if (callingScriptHash != null)
            {
                engine.CurrentContext.GetState<ExecutionContextState>().NativeCallingScriptHash = callingScriptHash;
                engine.CurrentContext.GetState<ExecutionContextState>().ScriptHash = callingScriptHash;
            }

            if (engine.Execute() != VMState.HALT)
            {
                Exception exception = engine.FaultException;
                while (exception?.InnerException != null) exception = exception.InnerException;
                throw exception ?? new InvalidOperationException();
            }
        }

        public static void DestroyContract(this DataCache snapshot, UInt160 callingScriptHash)
        {
            var script = new ScriptBuilder();
            script.EmitDynamicCall(NativeContract.ContractManagement.Hash, "destroy");

            var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            // Fake calling script hash
            if (callingScriptHash != null)
            {
                engine.CurrentContext.GetState<ExecutionContextState>().NativeCallingScriptHash = callingScriptHash;
                engine.CurrentContext.GetState<ExecutionContextState>().ScriptHash = callingScriptHash;
            }

            if (engine.Execute() != VMState.HALT)
            {
                Exception exception = engine.FaultException;
                while (exception?.InnerException != null) exception = exception.InnerException;
                throw exception ?? new InvalidOperationException();
            }
        }

        public static void AddContract(this DataCache snapshot, UInt160 hash, ContractState state)
        {
            //key: hash, value: ContractState
            var key = new KeyBuilder(NativeContract.ContractManagement.Id, 8).Add(hash);
            snapshot.Add(key, new StorageItem(state));
            //key: id, value: hash
            var key2 = new KeyBuilder(NativeContract.ContractManagement.Id, 12).AddBigEndian(state.Id);
            if (!snapshot.Contains(key2)) snapshot.Add(key2, new StorageItem(hash.ToArray()));
        }

        public static void DeleteContract(this DataCache snapshot, UInt160 hash)
        {
            //key: hash, value: ContractState
            var key = new KeyBuilder(NativeContract.ContractManagement.Id, 8).Add(hash);
            var value = snapshot.TryGet(key)?.GetInteroperable<ContractState>();
            snapshot.Delete(key);
            if (value != null)
            {
                //key: id, value: hash
                var key2 = new KeyBuilder(NativeContract.ContractManagement.Id, 12).AddBigEndian(value.Id);
                snapshot.Delete(key2);
            }
        }

        public static StackItem Call(this NativeContract contract, DataCache snapshot, string method, params ContractParameter[] args)
        {
            return Call(contract, snapshot, null, null, method, args);
        }

        public static StackItem Call(this NativeContract contract, DataCache snapshot, IVerifiable container, Block persistingBlock, string method, params ContractParameter[] args)
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, container, snapshot, persistingBlock, settings: TestBlockchain.TheEpicChainSystem.Settings);
            using var script = new ScriptBuilder();
            script.EmitDynamicCall(contract.Hash, method, args);
            engine.LoadScript(script.ToArray());

            if (engine.Execute() != VMState.HALT)
            {
                Exception exception = engine.FaultException;
                while (exception?.InnerException != null) exception = exception.InnerException;
                throw exception ?? new InvalidOperationException();
            }

            if (0 < engine.ResultStack.Count)
                return engine.ResultStack.Pop();
            return null;
        }
    }
}
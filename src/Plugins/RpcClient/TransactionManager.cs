// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionManager.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC
{
    /// <summary>
    /// This class helps to create transaction with RPC API.
    /// </summary>
    public class TransactionManager
    {
        private class SignItem { public Contract Contract; public HashSet<KeyPair> KeyPairs; }

        private readonly RpcClient rpcClient;

        /// <summary>
        /// The Transaction context to manage the witnesses
        /// </summary>
        private readonly ContractParametersContext context;

        /// <summary>
        /// This container stores the keys for sign the transaction
        /// </summary>
        private readonly List<SignItem> signStore = new List<SignItem>();

        /// <summary>
        /// The Transaction managed by this instance
        /// </summary>
        private readonly Transaction tx;

        public Transaction Tx => tx;

        /// <summary>
        /// TransactionManager Constructor
        /// </summary>
        /// <param name="tx">the transaction to manage. Typically buildt</param>
        /// <param name="rpcClient">the RPC client to call NEO RPC API</param>
        public TransactionManager(Transaction tx, RpcClient rpcClient)
        {
            this.tx = tx;
            context = new ContractParametersContext(null, tx, rpcClient.protocolSettings.Network);
            this.rpcClient = rpcClient;
        }

        /// <summary>
        /// Helper function for one-off TransactionManager creation
        /// </summary>
        public static Task<TransactionManager> MakeTransactionAsync(RpcClient rpcClient, ReadOnlyMemory<byte> script, Signer[] signers = null, TransactionAttribute[] attributes = null)
        {
            var factory = new TransactionManagerFactory(rpcClient);
            return factory.MakeTransactionAsync(script, signers, attributes);
        }

        /// <summary>
        /// Helper function for one-off TransactionManager creation
        /// </summary>
        public static Task<TransactionManager> MakeTransactionAsync(RpcClient rpcClient, ReadOnlyMemory<byte> script, long systemFee, Signer[] signers = null, TransactionAttribute[] attributes = null)
        {
            var factory = new TransactionManagerFactory(rpcClient);
            return factory.MakeTransactionAsync(script, systemFee, signers, attributes);
        }

        /// <summary>
        /// Add Signature
        /// </summary>
        /// <param name="key">The KeyPair to sign transction</param>
        /// <returns></returns>
        public TransactionManager AddSignature(KeyPair key)
        {
            var contract = Contract.CreateSignatureContract(key.PublicKey);
            AddSignItem(contract, key);
            return this;
        }

        /// <summary>
        /// Add Multi-Signature
        /// </summary>
        /// <param name="key">The KeyPair to sign transction</param>
        /// <param name="m">The least count of signatures needed for multiple signature contract</param>
        /// <param name="publicKeys">The Public Keys construct the multiple signature contract</param>
        public TransactionManager AddMultiSig(KeyPair key, int m, params ECPoint[] publicKeys)
        {
            Contract contract = Contract.CreateMultiSigContract(m, publicKeys);
            AddSignItem(contract, key);
            return this;
        }

        /// <summary>
        /// Add Multi-Signature
        /// </summary>
        /// <param name="keys">The KeyPairs to sign transction</param>
        /// <param name="m">The least count of signatures needed for multiple signature contract</param>
        /// <param name="publicKeys">The Public Keys construct the multiple signature contract</param>
        public TransactionManager AddMultiSig(KeyPair[] keys, int m, params ECPoint[] publicKeys)
        {
            Contract contract = Contract.CreateMultiSigContract(m, publicKeys);
            for (int i = 0; i < keys.Length; i++)
            {
                AddSignItem(contract, keys[i]);
            }
            return this;
        }

        private void AddSignItem(Contract contract, KeyPair key)
        {
            if (!Tx.GetScriptHashesForVerifying(null).Contains(contract.ScriptHash))
            {
                throw new Exception($"Add SignItem error: Mismatch ScriptHash ({contract.ScriptHash})");
            }

            SignItem item = signStore.FirstOrDefault(p => p.Contract.ScriptHash == contract.ScriptHash);
            if (item is null)
            {
                signStore.Add(new SignItem { Contract = contract, KeyPairs = new HashSet<KeyPair> { key } });
            }
            else if (!item.KeyPairs.Contains(key))
            {
                item.KeyPairs.Add(key);
            }
        }

        /// <summary>
        /// Add Witness with contract
        /// </summary>
        /// <param name="contract">The witness verification contract</param>
        /// <param name="parameters">The witness invocation parameters</param>
        public TransactionManager AddWitness(Contract contract, params object[] parameters)
        {
            if (!context.Add(contract, parameters))
            {
                throw new Exception("AddWitness failed!");
            };
            return this;
        }

        /// <summary>
        /// Add Witness with scriptHash
        /// </summary>
        /// <param name="scriptHash">The witness verification contract hash</param>
        /// <param name="parameters">The witness invocation parameters</param>
        public TransactionManager AddWitness(UInt160 scriptHash, params object[] parameters)
        {
            var contract = Contract.Create(scriptHash);
            return AddWitness(contract, parameters);
        }

        /// <summary>
        /// Verify Witness count and add witnesses
        /// </summary>
        public async Task<Transaction> SignAsync()
        {
            // Calculate NetworkFee
            Tx.Witnesses = Tx.GetScriptHashesForVerifying(null).Select(u => new Witness()
            {
                InvocationScript = Array.Empty<byte>(),
                VerificationScript = GetVerificationScript(u)
            }).ToArray();
            Tx.NetworkFee = await rpcClient.CalculateNetworkFeeAsync(Tx).ConfigureAwait(false);
            Tx.Witnesses = null;

            var epicpulseBalance = await new Xep17API(rpcClient).BalanceOfAsync(NativeContract.EpicPulse.Hash, Tx.Sender).ConfigureAwait(false);
            if (epicpulseBalance < Tx.SystemFee + Tx.NetworkFee)
                throw new InvalidOperationException($"Insufficient EpicPulse in address: {Tx.Sender.ToAddress(rpcClient.protocolSettings.AddressVersion)}");

            // Sign with signStore
            for (int i = 0; i < signStore.Count; i++)
            {
                foreach (var key in signStore[i].KeyPairs)
                {
                    byte[] signature = Tx.Sign(key, rpcClient.protocolSettings.Network);
                    if (!context.AddSignature(signStore[i].Contract, key.PublicKey, signature))
                    {
                        throw new Exception("AddSignature failed!");
                    }
                }
            }

            // Verify witness count
            if (!context.Completed)
            {
                throw new Exception($"Please add signature or witness first!");
            }
            Tx.Witnesses = context.GetWitnesses();
            return Tx;
        }

        private byte[] GetVerificationScript(UInt160 hash)
        {
            foreach (var item in signStore)
            {
                if (item.Contract.ScriptHash == hash) return item.Contract.Script;
            }

            return Array.Empty<byte>();
        }
    }
}

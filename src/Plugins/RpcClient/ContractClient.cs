// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractClient.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Payloads;
using EpicChain.Network.RPC.Models;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC
{
    /// <summary>
    /// Contract related operations through RPC API
    /// </summary>
    public class ContractClient
    {
        protected readonly RpcClient rpcClient;

        /// <summary>
        /// ContractClient Constructor
        /// </summary>
        /// <param name="rpc">the RPC client to call EpicChain RPC methods</param>
        public ContractClient(RpcClient rpc)
        {
            rpcClient = rpc;
        }

        /// <summary>
        /// Use RPC method to test invoke operation.
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <param name="operation">contract operation</param>
        /// <param name="args">operation arguments</param>
        /// <returns></returns>
        public Task<RpcInvokeResult> TestInvokeAsync(UInt160 scriptHash, string operation, params object[] args)
        {
            byte[] script = scriptHash.MakeScript(operation, args);
            return rpcClient.InvokeScriptAsync(script);
        }

        /// <summary>
        /// Deploy Contract, return signed transaction
        /// </summary>
        /// <param name="nefFile">epicchain contract executable file</param>
        /// <param name="manifest">contract manifest</param>
        /// <param name="key">sender KeyPair</param>
        /// <returns></returns>
        public async Task<Transaction> CreateDeployContractTxAsync(byte[] nefFile, ContractManifest manifest, KeyPair key)
        {
            byte[] script;
            using (ScriptBuilder sb = new ScriptBuilder())
            {
                sb.EmitDynamicCall(NativeContract.ContractManagement.Hash, "deploy", nefFile, manifest.ToJson().ToString());
                script = sb.ToArray();
            }
            UInt160 sender = Contract.CreateSignatureRedeemScript(key.PublicKey).ToScriptHash();
            Signer[] signers = new[] { new Signer { Scopes = WitnessScope.CalledByEntry, Account = sender } };

            TransactionManagerFactory factory = new TransactionManagerFactory(rpcClient);
            TransactionManager manager = await factory.MakeTransactionAsync(script, signers).ConfigureAwait(false);
            return await manager
                .AddSignature(key)
                .SignAsync().ConfigureAwait(false);
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// Xep17API.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using EpicChain.Network.RPC.Models;
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.Wallets;
using System;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC
{
    /// <summary>
    /// Call Xep17 methods with RPC API
    /// </summary>
    public class Xep17API : ContractClient
    {
        /// <summary>
        /// Xep17API Constructor
        /// </summary>
        /// <param name="rpcClient">the RPC client to call EpicChain RPC methods</param>
        public Xep17API(RpcClient rpcClient) : base(rpcClient) { }

        /// <summary>
        /// Get balance of Xep17 token
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <param name="account">account script hash</param>
        /// <returns></returns>
        public async Task<BigInteger> BalanceOfAsync(UInt160 scriptHash, UInt160 account)
        {
            var result = await TestInvokeAsync(scriptHash, "balanceOf", account).ConfigureAwait(false);
            BigInteger balance = result.Stack.Single().GetInteger();
            return balance;
        }

        /// <summary>
        /// Get symbol of Xep17 token
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <returns></returns>
        public async Task<string> SymbolAsync(UInt160 scriptHash)
        {
            var result = await TestInvokeAsync(scriptHash, "symbol").ConfigureAwait(false);
            return result.Stack.Single().GetString();
        }

        /// <summary>
        /// Get decimals of Xep17 token
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <returns></returns>
        public async Task<byte> DecimalsAsync(UInt160 scriptHash)
        {
            var result = await TestInvokeAsync(scriptHash, "decimals").ConfigureAwait(false);
            return (byte)result.Stack.Single().GetInteger();
        }

        /// <summary>
        /// Get total supply of Xep17 token
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <returns></returns>
        public async Task<BigInteger> TotalSupplyAsync(UInt160 scriptHash)
        {
            var result = await TestInvokeAsync(scriptHash, "totalSupply").ConfigureAwait(false);
            return result.Stack.Single().GetInteger();
        }

        /// <summary>
        /// Get token information in one rpc call
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <returns></returns>
        public async Task<RpcXep17TokenInfo> GetTokenInfoAsync(UInt160 scriptHash)
        {
            var contractState = await rpcClient.GetContractStateAsync(scriptHash.ToString()).ConfigureAwait(false);
            byte[] script = [
                .. scriptHash.MakeScript("symbol"),
                .. scriptHash.MakeScript("decimals"),
                .. scriptHash.MakeScript("totalSupply")];
            var name = contractState.Manifest.Name;
            var result = await rpcClient.InvokeScriptAsync(script).ConfigureAwait(false);
            var stack = result.Stack;

            return new RpcXep17TokenInfo
            {
                Name = name,
                Symbol = stack[0].GetString(),
                Decimals = (byte)stack[1].GetInteger(),
                TotalSupply = stack[2].GetInteger()
            };
        }

        public async Task<RpcXep17TokenInfo> GetTokenInfoAsync(string contractHash)
        {
            var contractState = await rpcClient.GetContractStateAsync(contractHash).ConfigureAwait(false);
            byte[] script = [
                .. contractState.Hash.MakeScript("symbol"),
                .. contractState.Hash.MakeScript("decimals"),
                .. contractState.Hash.MakeScript("totalSupply")];
            var name = contractState.Manifest.Name;
            var result = await rpcClient.InvokeScriptAsync(script).ConfigureAwait(false);
            var stack = result.Stack;

            return new RpcXep17TokenInfo
            {
                Name = name,
                Symbol = stack[0].GetString(),
                Decimals = (byte)stack[1].GetInteger(),
                TotalSupply = stack[2].GetInteger()
            };
        }

        /// <summary>
        /// Create Xep17 token transfer transaction
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <param name="fromKey">from KeyPair</param>
        /// <param name="to">to account script hash</param>
        /// <param name="amount">transfer amount</param>
        /// <param name="data">onPayment data</param>
        /// <param name="addAssert">Add assert at the end of the script</param>
        /// <returns></returns>
        public async Task<Transaction> CreateTransferTxAsync(UInt160 scriptHash, KeyPair fromKey, UInt160 to, BigInteger amount, object data = null, bool addAssert = true)
        {
            var sender = Contract.CreateSignatureRedeemScript(fromKey.PublicKey).ToScriptHash();
            Signer[] signers = new[] { new Signer { Scopes = WitnessScope.CalledByEntry, Account = sender } };
            byte[] script = scriptHash.MakeScript("transfer", sender, to, amount, data);
            if (addAssert) script = script.Concat(new[] { (byte)OpCode.ASSERT }).ToArray();

            TransactionManagerFactory factory = new(rpcClient);
            TransactionManager manager = await factory.MakeTransactionAsync(script, signers).ConfigureAwait(false);

            return await manager
                .AddSignature(fromKey)
                .SignAsync().ConfigureAwait(false);
        }

        /// <summary>
        /// Create Xep17 token transfer transaction from multi-sig account
        /// </summary>
        /// <param name="scriptHash">contract script hash</param>
        /// <param name="m">multi-sig min signature count</param>
        /// <param name="pubKeys">multi-sig pubKeys</param>
        /// <param name="fromKeys">sign keys</param>
        /// <param name="to">to account</param>
        /// <param name="amount">transfer amount</param>
        /// <param name="data">onPayment data</param>
        /// <param name="addAssert">Add assert at the end of the script</param>
        /// <returns></returns>
        public async Task<Transaction> CreateTransferTxAsync(UInt160 scriptHash, int m, ECPoint[] pubKeys, KeyPair[] fromKeys, UInt160 to, BigInteger amount, object data = null, bool addAssert = true)
        {
            if (m > fromKeys.Length)
                throw new ArgumentException($"Need at least {m} KeyPairs for signing!");
            var sender = Contract.CreateMultiSigContract(m, pubKeys).ScriptHash;
            Signer[] signers = new[] { new Signer { Scopes = WitnessScope.CalledByEntry, Account = sender } };
            byte[] script = scriptHash.MakeScript("transfer", sender, to, amount, data);
            if (addAssert) script = script.Concat(new[] { (byte)OpCode.ASSERT }).ToArray();

            TransactionManagerFactory factory = new(rpcClient);
            TransactionManager manager = await factory.MakeTransactionAsync(script, signers).ConfigureAwait(false);

            return await manager
                .AddMultiSig(fromKeys, m, pubKeys)
                .SignAsync().ConfigureAwait(false);
        }
    }
}

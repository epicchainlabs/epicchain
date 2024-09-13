// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionManagerFactory.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC
{
    public class TransactionManagerFactory
    {
        private readonly RpcClient rpcClient;

        /// <summary>
        /// TransactionManagerFactory Constructor
        /// </summary>
        /// <param name="rpcClient">the RPC client to call NEO RPC API</param>
        public TransactionManagerFactory(RpcClient rpcClient)
        {
            this.rpcClient = rpcClient;
        }

        /// <summary>
        /// Create an unsigned Transaction object with given parameters.
        /// </summary>
        /// <param name="script">Transaction Script</param>
        /// <param name="signers">Transaction Signers</param>
        /// <param name="attributes">Transaction Attributes</param>
        /// <returns></returns>
        public async Task<TransactionManager> MakeTransactionAsync(ReadOnlyMemory<byte> script, Signer[] signers = null, TransactionAttribute[] attributes = null)
        {
            RpcInvokeResult invokeResult = await rpcClient.InvokeScriptAsync(script, signers).ConfigureAwait(false);
            return await MakeTransactionAsync(script, invokeResult.EpicPulseConsumed, signers, attributes).ConfigureAwait(false);
        }

        /// <summary>
        /// Create an unsigned Transaction object with given parameters.
        /// </summary>
        /// <param name="script">Transaction Script</param>
        /// <param name="systemFee">Transaction System Fee</param>
        /// <param name="signers">Transaction Signers</param>
        /// <param name="attributes">Transaction Attributes</param>
        /// <returns></returns>
        public async Task<TransactionManager> MakeTransactionAsync(ReadOnlyMemory<byte> script, long systemFee, Signer[] signers = null, TransactionAttribute[] attributes = null)
        {
            uint blockCount = await rpcClient.GetBlockCountAsync().ConfigureAwait(false) - 1;

            var tx = new Transaction
            {
                Version = 0,
                Nonce = (uint)new Random().Next(),
                Script = script,
                Signers = signers ?? Array.Empty<Signer>(),
                ValidUntilBlock = blockCount - 1 + rpcClient.protocolSettings.MaxValidUntilBlockIncrement,
                SystemFee = systemFee,
                Attributes = attributes ?? Array.Empty<TransactionAttribute>(),
            };

            return new TransactionManager(tx, rpcClient);
        }
    }
}

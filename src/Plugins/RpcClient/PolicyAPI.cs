// Copyright (C) 2021-2024 EpicChain Labs.

//
// PolicyAPI.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.SmartContract.Native;
using System.Linq;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC
{
    /// <summary>
    /// Get Policy info by RPC API
    /// </summary>
    public class PolicyAPI : ContractClient
    {
        readonly UInt160 scriptHash = NativeContract.Policy.Hash;

        /// <summary>
        /// PolicyAPI Constructor
        /// </summary>
        /// <param name="rpcClient">the RPC client to call EpicChain RPC methods</param>
        public PolicyAPI(RpcClient rpcClient) : base(rpcClient) { }

        /// <summary>
        /// Get Fee Factor
        /// </summary>
        /// <returns></returns>
        public async Task<uint> GetExecFeeFactorAsync()
        {
            var result = await TestInvokeAsync(scriptHash, "getExecFeeFactor").ConfigureAwait(false);
            return (uint)result.Stack.Single().GetInteger();
        }

        /// <summary>
        /// Get Storage Price
        /// </summary>
        /// <returns></returns>
        public async Task<uint> GetStoragePriceAsync()
        {
            var result = await TestInvokeAsync(scriptHash, "getStoragePrice").ConfigureAwait(false);
            return (uint)result.Stack.Single().GetInteger();
        }

        /// <summary>
        /// Get Network Fee Per Byte
        /// </summary>
        /// <returns></returns>
        public async Task<long> GetFeePerByteAsync()
        {
            var result = await TestInvokeAsync(scriptHash, "getFeePerByte").ConfigureAwait(false);
            return (long)result.Stack.Single().GetInteger();
        }

        /// <summary>
        /// Get Ploicy Blocked Accounts
        /// </summary>
        /// <returns></returns>
        public async Task<bool> IsBlockedAsync(UInt160 account)
        {
            var result = await TestInvokeAsync(scriptHash, "isBlocked", new object[] { account }).ConfigureAwait(false);
            return result.Stack.Single().GetBoolean();
        }
    }
}

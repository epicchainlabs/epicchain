// Copyright (C) 2021-2024 EpicChain Labs.

//
// StateAPI.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Json;
using EpicChain.Network.RPC.Models;
using System;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC
{
    public class StateAPI
    {
        private readonly RpcClient rpcClient;

        public StateAPI(RpcClient rpc)
        {
            rpcClient = rpc;
        }

        public async Task<RpcStateRoot> GetStateRootAsync(uint index)
        {
            var result = await rpcClient.RpcSendAsync(RpcClient.GetRpcName(), index).ConfigureAwait(false);
            return RpcStateRoot.FromJson((JObject)result);
        }

        public async Task<byte[]> GetProofAsync(UInt256 rootHash, UInt160 scriptHash, byte[] key)
        {
            var result = await rpcClient.RpcSendAsync(RpcClient.GetRpcName(),
                rootHash.ToString(), scriptHash.ToString(), Convert.ToBase64String(key)).ConfigureAwait(false);
            return Convert.FromBase64String(result.AsString());
        }

        public async Task<byte[]> VerifyProofAsync(UInt256 rootHash, byte[] proofBytes)
        {
            var result = await rpcClient.RpcSendAsync(RpcClient.GetRpcName(),
                rootHash.ToString(), Convert.ToBase64String(proofBytes)).ConfigureAwait(false);

            return Convert.FromBase64String(result.AsString());
        }

        public async Task<(uint? localRootIndex, uint? validatedRootIndex)> GetStateHeightAsync()
        {
            var result = await rpcClient.RpcSendAsync(RpcClient.GetRpcName()).ConfigureAwait(false);
            var localRootIndex = ToNullableUint(result["localrootindex"]);
            var validatedRootIndex = ToNullableUint(result["validatedrootindex"]);
            return (localRootIndex, validatedRootIndex);
        }

        static uint? ToNullableUint(JToken json) => (json == null) ? null : (uint?)json.AsNumber();

        public static JToken[] MakeFindStatesParams(UInt256 rootHash, UInt160 scriptHash, ReadOnlySpan<byte> prefix, ReadOnlySpan<byte> from = default, int? count = null)
        {
            var @params = new JToken[count.HasValue ? 5 : 4];
            @params[0] = rootHash.ToString();
            @params[1] = scriptHash.ToString();
            @params[2] = Convert.ToBase64String(prefix);
            @params[3] = Convert.ToBase64String(from);
            if (count.HasValue)
            {
                @params[4] = count.Value;
            }
            return @params;
        }

        public async Task<RpcFoundStates> FindStatesAsync(UInt256 rootHash, UInt160 scriptHash, ReadOnlyMemory<byte> prefix, ReadOnlyMemory<byte> from = default, int? count = null)
        {
            var @params = MakeFindStatesParams(rootHash, scriptHash, prefix.Span, from.Span, count);
            var result = await rpcClient.RpcSendAsync(RpcClient.GetRpcName(), @params).ConfigureAwait(false);

            return RpcFoundStates.FromJson((JObject)result);
        }

        public async Task<byte[]> GetStateAsync(UInt256 rootHash, UInt160 scriptHash, byte[] key)
        {
            var result = await rpcClient.RpcSendAsync(RpcClient.GetRpcName(),
                rootHash.ToString(), scriptHash.ToString(), Convert.ToBase64String(key)).ConfigureAwait(false);
            return Convert.FromBase64String(result.AsString());
        }
    }
}

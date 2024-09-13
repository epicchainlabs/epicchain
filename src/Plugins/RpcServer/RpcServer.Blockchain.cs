// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcServer.Blockchain.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins.RpcServer.Model;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.Plugins.RpcServer
{
    partial class RpcServer
    {
        /// <summary>
        /// Gets the hash of the best (most recent) block.
        /// </summary>
        /// <returns>The hash of the best block as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetBestBlockHash()
        {
            return NativeContract.Ledger.CurrentHash(system.StoreView).ToString();
        }

        /// <summary>
        /// Gets a block by its hash or index.
        /// </summary>
        /// <param name="blockHashOrIndex">The block hash or index.</param>
        /// <param name="verbose">Optional, the default value is false.</param>
        /// <returns>The block data as a <see cref="JToken"/>. If the second item of _params is true, then
        /// block data is json format, otherwise, the return type is Base64-encoded byte array.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetBlock(BlockHashOrIndex blockHashOrIndex, bool verbose = false)
        {
            using var snapshot = system.GetSnapshotCache();
            var block = blockHashOrIndex.IsIndex ? NativeContract.Ledger.GetBlock(snapshot, blockHashOrIndex.AsIndex()) : NativeContract.Ledger.GetBlock(snapshot, blockHashOrIndex.AsHash());
            block.NotNull_Or(RpcError.UnknownBlock);
            if (verbose)
            {
                JObject json = Utility.BlockToJson(block, system.Settings);
                json["confirmations"] = NativeContract.Ledger.CurrentIndex(snapshot) - block.Index + 1;
                UInt256 hash = NativeContract.Ledger.GetBlockHash(snapshot, block.Index + 1);
                if (hash != null)
                    json["nextblockhash"] = hash.ToString();
                return json;
            }
            return Convert.ToBase64String(block.ToArray());
        }

        /// <summary>
        /// Gets the number of block headers in the blockchain.
        /// </summary>
        /// <returns>The count of block headers as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        internal virtual JToken GetBlockHeaderCount()
        {
            return (system.HeaderCache.Last?.Index ?? NativeContract.Ledger.CurrentIndex(system.StoreView)) + 1;
        }

        /// <summary>
        /// Gets the number of blocks in the blockchain.
        /// </summary>
        /// <returns>The count of blocks as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetBlockCount()
        {
            return NativeContract.Ledger.CurrentIndex(system.StoreView) + 1;
        }

        /// <summary>
        /// Gets the hash of the block at the specified height.
        /// </summary>
        /// <param name="height">Block index (block height)</param>
        /// <returns>The hash of the block at the specified height as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetBlockHash(uint height)
        {
            var snapshot = system.StoreView;
            if (height <= NativeContract.Ledger.CurrentIndex(snapshot))
            {
                return NativeContract.Ledger.GetBlockHash(snapshot, height).ToString();
            }
            throw new RpcException(RpcError.UnknownHeight);
        }

        /// <summary>
        /// Gets a block header by its hash or index.
        /// </summary>
        /// <param name="blockHashOrIndex">The block script hash or index (i.e. block height=number of blocks - 1).</param>
        /// <param name="verbose">Optional, the default value is false.</param>
        /// <remarks>
        /// When verbose is false, serialized information of the block is returned in a hexadecimal string.
        /// If you need the detailed information, use the SDK for deserialization.
        /// When verbose is true or 1, detailed information of the block is returned in Json format.
        /// </remarks>
        /// <returns>The block header data as a <see cref="JToken"/>. In json format if the second item of _params is true, otherwise Base64-encoded byte array.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetBlockHeader(BlockHashOrIndex blockHashOrIndex, bool verbose = false)
        {
            var snapshot = system.StoreView;
            Header header;
            if (blockHashOrIndex.IsIndex)
            {
                header = NativeContract.Ledger.GetHeader(snapshot, blockHashOrIndex.AsIndex()).NotNull_Or(RpcError.UnknownBlock);
            }
            else
            {
                header = NativeContract.Ledger.GetHeader(snapshot, blockHashOrIndex.AsHash()).NotNull_Or(RpcError.UnknownBlock);
            }
            if (verbose)
            {
                JObject json = header.ToJson(system.Settings);
                json["confirmations"] = NativeContract.Ledger.CurrentIndex(snapshot) - header.Index + 1;
                UInt256 hash = NativeContract.Ledger.GetBlockHash(snapshot, header.Index + 1);
                if (hash != null)
                    json["nextblockhash"] = hash.ToString();
                return json;
            }

            return Convert.ToBase64String(header.ToArray());
        }

        /// <summary>
        /// Gets the state of a contract by its ID or script hash or (only for native contracts) by case-insensitive name.
        /// </summary>
        /// <param name="contractNameOrHashOrId">Contract name or script hash or the native contract id.</param>
        /// <returns>The contract state in json format as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetContractState(ContractNameOrHashOrId contractNameOrHashOrId)
        {
            if (contractNameOrHashOrId.IsId)
            {
                var contractState = NativeContract.ContractManagement.GetContractById(system.StoreView, contractNameOrHashOrId.AsId());
                return contractState.NotNull_Or(RpcError.UnknownContract).ToJson();
            }

            var hash = contractNameOrHashOrId.IsName ? ToScriptHash(contractNameOrHashOrId.AsName()) : contractNameOrHashOrId.AsHash();
            var contract = NativeContract.ContractManagement.GetContract(system.StoreView, hash);
            return contract.NotNull_Or(RpcError.UnknownContract).ToJson();
        }

        private static UInt160 ToScriptHash(string keyword)
        {
            foreach (var native in NativeContract.Contracts)
            {
                if (keyword.Equals(native.Name, StringComparison.InvariantCultureIgnoreCase) || keyword == native.Id.ToString())
                    return native.Hash;
            }

            return UInt160.Parse(keyword);
        }

        /// <summary>
        /// Gets the current memory pool transactions.
        /// </summary>
        /// <param name="shouldGetUnverified">Optional, the default value is false.</param>
        /// <returns>The memory pool transactions in json format as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetRawMemPool(bool shouldGetUnverified = false)
        {
            if (!shouldGetUnverified)
                return new JArray(system.MemPool.GetVerifiedTransactions().Select(p => (JToken)p.Hash.ToString()));

            JObject json = new();
            json["height"] = NativeContract.Ledger.CurrentIndex(system.StoreView);
            system.MemPool.GetVerifiedAndUnverifiedTransactions(
                out IEnumerable<Transaction> verifiedTransactions,
                out IEnumerable<Transaction> unverifiedTransactions);
            json["verified"] = new JArray(verifiedTransactions.Select(p => (JToken)p.Hash.ToString()));
            json["unverified"] = new JArray(unverifiedTransactions.Select(p => (JToken)p.Hash.ToString()));
            return json;
        }

        /// <summary>
        /// Gets a transaction by its hash.
        /// </summary>
        /// <param name="hash">The transaction hash.</param>
        /// <param name="verbose">Optional, the default value is false.</param>
        /// <returns>The transaction data as a <see cref="JToken"/>. In json format if verbose is true, otherwise base64string. </returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetRawTransaction(UInt256 hash, bool verbose = false)
        {
            if (system.MemPool.TryGetValue(hash, out var tx) && !verbose)
                return Convert.ToBase64String(tx.ToArray());
            var snapshot = system.StoreView;
            var state = NativeContract.Ledger.GetTransactionState(snapshot, hash);
            tx ??= state?.Transaction;
            tx.NotNull_Or(RpcError.UnknownTransaction);
            if (!verbose) return Convert.ToBase64String(tx.ToArray());
            var json = Utility.TransactionToJson(tx, system.Settings);
            if (state is not null)
            {
                var block = NativeContract.Ledger.GetTrimmedBlock(snapshot, NativeContract.Ledger.GetBlockHash(snapshot, state.BlockIndex));
                json["blockhash"] = block.Hash.ToString();
                json["confirmations"] = NativeContract.Ledger.CurrentIndex(snapshot) - block.Index + 1;
                json["blocktime"] = block.Header.Timestamp;
            }
            return json;
        }

        /// <summary>
        /// Gets the storage item by contract ID or script hash and key.
        /// </summary>
        /// <param name="contractNameOrHashOrId">The contract ID or script hash.</param>
        /// <param name="base64Key">The Base64-encoded storage key.</param>
        /// <returns>The storage item as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetStorage(ContractNameOrHashOrId contractNameOrHashOrId, string base64Key)
        {
            using var snapshot = system.GetSnapshotCache();
            int id;
            if (contractNameOrHashOrId.IsHash)
            {
                var hash = contractNameOrHashOrId.AsHash();
                var contract = NativeContract.ContractManagement.GetContract(snapshot, hash).NotNull_Or(RpcError.UnknownContract);
                id = contract.Id;
            }
            else
            {
                id = contractNameOrHashOrId.AsId();
            }
            var key = Convert.FromBase64String(base64Key);
            var item = snapshot.TryGet(new StorageKey
            {
                Id = id,
                Key = key
            }).NotNull_Or(RpcError.UnknownStorageItem);
            return Convert.ToBase64String(item.Value.Span);
        }

        /// <summary>
        /// Finds storage items by contract ID or script hash and prefix.
        /// </summary>
        /// <param name="contractNameOrHashOrId">The contract ID (int) or script hash (UInt160).</param>
        /// <param name="base64KeyPrefix">The Base64-encoded storage key prefix.</param>
        /// <param name="start">The start index.</param>
        /// <returns>The found storage items <see cref="StorageItem"/> as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken FindStorage(ContractNameOrHashOrId contractNameOrHashOrId, string base64KeyPrefix, int start = 0)
        {
            using var snapshot = system.GetSnapshotCache();
            int id;
            if (contractNameOrHashOrId.IsHash)
            {
                ContractState contract = NativeContract.ContractManagement.GetContract(snapshot, contractNameOrHashOrId.AsHash()).NotNull_Or(RpcError.UnknownContract);
                id = contract.Id;
            }
            else
            {
                id = contractNameOrHashOrId.AsId();
            }

            byte[] prefix = Result.Ok_Or(() => Convert.FromBase64String(base64KeyPrefix), RpcError.InvalidParams.WithData($"Invalid Base64 string{base64KeyPrefix}"));
            byte[] prefix_key = StorageKey.CreateSearchPrefix(id, prefix);

            JObject json = new();
            JArray jarr = new();
            int pageSize = settings.FindStoragePageSize;
            int i = 0;

            using (var iter = snapshot.Find(prefix_key).Skip(count: start).GetEnumerator())
            {
                var hasMore = false;
                while (iter.MoveNext())
                {
                    if (i == pageSize)
                    {
                        hasMore = true;
                        break;
                    }

                    JObject j = new();
                    j["key"] = Convert.ToBase64String(iter.Current.Key.Key.Span);
                    j["value"] = Convert.ToBase64String(iter.Current.Value.Value.Span);
                    jarr.Add(j);
                    i++;
                }
                json["truncated"] = hasMore;
            }

            json["next"] = start + i;
            json["results"] = jarr;
            return json;
        }

        /// <summary>
        /// Gets the height of a transaction by its hash.
        /// </summary>
        /// <param name="hash">The transaction hash.</param>
        /// <returns>The height of the transaction as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetTransactionHeight(UInt256 hash)
        {
            uint? height = NativeContract.Ledger.GetTransactionState(system.StoreView, hash)?.BlockIndex;
            if (height.HasValue) return height.Value;
            throw new RpcException(RpcError.UnknownTransaction);
        }

        /// <summary>
        /// Gets the next block validators.
        /// </summary>
        /// <returns>The next block validators as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetNextBlockValidators()
        {
            using var snapshot = system.GetSnapshotCache();
            var validators = NativeContract.EpicChain.GetNextBlockValidators(snapshot, system.Settings.ValidatorsCount);
            return validators.Select(p =>
            {
                JObject validator = new();
                validator["publickey"] = p.ToString();
                validator["votes"] = (int)NativeContract.EpicChain.GetCandidateVote(snapshot, p);
                return validator;
            }).ToArray();
        }

        /// <summary>
        /// Gets the list of candidates for the next block validators.
        /// </summary>
        /// <returns>The candidates public key list as a JToken.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetCandidates()
        {
            using var snapshot = system.GetSnapshotCache();
            byte[] script;
            using (ScriptBuilder sb = new())
            {
                script = sb.EmitDynamicCall(NativeContract.EpicChain.Hash, "getCandidates", null).ToArray();
            }
            StackItem[] resultstack;
            try
            {
                using ApplicationEngine engine = ApplicationEngine.Run(script, snapshot, settings: system.Settings, epicpulse: settings.maxEpicPulseInvoke);
                resultstack = engine.ResultStack.ToArray();
            }
            catch
            {
                throw new RpcException(RpcError.InternalServerError.WithData("Can't get candidates."));
            }

            JObject json = new();
            try
            {
                if (resultstack.Length > 0)
                {
                    JArray jArray = new();
                    var validators = NativeContract.EpicChain.GetNextBlockValidators(snapshot, system.Settings.ValidatorsCount) ?? throw new RpcException(RpcError.InternalServerError.WithData("Can't get next block validators."));

                    foreach (var item in resultstack)
                    {
                        var value = (VM.Types.Array)item;
                        foreach (Struct ele in value)
                        {
                            var publickey = ele[0].GetSpan().ToHexString();
                            json["publickey"] = publickey;
                            json["votes"] = ele[1].GetInteger().ToString();
                            json["active"] = validators.ToByteArray().ToHexString().Contains(publickey);
                            jArray.Add(json);
                            json = new();
                        }
                        return jArray;
                    }
                }
            }
            catch
            {
                throw new RpcException(RpcError.InternalServerError.WithData("Can't get next block validators"));
            }

            return json;
        }

        /// <summary>
        /// Gets the list of committee members.
        /// </summary>
        /// <returns>The committee members publickeys as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetCommittee()
        {
            return new JArray(NativeContract.EpicChain.GetCommittee(system.StoreView).Select(p => (JToken)p.ToString()));
        }

        /// <summary>
        /// Gets the list of native contracts.
        /// </summary>
        /// <returns>The native contract states <see cref="ContractState"/> as a <see cref="JToken"/>.</returns>
        [RpcMethodWithParams]
        protected internal virtual JToken GetNativeContracts()
        {
            return new JArray(NativeContract.Contracts.Select(p => NativeContract.ContractManagement.GetContract(system.StoreView, p.Hash).ToJson()));
        }
    }
}

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
// OracleContract.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

#pragma warning disable IDE0051

using Neo.Cryptography;
using Neo.IO;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract.Manifest;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Buffers.Binary;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace Neo.SmartContract.Native
{
    /// <summary>
    /// The native Oracle service for NEO system.
    /// </summary>
    public sealed class OracleContract : NativeContract
    {
        private const int MaxUrlLength = 256;
        private const int MaxFilterLength = 128;
        private const int MaxCallbackLength = 32;
        private const int MaxUserDataLength = 512;

        private const byte Prefix_Price = 5;
        private const byte Prefix_RequestId = 9;
        private const byte Prefix_Request = 7;
        private const byte Prefix_IdList = 6;

        internal OracleContract()
        {
            var events = new List<ContractEventDescriptor>(Manifest.Abi.Events)
            {
                new ContractEventDescriptor
                {
                    Name = "OracleRequest",
                    Parameters = new ContractParameterDefinition[]
                    {
                        new ContractParameterDefinition()
                        {
                            Name = "Id",
                            Type = ContractParameterType.Integer
                        },
                        new ContractParameterDefinition()
                        {
                            Name = "RequestContract",
                            Type = ContractParameterType.Hash160
                        },
                        new ContractParameterDefinition()
                        {
                            Name = "Url",
                            Type = ContractParameterType.String
                        },
                        new ContractParameterDefinition()
                        {
                            Name = "Filter",
                            Type = ContractParameterType.String
                        }
                    }
                },
                new ContractEventDescriptor
                {
                    Name = "OracleResponse",
                    Parameters = new ContractParameterDefinition[]
                    {
                        new ContractParameterDefinition()
                        {
                            Name = "Id",
                            Type = ContractParameterType.Integer
                        },
                        new ContractParameterDefinition()
                        {
                            Name = "OriginalTx",
                            Type = ContractParameterType.Hash256
                        }
                    }
                }
            };

            Manifest.Abi.Events = events.ToArray();
        }

        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.States)]
        private void SetPrice(ApplicationEngine engine, long price)
        {
            if (price <= 0)
                throw new ArgumentOutOfRangeException(nameof(price));
            if (!CheckCommittee(engine)) throw new InvalidOperationException();
            engine.Snapshot.GetAndChange(CreateStorageKey(Prefix_Price)).Set(price);
        }

        /// <summary>
        /// Gets the price for an Oracle request.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The price for an Oracle request.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public long GetPrice(DataCache snapshot)
        {
            return (long)(BigInteger)snapshot[CreateStorageKey(Prefix_Price)];
        }

        [ContractMethod(RequiredCallFlags = CallFlags.States | CallFlags.AllowCall | CallFlags.AllowNotify)]
        private ContractTask Finish(ApplicationEngine engine)
        {
            if (engine.InvocationStack.Count != 2) throw new InvalidOperationException();
            if (engine.GetInvocationCounter() != 1) throw new InvalidOperationException();
            Transaction tx = (Transaction)engine.ScriptContainer;
            OracleResponse response = tx.GetAttribute<OracleResponse>();
            if (response == null) throw new ArgumentException("Oracle response was not found");
            OracleRequest request = GetRequest(engine.Snapshot, response.Id);
            if (request == null) throw new ArgumentException("Oracle request was not found");
            engine.SendNotification(Hash, "OracleResponse", new VM.Types.Array(engine.ReferenceCounter) { response.Id, request.OriginalTxid.ToArray() });
            StackItem userData = BinarySerializer.Deserialize(request.UserData, engine.Limits, engine.ReferenceCounter);
            return engine.CallFromNativeContract(Hash, request.CallbackContract, request.CallbackMethod, request.Url, userData, (int)response.Code, response.Result);
        }

        private UInt256 GetOriginalTxid(ApplicationEngine engine)
        {
            Transaction tx = (Transaction)engine.ScriptContainer;
            OracleResponse response = tx.GetAttribute<OracleResponse>();
            if (response is null) return tx.Hash;
            OracleRequest request = GetRequest(engine.Snapshot, response.Id);
            return request.OriginalTxid;
        }

        /// <summary>
        /// Gets a pending request with the specified id.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="id">The id of the request.</param>
        /// <returns>The pending request. Or <see langword="null"/> if no request with the specified id is found.</returns>
        public OracleRequest GetRequest(DataCache snapshot, ulong id)
        {
            return snapshot.TryGet(CreateStorageKey(Prefix_Request).AddBigEndian(id))?.GetInteroperable<OracleRequest>();
        }

        /// <summary>
        /// Gets all the pending requests.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>All the pending requests.</returns>
        public IEnumerable<(ulong, OracleRequest)> GetRequests(DataCache snapshot)
        {
            return snapshot.Find(CreateStorageKey(Prefix_Request).ToArray()).Select(p => (BinaryPrimitives.ReadUInt64BigEndian(p.Key.Key.Span[1..]), p.Value.GetInteroperable<OracleRequest>()));
        }

        /// <summary>
        /// Gets the requests with the specified url.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="url">The url of the requests.</param>
        /// <returns>All the requests with the specified url.</returns>
        public IEnumerable<(ulong, OracleRequest)> GetRequestsByUrl(DataCache snapshot, string url)
        {
            IdList list = snapshot.TryGet(CreateStorageKey(Prefix_IdList).Add(GetUrlHash(url)))?.GetInteroperable<IdList>();
            if (list is null) yield break;
            foreach (ulong id in list)
                yield return (id, snapshot[CreateStorageKey(Prefix_Request).AddBigEndian(id)].GetInteroperable<OracleRequest>());
        }

        private static byte[] GetUrlHash(string url)
        {
            return Crypto.Hash160(Utility.StrictUTF8.GetBytes(url));
        }

        internal override ContractTask Initialize(ApplicationEngine engine)
        {
            engine.Snapshot.Add(CreateStorageKey(Prefix_RequestId), new StorageItem(BigInteger.Zero));
            engine.Snapshot.Add(CreateStorageKey(Prefix_Price), new StorageItem(0_50000000));
            return ContractTask.CompletedTask;
        }

        internal override async ContractTask PostPersist(ApplicationEngine engine)
        {
            (UInt160 Account, BigInteger GAS)[] nodes = null;
            foreach (Transaction tx in engine.PersistingBlock.Transactions)
            {
                //Filter the response transactions
                OracleResponse response = tx.GetAttribute<OracleResponse>();
                if (response is null) continue;

                //Remove the request from storage
                StorageKey key = CreateStorageKey(Prefix_Request).AddBigEndian(response.Id);
                OracleRequest request = engine.Snapshot.TryGet(key)?.GetInteroperable<OracleRequest>();
                if (request == null) continue;
                engine.Snapshot.Delete(key);

                //Remove the id from IdList
                key = CreateStorageKey(Prefix_IdList).Add(GetUrlHash(request.Url));
                IdList list = engine.Snapshot.GetAndChange(key).GetInteroperable<IdList>();
                if (!list.Remove(response.Id)) throw new InvalidOperationException();
                if (list.Count == 0) engine.Snapshot.Delete(key);

                //Mint GAS for oracle nodes
                nodes ??= RoleManagement.GetDesignatedByRole(engine.Snapshot, Role.Oracle, engine.PersistingBlock.Index).Select(p => (Contract.CreateSignatureRedeemScript(p).ToScriptHash(), BigInteger.Zero)).ToArray();
                if (nodes.Length > 0)
                {
                    int index = (int)(response.Id % (ulong)nodes.Length);
                    nodes[index].GAS += GetPrice(engine.Snapshot);
                }
            }
            if (nodes != null)
            {
                foreach (var (account, gas) in nodes)
                {
                    if (gas.Sign > 0)
                        await GAS.Mint(engine, account, gas, false);
                }
            }
        }

        [ContractMethod(RequiredCallFlags = CallFlags.States | CallFlags.AllowNotify)]
        private async ContractTask Request(ApplicationEngine engine, string url, string filter, string callback, StackItem userData, long gasForResponse)
        {
            //Check the arguments
            if (Utility.StrictUTF8.GetByteCount(url) > MaxUrlLength
                || (filter != null && Utility.StrictUTF8.GetByteCount(filter) > MaxFilterLength)
                || Utility.StrictUTF8.GetByteCount(callback) > MaxCallbackLength || callback.StartsWith('_')
                || gasForResponse < 0_10000000)
                throw new ArgumentException();

            engine.AddGas(GetPrice(engine.Snapshot));

            //Mint gas for the response
            engine.AddGas(gasForResponse);
            await GAS.Mint(engine, Hash, gasForResponse, false);

            //Increase the request id
            StorageItem item_id = engine.Snapshot.GetAndChange(CreateStorageKey(Prefix_RequestId));
            ulong id = (ulong)(BigInteger)item_id;
            item_id.Add(1);

            //Put the request to storage
            if (ContractManagement.GetContract(engine.Snapshot, engine.CallingScriptHash) is null)
                throw new InvalidOperationException();
            engine.Snapshot.Add(CreateStorageKey(Prefix_Request).AddBigEndian(id), new StorageItem(new OracleRequest
            {
                OriginalTxid = GetOriginalTxid(engine),
                GasForResponse = gasForResponse,
                Url = url,
                Filter = filter,
                CallbackContract = engine.CallingScriptHash,
                CallbackMethod = callback,
                UserData = BinarySerializer.Serialize(userData, MaxUserDataLength, engine.Limits.MaxStackSize)
            }));

            //Add the id to the IdList
            var list = engine.Snapshot.GetAndChange(CreateStorageKey(Prefix_IdList).Add(GetUrlHash(url)), () => new StorageItem(new IdList())).GetInteroperable<IdList>();
            if (list.Count >= 256)
                throw new InvalidOperationException("There are too many pending responses for this url");
            list.Add(id);

            engine.SendNotification(Hash, "OracleRequest", new VM.Types.Array(engine.ReferenceCounter) { id, engine.CallingScriptHash.ToArray(), url, filter ?? StackItem.Null });
        }

        [ContractMethod(CpuFee = 1 << 15)]
        private bool Verify(ApplicationEngine engine)
        {
            Transaction tx = (Transaction)engine.ScriptContainer;
            return tx?.GetAttribute<OracleResponse>() != null;
        }

        private class IdList : InteroperableList<ulong>
        {
            protected override ulong ElementFromStackItem(StackItem item)
            {
                return (ulong)item.GetInteger();
            }

            protected override StackItem ElementToStackItem(ulong element, ReferenceCounter referenceCounter)
            {
                return element;
            }
        }
    }
}

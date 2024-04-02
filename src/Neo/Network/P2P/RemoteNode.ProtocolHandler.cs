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
// RemoteNode.ProtocolHandler.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Akka.Actor;
using Neo.Cryptography;
using Neo.IO.Caching;
using Neo.Ledger;
using Neo.Network.P2P.Capabilities;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract.Native;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Net;

namespace Neo.Network.P2P
{
    public delegate bool MessageReceivedHandler(NeoSystem system, Message message);

    partial class RemoteNode
    {
        private class Timer { }
        private class PendingKnownHashesCollection : KeyedCollectionSlim<UInt256, (UInt256, DateTime)>
        {
            protected override UInt256 GetKeyForItem((UInt256, DateTime) item)
            {
                return item.Item1;
            }
        }

        public static event MessageReceivedHandler MessageReceived
        {
            add => handlers.Add(value);
            remove => handlers.Remove(value);
        }

        private static readonly List<MessageReceivedHandler> handlers = new();
        private readonly PendingKnownHashesCollection pendingKnownHashes = new();
        private readonly HashSetCache<UInt256> knownHashes;
        private readonly HashSetCache<UInt256> sentHashes;
        private bool verack = false;
        private BloomFilter bloom_filter;

        private static readonly TimeSpan TimerInterval = TimeSpan.FromSeconds(30);
        private readonly ICancelable timer = Context.System.Scheduler.ScheduleTellRepeatedlyCancelable(TimerInterval, TimerInterval, Context.Self, new Timer(), ActorRefs.NoSender);

        private void OnMessage(Message msg)
        {
            foreach (MessageReceivedHandler handler in handlers)
                if (!handler(system, msg))
                    return;
            if (Version == null)
            {
                if (msg.Command != MessageCommand.Version)
                    throw new ProtocolViolationException();
                OnVersionMessageReceived((VersionPayload)msg.Payload);
                return;
            }
            if (!verack)
            {
                if (msg.Command != MessageCommand.Verack)
                    throw new ProtocolViolationException();
                OnVerackMessageReceived();
                return;
            }
            switch (msg.Command)
            {
                case MessageCommand.Addr:
                    OnAddrMessageReceived((AddrPayload)msg.Payload);
                    break;
                case MessageCommand.Block:
                case MessageCommand.Extensible:
                    OnInventoryReceived((IInventory)msg.Payload);
                    break;
                case MessageCommand.FilterAdd:
                    OnFilterAddMessageReceived((FilterAddPayload)msg.Payload);
                    break;
                case MessageCommand.FilterClear:
                    OnFilterClearMessageReceived();
                    break;
                case MessageCommand.FilterLoad:
                    OnFilterLoadMessageReceived((FilterLoadPayload)msg.Payload);
                    break;
                case MessageCommand.GetAddr:
                    OnGetAddrMessageReceived();
                    break;
                case MessageCommand.GetBlocks:
                    OnGetBlocksMessageReceived((GetBlocksPayload)msg.Payload);
                    break;
                case MessageCommand.GetBlockByIndex:
                    OnGetBlockByIndexMessageReceived((GetBlockByIndexPayload)msg.Payload);
                    break;
                case MessageCommand.GetData:
                    OnGetDataMessageReceived((InvPayload)msg.Payload);
                    break;
                case MessageCommand.GetHeaders:
                    OnGetHeadersMessageReceived((GetBlockByIndexPayload)msg.Payload);
                    break;
                case MessageCommand.Headers:
                    OnHeadersMessageReceived((HeadersPayload)msg.Payload);
                    break;
                case MessageCommand.Inv:
                    OnInvMessageReceived((InvPayload)msg.Payload);
                    break;
                case MessageCommand.Mempool:
                    OnMemPoolMessageReceived();
                    break;
                case MessageCommand.Ping:
                    OnPingMessageReceived((PingPayload)msg.Payload);
                    break;
                case MessageCommand.Pong:
                    OnPongMessageReceived((PingPayload)msg.Payload);
                    break;
                case MessageCommand.Transaction:
                    if (msg.Payload.Size <= Transaction.MaxTransactionSize)
                        OnInventoryReceived((Transaction)msg.Payload);
                    break;
                case MessageCommand.Verack:
                case MessageCommand.Version:
                    throw new ProtocolViolationException();
                case MessageCommand.Alert:
                case MessageCommand.MerkleBlock:
                case MessageCommand.NotFound:
                case MessageCommand.Reject:
                default: break;
            }
        }

        private void OnAddrMessageReceived(AddrPayload payload)
        {
            ref bool sent = ref sentCommands[(byte)MessageCommand.GetAddr];
            if (!sent) return;
            sent = false;
            system.LocalNode.Tell(new Peer.Peers
            {
                EndPoints = payload.AddressList.Select(p => p.EndPoint).Where(p => p.Port > 0)
            });
        }

        private void OnFilterAddMessageReceived(FilterAddPayload payload)
        {
            bloom_filter?.Add(payload.Data);
        }

        private void OnFilterClearMessageReceived()
        {
            bloom_filter = null;
        }

        private void OnFilterLoadMessageReceived(FilterLoadPayload payload)
        {
            bloom_filter = new BloomFilter(payload.Filter.Length * 8, payload.K, payload.Tweak, payload.Filter);
        }

        /// <summary>
        /// Will be triggered when a MessageCommand.GetAddr message is received.
        /// Randomly select nodes from the local RemoteNodes and tells to RemoteNode actors a MessageCommand.Addr message.
        /// The message contains a list of networkAddresses from those selected random peers.
        /// </summary>
        private void OnGetAddrMessageReceived()
        {
            Random rand = new();
            IEnumerable<RemoteNode> peers = localNode.RemoteNodes.Values
                .Where(p => p.ListenerTcpPort > 0)
                .GroupBy(p => p.Remote.Address, (k, g) => g.First())
                .OrderBy(p => rand.Next())
                .Take(AddrPayload.MaxCountToSend);
            NetworkAddressWithTime[] networkAddresses = peers.Select(p => NetworkAddressWithTime.Create(p.Listener.Address, p.Version.Timestamp, p.Version.Capabilities)).ToArray();
            if (networkAddresses.Length == 0) return;
            EnqueueMessage(Message.Create(MessageCommand.Addr, AddrPayload.Create(networkAddresses)));
        }

        /// <summary>
        /// Will be triggered when a MessageCommand.GetBlocks message is received.
        /// Tell the specified number of blocks' hashes starting with the requested HashStart until payload.Count or MaxHashesCount
        /// Responses are sent to RemoteNode actor as MessageCommand.Inv Message.
        /// </summary>
        /// <param name="payload">A GetBlocksPayload including start block Hash and number of blocks requested.</param>
        private void OnGetBlocksMessageReceived(GetBlocksPayload payload)
        {
            // The default value of payload.Count is -1
            int count = payload.Count < 0 || payload.Count > InvPayload.MaxHashesCount ? InvPayload.MaxHashesCount : payload.Count;
            DataCache snapshot = system.StoreView;
            UInt256 hash = payload.HashStart;
            TrimmedBlock state = NativeContract.Ledger.GetTrimmedBlock(snapshot, hash);
            if (state == null) return;
            uint currentHeight = NativeContract.Ledger.CurrentIndex(snapshot);
            List<UInt256> hashes = new();
            for (uint i = 1; i <= count; i++)
            {
                uint index = state.Index + i;
                if (index > currentHeight)
                    break;
                hash = NativeContract.Ledger.GetBlockHash(snapshot, index);
                if (hash == null) break;
                hashes.Add(hash);
            }
            if (hashes.Count == 0) return;
            EnqueueMessage(Message.Create(MessageCommand.Inv, InvPayload.Create(InventoryType.Block, hashes.ToArray())));
        }

        private void OnGetBlockByIndexMessageReceived(GetBlockByIndexPayload payload)
        {
            uint count = payload.Count == -1 ? InvPayload.MaxHashesCount : Math.Min((uint)payload.Count, InvPayload.MaxHashesCount);
            for (uint i = payload.IndexStart, max = payload.IndexStart + count; i < max; i++)
            {
                Block block = NativeContract.Ledger.GetBlock(system.StoreView, i);
                if (block == null)
                    break;

                if (bloom_filter == null)
                {
                    EnqueueMessage(Message.Create(MessageCommand.Block, block));
                }
                else
                {
                    BitArray flags = new(block.Transactions.Select(p => bloom_filter.Test(p)).ToArray());
                    EnqueueMessage(Message.Create(MessageCommand.MerkleBlock, MerkleBlockPayload.Create(block, flags)));
                }
            }
        }

        /// <summary>
        /// Will be triggered when a MessageCommand.GetData message is received.
        /// The payload includes an array of hash values.
        /// For different payload.Type (Tx, Block, Consensus),
        /// get the corresponding (Txs, Blocks, Consensus) and tell them to RemoteNode actor.
        /// </summary>
        /// <param name="payload">The payload containing the requested information.</param>
        private void OnGetDataMessageReceived(InvPayload payload)
        {
            var notFound = new List<UInt256>();
            foreach (UInt256 hash in payload.Hashes.Where(p => sentHashes.Add(p)))
            {
                switch (payload.Type)
                {
                    case InventoryType.TX:
                        if (system.MemPool.TryGetValue(hash, out Transaction tx))
                            EnqueueMessage(Message.Create(MessageCommand.Transaction, tx));
                        else
                            notFound.Add(hash);
                        break;
                    case InventoryType.Block:
                        Block block = NativeContract.Ledger.GetBlock(system.StoreView, hash);
                        if (block != null)
                        {
                            if (bloom_filter == null)
                            {
                                EnqueueMessage(Message.Create(MessageCommand.Block, block));
                            }
                            else
                            {
                                BitArray flags = new(block.Transactions.Select(p => bloom_filter.Test(p)).ToArray());
                                EnqueueMessage(Message.Create(MessageCommand.MerkleBlock, MerkleBlockPayload.Create(block, flags)));
                            }
                        }
                        else
                        {
                            notFound.Add(hash);
                        }
                        break;
                    default:
                        if (system.RelayCache.TryGet(hash, out IInventory inventory))
                            EnqueueMessage(Message.Create((MessageCommand)payload.Type, inventory));
                        break;
                }
            }

            if (notFound.Count > 0)
            {
                foreach (InvPayload entry in InvPayload.CreateGroup(payload.Type, notFound.ToArray()))
                    EnqueueMessage(Message.Create(MessageCommand.NotFound, entry));
            }
        }

        /// <summary>
        /// Will be triggered when a MessageCommand.GetHeaders message is received.
        /// Tell the specified number of blocks' headers starting with the requested IndexStart to RemoteNode actor.
        /// A limit set by HeadersPayload.MaxHeadersCount is also applied to the number of requested Headers, namely payload.Count.
        /// </summary>
        /// <param name="payload">A GetBlockByIndexPayload including start block index and number of blocks' headers requested.</param>
        private void OnGetHeadersMessageReceived(GetBlockByIndexPayload payload)
        {
            DataCache snapshot = system.StoreView;
            if (payload.IndexStart > NativeContract.Ledger.CurrentIndex(snapshot)) return;
            List<Header> headers = new();
            uint count = payload.Count == -1 ? HeadersPayload.MaxHeadersCount : (uint)payload.Count;
            for (uint i = 0; i < count; i++)
            {
                var header = NativeContract.Ledger.GetHeader(snapshot, payload.IndexStart + i);
                if (header == null) break;
                headers.Add(header);
            }
            if (headers.Count == 0) return;
            EnqueueMessage(Message.Create(MessageCommand.Headers, HeadersPayload.Create(headers.ToArray())));
        }

        private void OnHeadersMessageReceived(HeadersPayload payload)
        {
            UpdateLastBlockIndex(payload.Headers[^1].Index);
            system.Blockchain.Tell(payload.Headers);
        }

        private void OnInventoryReceived(IInventory inventory)
        {
            if (!knownHashes.Add(inventory.Hash)) return;
            pendingKnownHashes.Remove(inventory.Hash);
            system.TaskManager.Tell(inventory);
            switch (inventory)
            {
                case Transaction transaction:
                    if (!(system.ContainsTransaction(transaction.Hash) != ContainsTransactionType.NotExist || system.ContainsConflictHash(transaction.Hash, transaction.Signers.Select(s => s.Account))))
                        system.TxRouter.Tell(new TransactionRouter.Preverify(transaction, true));
                    break;
                case Block block:
                    UpdateLastBlockIndex(block.Index);
                    if (block.Index > NativeContract.Ledger.CurrentIndex(system.StoreView) + InvPayload.MaxHashesCount) return;
                    system.Blockchain.Tell(inventory);
                    break;
                default:
                    system.Blockchain.Tell(inventory);
                    break;
            }
        }

        private void OnInvMessageReceived(InvPayload payload)
        {
            UInt256[] hashes = payload.Hashes.Where(p => !pendingKnownHashes.Contains(p) && !knownHashes.Contains(p) && !sentHashes.Contains(p)).ToArray();
            if (hashes.Length == 0) return;
            switch (payload.Type)
            {
                case InventoryType.Block:
                    {
                        DataCache snapshot = system.StoreView;
                        hashes = hashes.Where(p => !NativeContract.Ledger.ContainsBlock(snapshot, p)).ToArray();
                    }
                    break;
                case InventoryType.TX:
                    {
                        DataCache snapshot = system.StoreView;
                        hashes = hashes.Where(p => !NativeContract.Ledger.ContainsTransaction(snapshot, p)).ToArray();
                    }
                    break;
            }
            if (hashes.Length == 0) return;
            foreach (UInt256 hash in hashes)
                pendingKnownHashes.Add((hash, TimeProvider.Current.UtcNow));
            system.TaskManager.Tell(new TaskManager.NewTasks { Payload = InvPayload.Create(payload.Type, hashes) });
        }

        private void OnMemPoolMessageReceived()
        {
            foreach (InvPayload payload in InvPayload.CreateGroup(InventoryType.TX, system.MemPool.GetVerifiedTransactions().Select(p => p.Hash).ToArray()))
                EnqueueMessage(Message.Create(MessageCommand.Inv, payload));
        }

        private void OnPingMessageReceived(PingPayload payload)
        {
            UpdateLastBlockIndex(payload.LastBlockIndex);
            EnqueueMessage(Message.Create(MessageCommand.Pong, PingPayload.Create(NativeContract.Ledger.CurrentIndex(system.StoreView), payload.Nonce)));
        }

        private void OnPongMessageReceived(PingPayload payload)
        {
            UpdateLastBlockIndex(payload.LastBlockIndex);
        }

        private void OnVerackMessageReceived()
        {
            verack = true;
            system.TaskManager.Tell(new TaskManager.Register { Version = Version });
            CheckMessageQueue();
        }

        private void OnVersionMessageReceived(VersionPayload payload)
        {
            Version = payload;
            foreach (NodeCapability capability in payload.Capabilities)
            {
                switch (capability)
                {
                    case FullNodeCapability fullNodeCapability:
                        IsFullNode = true;
                        LastBlockIndex = fullNodeCapability.StartHeight;
                        break;
                    case ServerCapability serverCapability:
                        if (serverCapability.Type == NodeCapabilityType.TcpServer)
                            ListenerTcpPort = serverCapability.Port;
                        break;
                }
            }
            if (!localNode.AllowNewConnection(Self, this))
            {
                Disconnect(true);
                return;
            }
            SendMessage(Message.Create(MessageCommand.Verack));
        }

        private void OnTimer()
        {
            DateTime oneMinuteAgo = TimeProvider.Current.UtcNow.AddMinutes(-1);
            while (pendingKnownHashes.Count > 0)
            {
                var (_, time) = pendingKnownHashes.First;
                if (oneMinuteAgo <= time) break;
                pendingKnownHashes.RemoveFirst();
            }
            if (oneMinuteAgo > lastSent)
                EnqueueMessage(Message.Create(MessageCommand.Ping, PingPayload.Create(NativeContract.Ledger.CurrentIndex(system.StoreView))));
        }

        private void UpdateLastBlockIndex(uint lastBlockIndex)
        {
            if (lastBlockIndex > LastBlockIndex)
            {
                LastBlockIndex = lastBlockIndex;
                system.TaskManager.Tell(new TaskManager.Update { LastBlockIndex = LastBlockIndex });
            }
        }
    }
}

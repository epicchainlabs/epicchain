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
// TaskManager.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Akka.Actor;
using Akka.Configuration;
using Akka.IO;
using Neo.IO.Actors;
using Neo.IO.Caching;
using Neo.Ledger;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract.Native;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;

namespace Neo.Network.P2P
{
    /// <summary>
    /// Actor used to manage the tasks of inventories.
    /// </summary>
    public class TaskManager : UntypedActor
    {
        internal class Register { public VersionPayload Version; }
        internal class Update { public uint LastBlockIndex; }
        internal class NewTasks { public InvPayload Payload; }

        /// <summary>
        /// Sent to <see cref="TaskManager"/> to restart tasks for inventories.
        /// </summary>
        public class RestartTasks
        {
            /// <summary>
            /// The inventories that need to restart.
            /// </summary>
            public InvPayload Payload { get; init; }
        }

        private class Timer { }

        private static readonly TimeSpan TimerInterval = TimeSpan.FromSeconds(30);
        private static readonly TimeSpan TaskTimeout = TimeSpan.FromMinutes(1);
        private static readonly UInt256 HeaderTaskHash = UInt256.Zero;

        private const int MaxConcurrentTasks = 3;

        private readonly NeoSystem system;
        /// <summary>
        /// A set of known hashes, of inventories or payloads, already received.
        /// </summary>
        private readonly HashSetCache<UInt256> knownHashes;
        private readonly Dictionary<UInt256, int> globalInvTasks = new();
        private readonly Dictionary<uint, int> globalIndexTasks = new();
        private readonly Dictionary<IActorRef, TaskSession> sessions = new();
        private readonly ICancelable timer = Context.System.Scheduler.ScheduleTellRepeatedlyCancelable(TimerInterval, TimerInterval, Context.Self, new Timer(), ActorRefs.NoSender);
        private uint lastSeenPersistedIndex = 0;

        private bool HasHeaderTask => globalInvTasks.ContainsKey(HeaderTaskHash);

        /// <summary>
        /// Initializes a new instance of the <see cref="TaskManager"/> class.
        /// </summary>
        /// <param name="system">The <see cref="NeoSystem"/> object that contains the <see cref="TaskManager"/>.</param>
        public TaskManager(NeoSystem system)
        {
            this.system = system;
            this.knownHashes = new HashSetCache<UInt256>(system.MemPool.Capacity * 2 / 5);
            Context.System.EventStream.Subscribe(Self, typeof(Blockchain.PersistCompleted));
            Context.System.EventStream.Subscribe(Self, typeof(Blockchain.RelayResult));
        }

        private void OnHeaders(Header[] _)
        {
            if (!sessions.TryGetValue(Sender, out TaskSession session))
                return;
            if (session.InvTasks.Remove(HeaderTaskHash))
                DecrementGlobalTask(HeaderTaskHash);
            RequestTasks(Sender, session);
        }

        private void OnInvalidBlock(Block invalidBlock)
        {
            foreach (var (actor, session) in sessions)
                if (session.ReceivedBlock.TryGetValue(invalidBlock.Index, out Block block))
                    if (block.Hash == invalidBlock.Hash)
                        actor.Tell(Tcp.Abort.Instance);
        }

        private void OnNewTasks(InvPayload payload)
        {
            if (!sessions.TryGetValue(Sender, out TaskSession session))
                return;

            // Do not accept payload of type InventoryType.TX if not synced on HeaderHeight
            uint currentHeight = Math.Max(NativeContract.Ledger.CurrentIndex(system.StoreView), lastSeenPersistedIndex);
            uint headerHeight = system.HeaderCache.Last?.Index ?? currentHeight;
            if (currentHeight < headerHeight && (payload.Type == InventoryType.TX || (payload.Type == InventoryType.Block && currentHeight < session.LastBlockIndex - InvPayload.MaxHashesCount)))
            {
                RequestTasks(Sender, session);
                return;
            }

            HashSet<UInt256> hashes = new(payload.Hashes);
            // Remove all previously processed knownHashes from the list that is being requested
            hashes.Remove(knownHashes);
            // Add to AvailableTasks the ones, of type InventoryType.Block, that are global (already under process by other sessions)
            if (payload.Type == InventoryType.Block)
                session.AvailableTasks.UnionWith(hashes.Where(p => globalInvTasks.ContainsKey(p)));

            // Remove those that are already in process by other sessions
            hashes.Remove(globalInvTasks);
            if (hashes.Count == 0)
            {
                RequestTasks(Sender, session);
                return;
            }

            // Update globalTasks with the ones that will be requested within this current session
            foreach (UInt256 hash in hashes)
            {
                IncrementGlobalTask(hash);
                session.InvTasks[hash] = TimeProvider.Current.UtcNow;
            }

            foreach (InvPayload group in InvPayload.CreateGroup(payload.Type, hashes.ToArray()))
                Sender.Tell(Message.Create(MessageCommand.GetData, group));
        }

        private void OnPersistCompleted(Block block)
        {
            lastSeenPersistedIndex = block.Index;

            foreach (var (actor, session) in sessions)
                if (session.ReceivedBlock.Remove(block.Index, out Block receivedBlock))
                {
                    if (block.Hash == receivedBlock.Hash)
                        RequestTasks(actor, session);
                    else
                        actor.Tell(Tcp.Abort.Instance);
                }
        }

        protected override void OnReceive(object message)
        {
            switch (message)
            {
                case Register register:
                    OnRegister(register.Version);
                    break;
                case Update update:
                    OnUpdate(update);
                    break;
                case NewTasks tasks:
                    OnNewTasks(tasks.Payload);
                    break;
                case RestartTasks restart:
                    OnRestartTasks(restart.Payload);
                    break;
                case Header[] headers:
                    OnHeaders(headers);
                    break;
                case IInventory inventory:
                    OnTaskCompleted(inventory);
                    break;
                case Blockchain.PersistCompleted pc:
                    OnPersistCompleted(pc.Block);
                    break;
                case Blockchain.RelayResult rr:
                    if (rr.Inventory is Block invalidBlock && rr.Result == VerifyResult.Invalid)
                        OnInvalidBlock(invalidBlock);
                    break;
                case Timer _:
                    OnTimer();
                    break;
                case Terminated terminated:
                    OnTerminated(terminated.ActorRef);
                    break;
            }
        }

        private void OnRegister(VersionPayload version)
        {
            Context.Watch(Sender);
            TaskSession session = new(version);
            sessions.Add(Sender, session);
            RequestTasks(Sender, session);
        }

        private void OnUpdate(Update update)
        {
            if (!sessions.TryGetValue(Sender, out TaskSession session))
                return;
            session.LastBlockIndex = update.LastBlockIndex;
        }

        private void OnRestartTasks(InvPayload payload)
        {
            knownHashes.ExceptWith(payload.Hashes);
            foreach (UInt256 hash in payload.Hashes)
                globalInvTasks.Remove(hash);
            foreach (InvPayload group in InvPayload.CreateGroup(payload.Type, payload.Hashes))
                system.LocalNode.Tell(Message.Create(MessageCommand.GetData, group));
        }

        private void OnTaskCompleted(IInventory inventory)
        {
            Block block = inventory as Block;
            knownHashes.Add(inventory.Hash);
            globalInvTasks.Remove(inventory.Hash);
            if (block is not null)
                globalIndexTasks.Remove(block.Index);
            foreach (TaskSession ms in sessions.Values)
                ms.AvailableTasks.Remove(inventory.Hash);
            if (sessions.TryGetValue(Sender, out TaskSession session))
            {
                session.InvTasks.Remove(inventory.Hash);
                if (block is not null)
                {
                    session.IndexTasks.Remove(block.Index);
                    if (session.ReceivedBlock.TryGetValue(block.Index, out var block_old))
                    {
                        if (block.Hash != block_old.Hash)
                        {
                            Sender.Tell(Tcp.Abort.Instance);
                            return;
                        }
                    }
                    else
                    {
                        session.ReceivedBlock.Add(block.Index, block);
                    }
                }
                else
                {
                    RequestTasks(Sender, session);
                }
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private void DecrementGlobalTask(UInt256 hash)
        {
            if (globalInvTasks.TryGetValue(hash, out var value))
            {
                if (value == 1)
                    globalInvTasks.Remove(hash);
                else
                    globalInvTasks[hash] = value - 1;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private void DecrementGlobalTask(uint index)
        {
            if (globalIndexTasks.TryGetValue(index, out var value))
            {
                if (value == 1)
                    globalIndexTasks.Remove(index);
                else
                    globalIndexTasks[index] = value - 1;
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private bool IncrementGlobalTask(UInt256 hash)
        {
            if (!globalInvTasks.TryGetValue(hash, out var value))
            {
                globalInvTasks[hash] = 1;
                return true;
            }
            if (value >= MaxConcurrentTasks)
                return false;

            globalInvTasks[hash] = value + 1;
            return true;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private bool IncrementGlobalTask(uint index)
        {
            if (!globalIndexTasks.TryGetValue(index, out var value))
            {
                globalIndexTasks[index] = 1;
                return true;
            }
            if (value >= MaxConcurrentTasks)
                return false;

            globalIndexTasks[index] = value + 1;
            return true;
        }

        private void OnTerminated(IActorRef actor)
        {
            if (!sessions.TryGetValue(actor, out TaskSession session))
                return;
            foreach (UInt256 hash in session.InvTasks.Keys)
                DecrementGlobalTask(hash);
            foreach (uint index in session.IndexTasks.Keys)
                DecrementGlobalTask(index);
            sessions.Remove(actor);
        }

        private void OnTimer()
        {
            foreach (TaskSession session in sessions.Values)
            {
                foreach (var (hash, time) in session.InvTasks.ToArray())
                    if (TimeProvider.Current.UtcNow - time > TaskTimeout)
                    {
                        if (session.InvTasks.Remove(hash))
                            DecrementGlobalTask(hash);
                    }
                foreach (var (index, time) in session.IndexTasks.ToArray())
                    if (TimeProvider.Current.UtcNow - time > TaskTimeout)
                    {
                        if (session.IndexTasks.Remove(index))
                            DecrementGlobalTask(index);
                    }
            }
            foreach (var (actor, session) in sessions)
                RequestTasks(actor, session);
        }

        protected override void PostStop()
        {
            timer.CancelIfNotNull();
            base.PostStop();
        }

        /// <summary>
        /// Gets a <see cref="Akka.Actor.Props"/> object used for creating the <see cref="TaskManager"/> actor.
        /// </summary>
        /// <param name="system">The <see cref="NeoSystem"/> object that contains the <see cref="TaskManager"/>.</param>
        /// <returns>The <see cref="Akka.Actor.Props"/> object used for creating the <see cref="TaskManager"/> actor.</returns>
        public static Props Props(NeoSystem system)
        {
            return Akka.Actor.Props.Create(() => new TaskManager(system)).WithMailbox("task-manager-mailbox");
        }

        private void RequestTasks(IActorRef remoteNode, TaskSession session)
        {
            if (session.HasTooManyTasks) return;

            DataCache snapshot = system.StoreView;

            // If there are pending tasks of InventoryType.Block we should process them
            if (session.AvailableTasks.Count > 0)
            {
                session.AvailableTasks.Remove(knownHashes);
                // Search any similar hash that is on Singleton's knowledge, which means, on the way or already processed
                session.AvailableTasks.RemoveWhere(p => NativeContract.Ledger.ContainsBlock(snapshot, p));
                HashSet<UInt256> hashes = new(session.AvailableTasks);
                if (hashes.Count > 0)
                {
                    foreach (UInt256 hash in hashes.ToArray())
                    {
                        if (!IncrementGlobalTask(hash))
                            hashes.Remove(hash);
                    }
                    session.AvailableTasks.Remove(hashes);
                    foreach (UInt256 hash in hashes)
                        session.InvTasks[hash] = DateTime.UtcNow;
                    foreach (InvPayload group in InvPayload.CreateGroup(InventoryType.Block, hashes.ToArray()))
                        remoteNode.Tell(Message.Create(MessageCommand.GetData, group));
                    return;
                }
            }

            uint currentHeight = Math.Max(NativeContract.Ledger.CurrentIndex(snapshot), lastSeenPersistedIndex);
            uint headerHeight = system.HeaderCache.Last?.Index ?? currentHeight;
            // When the number of AvailableTasks is no more than 0,
            // no pending tasks of InventoryType.Block, it should process pending the tasks of headers
            // If not HeaderTask pending to be processed it should ask for more Blocks
            if ((!HasHeaderTask || globalInvTasks[HeaderTaskHash] < MaxConcurrentTasks) && headerHeight < session.LastBlockIndex && !system.HeaderCache.Full)
            {
                session.InvTasks[HeaderTaskHash] = DateTime.UtcNow;
                IncrementGlobalTask(HeaderTaskHash);
                remoteNode.Tell(Message.Create(MessageCommand.GetHeaders, GetBlockByIndexPayload.Create(headerHeight + 1)));
            }
            else if (currentHeight < session.LastBlockIndex)
            {
                uint startHeight = currentHeight + 1;
                while (globalIndexTasks.ContainsKey(startHeight) || session.ReceivedBlock.ContainsKey(startHeight)) { startHeight++; }
                if (startHeight > session.LastBlockIndex || startHeight >= currentHeight + InvPayload.MaxHashesCount) return;
                uint endHeight = startHeight;
                while (!globalIndexTasks.ContainsKey(++endHeight) && endHeight <= session.LastBlockIndex && endHeight <= currentHeight + InvPayload.MaxHashesCount) { }
                uint count = Math.Min(endHeight - startHeight, InvPayload.MaxHashesCount);
                for (uint i = 0; i < count; i++)
                {
                    session.IndexTasks[startHeight + i] = TimeProvider.Current.UtcNow;
                    IncrementGlobalTask(startHeight + i);
                }
                remoteNode.Tell(Message.Create(MessageCommand.GetBlockByIndex, GetBlockByIndexPayload.Create(startHeight, (short)count)));
            }
            else if (!session.MempoolSent)
            {
                session.MempoolSent = true;
                remoteNode.Tell(Message.Create(MessageCommand.Mempool));
            }
        }
    }

    internal class TaskManagerMailbox : PriorityMailbox
    {
        public TaskManagerMailbox(Akka.Actor.Settings settings, Config config)
            : base(settings, config)
        {
        }

        internal protected override bool IsHighPriority(object message)
        {
            switch (message)
            {
                case TaskManager.Register _:
                case TaskManager.Update _:
                case TaskManager.RestartTasks _:
                    return true;
                case TaskManager.NewTasks tasks:
                    if (tasks.Payload.Type == InventoryType.Block || tasks.Payload.Type == InventoryType.Extensible)
                        return true;
                    return false;
                default:
                    return false;
            }
        }

        internal protected override bool ShallDrop(object message, IEnumerable queue)
        {
            if (message is not TaskManager.NewTasks tasks) return false;
            // Remove duplicate tasks
            if (queue.OfType<TaskManager.NewTasks>().Any(x => x.Payload.Type == tasks.Payload.Type && x.Payload.Hashes.SequenceEqual(tasks.Payload.Hashes))) return true;
            return false;
        }
    }
}

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
// NeoSystem.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Akka.Actor;
using Neo.IO.Caching;
using Neo.Ledger;
using Neo.Network.P2P;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.Plugins;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Threading;

namespace Neo
{
    /// <summary>
    /// Represents the basic unit that contains all the components required for running of a NEO node.
    /// </summary>
    public class NeoSystem : IDisposable
    {
        /// <summary>
        /// Triggered when a service is added to the <see cref="NeoSystem"/>.
        /// </summary>
        public event EventHandler<object> ServiceAdded;

        /// <summary>
        /// The protocol settings of the <see cref="NeoSystem"/>.
        /// </summary>
        public ProtocolSettings Settings { get; }

        /// <summary>
        /// The <see cref="Akka.Actor.ActorSystem"/> used to create actors for the <see cref="NeoSystem"/>.
        /// </summary>
        public ActorSystem ActorSystem { get; } = ActorSystem.Create(nameof(NeoSystem),
            $"akka {{ log-dead-letters = off , loglevel = warning, loggers = [ \"{typeof(Utility.Logger).AssemblyQualifiedName}\" ] }}" +
            $"blockchain-mailbox {{ mailbox-type: \"{typeof(BlockchainMailbox).AssemblyQualifiedName}\" }}" +
            $"task-manager-mailbox {{ mailbox-type: \"{typeof(TaskManagerMailbox).AssemblyQualifiedName}\" }}" +
            $"remote-node-mailbox {{ mailbox-type: \"{typeof(RemoteNodeMailbox).AssemblyQualifiedName}\" }}");

        /// <summary>
        /// The genesis block of the NEO blockchain.
        /// </summary>
        public Block GenesisBlock { get; }

        /// <summary>
        /// The <see cref="Ledger.Blockchain"/> actor of the <see cref="NeoSystem"/>.
        /// </summary>
        public IActorRef Blockchain { get; }

        /// <summary>
        /// The <see cref="Network.P2P.LocalNode"/> actor of the <see cref="NeoSystem"/>.
        /// </summary>
        public IActorRef LocalNode { get; }

        /// <summary>
        /// The <see cref="Network.P2P.TaskManager"/> actor of the <see cref="NeoSystem"/>.
        /// </summary>
        public IActorRef TaskManager { get; }

        /// <summary>
        /// The transaction router actor of the <see cref="NeoSystem"/>.
        /// </summary>
        public IActorRef TxRouter;

        /// <summary>
        /// A readonly view of the store.
        /// </summary>
        /// <remarks>
        /// It doesn't need to be disposed because the <see cref="ISnapshot"/> inside it is null.
        /// </remarks>
        public DataCache StoreView => new SnapshotCache(store);

        /// <summary>
        /// The memory pool of the <see cref="NeoSystem"/>.
        /// </summary>
        public MemoryPool MemPool { get; }

        /// <summary>
        /// The header cache of the <see cref="NeoSystem"/>.
        /// </summary>
        public HeaderCache HeaderCache { get; } = new();

        internal RelayCache RelayCache { get; } = new(100);

        private ImmutableList<object> services = ImmutableList<object>.Empty;
        private readonly IStore store;
        private readonly IStoreProvider storageProvider;
        private ChannelsConfig start_message = null;
        private int suspend = 0;

        static NeoSystem()
        {
            // Unify unhandled exceptions
            AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;

            Plugin.LoadPlugins();
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="NeoSystem"/> class.
        /// </summary>
        /// <param name="settings">The protocol settings of the <see cref="NeoSystem"/>.</param>
        /// <param name="storageProvider">The storage engine used to create the <see cref="IStoreProvider"/> objects. If this parameter is <see langword="null"/>, a default in-memory storage engine will be used.</param>
        /// <param name="storagePath">The path of the storage. If <paramref name="storageProvider"/> is the default in-memory storage engine, this parameter is ignored.</param>
        public NeoSystem(ProtocolSettings settings, string? storageProvider = null, string? storagePath = null) :
            this(settings, StoreFactory.GetStoreProvider(storageProvider ?? nameof(MemoryStore))
                ?? throw new ArgumentException($"Can't find the storage provider {storageProvider}", nameof(storageProvider)), storagePath)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="NeoSystem"/> class.
        /// </summary>
        /// <param name="settings">The protocol settings of the <see cref="NeoSystem"/>.</param>
        /// <param name="storageProvider">The <see cref="IStoreProvider"/> to use.</param>
        /// <param name="storagePath">The path of the storage. If <paramref name="storageProvider"/> is the default in-memory storage engine, this parameter is ignored.</param>
        public NeoSystem(ProtocolSettings settings, IStoreProvider storageProvider, string? storagePath = null)
        {
            this.Settings = settings;
            this.GenesisBlock = CreateGenesisBlock(settings);
            this.storageProvider = storageProvider;
            this.store = storageProvider.GetStore(storagePath);
            this.MemPool = new MemoryPool(this);
            this.Blockchain = ActorSystem.ActorOf(Ledger.Blockchain.Props(this));
            this.LocalNode = ActorSystem.ActorOf(Network.P2P.LocalNode.Props(this));
            this.TaskManager = ActorSystem.ActorOf(Network.P2P.TaskManager.Props(this));
            this.TxRouter = ActorSystem.ActorOf(TransactionRouter.Props(this));
            foreach (var plugin in Plugin.Plugins)
                plugin.OnSystemLoaded(this);
            Blockchain.Ask(new Blockchain.Initialize()).Wait();
        }

        /// <summary>
        /// Creates the genesis block for the NEO blockchain.
        /// </summary>
        /// <param name="settings">The <see cref="ProtocolSettings"/> of the NEO system.</param>
        /// <returns>The genesis block.</returns>
        public static Block CreateGenesisBlock(ProtocolSettings settings) => new()
        {
            Header = new Header
            {
                PrevHash = UInt256.Zero,
                MerkleRoot = UInt256.Zero,
                Timestamp = (new DateTime(2016, 7, 15, 15, 8, 21, DateTimeKind.Utc)).ToTimestampMS(),
                Nonce = 2083236893, // nonce from the Bitcoin genesis block.
                Index = 0,
                PrimaryIndex = 0,
                NextConsensus = Contract.GetBFTAddress(settings.StandbyValidators),
                Witness = new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = new[] { (byte)OpCode.PUSH1 }
                },
            },
            Transactions = Array.Empty<Transaction>()
        };

        private static void CurrentDomain_UnhandledException(object sender, UnhandledExceptionEventArgs e)
        {
            Utility.Log("UnhandledException", LogLevel.Fatal, e.ExceptionObject);
        }

        public void Dispose()
        {
            EnsureStopped(LocalNode);
            EnsureStopped(Blockchain);
            foreach (var p in Plugin.Plugins)
                p.Dispose();
            // Dispose will call ActorSystem.Terminate()
            ActorSystem.Dispose();
            ActorSystem.WhenTerminated.Wait();
            HeaderCache.Dispose();
            store.Dispose();
        }

        /// <summary>
        /// Adds a service to the <see cref="NeoSystem"/>.
        /// </summary>
        /// <param name="service">The service object to be added.</param>
        public void AddService(object service)
        {
            ImmutableInterlocked.Update(ref services, p => p.Add(service));
            ServiceAdded?.Invoke(this, service);
        }

        /// <summary>
        /// Gets a specified type of service object from the <see cref="NeoSystem"/>.
        /// </summary>
        /// <typeparam name="T">The type of the service object.</typeparam>
        /// <param name="filter">An action used to filter the service objects. This parameter can be <see langword="null"/>.</param>
        /// <returns>The service object found.</returns>
        public T GetService<T>(Func<T, bool> filter = null)
        {
            IEnumerable<T> result = services.OfType<T>();
            if (filter is null)
                return result.FirstOrDefault();
            return result.FirstOrDefault(filter);
        }

        /// <summary>
        /// Blocks the current thread until the specified actor has stopped.
        /// </summary>
        /// <param name="actor">The actor to wait.</param>
        public void EnsureStopped(IActorRef actor)
        {
            using Inbox inbox = Inbox.Create(ActorSystem);
            inbox.Watch(actor);
            ActorSystem.Stop(actor);
            inbox.Receive(TimeSpan.FromMinutes(5));
        }

        /// <summary>
        /// Loads an <see cref="IStore"/> at the specified path.
        /// </summary>
        /// <param name="path">The path of the storage.</param>
        /// <returns>The loaded <see cref="IStore"/>.</returns>
        public IStore LoadStore(string path)
        {
            return storageProvider.GetStore(path);
        }

        /// <summary>
        /// Resumes the startup process of <see cref="LocalNode"/>.
        /// </summary>
        /// <returns><see langword="true"/> if the startup process is resumed; otherwise, <see langword="false"/>.</returns>
        public bool ResumeNodeStartup()
        {
            if (Interlocked.Decrement(ref suspend) != 0)
                return false;
            if (start_message != null)
            {
                LocalNode.Tell(start_message);
                start_message = null;
            }
            return true;
        }

        /// <summary>
        /// Starts the <see cref="LocalNode"/> with the specified configuration.
        /// </summary>
        /// <param name="config">The configuration used to start the <see cref="LocalNode"/>.</param>
        public void StartNode(ChannelsConfig config)
        {
            start_message = config;

            if (suspend == 0)
            {
                LocalNode.Tell(start_message);
                start_message = null;
            }
        }

        /// <summary>
        /// Suspends the startup process of <see cref="LocalNode"/>.
        /// </summary>
        public void SuspendNodeStartup()
        {
            Interlocked.Increment(ref suspend);
        }

        /// <summary>
        /// Gets a snapshot of the blockchain storage.
        /// </summary>
        /// <returns></returns>
        public SnapshotCache GetSnapshot()
        {
            return new SnapshotCache(store.GetSnapshot());
        }

        /// <summary>
        /// Determines whether the specified transaction exists in the memory pool or storage.
        /// </summary>
        /// <param name="hash">The hash of the transaction</param>
        /// <returns><see langword="true"/> if the transaction exists; otherwise, <see langword="false"/>.</returns>
        public ContainsTransactionType ContainsTransaction(UInt256 hash)
        {
            if (MemPool.ContainsKey(hash)) return ContainsTransactionType.ExistsInPool;
            return NativeContract.Ledger.ContainsTransaction(StoreView, hash) ?
                ContainsTransactionType.ExistsInLedger : ContainsTransactionType.NotExist;
        }

        /// <summary>
        /// Determines whether the specified transaction conflicts with some on-chain transaction.
        /// </summary>
        /// <param name="hash">The hash of the transaction</param>
        /// <param name="signers">The list of signer accounts of the transaction</param>
        /// <returns><see langword="true"/> if the transaction conflicts with on-chain transaction; otherwise, <see langword="false"/>.</returns>
        public bool ContainsConflictHash(UInt256 hash, IEnumerable<UInt160> signers)
        {
            return NativeContract.Ledger.ContainsConflictHash(StoreView, hash, signers, Settings.MaxTraceableBlocks);
        }
    }
}

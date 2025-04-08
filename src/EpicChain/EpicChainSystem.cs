// Copyright (C) 2021-2024 EpicChain Labs.

//
// EpicChainSystem.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.Actor;
using EpicChain.Extensions;
using EpicChain.IO.Caching;
using EpicChain.Ledger;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.Plugins;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Threading;

namespace EpicChain
{
    /// <summary>
    /// Represents the basic unit that contains all the components required for running of a EpicChain node.
    /// </summary>
    public class EpicChainSystem : IDisposable
    {
        /// <summary>
        /// Triggered when a service is added to the <see cref="EpicChainSystem"/>.
        /// </summary>
        public event EventHandler<object> ServiceAdded;

        /// <summary>
        /// The protocol settings of the <see cref="EpicChainSystem"/>.
        /// </summary>
        public ProtocolSettings Settings { get; }

        /// <summary>
        /// The <see cref="Akka.Actor.ActorSystem"/> used to create actors for the <see cref="EpicChainSystem"/>.
        /// </summary>
        public ActorSystem ActorSystem { get; } = ActorSystem.Create(nameof(EpicChainSystem),
            $"akka {{ log-dead-letters = off , loglevel = warning, loggers = [ \"{typeof(Utility.Logger).AssemblyQualifiedName}\" ] }}" +
            $"blockchain-mailbox {{ mailbox-type: \"{typeof(BlockchainMailbox).AssemblyQualifiedName}\" }}" +
            $"task-manager-mailbox {{ mailbox-type: \"{typeof(TaskManagerMailbox).AssemblyQualifiedName}\" }}" +
            $"remote-node-mailbox {{ mailbox-type: \"{typeof(RemoteNodeMailbox).AssemblyQualifiedName}\" }}");

        /// <summary>
        /// The genesis block of the EpicChain blockchain.
        /// </summary>
        public Block GenesisBlock { get; }

        /// <summary>
        /// The <see cref="Ledger.Blockchain"/> actor of the <see cref="EpicChainSystem"/>.
        /// </summary>
        public IActorRef Blockchain { get; }

        /// <summary>
        /// The <see cref="Network.P2P.LocalNode"/> actor of the <see cref="EpicChainSystem"/>.
        /// </summary>
        public IActorRef LocalNode { get; }

        /// <summary>
        /// The <see cref="Network.P2P.TaskManager"/> actor of the <see cref="EpicChainSystem"/>.
        /// </summary>
        public IActorRef TaskManager { get; }

        /// <summary>
        /// The transaction router actor of the <see cref="EpicChainSystem"/>.
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
        /// The memory pool of the <see cref="EpicChainSystem"/>.
        /// </summary>
        public MemoryPool MemPool { get; }

        /// <summary>
        /// The header cache of the <see cref="EpicChainSystem"/>.
        /// </summary>
        public HeaderCache HeaderCache { get; } = new();

        internal RelayCache RelayCache { get; } = new(100);

        private ImmutableList<object> services = ImmutableList<object>.Empty;
        private readonly IStore store;
        private readonly IStoreProvider storageProvider;
        private ChannelsConfig? start_message = null;
        private int suspend = 0;

        static EpicChainSystem()
        {
            // Unify unhandled exceptions
            AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;

            Plugin.LoadPlugins();
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="EpicChainSystem"/> class.
        /// </summary>
        /// <param name="settings">The protocol settings of the <see cref="EpicChainSystem"/>.</param>
        /// <param name="storageProvider">The storage engine used to create the <see cref="IStoreProvider"/> objects. If this parameter is <see langword="null"/>, a default in-memory storage engine will be used.</param>
        /// <param name="storagePath">The path of the storage. If <paramref name="storageProvider"/> is the default in-memory storage engine, this parameter is ignored.</param>
        public EpicChainSystem(ProtocolSettings settings, string storageProvider = null, string storagePath = null) :
            this(settings, StoreFactory.GetStoreProvider(storageProvider ?? nameof(MemoryStore))
                ?? throw new ArgumentException($"Can't find the storage provider {storageProvider}", nameof(storageProvider)), storagePath)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="EpicChainSystem"/> class.
        /// </summary>
        /// <param name="settings">The protocol settings of the <see cref="EpicChainSystem"/>.</param>
        /// <param name="storageProvider">The <see cref="IStoreProvider"/> to use.</param>
        /// <param name="storagePath">The path of the storage. If <paramref name="storageProvider"/> is the default in-memory storage engine, this parameter is ignored.</param>
        public EpicChainSystem(ProtocolSettings settings, IStoreProvider storageProvider, string storagePath = null)
        {
            Settings = settings;
            GenesisBlock = CreateGenesisBlock(settings);
            this.storageProvider = storageProvider;
            store = storageProvider.GetStore(storagePath);
            MemPool = new MemoryPool(this);
            Blockchain = ActorSystem.ActorOf(Ledger.Blockchain.Props(this));
            LocalNode = ActorSystem.ActorOf(Network.P2P.LocalNode.Props(this));
            TaskManager = ActorSystem.ActorOf(Network.P2P.TaskManager.Props(this));
            TxRouter = ActorSystem.ActorOf(TransactionRouter.Props(this));
            foreach (var plugin in Plugin.Plugins)
                plugin.OnSystemLoaded(this);
            Blockchain.Ask(new Blockchain.Initialize()).Wait();
        }

        /// <summary>
        /// Creates the genesis block for the EpicChain blockchain.
        /// </summary>
        /// <param name="settings">The <see cref="ProtocolSettings"/> of the EpicChain system.</param>
        /// <returns>The genesis block.</returns>
        public static Block CreateGenesisBlock(ProtocolSettings settings) => new()
        {
            Header = new Header
            {
                PrevHash = UInt256.Zero,
                MerkleRoot = UInt256.Zero,
                Timestamp = (new DateTime(2021, 10, 20, 07, 11, 48, DateTimeKind.Utc)).ToTimestampMS(),
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
        /// Adds a service to the <see cref="EpicChainSystem"/>.
        /// </summary>
        /// <param name="service">The service object to be added.</param>
        public void AddService(object service)
        {
            ImmutableInterlocked.Update(ref services, p => p.Add(service));
            ServiceAdded?.Invoke(this, service);
        }

        /// <summary>
        /// Gets a specified type of service object from the <see cref="EpicChainSystem"/>.
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
        /// <returns>An instance of <see cref="SnapshotCache"/></returns>
        [Obsolete("This method is obsolete, use GetSnapshotCache instead.")]
        public SnapshotCache GetSnapshot()
        {
            return new SnapshotCache(store.GetSnapshot());
        }

        /// <summary>
        /// Gets a snapshot of the blockchain storage with an execution cache.
        /// With the snapshot, we have the latest state of the blockchain, with the cache,
        /// we can run transactions in a sandboxed environment.
        /// </summary>
        /// <returns>An instance of <see cref="SnapshotCache"/></returns>
        public SnapshotCache GetSnapshotCache()
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

// Copyright (C) 2021-2024 EpicChain Labs.

//
// Blockchain.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Akka.Configuration;
using Akka.IO;
using EpicChain.IO.Actors;
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
using System.Diagnostics;
using System.Linq;
using System.Runtime.CompilerServices;

namespace EpicChain.Ledger
{
    public delegate void CommittingHandler(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList);
    public delegate void CommittedHandler(EpicChainSystem system, Block block);

    /// <summary>
    /// Actor used to verify and relay <see cref="IInventory"/>.
    /// </summary>
    public sealed partial class Blockchain : UntypedActor
    {
        /// <summary>
        /// Sent by the <see cref="Blockchain"/> when a smart contract is executed.
        /// </summary>
        public partial class ApplicationExecuted { }

        /// <summary>
        /// Sent by the <see cref="Blockchain"/> when a <see cref="Network.P2P.Payloads.Block"/> is persisted.
        /// </summary>
        public class PersistCompleted
        {
            /// <summary>
            /// The <see cref="Network.P2P.Payloads.Block"/> that is persisted.
            /// </summary>
            public Block Block { get; init; }
        }

        /// <summary>
        /// Sent to the <see cref="Blockchain"/> when importing blocks.
        /// </summary>
        public class Import
        {
            /// <summary>
            /// The blocks to be imported.
            /// </summary>
            public IEnumerable<Block> Blocks { get; init; }

            /// <summary>
            /// Indicates whether the blocks need to be verified when importing.
            /// </summary>
            public bool Verify { get; init; } = true;
        }

        /// <summary>
        /// Sent by the <see cref="Blockchain"/> when the import is complete.
        /// </summary>
        public class ImportCompleted { }

        /// <summary>
        /// Sent to the <see cref="Blockchain"/> when the consensus is filling the memory pool.
        /// </summary>
        public class FillMemoryPool
        {
            /// <summary>
            /// The transactions to be sent.
            /// </summary>
            public IEnumerable<Transaction> Transactions { get; init; }
        }

        /// <summary>
        /// Sent by the <see cref="Blockchain"/> when the memory pool is filled.
        /// </summary>
        public class FillCompleted { }

        /// <summary>
        /// Sent to the <see cref="Blockchain"/> when inventories need to be re-verified.
        /// </summary>
        public class Reverify
        {
            /// <summary>
            /// The inventories to be re-verified.
            /// </summary>
            public IReadOnlyList<IInventory> Inventories { get; init; }
        }

        /// <summary>
        /// Sent by the <see cref="Blockchain"/> when an <see cref="IInventory"/> is relayed.
        /// </summary>
        public class RelayResult
        {
            /// <summary>
            /// The <see cref="IInventory"/> that is relayed.
            /// </summary>
            public IInventory Inventory { get; init; }
            /// <summary>
            /// The result.
            /// </summary>
            public VerifyResult Result { get; init; }
        }

        internal class Initialize { }
        private class UnverifiedBlocksList { public LinkedList<Block> Blocks = new(); public HashSet<IActorRef> Nodes = new(); }

        public static event CommittingHandler Committing;
        public static event CommittedHandler Committed;

        private readonly static Script onPersistScript, postPersistScript;
        private const int MaxTxToReverifyPerIdle = 10;
        private readonly EpicChainSystem system;
        private readonly Dictionary<UInt256, Block> block_cache = new();
        private readonly Dictionary<uint, UnverifiedBlocksList> block_cache_unverified = new();
        private ImmutableHashSet<UInt160> extensibleWitnessWhiteList;

        static Blockchain()
        {
            using (ScriptBuilder sb = new())
            {
                sb.EmitSysCall(ApplicationEngine.System_Contract_NativeOnPersist);
                onPersistScript = sb.ToArray();
            }
            using (ScriptBuilder sb = new())
            {
                sb.EmitSysCall(ApplicationEngine.System_Contract_NativePostPersist);
                postPersistScript = sb.ToArray();
            }
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="Blockchain"/> class.
        /// </summary>
        /// <param name="system">The <see cref="EpicChainSystem"/> object that contains the <see cref="Blockchain"/>.</param>
        public Blockchain(EpicChainSystem system)
        {
            this.system = system;
        }

        private void OnImport(IEnumerable<Block> blocks, bool verify)
        {
            uint currentHeight = NativeContract.Ledger.CurrentIndex(system.StoreView);
            foreach (Block block in blocks)
            {
                if (block.Index <= currentHeight) continue;
                if (block.Index != currentHeight + 1)
                    throw new InvalidOperationException();
                if (verify && !block.Verify(system.Settings, system.StoreView))
                    throw new InvalidOperationException();
                Persist(block);
                ++currentHeight;
            }
            Sender.Tell(new ImportCompleted());
        }

        private void AddUnverifiedBlockToCache(Block block)
        {
            // Check if any block proposal for height `block.Index` exists
            if (!block_cache_unverified.TryGetValue(block.Index, out var list))
            {
                // There are no blocks, a new UnverifiedBlocksList is created and, consequently, the current block is added to the list
                list = new UnverifiedBlocksList();
                block_cache_unverified.Add(block.Index, list);
            }
            else
            {
                // Check if any block with the hash being added already exists on possible candidates to be processed
                foreach (var unverifiedBlock in list.Blocks)
                {
                    if (block.Hash == unverifiedBlock.Hash)
                        return;
                }

                if (!list.Nodes.Add(Sender))
                {
                    // Same index with different hash
                    Sender.Tell(Tcp.Abort.Instance);
                    return;
                }
            }

            list.Blocks.AddLast(block);
        }

        private void OnFillMemoryPool(IEnumerable<Transaction> transactions)
        {
            // Invalidate all the transactions in the memory pool, to avoid any failures when adding new transactions.
            system.MemPool.InvalidateAllTransactions();

            DataCache snapshot = system.StoreView;

            // Add the transactions to the memory pool
            foreach (var tx in transactions)
            {
                if (NativeContract.Ledger.ContainsTransaction(snapshot, tx.Hash))
                    continue;
                if (NativeContract.Ledger.ContainsConflictHash(snapshot, tx.Hash, tx.Signers.Select(s => s.Account), system.Settings.MaxTraceableBlocks))
                    continue;
                // First remove the tx if it is unverified in the pool.
                system.MemPool.TryRemoveUnVerified(tx.Hash, out _);
                // Add to the memory pool
                system.MemPool.TryAdd(tx, snapshot);
            }
            // Transactions originally in the pool will automatically be reverified based on their priority.

            Sender.Tell(new FillCompleted());
        }

        private void OnInitialize()
        {
            if (!NativeContract.Ledger.Initialized(system.StoreView))
                Persist(system.GenesisBlock);
            Sender.Tell(new object());
        }

        private void OnInventory(IInventory inventory, bool relay = true)
        {
            VerifyResult result = inventory switch
            {
                Block block => OnNewBlock(block),
                Transaction transaction => OnNewTransaction(transaction),
                ExtensiblePayload payload => OnNewExtensiblePayload(payload),
                _ => throw new NotSupportedException()
            };
            if (result == VerifyResult.Succeed && relay)
            {
                system.LocalNode.Tell(new LocalNode.RelayDirectly { Inventory = inventory });
            }
            SendRelayResult(inventory, result);
        }

        private VerifyResult OnNewBlock(Block block)
        {
            DataCache snapshot = system.StoreView;
            uint currentHeight = NativeContract.Ledger.CurrentIndex(snapshot);
            uint headerHeight = system.HeaderCache.Last?.Index ?? currentHeight;
            if (block.Index <= currentHeight)
                return VerifyResult.AlreadyExists;
            if (block.Index - 1 > headerHeight)
            {
                AddUnverifiedBlockToCache(block);
                return VerifyResult.UnableToVerify;
            }
            if (block.Index == headerHeight + 1)
            {
                if (!block.Verify(system.Settings, snapshot, system.HeaderCache))
                    return VerifyResult.Invalid;
            }
            else
            {
                if (!block.Hash.Equals(system.HeaderCache[block.Index].Hash))
                    return VerifyResult.Invalid;
            }
            block_cache.TryAdd(block.Hash, block);
            if (block.Index == currentHeight + 1)
            {
                Block block_persist = block;
                List<Block> blocksToPersistList = new();
                while (true)
                {
                    blocksToPersistList.Add(block_persist);
                    if (block_persist.Index + 1 > headerHeight) break;
                    UInt256 hash = system.HeaderCache[block_persist.Index + 1].Hash;
                    if (!block_cache.TryGetValue(hash, out block_persist)) break;
                }

                int blocksPersisted = 0;
                uint extraRelayingBlocks = system.Settings.MillisecondsPerBlock < ProtocolSettings.Default.MillisecondsPerBlock
                    ? (ProtocolSettings.Default.MillisecondsPerBlock - system.Settings.MillisecondsPerBlock) / 1000
                    : 0;
                foreach (Block blockToPersist in blocksToPersistList)
                {
                    block_cache_unverified.Remove(blockToPersist.Index);
                    Persist(blockToPersist);

                    if (blocksPersisted++ < blocksToPersistList.Count - (2 + extraRelayingBlocks)) continue;
                    // Empirically calibrated for relaying the most recent 2 blocks persisted with 15s network
                    // Increase in the rate of 1 block per second in configurations with faster blocks

                    if (blockToPersist.Index + 99 >= headerHeight)
                        system.LocalNode.Tell(new LocalNode.RelayDirectly { Inventory = blockToPersist });
                }
                if (block_cache_unverified.TryGetValue(currentHeight + 1, out var unverifiedBlocks))
                {
                    foreach (var unverifiedBlock in unverifiedBlocks.Blocks)
                        Self.Tell(unverifiedBlock, ActorRefs.NoSender);
                    block_cache_unverified.Remove(block.Index + 1);
                }
            }
            else
            {
                if (block.Index + 99 >= headerHeight)
                    system.LocalNode.Tell(new LocalNode.RelayDirectly { Inventory = block });
                if (block.Index == headerHeight + 1)
                    system.HeaderCache.Add(block.Header);
            }
            return VerifyResult.Succeed;
        }

        private void OnNewHeaders(Header[] headers)
        {
            if (!system.HeaderCache.Full)
            {
                DataCache snapshot = system.StoreView;
                uint headerHeight = system.HeaderCache.Last?.Index ?? NativeContract.Ledger.CurrentIndex(snapshot);
                foreach (Header header in headers)
                {
                    if (header.Index > headerHeight + 1) break;
                    if (header.Index < headerHeight + 1) continue;
                    if (!header.Verify(system.Settings, snapshot, system.HeaderCache)) break;
                    system.HeaderCache.Add(header);
                    ++headerHeight;
                }
            }
            system.TaskManager.Tell(headers, Sender);
        }

        private VerifyResult OnNewExtensiblePayload(ExtensiblePayload payload)
        {
            DataCache snapshot = system.StoreView;
            extensibleWitnessWhiteList ??= UpdateExtensibleWitnessWhiteList(system.Settings, snapshot);
            if (!payload.Verify(system.Settings, snapshot, extensibleWitnessWhiteList)) return VerifyResult.Invalid;
            system.RelayCache.Add(payload);
            return VerifyResult.Succeed;
        }

        private VerifyResult OnNewTransaction(Transaction transaction)
        {
            switch (system.ContainsTransaction(transaction.Hash))
            {
                case ContainsTransactionType.ExistsInPool: return VerifyResult.AlreadyInPool;
                case ContainsTransactionType.ExistsInLedger: return VerifyResult.AlreadyExists;
            }
            if (system.ContainsConflictHash(transaction.Hash, transaction.Signers.Select(s => s.Account))) return VerifyResult.HasConflicts;
            return system.MemPool.TryAdd(transaction, system.StoreView);
        }

        private void OnPreverifyCompleted(TransactionRouter.PreverifyCompleted task)
        {
            if (task.Result == VerifyResult.Succeed)
                OnInventory(task.Transaction, task.Relay);
            else
                SendRelayResult(task.Transaction, task.Result);
        }

        protected override void OnReceive(object message)
        {
            switch (message)
            {
                case Initialize:
                    OnInitialize();
                    break;
                case Import import:
                    OnImport(import.Blocks, import.Verify);
                    break;
                case FillMemoryPool fill:
                    OnFillMemoryPool(fill.Transactions);
                    break;
                case Header[] headers:
                    OnNewHeaders(headers);
                    break;
                case Block block:
                    OnInventory(block, false);
                    break;
                case Transaction tx:
                    OnTransaction(tx);
                    break;
                case IInventory inventory:
                    OnInventory(inventory);
                    break;
                case TransactionRouter.PreverifyCompleted task:
                    OnPreverifyCompleted(task);
                    break;
                case Reverify reverify:
                    foreach (IInventory inventory in reverify.Inventories)
                        OnInventory(inventory, false);
                    break;
                case Idle _:
                    if (system.MemPool.ReVerifyTopUnverifiedTransactionsIfNeeded(MaxTxToReverifyPerIdle, system.StoreView))
                        Self.Tell(Idle.Instance, ActorRefs.NoSender);
                    break;
            }
        }

        private void OnTransaction(Transaction tx)
        {
            switch (system.ContainsTransaction(tx.Hash))
            {
                case ContainsTransactionType.ExistsInPool:
                    SendRelayResult(tx, VerifyResult.AlreadyInPool);
                    break;
                case ContainsTransactionType.ExistsInLedger:
                    SendRelayResult(tx, VerifyResult.AlreadyExists);
                    break;
                default:
                    {
                        if (system.ContainsConflictHash(tx.Hash, tx.Signers.Select(s => s.Account)))
                            SendRelayResult(tx, VerifyResult.HasConflicts);
                        else system.TxRouter.Forward(new TransactionRouter.Preverify(tx, true));
                        break;
                    }
            }
        }

        private void Persist(Block block)
        {
            using (SnapshotCache snapshot = system.GetSnapshotCache())
            {
                List<ApplicationExecuted> all_application_executed = new();
                TransactionState[] transactionStates;
                using (ApplicationEngine engine = ApplicationEngine.Create(TriggerType.OnPersist, null, snapshot, block, system.Settings, 0))
                {
                    engine.LoadScript(onPersistScript);
                    if (engine.Execute() != VMState.HALT)
                    {
                        if (engine.FaultException != null)
                            throw engine.FaultException;
                        throw new InvalidOperationException();
                    }
                    ApplicationExecuted application_executed = new(engine);
                    Context.System.EventStream.Publish(application_executed);
                    all_application_executed.Add(application_executed);
                    transactionStates = engine.GetState<TransactionState[]>();
                }
                DataCache clonedSnapshot = snapshot.CloneCache();
                // Warning: Do not write into variable snapshot directly. Write into variable clonedSnapshot and commit instead.
                foreach (TransactionState transactionState in transactionStates)
                {
                    Transaction tx = transactionState.Transaction;
                    using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Application, tx, clonedSnapshot, block, system.Settings, tx.SystemFee);
                    engine.LoadScript(tx.Script);
                    transactionState.State = engine.Execute();
                    if (transactionState.State == VMState.HALT)
                    {
                        clonedSnapshot.Commit();
                    }
                    else
                    {
                        clonedSnapshot = snapshot.CloneCache();
                    }
                    ApplicationExecuted application_executed = new(engine);
                    Context.System.EventStream.Publish(application_executed);
                    all_application_executed.Add(application_executed);
                }
                using (ApplicationEngine engine = ApplicationEngine.Create(TriggerType.PostPersist, null, snapshot, block, system.Settings, 0))
                {
                    engine.LoadScript(postPersistScript);
                    if (engine.Execute() != VMState.HALT)
                    {
                        if (engine.FaultException != null)
                            throw engine.FaultException;
                        throw new InvalidOperationException();
                    }
                    ApplicationExecuted application_executed = new(engine);
                    Context.System.EventStream.Publish(application_executed);
                    all_application_executed.Add(application_executed);
                }
                InvokeCommitting(system, block, snapshot, all_application_executed);
                snapshot.Commit();
            }
            InvokeCommitted(system, block);
            system.MemPool.UpdatePoolForBlockPersisted(block, system.StoreView);
            extensibleWitnessWhiteList = null;
            block_cache.Remove(block.PrevHash);
            Context.System.EventStream.Publish(new PersistCompleted { Block = block });
            if (system.HeaderCache.TryRemoveFirst(out Header header))
                Debug.Assert(header.Index == block.Index);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static void InvokeCommitting(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<ApplicationExecuted> applicationExecutedList)
        {
            InvokeHandlers(Committing?.GetInvocationList(), h => ((CommittingHandler)h)(system, block, snapshot, applicationExecutedList));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static void InvokeCommitted(EpicChainSystem system, Block block)
        {
            InvokeHandlers(Committed?.GetInvocationList(), h => ((CommittedHandler)h)(system, block));
        }

        private static void InvokeHandlers(Delegate[] handlers, Action<Delegate> handlerAction)
        {
            if (handlers == null) return;

            foreach (var handler in handlers)
            {
                try
                {
                    // skip stopped plugin.
                    if (handler.Target is Plugin { IsStopped: true })
                    {
                        continue;
                    }

                    handlerAction(handler);
                }
                catch (Exception ex) when (handler.Target is Plugin plugin)
                {
                    Utility.Log(nameof(plugin), LogLevel.Error, ex);
                    switch (plugin.ExceptionPolicy)
                    {
                        case UnhandledExceptionPolicy.StopNode:
                            throw;
                        case UnhandledExceptionPolicy.StopPlugin:
                            //Stop plugin on exception
                            plugin.IsStopped = true;
                            break;
                        case UnhandledExceptionPolicy.Ignore:
                            // Log the exception and continue with the next handler
                            break;
                        default:
                            throw new InvalidCastException(
                                $"The exception policy {plugin.ExceptionPolicy} is not valid.");
                    }
                }
            }
        }

        /// <summary>
        /// Gets a <see cref="Akka.Actor.Props"/> object used for creating the <see cref="Blockchain"/> actor.
        /// </summary>
        /// <param name="system">The <see cref="EpicChainSystem"/> object that contains the <see cref="Blockchain"/>.</param>
        /// <returns>The <see cref="Akka.Actor.Props"/> object used for creating the <see cref="Blockchain"/> actor.</returns>
        public static Props Props(EpicChainSystem system)
        {
            return Akka.Actor.Props.Create(() => new Blockchain(system)).WithMailbox("blockchain-mailbox");
        }

        private void SendRelayResult(IInventory inventory, VerifyResult result)
        {
            RelayResult rr = new()
            {
                Inventory = inventory,
                Result = result
            };
            Sender.Tell(rr);
            Context.System.EventStream.Publish(rr);
        }

        private static ImmutableHashSet<UInt160> UpdateExtensibleWitnessWhiteList(ProtocolSettings settings, DataCache snapshot)
        {
            uint currentHeight = NativeContract.Ledger.CurrentIndex(snapshot);
            var builder = ImmutableHashSet.CreateBuilder<UInt160>();
            builder.Add(NativeContract.EpicChain.GetCommitteeAddress(snapshot));
            var validators = NativeContract.EpicChain.GetNextBlockValidators(snapshot, settings.ValidatorsCount);
            builder.Add(Contract.GetBFTAddress(validators));
            builder.UnionWith(validators.Select(u => Contract.CreateSignatureRedeemScript(u).ToScriptHash()));
            var stateValidators = NativeContract.QuantumGuardNexus.GetDesignatedByRole(snapshot, Role.StateValidator, currentHeight);
            if (stateValidators.Length > 0)
            {
                builder.Add(Contract.GetBFTAddress(stateValidators));
                builder.UnionWith(stateValidators.Select(u => Contract.CreateSignatureRedeemScript(u).ToScriptHash()));
            }
            return builder.ToImmutable();
        }
    }

    internal class BlockchainMailbox : PriorityMailbox
    {
        public BlockchainMailbox(Settings settings, Config config)
            : base(settings, config)
        {
        }

        internal protected override bool IsHighPriority(object message)
        {
            return message switch
            {
                Header[] or Block or ExtensiblePayload or Terminated => true,
                _ => false,
            };
        }
    }
}

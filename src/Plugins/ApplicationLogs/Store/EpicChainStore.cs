// Copyright (C) 2021-2024 EpicChain Labs.

//
// EpicChainStore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.Plugins.ApplicationLogs.Store.Models;
using EpicChain.Plugins.ApplicationLogs.Store.States;
using EpicChain.SmartContract;
using EpicChain.VM.Types;

namespace EpicChain.Plugins.ApplicationLogs.Store
{
    public sealed class EpicChainStore : IDisposable
    {
        #region Globals

        private readonly IStore _store;
        private ISnapshot _blocklogsnapshot;

        #endregion

        #region ctor

        public EpicChainStore(
            IStore store)
        {
            _store = store;
        }

        #endregion

        #region IDisposable

        public void Dispose()
        {
            _store?.Dispose();
            GC.SuppressFinalize(this);
        }

        #endregion

        #region Batching

        public void StartBlockLogBatch()
        {
            _blocklogsnapshot?.Dispose();
            _blocklogsnapshot = _store.GetSnapshot();
        }

        public void CommitBlockLog() =>
            _blocklogsnapshot?.Commit();

        #endregion

        #region Store

        public IStore GetStore() => _store;

        #endregion

        #region Contract

        public IReadOnlyCollection<(BlockchainEventModel NotifyLog, UInt256 TxHash)> GetContractLog(UInt160 scriptHash, uint page = 1, uint pageSize = 10)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            var lstModels = new List<(BlockchainEventModel NotifyLog, UInt256 TxHash)>();
            foreach (var contractState in lss.FindContractState(scriptHash, page, pageSize))
                lstModels.Add((BlockchainEventModel.Create(contractState, CreateStackItemArray(lss, contractState.StackItemIds)), contractState.TransactionHash));
            return lstModels;
        }

        public IReadOnlyCollection<(BlockchainEventModel NotifyLog, UInt256 TxHash)> GetContractLog(UInt160 scriptHash, TriggerType triggerType, uint page = 1, uint pageSize = 10)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            var lstModels = new List<(BlockchainEventModel NotifyLog, UInt256 TxHash)>();
            foreach (var contractState in lss.FindContractState(scriptHash, triggerType, page, pageSize))
                lstModels.Add((BlockchainEventModel.Create(contractState, CreateStackItemArray(lss, contractState.StackItemIds)), contractState.TransactionHash));
            return lstModels;
        }

        public IReadOnlyCollection<(BlockchainEventModel NotifyLog, UInt256 TxHash)> GetContractLog(UInt160 scriptHash, TriggerType triggerType, string eventName, uint page = 1, uint pageSize = 10)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            var lstModels = new List<(BlockchainEventModel NotifyLog, UInt256 TxHash)>();
            foreach (var contractState in lss.FindContractState(scriptHash, triggerType, eventName, page, pageSize))
                lstModels.Add((BlockchainEventModel.Create(contractState, CreateStackItemArray(lss, contractState.StackItemIds)), contractState.TransactionHash));
            return lstModels;
        }

        #endregion

        #region Engine

        public void PutTransactionEngineLogState(UInt256 hash, IReadOnlyList<LogEventArgs> logs)
        {
            using var lss = new LogStorageStore(_blocklogsnapshot);
            var ids = new List<Guid>();
            foreach (var log in logs)
                ids.Add(lss.PutEngineState(EngineLogState.Create(log.ScriptHash, log.Message)));
            lss.PutTransactionEngineState(hash, TransactionEngineLogState.Create(ids.ToArray()));
        }

        #endregion

        #region Block

        public BlockchainExecutionModel GetBlockLog(UInt256 hash, TriggerType trigger)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            if (lss.TryGetExecutionBlockState(hash, trigger, out var executionBlockStateId) &&
                lss.TryGetExecutionState(executionBlockStateId, out var executionLogState))
            {
                var model = BlockchainExecutionModel.Create(trigger, executionLogState, CreateStackItemArray(lss, executionLogState.StackItemIds));
                if (lss.TryGetBlockState(hash, trigger, out var blockLogState))
                {
                    var lstOfEventModel = new List<BlockchainEventModel>();
                    foreach (var notifyLogItem in blockLogState.NotifyLogIds)
                    {
                        if (lss.TryGetNotifyState(notifyLogItem, out var notifyLogState))
                            lstOfEventModel.Add(BlockchainEventModel.Create(notifyLogState, CreateStackItemArray(lss, notifyLogState.StackItemIds)));
                    }
                    model.Notifications = lstOfEventModel.ToArray();
                }
                return model;
            }
            return null;
        }

        public BlockchainExecutionModel GetBlockLog(UInt256 hash, TriggerType trigger, string eventName)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            if (lss.TryGetExecutionBlockState(hash, trigger, out var executionBlockStateId) &&
                lss.TryGetExecutionState(executionBlockStateId, out var executionLogState))
            {
                var model = BlockchainExecutionModel.Create(trigger, executionLogState, CreateStackItemArray(lss, executionLogState.StackItemIds));
                if (lss.TryGetBlockState(hash, trigger, out var blockLogState))
                {
                    var lstOfEventModel = new List<BlockchainEventModel>();
                    foreach (var notifyLogItem in blockLogState.NotifyLogIds)
                    {
                        if (lss.TryGetNotifyState(notifyLogItem, out var notifyLogState))
                        {
                            if (notifyLogState.EventName.Equals(eventName, StringComparison.OrdinalIgnoreCase))
                                lstOfEventModel.Add(BlockchainEventModel.Create(notifyLogState, CreateStackItemArray(lss, notifyLogState.StackItemIds)));
                        }
                    }
                    model.Notifications = lstOfEventModel.ToArray();
                }
                return model;
            }
            return null;
        }

        public void PutBlockLog(Block block, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList)
        {
            foreach (var appExecution in applicationExecutedList)
            {
                using var lss = new LogStorageStore(_blocklogsnapshot);
                var exeStateId = PutExecutionLogBlock(lss, block, appExecution);
                PutBlockAndTransactionLog(lss, block, appExecution, exeStateId);
            }
        }

        private static Guid PutExecutionLogBlock(LogStorageStore logStore, Block block, Blockchain.ApplicationExecuted appExecution)
        {
            var exeStateId = logStore.PutExecutionState(ExecutionLogState.Create(appExecution, CreateStackItemIdList(logStore, appExecution)));
            logStore.PutExecutionBlockState(block.Hash, appExecution.Trigger, exeStateId);
            return exeStateId;
        }

        #endregion

        #region Transaction

        public BlockchainExecutionModel GetTransactionLog(UInt256 hash)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            if (lss.TryGetExecutionTransactionState(hash, out var executionTransactionStateId) &&
                lss.TryGetExecutionState(executionTransactionStateId, out var executionLogState))
            {
                var model = BlockchainExecutionModel.Create(TriggerType.Application, executionLogState, CreateStackItemArray(lss, executionLogState.StackItemIds));
                if (lss.TryGetTransactionState(hash, out var transactionLogState))
                {
                    var lstOfEventModel = new List<BlockchainEventModel>();
                    foreach (var notifyLogItem in transactionLogState.NotifyLogIds)
                    {
                        if (lss.TryGetNotifyState(notifyLogItem, out var notifyLogState))
                            lstOfEventModel.Add(BlockchainEventModel.Create(notifyLogState, CreateStackItemArray(lss, notifyLogState.StackItemIds)));
                    }
                    model.Notifications = lstOfEventModel.ToArray();

                    if (lss.TryGetTransactionEngineState(hash, out var transactionEngineLogState))
                    {
                        var lstOfLogs = new List<ApplicationEngineLogModel>();
                        foreach (var logItem in transactionEngineLogState.LogIds)
                        {
                            if (lss.TryGetEngineState(logItem, out var engineLogState))
                                lstOfLogs.Add(ApplicationEngineLogModel.Create(engineLogState));
                        }
                        model.Logs = lstOfLogs.ToArray();
                    }
                }
                return model;
            }
            return null;
        }

        public BlockchainExecutionModel GetTransactionLog(UInt256 hash, string eventName)
        {
            using var lss = new LogStorageStore(_store.GetSnapshot());
            if (lss.TryGetExecutionTransactionState(hash, out var executionTransactionStateId) &&
                lss.TryGetExecutionState(executionTransactionStateId, out var executionLogState))
            {
                var model = BlockchainExecutionModel.Create(TriggerType.Application, executionLogState, CreateStackItemArray(lss, executionLogState.StackItemIds));
                if (lss.TryGetTransactionState(hash, out var transactionLogState))
                {
                    var lstOfEventModel = new List<BlockchainEventModel>();
                    foreach (var notifyLogItem in transactionLogState.NotifyLogIds)
                    {
                        if (lss.TryGetNotifyState(notifyLogItem, out var notifyLogState))
                        {
                            if (notifyLogState.EventName.Equals(eventName, StringComparison.OrdinalIgnoreCase))
                                lstOfEventModel.Add(BlockchainEventModel.Create(notifyLogState, CreateStackItemArray(lss, notifyLogState.StackItemIds)));
                        }
                    }
                    model.Notifications = lstOfEventModel.ToArray();

                    if (lss.TryGetTransactionEngineState(hash, out var transactionEngineLogState))
                    {
                        var lstOfLogs = new List<ApplicationEngineLogModel>();
                        foreach (var logItem in transactionEngineLogState.LogIds)
                        {
                            if (lss.TryGetEngineState(logItem, out var engineLogState))
                                lstOfLogs.Add(ApplicationEngineLogModel.Create(engineLogState));
                        }
                        model.Logs = lstOfLogs.ToArray();
                    }
                }
                return model;
            }
            return null;
        }

        private static void PutBlockAndTransactionLog(LogStorageStore logStore, Block block, Blockchain.ApplicationExecuted appExecution, Guid executionStateId)
        {
            if (appExecution.Transaction != null)
                logStore.PutExecutionTransactionState(appExecution.Transaction.Hash, executionStateId); // For looking up execution log by transaction hash

            var lstNotifyLogIds = new List<Guid>();
            for (uint i = 0; i < appExecution.Notifications.Length; i++)
            {
                var notifyItem = appExecution.Notifications[i];
                var stackItemStateIds = CreateStackItemIdList(logStore, notifyItem); // Save notify stack items
                logStore.PutContractState(notifyItem.ScriptHash, block.Timestamp, i, // save notifylog for the contracts
                    ContractLogState.Create(appExecution, notifyItem, stackItemStateIds));
                lstNotifyLogIds.Add(logStore.PutNotifyState(NotifyLogState.Create(notifyItem, stackItemStateIds)));
            }

            if (appExecution.Transaction != null)
                logStore.PutTransactionState(appExecution.Transaction.Hash, TransactionLogState.Create(lstNotifyLogIds.ToArray()));

            logStore.PutBlockState(block.Hash, appExecution.Trigger, BlockLogState.Create(lstNotifyLogIds.ToArray()));
        }

        #endregion

        #region StackItem

        private static StackItem[] CreateStackItemArray(LogStorageStore logStore, Guid[] stackItemIds)
        {
            var lstStackItems = new List<StackItem>();
            foreach (var stackItemId in stackItemIds)
                if (logStore.TryGetStackItemState(stackItemId, out var stackItem))
                    lstStackItems.Add(stackItem);
            return lstStackItems.ToArray();
        }

        private static Guid[] CreateStackItemIdList(LogStorageStore logStore, Blockchain.ApplicationExecuted appExecution)
        {
            var lstStackItemIds = new List<Guid>();
            foreach (var stackItem in appExecution.Stack)
                lstStackItemIds.Add(logStore.PutStackItemState(stackItem));
            return lstStackItemIds.ToArray();
        }

        private static Guid[] CreateStackItemIdList(LogStorageStore logStore, NotifyEventArgs notifyEventArgs)
        {
            var lstStackItemIds = new List<Guid>();
            foreach (var stackItem in notifyEventArgs.State)
                lstStackItemIds.Add(logStore.PutStackItemState(stackItem));
            return lstStackItemIds.ToArray();
        }

        #endregion
    }
}

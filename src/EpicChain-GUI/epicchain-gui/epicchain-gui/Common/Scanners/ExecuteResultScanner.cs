using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Neo.Common.Storage;
using Neo.Common.Storage.LevelDBModules;
using Neo.Common.Storage.SQLiteModules;
using Neo.Common.Utility;
using Neo.Ledger;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract.Native;
using Neo.VM;

namespace Neo.Common.Scanners
{
    /// <summary>
    /// Scan execute result log background
    /// </summary>
    public class ExecuteResultScanner
    {
        private TrackDB _db = new TrackDB();
        private LevelDbContext _levelDb = new LevelDbContext();
        private bool _running = true;
        private uint _scanHeight = 0;

        private uint _lastHeight = 0;
        private DateTime _lastTime;

        public async Task Start()
        {
            _running = true;
            _scanHeight = _db.GetMaxSyncIndex() ?? 0;
            _lastHeight = _scanHeight;
            _lastTime = DateTime.Now;
            while (_running)
            {
                try
                {
                    if (await Sync(_scanHeight))
                    {
                        if (_scanHeight - _lastHeight >= 500)
                        {
                            var span = DateTime.Now - _lastTime;
                            Console.WriteLine($"Sync[{_lastHeight}-{_scanHeight}],cost:{span.TotalSeconds}");
                            _lastTime = DateTime.Now;
                            _lastHeight = _scanHeight;
                        }
                        _scanHeight++;
                    }
                    if (_scanHeight > this.GetCurrentHeight())
                    {
                        await Task.Delay(TimeSpan.FromSeconds(5));
                    }
                }
                catch (Exception e)
                {
                    Console.WriteLine(e);
                    throw;
                }

            }
        }

        public void Stop()
        {
            _running = false;
        }


        /// <summary>
        /// analysis block transaction execute result logs
        /// </summary>
        /// <param name="blockHeight"></param>
        /// <returns></returns>
        public async Task<bool> Sync(uint blockHeight)
        {
            if (blockHeight > this.GetCurrentHeight())
            {
                return false;
            }

            if (_db.HasSyncIndex(blockHeight))
            {
                return true;
            }

            var block = blockHeight.GetBlock();
            var blockTime = block.Timestamp.FromTimestampMS();
            if (blockHeight == 0)
            {
                SyncNativeContracts(blockTime);
            }

            bool hasTxs = block.Transactions.Length > 0;
            if (hasTxs)
            {
                SyncContracts(blockHeight, blockTime);
                foreach (var transaction in block.Transactions)
                {
                    _db.AddTransaction(new TransactionInfo()
                    {
                        TxId = transaction.Hash,
                        BlockHeight = blockHeight,
                        Sender = transaction.Sender,
                        Time = blockTime,
                    });
                }
                var transfers = new List<TransferInfo>();
                var transferItems = _levelDb.GetTransfers(blockHeight);
                if (transferItems.NotEmpty())
                {
                    foreach (var item in transferItems.Where(t => t.TxId != null))
                    {
                        transfers.Add(new TransferInfo()
                        {
                            BlockHeight = blockHeight,
                            TimeStamp = block.Timestamp,
                            TxId = item.TxId,
                            From = item.From,
                            To = item.To,
                            Amount = item.Amount,
                            Asset = item.Asset,
                            Trigger = item.Trigger,
                            TokenId = item.TokenId
                        });
                    }
                }
                foreach (var transferInfo in transfers)
                {
                    _db.AddTransfer(transferInfo);
                }
            }
            SyncBalanceChanges(blockHeight);

            _db.AddSyncIndex(blockHeight);

            CommitAndReset(hasTxs);
            if (hasTxs)
            {
                Console.WriteLine($"Synced:{_scanHeight}[{block.Transactions.Length}]");
            }
            return true;
        }


        private void CommitAndReset(bool immediate)
        {
            var deadline = _db.LiveTime.TotalSeconds > 15;
            if (immediate || deadline)
            {
                _db.Commit();
            }
            if (deadline)
            {
                //release memory
                _db.Dispose();
                _db = new TrackDB();
            }
        }


        /// <summary>
        /// Should run after <see cref="SyncContracts"/> method
        /// update record will save after call <see cref="_db.Commit()"/> method;
        /// new record will save immediately
        /// </summary>
        /// <param name="blockHeight"></param>
        private void SyncBalanceChanges(uint blockHeight)
        {
            var balanceChanges = _levelDb.GetBalancingAccounts(blockHeight);
            if (balanceChanges.NotEmpty())
            {
                var snapshot = this.GetDefaultSnapshot();
                foreach (var balanceChange in balanceChanges)
                {
                    UpdateBalance(balanceChange.Account, balanceChange.Asset, snapshot);
                }
            }
        }


        /// <summary>
        /// sync native contracts state into sqldb immediately
        /// </summary>
        /// <param name="blockTime"></param>
        private void SyncNativeContracts(DateTime blockTime)
        {
            foreach (var nativeContract in NativeContract.Contracts)
            {
                var entity = new ContractEntity()
                {
                    Name = nativeContract.Name,
                    Hash = nativeContract.Hash.ToBigEndianHex(),
                    CreateTime = blockTime,
                };
                if (nativeContract is NeoToken neo)
                {
                    entity.Symbol = neo.Symbol;
                    entity.Decimals = neo.Decimals;
                    entity.AssetType = AssetType.Nep17;
                }
                if (nativeContract is GasToken gas)
                {
                    entity.Symbol = gas.Symbol;
                    entity.Decimals = gas.Decimals;
                    entity.AssetType = AssetType.Nep17;
                }
                _db.CreateContract(entity);
            }
        }

        /// <summary>
        /// sync contract create\update\delete state into sqldb immediately
        /// </summary>
        /// <param name="blockHeight"></param>
        /// <param name="blockTime"></param>
        private void SyncContracts(uint blockHeight, DateTime blockTime)
        {
            var contractEvents = _levelDb.GetContractEvent(blockHeight);
            if (contractEvents.NotEmpty())
            {
                foreach (var keyValuePair in contractEvents)
                {
                    var txId = keyValuePair.Key;
                    keyValuePair.Value?.ForEach(contractEvent => ProcessContractEvent(contractEvent, txId, blockTime));
                }
            }
        }

        /// <summary>
        /// save contract change info into sqldb immediately
        /// </summary>
        /// <param name="contractEvent"></param>
        /// <param name="txId"></param>
        /// <param name="blockTime"></param>
        private void ProcessContractEvent(ContractEventInfo contractEvent, UInt256 txId, DateTime blockTime)
        {
            switch (contractEvent.Event)
            {
                case ContractEventType.Create:
                    {
                        var newContract = GenerateContractEntity(contractEvent);
                        newContract.CreateTxId = txId.ToBigEndianHex();
                        newContract.CreateTime = blockTime;
                        _db.CreateContract(newContract);
                        break;
                    }
                case ContractEventType.Destroy:
                    _db.DeleteContract(contractEvent.Contract, txId, blockTime);
                    break;
                case ContractEventType.Migrate:
                    var migrateContract = GenerateContractEntity(contractEvent);
                    migrateContract.MigrateTxId = txId.ToBigEndianHex();
                    migrateContract.MigrateTime = blockTime;
                    _db.MigrateContract(migrateContract);
                    break;
            }
        }


        private ContractEntity GenerateContractEntity(ContractEventInfo contractEvent)
        {
            var newContract = new ContractEntity()
            {
                Hash = contractEvent.Contract.ToBigEndianHex(),
                Name = contractEvent.Name,
            };
            var asset = AssetCache.GetAssetInfoFromLevelDb(contractEvent.Contract);
            if (asset != null)
            {
                newContract.Name = asset.Name;
                newContract.Symbol = asset.Symbol;
                newContract.Decimals = asset.Decimals;
                newContract.AssetType = asset.Type;
            }
            return newContract;
        }


        /// <summary>
        /// get invoke methods from transaction script
        /// </summary>
        /// <param name="tx"></param>
        /// <returns></returns>
        private Dictionary<UInt160, HashSet<string>> GetInvokeMethods(Transaction tx)
        {
            var methodBox = new Dictionary<UInt160, HashSet<string>>();

            var instructions = OpCodeConverter.Parse(tx.Script);
            for (int i = 2; i < instructions.Count; i++)
            {
                var currentInstruction = instructions[i];
                if (currentInstruction.OpCode == OpCode.SYSCALL && currentInstruction.SystemCallMethod == "System.Contract.Call")
                {
                    var contractInstruction = instructions[i - 1];
                    var contract = contractInstruction.OpData.ToUInt160();
                    if (contract == null) { continue; }

                    var methodInstruction = instructions[i - 2];
                    var method = methodInstruction.OpDataUtf8String;
                    if (method.NotNull())
                    {
                        var box = methodBox.ContainsKey(contract) ? methodBox[contract] : new HashSet<string>();
                        box.Add(method);
                        methodBox[contract] = box;
                    }
                }
            }
            return methodBox;
        }


        /// <summary>
        /// update record will save after call <see cref="_db.Commit()"/> method;
        /// new record will save immediately
        /// </summary>
        /// <param name="account"></param>
        /// <param name="asset"></param>
        /// <param name="snapshot"></param>
        private void UpdateBalance(UInt160 account, UInt160 asset, DataCache snapshot)
        {
            try
            {
                var balance = account.GetBalanceOf(asset, snapshot).Value;
                _db.UpdateBalance(account, asset, balance, snapshot.GetHeight());
            }
            catch
            {
                var backupBalance = _levelDb.GetBalance(account, asset);
                if (backupBalance != null)
                {
                    _db.UpdateBalance(account, asset, backupBalance.Balance, backupBalance.Height);
                }
            }
        }
    }
}

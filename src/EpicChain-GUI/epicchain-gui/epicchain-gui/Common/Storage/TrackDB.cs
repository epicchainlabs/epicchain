using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Linq.Expressions;
using System.Numerics;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using Neo.Common.Consoles;
using Neo.Common.Storage.LevelDBModules;
using Neo.Common.Storage.SQLiteModules;
using Neo.Models;

namespace Neo.Common.Storage
{
    public class TrackDB : IDisposable
    {
        private readonly uint _magic;
        private readonly SQLiteContext _sqldb;
        public readonly LevelDbContext _leveldb;
        private readonly DateTime _createTime = DateTime.Now;

        public TimeSpan LiveTime => DateTime.Now - _createTime;

        private static bool _hasConsistencyCheck = false;
        private static bool _hasInitCahce = false;

        private static readonly ConcurrentDictionary<long, ContractEntity> ContractCache = new();
        private static readonly ConcurrentDictionary<UInt160, AddressEntity> AddressHashCache = new();
        private static readonly ConcurrentDictionary<long, AddressEntity> AddressIdCache = new();

        static TrackDB()
        {
            if (!Directory.Exists("Data_Track"))
            {
                Directory.CreateDirectory("Data_Track");
            }
        }

        void InitCache()
        {
            var contracts = _sqldb.Contracts.ToList();
            contracts.ForEach(c => ContractCache[c.Id] = c);

            var addresses = _sqldb.Addresses.ToList();
            addresses.ForEach(a => AddressIdCache[a.Id] = a);
        }


        public TrackDB()
        {
            _magic = CliSettings.Default.Protocol.Network;
            _sqldb = new SQLiteContext(Path.Combine($"Data_Track", $"track.{_magic}.db"));
            _leveldb = new LevelDbContext(Path.Combine("Data_Track", $"TransactionLog_LevelDB_{_magic}"));

            if (!_hasConsistencyCheck)
            {
                _hasConsistencyCheck = true;
                InitConsistencyCheck();
            }

            //if (!_hasInitCahce)
            //{
            //    _hasInitCahce = true;
            //    //InitCache();
            //}
        }


        /// <summary>
        /// check 2 db has the same sync index
        /// </summary>
        public void InitConsistencyCheck()
        {
            var levelMax = _leveldb.GetMaxSyncIndex(_sqldb.Identity);
            var sqlMax = _sqldb.GetMaxSyncIndex();
            if (levelMax == sqlMax)
            {
                return;
            }
            //if ((levelMax == null && sqlMax == 0) || levelMax < sqlMax)
            if (sqlMax.HasValue)
            {
                // try repair sync height
                // when last sync: sqldb save successfully, set leveldb  with same sqldb index
                Console.WriteLine($"Warning height:level[{levelMax}]-sql[{sqlMax}]");
                _leveldb.SetMaxSyncIndexForce(_sqldb.Identity, sqlMax.Value);
                _leveldb.Commit();
                return;
            }
            //throw new Exception("track db damaged!");
        }

        public void Commit()
        {
            _sqldb.SaveChanges();
            _leveldb.Commit();
        }

        #region SyncIndex

        public void AddSyncIndex(uint index)
        {
            _leveldb.SetMaxSyncIndex(_sqldb.Identity, index);
            _sqldb.SyncIndexes.Add(new SyncIndex() { BlockHeight = index });
        }

        public bool HasSyncIndex(uint index)
        {
            return _leveldb.HasSyncIndex(_sqldb.Identity, index);
        }


        public uint? GetMaxSyncIndex()
        {
            return _leveldb.GetMaxSyncIndex(_sqldb.Identity);
        }


        #endregion


        #region ExecuteResult

        /// <summary>
        /// 
        /// </summary>
        /// <param name="txId"></param>
        /// <returns></returns>
        public ExecuteResultInfo GetExecuteLog(UInt256 txId)
        {
            return _leveldb.GetExecuteLog(txId);
        }



        #endregion

        #region transfer


        /// <summary>
        /// will save after call <see cref="Commit"/> method
        /// </summary>
        /// <param name="transfer"></param>
        public void AddTransfer(TransferInfo transfer)
        {
            var from = GetOrCreateAddress(transfer.From);
            var to = GetOrCreateAddress(transfer.To);
            var asset = GetActiveContract(transfer.Asset);

            var tran = new TransferEntity
            {
                BlockHeight = transfer.BlockHeight,
                TxId = transfer.TxId?.ToBigEndianHex(),
                FromId = from?.Id,
                ToId = to?.Id,
                Amount = transfer.Amount.ToByteArray(),
                AssetId = asset.Id,
                Time = transfer.TimeStamp.FromTimestampMS(),
                Trigger = transfer.Trigger,
                TokenId = transfer.TokenId,
            };
            _sqldb.Transfers.Add(tran);
        }


        /// <summary>
        /// update record will save after call <see cref="Commit"/> method;
        /// new record will save immediately
        /// </summary>
        /// <param name="addressHash"></param>
        /// <param name="assetHash"></param>
        /// <param name="balance"></param>
        /// <param name="height"></param>
        public void UpdateBalance(UInt160 addressHash, UInt160 assetHash, BigInteger balance, uint height)
        {
            if (addressHash == null || assetHash == null) return;
            var asset = GetActiveContract(assetHash);
            if (asset == null)
            {
                throw new Exception($"Unkown asset:{assetHash}");
            }
            var address = GetOrCreateAddress(addressHash);
            var balanceRecord = GetOrCreateBalance(address, asset, balance, height);

            if (balanceRecord.BlockHeight >= height)
            {
                //no need update
                return;
            }
            balanceRecord.Balance = balance.ToByteArray();
            balanceRecord.BlockHeight = height;
        }


        /// <summary>
        ///  Paged by Transactions
        /// </summary>
        /// <param name="filter"></param>
        /// <returns></returns>
        public PageList<TransactionInfo> QueryTransactions(TransactionFilter filter, bool includeTransfers = false)
        {
            IQueryable<TransactionEntity> query = _sqldb.Transactions;
            if (includeTransfers)
            {
                query = query.Include(t => t.Transfers);
            }
            if (filter.StartTime != null)
            {
                query = query.Where(r => r.Time >= filter.StartTime.Value.ToUniversalTime());
            }
            if (filter.EndTime != null)
            {
                query = query.Where(r => r.Time <= filter.EndTime.Value.ToUniversalTime());
            }

            if (filter.BlockHeight != null)
            {
                query = query.Where(r => r.BlockHeight == filter.BlockHeight);
            }
            if (filter.TxIds.NotEmpty())
            {
                var txids = filter.TxIds.Select(t => t.ToBigEndianHex()).Distinct().ToList();
                query = query.Where(r => txids.Contains(r.TxId));
            }
            if (filter.FromOrTo.NotEmpty())
            {
                var addresses = filter.FromOrTo.Select(a => a.ToBigEndianHex()).Distinct().ToList();
                query = query.Where(tx =>
                    tx.Transfers.Any(t => addresses.Contains(t.From.Hash) || addresses.Contains(t.To.Hash)));
            }
            if (filter.From.NotEmpty())
            {
                var addresses = filter.From.Select(a => a.ToBigEndianHex()).Distinct().ToList();
                query = query.Where(tx =>
                    tx.Transfers.Any(t => addresses.Contains(t.From.Hash)));
            }
            if (filter.To.NotEmpty())
            {
                var addresses = filter.To.Select(a => a.ToBigEndianHex()).Distinct().ToList();
                query = query.Where(tx =>
                    tx.Transfers.Any(t => addresses.Contains(t.To.Hash)));
            }

            //if (filter.Contracts.NotEmpty())
            //{
            //    var contracts = filter.Contracts.Select(a => a.ToBigEndianHex()).Distinct().ToList();
            //    query = query.Where(tx => tx.InvokeContracts.Any(c => contracts.Contains(c.Contract.Hash) && c.Contract.DeleteTxId == null));
            //}
            if (filter.Assets.NotEmpty())
            {
                var assets = filter.Assets.Select(a => a.ToBigEndianHex()).Distinct().ToList();
                var assetIds = _sqldb.Contracts.Where(c => assets.Contains(c.Hash) && c.DeleteTxId == null).Select(c => c.Id).ToList();
                query = query.Where(tx => tx.Transfers.Any(c => assetIds.Contains(c.AssetId)));
            }
            var pageList = new PageList<TransactionInfo>();
            var pageIndex = filter.PageIndex <= 0 ? 0 : filter.PageIndex - 1;
            pageList.TotalCount = query.Count();
            pageList.PageIndex = pageIndex + 1;
            pageList.PageSize = filter.PageSize;
            if (filter.PageSize > 0)
            {
                var list = query.OrderByDescending(g => g.BlockHeight)
                    .Skip(pageIndex * filter.PageSize)
                    .Take(filter.PageSize).ToList();

                foreach (var tx in list.ToList())
                {
                    var transaction = new TransactionInfo()
                    {
                        TxId = UInt256.Parse(tx.TxId),
                        BlockHeight = tx.BlockHeight,
                        Sender = tx.Sender != null ? UInt160.Parse(tx.Sender.Hash) : null,
                        Time = tx.Time.AsUtcTime(),
                    };
                    if (tx.Transfers.NotEmpty())
                    {
                        var time = tx.Time.ToTimestampMS();
                        var transfers = new List<TransferInfo>();
                        foreach (var t in tx.Transfers)
                        {
                            transfers.Add(new TransferInfo()
                            {
                                From = t.FromId != null ? UInt160.Parse(GetAddress(t.FromId.Value).Hash) : null,
                                To = t.ToId != null ? UInt160.Parse(GetAddress(t.ToId.Value).Hash) : null,
                                Amount = new BigInteger(t.Amount),
                                TxId = UInt256.Parse(t.TxId),
                                Asset = UInt160.Parse(GetContract(t.AssetId).Hash),
                                TimeStamp = time,
                            });
                        }

                        transaction.Transfers = transfers;
                    }
                    pageList.List.Add(transaction);
                }
            }
            return pageList;
        }


        ///// <summary>
        ///// query without transfers(High Performance)
        ///// </summary>
        //private readonly Expression<Func<TransactionEntity, TransactionInfo>> ToTransactionWithoutTransfer = (tx) => new TransactionInfo()
        //{
        //    TxId = UInt256.Parse(tx.TxId),
        //    BlockHeight = tx.BlockHeight,
        //    Sender = tx.Sender != null ? UInt160.Parse(tx.Sender.Hash) : null,
        //    Time = tx.Time.AsUtcTime(),
        //};

        ///// <summary>
        /////  query with transfers(Low Performance)
        ///// </summary>
        //private readonly Expression<Func<TransactionEntity, TransactionInfo>> ToTransactionWithTransfer = (tx) => new TransactionInfo()
        //{
        //    TxId = UInt256.Parse(tx.TxId),
        //    BlockHeight = tx.BlockHeight,
        //    Sender = tx.Sender != null ? UInt160.Parse(tx.Sender.Hash) : null,
        //    Time = tx.Time.AsUtcTime(),
        //    //Transfers = tx.Transfers.ToList(),
        //    //    .Select(t=>new TransferInfo(){
        //    //    From = t.FromId != null ? UInt160.Parse(GetAddress(t.FromId.Value).Hash) : null,
        //    //    To = t.To != null ? UInt160.Parse(t.To.Hash) : null,
        //    //    Amount = new BigInteger(t.Amount),
        //    //    TxId = UInt256.Parse(t.TxId),
        //    //    Asset = UInt160.Parse(t.Asset.Hash),
        //    //    TimeStamp = t.Time.ToTimestampMS(),
        //    //}),
        //    //Transfers = tx.Transfers.Select(t => new TransferInfo()
        //    //{
        //    //    From = t.From != null ? UInt160.Parse(t.From.Hash) : null,
        //    //    To = t.To != null ? UInt160.Parse(t.To.Hash) : null,
        //    //    Amount = new BigInteger(t.Amount),
        //    //    TxId = UInt256.Parse(t.TxId),
        //    //    Asset = UInt160.Parse(t.Asset.Hash),
        //    //    TimeStamp = t.Time.ToTimestampMS(),
        //    //}).ToList()
        //};



        /// <summary>
        ///  Query Transfers Paged by Transactions
        /// </summary>
        /// <param name="filter"></param>
        /// <returns></returns>
        public PageList<TransferInfo> QueryTransfersPagedByTx(TransferFilter filter)
        {
            var query = BuildQuery(filter);
            var pageList = new PageList<TransferInfo>();
            var pageIndex = filter.PageIndex <= 0 ? 0 : filter.PageIndex - 1;
            pageList.TotalCount = query.GroupBy(q => q.TxId).Count();
            pageList.PageIndex = pageIndex + 1;
            pageList.PageSize = filter.PageSize;
            if (filter.PageSize > 0)
            {
                var txIds = query.GroupBy(q => new { q.TxId, q.Time }).OrderByDescending(g => g.Key.Time).Select(g => g.Key)
                    .Skip(pageIndex * filter.PageSize)
                    .Take(filter.PageSize).Select(g => g.TxId).ToList();
                pageList.List.AddRange(query.Where(q => txIds.Contains(q.TxId)).OrderByDescending(r => r.Time).ToList().Select(ToTransferInfo));
            }
            return pageList;
        }


        /// <summary>
        /// Query Transfers Paged by transfer
        /// </summary>
        /// <param name="filter"></param>
        /// <returns></returns>
        public PageList<TransferInfo> QueryTransfers(TransferFilter filter)
        {
            var query = BuildQuery(filter);
            var pageList = new PageList<TransferInfo>();
            var pageIndex = filter.PageIndex <= 0 ? 0 : filter.PageIndex - 1;
            pageList.TotalCount = query.Count();
            pageList.PageIndex = pageIndex + 1;
            pageList.PageSize = filter.PageSize;
            if (filter.PageSize > 0)
            {
                pageList.List.AddRange(query.OrderByDescending(r => r.Time).Skip(pageIndex * filter.PageSize)
                    .Take(filter.PageSize).ToList().Select(ToTransferInfo));
            }
            return pageList;
        }


        /// <summary>
        /// query balances
        /// </summary>
        /// <param name="filter"></param>
        /// <returns></returns>
        public IEnumerable<BalanceInfo> FindAssetBalance(BalanceFilter filter)
        {
            IQueryable<AssetBalanceEntity> query = _sqldb.AssetBalances.Include(a => a.Address).Include(a => a.Asset).Where(a => a.Asset.DeleteTxId == null);
            if (filter.Addresses.NotEmpty())
            {
                var addrs = filter.Addresses.Select(a => a.ToBigEndianHex()).ToList();
                query = query.Where(q => addrs.Contains(q.Address.Hash));
            }

            if (filter.Assets.NotEmpty())
            {
                var assets = filter.Assets.Select(a => a.ToBigEndianHex()).ToList();
                query = query.Where(q => assets.Contains(q.Asset.Hash));
            }

            var balances = query.ToList();
            return balances.Select(b => new BalanceInfo()
            {
                Address = UInt160.Parse(b.Address.Hash),
                Asset = UInt160.Parse(b.Asset.Hash),
                AssetName = b.Asset.Name,
                AssetSymbol = b.Asset.Symbol,
                AssetDecimals = b.Asset.Decimals,
                Balance = new BigInteger(b.Balance),
                BlockHeight = b.BlockHeight,
            });
        }



        #endregion


        #region Transaction

        public void AddTransaction(TransactionInfo transaction)
        {
            var txId = transaction.TxId.ToBigEndianHex();
            var old = _sqldb.Transactions.FirstOrDefault(t => t.TxId == txId);
            if (old != null)
            {
                return;
            }
            var sender = GetOrCreateAddress(transaction.Sender);
            _sqldb.Transactions.Add(new TransactionEntity()
            {
                TxId = txId,
                BlockHeight = transaction.BlockHeight,
                Time = transaction.Time,
                SenderId = sender?.Id,
            });
        }


        //public void AddInvokeTransaction(UInt256 txId, UInt160 contract, string method)
        //{
        //    var contractEntity = GetActiveContract(contract);
        //    if (contractEntity != null)
        //    {
        //        _sqldb.InvokeRecords.Add(new InvokeRecordEntity()
        //        {
        //            ContractId = contractEntity.Id,
        //            TxId = txId.ToBigEndianHex(),
        //            Methods = method,
        //        });
        //    }
        //}
        #endregion


        #region Contract




        /// <summary>
        /// Create contract, save immediately
        /// </summary>
        /// <param name="newContract"></param>
        public void CreateContract(ContractEntity newContract)
        {
            var old = GetActiveContract(newContract.Hash);
            if (old == null)
            {
                _sqldb.Contracts.Add(newContract);
                _sqldb.SaveChanges();
            }
        }


        /// <summary>
        /// Destroy contract, save immediately
        /// </summary>
        /// <param name="contractHash"></param>
        /// <param name="txId"></param>
        /// <param name="time"></param>
        public void DeleteContract(UInt160 contractHash, UInt256 txId, DateTime time)
        {
            var old = GetActiveContract(contractHash);
            if (old != null)
            {
                old.DeleteTxId = txId.ToBigEndianHex();
                old.DeleteTime = time;
                _sqldb.SaveChanges();
            }
        }

        /// <summary>
        /// Migrate contract, save immediately
        /// </summary>
        /// <param name="migrateContract"></param>
        public void MigrateContract(ContractEntity migrateContract)
        {
            var old = GetActiveContract(migrateContract.Hash);
            if (old != null)
            {
                var newUpdateRecord = new ContractUpdateRecordEntity()
                {
                    Hash = old.Hash,
                    MigrateTxId = migrateContract.MigrateTxId,
                    MigrateTime = migrateContract.MigrateTime.Value,
                };

                old.MigrateTxId = migrateContract.MigrateTxId;
                old.MigrateTime = migrateContract.MigrateTime;
                old.Symbol = migrateContract.Symbol;
                old.AssetType = migrateContract.AssetType;
                old.Decimals = migrateContract.Decimals;

                _sqldb.ContractUpdateRecords.Add(newUpdateRecord);
                _sqldb.SaveChanges();
            }
        }

        public IEnumerable<ContractEntity> GetAllContracts()
        {
            return _sqldb.Contracts.ToList();
        }

        /// <summary>
        /// Get NOT Deleted contract
        /// </summary>
        /// <param name="contract"></param>
        /// <returns></returns>
        public ContractEntity GetActiveContract(UInt160 contract)
        {
            var contractHash = contract.ToBigEndianHex();
            return GetActiveContract(contractHash);
        }

        /// <summary>
        /// Get NOT Deleted contract
        /// </summary>
        /// <param name="contractHash"></param>
        /// <returns></returns>
        private ContractEntity GetActiveContract(string contractHash)
        {
            return _sqldb.Contracts.FirstOrDefault(c => c.Hash == contractHash && c.DeleteTxId == null);
        }

        /// <summary>
        /// Ger contract by ID
        /// </summary>
        /// <param name="id"></param>
        /// <returns></returns>
        public ContractEntity GetContract(long id)
        {
            if (ContractCache.ContainsKey(id))
            {
                return ContractCache[id];
            }
            var contract = _sqldb.Contracts.FirstOrDefault(c => c.Id == id);
            if (contract != null)
            {
                ContractCache[id] = contract;
            }
            return contract;
        }

        #endregion

        #region Private



        private AddressEntity GetOrCreateAddress(UInt160 address)
        {
            if (address == null) return null;
            if (AddressHashCache.ContainsKey(address))
            {
                return AddressHashCache[address];
            }
            var addr = address.ToBigEndianHex();
            var old = _sqldb.Addresses.FirstOrDefault(a => a.Hash == addr);
            if (old == null)
            {
                old = new AddressEntity() { Hash = addr };
                _sqldb.Addresses.Add(old);
                _sqldb.SaveChanges();
            }
            AddressHashCache[address] = old;
            AddressIdCache[old.Id] = old;
            return old;
        }

        private AssetBalanceEntity GetOrCreateBalance(AddressEntity address, ContractEntity asset, BigInteger balance, uint height)
        {
            var old = _sqldb.AssetBalances.FirstOrDefault(a => a.AddressId == address.Id && a.AssetId == asset.Id);
            if (old == null)
            {
                old = new AssetBalanceEntity() { AddressId = address.Id, AssetId = asset.Id, Balance = balance.ToByteArray(), BlockHeight = height };
                _sqldb.AssetBalances.Add(old);
                _sqldb.SaveChanges();
            }
            return old;
        }


        private IQueryable<TransferEntity> BuildQuery(TransferFilter filter)
        {
            IQueryable<TransferEntity> query = _sqldb.Transfers.Where(t => t.TxId != null)
                .Include(t => t.From)
                .Include(t => t.To)
                .Include(t => t.Asset).Where(t => t.Asset.DeleteTxId == null);

            if (filter.Asset != null)
            {
                var assetHash = filter.Asset.ToBigEndianHex();
                var asset = _sqldb.Contracts.FirstOrDefault(a => a.Hash == assetHash);
                var assetId = asset?.Id ?? -1;
                query = query.Where(r => r.AssetId == assetId);
            }
            if (filter.TxIds.NotEmpty())
            {
                var txids = filter.TxIds.Select(t => t.ToBigEndianHex()).Distinct().ToList();
                query = query.Where(r => txids.Contains(r.TxId));
            }
            if (filter.FromOrTo.NotEmpty())
            {
                var addresses = filter.FromOrTo.Select(a => a.ToBigEndianHex()).ToList();
                var addressIds = _sqldb.Addresses.Where(a => addresses.Contains(a.Hash)).Select(a => a.Id).ToList();
                query = query.Where(r => addressIds.Contains(r.FromId.Value) || addressIds.Contains(r.ToId.Value));
            }
            if (filter.From.NotEmpty())
            {
                var addresses = filter.From.Select(a => a.ToBigEndianHex()).ToList();
                var addressIds = _sqldb.Addresses.Where(a => addresses.Contains(a.Hash)).Select(a => a.Id).ToList();
                query = query.Where(r => addressIds.Contains(r.FromId.Value));
            }
            if (filter.To.NotEmpty())
            {
                var addresses = filter.To.Select(a => a.ToBigEndianHex()).ToList();
                var addressIds = _sqldb.Addresses.Where(a => addresses.Contains(a.Hash)).Select(a => a.Id).ToList();
                query = query.Where(r => addressIds.Contains(r.ToId.Value));
            }
            if (filter.StartTime != null)
            {
                query = query.Where(r => r.Time >= filter.StartTime.Value.ToUniversalTime());
            }
            if (filter.EndTime != null)
            {
                query = query.Where(r => r.Time <= filter.EndTime.Value.ToUniversalTime());
            }
            if (filter.BlockHeight != null)
            {
                query = query.Where(r => r.BlockHeight == filter.BlockHeight);
            }
            return query;
        }


        private TransferInfo ToTransferInfo(TransferEntity entity)
        {
            return new TransferInfo()
            {
                BlockHeight = entity.BlockHeight,
                TxId = entity.TxId != null ? UInt256.Parse(entity.TxId) : null,
                From = entity.From != null ? UInt160.Parse(entity.From.Hash) : null,
                To = entity.To != null ? UInt160.Parse(entity.To.Hash) : null,
                Amount = new BigInteger(entity.Amount),
                Asset = UInt160.Parse(entity.Asset.Hash),
                TokenId = entity.TokenId,
                TimeStamp = entity.Time.AsUtcTime().ToTimestampMS(),
            };
        }

        #endregion

        #region AddressCache



        public AddressEntity GetAddress(long id)
        {
            if (AddressIdCache.ContainsKey(id))
            {
                return AddressIdCache[id];
            }
            var address = _sqldb.Addresses.FirstOrDefault(c => c.Id == id);
            if (address != null)
            {
                AddressIdCache[id] = address;
            }
            return address;
        }

        #endregion


        public void Dispose()
        {
            _sqldb?.Dispose();
            _leveldb?.Dispose();
        }
    }
}

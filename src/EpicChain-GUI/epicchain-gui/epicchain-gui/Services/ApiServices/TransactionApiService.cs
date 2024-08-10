using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Akka.Actor;
using Neo.Common.Consoles;
using Neo.Common.Storage;
using Neo.Common.Utility;
using Neo.IO;
using Neo.Ledger;
using Neo.Models;
using Neo.Models.Transactions;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract.Native;

namespace Neo.Services.ApiServices
{
    public class TransactionApiService : ApiService
    {


        /// <summary>
        /// query transaction info
        /// </summary>
        /// <param name="txId"></param>
        /// <returns></returns>
        public async Task<object> GetTransaction(UInt256 txId)
        {
            var snapshot = Helpers.GetDefaultSnapshot();
            var transaction = snapshot.GetTransaction(txId);
            if (transaction == null)
            {
                return Error(ErrorCode.TxIdNotFound);
            }

            var model = new TransactionModel(transaction);

            var txState = snapshot.GetTransactionState(txId);
            if (txState != null)
            {
                Header header = snapshot.GetHeader(txState.BlockIndex);
                model.BlockHash = header.Hash;
                model.BlockHeight = txState.BlockIndex;
                model.Timestamp = header.Timestamp;
                model.Confirmations = snapshot.GetHeight() - header.Index + 1;
            }
            using var db = new TrackDB();
            var trans = db.QueryTransfers(new TransferFilter() { TxIds = new List<UInt256>() { txId }, PageSize = int.MaxValue }).List;
            model.Transfers = trans.Select(tx => tx.ToTransferModel()).ToList();

            var executeResult = db.GetExecuteLog(txId);
            if (executeResult?.Notifications.NotEmpty() == true)
            {
                model.Notifies.AddRange(
                    executeResult.Notifications.Select(n => new NotifyModel()
                    {
                        Contract = n.Contract,
                        EventName = n.EventName,
                        State = n.State.ToJStackItem(),
                    }));
            }
            return model;
        }



        /// <summary>
        /// query transaction info
        /// </summary>
        /// <param name="txId"></param>
        /// <param name="showJson"></param>
        /// <returns></returns>
        public async Task<object> GetRawTransaction(UInt256 txId, bool showJson = false)
        {
            var snapshot = Helpers.GetDefaultSnapshot();
            var transaction = snapshot.GetTransaction(txId);
            if (transaction == null)
            {
                return Error(ErrorCode.TxIdNotFound);
            }
            if (showJson)
            {
                var json = transaction.ToJson(CliSettings.Default.Protocol);
                TransactionState txState = snapshot.GetTransactionState(txId);
                if (txState != null)
                {
                    Header header = snapshot.GetHeader(txState.BlockIndex);
                    json["blockhash"] = header.Hash.ToString();
                    json["confirmations"] = snapshot.GetHeight() - header.Index + 1;
                    json["blocktime"] = header.Timestamp;
                    using var db = new TrackDB();
                    var executelog = db.GetExecuteLog(txId);
                    json["vm_state"] = executelog?.VMState;
                }
                return json.ToString();
            }
            return transaction.ToArray().ToHexString();
        }


        public async Task<object> GetUnconfirmedTransaction(UInt256 txId)
        {
            Program.Starter.NeoSystem.MemPool.TryGetValue(txId, out var tx);
            if (tx == null)
            {
                UnconfirmedTransactionCache.RemoveUnconfirmedTransactions(txId);
                return Error(ErrorCode.TxIdNotFound);
            }
            //var transaction = Helpers.GetDefaultSnapshot().GetTransaction(txId);
            //if (transaction == null)
            //{
            //}

            var model = new TransactionModel(tx);
            var tempTx = UnconfirmedTransactionCache.GetUnconfirmedTransaction(txId);
            if (tempTx?.Transfers.NotEmpty() == true)
            {
                model.Transfers = tempTx.Transfers.Select(n => new TransferModel()
                {
                    From = n.From,
                    To = n.To,
                    Amount = new BigDecimal(n.Amount, n.Decimals).ToString(),
                    Symbol = n.Symbol,
                }).ToList();
            }
            return model;
        }


        /// <summary>
        /// get all unconfirmed transactions
        /// </summary>
        /// <param name="pageIndex"></param>
        /// <param name="limit"></param>
        /// <returns></returns>
        public async Task<object> GetUnconfirmTransactions(int pageIndex = 1, int limit = 100)
        {
            var tempTransactions = UnconfirmedTransactionCache.GetUnconfirmedTransactions(null, pageIndex, limit);
            var result = tempTransactions.Project(t => t.ToTransactionPreviewModel());
            return result;
        }


        public async Task<object> RemoveUnconfirmTransaction(UInt256 txId)
        {
            UnconfirmedTransactionCache.RemoveUnconfirmedTransactions(txId);
            return true;
        }


        public async Task<object> SendTransaction(string txRaw)
        {
            Transaction tx = Convert.FromBase64String(txRaw).AsSerializable<Transaction>();
            Blockchain.RelayResult reason = Program.Starter.NeoSystem.Blockchain.Ask<Blockchain.RelayResult>(tx).Result;
            return tx.ToJson(null);
        }

        /// <summary>
        /// query all transactions(on chain)
        /// </summary>
        /// <returns></returns>
        public async Task<object> QueryTransactions(int pageIndex = 1, int limit = 100, uint? blockHeight = null, UInt160 address = null, UInt160 contract = null, UInt256 blockHash = null)
        {
            using var db = new TrackDB();
            var filter = new TransactionFilter()
            {
                BlockHeight = blockHeight,
                PageIndex = pageIndex,
                PageSize = limit
            };
            if (address != null)
            {
                filter.FromOrTo = new List<UInt160>() { address };
            }
            if (contract != null)
            {
                filter.Assets = new List<UInt160>() { contract };
            }
            if (blockHash != null)
            {
                var block = this.GetDefaultSnapshot().GetBlock(blockHash);
                if (block != null)
                {
                    filter.BlockHeight = block.Index;
                }
            }

            var trans = db.QueryTransactions(filter, true);
            var result = new PageList<TransactionPreviewModel>
            {
                TotalCount = trans.TotalCount,
                PageSize = trans.PageSize,
                PageIndex = pageIndex,
                List = ConvertToTransactionPreviewModel(trans.List),
            };
            return result;
        }



        /// <summary>
        /// query all nep transactions(on chain)
        /// </summary>
        /// <returns></returns>
        public async Task<object> QueryNep5Transactions(int pageIndex = 1, int limit = 100, UInt160 address = null, UInt160 asset = null, uint? blockHeight = null)
        {
            using var db = new TrackDB();
            var filter = new TransferFilter()
            {
                BlockHeight = blockHeight,
                PageIndex = pageIndex,
                Asset = asset,
                PageSize = limit
            };
            if (address != null)
            {
                filter.FromOrTo = new List<UInt160>() { address };
            }
      
            var trans = db.QueryTransfersPagedByTx(filter);
            var result = new PageList<TransactionPreviewModel>
            {
                TotalCount = trans.TotalCount,
                PageSize = trans.PageSize,
                PageIndex = pageIndex,
                List = trans.List.ToTransactionPreviewModel(),
            };
            return result;
        }

        /// <summary>
        /// query transaction info
        /// </summary>
        /// <returns></returns>
        public async Task<PageList<TransferInfo>> QueryTransfers(TransferFilter filter)
        {
            using var db = new TrackDB();
            var result = db.QueryTransfers(filter) as PageList<TransferInfo>;
            return result;
        }


        /// <summary>
        /// query transaction info
        /// </summary>
        /// <returns></returns>
        public async Task<PageList<TransferInfo>> QueryTransfersByTx(TransferFilter filter)
        {
            using var db = new TrackDB();
            var result = db.QueryTransfersPagedByTx(filter) as PageList<TransferInfo>;
            return result;
        }

        /// <summary>
        /// GetTransactionLog
        /// </summary>
        /// <returns></returns>
        public async Task<object> GetApplicationLog(UInt256 txId)
        {
            var db = new TrackDB();
            var result = db.GetExecuteLog(txId);
            return result;
        }


        /// <summary>
        /// test
        /// </summary>
        /// <param name="index"></param>
        /// <returns></returns>
        public async Task<object> Analysis(uint index)
        {
            var result = Program.Starter.ExecuteResultScanner.Sync(index);
            return result;
        }



        private List<TransactionPreviewModel> ConvertToTransactionPreviewModel(List<TransactionInfo> transactions)
        {
            var model = transactions.Select(tx => new TransactionPreviewModel()
            {
                TxId = tx.TxId,
                Timestamp = tx.Time.ToTimestampMS(),
                BlockHeight = tx.BlockHeight,
                Transfers = tx.Transfers?.Select(t => t.ToTransferModel()).ToList(),
            }).ToList();

            return model;
        }

    }
}

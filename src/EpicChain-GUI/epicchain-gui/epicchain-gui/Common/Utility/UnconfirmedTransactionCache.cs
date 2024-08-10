using System.Collections.Generic;
using System.Linq;
using Akka.Actor;
using Neo.Ledger;
using Neo.Models;
using Neo.Models.Jobs;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using VmArray = Neo.VM.Types.Array;

namespace Neo.Common.Utility
{
    public class UnconfirmedTransactionCache
    {
        private static readonly Dictionary<UInt256, TempTransaction> _unconfirmedTransactions = new Dictionary<UInt256, TempTransaction>();

        private static IActorRef _actor;



        public static void RegisterBlockPersistEvent(NeoSystem neoSystem)
        {
            if (_actor == null)
            {
                _actor = neoSystem.ActorSystem.ActorOf(EventWrapper<Blockchain.PersistCompleted>.Props(Blockchain_PersistCompleted));
            }
        }

        public static void AddTransaction(Transaction tx)
        {
            if (_unconfirmedTransactions.ContainsKey(tx.Hash))
            {
                return;
            }

            var transfers = new List<TransferNotifyItem>();
            using var exeResult = tx.Script.RunTestMode(null, tx);
            if (exeResult.Notifications.NotEmpty())
            {
                foreach (var notification in exeResult.Notifications)
                {
                    var transfer = notification.ConvertToTransfer();
                    if (transfer == null)
                    {
                        continue;
                    }
                    var asset = AssetCache.GetAssetInfo(transfer.Asset);
                    if (asset == null)
                    {
                        continue;
                    }
                    transfer.Symbol = asset.Symbol;
                    transfer.Decimals = asset.Decimals;
                    transfers.Add(transfer);
                }
            }
            SaveTransfer(tx, transfers);
        }

        private static void SaveTransfer(Transaction tx, List<TransferNotifyItem> transfers)
        {
            _unconfirmedTransactions[tx.Hash] = new TempTransaction()
            {
                Tx = tx,
                Transfers = transfers,
            };
        }

        public static void RemoveUnconfirmedTransactions(UInt256 txId)
        {
            if (_unconfirmedTransactions.ContainsKey(txId))
            {
                var confirmTransaction = _unconfirmedTransactions[txId];
                TransactionConfirmJob.AddConfirmedTransaction(confirmTransaction);
                _unconfirmedTransactions.Remove(txId);
            }
        }


        public static TempTransaction GetUnconfirmedTransaction(UInt256 txId)
        {
            if (_unconfirmedTransactions.TryGetValue(txId, out var tx))
            {
                return tx;
            }
            return null;
        }
        public static PageList<TempTransaction> GetUnconfirmedTransactions(IEnumerable<UInt160> addresses = null, int pageIndex = 1, int pageSize = 10)
        {
            IEnumerable<TempTransaction> query = _unconfirmedTransactions.Values;
            if (addresses.NotEmpty())
            {
                query = query.Where(tx => tx.Transfers.Any(t => addresses.Contains(t.From) || addresses.Contains(t.To)));
            }
            var trans = query.Reverse().ToList();
            var result = new PageList<TempTransaction>();
            result.TotalCount = trans.Count();
            result.PageSize = pageSize;
            result.PageIndex = pageIndex;
            result.List = trans.Skip((pageIndex - 1) * pageSize).Take(pageSize).ToList();
            return result;
        }

        private static void Blockchain_PersistCompleted(Blockchain.PersistCompleted e)
        {
            if (e.Block.Transactions.NotEmpty())
            {
                foreach (var blockTransaction in e.Block.Transactions)
                {
                    RemoveUnconfirmedTransactions(blockTransaction.Hash);
                }
            }
        }


        public class TempTransaction
        {
            public Transaction Tx { get; set; }
            public List<TransferNotifyItem> Transfers { get; set; }
        }
    }

}

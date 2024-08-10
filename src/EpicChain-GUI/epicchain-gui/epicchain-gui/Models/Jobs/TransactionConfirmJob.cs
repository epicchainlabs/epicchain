using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Common.Utility;

namespace Neo.Models.Jobs
{
    public class TransactionConfirmJob : Job
    {
        private static readonly ConcurrentBag<UnconfirmedTransactionCache.TempTransaction> _confirmedTransactions = new();


        public TransactionConfirmJob(TimeSpan timeSpan)
        {
            IntervalTime = timeSpan;
        }

        public static void AddConfirmedTransaction(UnconfirmedTransactionCache.TempTransaction tx)
        {
            _confirmedTransactions.Add(tx);
        }

        public override async Task<WsMessage> Invoke()
        {
            var allConfirmTransactions = _confirmedTransactions.Select(tx => tx.Tx.Hash).ToList();
            if (allConfirmTransactions.IsEmpty())
            {
                return null;
            }

            var model = new ConfirmStateModel
            {
                Confirms = allConfirmTransactions,
            };
            if (Program.Starter.CurrentWallet != null)
            {
                var accounts = Program.Starter.CurrentWallet.GetAccounts().Select(a => a.ScriptHash).ToList();
                model.MyConfirms = _confirmedTransactions.Where(tx => tx.Transfers.Any(t => accounts.Contains(t.From) || accounts.Contains(t.To))).Select(tx => tx.Tx.Hash).ToList();
            }
            _confirmedTransactions.Clear();
            return new WsMessage()
            {
                MsgType = WsMessageType.Push,
                Method = "getLastConfirmTransactions",
                Result = model,
            };
        }
    }
}

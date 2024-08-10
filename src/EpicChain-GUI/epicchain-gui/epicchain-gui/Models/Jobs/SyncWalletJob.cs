using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading.Tasks;
using Neo.Ledger;
using Neo.Models.Wallets;
using Neo.Persistence;
using Neo.SmartContract.Native;

namespace Neo.Models.Jobs
{
    public class SyncWalletJob : Job
    {


        private uint _lastHeight = 0;
        public SyncWalletJob(TimeSpan timeSpan)
        {
            IntervalTime = timeSpan;
        }
        public override async Task<WsMessage> Invoke()
        {
            if (Program.Starter.CurrentWallet != null)
            {
                var accounts = Program.Starter.CurrentWallet.GetAccounts().ToList();

                var snapshot = this.GetDefaultSnapshot();
                if (snapshot.GetHeight() <= _lastHeight)
                {
                    return null;
                }
                var neoBalances = accounts.Select(a => a.ScriptHash).GetNativeBalanceOf(NativeContract.NEO, snapshot);
                var gasBalances = accounts.Select(a => a.ScriptHash).GetNativeBalanceOf(NativeContract.GAS, snapshot);
                var list = accounts.Select((a, i) => new AccountModel
                {
                    ScriptHash = a.ScriptHash,
                    Address = a.Address,
                    WatchOnly = a.WatchOnly,
                    AccountType = a.GetAccountType(snapshot),
                    Neo = neoBalances[i].ToString(),
                    Gas = gasBalances[i].ToString(),
                }).ToList();


                BigInteger gas = BigInteger.Zero;
                foreach (UInt160 account in accounts.Where(a => !a.WatchOnly).Select(p => p.ScriptHash))
                {
                    gas += NativeContract.NEO.UnclaimedGas(snapshot, account, snapshot.GetHeight() + 1);
                }

                var unclaimedGas = new BigDecimal(gas, NativeContract.GAS.Decimals);
                var model = new NotifyWalletBalanceModel
                {
                    UnclaimedGas = unclaimedGas,
                    Accounts = list,
                };
                _lastHeight = snapshot.GetHeight();
                return new WsMessage()
                {
                    MsgType = WsMessageType.Push,
                    Method = "getWalletBalance",
                    Result = model,
                };
            }

            return null;
        }
    }
}

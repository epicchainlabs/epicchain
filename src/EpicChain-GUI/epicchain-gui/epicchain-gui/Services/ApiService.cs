using System;
using System.Threading;
using System.Threading.Tasks;
using Neo.Common;
using Neo.Models;
using Neo.Models.Contracts;
using Neo.Network.P2P.Payloads;
using Neo.Wallets;

namespace Neo.Services
{
    public interface IApiService { }
    public abstract class ApiService : IApiService
    {
        protected Wallet CurrentWallet => Program.Starter.CurrentWallet;

        private readonly AsyncLocal<IWebSocketConnection> _asyncClient = new AsyncLocal<IWebSocketConnection>();

        public IWebSocketConnection Client
        {
            get => _asyncClient.Value;
            set => _asyncClient.Value = value;
        }


        protected WsError Error(ErrorCode code)
        {
            return code.ToError();
        }
        protected WsError Error(ErrorCode code, string message)
        {
            return new WsError() { Code = (int)code, Message = message };
        }

        protected WsError Error(int code, string message)
        {
            return new WsError() { Code = code, Message = message };
        }

        protected async Task<object> SignAndBroadcastTxWithSender(byte[] script, UInt160 sender, params UInt160[] signers)
        {
            Transaction tx;
            try
            {
                tx = CurrentWallet.InitTransaction(script, sender, signers);
            }
            catch (InvalidOperationException ex)
            {
                return Error(ErrorCode.EngineFault, ex.Message);
            }
            catch (Exception ex)
            {
                if (ex.Message.Contains("Insufficient GAS"))
                {
                    return Error(ErrorCode.GasNotEnough);
                }
                throw;
            }
            var (signSuccess, context) = CurrentWallet.TrySignTx(tx);
            if (!signSuccess)
            {
                return Error(ErrorCode.SignFail, context.SafeSerialize());
            }
            var result = new TxResultModel();
            await tx.Broadcast();
            result.TxId = tx.Hash;
            return result;
        }

        protected async Task<object> SignAndBroadcastTx(byte[] script, params UInt160[] signers)
        {
            return await SignAndBroadcastTxWithSender(script, null, signers);
        }
    }
}
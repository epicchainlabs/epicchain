using System;
using System.Threading.Tasks;
using Neo.Common.Consoles;
using Neo.Models;
using Neo.Wallets;

namespace Neo.Services.ApiServices
{
    public class VerifyApiService : ApiService
    {
        /// <summary>
        /// verify hex or wif private key
        /// </summary>
        /// <param name="privateKey"></param>
        /// <returns></returns>
        public async Task<object> VerifyPrivateKey(string privateKey)
        {
            if (privateKey.IsNull())
            {
                return Error(ErrorCode.ParameterIsNull);
            }

            var result = privateKey.TryGetPrivateKey();
            if (result.IsEmpty())
            {
                return Error(ErrorCode.InvalidPrivateKey);
            }
            return result;
        }


        /// <summary>
        /// verify nep2 private key
        /// </summary>
        /// <param name="nep2Key"></param>
        /// <param name="password"></param>
        /// <returns></returns>
        public async Task<object> VerifyNep2Key(string nep2Key, string password)
        {
            if (nep2Key.IsNull() || password.IsNull())
            {
                return Error(ErrorCode.ParameterIsNull);
            }
            try
            {
                var key = Wallet.GetPrivateKeyFromNEP2(nep2Key, password, CliSettings.Default.Protocol.AddressVersion);
                if (key.Length != 32)
                {
                    return Error(ErrorCode.InvalidPrivateKey);
                }
                return key.ToHexString();
            }
            catch (Exception ex)
            {
                return Error(ErrorCode.InvalidPrivateKey);
            }
        }
    }
}

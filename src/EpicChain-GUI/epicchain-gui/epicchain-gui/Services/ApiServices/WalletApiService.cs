using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;
using Akka.Util.Internal;
using Neo.Common.Consoles;
using Neo.Common.Storage;
using Neo.Common.Utility;
using Neo.Cryptography;
using Neo.Json;
using Neo.Models;
using Neo.Models.Transactions;
using Neo.Models.Wallets;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using Neo.Wallets;
using Neo.Wallets.NEP6;
using ECCurve = Neo.Cryptography.ECC.ECCurve;
using ECPoint = Neo.Cryptography.ECC.ECPoint;


namespace Neo.Services.ApiServices
{
    public class WalletApiService : ApiService
    {
        /// <summary>
        /// open wallet
        /// </summary>
        /// <param name="path">wallet path</param>
        /// <param name="password">wallet password</param>
        /// <returns></returns>
        public async Task<object> OpenWallet(string path, string password)
        {
            if (!File.Exists(path))
            {
                return Error(ErrorCode.WalletFileNotFound);
            }
            try
            {
                Program.Starter.OpenWallet(path, password);
            }
            catch (CryptographicException e)
            {
                return Error(ErrorCode.WrongPassword);
            }
            catch (Exception e)
            {
                return Error(ErrorCode.FailToOpenWallet);
            }
            return GetWalletAddress(CurrentWallet, int.MaxValue);
        }



        /// <summary>
        /// close wallet
        /// </summary>
        /// <returns></returns>
        public async Task<bool> CloseWallet()
        {
            Program.Starter.OnCloseWalletCommand(null);
            return true;
        }



        /// <summary>
        /// create new wallet
        /// </summary>
        /// <param name="path"></param>
        /// <param name="password"></param>
        /// <param name="privateKey"></param>
        /// <returns></returns>
        public async Task<object> CreateWallet(string path, string password, string privateKey = null)
        {
            var result = new WalletModel();
            var hexPrivateKey = privateKey.TryGetPrivateKey();
            try
            {
                switch (Path.GetExtension(path))
                {
                    //case ".db3":
                    //    {
                    //        UserWallet wallet = UserWallet.Create(path, password, CliSettings.Default.Protocol);
                    //        var account = hexPrivateKey.NotEmpty() ? wallet.CreateAccount(hexPrivateKey) : wallet.CreateAccount();
                    //        result.Accounts.Add(new AccountModel()
                    //        {
                    //            AccountType = AccountType.Standard,
                    //            Address = account.Address,
                    //            ScriptHash = account.ScriptHash,

                    //        });
                    //        Program.Starter.CurrentWallet = wallet;
                    //    }
                    //    break;
                    case ".json":
                        {
                            NEP6Wallet wallet = new NEP6Wallet(path, password, CliSettings.Default.Protocol);
                            var account = hexPrivateKey.NotEmpty() ? wallet.CreateAccount(hexPrivateKey) : wallet.CreateAccount();
                            wallet.Save();
                            result.Accounts.Add(new AccountModel()
                            {
                                AccountType = AccountType.Standard,
                                ScriptHash = account.ScriptHash,
                                Address = account.Address,
                            });
                            Program.Starter.CurrentWallet = wallet;
                        }
                        break;
                    default:
                        throw new Exception("Wallet files in that format are not supported, please use a .json or .db3 file extension.");
                }
            }
            catch (CryptographicException e)
            {
                return Error(ErrorCode.WrongPassword);
            }


            GetNeoAndGas(result.Accounts);
            return result;
        }


        public async Task<object> ChangePassword(string oldPassword, string newPassword)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            if (CurrentWallet.ChangePassword(oldPassword, newPassword))
            {
                if (CurrentWallet is NEP6Wallet wallet)
                {
                    wallet.Save();
                }
                return true;
            }
            return false;
        }

        /// <summary>
        /// create new standard address
        /// </summary>
        /// <returns></returns>
        public async Task<object> CreateAddress()
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var newAccount = CurrentWallet.CreateAccount();
            if (CurrentWallet is NEP6Wallet wallet)
            {
                wallet.Save();
            }
            return new AccountModel()
            {
                AccountType = AccountType.Standard,
                Address = newAccount.Address,
                ScriptHash = newAccount.ScriptHash,
            };
        }

        /// <summary>
        /// create new multi address
        /// </summary>
        /// <returns></returns>
        public async Task<object> CreateMultiAddress(int limit, string[] publicKeys)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            ECPoint[] points = null;
            try
            {
                points = publicKeys.Select(p => ECPoint.DecodePoint(Helper.HexToBytes(p), ECCurve.Secp256r1)).ToArray();

            }
            catch (FormatException ex)
            {
                return Error(ErrorCode.InvalidPara, ex.Message);
            }
            Contract contract = Contract.CreateMultiSigContract(limit, points);
            if (contract == null)
            {
                return Error(ErrorCode.CreateMultiContractFail);
            }
            var hashSet = new HashSet<ECPoint>(points);
            var key = CurrentWallet.GetAccounts().FirstOrDefault(p => p.HasKey && hashSet.Contains(p.GetKey().PublicKey))?.GetKey();
            var newAccount = CurrentWallet.CreateAccount(contract, key);
            if (CurrentWallet is NEP6Wallet wallet)
            {
                wallet.Save();
            }
            return new AccountModel()
            {
                AccountType = AccountType.MultiSignature,
                Address = newAccount.Address,
                ScriptHash = newAccount.ScriptHash,
            };
        }

        /// <summary>
        /// create new contract address
        /// </summary>
        /// <param name="parameterTypes"></param>
        /// <param name="script"></param>
        /// <param name="privateKey"></param>
        /// <returns></returns>
        public async Task<object> CreateContractAddress(ContractParameterType[] parameterTypes, string script, string privateKey)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            Contract contract = Contract.Create(parameterTypes, script.HexToBytes());
            if (contract == null)
            {
                return Error(ErrorCode.CreateContractAddressFail);
            }
            byte[] keyBytes = privateKey.ToPrivateKeyBytes();
            var key = new KeyPair(keyBytes);
            var newAccount = CurrentWallet.CreateAccount(contract, key);
            if (CurrentWallet is NEP6Wallet wallet)
            {
                wallet.Save();
            }
            return new AccountModel()
            {
                AccountType = AccountType.NonStandard,
                Address = newAccount.Address,
                ScriptHash = newAccount.ScriptHash,
            };
        }

        /// <summary>
        /// delete a address
        /// </summary>
        /// <returns></returns>
        public async Task<object> DeleteAddress(UInt160[] addresses)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var result = addresses.Select(scriptHash => CurrentWallet.DeleteAccount(scriptHash)).ToList();
            if (CurrentWallet is NEP6Wallet wallet)
            {
                wallet.Save();
            }
            return result;
        }

        /// <summary>
        /// list current wallet address
        /// </summary>
        /// <returns></returns>
        public async Task<object> ListAddress(int count = 100)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            return GetWalletAddress(CurrentWallet, count);
        }

        /// <summary>
        /// list current wallet address
        /// </summary>
        /// <returns></returns>
        public async Task<object> ListPublicKey(int count = 100)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var accounts = CurrentWallet.GetAccounts().Where(a => a.HasKey).Take(count).ToList();
            return accounts.Select(a => new PublicKeyModel
            {
                Address = a.Address,
                PublicKey = a.GetKey().PublicKey.EncodePoint(true),
            });
        }


        /// <summary>
        /// list current wallet candidate address(only single sign address)
        /// </summary>
        /// <returns></returns>
        public async Task<object> ListCandidatePublicKey(int count = 100)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var accounts = CurrentWallet.GetAccounts().Where(a => !a.WatchOnly && a.Contract.Script.IsSignatureContract()).Take(count).ToList();
            return accounts.Select(a => new PublicKeyModel
            {
                Address = a.Address,
                PublicKey = a.GetKey().PublicKey.EncodePoint(true),
            });
        }

        /// <summary>
        /// import watch only addresses
        /// </summary>
        /// <param name="addresses"></param>
        /// <returns></returns>
        public async Task<object> ImportWatchOnlyAddress(UInt160[] addresses)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var importedAccounts = new List<AccountModel>();
            foreach (var address in addresses)
            {
                var account = CurrentWallet.CreateAccount(address);
                importedAccounts.Add(new AccountModel
                {
                    Address = account.Address,
                    ScriptHash = account.ScriptHash,
                });
            }
            if (CurrentWallet is NEP6Wallet wallet)
            {
                wallet.Save();
            }
            GetNeoAndGas(importedAccounts);
            return importedAccounts;
        }

        /// <summary>
        /// import wif or hex private keys
        /// </summary>
        /// <param name="keys"></param>
        /// <returns></returns>
        public async Task<object> ImportAccounts(string[] keys)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            var importKeys = new List<byte[]>();
            foreach (var key in keys)
            {
                var priKey = key.TryGetPrivateKey();
                if (priKey.IsEmpty())
                {
                    return Error(ErrorCode.InvalidPrivateKey);
                }
                importKeys.Add(priKey);
            }
            var importedAccounts = new List<AccountModel>();
            foreach (var privateKey in importKeys)
            {
                var account = CurrentWallet.CreateAccount(privateKey);
                importedAccounts.Add(new AccountModel
                {
                    Address = account.Address,
                    ScriptHash = account.ScriptHash,
                });
            }
            if (CurrentWallet is NEP6Wallet wallet)
            {
                wallet.Save();
            }
            GetNeoAndGas(importedAccounts);
            return importedAccounts;
        }

        /// <summary>
        /// show private key
        /// </summary>
        /// <param name="address"></param>
        /// <returns></returns>
        public async Task<object> ShowPrivateKey(UInt160 address)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var account = CurrentWallet.GetAccount(address);
            if (account == null)
            {
                return Error(ErrorCode.AddressNotFound);
            }
            if (!account.HasKey)
            {
                return Error(ErrorCode.AddressNotFoundPrivateKey);
            }
            var key = account.GetKey();
            return new PrivateKeyModel()
            {
                ScriptHash = address,
                PublicKey = key.PublicKey.EncodePoint(true).ToHexString(),
                PrivateKey = key.PrivateKey.ToHexString(),
                Wif = key.Export(),
            };
        }

        /// <summary>
        /// show unclaimed gas amount
        /// </summary>
        /// <returns></returns>
        public async Task<object> ShowGas()
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            BigInteger gas = BigInteger.Zero;
            var snapshot = Helpers.GetDefaultSnapshot();
            foreach (UInt160 account in CurrentWallet.GetAccounts().Where(a => !a.WatchOnly).Select(p => p.ScriptHash))
            {
                gas += NativeContract.NEO.UnclaimedGas(snapshot, account, snapshot.GetHeight() + 1);
            }
            return new UnclaimedGasModel()
            {
                UnclaimedGas = new BigDecimal(gas, NativeContract.GAS.Decimals)
            };
        }

        /// <summary>
        /// show private key
        /// </summary>
        /// <returns></returns>
        public async Task<object> ClaimGas()
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var addresses = CurrentWallet.GetAccounts().Where(a => !a.Lock && !a.WatchOnly && a.Contract.Script.IsSignatureContract()).Select(a => a.ScriptHash).ToList();

            var balances = addresses.GetBalanceOf(NativeContract.NEO.Hash);
            balances = balances.Where(b => b.Value > 0).ToList();
            if (balances.IsEmpty())
            {
                return Error(ErrorCode.NoNeedClaimGas);
            }
            var outputs = balances.Select((t, index) => new TransferOutput()
            {
                AssetId = NativeContract.NEO.Hash,
                Value = t,
                ScriptHash = addresses[index],
            }).ToArray();
            try
            {
                Transaction tx = CurrentWallet.MakeTransaction(Helpers.GetDefaultSnapshot(), outputs);
                if (tx == null)
                {
                    return Error(ErrorCode.ClaimGasFail);
                }
                var (signSuccess, context) = CurrentWallet.TrySignTx(tx);
                if (!signSuccess)
                {
                    return Error(ErrorCode.SignFail, context.SafeSerialize());
                }
                await tx.Broadcast();
                return new TransactionModel(tx);
            }
            catch (Exception ex)
            {
                if (ex.Message.Contains("Insufficient GAS"))
                {
                    return Error(ErrorCode.GasNotEnough);
                }
                return Error(ErrorCode.ClaimGasFail, ex.Message);
            }
        }



        /// <summary>
        /// send asset
        /// </summary>
        /// <param name="sender"></param>
        /// <param name="receiver"></param>
        /// <param name="amount"></param>
        /// <param name="asset"></param>
        /// <returns></returns>
        public async Task<object> SendToAddress(UInt160 receiver, string amount, string asset = "neo", UInt160 sender = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            if (receiver == null)
            {
                return Error(ErrorCode.ParameterIsNull, $"receiver address is null!");
            }
            UInt160 assetHash = ConvertToAssetId(asset, out var convertError);
            if (assetHash == null)
            {
                return Error(ErrorCode.InvalidPara, $"asset is not valid:{convertError}");
            }
            var assetInfo = AssetCache.GetAssetInfo(assetHash);
            if (assetInfo == null)
            {
                return Error(ErrorCode.InvalidPara, $"asset is not valid:{convertError}");
            }
            if (!BigDecimal.TryParse(amount, assetInfo.Decimals, out BigDecimal sendAmount) || sendAmount.Sign <= 0)
            {
                return Error(ErrorCode.InvalidPara, "Incorrect Amount Format");
            }

            if (sender != null)
            {
                var account = CurrentWallet.GetAccount(sender);
                if (account == null)
                {
                    return Error(ErrorCode.AddressNotFound);
                }
                var balance = sender.GetBalanceOf(assetHash);
                if (balance.Value < sendAmount.Value)
                {
                    return Error(ErrorCode.BalanceNotEnough);
                }
            }

            try
            {
                Transaction tx = CurrentWallet.MakeTransaction(Helpers.GetDefaultSnapshot(), new[]
                {
                    new TransferOutput
                    {
                        AssetId = assetHash,
                        Value = sendAmount,
                        ScriptHash = receiver
                    }
                }, sender);

                if (tx == null)
                {
                    return Error(ErrorCode.BalanceNotEnough, "Insufficient funds");
                }

                var (signSuccess, context) = CurrentWallet.TrySignTx(tx);
                if (!signSuccess)
                {
                    return Error(ErrorCode.SignFail, context.SafeSerialize());
                }
                await tx.Broadcast();
                return new TransactionModel(tx);
            }
            catch (Exception ex)
            {
                if (ex.Message.Contains("Insufficient GAS"))
                {
                    return Error(ErrorCode.GasNotEnough);
                }
                return Error(ErrorCode.TransferError, ex.Message);
            }
        }



        /// <summary>
        /// send asset
        /// </summary>
        /// <returns></returns>
        public async Task<object> SendTo(TransferRequest[] transfers)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            if (transfers.IsEmpty())
            {
                return Error(ErrorCode.ParameterIsNull);
            }

            var transferList = new List<TransferRequestModel>();
            foreach (var transferRequest in transfers)
            {
                var (transfer, error) = ParseTransferRequest(transferRequest);
                if (error.NotNull())
                {
                    return Error(ErrorCode.InvalidPara, error);
                }
                transferList.Add(transfer);
            }

            try
            {
                var tx = MakeTransaction(transferList);
                if (tx == null)
                {
                    return Error(ErrorCode.BalanceNotEnough, "Insufficient funds");
                }
                var (signSuccess, context) = CurrentWallet.TrySignTx(tx);
                if (!signSuccess)
                {
                    return Error(ErrorCode.SignFail, context.SafeSerialize());
                }
                await tx.Broadcast();
                return new TransactionModel(tx);
            }
            catch (Exception ex)
            {
                if (ex.Message.Contains("Insufficient GAS"))
                {
                    return Error(ErrorCode.GasNotEnough);
                }

                var error = ex.Message;
                var currentEx = ex;
                while (currentEx.InnerException != null)
                {
                    currentEx = currentEx.InnerException;
                    error += $"\r\n{currentEx.Message}";
                }
                return Error(ErrorCode.TransferError, error);
            }
        }

        private (TransferRequestModel, string) ParseTransferRequest(TransferRequest request)
        {
            var model = new TransferRequestModel()
            {
                Receiver = request.Receiver,
                Sender = request.Sender,
            };
            model.Asset = ConvertToAssetId(request.Asset, out var convertError);
            if (model.Asset == null)
            {
                return (null, $"asset is not valid:{convertError}");
            }
            var assetInfo = AssetCache.GetAssetInfo(model.Asset);
            if (assetInfo == null)
            {
                return (null, $"asset is not valid:{convertError}");
            }
            if (!BigDecimal.TryParse(request.Amount, assetInfo.Decimals, out BigDecimal sendAmount) || sendAmount.Sign <= 0)
            {
                return (null, "Incorrect Amount Format");
            }
            model.Amount = sendAmount;
            return (model, null);
        }

        private Transaction MakeTransaction(List<TransferRequestModel> transfers)
        {
            var lookup = transfers.ToLookup(t => new
            {
                t.Sender,
                t.Asset
            });
            using var sb = new ScriptBuilder();
            var snapshot = Helpers.GetDefaultSnapshot();

            foreach (var transferRequests in lookup)
            {
                var sender = transferRequests.Key.Sender;
                var assetHash = transferRequests.Key.Asset;
                BigInteger amount = 0;
                transferRequests.ForEach(t => amount += t.Amount.Value);
                Console.WriteLine($"Transfer[{transferRequests.Key.Asset}]:{transferRequests.Key.Sender}=>{amount}");
                var balance = sender.GetBalanceOf(assetHash, snapshot).Value;
                if (balance < amount)
                {
                    //balance not enough
                    return null;
                }
                foreach (var transfer in transferRequests)
                {
                    sb.EmitDynamicCall(assetHash, "transfer", sender, transfer.Receiver, transfer.Amount.Value, null);
                    sb.Emit(OpCode.ASSERT);
                }
            }

            var script = sb.ToArray();
            var senders = transfers.Select(t => t.Sender).ToHashSet();
            var cosigners = senders.Select(p =>
                new Signer()
                {
                    // default access for transfers should be valid only for first invocation
                    Scopes = WitnessScope.CalledByEntry,
                    Account = p
                }).ToArray();
            return CurrentWallet.MakeTransaction(snapshot, script, null, cosigners, new TransactionAttribute[0]);
        }


        /// <summary>
        /// send asset
        /// </summary>
        /// <param name="sender"></param>
        /// <param name="receivers"></param>
        /// <param name="asset"></param>
        /// <returns></returns>
        public async Task<object> SendToMultiAddress(MultiReceiverRequest[] receivers, string asset = "neo", UInt160 sender = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            if (receivers.IsEmpty())
            {
                return Error(ErrorCode.ParameterIsNull, $"receivers is null!");
            }
            UInt160 assetHash = ConvertToAssetId(asset, out var convertError);
            if (assetHash == null)
            {
                return Error(ErrorCode.InvalidPara, $"asset is not valid:{convertError}");
            }

            var assetInfo = AssetCache.GetAssetInfo(assetHash);
            if (assetInfo == null)
            {
                return Error(ErrorCode.InvalidPara, $"asset is not valid:{convertError}");
            }
            if (sender != null)
            {
                var account = CurrentWallet.GetAccount(sender);
                if (account == null)
                {
                    return Error(ErrorCode.AddressNotFound);
                }
            }
            var toes = new List<(UInt160 scriptHash, BigDecimal amount)>();
            foreach (var receiver in receivers)
            {
                if (!BigDecimal.TryParse(receiver.Amount, assetInfo.Decimals, out BigDecimal sendAmount) || sendAmount.Sign <= 0)
                {
                    return Error(ErrorCode.InvalidPara, $"Incorrect Amount Format:{receiver.Amount}");
                }
                toes.Add((receiver.Address, sendAmount));
            }
            var outputs = toes.Select(t => new TransferOutput()
            {
                AssetId = assetHash,
                Value = t.amount,
                ScriptHash = t.scriptHash,
            }).ToArray();

            try
            {
                Transaction tx = CurrentWallet.MakeTransaction(Helpers.GetDefaultSnapshot(), outputs, sender);
                if (tx == null)
                {
                    return Error(ErrorCode.BalanceNotEnough, "Insufficient funds");
                }

                var (signSuccess, context) = CurrentWallet.TrySignTx(tx);
                if (!signSuccess)
                {
                    return Error(ErrorCode.SignFail, context.SafeSerialize());
                }
                await tx.Broadcast();
                return new TransactionModel(tx);
            }
            catch (Exception ex)
            {
                if (ex.Message.Contains("Insufficient GAS"))
                {
                    return Error(ErrorCode.GasNotEnough);
                }
                return Error(ErrorCode.TransferError, ex.Message);
            }
        }


        /// <summary>
        /// append signature for multi address tx
        /// </summary>
        /// <param name="signContext"></param>
        /// <returns></returns>
        public async Task<object> AppendSignature(string signContext)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            ContractParametersContext context;
            try
            {
                context = ContractParametersContext.FromJson(signContext.DeserializeJson<JObject>(), Helpers.GetDefaultSnapshot());
            }
            catch (Exception e)
            {
                return Error(ErrorCode.InvalidPara);
            }

            if (CurrentWallet.SignContext(context))
            {
                return context.SafeSerialize();
            }
            return Error(ErrorCode.SignFail, context.SafeSerialize());
        }


        /// <summary>
        /// append signature for text
        /// </summary>
        /// <param name="text"></param>
        /// <param name="address"></param>
        /// <returns></returns>
        public async Task<object> AppendTextSignature(string text, UInt160 address)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }
            var account = CurrentWallet.GetAccount(address);
            if (!account.HasKey)
            {
                return Error(ErrorCode.AddressNotFoundPrivateKey);
            }
            try
            {
                KeyPair key = account.GetKey();
                byte[] signature = Crypto.Sign(Encoding.UTF8.GetBytes(text), key.PrivateKey, ECCurve.Secp256r1);
                return signature;
            }
            catch (Exception e)
            {
                return Error(ErrorCode.InvalidPara, e.ToString());
            }
        }
        /// <summary>
        /// Broadcast complete signed transaction
        /// </summary>
        /// <param name="signContext"></param>
        /// <returns></returns>
        public async Task<object> BroadcastTransaction(string signContext)
        {
            Transaction transaction = null;
            try
            {
                ContractParametersContext context = ContractParametersContext.FromJson(signContext.DeserializeJson<JObject>(), Helpers.GetDefaultSnapshot());
                if (!context.Completed)
                {
                    return Error(ErrorCode.SignFail, signContext);
                }
                transaction = (Transaction)context.Verifiable;
                transaction.Witnesses = context.GetWitnesses();
            }
            catch (Exception e)
            {
                return Error(ErrorCode.InvalidPara);
            }
            await transaction.Broadcast();
            return transaction.Hash;
        }


        /// <summary>
        /// get my wallet unconfirmed transactions
        /// </summary>
        /// <returns></returns>
        public async Task<object> GetMyUnconfirmedTransactions(int pageIndex = 1, int limit = 100)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            var addresses = CurrentWallet.GetAccounts().Select(a => a.ScriptHash).ToList();
            if (addresses.IsEmpty())
            {
                return new PageList<TransactionPreviewModel>();
            }
            var tempTransactions = UnconfirmedTransactionCache.GetUnconfirmedTransactions(addresses, pageIndex, limit);
            var result = tempTransactions.Project(t => t.ToTransactionPreviewModel());
            return result;
        }

        /// <summary>
        /// query relate my wallet transactions(on chain)
        /// </summary>
        /// <returns></returns>
        public async Task<object> GetMyTransactions(int pageIndex = 1, int limit = 100, UInt160 address = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            var addresses = address != null ? new List<UInt160>() { address } : CurrentWallet.GetAccounts().Select(a => a.ScriptHash).ToList();
            if (addresses.IsEmpty())
            {
                return new PageList<TransactionPreviewModel>();
            }
            using var db = new TrackDB();
            var trans = db.QueryTransfersPagedByTx(new TransferFilter() { FromOrTo = addresses, PageIndex = pageIndex, PageSize = limit });
            var result = new PageList<TransactionPreviewModel>
            {
                TotalCount = trans.TotalCount,
                PageSize = trans.PageSize,
                PageIndex = pageIndex,
                List = trans.List?.ToTransactionPreviewModel(),
            };
            return result;
        }

        /// <summary>
        /// query relate my wallet balances
        /// </summary>
        /// <returns></returns>
        public async Task<object> GetMyBalances(UInt160 address = null, UInt160[] assets = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            var addresses = CurrentWallet.GetAccounts().Select(a => a.ScriptHash).ToList();
            if (address != null)
            {
                if (!addresses.Contains(address))
                {
                    return Error(ErrorCode.AddressNotFound);
                }
                addresses = new List<UInt160>() { address };
            }

            if (addresses.IsEmpty())
            {
                return new List<AddressBalanceModel>();
            }
            using var db = new TrackDB();
            var balances = db.FindAssetBalance(new BalanceFilter() { Addresses = addresses, Assets = assets });

            var result = balances.ToLookup(b => b.Address).ToAddressBalanceModels();
            if (assets.IsEmpty())
            {
                AppendDefaultNeoAndGas(result, addresses);
            }
            return result;
        }



        /// <summary>
        /// query relate my wallet balances
        /// </summary>
        /// <returns></returns>
        public async Task<object> GetMyTotalBalance(UInt160[] assets = null)
        {
            if (CurrentWallet == null)
            {
                return Error(ErrorCode.WalletNotOpen);
            }

            var addresses = CurrentWallet.GetAccounts().Select(a => a.ScriptHash).ToList();
            if (addresses.IsEmpty())
            {
                return new List<AddressBalanceModel>();
            }
            using var db = new TrackDB();
            var balances = db.FindAssetBalance(new BalanceFilter() { Addresses = addresses, Assets = assets });

            var result = balances.GroupBy(b => new { b.Asset, b.AssetDecimals, b.AssetSymbol }).Select(g => new AssetBalanceModel
            {
                Asset = g.Key.Asset,
                Symbol = g.Key.AssetSymbol,
                Balance = new BigDecimal(g.Select(b => b.Balance).Sum(), g.Key.AssetDecimals)
            }).ToList();
            if (assets.IsEmpty())
            {
                AppendDefaultNeoAndGas(result);
            }
            return result;
        }



        #region Private


        /// <summary>
        /// add neo or gas balance if not found
        /// </summary>
        /// <param name="list"></param>
        private void AppendDefaultNeoAndGas(IList<AssetBalanceModel> list)
        {
            bool hasNeo = false;
            bool hasGas = false;
            foreach (var assetBalanceModel in list)
            {
                hasNeo ^= assetBalanceModel.Asset == NativeContract.NEO.Hash;
                hasGas ^= assetBalanceModel.Asset == NativeContract.GAS.Hash;
                if (hasNeo && hasGas)
                {
                    break;
                }
            }

            if (!hasGas)
            {
                list.Insert(0, _defaultGasBalance);
            }

            if (!hasNeo)
            {
                list.Insert(0, _defaultNeoBalance);
            }
        }


        /// <summary>
        /// add neo or gas balance if not found
        /// </summary>
        /// <param name="list"></param>
        private void AppendDefaultNeoAndGas(List<AddressBalanceModel> list, List<UInt160> addresses)
        {
            var lookup = list.ToLookup(l => l.AddressHash);
            var unfoundAddreses = addresses.Except(lookup.Select(l => l.Key)).ToList();
            foreach (var address in unfoundAddreses)
            {
                list.Add(new AddressBalanceModel()
                {
                    AddressHash = address,
                    Balances = new List<AssetBalanceModel>() { _defaultNeoBalance, _defaultGasBalance }
                });
            }
            foreach (var addressBalanceModel in list)
            {
                if (addressBalanceModel.Balances.All(b => b.Asset != NativeContract.NEO.Hash))
                {
                    addressBalanceModel.Balances.Add(_defaultNeoBalance);
                }
                if (addressBalanceModel.Balances.All(b => b.Asset != NativeContract.GAS.Hash))
                {
                    addressBalanceModel.Balances.Add(_defaultGasBalance);
                }
            }
        }

        private AssetBalanceModel _defaultNeoBalance = new AssetBalanceModel()
        {
            Asset = NativeContract.NEO.Hash,
            Symbol = NativeContract.NEO.Symbol.ToUpper(),
            Balance = new BigInteger(0).ToNeo()
        };

        private AssetBalanceModel _defaultGasBalance = new AssetBalanceModel()
        {
            Asset = NativeContract.GAS.Hash,
            Symbol = NativeContract.GAS.Symbol.ToUpper(),
            Balance = new BigInteger(0).ToGas()
        };



        /// <summary>
        /// convert input address string to address hash
        /// </summary>
        /// <param name="address"></param>
        /// <param name="error"></param>
        /// <returns></returns>
        private UInt160 ConvertToAddress(string address, out string error)
        {
            error = "";
            try
            {
                return address.ToScriptHash();
            }
            catch (Exception e)
            {
                error = e.Message;
                return null;
            }
        }

        /// <summary>
        /// convert input asset string to asset hash
        /// </summary>
        /// <param name="asset"></param>
        /// <returns></returns>
        private UInt160 ConvertToAssetId(string asset, out string error)
        {
            error = "";
            if ("neo".Equals(asset, StringComparison.OrdinalIgnoreCase))
            {
                return NativeContract.NEO.Hash;
            }
            if ("gas".Equals(asset, StringComparison.OrdinalIgnoreCase))
            {
                return NativeContract.GAS.Hash;
            }
            try
            {
                return UInt160.Parse(asset);
            }
            catch (Exception e)
            {
                error = e.Message;
                return null;
            }
        }

        /// <summary>
        /// get all address info from wallet
        /// </summary>
        /// <param name="wallet"></param>
        /// <returns></returns>
        private WalletModel GetWalletAddress(Wallet wallet, int count)
        {
            var result = new WalletModel();
            var snapshot = Helpers.GetDefaultSnapshot();

            result.Accounts.AddRange(wallet.GetAccounts().Take(count).Select(account => new AccountModel()
            {
                ScriptHash = account.ScriptHash,
                Address = account.Address,
                WatchOnly = account.WatchOnly,
                AccountType = account.GetAccountType(snapshot),
            }));

            GetNeoAndGas(result.Accounts);
            return result;
        }




        private List<AccountModel> GetNeoAndGas(IEnumerable<AccountModel> accounts)
        {
            var addresses = accounts.Select(t => t.ScriptHash).ToList();
            var neos = addresses.GetBalanceOf(NativeContract.NEO.Hash);
            var gases = addresses.GetBalanceOf(NativeContract.GAS.Hash);

            return accounts.Select((account, index) =>
            {
                account.Neo = neos[index].ToString();
                account.Gas = gases[index].ToString();
                return account;

            }).ToList();
        }

        #endregion


    }
}

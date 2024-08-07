// 
// Copyright (C) 2021-2024 EpicChain Lab's
// All rights reserved.
// 
// This file is part of the EpicChain project, developed by xmoohad.
// 
// This file is subject to the terms and conditions defined in the LICENSE file found in the top-level 
// directory of this distribution. Unauthorized copying, modification, or distribution of this file,
// via any medium, is strictly prohibited. Any use of this file without explicit permission from EpicChain Lab's
// is a violation of copyright law and will be prosecuted to the fullest extent possible.
// 
// This file is licensed under the MIT License; you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://opensource.org/licenses/MIT
// 
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed 
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for 
// the specific language governing permissions and limitations under the License.
// 
// For more information about EpicChain Lab's projects and innovations, visit our website at https://epic-chain.org
// or contact us at xmoohad@epic-chain.org.
// 
//

using Akka.Actor;
using Neo.ConsoleService;
using Neo.Cryptography.ECC;
using Neo.Json;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using Neo.Wallets;
using Neo.Wallets.NEP6;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;
using static Neo.SmartContract.Helper;

namespace Neo.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "open wallet" command
        /// </summary>
        /// <param name="path">Path</param>
        [ConsoleCommand("open wallet", Category = "Wallet Commands")]
        private void OnOpenWallet(string path)
        {
            if (!File.Exists(path))
            {
                ConsoleHelper.Error("File does not exist");
                return;
            }
            string password = ConsoleHelper.ReadUserInput("password", true);
            if (password.Length == 0)
            {
                ConsoleHelper.Info("Cancelled");
                return;
            }
            try
            {
                OpenWallet(path, password);
            }
            catch (System.Security.Cryptography.CryptographicException)
            {
                ConsoleHelper.Error($"Failed to open file \"{path}\"");
            }
        }

        /// <summary>
        /// Process "close wallet" command
        /// </summary>
        [ConsoleCommand("close wallet", Category = "Wallet Commands")]
        private void OnCloseWalletCommand()
        {
            if (NoWallet()) return;
            CurrentWallet = null;
            ConsoleHelper.Info("Wallet is closed");
        }

        /// <summary>
        /// Process "upgrade wallet" command
        /// </summary>
        [ConsoleCommand("upgrade wallet", Category = "Wallet Commands")]
        private void OnUpgradeWalletCommand(string path)
        {
            if (Path.GetExtension(path).ToLowerInvariant() != ".db3")
            {
                ConsoleHelper.Warning("Can't upgrade the wallet file. Check if your wallet is in db3 format.");
                return;
            }
            if (!File.Exists(path))
            {
                ConsoleHelper.Error("File does not exist.");
                return;
            }
            string password = ConsoleHelper.ReadUserInput("password", true);
            if (password.Length == 0)
            {
                ConsoleHelper.Info("Cancelled");
                return;
            }
            string pathNew = Path.ChangeExtension(path, ".json");
            if (File.Exists(pathNew))
            {
                ConsoleHelper.Warning($"File '{pathNew}' already exists");
                return;
            }
            NEP6Wallet.Migrate(pathNew, path, password, NeoSystem.Settings).Save();
            Console.WriteLine($"Wallet file upgrade complete. New wallet file has been auto-saved at: {pathNew}");
        }

        /// <summary>
        /// Process "create address" command
        /// </summary>
        /// <param name="count">Count</param>
        [ConsoleCommand("generate address", Category = "Wallet Commands")]
        private void OnCreateAddressCommand(ushort count = 1)
        {
            if (NoWallet()) return;
            string path = "epicchain-address.txt";
            if (File.Exists(path))
            {
                if (!ConsoleHelper.ReadUserInput($"The file '{path}' already exists, do you want to overwrite it? (yes|no)", false).IsYes())
                {
                    return;
                }
            }

            List<string> addresses = new List<string>();
            using (var percent = new ConsolePercent(0, count))
            {
                Parallel.For(0, count, (i) =>
                {
                    WalletAccount account = CurrentWallet!.CreateAccount();
                    lock (addresses)
                    {
                        addresses.Add(account.Address);
                        percent.Value++;
                    }
                });
            }

            if (CurrentWallet is NEP6Wallet wallet)
                wallet.Save();

            Console.WriteLine($"Export addresses to {path}");
            File.WriteAllLines(path, addresses);
        }

        /// <summary>
        /// Process "delete address" command
        /// </summary>
        /// <param name="address">Address</param>
        [ConsoleCommand("delete address", Category = "Wallet Commands")]
        private void OnDeleteAddressCommand(UInt160 address)
        {
            if (NoWallet()) return;

            if (ConsoleHelper.ReadUserInput($"Warning: Irrevocable operation!\nAre you sure to delete account {address.ToAddress(NeoSystem.Settings.AddressVersion)}? (no|yes)").IsYes())
            {
                if (CurrentWallet!.DeleteAccount(address))
                {
                    if (CurrentWallet is NEP6Wallet wallet)
                    {
                        wallet.Save();
                    }
                    ConsoleHelper.Info($"Address {address} deleted.");
                }
                else
                {
                    ConsoleHelper.Warning($"Address {address} doesn't exist.");
                }
            }
        }

        /// <summary>
        /// Process "export key" command
        /// </summary>
        /// <param name="path">Path</param>
        /// <param name="scriptHash">ScriptHash</param>
        [ConsoleCommand("export key", Category = "Wallet Commands")]
        private void OnExportKeyCommand(string? path = null, UInt160? scriptHash = null)
        {
            if (NoWallet()) return;
            if (path != null && File.Exists(path))
            {
                ConsoleHelper.Error($"File '{path}' already exists");
                return;
            }
            string password = ConsoleHelper.ReadUserInput("password", true);
            if (password.Length == 0)
            {
                ConsoleHelper.Info("Cancelled");
                return;
            }
            if (!CurrentWallet!.VerifyPassword(password))
            {
                ConsoleHelper.Error("Incorrect password");
                return;
            }
            IEnumerable<KeyPair> keys;
            if (scriptHash == null)
                keys = CurrentWallet.GetAccounts().Where(p => p.HasKey).Select(p => p.GetKey());
            else
            {
                var account = CurrentWallet.GetAccount(scriptHash);
                keys = account?.HasKey != true ? Array.Empty<KeyPair>() : new[] { account.GetKey() };
            }
            if (path == null)
                foreach (KeyPair key in keys)
                    Console.WriteLine(key.Export());
            else
                File.WriteAllLines(path, keys.Select(p => p.Export()));
        }

        /// <summary>
        /// Process "create wallet" command
        /// </summary>
        [ConsoleCommand("create wallet", Category = "Wallet Commands")]
        private void OnCreateWalletCommand(string path, string? wifOrFile = null)
        {
            string password = ConsoleHelper.ReadUserInput("password", true);
            if (password.Length == 0)
            {
                ConsoleHelper.Info("Cancelled");
                return;
            }
            string password2 = ConsoleHelper.ReadUserInput("repeat password", true);
            if (password != password2)
            {
                ConsoleHelper.Error("Two passwords not match.");
                return;
            }
            if (File.Exists(path))
            {
                Console.WriteLine("This wallet already exists, please create another one.");
                return;
            }
            bool createDefaultAccount = wifOrFile is null;
            CreateWallet(path, password, createDefaultAccount);
            if (!createDefaultAccount) OnImportKeyCommand(wifOrFile!);
        }

        /// <summary>
        /// Process "import multisigaddress" command
        /// </summary>
        /// <param name="m">Required signatures</param>
        /// <param name="publicKeys">Public keys</param>
        [ConsoleCommand("import multisigaddress", Category = "Wallet Commands")]
        private void OnImportMultisigAddress(ushort m, ECPoint[] publicKeys)
        {
            if (NoWallet()) return;
            int n = publicKeys.Length;

            if (m < 1 || m > n || n > 1024)
            {
                ConsoleHelper.Error("Invalid parameters.");
                return;
            }

            Contract multiSignContract = Contract.CreateMultiSigContract(m, publicKeys);
            KeyPair? keyPair = CurrentWallet!.GetAccounts().FirstOrDefault(p => p.HasKey && publicKeys.Contains(p.GetKey().PublicKey))?.GetKey();

            CurrentWallet.CreateAccount(multiSignContract, keyPair);
            if (CurrentWallet is NEP6Wallet wallet)
                wallet.Save();

            ConsoleHelper.Info("Multisig. Addr.: ", multiSignContract.ScriptHash.ToAddress(NeoSystem.Settings.AddressVersion));
        }

        /// <summary>
        /// Process "import key" command
        /// </summary>
        [ConsoleCommand("import key", Category = "Wallet Commands")]
        private void OnImportKeyCommand(string wifOrFile)
        {
            if (NoWallet()) return;
            byte[]? prikey = null;
            try
            {
                prikey = Wallet.GetPrivateKeyFromWIF(wifOrFile);
            }
            catch (FormatException) { }
            if (prikey == null)
            {
                var fileInfo = new FileInfo(wifOrFile);

                if (!fileInfo.Exists)
                {
                    ConsoleHelper.Error($"File '{fileInfo.FullName}' doesn't exists");
                    return;
                }

                if (wifOrFile.Length > 1024 * 1024)
                {
                    if (!ConsoleHelper.ReadUserInput($"The file '{fileInfo.FullName}' is too big, do you want to continue? (yes|no)", false).IsYes())
                    {
                        return;
                    }
                }

                string[] lines = File.ReadAllLines(fileInfo.FullName).Where(u => !string.IsNullOrEmpty(u)).ToArray();
                using (var percent = new ConsolePercent(0, lines.Length))
                {
                    for (int i = 0; i < lines.Length; i++)
                    {
                        if (lines[i].Length == 64)
                            prikey = lines[i].HexToBytes();
                        else
                            prikey = Wallet.GetPrivateKeyFromWIF(lines[i]);
                        CurrentWallet!.CreateAccount(prikey);
                        Array.Clear(prikey, 0, prikey.Length);
                        percent.Value++;
                    }
                }
            }
            else
            {
                WalletAccount account = CurrentWallet!.CreateAccount(prikey);
                Array.Clear(prikey, 0, prikey.Length);
                ConsoleHelper.Info("Address: ", account.Address);
                ConsoleHelper.Info(" Pubkey: ", account.GetKey().PublicKey.EncodePoint(true).ToHexString());
            }
            if (CurrentWallet is NEP6Wallet wallet)
                wallet.Save();
        }

        /// <summary>
        /// Process "import watchonly" command
        /// </summary>
        [ConsoleCommand("import watchonly", Category = "Wallet Commands")]
        private void OnImportWatchOnlyCommand(string addressOrFile)
        {
            if (NoWallet()) return;
            UInt160? address = null;
            try
            {
                address = StringToAddress(addressOrFile, NeoSystem.Settings.AddressVersion);
            }
            catch (FormatException) { }
            if (address is null)
            {
                var fileInfo = new FileInfo(addressOrFile);

                if (!fileInfo.Exists)
                {
                    ConsoleHelper.Warning($"File '{fileInfo.FullName}' doesn't exists");
                    return;
                }

                if (fileInfo.Length > 1024 * 1024)
                {
                    if (!ConsoleHelper.ReadUserInput($"The file '{fileInfo.FullName}' is too big, do you want to continue? (yes|no)", false).IsYes())
                    {
                        return;
                    }
                }

                string[] lines = File.ReadAllLines(fileInfo.FullName).Where(u => !string.IsNullOrEmpty(u)).ToArray();
                using (var percent = new ConsolePercent(0, lines.Length))
                {
                    for (int i = 0; i < lines.Length; i++)
                    {
                        address = StringToAddress(lines[i], NeoSystem.Settings.AddressVersion);
                        CurrentWallet!.CreateAccount(address);
                        percent.Value++;
                    }
                }
            }
            else
            {
                WalletAccount account = CurrentWallet!.GetAccount(address);
                if (account is not null)
                {
                    ConsoleHelper.Warning("This address is already in your wallet");
                }
                else
                {
                    account = CurrentWallet.CreateAccount(address);
                    ConsoleHelper.Info("Address: ", account.Address);
                }
            }
            if (CurrentWallet is NEP6Wallet wallet)
                wallet.Save();
        }

        /// <summary>
        /// Process "list address" command
        /// </summary>
        [ConsoleCommand("show address", Category = "Wallet Commands")]
        private void OnListAddressCommand()
        {
            if (NoWallet()) return;
            var snapshot = NeoSystem.StoreView;
            foreach (var account in CurrentWallet!.GetAccounts())
            {
                var contract = account.Contract;
                var type = "Nonstandard";

                if (account.WatchOnly)
                {
                    type = "WatchOnly";
                }
                else if (IsMultiSigContract(contract.Script))
                {
                    type = "MultiSignature";
                }
                else if (IsSignatureContract(contract.Script))
                {
                    type = "Standard";
                }
                else if (NativeContract.ContractManagement.GetContract(snapshot, account.ScriptHash) != null)
                {
                    type = "Deployed-Nonstandard";
                }

                ConsoleHelper.Info("   Address: ", $"{account.Address}\t{type}");
                ConsoleHelper.Info("ScriptHash: ", $"{account.ScriptHash}\n");
            }
        }

        /// <summary>
        /// Process "list asset" command
        /// </summary>
        [ConsoleCommand("show balance", Category = "Wallet Commands")]
        private void OnListAssetCommand()
        {
            var snapshot = NeoSystem.StoreView;
            if (NoWallet()) return;
            foreach (UInt160 account in CurrentWallet!.GetAccounts().Select(p => p.ScriptHash))
            {
                Console.WriteLine();
                Console.WriteLine();
                Console.WriteLine(account.ToAddress(NeoSystem.Settings.AddressVersion));
                Console.WriteLine();
                Console.WriteLine();
                ConsoleHelper.Info("EpicChain: ", $"{CurrentWallet.GetBalance(snapshot, NativeContract.NEO.Hash, account)}");
                ConsoleHelper.Info("EpicPulse: ", $"{CurrentWallet.GetBalance(snapshot, NativeContract.GAS.Hash, account)}");
                Console.WriteLine();
            }
            Console.WriteLine();
            Console.WriteLine();
            Console.WriteLine("----------------------------------------------------");
            Console.WriteLine("----------------------------------------------------");
            Console.WriteLine();
            ConsoleHelper.Info("EpicChain hash: ", NativeContract.NEO.Hash.ToString());
            ConsoleHelper.Info("EpicPulse hash: ", NativeContract.GAS.Hash.ToString());
            Console.WriteLine();
            Console.WriteLine();
            Console.WriteLine("----------------------------------------------------");
            ConsoleHelper.Info("Total:   EpicChain: ", $"{CurrentWallet.GetAvailable(snapshot, NativeContract.NEO.Hash),10}     ", "EpicPulse: ", $"{CurrentWallet.GetAvailable(snapshot, NativeContract.GAS.Hash),18}");
            Console.WriteLine();
        }

        /// <summary>
        /// Process "list key" command
        /// </summary>
        [ConsoleCommand("show key", Category = "Wallet Commands")]
        private void OnListKeyCommand()
        {
            if (NoWallet()) return;
            foreach (WalletAccount account in CurrentWallet!.GetAccounts().Where(p => p.HasKey))
            {
                ConsoleHelper.Info("   Address: ", account.Address);
                ConsoleHelper.Info("ScriptHash: ", account.ScriptHash.ToString());
                ConsoleHelper.Info(" PublicKey: ", account.GetKey().PublicKey.EncodePoint(true).ToHexString());
                Console.WriteLine();
            }
        }

        /// <summary>
        /// Process "sign" command
        /// </summary>
        /// <param name="jsonObjectToSign">Json object to sign</param>
        [ConsoleCommand("sign", Category = "Wallet Commands")]
        private void OnSignCommand(JObject jsonObjectToSign)
        {
            if (NoWallet()) return;

            if (jsonObjectToSign == null)
            {
                ConsoleHelper.Warning("You must input JSON object pending signature data.");
                return;
            }
            try
            {
                var snapshot = NeoSystem.StoreView;
                ContractParametersContext context = ContractParametersContext.Parse(jsonObjectToSign.ToString(), snapshot);
                if (context.Network != NeoSystem.Settings.Network)
                {
                    ConsoleHelper.Warning("Network mismatch.");
                    return;
                }
                else if (!CurrentWallet!.Sign(context))
                {
                    ConsoleHelper.Warning("Non-existent private key in wallet.");
                    return;
                }
                ConsoleHelper.Info("Signed Output: ", $"{Environment.NewLine}{context}");
            }
            catch (Exception e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
            }
        }

        /// <summary>
        /// Process "send" command
        /// </summary>
        /// <param name="asset">Asset id</param>
        /// <param name="to">To</param>
        /// <param name="amount">Amount</param>
        /// <param name="from">From</param>
        /// <param name="data">Data</param>
        /// <param name="signerAccounts">Signer's accounts</param>
        [ConsoleCommand("send", Category = "Wallet Commands")]
        private void OnSendCommand(UInt160 asset, UInt160 to, string amount, UInt160? from = null, string? data = null, UInt160[]? signerAccounts = null)
        {
            if (NoWallet()) return;
            string password = ConsoleHelper.ReadUserInput("password", true);
            if (password.Length == 0)
            {
                ConsoleHelper.Info("Cancelled");
                return;
            }
            if (!CurrentWallet!.VerifyPassword(password))
            {
                ConsoleHelper.Error("Incorrect password");
                return;
            }

            var snapshot = NeoSystem.StoreView;
            Transaction tx;
            AssetDescriptor descriptor = new(snapshot, NeoSystem.Settings, asset);
            if (!BigDecimal.TryParse(amount, descriptor.Decimals, out BigDecimal decimalAmount) || decimalAmount.Sign <= 0)
            {
                ConsoleHelper.Error("Incorrect Amount Format");
                return;
            }
            try
            {
                tx = CurrentWallet.MakeTransaction(snapshot, new[]
                {
                    new TransferOutput
                    {
                        AssetId = asset,
                        Value = decimalAmount,
                        ScriptHash = to,
                        Data = data
                    }
                }, from: from, cosigners: signerAccounts?.Select(p => new Signer
                {
                    // default access for transfers should be valid only for first invocation
                    Scopes = WitnessScope.CalledByEntry,
                    Account = p
                })
                .ToArray() ?? Array.Empty<Signer>());
            }
            catch (Exception e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
                return;
            }

            if (tx == null)
            {
                ConsoleHelper.Warning("Insufficient funds");
                return;
            }

            ConsoleHelper.Info(
                "Send To: ", $"{to.ToAddress(NeoSystem.Settings.AddressVersion)}\n",
                "Network fee: ", $"{new BigDecimal((BigInteger)tx.NetworkFee, NativeContract.GAS.Decimals)}\t",
                "Total fee: ", $"{new BigDecimal((BigInteger)(tx.SystemFee + tx.NetworkFee), NativeContract.GAS.Decimals)} EpicPulse");
            if (!ConsoleHelper.ReadUserInput("Relay Transaction? (no|yes)").IsYes())
            {
                return;
            }
            SignAndSendTx(NeoSystem.StoreView, tx);
        }

        /// <summary>
        /// Process "cancel" command
        /// </summary>
        /// <param name="txid">conflict txid</param>
        /// <param name="sender">Transaction's sender</param>
        /// <param name="signerAccounts">Signer's accounts</param>
        [ConsoleCommand("cancel", Category = "Wallet Commands")]
        private void OnCancelCommand(UInt256 txid, UInt160? sender = null, UInt160[]? signerAccounts = null)
        {
            if (NoWallet()) return;

            TransactionState state = NativeContract.Ledger.GetTransactionState(NeoSystem.StoreView, txid);
            if (state != null)
            {
                ConsoleHelper.Error("This tx is already confirmed, can't be cancelled.");
                return;
            }

            var conflict = new TransactionAttribute[] { new Conflicts() { Hash = txid } };
            Signer[] signers = Array.Empty<Signer>();
            if (sender != null)
            {
                if (signerAccounts == null)
                    signerAccounts = new UInt160[1] { sender };
                else if (signerAccounts.Contains(sender) && signerAccounts[0] != sender)
                {
                    var signersList = signerAccounts.ToList();
                    signersList.Remove(sender);
                    signerAccounts = signersList.Prepend(sender).ToArray();
                }
                else if (!signerAccounts.Contains(sender))
                {
                    signerAccounts = signerAccounts.Prepend(sender).ToArray();
                }
                signers = signerAccounts.Select(p => new Signer() { Account = p, Scopes = WitnessScope.None }).ToArray();
            }

            Transaction tx = new()
            {
                Signers = signers,
                Attributes = conflict,
                Witnesses = Array.Empty<Witness>(),
            };

            try
            {
                using ScriptBuilder scriptBuilder = new();
                scriptBuilder.Emit(OpCode.RET);
                tx = CurrentWallet!.MakeTransaction(NeoSystem.StoreView, scriptBuilder.ToArray(), sender, signers, conflict);
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
                return;
            }

            if (NeoSystem.MemPool.TryGetValue(txid, out Transaction conflictTx))
            {
                tx.NetworkFee = Math.Max(tx.NetworkFee, conflictTx.NetworkFee) + 1;
            }
            else
            {
                var snapshot = NeoSystem.StoreView;
                AssetDescriptor descriptor = new(snapshot, NeoSystem.Settings, NativeContract.GAS.Hash);
                string extracFee = ConsoleHelper.ReadUserInput("This tx is not in mempool, please input extra fee (datoshi) manually");
                if (!BigDecimal.TryParse(extracFee, descriptor.Decimals, out BigDecimal decimalExtraFee) || decimalExtraFee.Sign <= 0)
                {
                    ConsoleHelper.Error("Incorrect Amount Format");
                    return;
                }
                tx.NetworkFee += (long)decimalExtraFee.Value;
            };

            ConsoleHelper.Info("Network fee: ",
                $"{new BigDecimal((BigInteger)tx.NetworkFee, NativeContract.GAS.Decimals)} EpicPulse\t",
                "Total fee: ",
                $"{new BigDecimal((BigInteger)(tx.SystemFee + tx.NetworkFee), NativeContract.GAS.Decimals)} EpicPulse");
            if (!ConsoleHelper.ReadUserInput("Relay Transaction? (no|yes)").IsYes())
            {
                return;
            }
            SignAndSendTx(NeoSystem.StoreView, tx);
        }

        /// <summary>
        /// Process "show gas" command
        /// </summary>
        [ConsoleCommand("show epicpulse", Category = "Wallet Commands")]
        private void OnShowGasCommand()
        {
            if (NoWallet()) return;
            BigInteger gas = BigInteger.Zero;
            var snapshot = NeoSystem.StoreView;
            uint height = NativeContract.Ledger.CurrentIndex(snapshot) + 1;
            foreach (UInt160 account in CurrentWallet!.GetAccounts().Select(p => p.ScriptHash))
                gas += NativeContract.NEO.UnclaimedGas(snapshot, account, height);
            ConsoleHelper.Info("Unclaimed EpicPulse: ", new BigDecimal(gas, NativeContract.GAS.Decimals).ToString());
        }

        /// <summary>
        /// Process "change password" command
        /// </summary>
        [ConsoleCommand("change password", Category = "Wallet Commands")]
        private void OnChangePasswordCommand()
        {
            if (NoWallet()) return;
            string oldPassword = ConsoleHelper.ReadUserInput("password", true);
            if (oldPassword.Length == 0)
            {
                ConsoleHelper.Info("Cancelled");
                return;
            }
            if (!CurrentWallet!.VerifyPassword(oldPassword))
            {
                ConsoleHelper.Error("Incorrect password");
                return;
            }
            string newPassword = ConsoleHelper.ReadUserInput("New password", true);
            string newPasswordReEntered = ConsoleHelper.ReadUserInput("Re-Enter Password", true);
            if (!newPassword.Equals(newPasswordReEntered))
            {
                ConsoleHelper.Error("Two passwords entered are inconsistent!");
                return;
            }

            if (CurrentWallet is NEP6Wallet wallet)
            {
                string backupFile = wallet.Path + ".bak";
                if (!File.Exists(wallet.Path) || File.Exists(backupFile))
                {
                    ConsoleHelper.Error("Wallet backup fail");
                    return;
                }
                try
                {
                    File.Copy(wallet.Path, backupFile);
                }
                catch (IOException)
                {
                    ConsoleHelper.Error("Wallet backup fail");
                    return;
                }
            }

            bool succeed = CurrentWallet.ChangePassword(oldPassword, newPassword);
            if (succeed)
            {
                if (CurrentWallet is NEP6Wallet nep6Wallet)
                    nep6Wallet.Save();
                Console.WriteLine("Password changed successfully");
            }
            else
            {
                ConsoleHelper.Error("Failed to change password");
            }
        }

        private void SignAndSendTx(DataCache snapshot, Transaction tx)
        {
            if (NoWallet()) return;

            ContractParametersContext context;
            try
            {
                context = new ContractParametersContext(snapshot, tx, NeoSystem.Settings.Network);
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error("Failed creating contract params: " + GetExceptionMessage(e));
                throw;
            }
            CurrentWallet!.Sign(context);
            if (context.Completed)
            {
                tx.Witnesses = context.GetWitnesses();
                NeoSystem.Blockchain.Tell(tx);
                ConsoleHelper.Info("Signed and relayed transaction with hash:\n", $"{tx.Hash}");
            }
            else
            {
                ConsoleHelper.Info("Incomplete signature:\n", $"{context}");
            }
        }
    }
}

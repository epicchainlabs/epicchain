// Copyright (C) 2021-2024 EpicChain Labs.

//
// XEP6Wallet.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


using EpicChain.Extensions;
using EpicChain.Json;
using EpicChain.SmartContract;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Security;
using System.Security.Cryptography;
using System.Security.Cryptography.X509Certificates;
using System.Threading.Tasks;

namespace EpicChain.Wallets.XEP6
{
    /// <summary>
    /// An implementation of the XEP-6 wallet standard.
    /// </summary>
    /// <remarks>https://github.com/epicchainlabs/epicchain
    public class XEP6Wallet : Wallet
    {
        private SecureString password;
        private string name;
        private Version version;
        private readonly Dictionary<UInt160, XEP6Account> accounts;
        private readonly JToken extra;

        /// <summary>
        /// The parameters of the SCrypt algorithm used for encrypting and decrypting the private keys in the wallet.
        /// </summary>
        public readonly ScryptParameters Scrypt;

        public override string Name => name;

        /// <summary>
        /// The version of the wallet standard. It is currently fixed at 1.0 and will be used for functional upgrades in the future.
        /// </summary>
        public override Version Version => version;

        /// <summary>
        /// Loads or creates a wallet at the specified path.
        /// </summary>
        /// <param name="path">The path of the wallet file.</param>
        /// <param name="password">The password of the wallet.</param>
        /// <param name="settings">The <see cref="ProtocolSettings"/> to be used by the wallet.</param>
        /// <param name="name">The name of the wallet. If the wallet is loaded from an existing file, this parameter is ignored.</param>
        public XEP6Wallet(string path, string password, ProtocolSettings settings, string name = null) : base(path, settings)
        {
            this.password = password.ToSecureString();
            if (File.Exists(path))
            {
                var wallet = (JObject)JToken.Parse(File.ReadAllBytes(path));
                LoadFromJson(wallet, out Scrypt, out accounts, out extra);
            }
            else
            {
                this.name = name;
                version = Version.Parse("1.0");
                Scrypt = ScryptParameters.Default;
                accounts = new Dictionary<UInt160, XEP6Account>();
                extra = JToken.Null;
            }
        }

        /// <summary>
        /// Loads the wallet with the specified JSON string.
        /// </summary>
        /// <param name="path">The path of the wallet.</param>
        /// <param name="password">The password of the wallet.</param>
        /// <param name="settings">The <see cref="ProtocolSettings"/> to be used by the wallet.</param>
        /// <param name="json">The JSON object representing the wallet.</param>
        public XEP6Wallet(string path, string password, ProtocolSettings settings, JObject json) : base(path, settings)
        {
            this.password = password.ToSecureString();
            LoadFromJson(json, out Scrypt, out accounts, out extra);
        }

        private void LoadFromJson(JObject wallet, out ScryptParameters scrypt, out Dictionary<UInt160, XEP6Account> accounts, out JToken extra)
        {
            version = Version.Parse(wallet["version"].AsString());
            name = wallet["name"]?.AsString();
            scrypt = ScryptParameters.FromJson((JObject)wallet["scrypt"]);
            accounts = ((JArray)wallet["accounts"]).Select(p => XEP6Account.FromJson((JObject)p, this)).ToDictionary(p => p.ScriptHash);
            extra = wallet["extra"];
            if (!VerifyPasswordInternal(password.GetClearText()))
                throw new InvalidOperationException("Wrong password.");
        }

        private void AddAccount(XEP6Account account)
        {
            lock (accounts)
            {
                if (accounts.TryGetValue(account.ScriptHash, out XEP6Account account_old))
                {
                    account.Label = account_old.Label;
                    account.IsDefault = account_old.IsDefault;
                    account.Lock = account_old.Lock;
                    if (account.Contract == null)
                    {
                        account.Contract = account_old.Contract;
                    }
                    else
                    {
                        XEP6Contract contract_old = (XEP6Contract)account_old.Contract;
                        if (contract_old != null)
                        {
                            XEP6Contract contract = (XEP6Contract)account.Contract;
                            contract.ParameterNames = contract_old.ParameterNames;
                            contract.Deployed = contract_old.Deployed;
                        }
                    }
                    account.Extra = account_old.Extra;
                }
                accounts[account.ScriptHash] = account;
            }
        }

        public override bool Contains(UInt160 scriptHash)
        {
            lock (accounts)
            {
                return accounts.ContainsKey(scriptHash);
            }
        }

        public override WalletAccount CreateAccount(byte[] privateKey)
        {
            if (privateKey is null) throw new ArgumentNullException(nameof(privateKey));
            KeyPair key = new(privateKey);
            if (key.PublicKey.IsInfinity) throw new ArgumentException(null, nameof(privateKey));
            XEP6Contract contract = new()
            {
                Script = Contract.CreateSignatureRedeemScript(key.PublicKey),
                ParameterList = new[] { ContractParameterType.Signature },
                ParameterNames = new[] { "signature" },
                Deployed = false
            };
            XEP6Account account = new(this, contract.ScriptHash, key, password.GetClearText())
            {
                Contract = contract
            };
            AddAccount(account);
            return account;
        }

        public override WalletAccount CreateAccount(Contract contract, KeyPair key = null)
        {
            if (contract is not XEP6Contract XEP6Contract)
            {
                XEP6Contract = new XEP6Contract
                {
                    Script = contract.Script,
                    ParameterList = contract.ParameterList,
                    ParameterNames = contract.ParameterList.Select((p, i) => $"parameter{i}").ToArray(),
                    Deployed = false
                };
            }
            XEP6Account account;
            if (key == null)
                account = new XEP6Account(this, XEP6Contract.ScriptHash);
            else
                account = new XEP6Account(this, XEP6Contract.ScriptHash, key, password.GetClearText());
            account.Contract = XEP6Contract;
            AddAccount(account);
            return account;
        }

        public override WalletAccount CreateAccount(UInt160 scriptHash)
        {
            XEP6Account account = new(this, scriptHash);
            AddAccount(account);
            return account;
        }

        /// <summary>
        /// Decrypts the specified XEP-2 string with the password of the wallet.
        /// </summary>
        /// <param name="Xep2key">The XEP-2 string to decrypt.</param>
        /// <returns>The decrypted private key.</returns>
        internal KeyPair DecryptKey(string Xep2key)
        {
            return new KeyPair(GetPrivateKeyFromXEP2(Xep2key, password.GetClearText(), ProtocolSettings.AddressVersion, Scrypt.N, Scrypt.R, Scrypt.P));
        }

        public override void Delete()
        {
            if (File.Exists(Path)) File.Delete(Path);
        }

        public override bool DeleteAccount(UInt160 scriptHash)
        {
            lock (accounts)
            {
                return accounts.Remove(scriptHash);
            }
        }

        public override WalletAccount GetAccount(UInt160 scriptHash)
        {
            lock (accounts)
            {
                accounts.TryGetValue(scriptHash, out XEP6Account account);
                return account;
            }
        }

        public override IEnumerable<WalletAccount> GetAccounts()
        {
            lock (accounts)
            {
                foreach (XEP6Account account in accounts.Values)
                    yield return account;
            }
        }

        public override WalletAccount Import(X509Certificate2 cert)
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                throw new PlatformNotSupportedException("Importing certificates is not supported on macOS.");
            }
            KeyPair key;
            using (ECDsa ecdsa = cert.GetECDsaPrivateKey())
            {
                key = new KeyPair(ecdsa.ExportParameters(true).D);
            }
            XEP6Contract contract = new()
            {
                Script = Contract.CreateSignatureRedeemScript(key.PublicKey),
                ParameterList = new[] { ContractParameterType.Signature },
                ParameterNames = new[] { "signature" },
                Deployed = false
            };
            XEP6Account account = new(this, contract.ScriptHash, key, password.GetClearText())
            {
                Contract = contract
            };
            AddAccount(account);
            return account;
        }

        public override WalletAccount Import(string wif)
        {
            KeyPair key = new(GetPrivateKeyFromWIF(wif));
            XEP6Contract contract = new()
            {
                Script = Contract.CreateSignatureRedeemScript(key.PublicKey),
                ParameterList = new[] { ContractParameterType.Signature },
                ParameterNames = new[] { "signature" },
                Deployed = false
            };
            XEP6Account account = new(this, contract.ScriptHash, key, password.GetClearText())
            {
                Contract = contract
            };
            AddAccount(account);
            return account;
        }

        public override WalletAccount Import(string xep2, string passphrase, int N = 16384, int r = 8, int p = 8)
        {
            KeyPair key = new(GetPrivateKeyFromXEP2(xep2, passphrase, ProtocolSettings.AddressVersion, N, r, p));
            XEP6Contract contract = new()
            {
                Script = Contract.CreateSignatureRedeemScript(key.PublicKey),
                ParameterList = new[] { ContractParameterType.Signature },
                ParameterNames = new[] { "signature" },
                Deployed = false
            };
            XEP6Account account;
            if (Scrypt.N == 16384 && Scrypt.R == 8 && Scrypt.P == 8)
                account = new XEP6Account(this, contract.ScriptHash, xep2);
            else
                account = new XEP6Account(this, contract.ScriptHash, key, passphrase);
            account.Contract = contract;
            AddAccount(account);
            return account;
        }

        /// <summary>
        /// Exports the wallet as JSON
        /// </summary>
        public JObject ToJson()
        {
            return new()
            {
                ["name"] = name,
                ["version"] = version.ToString(),
                ["scrypt"] = Scrypt.ToJson(),
                ["accounts"] = accounts.Values.Select(p => p.ToJson()).ToArray(),
                ["extra"] = extra
            };
        }

        public override void Save()
        {
            File.WriteAllText(Path, ToJson().ToString());
        }

        public override bool VerifyPassword(string password)
        {
            return this.password.GetClearText() == password;
        }

        private bool VerifyPasswordInternal(string password)
        {
            lock (accounts)
            {
                XEP6Account account = accounts.Values.FirstOrDefault(p => !p.Decrypted);
                if (account == null)
                {
                    account = accounts.Values.FirstOrDefault(p => p.HasKey);
                }
                if (account == null) return true;
                if (account.Decrypted)
                {
                    return account.VerifyPassword(password);
                }
                else
                {
                    try
                    {
                        account.GetKey(password);
                        return true;
                    }
                    catch (FormatException)
                    {
                        return false;
                    }
                }
            }
        }

        public override bool ChangePassword(string oldPassword, string newPassword)
        {
            bool succeed = true;
            lock (accounts)
            {
                Parallel.ForEach(accounts.Values, (account, state) =>
                {
                    if (!account.ChangePasswordPrepare(oldPassword, newPassword))
                    {
                        state.Stop();
                        succeed = false;
                    }
                });
            }
            if (succeed)
            {
                foreach (XEP6Account account in accounts.Values)
                    account.ChangePasswordCommit();
                password = newPassword.ToSecureString();
            }
            else
            {
                foreach (XEP6Account account in accounts.Values)
                    account.ChangePasswordRollback();
            }
            return succeed;
        }
    }
}

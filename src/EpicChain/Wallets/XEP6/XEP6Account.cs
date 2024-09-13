// Copyright (C) 2021-2024 EpicChain Labs.

//
// XEP6Account.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Json;
using System;
using System.Threading;

namespace EpicChain.Wallets.XEP6
{
    sealed class XEP6Account : WalletAccount
    {
        private readonly XEP6Wallet wallet;
        private string Xep2key;
        private string Xep2keyNew = null;
        private KeyPair key;
        public JToken Extra;

        public bool Decrypted => Xep2key == null || key != null;
        public override bool HasKey => Xep2key != null;

        public XEP6Account(XEP6Wallet wallet, UInt160 scriptHash, string Xep2key = null)
            : base(scriptHash, wallet.ProtocolSettings)
        {
            this.wallet = wallet;
            this.Xep2key = Xep2key;
        }

        public XEP6Account(XEP6Wallet wallet, UInt160 scriptHash, KeyPair key, string password)
            : this(wallet, scriptHash, key.Export(password, wallet.ProtocolSettings.AddressVersion, wallet.Scrypt.N, wallet.Scrypt.R, wallet.Scrypt.P))
        {
            this.key = key;
        }

        public static XEP6Account FromJson(JObject json, XEP6Wallet wallet)
        {
            return new XEP6Account(wallet, json["address"].GetString().ToScriptHash(wallet.ProtocolSettings.AddressVersion), json["key"]?.GetString())
            {
                Label = json["label"]?.GetString(),
                IsDefault = json["isDefault"].GetBoolean(),
                Lock = json["lock"].GetBoolean(),
                Contract = XEP6Contract.FromJson((JObject)json["contract"]),
                Extra = json["extra"]
            };
        }

        public override KeyPair GetKey()
        {
            if (Xep2key == null) return null;
            if (key == null)
            {
                key = wallet.DecryptKey(Xep2key);
            }
            return key;
        }

        public KeyPair GetKey(string password)
        {
            if (Xep2key == null) return null;
            if (key == null)
            {
                key = new KeyPair(Wallet.GetPrivateKeyFromXEP2(Xep2key, password, ProtocolSettings.AddressVersion, wallet.Scrypt.N, wallet.Scrypt.R, wallet.Scrypt.P));
            }
            return key;
        }

        public JObject ToJson()
        {
            JObject account = new();
            account["address"] = ScriptHash.ToAddress(ProtocolSettings.AddressVersion);
            account["label"] = Label;
            account["isDefault"] = IsDefault;
            account["lock"] = Lock;
            account["key"] = Xep2key;
            account["contract"] = ((XEP6Contract)Contract)?.ToJson();
            account["extra"] = Extra;
            return account;
        }

        public bool VerifyPassword(string password)
        {
            try
            {
                Wallet.GetPrivateKeyFromXEP2(Xep2key, password, ProtocolSettings.AddressVersion, wallet.Scrypt.N, wallet.Scrypt.R, wallet.Scrypt.P);
                return true;
            }
            catch (FormatException)
            {
                return false;
            }
        }

        /// <summary>
        /// Cache draft Xep2key during wallet password changing process. Should not be called alone for a single account
        /// </summary>
        internal bool ChangePasswordPrepare(string password_old, string password_new)
        {
            if (WatchOnly) return true;
            KeyPair keyTemplate = key;
            if (Xep2key == null)
            {
                if (keyTemplate == null)
                {
                    return true;
                }
            }
            else
            {
                try
                {
                    keyTemplate = new KeyPair(Wallet.GetPrivateKeyFromXEP2(Xep2key, password_old, ProtocolSettings.AddressVersion, wallet.Scrypt.N, wallet.Scrypt.R, wallet.Scrypt.P));
                }
                catch
                {
                    return false;
                }
            }
            Xep2keyNew = keyTemplate.Export(password_new, ProtocolSettings.AddressVersion, wallet.Scrypt.N, wallet.Scrypt.R, wallet.Scrypt.P);
            return true;
        }

        internal void ChangePasswordCommit()
        {
            if (Xep2keyNew != null)
            {
                Xep2key = Interlocked.Exchange(ref Xep2keyNew, null);
            }
        }

        internal void ChangePasswordRollback()
        {
            Xep2keyNew = null;
        }
    }
}

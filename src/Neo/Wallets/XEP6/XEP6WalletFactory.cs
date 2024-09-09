// Copyright (C) 2015-2024 The Neo Project.
//
// XEP6WalletFactory.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.IO;

namespace Neo.Wallets.XEP6
{
    class XEP6WalletFactory : IWalletFactory
    {
        public static readonly XEP6WalletFactory Instance = new();

        public bool Handle(string path)
        {
            return Path.GetExtension(path).ToLowerInvariant() == ".json";
        }

        public Wallet CreateWallet(string name, string path, string password, ProtocolSettings settings)
        {
            if (File.Exists(path))
                throw new InvalidOperationException("The wallet file already exists.");
            XEP6Wallet wallet = new XEP6Wallet(path, password, settings, name);
            wallet.Save();
            return wallet;
        }

        public Wallet OpenWallet(string path, string password, ProtocolSettings settings)
        {
            return new XEP6Wallet(path, password, settings);
        }
    }
}

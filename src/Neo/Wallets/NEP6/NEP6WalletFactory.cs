// Copyright (C) 2021-2024 The EpicChain Labs.
//
// NEP6WalletFactory.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.IO;

namespace Neo.Wallets.NEP6
{
    class NEP6WalletFactory : IWalletFactory
    {
        public static readonly NEP6WalletFactory Instance = new();

        public bool Handle(string path)
        {
            return Path.GetExtension(path).ToLowerInvariant() == ".json";
        }

        public Wallet CreateWallet(string name, string path, string password, ProtocolSettings settings)
        {
            if (File.Exists(path))
                throw new InvalidOperationException("The wallet file already exists.");
            NEP6Wallet wallet = new NEP6Wallet(path, password, settings, name);
            wallet.Save();
            return wallet;
        }

        public Wallet OpenWallet(string path, string password, ProtocolSettings settings)
        {
            return new NEP6Wallet(path, password, settings);
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// XEP6WalletFactory.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.IO;

namespace EpicChain.Wallets.XEP6
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

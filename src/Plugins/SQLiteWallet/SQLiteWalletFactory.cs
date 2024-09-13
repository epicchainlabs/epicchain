// Copyright (C) 2021-2024 EpicChain Labs.

//
// SQLiteWalletFactory.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Plugins;
using static System.IO.Path;

namespace EpicChain.Wallets.SQLite;

public class SQLiteWalletFactory : Plugin, IWalletFactory
{
    public override string Name => "SQLiteWallet";
    public override string Description => "A SQLite-based wallet provider that supports wallet files with .db3 suffix.";

    public SQLiteWalletFactory()
    {
        Wallet.RegisterFactory(this);
    }

    public bool Handle(string path)
    {
        return GetExtension(path).ToLowerInvariant() == ".db3";
    }

    public Wallet CreateWallet(string name, string path, string password, ProtocolSettings settings)
    {
        return SQLiteWallet.Create(path, password, settings);
    }

    public Wallet OpenWallet(string path, string password, ProtocolSettings settings)
    {
        return SQLiteWallet.Open(path, password, settings);
    }
}

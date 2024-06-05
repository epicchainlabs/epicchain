// Copyright (C) 2021-2024 The EpicChain Labs.
//
// SQLiteWalletFactory.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Plugins;
using static System.IO.Path;

namespace Neo.Wallets.SQLite;

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

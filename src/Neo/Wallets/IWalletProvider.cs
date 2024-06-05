// Copyright (C) 2021-2024 The EpicChain Labs.
//
// IWalletProvider.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.Wallets
{
    /// <summary>
    /// A provider for obtaining wallet instance.
    /// </summary>
    public interface IWalletProvider
    {
        /// <summary>
        /// Triggered when a wallet is opened or closed.
        /// </summary>
        event EventHandler<Wallet> WalletChanged;

        /// <summary>
        /// Get the currently opened <see cref="Wallet"/> instance.
        /// </summary>
        /// <returns>The opened wallet. Or <see langword="null"/> if no wallet is opened.</returns>
        Wallet GetWallet();
    }
}

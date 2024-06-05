// Copyright (C) 2021-2024 The EpicChain Labs.
//
// WalletAccount.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.SmartContract;

namespace Neo.Wallets
{
    /// <summary>
    /// Represents an account in a wallet.
    /// </summary>
    public abstract class WalletAccount
    {
        /// <summary>
        /// The <see cref="Neo.ProtocolSettings"/> to be used by the wallet.
        /// </summary>
        protected readonly ProtocolSettings ProtocolSettings;

        /// <summary>
        /// The hash of the account.
        /// </summary>
        public readonly UInt160 ScriptHash;

        /// <summary>
        /// The label of the account.
        /// </summary>
        public string Label;

        /// <summary>
        /// Indicates whether the account is the default account in the wallet.
        /// </summary>
        public bool IsDefault;

        /// <summary>
        /// Indicates whether the account is locked.
        /// </summary>
        public bool Lock;

        /// <summary>
        /// The contract of the account.
        /// </summary>
        public Contract Contract;

        /// <summary>
        /// The address of the account.
        /// </summary>
        public string Address => ScriptHash.ToAddress(ProtocolSettings.AddressVersion);

        /// <summary>
        /// Indicates whether the account contains a private key.
        /// </summary>
        public abstract bool HasKey { get; }

        /// <summary>
        /// Indicates whether the account is a watch-only account.
        /// </summary>
        public bool WatchOnly => Contract == null;

        /// <summary>
        /// Gets the private key of the account.
        /// </summary>
        /// <returns>The private key of the account. Or <see langword="null"/> if there is no private key in the account.</returns>
        public abstract KeyPair GetKey();

        /// <summary>
        /// Initializes a new instance of the <see cref="WalletAccount"/> class.
        /// </summary>
        /// <param name="scriptHash">The hash of the account.</param>
        /// <param name="settings">The <see cref="Neo.ProtocolSettings"/> to be used by the wallet.</param>
        protected WalletAccount(UInt160 scriptHash, ProtocolSettings settings)
        {
            ProtocolSettings = settings;
            ScriptHash = scriptHash;
        }
    }
}

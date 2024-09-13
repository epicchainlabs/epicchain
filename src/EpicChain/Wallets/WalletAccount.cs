// Copyright (C) 2021-2024 EpicChain Labs.

//
// WalletAccount.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.SmartContract;

namespace EpicChain.Wallets
{
    /// <summary>
    /// Represents an account in a wallet.
    /// </summary>
    public abstract class WalletAccount
    {
        /// <summary>
        /// The <see cref="EpicChain.ProtocolSettings"/> to be used by the wallet.
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
        /// <param name="settings">The <see cref="EpicChain.ProtocolSettings"/> to be used by the wallet.</param>
        protected WalletAccount(UInt160 scriptHash, ProtocolSettings settings)
        {
            ProtocolSettings = settings;
            ScriptHash = scriptHash;
        }
    }
}

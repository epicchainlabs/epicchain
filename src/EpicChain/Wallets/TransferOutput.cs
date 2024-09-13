// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransferOutput.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.Wallets
{
    /// <summary>
    /// Represents an output of a transfer.
    /// </summary>
    public class TransferOutput
    {
        /// <summary>
        /// The id of the asset to transfer.
        /// </summary>
        public UInt160 AssetId;

        /// <summary>
        /// The amount of the asset to transfer.
        /// </summary>
        public BigDecimal Value;

        /// <summary>
        /// The account to transfer to.
        /// </summary>
        public UInt160 ScriptHash;

        /// <summary>
        /// The object to be passed to the transfer method of XEP-17.
        /// </summary>
        public object Data;
    }
}

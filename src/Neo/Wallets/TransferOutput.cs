// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TransferOutput.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.Wallets
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
        /// The object to be passed to the transfer method of NEP-17.
        /// </summary>
        public object Data;
    }
}

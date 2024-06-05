// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TransactionRemovalReason.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.Ledger
{
    /// <summary>
    /// The reason a transaction was removed.
    /// </summary>
    public enum TransactionRemovalReason : byte
    {
        /// <summary>
        /// The transaction was rejected since it was the lowest priority transaction and the memory pool capacity was exceeded.
        /// </summary>
        CapacityExceeded,

        /// <summary>
        /// The transaction was rejected due to failing re-validation after a block was persisted.
        /// </summary>
        NoLongerValid,

        /// <summary>
        /// The transaction was rejected due to conflict with higher priority transactions with Conflicts attribute.
        /// </summary>
        Conflict,
    }
}

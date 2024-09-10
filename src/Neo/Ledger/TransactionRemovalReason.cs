// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionRemovalReason.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

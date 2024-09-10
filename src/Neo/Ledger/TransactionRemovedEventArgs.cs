// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionRemovedEventArgs.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Neo.Network.P2P.Payloads;
using System.Collections.Generic;

namespace Neo.Ledger;

/// <summary>
/// Represents the event data of <see cref="MemoryPool.TransactionRemoved"/>.
/// </summary>
public sealed class TransactionRemovedEventArgs
{
    /// <summary>
    /// The <see cref="Transaction"/>s that is being removed.
    /// </summary>
    public IReadOnlyCollection<Transaction> Transactions { get; init; }

    /// <summary>
    /// The reason a transaction was removed.
    /// </summary>
    public TransactionRemovalReason Reason { get; init; }
}

// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TransactionRemovedEventArgs.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


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

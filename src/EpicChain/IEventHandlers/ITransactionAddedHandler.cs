// Copyright (C) 2021-2024 EpicChain Labs.

//
// ITransactionAddedHandler.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;

namespace EpicChain.IEventHandlers;

public interface ITransactionAddedHandler
{
    /// <summary>
    /// The handler of TransactionAdded event from the <see cref="MemoryPool"/>.
    /// Triggered when a transaction is added to the <see cref="MemoryPool"/>.
    /// </summary>
    /// <param name="sender">The source of the event</param>
    /// <param name="tx">The transaction added to the memory pool <see cref="MemoryPool"/>.</param>
    void MemoryPool_TransactionAdded_Handler(object sender, Transaction tx);
}
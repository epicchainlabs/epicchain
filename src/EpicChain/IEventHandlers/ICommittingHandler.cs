// Copyright (C) 2021-2024 EpicChain Labs.

//
// ICommittingHandler.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Persistence;
using System.Collections.Generic;

namespace EpicChain.IEventHandlers;

public interface ICommittingHandler
{
    /// <summary>
    /// This is the handler of Committing event from <see cref="Blockchain"/>
    /// Triggered when a new block is committing, and the state is still in the cache.
    /// </summary>
    /// <param name="system">The <see cref="EpicChainSystem"/> instance associated with the event.</param>
    /// <param name="block">The block that is being committed.</param>
    /// <param name="snapshot">The current data snapshot.</param>
    /// <param name="applicationExecutedList">A list of executed applications associated with the block.</param>
    void Blockchain_Committing_Handler(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList);
}

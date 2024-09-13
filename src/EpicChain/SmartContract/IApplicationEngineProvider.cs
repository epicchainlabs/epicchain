// Copyright (C) 2021-2024 EpicChain Labs.

//
// IApplicationEngineProvider.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.VM;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// A provider for creating <see cref="ApplicationEngine"/> instances.
    /// </summary>
    public interface IApplicationEngineProvider
    {
        /// <summary>
        /// Creates a new instance of the <see cref="ApplicationEngine"/> class or its subclass. This method will be called by <see cref="ApplicationEngine.Create"/>.
        /// </summary>
        /// <param name="trigger">The trigger of the execution.</param>
        /// <param name="container">The container of the script.</param>
        /// <param name="snapshot">The snapshot used by the engine during execution.</param>
        /// <param name="persistingBlock">The block being persisted. It should be <see langword="null"/> if the <paramref name="trigger"/> is <see cref="TriggerType.Verification"/>.</param>
        /// <param name="settings">The <see cref="ProtocolSettings"/> used by the engine.</param>
        /// <param name="epicpulse">The maximum epicpulse used in this execution. The execution will fail when the epicpulse is exhausted.</param>
        /// <param name="diagnostic">The diagnostic to be used by the <see cref="ApplicationEngine"/>.</param>
        /// <param name="jumpTable">The jump table to be used by the <see cref="ApplicationEngine"/>.</param>
        /// <returns>The engine instance created.</returns>
        ApplicationEngine Create(TriggerType trigger, IVerifiable container, DataCache snapshot, Block persistingBlock, ProtocolSettings settings, long epicpulse, IDiagnostic diagnostic, JumpTable jumpTable);
    }
}

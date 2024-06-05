// Copyright (C) 2021-2024 The EpicChain Labs.
//
// IApplicationEngineProvider.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.Persistence;
using Neo.VM;

namespace Neo.SmartContract
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
        /// <param name="gas">The maximum gas used in this execution. The execution will fail when the gas is exhausted.</param>
        /// <param name="diagnostic">The diagnostic to be used by the <see cref="ApplicationEngine"/>.</param>
        /// <param name="jumpTable">The jump table to be used by the <see cref="ApplicationEngine"/>.</param>
        /// <returns>The engine instance created.</returns>
        ApplicationEngine Create(TriggerType trigger, IVerifiable container, DataCache snapshot, Block persistingBlock, ProtocolSettings settings, long gas, IDiagnostic diagnostic, JumpTable jumpTable);
    }
}

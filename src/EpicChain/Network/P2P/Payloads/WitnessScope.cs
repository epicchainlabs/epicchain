// Copyright (C) 2021-2024 EpicChain Labs.

//
// WitnessScope.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents the scope of a <see cref="Witness"/>.
    /// </summary>
    [Flags]
    public enum WitnessScope : byte
    {
        /// <summary>
        /// Indicates that no contract was witnessed. Only sign the transaction.
        /// </summary>
        None = 0,

        /// <summary>
        /// Indicates that the calling contract must be the entry contract.
        /// The witness/permission/signature given on first invocation will automatically expire if entering deeper internal invokes.
        /// This can be the default safe choice for native EpicChain/EpicPulse (previously used on EpicChain 2 as "attach" mode).
        /// </summary>
        CalledByEntry = 0x01,

        /// <summary>
        /// Custom hash for contract-specific.
        /// </summary>
        CustomContracts = 0x10,

        /// <summary>
        /// Custom pubkey for group members.
        /// </summary>
        CustomGroups = 0x20,

        /// <summary>
        /// Indicates that the current context must satisfy the specified rules.
        /// </summary>
        WitnessRules = 0x40,

        /// <summary>
        /// This allows the witness in all contexts (default EpicChain behavior).
        /// </summary>
        /// <remarks>Note: It cannot be combined with other flags.</remarks>
        Global = 0x80
    }
}

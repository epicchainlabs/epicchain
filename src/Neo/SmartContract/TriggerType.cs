// Copyright (C) 2021-2024 EpicChain Labs.

//
// TriggerType.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents the triggers for running smart contracts.
    /// </summary>
    [Flags]
    public enum TriggerType : byte
    {
        /// <summary>
        /// Indicate that the contract is triggered by the system to execute the OnPersist method of the native contracts.
        /// </summary>
        OnPersist = 0x01,

        /// <summary>
        /// Indicate that the contract is triggered by the system to execute the PostPersist method of the native contracts.
        /// </summary>
        PostPersist = 0x02,

        /// <summary>
        /// Indicates that the contract is triggered by the verification of a <see cref="IVerifiable"/>.
        /// </summary>
        Verification = 0x20,

        /// <summary>
        /// Indicates that the contract is triggered by the execution of transactions.
        /// </summary>
        Application = 0x40,

        /// <summary>
        /// The combination of all system triggers.
        /// </summary>
        System = OnPersist | PostPersist,

        /// <summary>
        /// The combination of all triggers.
        /// </summary>
        All = OnPersist | PostPersist | Verification | Application
    }
}

// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TriggerType.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

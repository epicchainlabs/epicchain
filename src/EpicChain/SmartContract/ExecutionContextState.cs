// Copyright (C) 2021-2024 EpicChain Labs.

//
// ExecutionContextState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Persistence;
using EpicChain.VM;
using System;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// Represents the custom state in <see cref="ExecutionContext"/>.
    /// </summary>
    public class ExecutionContextState
    {
        /// <summary>
        /// The script hash of the current context.
        /// </summary>
        public UInt160 ScriptHash { get; set; }

        /// <summary>
        /// The calling context.
        /// </summary>
        public ExecutionContext CallingContext { get; set; }

        /// <summary>
        /// The script hash of the calling native contract. Used in native contracts only.
        /// </summary>
        internal UInt160 NativeCallingScriptHash { get; set; }

        /// <summary>
        /// The <see cref="ContractState"/> of the current context.
        /// </summary>
        public ContractState Contract { get; set; }

        /// <summary>
        /// The <see cref="SmartContract.CallFlags"/> of the current context.
        /// </summary>
        public CallFlags CallFlags { get; set; } = CallFlags.All;

        [Obsolete("Use SnapshotCache instead")]
        public DataCache Snapshot => SnapshotCache;

        public DataCache SnapshotCache { get; set; }

        public int NotificationCount { get; set; }

        public bool IsDynamicCall { get; set; }
    }
}

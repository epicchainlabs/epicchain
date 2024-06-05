// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ExecutionContextState.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Persistence;
using Neo.VM;

namespace Neo.SmartContract
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

        public DataCache Snapshot { get; set; }

        public int NotificationCount { get; set; }

        public bool IsDynamicCall { get; set; }
    }
}

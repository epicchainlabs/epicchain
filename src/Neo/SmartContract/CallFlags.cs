// Copyright (C) 2021-2024 The EpicChain Labs.
//
// CallFlags.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents the operations allowed when a contract is called.
    /// </summary>
    [Flags]
    public enum CallFlags : byte
    {
        /// <summary>
        /// No flag is set.
        /// </summary>
        None = 0,

        /// <summary>
        /// Indicates that the called contract is allowed to read states.
        /// </summary>
        ReadStates = 0b00000001,

        /// <summary>
        /// Indicates that the called contract is allowed to write states.
        /// </summary>
        WriteStates = 0b00000010,

        /// <summary>
        /// Indicates that the called contract is allowed to call another contract.
        /// </summary>
        AllowCall = 0b00000100,

        /// <summary>
        /// Indicates that the called contract is allowed to send notifications.
        /// </summary>
        AllowNotify = 0b00001000,

        /// <summary>
        /// Indicates that the called contract is allowed to read or write states.
        /// </summary>
        States = ReadStates | WriteStates,

        /// <summary>
        /// Indicates that the called contract is allowed to read states or call another contract.
        /// </summary>
        ReadOnly = ReadStates | AllowCall,

        /// <summary>
        /// All flags are set.
        /// </summary>
        All = States | AllowCall | AllowNotify
    }
}

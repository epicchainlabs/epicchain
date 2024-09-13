// Copyright (C) 2021-2024 EpicChain Labs.

//
// WitnessConditionType.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO.Caching;

namespace EpicChain.Network.P2P.Payloads.Conditions
{
    /// <summary>
    /// Represents the type of <see cref="WitnessCondition"/>.
    /// </summary>
    public enum WitnessConditionType : byte
    {
        /// <summary>
        /// Indicates that the condition will always be met or not met.
        /// </summary>
        [ReflectionCache(typeof(BooleanCondition))]
        Boolean = 0x00,

        /// <summary>
        /// Reverse another condition.
        /// </summary>
        [ReflectionCache(typeof(NotCondition))]
        Not = 0x01,

        /// <summary>
        /// Indicates that all conditions must be met.
        /// </summary>
        [ReflectionCache(typeof(AndCondition))]
        And = 0x02,

        /// <summary>
        /// Indicates that any of the conditions meets.
        /// </summary>
        [ReflectionCache(typeof(OrCondition))]
        Or = 0x03,

        /// <summary>
        /// Indicates that the condition is met when the current context has the specified script hash.
        /// </summary>
        [ReflectionCache(typeof(ScriptHashCondition))]
        ScriptHash = 0x18,

        /// <summary>
        /// Indicates that the condition is met when the current context has the specified group.
        /// </summary>
        [ReflectionCache(typeof(GroupCondition))]
        Group = 0x19,

        /// <summary>
        /// Indicates that the condition is met when the current context is the entry point or is called by the entry point.
        /// </summary>
        [ReflectionCache(typeof(CalledByEntryCondition))]
        CalledByEntry = 0x20,

        /// <summary>
        /// Indicates that the condition is met when the current context is called by the specified contract.
        /// </summary>
        [ReflectionCache(typeof(CalledByContractCondition))]
        CalledByContract = 0x28,

        /// <summary>
        /// Indicates that the condition is met when the current context is called by the specified group.
        /// </summary>
        [ReflectionCache(typeof(CalledByGroupCondition))]
        CalledByGroup = 0x29
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// ExecutionEngineLimits.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Runtime.CompilerServices;

namespace EpicChain.VM
{
    /// <summary>
    /// Represents the restrictions on the VM.
    /// </summary>
    public sealed record ExecutionEngineLimits
    {
        /// <summary>
        /// The default strategy.
        /// </summary>
        public static readonly ExecutionEngineLimits Default = new();

        /// <summary>
        /// The maximum number of bits that <see cref="OpCode.SHL"/> and <see cref="OpCode.SHR"/> can shift.
        /// </summary>
        public int MaxShift { get; init; } = 256;

        /// <summary>
        /// The maximum number of items that can be contained in the VM's evaluation stacks and slots.
        /// </summary>
        public uint MaxStackSize { get; init; } = 2 * 1024;

        /// <summary>
        /// The maximum size of an item in the VM.
        /// </summary>
        public uint MaxItemSize { get; init; } = ushort.MaxValue * 2;

        /// <summary>
        /// The largest comparable size. If a <see cref="Types.ByteString"/> or <see cref="Types.Struct"/> exceeds this size, comparison operations on it cannot be performed in the VM.
        /// </summary>
        public uint MaxComparableSize { get; init; } = 65536;

        /// <summary>
        /// The maximum number of frames in the invocation stack of the VM.
        /// </summary>
        public uint MaxInvocationStackSize { get; init; } = 1024;

        /// <summary>
        /// The maximum nesting depth of <see langword="try"/>-<see langword="catch"/>-<see langword="finally"/> blocks.
        /// </summary>
        public uint MaxTryNestingDepth { get; init; } = 16;

        /// <summary>
        /// Allow to catch the ExecutionEngine Exceptions
        /// </summary>
        public bool CatchEngineExceptions { get; init; } = true;

        /// <summary>
        /// Assert that the size of the item meets the limit.
        /// </summary>
        /// <param name="size">The size to be checked.</param>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void AssertMaxItemSize(int size)
        {
            if (size < 0 || size > MaxItemSize)
            {
                throw new InvalidOperationException($"MaxItemSize exceed: {size}");
            }
        }

        /// <summary>
        /// Assert that the number of bits shifted meets the limit.
        /// </summary>
        /// <param name="shift">The number of bits shifted.</param>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void AssertShift(int shift)
        {
            if (shift > MaxShift || shift < 0)
            {
                throw new InvalidOperationException($"Invalid shift value: {shift}");
            }
        }
    }
}

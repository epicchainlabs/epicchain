// Copyright (C) 2021-2024 EpicChain Labs.

//
// ExceptionHandlingContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Diagnostics;

namespace EpicChain.VM
{
    /// <summary>
    /// Represents the context used for exception handling.
    /// </summary>
    [DebuggerDisplay("State={State}, CatchPointer={CatchPointer}, FinallyPointer={FinallyPointer}, EndPointer={EndPointer}")]
    public sealed class ExceptionHandlingContext
    {
        /// <summary>
        /// The position of the <see langword="catch"/> block.
        /// </summary>
        public int CatchPointer { get; }

        /// <summary>
        /// The position of the <see langword="finally"/> block.
        /// </summary>
        public int FinallyPointer { get; }

        /// <summary>
        /// The end position of the <see langword="try"/>-<see langword="catch"/>-<see langword="finally"/> block.
        /// </summary>
        public int EndPointer { get; internal set; } = -1;

        /// <summary>
        /// Indicates whether the <see langword="catch"/> block is included in the context.
        /// </summary>
        public bool HasCatch => CatchPointer >= 0;

        /// <summary>
        /// Indicates whether the <see langword="finally"/> block is included in the context.
        /// </summary>
        public bool HasFinally => FinallyPointer >= 0;

        /// <summary>
        /// Indicates the state of the context.
        /// </summary>
        public ExceptionHandlingState State { get; internal set; } = ExceptionHandlingState.Try;

        internal ExceptionHandlingContext(int catchPointer, int finallyPointer)
        {
            CatchPointer = catchPointer;
            FinallyPointer = finallyPointer;
        }
    }
}

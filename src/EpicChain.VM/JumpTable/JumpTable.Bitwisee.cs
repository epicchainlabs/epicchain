// Copyright (C) 2021-2024 EpicChain Labs.

//
// JumpTable.Bitwisee.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Runtime.CompilerServices;

namespace EpicChain.VM
{
    /// <summary>
    /// Partial class for performing bitwise and logical operations on integers within a jump table.
    /// </summary>
    public partial class JumpTable
    {
        /// <summary>
        /// Flips all of the bits of an integer.
        /// <see cref="OpCode.INVERT"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 1, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Invert(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop().GetInteger();
            engine.Push(~x);
        }

        /// <summary>
        /// Computes the bitwise AND of two integers.
        /// <see cref="OpCode.AND"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 2, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void And(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            engine.Push(x1 & x2);
        }

        /// <summary>
        /// Computes the bitwise OR of two integers.
        /// <see cref="OpCode.OR"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 2, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Or(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            engine.Push(x1 | x2);
        }

        /// <summary>
        /// Computes the bitwise XOR (exclusive OR) of two integers.
        /// <see cref="OpCode.XOR"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 2, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void XOr(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            engine.Push(x1 ^ x2);
        }

        /// <summary>
        /// Determines whether two objects are equal according to the execution engine's comparison rules.
        /// <see cref="OpCode.EQUAL"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 2, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Equal(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop();
            var x1 = engine.Pop();
            engine.Push(x1.Equals(x2, engine.Limits));
        }

        /// <summary>
        /// Determines whether two objects are not equal according to the execution engine's comparison rules.
        /// <see cref="OpCode.NOTEQUAL"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 2, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NotEqual(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop();
            var x1 = engine.Pop();
            engine.Push(!x1.Equals(x2, engine.Limits));
        }
    }
}

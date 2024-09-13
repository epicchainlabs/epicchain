// Copyright (C) 2021-2024 EpicChain Labs.

//
// JumpTable.Types.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM.Types;
using System;
using System.Runtime.CompilerServices;

namespace EpicChain.VM
{
    /// <summary>
    /// Partial class for type operations in the execution engine within a jump table.
    /// </summary>
    public partial class JumpTable
    {
        /// <summary>
        /// Determines whether the item on top of the evaluation stack is null.
        /// <see cref="OpCode.ISNULL"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 1, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void IsNull(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop();
            engine.Push(x.IsNull);
        }

        /// <summary>
        /// Determines whether the item on top of the evaluation stack has a specified type.
        /// <see cref="OpCode.ISTYPE"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 1, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void IsType(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop();
            var type = (StackItemType)instruction.TokenU8;
            if (type == StackItemType.Any || !Enum.IsDefined(typeof(StackItemType), type))
                throw new InvalidOperationException($"Invalid type: {type}");
            engine.Push(x.Type == type);
        }

        /// <summary>
        /// Converts the item on top of the evaluation stack to a specified type.
        /// <see cref="OpCode.CONVERT"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 1, Push 1</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Convert(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop();
            engine.Push(x.ConvertTo((StackItemType)instruction.TokenU8));
        }

        /// <summary>
        /// Aborts execution with a specified message.
        /// <see cref="OpCode.ABORTMSG"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 1, Push 0</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void AbortMsg(ExecutionEngine engine, Instruction instruction)
        {
            var msg = engine.Pop().GetString();
            throw new Exception($"{OpCode.ABORTMSG} is executed. Reason: {msg}");
        }

        /// <summary>
        /// Asserts a condition with a specified message, throwing an exception if the condition is false.
        /// <see cref="OpCode.ASSERTMSG"/>
        /// </summary>
        /// <param name="engine">The execution engine.</param>
        /// <param name="instruction">The instruction being executed.</param>
        /// <remarks>Pop 2, Push 0</remarks>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void AssertMsg(ExecutionEngine engine, Instruction instruction)
        {
            var msg = engine.Pop().GetString();
            var x = engine.Pop().GetBoolean();
            if (!x)
                throw new Exception($"{OpCode.ASSERTMSG} is executed with false result. Reason: {msg}");
        }
    }
}

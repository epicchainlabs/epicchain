// Copyright (C) 2021-2024 EpicChain Labs.

//
// JumpTable.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    public partial class JumpTable
    {
        /// <summary>
        /// Default JumpTable
        /// </summary>
        public static readonly JumpTable Default = new();

        public delegate void DelAction(ExecutionEngine engine, Instruction instruction);
        protected readonly DelAction[] Table = new DelAction[byte.MaxValue];

        public DelAction this[OpCode opCode]
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get => Table[(byte)opCode];
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            set { Table[(byte)opCode] = value; }
        }

        /// <summary>
        /// Jump table constructor
        /// </summary>
        /// <exception cref="InvalidOperationException">Throw an exception if the opcode was already set</exception>
        public JumpTable()
        {
            // Fill defined

            foreach (var mi in GetType().GetMethods())
            {
                if (Enum.TryParse<OpCode>(mi.Name, true, out var opCode))
                {
                    if (Table[(byte)opCode] is not null)
                    {
                        throw new InvalidOperationException($"Opcode {opCode} is already defined.");
                    }

                    Table[(byte)opCode] = (DelAction)mi.CreateDelegate(typeof(DelAction), this);
                }
            }

            // Fill with undefined

            for (var x = 0; x < Table.Length; x++)
            {
                if (Table[x] is not null) continue;

                Table[x] = InvalidOpcode;
            }
        }

        public virtual void InvalidOpcode(ExecutionEngine engine, Instruction instruction)
        {
            throw new InvalidOperationException($"Opcode {instruction.OpCode} is undefined.");
        }
    }
}

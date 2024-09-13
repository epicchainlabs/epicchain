// Copyright (C) 2021-2024 EpicChain Labs.

//
// Instruction.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Buffers.Binary;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Text;

namespace EpicChain.VM
{
    /// <summary>
    /// Represents instructions in the VM script.
    /// </summary>
    [DebuggerDisplay("OpCode={OpCode}")]
    public class Instruction
    {
        /// <summary>
        /// Represents the instruction with <see cref="OpCode.RET"/>.
        /// </summary>
        public static Instruction RET { get; } = new Instruction(OpCode.RET);

        /// <summary>
        /// The <see cref="VM.OpCode"/> of the instruction.
        /// </summary>
        public readonly OpCode OpCode;

        /// <summary>
        /// The operand of the instruction.
        /// </summary>
        public readonly ReadOnlyMemory<byte> Operand;

        private static readonly int[] OperandSizePrefixTable = new int[256];
        private static readonly int[] OperandSizeTable = new int[256];

        /// <summary>
        /// Gets the size of the instruction.
        /// </summary>
        public int Size
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                int prefixSize = OperandSizePrefixTable[(int)OpCode];
                return prefixSize > 0
                    ? 1 + prefixSize + Operand.Length
                    : 1 + OperandSizeTable[(int)OpCode];
            }
        }

        /// <summary>
        /// Gets the first operand as <see cref="short"/>.
        /// </summary>
        public short TokenI16
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return BinaryPrimitives.ReadInt16LittleEndian(Operand.Span);
            }
        }

        /// <summary>
        /// Gets the first operand as <see cref="int"/>.
        /// </summary>
        public int TokenI32
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return BinaryPrimitives.ReadInt32LittleEndian(Operand.Span);
            }
        }

        /// <summary>
        /// Gets the second operand as <see cref="int"/>.
        /// </summary>
        public int TokenI32_1
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return BinaryPrimitives.ReadInt32LittleEndian(Operand.Span[4..]);
            }
        }

        /// <summary>
        /// Gets the first operand as <see cref="sbyte"/>.
        /// </summary>
        public sbyte TokenI8
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return (sbyte)Operand.Span[0];
            }
        }

        /// <summary>
        /// Gets the second operand as <see cref="sbyte"/>.
        /// </summary>
        public sbyte TokenI8_1
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return (sbyte)Operand.Span[1];
            }
        }

        /// <summary>
        /// Gets the operand as <see cref="string"/>.
        /// </summary>
        public string TokenString
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return Encoding.ASCII.GetString(Operand.Span);
            }
        }

        /// <summary>
        /// Gets the first operand as <see cref="ushort"/>.
        /// </summary>
        public ushort TokenU16
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return BinaryPrimitives.ReadUInt16LittleEndian(Operand.Span);
            }
        }

        /// <summary>
        /// Gets the first operand as <see cref="uint"/>.
        /// </summary>
        public uint TokenU32
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return BinaryPrimitives.ReadUInt32LittleEndian(Operand.Span);
            }
        }

        /// <summary>
        /// Gets the first operand as <see cref="byte"/>.
        /// </summary>
        public byte TokenU8
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return Operand.Span[0];
            }
        }

        /// <summary>
        /// Gets the second operand as <see cref="byte"/>.
        /// </summary>
        public byte TokenU8_1
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return Operand.Span[1];
            }
        }

        static Instruction()
        {
            foreach (FieldInfo field in typeof(OpCode).GetFields(BindingFlags.Public | BindingFlags.Static))
            {
                OperandSizeAttribute? attribute = field.GetCustomAttribute<OperandSizeAttribute>();
                if (attribute == null) continue;
                int index = (int)(OpCode)field.GetValue(null)!;
                OperandSizePrefixTable[index] = attribute.SizePrefix;
                OperandSizeTable[index] = attribute.Size;
            }
        }

        private Instruction(OpCode opcode)
        {
            OpCode = opcode;
            if (!Enum.IsDefined(typeof(OpCode), opcode)) throw new BadScriptException();
        }

        internal Instruction(ReadOnlyMemory<byte> script, int ip) : this((OpCode)script.Span[ip++])
        {
            ReadOnlySpan<byte> span = script.Span;
            int operandSizePrefix = OperandSizePrefixTable[(int)OpCode];
            int operandSize = 0;
            switch (operandSizePrefix)
            {
                case 0:
                    operandSize = OperandSizeTable[(int)OpCode];
                    break;
                case 1:
                    if (ip >= span.Length)
                        throw new BadScriptException($"Instruction out of bounds. InstructionPointer: {ip}");
                    operandSize = span[ip];
                    break;
                case 2:
                    if (ip + 1 >= span.Length)
                        throw new BadScriptException($"Instruction out of bounds. InstructionPointer: {ip}");
                    operandSize = BinaryPrimitives.ReadUInt16LittleEndian(span[ip..]);
                    break;
                case 4:
                    if (ip + 3 >= span.Length)
                        throw new BadScriptException($"Instruction out of bounds. InstructionPointer: {ip}");
                    operandSize = BinaryPrimitives.ReadInt32LittleEndian(span[ip..]);
                    if (operandSize < 0)
                        throw new BadScriptException($"Instruction out of bounds. InstructionPointer: {ip}, operandSize: {operandSize}");
                    break;
            }
            ip += operandSizePrefix;
            if (operandSize > 0)
            {
                if (ip + operandSize > script.Length)
                    throw new BadScriptException($"Instrucion out of bounds. InstructionPointer: {ip}, operandSize: {operandSize}, length: {script.Length}");
                Operand = script.Slice(ip, operandSize);
            }
        }
    }
}

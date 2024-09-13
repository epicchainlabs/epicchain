// Copyright (C) 2021-2024 EpicChain Labs.

//
// ScriptBuilder.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.IO;
using System.Numerics;

namespace EpicChain.VM
{
    /// <summary>
    /// A helper class for building scripts.
    /// </summary>
    public class ScriptBuilder : IDisposable
    {
        private readonly MemoryStream ms = new();
        private readonly BinaryWriter writer;

        /// <summary>
        /// The length of the script.
        /// </summary>
        public int Length => (int)ms.Position;

        /// <summary>
        /// Initializes a new instance of the <see cref="ScriptBuilder"/> class.
        /// </summary>
        public ScriptBuilder()
        {
            writer = new BinaryWriter(ms);
        }

        public void Dispose()
        {
            writer.Dispose();
            ms.Dispose();
        }

        /// <summary>
        /// Emits an <see cref="Instruction"/> with the specified <see cref="OpCode"/> and operand.
        /// </summary>
        /// <param name="opcode">The <see cref="OpCode"/> to be emitted.</param>
        /// <param name="operand">The operand to be emitted.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder Emit(OpCode opcode, ReadOnlySpan<byte> operand = default)
        {
            writer.Write((byte)opcode);
            writer.Write(operand);
            return this;
        }

        /// <summary>
        /// Emits a call <see cref="Instruction"/> with the specified offset.
        /// </summary>
        /// <param name="offset">The offset to be called.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitCall(int offset)
        {
            if (offset < sbyte.MinValue || offset > sbyte.MaxValue)
                return Emit(OpCode.CALL_L, BitConverter.GetBytes(offset));
            else
                return Emit(OpCode.CALL, new[] { (byte)offset });
        }

        /// <summary>
        /// Emits a jump <see cref="Instruction"/> with the specified offset.
        /// </summary>
        /// <param name="opcode">The <see cref="OpCode"/> to be emitted. It must be a jump <see cref="OpCode"/></param>
        /// <param name="offset">The offset to jump.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitJump(OpCode opcode, int offset)
        {
            if (opcode < OpCode.JMP || opcode > OpCode.JMPLE_L)
                throw new ArgumentOutOfRangeException(nameof(opcode));
            if ((int)opcode % 2 == 0 && (offset < sbyte.MinValue || offset > sbyte.MaxValue))
                opcode += 1;
            if ((int)opcode % 2 == 0)
                return Emit(opcode, new[] { (byte)offset });
            else
                return Emit(opcode, BitConverter.GetBytes(offset));
        }

        /// <summary>
        /// Emits a push <see cref="Instruction"/> with the specified number.
        /// </summary>
        /// <param name="value">The number to be pushed.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitPush(BigInteger value)
        {
            if (value >= -1 && value <= 16) return Emit(OpCode.PUSH0 + (byte)(int)value);
            Span<byte> buffer = stackalloc byte[32];
            if (!value.TryWriteBytes(buffer, out int bytesWritten, isUnsigned: false, isBigEndian: false))
                throw new ArgumentOutOfRangeException(nameof(value));
            return bytesWritten switch
            {
                1 => Emit(OpCode.PUSHINT8, PadRight(buffer, bytesWritten, 1, value.Sign < 0)),
                2 => Emit(OpCode.PUSHINT16, PadRight(buffer, bytesWritten, 2, value.Sign < 0)),
                <= 4 => Emit(OpCode.PUSHINT32, PadRight(buffer, bytesWritten, 4, value.Sign < 0)),
                <= 8 => Emit(OpCode.PUSHINT64, PadRight(buffer, bytesWritten, 8, value.Sign < 0)),
                <= 16 => Emit(OpCode.PUSHINT128, PadRight(buffer, bytesWritten, 16, value.Sign < 0)),
                _ => Emit(OpCode.PUSHINT256, PadRight(buffer, bytesWritten, 32, value.Sign < 0)),
            };
        }

        /// <summary>
        /// Emits a push <see cref="Instruction"/> with the specified boolean value.
        /// </summary>
        /// <param name="value">The value to be pushed.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitPush(bool value)
        {
            return Emit(value ? OpCode.PUSHT : OpCode.PUSHF);
        }

        /// <summary>
        /// Emits a push <see cref="Instruction"/> with the specified data.
        /// </summary>
        /// <param name="data">The data to be pushed.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitPush(ReadOnlySpan<byte> data)
        {
            if (data == null)
                throw new ArgumentNullException(nameof(data));
            if (data.Length < 0x100)
            {
                Emit(OpCode.PUSHDATA1);
                writer.Write((byte)data.Length);
                writer.Write(data);
            }
            else if (data.Length < 0x10000)
            {
                Emit(OpCode.PUSHDATA2);
                writer.Write((ushort)data.Length);
                writer.Write(data);
            }
            else// if (data.Length < 0x100000000L)
            {
                Emit(OpCode.PUSHDATA4);
                writer.Write(data.Length);
                writer.Write(data);
            }
            return this;
        }

        /// <summary>
        /// Emits a push <see cref="Instruction"/> with the specified <see cref="string"/>.
        /// </summary>
        /// <param name="data">The <see cref="string"/> to be pushed.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitPush(string data)
        {
            return EmitPush(Utility.StrictUTF8.GetBytes(data));
        }

        /// <summary>
        /// Emits raw script.
        /// </summary>
        /// <param name="script">The raw script to be emitted.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitRaw(ReadOnlySpan<byte> script = default)
        {
            writer.Write(script);
            return this;
        }

        /// <summary>
        /// Emits an <see cref="Instruction"/> with <see cref="OpCode.SYSCALL"/>.
        /// </summary>
        /// <param name="api">The operand of <see cref="OpCode.SYSCALL"/>.</param>
        /// <returns>A reference to this instance after the emit operation has completed.</returns>
        public ScriptBuilder EmitSysCall(uint api)
        {
            return Emit(OpCode.SYSCALL, BitConverter.GetBytes(api));
        }

        /// <summary>
        /// Converts the value of this instance to a byte array.
        /// </summary>
        /// <returns>A byte array contains the script.</returns>
        public byte[] ToArray()
        {
            writer.Flush();
            return ms.ToArray();
        }

        private static ReadOnlySpan<byte> PadRight(Span<byte> buffer, int dataLength, int padLength, bool negative)
        {
            byte pad = negative ? (byte)0xff : (byte)0;
            for (int x = dataLength; x < padLength; x++)
                buffer[x] = pad;
            return buffer[..padLength];
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// Script.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;

namespace EpicChain.VM
{
    /// <summary>
    /// Represents the script executed in the VM.
    /// </summary>
    [DebuggerDisplay("Length={Length}")]
    public class Script
    {
        private readonly ReadOnlyMemory<byte> _value;
        private readonly bool strictMode;
        private readonly Dictionary<int, Instruction> _instructions = new();

        /// <summary>
        /// The length of the script.
        /// </summary>
        public int Length
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return _value.Length;
            }
        }

        /// <summary>
        /// Gets the <see cref="OpCode"/> at the specified index.
        /// </summary>
        /// <param name="index">The index to locate.</param>
        /// <returns>The <see cref="OpCode"/> at the specified index.</returns>
        public OpCode this[int index]
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return (OpCode)_value.Span[index];
            }
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="Script"/> class.
        /// </summary>
        /// <param name="script">The bytecodes of the script.</param>
        public Script(ReadOnlyMemory<byte> script) : this(script, false)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="Script"/> class.
        /// </summary>
        /// <param name="script">The bytecodes of the script.</param>
        /// <param name="strictMode">
        /// Indicates whether strict mode is enabled.
        /// In strict mode, the script will be checked, but the loading speed will be slower.
        /// </param>
        /// <exception cref="BadScriptException">In strict mode, the script was found to contain bad instructions.</exception>
        public Script(ReadOnlyMemory<byte> script, bool strictMode)
        {
            _value = script;
            if (strictMode)
            {
                for (int ip = 0; ip < script.Length; ip += GetInstruction(ip).Size) { }
                foreach (var (ip, instruction) in _instructions)
                {
                    switch (instruction.OpCode)
                    {
                        case OpCode.JMP:
                        case OpCode.JMPIF:
                        case OpCode.JMPIFNOT:
                        case OpCode.JMPEQ:
                        case OpCode.JMPNE:
                        case OpCode.JMPGT:
                        case OpCode.JMPGE:
                        case OpCode.JMPLT:
                        case OpCode.JMPLE:
                        case OpCode.CALL:
                        case OpCode.ENDTRY:
                            if (!_instructions.ContainsKey(checked(ip + instruction.TokenI8)))
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            break;
                        case OpCode.PUSHA:
                        case OpCode.JMP_L:
                        case OpCode.JMPIF_L:
                        case OpCode.JMPIFNOT_L:
                        case OpCode.JMPEQ_L:
                        case OpCode.JMPNE_L:
                        case OpCode.JMPGT_L:
                        case OpCode.JMPGE_L:
                        case OpCode.JMPLT_L:
                        case OpCode.JMPLE_L:
                        case OpCode.CALL_L:
                        case OpCode.ENDTRY_L:
                            if (!_instructions.ContainsKey(checked(ip + instruction.TokenI32)))
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            break;
                        case OpCode.TRY:
                            if (!_instructions.ContainsKey(checked(ip + instruction.TokenI8)))
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            if (!_instructions.ContainsKey(checked(ip + instruction.TokenI8_1)))
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            break;
                        case OpCode.TRY_L:
                            if (!_instructions.ContainsKey(checked(ip + instruction.TokenI32)))
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            if (!_instructions.ContainsKey(checked(ip + instruction.TokenI32_1)))
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            break;
                        case OpCode.NEWARRAY_T:
                        case OpCode.ISTYPE:
                        case OpCode.CONVERT:
                            StackItemType type = (StackItemType)instruction.TokenU8;
                            if (!Enum.IsDefined(typeof(StackItemType), type))
                                throw new BadScriptException();
                            if (instruction.OpCode != OpCode.NEWARRAY_T && type == StackItemType.Any)
                                throw new BadScriptException($"ip: {ip}, opcode: {instruction.OpCode}");
                            break;
                    }
                }
            }
            this.strictMode = strictMode;
        }

        /// <summary>
        /// Get the <see cref="Instruction"/> at the specified position.
        /// </summary>
        /// <param name="ip">The position to get the <see cref="Instruction"/>.</param>
        /// <returns>The <see cref="Instruction"/> at the specified position.</returns>
        /// <exception cref="ArgumentException">In strict mode, the <see cref="Instruction"/> was not found at the specified position.</exception>
        public Instruction GetInstruction(int ip)
        {
            if (ip >= Length) throw new ArgumentOutOfRangeException(nameof(ip));
            if (!_instructions.TryGetValue(ip, out Instruction? instruction))
            {
                if (strictMode) throw new ArgumentException($"ip not found with strict mode", nameof(ip));
                instruction = new Instruction(_value, ip);
                _instructions.Add(ip, instruction);
            }
            return instruction;
        }

        public static implicit operator ReadOnlyMemory<byte>(Script script) => script._value;
        public static implicit operator Script(ReadOnlyMemory<byte> script) => new(script);
        public static implicit operator Script(byte[] script) => new(script);
    }
}

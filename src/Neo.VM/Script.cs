// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// Script.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.VM.Types;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;

namespace Neo.VM
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
            this._value = script;
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

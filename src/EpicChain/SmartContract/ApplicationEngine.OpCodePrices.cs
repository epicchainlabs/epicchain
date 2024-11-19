// Copyright (C) 2021-2024 EpicChain Labs.

//
// ApplicationEngine.OpCodePrices.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM;
using System;
using System.Collections.Generic;

namespace EpicChain.SmartContract
{
    partial class ApplicationEngine
    {
        /// <summary>
        /// The prices of all the opcodes.
        /// </summary>
        [Obsolete("You should use OpCodePriceTable")]
        public static readonly IReadOnlyDictionary<OpCode, long> OpCodePrices = new Dictionary<OpCode, long>
        {
            [OpCode.PUSHINT8] = 1 << 0,
            [OpCode.PUSHINT16] = 1 << 0,
            [OpCode.PUSHINT32] = 1 << 0,
            [OpCode.PUSHINT64] = 1 << 0,
            [OpCode.PUSHINT128] = 1 << 2,
            [OpCode.PUSHINT256] = 1 << 2,
            [OpCode.PUSHT] = 1 << 0,
            [OpCode.PUSHF] = 1 << 0,
            [OpCode.PUSHA] = 1 << 2,
            [OpCode.PUSHNULL] = 1 << 0,
            [OpCode.PUSHDATA1] = 1 << 3,
            [OpCode.PUSHDATA2] = 1 << 9,
            [OpCode.PUSHDATA4] = 1 << 12,
            [OpCode.PUSHM1] = 1 << 0,
            [OpCode.PUSH0] = 1 << 0,
            [OpCode.PUSH1] = 1 << 0,
            [OpCode.PUSH2] = 1 << 0,
            [OpCode.PUSH3] = 1 << 0,
            [OpCode.PUSH4] = 1 << 0,
            [OpCode.PUSH5] = 1 << 0,
            [OpCode.PUSH6] = 1 << 0,
            [OpCode.PUSH7] = 1 << 0,
            [OpCode.PUSH8] = 1 << 0,
            [OpCode.PUSH9] = 1 << 0,
            [OpCode.PUSH10] = 1 << 0,
            [OpCode.PUSH11] = 1 << 0,
            [OpCode.PUSH12] = 1 << 0,
            [OpCode.PUSH13] = 1 << 0,
            [OpCode.PUSH14] = 1 << 0,
            [OpCode.PUSH15] = 1 << 0,
            [OpCode.PUSH16] = 1 << 0,
            [OpCode.NOP] = 1 << 0,
            [OpCode.JMP] = 1 << 1,
            [OpCode.JMP_L] = 1 << 1,
            [OpCode.JMPIF] = 1 << 1,
            [OpCode.JMPIF_L] = 1 << 1,
            [OpCode.JMPIFNOT] = 1 << 1,
            [OpCode.JMPIFNOT_L] = 1 << 1,
            [OpCode.JMPEQ] = 1 << 1,
            [OpCode.JMPEQ_L] = 1 << 1,
            [OpCode.JMPNE] = 1 << 1,
            [OpCode.JMPNE_L] = 1 << 1,
            [OpCode.JMPGT] = 1 << 1,
            [OpCode.JMPGT_L] = 1 << 1,
            [OpCode.JMPGE] = 1 << 1,
            [OpCode.JMPGE_L] = 1 << 1,
            [OpCode.JMPLT] = 1 << 1,
            [OpCode.JMPLT_L] = 1 << 1,
            [OpCode.JMPLE] = 1 << 1,
            [OpCode.JMPLE_L] = 1 << 1,
            [OpCode.CALL] = 1 << 9,
            [OpCode.CALL_L] = 1 << 9,
            [OpCode.CALLA] = 1 << 9,
            [OpCode.CALLT] = 1 << 15,
            [OpCode.ABORT] = 0,
            [OpCode.ABORTMSG] = 0,
            [OpCode.ASSERT] = 1 << 0,
            [OpCode.ASSERTMSG] = 1 << 0,
            [OpCode.THROW] = 1 << 9,
            [OpCode.TRY] = 1 << 2,
            [OpCode.TRY_L] = 1 << 2,
            [OpCode.ENDTRY] = 1 << 2,
            [OpCode.ENDTRY_L] = 1 << 2,
            [OpCode.ENDFINALLY] = 1 << 2,
            [OpCode.RET] = 0,
            [OpCode.SYSCALL] = 0,
            [OpCode.DEPTH] = 1 << 1,
            [OpCode.DROP] = 1 << 1,
            [OpCode.NIP] = 1 << 1,
            [OpCode.XDROP] = 1 << 4,
            [OpCode.CLEAR] = 1 << 4,
            [OpCode.DUP] = 1 << 1,
            [OpCode.OVER] = 1 << 1,
            [OpCode.PICK] = 1 << 1,
            [OpCode.TUCK] = 1 << 1,
            [OpCode.SWAP] = 1 << 1,
            [OpCode.ROT] = 1 << 1,
            [OpCode.ROLL] = 1 << 4,
            [OpCode.REVERSE3] = 1 << 1,
            [OpCode.REVERSE4] = 1 << 1,
            [OpCode.REVERSEN] = 1 << 4,
            [OpCode.INITSSLOT] = 1 << 4,
            [OpCode.INITSLOT] = 1 << 6,
            [OpCode.LDSFLD0] = 1 << 1,
            [OpCode.LDSFLD1] = 1 << 1,
            [OpCode.LDSFLD2] = 1 << 1,
            [OpCode.LDSFLD3] = 1 << 1,
            [OpCode.LDSFLD4] = 1 << 1,
            [OpCode.LDSFLD5] = 1 << 1,
            [OpCode.LDSFLD6] = 1 << 1,
            [OpCode.LDSFLD] = 1 << 1,
            [OpCode.STSFLD0] = 1 << 1,
            [OpCode.STSFLD1] = 1 << 1,
            [OpCode.STSFLD2] = 1 << 1,
            [OpCode.STSFLD3] = 1 << 1,
            [OpCode.STSFLD4] = 1 << 1,
            [OpCode.STSFLD5] = 1 << 1,
            [OpCode.STSFLD6] = 1 << 1,
            [OpCode.STSFLD] = 1 << 1,
            [OpCode.LDLOC0] = 1 << 1,
            [OpCode.LDLOC1] = 1 << 1,
            [OpCode.LDLOC2] = 1 << 1,
            [OpCode.LDLOC3] = 1 << 1,
            [OpCode.LDLOC4] = 1 << 1,
            [OpCode.LDLOC5] = 1 << 1,
            [OpCode.LDLOC6] = 1 << 1,
            [OpCode.LDLOC] = 1 << 1,
            [OpCode.STLOC0] = 1 << 1,
            [OpCode.STLOC1] = 1 << 1,
            [OpCode.STLOC2] = 1 << 1,
            [OpCode.STLOC3] = 1 << 1,
            [OpCode.STLOC4] = 1 << 1,
            [OpCode.STLOC5] = 1 << 1,
            [OpCode.STLOC6] = 1 << 1,
            [OpCode.STLOC] = 1 << 1,
            [OpCode.LDARG0] = 1 << 1,
            [OpCode.LDARG1] = 1 << 1,
            [OpCode.LDARG2] = 1 << 1,
            [OpCode.LDARG3] = 1 << 1,
            [OpCode.LDARG4] = 1 << 1,
            [OpCode.LDARG5] = 1 << 1,
            [OpCode.LDARG6] = 1 << 1,
            [OpCode.LDARG] = 1 << 1,
            [OpCode.STARG0] = 1 << 1,
            [OpCode.STARG1] = 1 << 1,
            [OpCode.STARG2] = 1 << 1,
            [OpCode.STARG3] = 1 << 1,
            [OpCode.STARG4] = 1 << 1,
            [OpCode.STARG5] = 1 << 1,
            [OpCode.STARG6] = 1 << 1,
            [OpCode.STARG] = 1 << 1,
            [OpCode.NEWBUFFER] = 1 << 8,
            [OpCode.MEMCPY] = 1 << 11,
            [OpCode.CAT] = 1 << 11,
            [OpCode.SUBSTR] = 1 << 11,
            [OpCode.LEFT] = 1 << 11,
            [OpCode.RIGHT] = 1 << 11,
            [OpCode.INVERT] = 1 << 2,
            [OpCode.AND] = 1 << 3,
            [OpCode.OR] = 1 << 3,
            [OpCode.XOR] = 1 << 3,
            [OpCode.EQUAL] = 1 << 5,
            [OpCode.NOTEQUAL] = 1 << 5,
            [OpCode.SIGN] = 1 << 2,
            [OpCode.ABS] = 1 << 2,
            [OpCode.NEGATE] = 1 << 2,
            [OpCode.INC] = 1 << 2,
            [OpCode.DEC] = 1 << 2,
            [OpCode.ADD] = 1 << 3,
            [OpCode.SUB] = 1 << 3,
            [OpCode.MUL] = 1 << 3,
            [OpCode.DIV] = 1 << 3,
            [OpCode.MOD] = 1 << 3,
            [OpCode.POW] = 1 << 6,
            [OpCode.SQRT] = 1 << 6,
            [OpCode.MODMUL] = 1 << 5,
            [OpCode.MODPOW] = 1 << 11,
            [OpCode.SHL] = 1 << 3,
            [OpCode.SHR] = 1 << 3,
            [OpCode.NOT] = 1 << 2,
            [OpCode.BOOLAND] = 1 << 3,
            [OpCode.BOOLOR] = 1 << 3,
            [OpCode.NZ] = 1 << 2,
            [OpCode.NUMEQUAL] = 1 << 3,
            [OpCode.NUMNOTEQUAL] = 1 << 3,
            [OpCode.LT] = 1 << 3,
            [OpCode.LE] = 1 << 3,
            [OpCode.GT] = 1 << 3,
            [OpCode.GE] = 1 << 3,
            [OpCode.MIN] = 1 << 3,
            [OpCode.MAX] = 1 << 3,
            [OpCode.WITHIN] = 1 << 3,
            [OpCode.PACKMAP] = 1 << 11,
            [OpCode.PACKSTRUCT] = 1 << 11,
            [OpCode.PACK] = 1 << 11,
            [OpCode.UNPACK] = 1 << 11,
            [OpCode.NEWARRAY0] = 1 << 4,
            [OpCode.NEWARRAY] = 1 << 9,
            [OpCode.NEWARRAY_T] = 1 << 9,
            [OpCode.NEWSTRUCT0] = 1 << 4,
            [OpCode.NEWSTRUCT] = 1 << 9,
            [OpCode.NEWMAP] = 1 << 3,
            [OpCode.SIZE] = 1 << 2,
            [OpCode.HASKEY] = 1 << 6,
            [OpCode.KEYS] = 1 << 4,
            [OpCode.VALUES] = 1 << 13,
            [OpCode.PICKITEM] = 1 << 6,
            [OpCode.APPEND] = 1 << 13,
            [OpCode.SETITEM] = 1 << 13,
            [OpCode.REVERSEITEMS] = 1 << 13,
            [OpCode.REMOVE] = 1 << 4,
            [OpCode.CLEARITEMS] = 1 << 4,
            [OpCode.POPITEM] = 1 << 4,
            [OpCode.ISNULL] = 1 << 1,
            [OpCode.ISTYPE] = 1 << 1,
            [OpCode.CONVERT] = 1 << 13,
        };

        /// <summary>
        /// The prices of all the opcodes.
        /// In the unit of datoshi, 1 datoshi = 1e-8 EpicPulse
        /// </summary>
        public static readonly long[] OpCodePriceTable = new long[byte.MaxValue];

        /// <summary>
        /// Init OpCodePrices
        /// </summary>
        static ApplicationEngine()
        {
#pragma warning disable CS0618 // Type or member is obsolete
            foreach (var entry in OpCodePrices)
#pragma warning restore CS0618 // Type or member is obsolete
            {
                OpCodePriceTable[(byte)entry.Key] = entry.Value;
            }
        }
    }
}
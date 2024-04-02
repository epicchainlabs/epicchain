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
// JumpTable.Push.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.VM.Types;
using System;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace Neo.VM
{
    public partial class JumpTable
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushInt8(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new BigInteger(instruction.Operand.Span));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushInt16(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new BigInteger(instruction.Operand.Span));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushInt32(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new BigInteger(instruction.Operand.Span));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushInt64(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new BigInteger(instruction.Operand.Span));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushInt128(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new BigInteger(instruction.Operand.Span));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushInt256(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new BigInteger(instruction.Operand.Span));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushT(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(StackItem.True);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushF(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(StackItem.False);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushA(ExecutionEngine engine, Instruction instruction)
        {
            var position = checked(engine.CurrentContext!.InstructionPointer + instruction.TokenI32);
            if (position < 0 || position > engine.CurrentContext.Script.Length)
                throw new InvalidOperationException($"Bad pointer address(Instruction instruction) {position}");
            engine.Push(new Pointer(engine.CurrentContext.Script, position));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushNull(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(StackItem.Null);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushData1(ExecutionEngine engine, Instruction instruction)
        {
            engine.Limits.AssertMaxItemSize(instruction.Operand.Length);
            engine.Push(instruction.Operand);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushData2(ExecutionEngine engine, Instruction instruction)
        {
            engine.Limits.AssertMaxItemSize(instruction.Operand.Length);
            engine.Push(instruction.Operand);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushData4(ExecutionEngine engine, Instruction instruction)
        {
            engine.Limits.AssertMaxItemSize(instruction.Operand.Length);
            engine.Push(instruction.Operand);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PushM1(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(-1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push0(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push1(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push2(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push3(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push4(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push5(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push6(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push7(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(7);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push8(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push9(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(9);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push10(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(10);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push11(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(11);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push12(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(12);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push13(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(13);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push14(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(14);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push15(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(15);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Push16(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(16);
        }
    }
}

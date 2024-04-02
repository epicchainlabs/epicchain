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
// JumpTable.Control.cs file belongs to the neo project and is free
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
using System.Runtime.CompilerServices;

namespace Neo.VM
{
    public partial class JumpTable
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Nop(ExecutionEngine engine, Instruction instruction)
        {
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Jmp(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Jmp_L(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpIf(ExecutionEngine engine, Instruction instruction)
        {
            if (engine.Pop().GetBoolean())
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpIf_L(ExecutionEngine engine, Instruction instruction)
        {
            if (engine.Pop().GetBoolean())
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpIfNot(ExecutionEngine engine, Instruction instruction)
        {
            if (!engine.Pop().GetBoolean())
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpIfNot_L(ExecutionEngine engine, Instruction instruction)
        {
            if (!engine.Pop().GetBoolean())
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpEq(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 == x2)
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpEq_L(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 == x2)
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpNe(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 != x2)
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpNe_L(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 != x2)
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpGt(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 > x2)
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpGt_L(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 > x2)
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpGe(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 >= x2)
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpGe_L(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 >= x2)
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpLt(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 < x2)
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpLt_L(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 < x2)
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JmpLe(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 <= x2)
                ExecuteJumpOffset(engine, instruction.TokenI8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void JMPLE_L(ExecutionEngine engine, Instruction instruction)
        {
            var x2 = engine.Pop().GetInteger();
            var x1 = engine.Pop().GetInteger();
            if (x1 <= x2)
                ExecuteJumpOffset(engine, instruction.TokenI32);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Call(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteCall(engine, checked(engine.CurrentContext!.InstructionPointer + instruction.TokenI8));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Call_L(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteCall(engine, checked(engine.CurrentContext!.InstructionPointer + instruction.TokenI32));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void CallA(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop<Pointer>();
            if (x.Script != engine.CurrentContext!.Script)
                throw new InvalidOperationException("Pointers can't be shared between scripts");
            ExecuteCall(engine, x.Position);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void CallT(ExecutionEngine engine, Instruction instruction)
        {
            throw new InvalidOperationException($"Token not found: {instruction.TokenU16}");
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Abort(ExecutionEngine engine, Instruction instruction)
        {
            throw new Exception($"{OpCode.ABORT} is executed.");
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Assert(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop().GetBoolean();
            if (!x)
                throw new Exception($"{OpCode.ASSERT} is executed with false result.");
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Throw(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteThrow(engine, engine.Pop());
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Try(ExecutionEngine engine, Instruction instruction)
        {
            int catchOffset = instruction.TokenI8;
            int finallyOffset = instruction.TokenI8_1;
            ExecuteTry(engine, catchOffset, finallyOffset);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Try_L(ExecutionEngine engine, Instruction instruction)
        {
            var catchOffset = instruction.TokenI32;
            var finallyOffset = instruction.TokenI32_1;
            ExecuteTry(engine, catchOffset, finallyOffset);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void EndTry(ExecutionEngine engine, Instruction instruction)
        {
            var endOffset = instruction.TokenI8;
            ExecuteEndTry(engine, endOffset);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void EndTry_L(ExecutionEngine engine, Instruction instruction)
        {
            var endOffset = instruction.TokenI32;
            ExecuteEndTry(engine, endOffset);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void EndFinally(ExecutionEngine engine, Instruction instruction)
        {
            if (engine.CurrentContext!.TryStack is null)
                throw new InvalidOperationException($"The corresponding TRY block cannot be found.");
            if (!engine.CurrentContext.TryStack.TryPop(out var currentTry))
                throw new InvalidOperationException($"The corresponding TRY block cannot be found.");

            if (engine.UncaughtException is null)
                engine.CurrentContext.InstructionPointer = currentTry.EndPointer;
            else
                ExecuteThrow(engine, engine.UncaughtException);

            engine.isJumping = true;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Ret(ExecutionEngine engine, Instruction instruction)
        {
            var context_pop = engine.InvocationStack.Pop();
            var stack_eval = engine.InvocationStack.Count == 0 ? engine.ResultStack : engine.InvocationStack.Peek().EvaluationStack;
            if (context_pop.EvaluationStack != stack_eval)
            {
                if (context_pop.RVCount >= 0 && context_pop.EvaluationStack.Count != context_pop.RVCount)
                    throw new InvalidOperationException("RVCount doesn't match with EvaluationStack");
                context_pop.EvaluationStack.CopyTo(stack_eval);
            }
            if (engine.InvocationStack.Count == 0)
                engine.State = VMState.HALT;
            engine.UnloadContext(context_pop);
            engine.isJumping = true;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Syscall(ExecutionEngine engine, Instruction instruction)
        {
            throw new InvalidOperationException($"Syscall not found: {instruction.TokenU32}");
        }

        #region Execute methods

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ExecuteCall(ExecutionEngine engine, int position)
        {
            engine.LoadContext(engine.CurrentContext!.Clone(position));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ExecuteEndTry(ExecutionEngine engine, int endOffset)
        {
            if (engine.CurrentContext!.TryStack is null)
                throw new InvalidOperationException($"The corresponding TRY block cannot be found.");
            if (!engine.CurrentContext.TryStack.TryPeek(out var currentTry))
                throw new InvalidOperationException($"The corresponding TRY block cannot be found.");
            if (currentTry.State == ExceptionHandlingState.Finally)
                throw new InvalidOperationException($"The opcode {OpCode.ENDTRY} can't be executed in a FINALLY block.");

            var endPointer = checked(engine.CurrentContext.InstructionPointer + endOffset);
            if (currentTry.HasFinally)
            {
                currentTry.State = ExceptionHandlingState.Finally;
                currentTry.EndPointer = endPointer;
                engine.CurrentContext.InstructionPointer = currentTry.FinallyPointer;
            }
            else
            {
                engine.CurrentContext.TryStack.Pop();
                engine.CurrentContext.InstructionPointer = endPointer;
            }
            engine.isJumping = true;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ExecuteJump(ExecutionEngine engine, int position)
        {
            if (position < 0 || position >= engine.CurrentContext!.Script.Length)
                throw new ArgumentOutOfRangeException($"Jump out of range for position: {position}");
            engine.CurrentContext.InstructionPointer = position;
            engine.isJumping = true;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ExecuteJumpOffset(ExecutionEngine engine, int offset)
        {
            ExecuteJump(engine, checked(engine.CurrentContext!.InstructionPointer + offset));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ExecuteTry(ExecutionEngine engine, int catchOffset, int finallyOffset)
        {
            if (catchOffset == 0 && finallyOffset == 0)
                throw new InvalidOperationException($"catchOffset and finallyOffset can't be 0 in a TRY block");
            if (engine.CurrentContext!.TryStack is null)
                engine.CurrentContext.TryStack = new Stack<ExceptionHandlingContext>();
            else if (engine.CurrentContext.TryStack.Count >= engine.Limits.MaxTryNestingDepth)
                throw new InvalidOperationException("MaxTryNestingDepth exceed.");
            var catchPointer = catchOffset == 0 ? -1 : checked(engine.CurrentContext.InstructionPointer + catchOffset);
            var finallyPointer = finallyOffset == 0 ? -1 : checked(engine.CurrentContext.InstructionPointer + finallyOffset);
            engine.CurrentContext.TryStack.Push(new ExceptionHandlingContext(catchPointer, finallyPointer));
        }

        public virtual void ExecuteThrow(ExecutionEngine engine, StackItem? ex)
        {
            engine.UncaughtException = ex;

            var pop = 0;
            foreach (var executionContext in engine.InvocationStack)
            {
                if (executionContext.TryStack != null)
                {
                    while (executionContext.TryStack.TryPeek(out var tryContext))
                    {
                        if (tryContext.State == ExceptionHandlingState.Finally || (tryContext.State == ExceptionHandlingState.Catch && !tryContext.HasFinally))
                        {
                            executionContext.TryStack.Pop();
                            continue;
                        }
                        for (var i = 0; i < pop; i++)
                        {
                            engine.UnloadContext(engine.InvocationStack.Pop());
                        }
                        if (tryContext.State == ExceptionHandlingState.Try && tryContext.HasCatch)
                        {
                            tryContext.State = ExceptionHandlingState.Catch;
                            engine.Push(engine.UncaughtException!);
                            executionContext.InstructionPointer = tryContext.CatchPointer;
                            engine.UncaughtException = null;
                        }
                        else
                        {
                            tryContext.State = ExceptionHandlingState.Finally;
                            executionContext.InstructionPointer = tryContext.FinallyPointer;
                        }
                        engine.isJumping = true;
                        return;
                    }
                }
                ++pop;
            }

            throw new VMUnhandledException(engine.UncaughtException!);
        }

        #endregion
    }
}

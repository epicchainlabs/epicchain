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
// JumpTable.Slot.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.VM;
using Neo.VM.Types;
using System;
using System.Runtime.CompilerServices;

namespace Neo.VM
{
    public partial class JumpTable
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void InitSSlot(ExecutionEngine engine, Instruction instruction)
        {
            if (engine.CurrentContext!.StaticFields != null)
                throw new InvalidOperationException($"{instruction.OpCode} cannot be executed twice.");
            if (instruction.TokenU8 == 0)
                throw new InvalidOperationException($"The operand {instruction.TokenU8} is invalid for OpCode.{instruction.OpCode}.");
            engine.CurrentContext.StaticFields = new Slot(instruction.TokenU8, engine.ReferenceCounter);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void InitSlot(ExecutionEngine engine, Instruction instruction)
        {
            if (engine.CurrentContext!.LocalVariables != null || engine.CurrentContext.Arguments != null)
                throw new InvalidOperationException($"{instruction.OpCode} cannot be executed twice.");
            if (instruction.TokenU16 == 0)
                throw new InvalidOperationException($"The operand {instruction.TokenU16} is invalid for OpCode.{instruction.OpCode}.");
            if (instruction.TokenU8 > 0)
            {
                engine.CurrentContext.LocalVariables = new Slot(instruction.TokenU8, engine.ReferenceCounter);
            }
            if (instruction.TokenU8_1 > 0)
            {
                var items = new StackItem[instruction.TokenU8_1];
                for (var i = 0; i < instruction.TokenU8_1; i++)
                {
                    items[i] = engine.Pop();
                }
                engine.CurrentContext.Arguments = new Slot(items, engine.ReferenceCounter);
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld0(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld1(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld2(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld3(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld4(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld5(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld6(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, 6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdSFld(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.StaticFields, instruction.TokenU8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld0(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld1(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld2(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld3(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld4(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld5(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld6(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, 6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StSFld(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.StaticFields, instruction.TokenU8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc0(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc1(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc2(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc3(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc4(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc5(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc6(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, 6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdLoc(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.LocalVariables, instruction.TokenU8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc0(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc1(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc2(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc3(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc4(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc5(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc6(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, 6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StLoc(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.LocalVariables, instruction.TokenU8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg0(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg1(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg2(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg3(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg4(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg5(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg6(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, 6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void LdArg(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteLoadFromSlot(engine, engine.CurrentContext!.Arguments, instruction.TokenU8);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg0(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 0);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg1(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 1);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg2(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 2);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg3(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 3);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg4(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 4);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg5(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 5);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg6(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, 6);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void StArg(ExecutionEngine engine, Instruction instruction)
        {
            ExecuteStoreToSlot(engine, engine.CurrentContext!.Arguments, instruction.TokenU8);
        }

        #region Execute methods

        public virtual void ExecuteStoreToSlot(ExecutionEngine engine, Slot? slot, int index)
        {
            if (slot is null)
                throw new InvalidOperationException("Slot has not been initialized.");
            if (index < 0 || index >= slot.Count)
                throw new InvalidOperationException($"Index out of range when storing to slot: {index}");
            slot[index] = engine.Pop();
        }

        public virtual void ExecuteLoadFromSlot(ExecutionEngine engine, Slot? slot, int index)
        {
            if (slot is null)
                throw new InvalidOperationException("Slot has not been initialized.");
            if (index < 0 || index >= slot.Count)
                throw new InvalidOperationException($"Index out of range when loading from slot: {index}");
            engine.Push(slot[index]);
        }

        #endregion
    }
}

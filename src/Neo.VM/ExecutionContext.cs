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
// ExecutionContext.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;

namespace Neo.VM
{
    /// <summary>
    /// Represents a frame in the VM execution stack.
    /// </summary>
    [DebuggerDisplay("InstructionPointer={InstructionPointer}")]
    public sealed partial class ExecutionContext
    {
        private readonly SharedStates shared_states;
        private int instructionPointer;

        /// <summary>
        /// Indicates the number of values that the context should return when it is unloaded.
        /// </summary>
        public int RVCount { get; }

        /// <summary>
        /// The script to run in this context.
        /// </summary>
        public Script Script => shared_states.Script;

        /// <summary>
        /// The evaluation stack for this context.
        /// </summary>
        public EvaluationStack EvaluationStack => shared_states.EvaluationStack;

        /// <summary>
        /// The slot used to store the static fields.
        /// </summary>
        public Slot? StaticFields
        {
            get => shared_states.StaticFields;
            internal set => shared_states.StaticFields = value;
        }

        /// <summary>
        /// The slot used to store the local variables of the current method.
        /// </summary>
        public Slot? LocalVariables { get; internal set; }

        /// <summary>
        /// The slot used to store the arguments of the current method.
        /// </summary>
        public Slot? Arguments { get; internal set; }

        /// <summary>
        /// The stack containing nested <see cref="ExceptionHandlingContext"/>.
        /// </summary>
        public Stack<ExceptionHandlingContext>? TryStack { get; internal set; }

        /// <summary>
        /// The pointer indicating the current instruction.
        /// </summary>
        public int InstructionPointer
        {
            get
            {
                return instructionPointer;
            }
            internal set
            {
                if (value < 0 || value > Script.Length)
                    throw new ArgumentOutOfRangeException(nameof(value));
                instructionPointer = value;
            }
        }

        /// <summary>
        /// Returns the current <see cref="Instruction"/>.
        /// </summary>
        public Instruction? CurrentInstruction
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                return GetInstruction(InstructionPointer);
            }
        }

        /// <summary>
        /// Returns the next <see cref="Instruction"/>.
        /// </summary>
        public Instruction? NextInstruction
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get
            {
                Instruction? current = CurrentInstruction;
                if (current is null) return null;
                return GetInstruction(InstructionPointer + current.Size);
            }
        }

        internal ExecutionContext(Script script, int rvcount, ReferenceCounter referenceCounter)
            : this(new SharedStates(script, referenceCounter), rvcount, 0)
        {
        }

        private ExecutionContext(SharedStates shared_states, int rvcount, int initialPosition)
        {
            if (rvcount < -1 || rvcount > ushort.MaxValue)
                throw new ArgumentOutOfRangeException(nameof(rvcount));
            this.shared_states = shared_states;
            this.RVCount = rvcount;
            this.InstructionPointer = initialPosition;
        }

        /// <summary>
        /// Clones the context so that they share the same script, stack, and static fields.
        /// </summary>
        /// <returns>The cloned context.</returns>
        public ExecutionContext Clone()
        {
            return Clone(InstructionPointer);
        }

        /// <summary>
        /// Clones the context so that they share the same script, stack, and static fields.
        /// </summary>
        /// <param name="initialPosition">The instruction pointer of the new context.</param>
        /// <returns>The cloned context.</returns>
        public ExecutionContext Clone(int initialPosition)
        {
            return new ExecutionContext(shared_states, 0, initialPosition);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private Instruction? GetInstruction(int ip) => ip >= Script.Length ? null : Script.GetInstruction(ip);

        /// <summary>
        /// Gets custom data of the specified type. If the data does not exist, create a new one.
        /// </summary>
        /// <typeparam name="T">The type of data to be obtained.</typeparam>
        /// <param name="factory">A delegate used to create the entry. If factory is null, new() will be used.</param>
        /// <returns>The custom data of the specified type.</returns>
        public T GetState<T>(Func<T>? factory = null) where T : class, new()
        {
            if (!shared_states.States.TryGetValue(typeof(T), out object? value))
            {
                value = factory is null ? new T() : factory();
                shared_states.States[typeof(T)] = value;
            }
            return (T)value;
        }

        internal bool MoveNext()
        {
            Instruction? current = CurrentInstruction;
            if (current is null) return false;
            InstructionPointer += current.Size;
            return InstructionPointer < Script.Length;
        }
    }
}

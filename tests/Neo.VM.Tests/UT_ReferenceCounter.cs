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
// UT_ReferenceCounter.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.VM;
using Neo.VM.Types;

namespace Neo.Test
{
    [TestClass]
    public class UT_ReferenceCounter
    {
        [TestMethod]
        public void TestCircularReferences()
        {
            using ScriptBuilder sb = new();
            sb.Emit(OpCode.INITSSLOT, new byte[] { 1 }); //{}|{null}:1
            sb.EmitPush(0); //{0}|{null}:2
            sb.Emit(OpCode.NEWARRAY); //{A[]}|{null}:2
            sb.Emit(OpCode.DUP); //{A[],A[]}|{null}:3
            sb.Emit(OpCode.DUP); //{A[],A[],A[]}|{null}:4
            sb.Emit(OpCode.APPEND); //{A[A]}|{null}:3
            sb.Emit(OpCode.DUP); //{A[A],A[A]}|{null}:4
            sb.EmitPush(0); //{A[A],A[A],0}|{null}:5
            sb.Emit(OpCode.NEWARRAY); //{A[A],A[A],B[]}|{null}:5
            sb.Emit(OpCode.STSFLD0); //{A[A],A[A]}|{B[]}:4
            sb.Emit(OpCode.LDSFLD0); //{A[A],A[A],B[]}|{B[]}:5
            sb.Emit(OpCode.APPEND); //{A[A,B]}|{B[]}:4
            sb.Emit(OpCode.LDSFLD0); //{A[A,B],B[]}|{B[]}:5
            sb.EmitPush(0); //{A[A,B],B[],0}|{B[]}:6
            sb.Emit(OpCode.NEWARRAY); //{A[A,B],B[],C[]}|{B[]}:6
            sb.Emit(OpCode.TUCK); //{A[A,B],C[],B[],C[]}|{B[]}:7
            sb.Emit(OpCode.APPEND); //{A[A,B],C[]}|{B[C]}:6
            sb.EmitPush(0); //{A[A,B],C[],0}|{B[C]}:7
            sb.Emit(OpCode.NEWARRAY); //{A[A,B],C[],D[]}|{B[C]}:7
            sb.Emit(OpCode.TUCK); //{A[A,B],D[],C[],D[]}|{B[C]}:8
            sb.Emit(OpCode.APPEND); //{A[A,B],D[]}|{B[C[D]]}:7
            sb.Emit(OpCode.LDSFLD0); //{A[A,B],D[],B[C]}|{B[C[D]]}:8
            sb.Emit(OpCode.APPEND); //{A[A,B]}|{B[C[D[B]]]}:7
            sb.Emit(OpCode.PUSHNULL); //{A[A,B],null}|{B[C[D[B]]]}:8
            sb.Emit(OpCode.STSFLD0); //{A[A,B[C[D[B]]]]}|{null}:7
            sb.Emit(OpCode.DUP); //{A[A,B[C[D[B]]]],A[A,B]}|{null}:8
            sb.EmitPush(1); //{A[A,B[C[D[B]]]],A[A,B],1}|{null}:9
            sb.Emit(OpCode.REMOVE); //{A[A]}|{null}:3
            sb.Emit(OpCode.STSFLD0); //{}|{A[A]}:2
            sb.Emit(OpCode.RET); //{}:0

            using ExecutionEngine engine = new();
            Debugger debugger = new(engine);
            engine.LoadScript(sb.ToArray());
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(1, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(2, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(2, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(3, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(3, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(5, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(5, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(5, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(5, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(6, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(6, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(7, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(6, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(7, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(7, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(8, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(7, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(8, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(7, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(8, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(7, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(8, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(9, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(6, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(5, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.HALT, debugger.Execute());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
        }

        [TestMethod]
        public void TestRemoveReferrer()
        {
            using ScriptBuilder sb = new();
            sb.Emit(OpCode.INITSSLOT, new byte[] { 1 }); //{}|{null}:1
            sb.EmitPush(0); //{0}|{null}:2
            sb.Emit(OpCode.NEWARRAY); //{A[]}|{null}:2
            sb.Emit(OpCode.DUP); //{A[],A[]}|{null}:3
            sb.EmitPush(0); //{A[],A[],0}|{null}:4
            sb.Emit(OpCode.NEWARRAY); //{A[],A[],B[]}|{null}:4
            sb.Emit(OpCode.STSFLD0); //{A[],A[]}|{B[]}:3
            sb.Emit(OpCode.LDSFLD0); //{A[],A[],B[]}|{B[]}:4
            sb.Emit(OpCode.APPEND); //{A[B]}|{B[]}:3
            sb.Emit(OpCode.DROP); //{}|{B[]}:1
            sb.Emit(OpCode.RET); //{}:0

            using ExecutionEngine engine = new();
            Debugger debugger = new(engine);
            engine.LoadScript(sb.ToArray());
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(1, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(2, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(2, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(3, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(3, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(4, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(3, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(2, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.HALT, debugger.Execute());
            Assert.AreEqual(1, engine.ReferenceCounter.Count);
        }

        [TestMethod]
        public void TestCheckZeroReferredWithArray()
        {
            using ScriptBuilder sb = new();

            sb.EmitPush(ExecutionEngineLimits.Default.MaxStackSize - 1);
            sb.Emit(OpCode.NEWARRAY);

            // Good with MaxStackSize

            using (ExecutionEngine engine = new())
            {
                engine.LoadScript(sb.ToArray());
                Assert.AreEqual(0, engine.ReferenceCounter.Count);

                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual((int)ExecutionEngineLimits.Default.MaxStackSize, engine.ReferenceCounter.Count);
            }

            // Fault with MaxStackSize+1

            sb.Emit(OpCode.PUSH1);

            using (ExecutionEngine engine = new())
            {
                engine.LoadScript(sb.ToArray());
                Assert.AreEqual(0, engine.ReferenceCounter.Count);

                Assert.AreEqual(VMState.FAULT, engine.Execute());
                Assert.AreEqual((int)ExecutionEngineLimits.Default.MaxStackSize + 1, engine.ReferenceCounter.Count);
            }
        }

        [TestMethod]
        public void TestCheckZeroReferred()
        {
            using ScriptBuilder sb = new();

            for (int x = 0; x < ExecutionEngineLimits.Default.MaxStackSize; x++)
                sb.Emit(OpCode.PUSH1);

            // Good with MaxStackSize

            using (ExecutionEngine engine = new())
            {
                engine.LoadScript(sb.ToArray());
                Assert.AreEqual(0, engine.ReferenceCounter.Count);

                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual((int)ExecutionEngineLimits.Default.MaxStackSize, engine.ReferenceCounter.Count);
            }

            // Fault with MaxStackSize+1

            sb.Emit(OpCode.PUSH1);

            using (ExecutionEngine engine = new())
            {
                engine.LoadScript(sb.ToArray());
                Assert.AreEqual(0, engine.ReferenceCounter.Count);

                Assert.AreEqual(VMState.FAULT, engine.Execute());
                Assert.AreEqual((int)ExecutionEngineLimits.Default.MaxStackSize + 1, engine.ReferenceCounter.Count);
            }
        }

        [TestMethod]
        public void TestArrayNoPush()
        {
            using ScriptBuilder sb = new();
            sb.Emit(OpCode.RET);
            using ExecutionEngine engine = new();
            engine.LoadScript(sb.ToArray());
            Assert.AreEqual(0, engine.ReferenceCounter.Count);
            Array array = new(engine.ReferenceCounter, new StackItem[] { 1, 2, 3, 4 });
            Assert.AreEqual(array.Count, engine.ReferenceCounter.Count);
            Assert.AreEqual(VMState.HALT, engine.Execute());
            Assert.AreEqual(array.Count, engine.ReferenceCounter.Count);
        }
    }
}

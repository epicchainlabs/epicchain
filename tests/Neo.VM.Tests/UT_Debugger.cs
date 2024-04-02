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
// UT_Debugger.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.VM;

namespace Neo.Test
{
    [TestClass]
    public class UT_Debugger
    {
        [TestMethod]
        public void TestBreakPoint()
        {
            using ExecutionEngine engine = new();
            using ScriptBuilder script = new();
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);

            engine.LoadScript(script.ToArray());

            Debugger debugger = new(engine);

            Assert.IsFalse(debugger.RemoveBreakPoint(engine.CurrentContext.Script, 3));

            Assert.AreEqual(OpCode.NOP, engine.CurrentContext.NextInstruction.OpCode);

            debugger.AddBreakPoint(engine.CurrentContext.Script, 2);
            debugger.AddBreakPoint(engine.CurrentContext.Script, 3);
            debugger.Execute();
            Assert.AreEqual(OpCode.NOP, engine.CurrentContext.NextInstruction.OpCode);
            Assert.AreEqual(2, engine.CurrentContext.InstructionPointer);
            Assert.AreEqual(VMState.BREAK, engine.State);

            Assert.IsTrue(debugger.RemoveBreakPoint(engine.CurrentContext.Script, 2));
            Assert.IsFalse(debugger.RemoveBreakPoint(engine.CurrentContext.Script, 2));
            Assert.IsTrue(debugger.RemoveBreakPoint(engine.CurrentContext.Script, 3));
            Assert.IsFalse(debugger.RemoveBreakPoint(engine.CurrentContext.Script, 3));
            debugger.Execute();
            Assert.AreEqual(VMState.HALT, engine.State);
        }

        [TestMethod]
        public void TestWithoutBreakPoints()
        {
            using ExecutionEngine engine = new();
            using ScriptBuilder script = new();
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);

            engine.LoadScript(script.ToArray());

            Debugger debugger = new(engine);

            Assert.AreEqual(OpCode.NOP, engine.CurrentContext.NextInstruction.OpCode);

            debugger.Execute();

            Assert.IsNull(engine.CurrentContext);
            Assert.AreEqual(VMState.HALT, engine.State);
        }

        [TestMethod]
        public void TestWithoutDebugger()
        {
            using ExecutionEngine engine = new();
            using ScriptBuilder script = new();
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);
            script.Emit(OpCode.NOP);

            engine.LoadScript(script.ToArray());

            Assert.AreEqual(OpCode.NOP, engine.CurrentContext.NextInstruction.OpCode);

            engine.Execute();

            Assert.IsNull(engine.CurrentContext);
            Assert.AreEqual(VMState.HALT, engine.State);
        }

        [TestMethod]
        public void TestStepOver()
        {
            using ExecutionEngine engine = new();
            using ScriptBuilder script = new();
            /* ┌     CALL
               │  ┌> NOT
               │  │  RET
               └> │  PUSH0
                └─┘  RET */
            script.EmitCall(4);
            script.Emit(OpCode.NOT);
            script.Emit(OpCode.RET);
            script.Emit(OpCode.PUSH0);
            script.Emit(OpCode.RET);

            engine.LoadScript(script.ToArray());

            Debugger debugger = new(engine);

            Assert.AreEqual(OpCode.NOT, engine.CurrentContext.NextInstruction.OpCode);
            Assert.AreEqual(VMState.BREAK, debugger.StepOver());
            Assert.AreEqual(2, engine.CurrentContext.InstructionPointer);
            Assert.AreEqual(VMState.BREAK, engine.State);
            Assert.AreEqual(OpCode.RET, engine.CurrentContext.NextInstruction.OpCode);

            debugger.Execute();

            Assert.AreEqual(true, engine.ResultStack.Pop().GetBoolean());
            Assert.AreEqual(VMState.HALT, engine.State);

            // Test step over again

            Assert.AreEqual(VMState.HALT, debugger.StepOver());
            Assert.AreEqual(VMState.HALT, engine.State);
        }

        [TestMethod]
        public void TestStepInto()
        {
            using ExecutionEngine engine = new();
            using ScriptBuilder script = new();
            /* ┌     CALL
               │  ┌> NOT
               │  │  RET
               └> │  PUSH0
                └─┘  RET */
            script.EmitCall(4);
            script.Emit(OpCode.NOT);
            script.Emit(OpCode.RET);
            script.Emit(OpCode.PUSH0);
            script.Emit(OpCode.RET);

            engine.LoadScript(script.ToArray());

            Debugger debugger = new(engine);

            var context = engine.CurrentContext;

            Assert.AreEqual(context, engine.CurrentContext);
            Assert.AreEqual(context, engine.EntryContext);
            Assert.AreEqual(OpCode.NOT, engine.CurrentContext.NextInstruction.OpCode);

            Assert.AreEqual(VMState.BREAK, debugger.StepInto());

            Assert.AreNotEqual(context, engine.CurrentContext);
            Assert.AreEqual(context, engine.EntryContext);
            Assert.AreEqual(OpCode.RET, engine.CurrentContext.NextInstruction.OpCode);

            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(VMState.BREAK, debugger.StepInto());

            Assert.AreEqual(context, engine.CurrentContext);
            Assert.AreEqual(context, engine.EntryContext);
            Assert.AreEqual(OpCode.RET, engine.CurrentContext.NextInstruction.OpCode);

            Assert.AreEqual(VMState.BREAK, debugger.StepInto());
            Assert.AreEqual(VMState.HALT, debugger.StepInto());

            Assert.AreEqual(true, engine.ResultStack.Pop().GetBoolean());
            Assert.AreEqual(VMState.HALT, engine.State);

            // Test step into again

            Assert.AreEqual(VMState.HALT, debugger.StepInto());
            Assert.AreEqual(VMState.HALT, engine.State);
        }

        [TestMethod]
        public void TestBreakPointStepOver()
        {
            using ExecutionEngine engine = new();
            using ScriptBuilder script = new();
            /* ┌     CALL
               │  ┌> NOT
               │  │  RET
               └>X│  PUSH0
                 └┘  RET */
            script.EmitCall(4);
            script.Emit(OpCode.NOT);
            script.Emit(OpCode.RET);
            script.Emit(OpCode.PUSH0);
            script.Emit(OpCode.RET);

            engine.LoadScript(script.ToArray());

            Debugger debugger = new(engine);

            Assert.AreEqual(OpCode.NOT, engine.CurrentContext.NextInstruction.OpCode);

            debugger.AddBreakPoint(engine.CurrentContext.Script, 5);
            Assert.AreEqual(VMState.BREAK, debugger.StepOver());

            Assert.IsNull(engine.CurrentContext.NextInstruction);
            Assert.AreEqual(5, engine.CurrentContext.InstructionPointer);
            Assert.AreEqual(VMState.BREAK, engine.State);

            debugger.Execute();

            Assert.AreEqual(true, engine.ResultStack.Pop().GetBoolean());
            Assert.AreEqual(VMState.HALT, engine.State);
        }
    }
}

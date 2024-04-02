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
// UT_Script.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.VM;
using System;
using System.Text;

namespace Neo.Test
{
    [TestClass]
    public class UT_Script
    {
        [TestMethod]
        public void TestConversion()
        {
            byte[] rawScript;
            using (var builder = new ScriptBuilder())
            {
                builder.Emit(OpCode.PUSH0);
                builder.Emit(OpCode.CALL, new byte[] { 0x00, 0x01 });
                builder.EmitSysCall(123);

                rawScript = builder.ToArray();
            }

            var script = new Script(rawScript);

            ReadOnlyMemory<byte> scriptConversion = script;
            Assert.AreEqual(rawScript, scriptConversion);
        }

        [TestMethod]
        public void TestStrictMode()
        {
            var rawScript = new byte[] { (byte)OpCode.PUSH0, 0xFF };
            Assert.ThrowsException<BadScriptException>(() => new Script(rawScript, true));

            var script = new Script(rawScript, false);
            Assert.AreEqual(2, script.Length);

            rawScript = new byte[] { (byte)OpCode.PUSHDATA1 };
            Assert.ThrowsException<BadScriptException>(() => new Script(rawScript, true));

            rawScript = new byte[] { (byte)OpCode.PUSHDATA2 };
            Assert.ThrowsException<BadScriptException>(() => new Script(rawScript, true));

            rawScript = new byte[] { (byte)OpCode.PUSHDATA4 };
            Assert.ThrowsException<BadScriptException>(() => new Script(rawScript, true));
        }

        [TestMethod]
        public void TestParse()
        {
            Script script;

            using (var builder = new ScriptBuilder())
            {
                builder.Emit(OpCode.PUSH0);
                builder.Emit(OpCode.CALL_L, new byte[] { 0x00, 0x01, 0x00, 0x00 });
                builder.EmitSysCall(123);

                script = new Script(builder.ToArray());
            }

            Assert.AreEqual(11, script.Length);

            var ins = script.GetInstruction(0);

            Assert.AreEqual(OpCode.PUSH0, ins.OpCode);
            Assert.IsTrue(ins.Operand.IsEmpty);
            Assert.AreEqual(1, ins.Size);
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => { var x = ins.TokenI16; });
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => { var x = ins.TokenU32; });

            ins = script.GetInstruction(1);

            Assert.AreEqual(OpCode.CALL_L, ins.OpCode);
            CollectionAssert.AreEqual(new byte[] { 0x00, 0x01, 0x00, 0x00 }, ins.Operand.ToArray());
            Assert.AreEqual(5, ins.Size);
            Assert.AreEqual(256, ins.TokenI32);
            Assert.AreEqual(Encoding.ASCII.GetString(new byte[] { 0x00, 0x01, 0x00, 0x00 }), ins.TokenString);

            ins = script.GetInstruction(6);

            Assert.AreEqual(OpCode.SYSCALL, ins.OpCode);
            CollectionAssert.AreEqual(new byte[] { 123, 0x00, 0x00, 0x00 }, ins.Operand.ToArray());
            Assert.AreEqual(5, ins.Size);
            Assert.AreEqual(123, ins.TokenI16);
            Assert.AreEqual(Encoding.ASCII.GetString(new byte[] { 123, 0x00, 0x00, 0x00 }), ins.TokenString);
            Assert.AreEqual(123U, ins.TokenU32);

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => script.GetInstruction(100));
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Script.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.VM;
using System;
using System.Text;

namespace EpicChain.Test
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

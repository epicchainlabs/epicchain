// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ExtensiblePayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.VM;
using System;

namespace EpicChain.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_ExtensiblePayload
    {
        [TestMethod]
        public void Size_Get()
        {
            var test = new ExtensiblePayload()
            {
                Sender = Array.Empty<byte>().ToScriptHash(),
                Category = "123",
                Data = new byte[] { 1, 2, 3 },
                Witness = new Witness() { InvocationScript = new byte[] { 3, 5, 6 }, VerificationScript = Array.Empty<byte>() }
            };
            test.Size.Should().Be(42);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = new ExtensiblePayload()
            {
                Category = "123",
                ValidBlockStart = 456,
                ValidBlockEnd = 789,
                Sender = Array.Empty<byte>().ToScriptHash(),
                Data = new byte[] { 1, 2, 3 },
                Witness = new Witness() { InvocationScript = new byte[] { (byte)OpCode.PUSH1, (byte)OpCode.PUSH2, (byte)OpCode.PUSH3 }, VerificationScript = Array.Empty<byte>() }
            };
            var clone = test.ToArray().AsSerializable<ExtensiblePayload>();

            Assert.AreEqual(test.Sender, clone.Witness.ScriptHash);
            Assert.AreEqual(test.Hash, clone.Hash);
            Assert.AreEqual(test.ValidBlockStart, clone.ValidBlockStart);
            Assert.AreEqual(test.ValidBlockEnd, clone.ValidBlockEnd);
            Assert.AreEqual(test.Category, clone.Category);
        }
    }
}

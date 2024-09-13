// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_NefFile.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using System;
using System.IO;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_NefFile
    {
        public NefFile file = new()
        {
            Compiler = "".PadLeft(32, ' '),
            Source = string.Empty,
            Tokens = Array.Empty<MethodToken>(),
            Script = new byte[] { 0x01, 0x02, 0x03 }
        };

        [TestInitialize]
        public void TestSetup()
        {
            file.CheckSum = NefFile.ComputeChecksum(file);
        }

        [TestMethod]
        public void TestDeserialize()
        {
            byte[] wrongMagic = { 0x00, 0x00, 0x00, 0x00 };
            using (MemoryStream ms = new(1024))
            using (BinaryWriter writer = new(ms))
            {
                ((ISerializable)file).Serialize(writer);
                ms.Seek(0, SeekOrigin.Begin);
                ms.Write(wrongMagic, 0, 4);
                ISerializable newFile = new NefFile();
                Assert.ThrowsException<FormatException>(() =>
                {
                    MemoryReader reader = new(ms.ToArray());
                    newFile.Deserialize(ref reader);
                    Assert.Fail();
                });
            }

            file.CheckSum = 0;
            using (MemoryStream ms = new(1024))
            using (BinaryWriter writer = new(ms))
            {
                ((ISerializable)file).Serialize(writer);
                ISerializable newFile = new NefFile();
                Assert.ThrowsException<FormatException>(() =>
                {
                    MemoryReader reader = new(ms.ToArray());
                    newFile.Deserialize(ref reader);
                    Assert.Fail();
                });
            }

            file.Script = Array.Empty<byte>();
            file.CheckSum = NefFile.ComputeChecksum(file);
            using (MemoryStream ms = new(1024))
            using (BinaryWriter writer = new(ms))
            {
                ((ISerializable)file).Serialize(writer);
                ISerializable newFile = new NefFile();
                Assert.ThrowsException<ArgumentException>(() =>
                {
                    MemoryReader reader = new(ms.ToArray());
                    newFile.Deserialize(ref reader);
                    Assert.Fail();
                });
            }

            file.Script = new byte[] { 0x01, 0x02, 0x03 };
            file.CheckSum = NefFile.ComputeChecksum(file);
            var data = file.ToArray();
            var newFile1 = data.AsSerializable<NefFile>();
            newFile1.Compiler.Should().Be(file.Compiler);
            newFile1.CheckSum.Should().Be(file.CheckSum);
            newFile1.Script.Span.SequenceEqual(file.Script.Span).Should().BeTrue();
        }

        [TestMethod]
        public void TestGetSize()
        {
            file.Size.Should().Be(4 + 32 + 32 + 2 + 1 + 2 + 4 + 4);
        }

        [TestMethod]
        public void ParseTest()
        {
            var file = new NefFile()
            {
                Compiler = "".PadLeft(32, ' '),
                Source = string.Empty,
                Tokens = Array.Empty<MethodToken>(),
                Script = new byte[] { 0x01, 0x02, 0x03 }
            };

            file.CheckSum = NefFile.ComputeChecksum(file);

            var data = file.ToArray();
            file = data.AsSerializable<NefFile>();

            Assert.AreEqual("".PadLeft(32, ' '), file.Compiler);
            CollectionAssert.AreEqual(new byte[] { 0x01, 0x02, 0x03 }, file.Script.ToArray());
        }

        [TestMethod]
        public void LimitTest()
        {
            var file = new NefFile()
            {
                Compiler = "".PadLeft(byte.MaxValue, ' '),
                Source = string.Empty,
                Tokens = Array.Empty<MethodToken>(),
                Script = new byte[1024 * 1024],
                CheckSum = 0
            };

            // Wrong compiler

            Assert.ThrowsException<ArgumentException>(() => file.ToArray());

            // Wrong script

            file.Compiler = "";
            file.Script = new byte[(1024 * 1024) + 1];
            var data = file.ToArray();

            Assert.ThrowsException<FormatException>(() => data.AsSerializable<NefFile>());

            // Wrong script hash

            file.Script = new byte[1024 * 1024];
            data = file.ToArray();

            Assert.ThrowsException<FormatException>(() => data.AsSerializable<NefFile>());

            // Wrong checksum

            file.Script = new byte[1024];
            data = file.ToArray();
            file.CheckSum = NefFile.ComputeChecksum(file) + 1;

            Assert.ThrowsException<FormatException>(() => data.AsSerializable<NefFile>());
        }
    }
}

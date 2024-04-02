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
// UT_NefFile.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using Neo.SmartContract;
using System;
using System.IO;

namespace Neo.UnitTests.SmartContract
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

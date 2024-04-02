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
// UT_IOHelper.cs file belongs to the neo project and is free
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
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace Neo.UnitTests.IO
{
    [TestClass]
    public class UT_IOHelper
    {
        [TestMethod]
        public void TestAsSerializableGeneric()
        {
            byte[] caseArray = new byte[] { 0x00,0x00,0x00,0x00,0x00,
                                            0x00,0x00,0x00,0x00,0x00,
                                            0x00,0x00,0x00,0x00,0x00,
                                            0x00,0x00,0x00,0x00,0x00 };
            UInt160 result = Neo.IO.Helper.AsSerializable<UInt160>(caseArray);
            Assert.AreEqual(UInt160.Zero, result);
        }

        [TestMethod]
        public void TestReadFixedBytes()
        {
            byte[] data = new byte[] { 0x01, 0x02, 0x03, 0x04 };

            // Less data

            using (BinaryReader reader = new(new MemoryStream(data), Encoding.UTF8, false))
            {
                byte[] result = Neo.IO.Helper.ReadFixedBytes(reader, 3);

                Assert.AreEqual("010203", result.ToHexString());
                Assert.AreEqual(3, reader.BaseStream.Position);
            }

            // Same data

            using (BinaryReader reader = new(new MemoryStream(data), Encoding.UTF8, false))
            {
                byte[] result = Neo.IO.Helper.ReadFixedBytes(reader, 4);

                Assert.AreEqual("01020304", result.ToHexString());
                Assert.AreEqual(4, reader.BaseStream.Position);
            }

            // More data

            using (BinaryReader reader = new(new MemoryStream(data), Encoding.UTF8, false))
            {
                Assert.ThrowsException<FormatException>(() => Neo.IO.Helper.ReadFixedBytes(reader, 5));
                Assert.AreEqual(4, reader.BaseStream.Position);
            }
        }

        [TestMethod]
        public void TestNullableArray()
        {
            var caseArray = new UInt160[]
            {
                null, UInt160.Zero, new UInt160(
                new byte[] {
                    0xAA,0x00,0x00,0x00,0x00,
                    0xBB,0x00,0x00,0x00,0x00,
                    0xCC,0x00,0x00,0x00,0x00,
                    0xDD,0x00,0x00,0x00,0x00
                })
            };

            byte[] data;
            using (MemoryStream stream = new())
            using (BinaryWriter writter = new(stream))
            {
                Neo.IO.Helper.WriteNullableArray(writter, caseArray);
                data = stream.ToArray();
            }

            // Read Error

            Assert.ThrowsException<FormatException>(() =>
            {
                var reader = new MemoryReader(data);
                reader.ReadNullableArray<UInt160>(2);
                Assert.Fail();
            });

            // Read 100%

            MemoryReader reader = new(data);
            var read = Neo.IO.Helper.ReadNullableArray<UInt160>(ref reader);
            CollectionAssert.AreEqual(caseArray, read);
        }

        [TestMethod]
        public void TestAsSerializable()
        {
            for (int i = 0; i < 2; i++)
            {
                if (i == 0)
                {
                    byte[] caseArray = new byte[] { 0x00,0x00,0x00,0x00,0x00,
                                                    0x00,0x00,0x00,0x00,0x00,
                                                    0x00,0x00,0x00,0x00,0x00,
                                                    0x00,0x00,0x00,0x00,0x00 };
                    ISerializable result = Neo.IO.Helper.AsSerializable(caseArray, typeof(UInt160));
                    Assert.AreEqual(UInt160.Zero, result);
                }
                else
                {
                    Action action = () => Neo.IO.Helper.AsSerializable(Array.Empty<byte>(), typeof(double));
                    action.Should().Throw<InvalidCastException>();
                }
            }
        }

        [TestMethod]
        public void TestCompression()
        {
            var data = new byte[] { 1, 2, 3, 4 };
            var byteArray = Neo.IO.Helper.CompressLz4(data);
            var result = Neo.IO.Helper.DecompressLz4(byteArray.Span, byte.MaxValue);

            CollectionAssert.AreEqual(result, data);

            // Compress

            data = new byte[255];
            for (int x = 0; x < data.Length; x++) data[x] = 1;

            byteArray = Neo.IO.Helper.CompressLz4(data);
            result = Neo.IO.Helper.DecompressLz4(byteArray.Span, byte.MaxValue);

            Assert.IsTrue(byteArray.Length < result.Length);
            CollectionAssert.AreEqual(result, data);

            // Error max length

            Assert.ThrowsException<FormatException>(() => Neo.IO.Helper.DecompressLz4(byteArray.Span, byte.MaxValue - 1));
            Assert.ThrowsException<FormatException>(() => Neo.IO.Helper.DecompressLz4(byteArray.Span, -1));

            // Error length

            byte[] data_wrong = byteArray.ToArray();
            data_wrong[0]++;
            Assert.ThrowsException<FormatException>(() => Neo.IO.Helper.DecompressLz4(data_wrong, byte.MaxValue));
        }

        [TestMethod]
        public void TestAsSerializableArray()
        {
            byte[] byteArray = Neo.IO.Helper.ToByteArray(new UInt160[] { UInt160.Zero });
            UInt160[] result = Neo.IO.Helper.AsSerializableArray<UInt160>(byteArray);
            Assert.AreEqual(1, result.Length);
            Assert.AreEqual(UInt160.Zero, result[0]);
        }

        [TestMethod]
        public void TestGetVarSizeInt()
        {
            for (int i = 0; i < 3; i++)
            {
                if (i == 0)
                {
                    int result = Neo.IO.Helper.GetVarSize(1);
                    Assert.AreEqual(1, result);
                }
                else if (i == 1)
                {
                    int result = Neo.IO.Helper.GetVarSize(0xFFFF);
                    Assert.AreEqual(3, result);
                }
                else
                {
                    int result = Neo.IO.Helper.GetVarSize(0xFFFFFF);
                    Assert.AreEqual(5, result);
                }
            }
        }
        enum TestEnum0 : sbyte
        {
            case1 = 1, case2 = 2
        }

        enum TestEnum1 : byte
        {
            case1 = 1, case2 = 2
        }

        enum TestEnum2 : short
        {
            case1 = 1, case2 = 2
        }

        enum TestEnum3 : ushort
        {
            case1 = 1, case2 = 2
        }

        enum TestEnum4 : int
        {
            case1 = 1, case2 = 2
        }

        enum TestEnum5 : uint
        {
            case1 = 1, case2 = 2
        }

        enum TestEnum6 : long
        {
            case1 = 1, case2 = 2
        }

        [TestMethod]
        public void TestGetVarSizeGeneric()
        {
            for (int i = 0; i < 9; i++)
            {
                if (i == 0)
                {
                    int result = Neo.IO.Helper.GetVarSize(new UInt160[] { UInt160.Zero });
                    Assert.AreEqual(21, result);
                }
                else if (i == 1)//sbyte
                {
                    List<TestEnum0> initList = new()
                    {
                        TestEnum0.case1
                    };
                    IReadOnlyCollection<TestEnum0> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(2, result);
                }
                else if (i == 2)//byte
                {
                    List<TestEnum1> initList = new()
                    {
                        TestEnum1.case1
                    };
                    IReadOnlyCollection<TestEnum1> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(2, result);
                }
                else if (i == 3)//short
                {
                    List<TestEnum2> initList = new()
                    {
                        TestEnum2.case1
                    };
                    IReadOnlyCollection<TestEnum2> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(3, result);
                }
                else if (i == 4)//ushort
                {
                    List<TestEnum3> initList = new()
                    {
                        TestEnum3.case1
                    };
                    IReadOnlyCollection<TestEnum3> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(3, result);
                }
                else if (i == 5)//int
                {
                    List<TestEnum4> initList = new()
                    {
                        TestEnum4.case1
                    };
                    IReadOnlyCollection<TestEnum4> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(5, result);
                }
                else if (i == 6)//uint
                {
                    List<TestEnum5> initList = new()
                    {
                        TestEnum5.case1
                    };
                    IReadOnlyCollection<TestEnum5> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(5, result);
                }
                else if (i == 7)//long
                {
                    List<TestEnum6> initList = new()
                    {
                        TestEnum6.case1
                    };
                    IReadOnlyCollection<TestEnum6> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize(testList);
                    Assert.AreEqual(9, result);
                }
                else if (i == 8)
                {
                    List<int> initList = new()
                    {
                        1
                    };
                    IReadOnlyCollection<int> testList = initList.AsReadOnly();
                    int result = Neo.IO.Helper.GetVarSize<int>(testList);
                    Assert.AreEqual(5, result);
                }
            }
        }

        [TestMethod]
        public void TestGetVarSizeString()
        {
            int result = Neo.IO.Helper.GetVarSize("AA");
            Assert.AreEqual(3, result);
        }

        [TestMethod]
        public void TestReadSerializable()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.Write(writer, UInt160.Zero);
            MemoryReader reader = new(stream.ToArray());
            UInt160 result = Neo.IO.Helper.ReadSerializable<UInt160>(ref reader);
            Assert.AreEqual(UInt160.Zero, result);
        }

        [TestMethod]
        public void TestReadSerializableArray()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.Write(writer, new UInt160[] { UInt160.Zero });
            MemoryReader reader = new(stream.ToArray());
            UInt160[] resultArray = Neo.IO.Helper.ReadSerializableArray<UInt160>(ref reader);
            Assert.AreEqual(1, resultArray.Length);
            Assert.AreEqual(UInt160.Zero, resultArray[0]);
        }

        [TestMethod]
        public void TestReadVarBytes()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.WriteVarBytes(writer, new byte[] { 0xAA, 0xAA });
            stream.Seek(0, SeekOrigin.Begin);
            BinaryReader reader = new(stream);
            byte[] byteArray = Neo.IO.Helper.ReadVarBytes(reader, 10);
            Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0xAA, 0xAA }), Encoding.Default.GetString(byteArray));
        }

        [TestMethod]
        public void TestReadVarInt()
        {
            for (int i = 0; i < 4; i++)
            {
                if (i == 0)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xFFFF);
                    stream.Seek(0, SeekOrigin.Begin);
                    BinaryReader reader = new(stream);
                    ulong result = Neo.IO.Helper.ReadVarInt(reader, 0xFFFF);
                    Assert.AreEqual((ulong)0xFFFF, result);
                }
                else if (i == 1)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xFFFFFFFF);
                    stream.Seek(0, SeekOrigin.Begin);
                    BinaryReader reader = new(stream);
                    ulong result = Neo.IO.Helper.ReadVarInt(reader, 0xFFFFFFFF);
                    Assert.AreEqual(0xFFFFFFFF, result);
                }
                else
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xFFFFFFFFFF);
                    stream.Seek(0, SeekOrigin.Begin);
                    BinaryReader reader = new(stream);
                    Action action = () => Neo.IO.Helper.ReadVarInt(reader, 0xFFFFFFFF);
                    action.Should().Throw<FormatException>();
                }
            }
        }

        [TestMethod]
        public void TestToArray()
        {
            byte[] byteArray = Neo.IO.Helper.ToArray(UInt160.Zero);
            Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0x00,0x00,0x00,0x00,0x00,
                                                                    0x00,0x00,0x00,0x00,0x00,
                                                                    0x00,0x00,0x00,0x00,0x00,
                                                                    0x00,0x00,0x00,0x00,0x00}), Encoding.Default.GetString(byteArray));
        }

        [TestMethod]
        public void TestToByteArrayGeneric()
        {
            byte[] byteArray = Neo.IO.Helper.ToByteArray(new UInt160[] { UInt160.Zero });
            Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0x01,0x00,0x00,0x00,0x00,0x00,
                                                                         0x00,0x00,0x00,0x00,0x00,
                                                                         0x00,0x00,0x00,0x00,0x00,
                                                                         0x00,0x00,0x00,0x00,0x00}), Encoding.Default.GetString(byteArray));
        }

        [TestMethod]
        public void TestWrite()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.Write(writer, UInt160.Zero);
            stream.Seek(0, SeekOrigin.Begin);
            byte[] byteArray = new byte[stream.Length];
            stream.Read(byteArray, 0, (int)stream.Length);
            Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0x00,0x00,0x00,0x00,0x00,
                                                                    0x00,0x00,0x00,0x00,0x00,
                                                                    0x00,0x00,0x00,0x00,0x00,
                                                                    0x00,0x00,0x00,0x00,0x00}), Encoding.Default.GetString(byteArray));
        }

        [TestMethod]
        public void TestWriteGeneric()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.Write(writer, new UInt160[] { UInt160.Zero });
            stream.Seek(0, SeekOrigin.Begin);
            byte[] byteArray = new byte[stream.Length];
            stream.Read(byteArray, 0, (int)stream.Length);
            Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0x01,0x00,0x00,0x00,0x00,0x00,
                                                                         0x00,0x00,0x00,0x00,0x00,
                                                                         0x00,0x00,0x00,0x00,0x00,
                                                                         0x00,0x00,0x00,0x00,0x00}), Encoding.Default.GetString(byteArray));
        }

        [TestMethod]
        public void TestWriteFixedString()
        {
            for (int i = 0; i < 5; i++)
            {
                if (i == 0)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Action action = () => Neo.IO.Helper.WriteFixedString(writer, null, 0);
                    action.Should().Throw<ArgumentNullException>();
                }
                else if (i == 1)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Action action = () => Neo.IO.Helper.WriteFixedString(writer, "AA", Encoding.UTF8.GetBytes("AA").Length - 1);
                    action.Should().Throw<ArgumentException>();
                }
                else if (i == 2)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Action action = () => Neo.IO.Helper.WriteFixedString(writer, "拉拉", Encoding.UTF8.GetBytes("拉拉").Length - 1);
                    action.Should().Throw<ArgumentException>();
                }
                else if (i == 3)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteFixedString(writer, "AA", Encoding.UTF8.GetBytes("AA").Length + 1);
                    stream.Seek(0, SeekOrigin.Begin);
                    byte[] byteArray = new byte[stream.Length];
                    stream.Read(byteArray, 0, (int)stream.Length);
                    byte[] newArray = new byte[Encoding.UTF8.GetBytes("AA").Length + 1];
                    Encoding.UTF8.GetBytes("AA").CopyTo(newArray, 0);
                    Assert.AreEqual(Encoding.Default.GetString(newArray), Encoding.Default.GetString(byteArray));
                }
            }
        }

        [TestMethod]
        public void TestWriteVarBytes()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.WriteVarBytes(writer, new byte[] { 0xAA });
            stream.Seek(0, SeekOrigin.Begin);
            byte[] byteArray = new byte[stream.Length];
            stream.Read(byteArray, 0, (int)stream.Length);
            Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0x01, 0xAA }), Encoding.Default.GetString(byteArray));
        }

        [TestMethod]
        public void TestWriteVarInt()
        {
            for (int i = 0; i < 5; i++)
            {
                if (i == 0)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Action action = () => Neo.IO.Helper.WriteVarInt(writer, -1);
                    action.Should().Throw<ArgumentOutOfRangeException>();
                }
                else if (i == 1)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xFC);
                    stream.Seek(0, SeekOrigin.Begin);
                    byte[] byteArray = new byte[stream.Length];
                    stream.Read(byteArray, 0, (int)stream.Length);
                    Assert.AreEqual(0xFC, byteArray[0]);
                }
                else if (i == 2)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xFFFF);
                    stream.Seek(0, SeekOrigin.Begin);
                    byte[] byteArray = new byte[stream.Length];
                    stream.Read(byteArray, 0, (int)stream.Length);
                    Assert.AreEqual(0xFD, byteArray[0]);
                    Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0xFF, 0xFF }), Encoding.Default.GetString(byteArray.Skip(1).Take(byteArray.Length - 1).ToArray()));
                }
                else if (i == 3)
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xFFFFFFFF);
                    stream.Seek(0, SeekOrigin.Begin);
                    byte[] byteArray = new byte[stream.Length];
                    stream.Read(byteArray, 0, (int)stream.Length);
                    Assert.AreEqual(0xFE, byteArray[0]);
                    Assert.AreEqual(0xFFFFFFFF, BitConverter.ToUInt32(byteArray, 1));
                }
                else
                {
                    MemoryStream stream = new();
                    BinaryWriter writer = new(stream);
                    Neo.IO.Helper.WriteVarInt(writer, 0xAEFFFFFFFF);
                    stream.Seek(0, SeekOrigin.Begin);
                    byte[] byteArray = new byte[stream.Length];
                    stream.Read(byteArray, 0, (int)stream.Length);
                    Assert.AreEqual(0xFF, byteArray[0]);
                    Assert.AreEqual(Encoding.Default.GetString(new byte[] { 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00 }), Encoding.Default.GetString(byteArray.Skip(1).Take(byteArray.Length - 1).ToArray()));
                }
            }
        }

        [TestMethod]
        public void TestWriteVarString()
        {
            MemoryStream stream = new();
            BinaryWriter writer = new(stream);
            Neo.IO.Helper.WriteVarString(writer, "a");
            stream.Seek(0, SeekOrigin.Begin);
            byte[] byteArray = new byte[stream.Length];
            stream.Read(byteArray, 0, (int)stream.Length);
            Assert.AreEqual(0x01, byteArray[0]);
            Assert.AreEqual(0x61, byteArray[1]);
        }
    }
}

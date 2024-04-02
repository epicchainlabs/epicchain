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
// UT_Helper.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO.Caching;
using Neo.Network.P2P;
using Neo.SmartContract;
using Neo.Wallets;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Numerics;

namespace Neo.UnitTests
{
    [TestClass]
    public class UT_Helper
    {
        [TestMethod]
        public void GetSignData()
        {
            TestVerifiable verifiable = new();
            byte[] res = verifiable.GetSignData(TestProtocolSettings.Default.Network);
            res.ToHexString().Should().Be("4e454f3350b51da6bb366be3ea50140cda45ba7df575287c0371000b2037ed3898ff8bf5");
        }

        [TestMethod]
        public void Sign()
        {
            TestVerifiable verifiable = new();
            byte[] res = verifiable.Sign(new KeyPair(TestUtils.GetByteArray(32, 0x42)), TestProtocolSettings.Default.Network);
            res.Length.Should().Be(64);
        }

        [TestMethod]
        public void ToScriptHash()
        {
            byte[] testByteArray = TestUtils.GetByteArray(64, 0x42);
            UInt160 res = testByteArray.ToScriptHash();
            res.Should().Be(UInt160.Parse("2d3b96ae1bcc5a585e075e3b81920210dec16302"));
        }

        [TestMethod]
        public void TestGetLowestSetBit()
        {
            var big1 = new BigInteger(0);
            big1.GetLowestSetBit().Should().Be(-1);

            var big2 = new BigInteger(512);
            big2.GetLowestSetBit().Should().Be(9);

            var big3 = new BigInteger(int.MinValue);
            big3.GetLowestSetBit().Should().Be(31);

            var big4 = new BigInteger(long.MinValue);
            big4.GetLowestSetBit().Should().Be(63);
        }

        [TestMethod]
        public void TestHexToBytes()
        {
            string nullStr = null;
            _ = nullStr.HexToBytes().ToHexString().Should().Be(Array.Empty<byte>().ToHexString());
            string emptyStr = "";
            emptyStr.HexToBytes().ToHexString().Should().Be(Array.Empty<byte>().ToHexString());
            string str1 = "hab";
            Action action = () => str1.HexToBytes();
            action.Should().Throw<FormatException>();
            string str2 = "0102";
            byte[] bytes = str2.HexToBytes();
            bytes.ToHexString().Should().Be(new byte[] { 0x01, 0x02 }.ToHexString());
        }

        [TestMethod]
        public void TestRemoveHashsetDictionary()
        {
            var a = new HashSet<int>
            {
                1,
                2,
                3
            };

            var b = new Dictionary<int, object>
            {
                [2] = null
            };

            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 1, 3 }, a.ToArray());

            b[4] = null;
            b[5] = null;
            b[1] = null;
            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 3 }, a.ToArray());
        }

        [TestMethod]
        public void TestRemoveHashsetSet()
        {
            var a = new HashSet<int>
            {
                1,
                2,
                3
            };

            var b = new SortedSet<int>()
            {
                2
            };

            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 1, 3 }, a.ToArray());

            b.Add(4);
            b.Add(5);
            b.Add(1);
            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 3 }, a.ToArray());
        }

        [TestMethod]
        public void TestRemoveHashsetHashSetCache()
        {
            var a = new HashSet<int>
            {
                1,
                2,
                3
            };

            var b = new HashSetCache<int>(10)
            {
                2
            };

            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 1, 3 }, a.ToArray());

            b.Add(4);
            b.Add(5);
            b.Add(1);
            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 3 }, a.ToArray());
        }

        [TestMethod]
        public void TestToHexString()
        {
            byte[] nullStr = null;
            Assert.ThrowsException<NullReferenceException>(() => nullStr.ToHexString());
            byte[] empty = Array.Empty<byte>();
            empty.ToHexString().Should().Be("");
            empty.ToHexString(false).Should().Be("");
            empty.ToHexString(true).Should().Be("");

            byte[] str1 = new byte[] { (byte)'n', (byte)'e', (byte)'o' };
            str1.ToHexString().Should().Be("6e656f");
            str1.ToHexString(false).Should().Be("6e656f");
            str1.ToHexString(true).Should().Be("6f656e");
        }

        [TestMethod]
        public void TestGetVersion()
        {
            // assembly without version

            var asm = AppDomain.CurrentDomain.GetAssemblies()
                .Where(u => u.FullName == "Anonymously Hosted DynamicMethods Assembly, Version=0.0.0.0, Culture=neutral, PublicKeyToken=null")
                .FirstOrDefault();
            string version = asm?.GetVersion() ?? "";
            version.Should().Be("0.0.0");
        }

        [TestMethod]
        public void TestToByteArrayStandard()
        {
            BigInteger number = BigInteger.Zero;
            Assert.AreEqual("", number.ToByteArrayStandard().ToHexString());

            number = BigInteger.One;
            Assert.AreEqual("01", number.ToByteArrayStandard().ToHexString());
        }

        [TestMethod]
        public void TestNextBigIntegerForRandom()
        {
            Random ran = new();
            Action action1 = () => ran.NextBigInteger(-1);
            action1.Should().Throw<ArgumentException>();

            ran.NextBigInteger(0).Should().Be(0);
            ran.NextBigInteger(8).Should().NotBeNull();
            ran.NextBigInteger(9).Should().NotBeNull();
        }

        [TestMethod]
        public void TestUnmapForIPAddress()
        {
            var addr = new IPAddress(new byte[] { 127, 0, 0, 1 });
            addr.Unmap().Should().Be(addr);

            var addr2 = addr.MapToIPv6();
            addr2.Unmap().Should().Be(addr);
        }

        [TestMethod]
        public void TestUnmapForIPEndPoin()
        {
            var addr = new IPAddress(new byte[] { 127, 0, 0, 1 });
            var endPoint = new IPEndPoint(addr, 8888);
            endPoint.Unmap().Should().Be(endPoint);

            var addr2 = addr.MapToIPv6();
            var endPoint2 = new IPEndPoint(addr2, 8888);
            endPoint2.Unmap().Should().Be(endPoint);
        }
    }
}

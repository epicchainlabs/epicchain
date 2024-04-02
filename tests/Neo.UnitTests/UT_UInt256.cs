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
// UT_UInt256.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

#pragma warning disable CS1718

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using System;
using System.IO;

namespace Neo.UnitTests.IO
{
    [TestClass]
    public class UT_UInt256
    {
        [TestMethod]
        public void TestFail()
        {
            Assert.ThrowsException<FormatException>(() => new UInt256(new byte[UInt256.Length + 1]));
        }

        [TestMethod]
        public void TestGernerator1()
        {
            UInt256 uInt256 = new();
            Assert.IsNotNull(uInt256);
        }

        [TestMethod]
        public void TestGernerator2()
        {
            UInt256 uInt256 = new(new byte[32]);
            Assert.IsNotNull(uInt256);
        }

        [TestMethod]
        public void TestCompareTo()
        {
            byte[] temp = new byte[32];
            temp[31] = 0x01;
            UInt256 result = new(temp);
            Assert.AreEqual(0, UInt256.Zero.CompareTo(UInt256.Zero));
            Assert.AreEqual(-1, UInt256.Zero.CompareTo(result));
            Assert.AreEqual(1, result.CompareTo(UInt256.Zero));
        }

        [TestMethod]
        public void TestDeserialize()
        {
            using MemoryStream stream = new();
            using BinaryWriter writer = new(stream);
            writer.Write(new byte[20]);
            UInt256 uInt256 = new();
            Assert.ThrowsException<FormatException>(() =>
            {
                MemoryReader reader = new(stream.ToArray());
                ((ISerializable)uInt256).Deserialize(ref reader);
            });
        }

        [TestMethod]
        public void TestEquals()
        {
            byte[] temp = new byte[32];
            temp[31] = 0x01;
            UInt256 result = new(temp);
            Assert.AreEqual(true, UInt256.Zero.Equals(UInt256.Zero));
            Assert.AreEqual(false, UInt256.Zero.Equals(result));
            Assert.AreEqual(false, result.Equals(null));
        }

        [TestMethod]
        public void TestEquals1()
        {
            UInt256 temp1 = new();
            UInt256 temp2 = new();
            UInt160 temp3 = new();
            Assert.AreEqual(false, temp1.Equals(null));
            Assert.AreEqual(true, temp1.Equals(temp1));
            Assert.AreEqual(true, temp1.Equals(temp2));
            Assert.AreEqual(false, temp1.Equals(temp3));
        }

        [TestMethod]
        public void TestEquals2()
        {
            UInt256 temp1 = new();
            object temp2 = null;
            object temp3 = new();
            Assert.AreEqual(false, temp1.Equals(temp2));
            Assert.AreEqual(false, temp1.Equals(temp3));
        }

        [TestMethod]
        public void TestParse()
        {
            Action action = () => UInt256.Parse(null);
            action.Should().Throw<FormatException>();
            UInt256 result = UInt256.Parse("0x0000000000000000000000000000000000000000000000000000000000000000");
            Assert.AreEqual(UInt256.Zero, result);
            Action action1 = () => UInt256.Parse("000000000000000000000000000000000000000000000000000000000000000");
            action1.Should().Throw<FormatException>();
            UInt256 result1 = UInt256.Parse("0000000000000000000000000000000000000000000000000000000000000000");
            Assert.AreEqual(UInt256.Zero, result1);
        }

        [TestMethod]
        public void TestTryParse()
        {
            Assert.AreEqual(false, UInt256.TryParse(null, out _));
            Assert.AreEqual(true, UInt256.TryParse("0x0000000000000000000000000000000000000000000000000000000000000000", out var temp));
            Assert.AreEqual(UInt256.Zero, temp);
            Assert.AreEqual(true, UInt256.TryParse("0x1230000000000000000000000000000000000000000000000000000000000000", out temp));
            Assert.AreEqual("0x1230000000000000000000000000000000000000000000000000000000000000", temp.ToString());
            Assert.AreEqual(false, UInt256.TryParse("000000000000000000000000000000000000000000000000000000000000000", out _));
            Assert.AreEqual(false, UInt256.TryParse("0xKK00000000000000000000000000000000000000000000000000000000000000", out _));
        }

        [TestMethod]
        public void TestOperatorEqual()
        {
            Assert.IsFalse(new UInt256() == null);
            Assert.IsFalse(null == new UInt256());
        }

        [TestMethod]
        public void TestOperatorLarger()
        {
            Assert.AreEqual(false, UInt256.Zero > UInt256.Zero);
        }

        [TestMethod]
        public void TestOperatorLargerAndEqual()
        {
            Assert.AreEqual(true, UInt256.Zero >= UInt256.Zero);
        }

        [TestMethod]
        public void TestOperatorSmaller()
        {
            Assert.AreEqual(false, UInt256.Zero < UInt256.Zero);
        }

        [TestMethod]
        public void TestOperatorSmallerAndEqual()
        {
            Assert.AreEqual(true, UInt256.Zero <= UInt256.Zero);
        }
    }
}

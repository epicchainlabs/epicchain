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
// UT_ECFieldElement.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography.ECC;
using System;
using System.Globalization;
using System.Numerics;
using System.Reflection;

namespace Neo.UnitTests.Cryptography.ECC
{
    [TestClass]
    public class UT_ECFieldElement
    {
        [TestMethod]
        public void TestECFieldElementConstructor()
        {
            BigInteger input = new(100);
            Action action = () => new ECFieldElement(input, ECCurve.Secp256k1);
            action.Should().NotThrow();

            input = ECCurve.Secp256k1.Q;
            action = () => new ECFieldElement(input, ECCurve.Secp256k1);
            action.Should().Throw<ArgumentException>();

            action = () => new ECFieldElement(input, null);
            action.Should().Throw<ArgumentNullException>();
        }

        [TestMethod]
        public void TestCompareTo()
        {
            ECFieldElement X1 = new(new BigInteger(100), ECCurve.Secp256k1);
            ECFieldElement Y1 = new(new BigInteger(200), ECCurve.Secp256k1);
            ECFieldElement X2 = new(new BigInteger(300), ECCurve.Secp256k1);
            ECFieldElement Y2 = new(new BigInteger(400), ECCurve.Secp256k1);
            ECFieldElement X3 = new(new BigInteger(100), ECCurve.Secp256r1);
            ECFieldElement Y3 = new(new BigInteger(400), ECCurve.Secp256r1);
            ECPoint point1 = new(X1, Y1, ECCurve.Secp256k1);
            ECPoint point2 = new(X2, Y1, ECCurve.Secp256k1);
            ECPoint point3 = new(X1, Y2, ECCurve.Secp256k1);
            ECPoint point4 = new(X3, Y3, ECCurve.Secp256r1);

            point1.CompareTo(point1).Should().Be(0);
            point1.CompareTo(point2).Should().Be(-1);
            point2.CompareTo(point1).Should().Be(1);
            point1.CompareTo(point3).Should().Be(-1);
            point3.CompareTo(point1).Should().Be(1);
            Action action = () => point3.CompareTo(point4);
            action.Should().Throw<InvalidOperationException>();
        }

        [TestMethod]
        public void TestEquals()
        {
            BigInteger input = new(100);
            object element = new ECFieldElement(input, ECCurve.Secp256k1);
            element.Equals(element).Should().BeTrue();
            element.Equals(1).Should().BeFalse();
            element.Equals(new ECFieldElement(input, ECCurve.Secp256r1)).Should().BeFalse();

            input = new BigInteger(200);
            element.Equals(new ECFieldElement(input, ECCurve.Secp256k1)).Should().BeFalse();
        }

        [TestMethod]
        public void TestSqrt()
        {
            ECFieldElement element = new(new BigInteger(100), ECCurve.Secp256k1);
            element.Sqrt().Should().Be(new ECFieldElement(BigInteger.Parse("115792089237316195423570985008687907853269984665640564039457584007908834671653"), ECCurve.Secp256k1));

            ConstructorInfo constructor = typeof(ECCurve).GetConstructor(BindingFlags.NonPublic | BindingFlags.Instance, null, new Type[] { typeof(BigInteger), typeof(BigInteger), typeof(BigInteger), typeof(BigInteger), typeof(byte[]) }, null);
            ECCurve testCruve = constructor.Invoke(new object[] {
                BigInteger.Parse("00FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFF0", NumberStyles.AllowHexSpecifier),
                BigInteger.Parse("00FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFF00", NumberStyles.AllowHexSpecifier),
                BigInteger.Parse("005AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B", NumberStyles.AllowHexSpecifier),
                BigInteger.Parse("00FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551", NumberStyles.AllowHexSpecifier),
                ("04" + "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296" + "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5").HexToBytes() }) as ECCurve;
            element = new ECFieldElement(new BigInteger(200), testCruve);
            element.Sqrt().Should().Be(null);
        }

        [TestMethod]
        public void TestToByteArray()
        {
            byte[] result = new byte[32];
            result[31] = 100;
            new ECFieldElement(new BigInteger(100), ECCurve.Secp256k1).ToByteArray().Should().BeEquivalentTo(result);

            byte[] result2 = { 2, 53, 250, 221, 129, 194, 130, 43, 179, 240, 120, 119, 151, 61, 80, 242, 139, 242, 42, 49, 190, 142, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
            new ECFieldElement(BigInteger.Pow(new BigInteger(10), 75), ECCurve.Secp256k1).ToByteArray().Should().BeEquivalentTo(result2);

            byte[] result3 = { 221, 21, 254, 134, 175, 250, 217, 18, 73, 239, 14, 183, 19, 243, 158, 190, 170, 152, 123, 110, 111, 210, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
            new ECFieldElement(BigInteger.Pow(new BigInteger(10), 77), ECCurve.Secp256k1).ToByteArray().Should().BeEquivalentTo(result3);
        }
    }
}

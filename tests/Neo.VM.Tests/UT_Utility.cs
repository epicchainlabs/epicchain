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
// UT_Utility.cs file belongs to the neo project and is free
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
using System.Numerics;

namespace Neo.Test
{
    [TestClass]
    public class UT_Utility
    {
        [TestMethod]
        public void TestSqrtTest()
        {
            Assert.ThrowsException<InvalidOperationException>(() => BigInteger.MinusOne.Sqrt());

            Assert.AreEqual(BigInteger.Zero, BigInteger.Zero.Sqrt());
            Assert.AreEqual(new BigInteger(1), new BigInteger(1).Sqrt());
            Assert.AreEqual(new BigInteger(1), new BigInteger(2).Sqrt());
            Assert.AreEqual(new BigInteger(1), new BigInteger(3).Sqrt());
            Assert.AreEqual(new BigInteger(2), new BigInteger(4).Sqrt());
            Assert.AreEqual(new BigInteger(9), new BigInteger(81).Sqrt());
        }

        private static byte[] GetRandomByteArray(Random random)
        {
            var byteValue = random.Next(0, 32);
            var value = new byte[byteValue];

            random.NextBytes(value);
            return value;
        }

        private void VerifyGetBitLength(BigInteger value, long expected)
        {
            var result = value.GetBitLength();
            Assert.AreEqual(expected, value.GetBitLength(), "Native method has not the expected result");
            Assert.AreEqual(result, Utility.GetBitLength(value), "Result doesn't match");
        }

        [TestMethod]
        public void TestGetBitLength()
        {
            var random = new Random();

            // Big Number (net standard didn't work)
            VerifyGetBitLength(BigInteger.One << 32 << int.MaxValue, 2147483680);

            // Trivial cases
            //                     sign bit|shortest two's complement
            //                              string w/o sign bit
            VerifyGetBitLength(0, 0);  // 0|
            VerifyGetBitLength(1, 1);  // 0|1
            VerifyGetBitLength(-1, 0); // 1|
            VerifyGetBitLength(2, 2);  // 0|10
            VerifyGetBitLength(-2, 1); // 1|0
            VerifyGetBitLength(3, 2);  // 0|11
            VerifyGetBitLength(-3, 2); // 1|01
            VerifyGetBitLength(4, 3);  // 0|100
            VerifyGetBitLength(-4, 2); // 1|00
            VerifyGetBitLength(5, 3);  // 0|101
            VerifyGetBitLength(-5, 3); // 1|011
            VerifyGetBitLength(6, 3);  // 0|110
            VerifyGetBitLength(-6, 3); // 1|010
            VerifyGetBitLength(7, 3);  // 0|111
            VerifyGetBitLength(-7, 3); // 1|001
            VerifyGetBitLength(8, 4);  // 0|1000
            VerifyGetBitLength(-8, 3); // 1|000

            // Random cases
            for (uint i = 0; i < 1000; i++)
            {
                var bi = new BigInteger(GetRandomByteArray(random));
                Assert.AreEqual(bi.GetBitLength(), Utility.GetBitLength(bi), message: $"Error comparing: {bi}");
            }

            foreach (var bi in new[] { BigInteger.Zero, BigInteger.One, BigInteger.MinusOne, new BigInteger(ulong.MaxValue), new BigInteger(long.MinValue) })
            {
                Assert.AreEqual(bi.GetBitLength(), Utility.GetBitLength(bi), message: $"Error comparing: {bi}");
            }
        }

        [TestMethod]
        public void TestModInverseTest()
        {
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => BigInteger.One.ModInverse(BigInteger.Zero));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => BigInteger.One.ModInverse(BigInteger.One));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => BigInteger.Zero.ModInverse(BigInteger.Zero));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => BigInteger.Zero.ModInverse(BigInteger.One));
            Assert.ThrowsException<InvalidOperationException>(() => new BigInteger(ushort.MaxValue).ModInverse(byte.MaxValue));

            Assert.AreEqual(new BigInteger(52), new BigInteger(19).ModInverse(141));
        }
    }
}

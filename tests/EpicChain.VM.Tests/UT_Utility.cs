// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Utility.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Numerics;

namespace EpicChain.Test
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

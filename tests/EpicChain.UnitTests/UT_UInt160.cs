// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_UInt160.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.Security.Cryptography;

namespace EpicChain.UnitTests.IO
{
    [TestClass]
    public class UT_UInt160
    {
        [TestMethod]
        public void TestFail()
        {
            Assert.ThrowsException<FormatException>(() => new UInt160(new byte[UInt160.Length + 1]));
        }

        [TestMethod]
        public void TestGernerator1()
        {
            UInt160 uInt160 = new UInt160();
            Assert.IsNotNull(uInt160);
        }

        [TestMethod]
        public void TestGernerator2()
        {
            UInt160 uInt160 = new byte[20];
            Assert.IsNotNull(uInt160);
        }

        [TestMethod]
        public void TestGernerator3()
        {
            UInt160 uInt160 = "0xff00000000000000000000000000000000000001";
            Assert.IsNotNull(uInt160);
            Assert.IsTrue(uInt160.ToString() == "0xff00000000000000000000000000000000000001");
        }

        [TestMethod]
        public void TestCompareTo()
        {
            byte[] temp = new byte[20];
            temp[19] = 0x01;
            UInt160 result = new UInt160(temp);
            Assert.AreEqual(0, UInt160.Zero.CompareTo(UInt160.Zero));
            Assert.AreEqual(-1, UInt160.Zero.CompareTo(result));
            Assert.AreEqual(1, result.CompareTo(UInt160.Zero));
        }

        [TestMethod]
        public void TestEquals()
        {
            byte[] temp = new byte[20];
            temp[19] = 0x01;
            UInt160 result = new UInt160(temp);
            Assert.IsTrue(UInt160.Zero.Equals(UInt160.Zero));
            Assert.IsFalse(UInt160.Zero.Equals(result));
            Assert.IsFalse(result.Equals(null));
            Assert.IsTrue(UInt160.Zero == UInt160.Zero);
            Assert.IsFalse(UInt160.Zero != UInt160.Zero);
            Assert.IsTrue(UInt160.Zero == "0x0000000000000000000000000000000000000000");
            Assert.IsFalse(UInt160.Zero == "0x0000000000000000000000000000000000000001");
        }

        [TestMethod]
        public void TestParse()
        {
            Action action = () => UInt160.Parse(null);
            action.Should().Throw<FormatException>();
            UInt160 result = UInt160.Parse("0x0000000000000000000000000000000000000000");
            Assert.AreEqual(UInt160.Zero, result);
            Action action1 = () => UInt160.Parse("000000000000000000000000000000000000000");
            action1.Should().Throw<FormatException>();
            UInt160 result1 = UInt160.Parse("0000000000000000000000000000000000000000");
            Assert.AreEqual(UInt160.Zero, result1);
        }

        [TestMethod]
        public void TestTryParse()
        {
            Assert.AreEqual(false, UInt160.TryParse(null, out _));
            Assert.AreEqual(true, UInt160.TryParse("0x0000000000000000000000000000000000000000", out var temp));
            Assert.AreEqual("0x0000000000000000000000000000000000000000", temp.ToString());
            Assert.AreEqual(UInt160.Zero, temp);
            Assert.AreEqual(true, UInt160.TryParse("0x1230000000000000000000000000000000000000", out temp));
            Assert.AreEqual("0x1230000000000000000000000000000000000000", temp.ToString());
            Assert.AreEqual(false, UInt160.TryParse("000000000000000000000000000000000000000", out _));
            Assert.AreEqual(false, UInt160.TryParse("0xKK00000000000000000000000000000000000000", out _));
        }

        [TestMethod]
        public void TestOperatorLarger()
        {
            Assert.AreEqual(false, UInt160.Zero > UInt160.Zero);
            Assert.IsFalse(UInt160.Zero > "0x0000000000000000000000000000000000000000");
        }

        [TestMethod]
        public void TestOperatorLargerAndEqual()
        {
            Assert.AreEqual(true, UInt160.Zero >= UInt160.Zero);
            Assert.IsTrue(UInt160.Zero >= "0x0000000000000000000000000000000000000000");
        }

        [TestMethod]
        public void TestOperatorSmaller()
        {
            Assert.AreEqual(false, UInt160.Zero < UInt160.Zero);
            Assert.IsFalse(UInt160.Zero < "0x0000000000000000000000000000000000000000");
        }

        [TestMethod]
        public void TestOperatorSmallerAndEqual()
        {
            Assert.AreEqual(true, UInt160.Zero <= UInt160.Zero);
            Assert.IsTrue(UInt160.Zero >= "0x0000000000000000000000000000000000000000");
        }
    }
}

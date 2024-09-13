// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ByteArrayEqualityComparer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO;
using System;
using System.Linq;

namespace EpicChain.UnitTests
{
    [TestClass]
    public class UT_ByteArrayEqualityComparer
    {
        [TestMethod]
        public void TestEqual()
        {
            var a = new byte[] { 1, 2, 3, 4, 1, 2, 3, 4, 5 };
            var b = new byte[] { 1, 2, 3, 4, 1, 2, 3, 4, 5 };
            var check = ByteArrayEqualityComparer.Default;

            Assert.IsTrue(check.Equals(a, a));
            Assert.IsTrue(check.Equals(a, b));
            Assert.IsFalse(check.Equals(null, b));
            Assert.IsFalse(check.Equals(a, null));
            Assert.IsTrue(check.Equals(null, null));

            Assert.IsFalse(check.Equals(a, new byte[] { 1, 2, 3 }));
            Assert.IsTrue(check.Equals(Array.Empty<byte>(), Array.Empty<byte>()));

            b[8]++;
            Assert.IsFalse(check.Equals(a, b));
            b[8]--;
            b[0]--;
            Assert.IsFalse(check.Equals(a, b));
        }

        [TestMethod]
        public void TestGetHashCode()
        {
            var a = new byte[] { 1, 2, 3, 4, 1, 2, 3, 4, 5 };
            var b = new byte[] { 1, 2, 3, 4, 1, 2, 3, 4, 5 };
            var check = ByteArrayEqualityComparer.Default;

            Assert.AreEqual(check.GetHashCode(a), check.GetHashCode(b));
            Assert.AreNotEqual(check.GetHashCode(a), check.GetHashCode(b.Take(8).ToArray()));
        }
    }
}

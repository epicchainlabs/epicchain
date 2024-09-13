// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ByteArrayComparer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;

namespace EpicChain.UnitTests.IO
{
    [TestClass]
    public class UT_ByteArrayComparer
    {
        [TestMethod]
        public void TestCompare()
        {
            ByteArrayComparer comparer = ByteArrayComparer.Default;
            byte[] x = null, y = null;
            comparer.Compare(x, y).Should().Be(0);

            x = new byte[] { 1, 2, 3, 4, 5 };
            y = x;
            comparer.Compare(x, y).Should().Be(0);
            comparer.Compare(x, x).Should().Be(0);

            y = null;
            comparer.Compare(x, y).Should().Be(5);

            y = x;
            x = null;
            comparer.Compare(x, y).Should().Be(-5);

            x = new byte[] { 1 };
            y = Array.Empty<byte>();
            comparer.Compare(x, y).Should().Be(1);
            y = x;
            comparer.Compare(x, y).Should().Be(0);

            x = new byte[] { 1 };
            y = new byte[] { 2 };
            comparer.Compare(x, y).Should().Be(-1);

            comparer = ByteArrayComparer.Reverse;
            x = new byte[] { 3 };
            comparer.Compare(x, y).Should().Be(-1);
            y = x;
            comparer.Compare(x, y).Should().Be(0);

            x = new byte[] { 1 };
            y = new byte[] { 2 };
            comparer.Compare(x, y).Should().Be(1);
        }
    }
}

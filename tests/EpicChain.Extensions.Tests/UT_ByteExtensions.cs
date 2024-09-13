// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ByteExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;

namespace EpicChain.Extensions.Tests
{
    [TestClass]
    public class UT_ByteExtensions
    {
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
        public void TestReadOnlySpanToHexString()
        {
            byte[] input = { 0x0F, 0xA4, 0x3B };
            var span = new ReadOnlySpan<byte>(input);
            string result = span.ToHexString();
            result.Should().Be("0fa43b");

            input = Array.Empty<byte>();
            span = new ReadOnlySpan<byte>(input);
            result = span.ToHexString();
            result.Should().BeEmpty();

            input = new byte[] { 0x5A };
            span = new ReadOnlySpan<byte>(input);
            result = span.ToHexString();
            result.Should().Be("5a");

            input = new byte[] { 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF };
            span = new ReadOnlySpan<byte>(input);
            result = span.ToHexString();
            result.Should().Be("0123456789abcdef");
        }
    }
}

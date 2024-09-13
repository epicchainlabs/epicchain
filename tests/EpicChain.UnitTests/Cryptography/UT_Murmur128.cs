// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Murmur128.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography;
using EpicChain.Extensions;
using System.Text;

namespace EpicChain.UnitTests.Cryptography
{
    [TestClass]
    public class UT_Murmur128
    {
        [TestMethod]
        public void TestGetHashSize()
        {
            Murmur128 murmur128 = new Murmur128(1);
            murmur128.HashSize.Should().Be(128);
        }

        [TestMethod]
        public void TestHashCore()
        {
            byte[] array = Encoding.ASCII.GetBytes("hello");
            array.Murmur128(123u).ToHexString().ToString().Should().Be("0bc59d0ad25fde2982ed65af61227a0e");

            array = Encoding.ASCII.GetBytes("world");
            array.Murmur128(123u).ToHexString().ToString().Should().Be("3d3810fed480472bd214a14023bb407f");

            array = Encoding.ASCII.GetBytes("hello world");
            array.Murmur128(123u).ToHexString().ToString().Should().Be("e0a0632d4f51302c55e3b3e48d28795d");

            array = "718f952132679baa9c5c2aa0d329fd2a".HexToBytes();
            array.Murmur128(123u).ToHexString().ToString().Should().Be("9b4aa747ff0cf4e41b3d96251551c8ae");
        }
    }
}

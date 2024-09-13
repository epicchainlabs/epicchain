// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Wallets_Helper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Neo.Cryptography;
using Neo.IO;
using Neo.Wallets;
using System;

namespace Neo.UnitTests.Wallets
{
    [TestClass]
    public class UT_Wallets_Helper
    {
        [TestMethod]
        public void TestToScriptHash()
        {
            byte[] array = { 0x01 };
            UInt160 scriptHash = new UInt160(Crypto.Hash160(array));
            "NdtB8RXRmJ7Nhw1FPTm7E6HoDZGnDw37nf".ToScriptHash(TestProtocolSettings.Default.AddressVersion).Should().Be(scriptHash);

            Action action = () => "3vQB7B6MrGQZaxCuFg4oh".ToScriptHash(TestProtocolSettings.Default.AddressVersion);
            action.Should().Throw<FormatException>();

            var address = scriptHash.ToAddress(ProtocolSettings.Default.AddressVersion);
            Span<byte> data = stackalloc byte[21];
            // EpicChain version is 0x17
            data[0] = 0x01;
            scriptHash.ToArray().CopyTo(data[1..]);
            address = Base58.Base58CheckEncode(data);
            action = () => address.ToScriptHash(ProtocolSettings.Default.AddressVersion);
            action.Should().Throw<FormatException>();
        }
    }
}

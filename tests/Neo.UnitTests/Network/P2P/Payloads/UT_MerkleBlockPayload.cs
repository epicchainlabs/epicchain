// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_MerkleBlockPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Neo.IO;
using Neo.Network.P2P.Payloads;
using System;
using System.Collections;

namespace Neo.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_MerkleBlockPayload
    {
        [TestMethod]
        public void Size_Get()
        {
            var test = MerkleBlockPayload.Create(TestBlockchain.TheEpicChainSystem.GenesisBlock, new BitArray(1024, false));
            test.Size.Should().Be(247); // 239 + nonce

            test = MerkleBlockPayload.Create(TestBlockchain.TheEpicChainSystem.GenesisBlock, new BitArray(0, false));
            test.Size.Should().Be(119); // 111 + nonce
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = MerkleBlockPayload.Create(TestBlockchain.TheEpicChainSystem.GenesisBlock, new BitArray(2, false));
            var clone = test.ToArray().AsSerializable<MerkleBlockPayload>();

            Assert.AreEqual(test.TxCount, clone.TxCount);
            Assert.AreEqual(test.Hashes.Length, clone.TxCount);
            CollectionAssert.AreEqual(test.Hashes, clone.Hashes);
            Assert.IsTrue(test.Flags.Span.SequenceEqual(clone.Flags.Span));
        }
    }
}

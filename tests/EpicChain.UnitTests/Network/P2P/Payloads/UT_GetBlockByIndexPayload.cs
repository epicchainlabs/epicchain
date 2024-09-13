// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_GetBlockByIndexPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using System;

namespace EpicChain.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_GetBlockByIndexPayload
    {
        [TestMethod]
        public void Size_Get()
        {
            var test = new GetBlockByIndexPayload() { Count = 5, IndexStart = 5 };
            test.Size.Should().Be(6);

            test = GetBlockByIndexPayload.Create(1, short.MaxValue);
            test.Size.Should().Be(6);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = new GetBlockByIndexPayload() { Count = -1, IndexStart = int.MaxValue };
            var clone = test.ToArray().AsSerializable<GetBlockByIndexPayload>();

            Assert.AreEqual(test.Count, clone.Count);
            Assert.AreEqual(test.IndexStart, clone.IndexStart);

            test = new GetBlockByIndexPayload() { Count = -2, IndexStart = int.MaxValue };
            Assert.ThrowsException<FormatException>(() => test.ToArray().AsSerializable<GetBlockByIndexPayload>());

            test = new GetBlockByIndexPayload() { Count = 0, IndexStart = int.MaxValue };
            Assert.ThrowsException<FormatException>(() => test.ToArray().AsSerializable<GetBlockByIndexPayload>());

            test = new GetBlockByIndexPayload() { Count = HeadersPayload.MaxHeadersCount + 1, IndexStart = int.MaxValue };
            Assert.ThrowsException<FormatException>(() => test.ToArray().AsSerializable<GetBlockByIndexPayload>());
        }
    }
}

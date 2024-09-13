// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ServerCapability.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Capabilities;
using System;

namespace EpicChain.UnitTests.Network.P2P.Capabilities
{
    [TestClass]
    public class UT_ServerCapability
    {
        [TestMethod]
        public void Size_Get()
        {
            var test = new ServerCapability(NodeCapabilityType.TcpServer) { Port = 1 };
            test.Size.Should().Be(3);

#pragma warning disable CS0612 // Type or member is obsolete
            test = new ServerCapability(NodeCapabilityType.WsServer) { Port = 2 };
#pragma warning restore CS0612 // Type or member is obsolete
            test.Size.Should().Be(3);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
#pragma warning disable CS0612 // Type or member is obsolete
            var test = new ServerCapability(NodeCapabilityType.WsServer) { Port = 2 };
            var buffer = test.ToArray();

            var br = new MemoryReader(buffer);
            var clone = (ServerCapability)NodeCapability.DeserializeFrom(ref br);

            Assert.AreEqual(test.Port, clone.Port);
            Assert.AreEqual(test.Type, clone.Type);

            clone = new ServerCapability(NodeCapabilityType.WsServer, 123);
#pragma warning restore CS0612 // Type or member is obsolete
            br = new MemoryReader(buffer);
            ((ISerializable)clone).Deserialize(ref br);

            Assert.AreEqual(test.Port, clone.Port);
            Assert.AreEqual(test.Type, clone.Type);

            clone = new ServerCapability(NodeCapabilityType.TcpServer, 123);

            Assert.ThrowsException<FormatException>(() =>
            {
                var br2 = new MemoryReader(buffer);
                ((ISerializable)clone).Deserialize(ref br2);
            });
            Assert.ThrowsException<ArgumentException>(() =>
            {
                _ = new ServerCapability(NodeCapabilityType.FullNode);
            });

            // Wrong type
            buffer[0] = 0xFF;
            Assert.ThrowsException<FormatException>(() =>
            {
                var br2 = new MemoryReader(buffer);
                NodeCapability.DeserializeFrom(ref br2);
            });
        }
    }
}

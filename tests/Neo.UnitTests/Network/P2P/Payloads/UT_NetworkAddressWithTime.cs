// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_NetworkAddressWithTime.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Neo.Network.P2P.Capabilities;
using Neo.Network.P2P.Payloads;
using System;
using System.Net;

namespace Neo.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_NetworkAddressWithTime
    {
        [TestMethod]
        public void SizeAndEndPoint_Get()
        {
            var test = new NetworkAddressWithTime() { Capabilities = new NodeCapability[0], Address = IPAddress.Any, Timestamp = 1 };
            test.Size.Should().Be(21);

            Assert.AreEqual(test.EndPoint.Port, 0);

            test = NetworkAddressWithTime.Create(IPAddress.Any, 1, new NodeCapability[] { new ServerCapability(NodeCapabilityType.TcpServer, 22) });
            test.Size.Should().Be(24);

            Assert.AreEqual(test.EndPoint.Port, 22);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = NetworkAddressWithTime.Create(IPAddress.Any, 1, new NodeCapability[] { new ServerCapability(NodeCapabilityType.TcpServer, 22) });
            var clone = test.ToArray().AsSerializable<NetworkAddressWithTime>();

            CollectionAssert.AreEqual(test.Capabilities.ToByteArray(), clone.Capabilities.ToByteArray());
            Assert.AreEqual(test.EndPoint.ToString(), clone.EndPoint.ToString());
            Assert.AreEqual(test.Timestamp, clone.Timestamp);
            Assert.AreEqual(test.Address, clone.Address);

            Assert.ThrowsException<FormatException>(() => NetworkAddressWithTime.Create(IPAddress.Any, 1,
                new NodeCapability[] {
                    new ServerCapability(NodeCapabilityType.TcpServer, 22) ,
                    new ServerCapability(NodeCapabilityType.TcpServer, 22)
                }).ToArray().AsSerializable<NetworkAddressWithTime>());
        }
    }
}

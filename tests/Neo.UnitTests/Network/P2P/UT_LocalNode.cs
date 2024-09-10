// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_LocalNode.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.TestKit.Xunit2;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Network.P2P;
using System;
using System.Linq;
using System.Net;

namespace Neo.UnitTests.Network.P2P
{
    [TestClass]
    public class UT_LocalNode : TestKit
    {
        private static EpicChainSystem testBlockchain;

        [TestInitialize]
        public void Init()
        {
            testBlockchain = TestBlockchain.TheEpicChainSystem;
        }

        [TestMethod]
        public void TestDefaults()
        {
            var senderProbe = CreateTestProbe();
            senderProbe.Send(testBlockchain.LocalNode, new LocalNode.GetInstance());
            var localnode = senderProbe.ExpectMsg<LocalNode>();

            Assert.AreEqual(0, localnode.ListenerTcpPort);
            Assert.AreEqual(3, localnode.MaxConnectionsPerAddress);
            Assert.AreEqual(10, localnode.MinDesiredConnections);
            Assert.AreEqual(40, localnode.MaxConnections);
            Assert.AreEqual(0, localnode.UnconnectedCount);

            CollectionAssert.AreEqual(Array.Empty<RemoteNode>(), localnode.GetRemoteNodes().ToArray());
            CollectionAssert.AreEqual(Array.Empty<IPEndPoint>(), localnode.GetUnconnectedPeers().ToArray());
        }
    }
}

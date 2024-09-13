// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_RemoteNode.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.IO;
using Akka.TestKit.Xunit2;
using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Capabilities;
using EpicChain.Network.P2P.Payloads;
using System.Net;

namespace EpicChain.UnitTests.Network.P2P
{
    [TestClass]
    public class UT_RemoteNode : TestKit
    {
        private static EpicChainSystem testBlockchain;

        public UT_RemoteNode()
            : base($"remote-node-mailbox {{ mailbox-type: \"{typeof(RemoteNodeMailbox).AssemblyQualifiedName}\" }}")
        {
        }

        [ClassInitialize]
        public static void TestSetup(TestContext ctx)
        {
            testBlockchain = TestBlockchain.TheEpicChainSystem;
        }

        [TestMethod]
        public void RemoteNode_Test_Abort_DifferentNetwork()
        {
            var connectionTestProbe = CreateTestProbe();
            var remoteNodeActor = ActorOfAsTestActorRef(() => new RemoteNode(testBlockchain, new LocalNode(testBlockchain), connectionTestProbe, null, null));

            var msg = Message.Create(MessageCommand.Version, new VersionPayload
            {
                UserAgent = "".PadLeft(1024, '0'),
                Nonce = 1,
                Network = 2,
                Timestamp = 5,
                Version = 6,
                Capabilities = new NodeCapability[]
                {
                    new ServerCapability(NodeCapabilityType.TcpServer, 25)
                }
            });

            var testProbe = CreateTestProbe();
            testProbe.Send(remoteNodeActor, new Tcp.Received((ByteString)msg.ToArray()));

            connectionTestProbe.ExpectMsg<Tcp.Abort>();
        }

        [TestMethod]
        public void RemoteNode_Test_Accept_IfSameNetwork()
        {
            var connectionTestProbe = CreateTestProbe();
            var remoteNodeActor = ActorOfAsTestActorRef(() => new RemoteNode(testBlockchain, new LocalNode(testBlockchain), connectionTestProbe, new IPEndPoint(IPAddress.Parse("192.168.1.2"), 8080), new IPEndPoint(IPAddress.Parse("192.168.1.1"), 8080)));

            var msg = Message.Create(MessageCommand.Version, new VersionPayload()
            {
                UserAgent = "Unit Test".PadLeft(1024, '0'),
                Nonce = 1,
                Network = TestProtocolSettings.Default.Network,
                Timestamp = 5,
                Version = 6,
                Capabilities = new NodeCapability[]
                {
                    new ServerCapability(NodeCapabilityType.TcpServer, 25)
                }
            });

            var testProbe = CreateTestProbe();
            testProbe.Send(remoteNodeActor, new Tcp.Received((ByteString)msg.ToArray()));

            var verackMessage = connectionTestProbe.ExpectMsg<Tcp.Write>();

            //Verack
            verackMessage.Data.Count.Should().Be(3);
        }
    }
}

// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// UT_RemoteNodeMailbox.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Akka.IO;
using Akka.TestKit.Xunit2;
using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using Neo.Network.P2P;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Neo.UnitTests.Network.P2P
{
    [TestClass]
    public class UT_RemoteNodeMailbox : TestKit
    {
        private static readonly Random TestRandom = new Random(1337); // use fixed seed for guaranteed determinism

        RemoteNodeMailbox uut;

        [TestCleanup]
        public void Cleanup()
        {
            Shutdown();
        }

        [TestInitialize]
        public void TestSetup()
        {
            Akka.Actor.ActorSystem system = Sys;
            var config = TestKit.DefaultConfig;
            var akkaSettings = new Akka.Actor.Settings(system, config);
            uut = new RemoteNodeMailbox(akkaSettings, config);
        }

        [TestMethod]
        public void RemoteNode_Test_IsHighPriority()
        {
            ISerializable s = null;

            //handshaking
            uut.IsHighPriority(Message.Create(MessageCommand.Version, s)).Should().Be(true);
            uut.IsHighPriority(Message.Create(MessageCommand.Verack, s)).Should().Be(true);

            //connectivity
            uut.IsHighPriority(Message.Create(MessageCommand.GetAddr, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Addr, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Ping, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Pong, s)).Should().Be(false);

            //synchronization
            uut.IsHighPriority(Message.Create(MessageCommand.GetHeaders, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Headers, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.GetBlocks, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Mempool, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Inv, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.GetData, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.NotFound, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Transaction, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Block, s)).Should().Be(false);
            uut.IsHighPriority(Message.Create(MessageCommand.Extensible, s)).Should().Be(true);
            uut.IsHighPriority(Message.Create(MessageCommand.Reject, s)).Should().Be(false);

            //SPV protocol
            uut.IsHighPriority(Message.Create(MessageCommand.FilterLoad, s)).Should().Be(true);
            uut.IsHighPriority(Message.Create(MessageCommand.FilterAdd, s)).Should().Be(true);
            uut.IsHighPriority(Message.Create(MessageCommand.FilterClear, s)).Should().Be(true);
            uut.IsHighPriority(Message.Create(MessageCommand.MerkleBlock, s)).Should().Be(false);

            //others
            uut.IsHighPriority(Message.Create(MessageCommand.Alert, s)).Should().Be(true);

            // high priority commands
            uut.IsHighPriority(new Tcp.ConnectionClosed()).Should().Be(true);
            uut.IsHighPriority(new Connection.Close()).Should().Be(true);
            uut.IsHighPriority(new Connection.Ack()).Should().Be(true);

            // any random object should not have priority
            object obj = null;
            uut.IsHighPriority(obj).Should().Be(false);
        }

        public void ProtocolHandlerMailbox_Test_ShallDrop()
        {
            // using this for messages
            ISerializable s = null;
            Message msg; // multiple uses
            // empty queue
            IEnumerable<object> emptyQueue = Enumerable.Empty<object>();

            // any random object (non Message) should be dropped
            object obj = null;
            uut.ShallDrop(obj, emptyQueue).Should().Be(true);

            //handshaking
            // Version (no drop)
            msg = Message.Create(MessageCommand.Version, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Verack (no drop)
            msg = Message.Create(MessageCommand.Verack, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);

            //connectivity
            // GetAddr (drop)
            msg = Message.Create(MessageCommand.GetAddr, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(true);
            // Addr (no drop)
            msg = Message.Create(MessageCommand.Addr, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Ping (no drop)
            msg = Message.Create(MessageCommand.Ping, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Pong (no drop)
            msg = Message.Create(MessageCommand.Pong, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);

            //synchronization
            // GetHeaders (drop)
            msg = Message.Create(MessageCommand.GetHeaders, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(true);
            // Headers (no drop)
            msg = Message.Create(MessageCommand.Headers, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // GetBlocks (drop)
            msg = Message.Create(MessageCommand.GetBlocks, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(true);
            // Mempool (drop)
            msg = Message.Create(MessageCommand.Mempool, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(true);
            // Inv (no drop)
            msg = Message.Create(MessageCommand.Inv, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // NotFound (no drop)
            msg = Message.Create(MessageCommand.NotFound, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Transaction (no drop)
            msg = Message.Create(MessageCommand.Transaction, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Block (no drop)
            msg = Message.Create(MessageCommand.Block, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Consensus (no drop)
            msg = Message.Create(MessageCommand.Extensible, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // Reject (no drop)
            msg = Message.Create(MessageCommand.Reject, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);

            //SPV protocol
            // FilterLoad (no drop)
            msg = Message.Create(MessageCommand.FilterLoad, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // FilterAdd (no drop)
            msg = Message.Create(MessageCommand.FilterAdd, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // FilterClear (no drop)
            msg = Message.Create(MessageCommand.FilterClear, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
            // MerkleBlock (no drop)
            msg = Message.Create(MessageCommand.MerkleBlock, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);

            //others
            // Alert (no drop)
            msg = Message.Create(MessageCommand.Alert, s);
            uut.ShallDrop(msg, emptyQueue).Should().Be(false);
            uut.ShallDrop(msg, new object[] { msg }).Should().Be(false);
        }
    }
}

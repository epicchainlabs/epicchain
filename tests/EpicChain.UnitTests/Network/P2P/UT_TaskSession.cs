// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_TaskSession.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Capabilities;
using EpicChain.Network.P2P.Payloads;
using System;
using Xunit.Sdk;

namespace EpicChain.UnitTests.Network.P2P
{
    [TestClass]
    public class UT_TaskSession
    {
        [TestMethod]
        public void CreateTest()
        {
            var ses = new TaskSession(new VersionPayload() { Capabilities = new NodeCapability[] { new FullNodeCapability(123) } });

            Assert.IsFalse(ses.HasTooManyTasks);
            Assert.AreEqual((uint)123, ses.LastBlockIndex);
            Assert.AreEqual(0, ses.IndexTasks.Count);
            Assert.IsTrue(ses.IsFullNode);

            ses = new TaskSession(new VersionPayload() { Capabilities = Array.Empty<NodeCapability>() });

            Assert.IsFalse(ses.HasTooManyTasks);
            Assert.AreEqual((uint)0, ses.LastBlockIndex);
            Assert.AreEqual(0, ses.IndexTasks.Count);
            Assert.IsFalse(ses.IsFullNode);
        }
    }
}

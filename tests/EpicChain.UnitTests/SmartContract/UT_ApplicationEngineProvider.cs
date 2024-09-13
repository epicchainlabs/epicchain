// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ApplicationEngineProvider.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.VM;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_ApplicationEngineProvider
    {
        [TestInitialize]
        public void TestInitialize()
        {
            ApplicationEngine.Provider = null;
        }

        [TestCleanup]
        public void TestCleanup()
        {
            ApplicationEngine.Provider = null;
        }

        [TestMethod]
        public void TestSetAppEngineProvider()
        {
            ApplicationEngine.Provider = new TestProvider();

            using var appEngine = ApplicationEngine.Create(TriggerType.Application, null, null, epicpulse: 0, settings: TestBlockchain.TheEpicChainSystem.Settings);
            (appEngine is TestEngine).Should().BeTrue();
        }

        [TestMethod]
        public void TestDefaultAppEngineProvider()
        {
            using var appEngine = ApplicationEngine.Create(TriggerType.Application, null, null, epicpulse: 0, settings: TestBlockchain.TheEpicChainSystem.Settings);
            (appEngine is ApplicationEngine).Should().BeTrue();
        }

        class TestProvider : IApplicationEngineProvider
        {
            public ApplicationEngine Create(TriggerType trigger, IVerifiable container, DataCache snapshot, Block persistingBlock, ProtocolSettings settings, long epicpulse, IDiagnostic diagnostic, JumpTable jumpTable)
            {
                return new TestEngine(trigger, container, snapshot, persistingBlock, settings, epicpulse, diagnostic, jumpTable);
            }
        }

        class TestEngine : ApplicationEngine
        {
            public TestEngine(TriggerType trigger, IVerifiable container, DataCache snapshotCache, Block persistingBlock, ProtocolSettings settings, long epicpulse, IDiagnostic diagnostic, JumpTable jumpTable)
                : base(trigger, container, snapshotCache, persistingBlock, settings, epicpulse, diagnostic, jumpTable)
            {
            }
        }
    }
}

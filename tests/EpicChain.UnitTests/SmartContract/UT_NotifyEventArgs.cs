// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_NotifyEventArgs.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.VM.Types;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_NotifyEventArgs
    {
        [TestMethod]
        public void TestGetScriptContainer()
        {
            IVerifiable container = new TestVerifiable();
            UInt160 script_hash = new byte[] { 0x00 }.ToScriptHash();
            NotifyEventArgs args = new NotifyEventArgs(container, script_hash, "Test", null);
            args.ScriptContainer.Should().Be(container);
        }


        [TestMethod]
        public void TestIssue3300() // https://github.com/epicchainlabs/epicchain/issues/3300
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, null, settings: TestProtocolSettings.Default, epicpulse: 1100_00000000);
            using (var script = new ScriptBuilder())
            {
                // Build call script calling disallowed method.
                script.Emit(OpCode.NOP);
                // Mock executing state to be a contract-based.
                engine.LoadScript(script.ToArray());
            }

            var ns = new Array(engine.ReferenceCounter);
            for (var i = 0; i < 500; i++)
            {
                ns.Add("");
            };

            var hash = UInt160.Parse("0x179ab5d297fd34ecd48643894242fc3527f42853");
            engine.SendNotification(hash, "Test", ns);
            // This should have being 0, but we have optimized the vm to not clean the reference counter
            // unless it is necessary, so the reference counter will be 1000.
            // Same reason why its 1504 instead of 504.
            Assert.AreEqual(1000, engine.ReferenceCounter.Count);
            // This will make a deepcopy for the notification, along with the 500 state items.
            engine.GetNotifications(hash);
            // With the fix of issue 3300, the reference counter calculates not only
            // the notifaction items, but also the subitems of the notification state.
            Assert.AreEqual(1504, engine.ReferenceCounter.Count);
        }
    }
}

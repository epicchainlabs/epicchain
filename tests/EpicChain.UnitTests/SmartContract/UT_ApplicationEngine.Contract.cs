// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ApplicationEngine.Contract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.VM;
using System.Linq;

namespace EpicChain.UnitTests.SmartContract
{
    public partial class UT_ApplicationEngine
    {
        [TestMethod]
        public void TestCreateStandardAccount()
        {
            var settings = TestProtocolSettings.Default;
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, null, settings: TestProtocolSettings.Default, epicpulse: 1100_00000000);

            using var script = new ScriptBuilder();
            script.EmitSysCall(ApplicationEngine.System_Contract_CreateStandardAccount, settings.StandbyCommittee[0].EncodePoint(true));
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(VMState.HALT, engine.Execute());

            var result = engine.ResultStack.Pop();
            new UInt160(result.GetSpan()).Should().Be(Contract.CreateSignatureRedeemScript(settings.StandbyCommittee[0]).ToScriptHash());
        }

        [TestMethod]
        public void TestCreateStandardMultisigAccount()
        {
            var settings = TestProtocolSettings.Default;
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, null, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: 1100_00000000);

            using var script = new ScriptBuilder();
            script.EmitSysCall(ApplicationEngine.System_Contract_CreateMultisigAccount, new object[]
            {
                2,
                3,
                settings.StandbyCommittee[0].EncodePoint(true),
                settings.StandbyCommittee[1].EncodePoint(true),
                settings.StandbyCommittee[2].EncodePoint(true)
            });
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(VMState.HALT, engine.Execute());

            var result = engine.ResultStack.Pop();
            new UInt160(result.GetSpan()).Should().Be(Contract.CreateMultiSigRedeemScript(2, settings.StandbyCommittee.Take(3).ToArray()).ToScriptHash());
        }
    }
}

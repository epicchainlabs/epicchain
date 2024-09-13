// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ContractState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using EpicChain.VM;
using System;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_ContractState
    {
        ContractState contract;
        readonly byte[] script = { 0x01 };
        ContractManifest manifest;

        [TestInitialize]
        public void TestSetup()
        {
            manifest = TestUtils.CreateDefaultManifest();
            contract = new ContractState
            {
                Nef = new NefFile
                {
                    Compiler = nameof(ScriptBuilder),
                    Source = string.Empty,
                    Tokens = Array.Empty<MethodToken>(),
                    Script = script
                },
                Hash = script.ToScriptHash(),
                Manifest = manifest
            };
            contract.Nef.CheckSum = NefFile.ComputeChecksum(contract.Nef);
        }

        [TestMethod]
        public void TestGetScriptHash()
        {
            // _scriptHash == null
            contract.Hash.Should().Be(script.ToScriptHash());
            // _scriptHash != null
            contract.Hash.Should().Be(script.ToScriptHash());
        }

        [TestMethod]
        public void TestIInteroperable()
        {
            IInteroperable newContract = new ContractState();
            newContract.FromStackItem(contract.ToStackItem(null));
            ((ContractState)newContract).Manifest.ToJson().ToString().Should().Be(contract.Manifest.ToJson().ToString());
            ((ContractState)newContract).Script.Span.SequenceEqual(contract.Script.Span).Should().BeTrue();
        }

        [TestMethod]
        public void TestCanCall()
        {
            var temp = new ContractState() { Manifest = TestUtils.CreateDefaultManifest() };

            Assert.AreEqual(true, temp.CanCall(new ContractState() { Hash = UInt160.Zero, Manifest = TestUtils.CreateDefaultManifest() }, "AAA"));
        }

        [TestMethod]
        public void TestToJson()
        {
            JObject json = contract.ToJson();
            json["hash"].AsString().Should().Be("0x820944cfdc70976602d71b0091445eedbc661bc5");
            json["nef"]["script"].AsString().Should().Be("AQ==");
            json["manifest"].AsString().Should().Be(manifest.ToJson().AsString());
        }
    }
}

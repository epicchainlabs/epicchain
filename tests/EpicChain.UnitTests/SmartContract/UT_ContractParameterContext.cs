// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ContractParameterContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography.ECC;
using EpicChain.Extensions;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.Wallets;
using System;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_ContractParameterContext
    {
        private static Contract contract;
        private static KeyPair key;

        [ClassInitialize]
        public static void ClassSetUp(TestContext ctx)
        {
            if (contract == null)
            {
                byte[] privateKey = new byte[] { 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                                 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                                 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                                 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01 };
                key = new KeyPair(privateKey);
                contract = Contract.CreateSignatureContract(key.PublicKey);
            }
        }

        [TestMethod]
        public void TestGetComplete()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Transaction tx = TestUtils.GetTransaction(UInt160.Parse("0x1bd5c777ec35768892bd3daab60fb7a1cb905066"));
            var context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.Completed.Should().BeFalse();
        }

        [TestMethod]
        public void TestToString()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Transaction tx = TestUtils.GetTransaction(UInt160.Parse("0x1bd5c777ec35768892bd3daab60fb7a1cb905066"));
            var context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.Add(contract, 0, new byte[] { 0x01 });
            string str = context.ToString();
            str.Should().Be(@"{""type"":""EpicChain.Network.P2P.Payloads.Transaction"",""hash"":""0x602c1fa1c08b041e4e6b87aa9a9f9c643166cd34bdd5215a3dd85778c59cce88"",""data"":""AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFmUJDLobcPtqo9vZKIdjXsd8fVGwEAARI="",""items"":{},""network"":" + TestProtocolSettings.Default.Network + "}");
        }

        [TestMethod]
        public void TestParse()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var ret = ContractParametersContext.Parse("{\"type\":\"EpicChain.Network.P2P.Payloads.Transaction\",\"data\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFmUJDLobcPtqo9vZKIdjXsd8fVGwEAARI=\",\"items\":{\"0xbecaad15c0ea585211faf99738a4354014f177f2\":{\"script\":\"IQJv8DuUkkHOHa3UNRnmlg4KhbQaaaBcMoEDqivOFZTKFmh0dHaq\",\"parameters\":[{\"type\":\"Signature\",\"value\":\"AQ==\"}],\"signatures\":{\"03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c\":\"AQ==\"}}},\"network\":" + TestProtocolSettings.Default.Network + "}", snapshotCache);
            ret.ScriptHashes[0].ToString().Should().Be("0x1bd5c777ec35768892bd3daab60fb7a1cb905066");
            ((Transaction)ret.Verifiable).Script.Span.ToHexString().Should().Be(new byte[] { 18 }.ToHexString());
        }

        [TestMethod]
        public void TestFromJson()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Action action = () => ContractParametersContext.Parse("{\"type\":\"wrongType\",\"data\":\"00000000007c97764845172d827d3c863743293931a691271a0000000000000000000000000000000000000000000100\",\"items\":{\"0x1bd5c777ec35768892bd3daab60fb7a1cb905066\":{\"script\":\"21026ff03b949241ce1dadd43519e6960e0a85b41a69a05c328103aa2bce1594ca1650680a906ad4\",\"parameters\":[{\"type\":\"Signature\",\"value\":\"01\"}]}}}", snapshotCache);
            action.Should().Throw<FormatException>();
        }

        [TestMethod]
        public void TestAdd()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Transaction tx = TestUtils.GetTransaction(UInt160.Zero);
            var context1 = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context1.Add(contract, 0, new byte[] { 0x01 }).Should().BeFalse();

            tx = TestUtils.GetTransaction(UInt160.Parse("0x902e0d38da5e513b6d07c1c55b85e77d3dce8063"));
            var context2 = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context2.Add(contract, 0, new byte[] { 0x01 }).Should().BeTrue();
            //test repeatlly createItem
            context2.Add(contract, 0, new byte[] { 0x01 }).Should().BeTrue();
        }

        [TestMethod]
        public void TestGetParameter()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Transaction tx = TestUtils.GetTransaction(UInt160.Parse("0x902e0d38da5e513b6d07c1c55b85e77d3dce8063"));
            var context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.GetParameter(tx.Sender, 0).Should().BeNull();

            context.Add(contract, 0, new byte[] { 0x01 });
            var ret = context.GetParameter(tx.Sender, 0);
            ((byte[])ret.Value).ToHexString().Should().Be(new byte[] { 0x01 }.ToHexString());
        }

        [TestMethod]
        public void TestGetWitnesses()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Transaction tx = TestUtils.GetTransaction(UInt160.Parse("0x902e0d38da5e513b6d07c1c55b85e77d3dce8063"));
            var context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.Add(contract, 0, new byte[] { 0x01 });
            Witness[] witnesses = context.GetWitnesses();
            witnesses.Length.Should().Be(1);
            witnesses[0].InvocationScript.Span.ToHexString().Should().Be(new byte[] { (byte)OpCode.PUSHDATA1, 0x01, 0x01 }.ToHexString());
            witnesses[0].VerificationScript.Span.ToHexString().Should().Be(contract.Script.ToHexString());
        }

        [TestMethod]
        public void TestAddSignature()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var singleSender = UInt160.Parse("0x902e0d38da5e513b6d07c1c55b85e77d3dce8063");
            Transaction tx = TestUtils.GetTransaction(singleSender);

            //singleSign

            var context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.AddSignature(contract, key.PublicKey, new byte[] { 0x01 }).Should().BeTrue();

            var contract1 = Contract.CreateSignatureContract(key.PublicKey);
            contract1.ParameterList = Array.Empty<ContractParameterType>();
            context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.AddSignature(contract1, key.PublicKey, new byte[] { 0x01 }).Should().BeFalse();

            contract1.ParameterList = new[] { ContractParameterType.Signature, ContractParameterType.Signature };
            Action action1 = () => context.AddSignature(contract1, key.PublicKey, new byte[] { 0x01 });
            action1.Should().Throw<NotSupportedException>();

            //multiSign

            byte[] privateKey2 = new byte[] { 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                              0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                              0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                              0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02 };
            var key2 = new KeyPair(privateKey2);
            var multiSignContract = Contract.CreateMultiSigContract(2,
                    new ECPoint[]
                    {
                        key.PublicKey,
                        key2.PublicKey
                    });
            var multiSender = UInt160.Parse("0xf76b51bc6605ac3cfcd188173af0930507f51210");
            tx = TestUtils.GetTransaction(multiSender);
            context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.AddSignature(multiSignContract, key.PublicKey, new byte[] { 0x01 }).Should().BeTrue();
            context.AddSignature(multiSignContract, key2.PublicKey, new byte[] { 0x01 }).Should().BeTrue();

            tx = TestUtils.GetTransaction(singleSender);
            context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            context.AddSignature(multiSignContract, key.PublicKey, new byte[] { 0x01 }).Should().BeFalse();

            tx = TestUtils.GetTransaction(multiSender);
            context = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            byte[] privateKey3 = new byte[] { 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                              0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                              0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                                              0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x03 };
            var key3 = new KeyPair(privateKey3);
            context.AddSignature(multiSignContract, key3.PublicKey, new byte[] { 0x01 }).Should().BeFalse();
        }
    }
}
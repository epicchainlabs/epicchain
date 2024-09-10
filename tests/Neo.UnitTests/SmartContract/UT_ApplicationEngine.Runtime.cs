// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ApplicationEngine.Runtime.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Manifest;
using System;
using System.Numerics;
using System.Text;

namespace Neo.UnitTests.SmartContract
{
    public partial class UT_ApplicationEngine
    {
        [TestMethod]
        public void TestGetNetworkAndAddressVersion()
        {
            var tx = TestUtils.GetTransaction(UInt160.Zero);
            using var engine = ApplicationEngine.Create(TriggerType.Application, tx, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);

            engine.GetNetwork().Should().Be(TestBlockchain.TheEpicChainSystem.Settings.Network);
            engine.GetAddressVersion().Should().Be(TestBlockchain.TheEpicChainSystem.Settings.AddressVersion);
        }

        [TestMethod]
        public void TestNotSupportedNotification()
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);
            engine.LoadScript(Array.Empty<byte>());
            engine.CurrentContext.GetState<ExecutionContextState>().Contract = new()
            {
                Manifest = new()
                {
                    Abi = new()
                    {
                        Events = new[]
                        {
                            new ContractEventDescriptor
                            {
                                Name = "e1",
                                Parameters = new[]
                                {
                                    new ContractParameterDefinition
                                    {
                                        Type = ContractParameterType.Array
                                    }
                                }
                            }
                        }
                    }
                }
            };

            // circular

            VM.Types.Array array = new();
            array.Add(array);

            Assert.ThrowsException<NotSupportedException>(() => engine.RuntimeNotify(Encoding.ASCII.GetBytes("e1"), array));

            // Buffer

            array.Clear();
            array.Add(new VM.Types.Buffer(1));
            engine.CurrentContext.GetState<ExecutionContextState>().Contract.Manifest.Abi.Events[0].Parameters[0].Type = ContractParameterType.ByteArray;

            engine.RuntimeNotify(Encoding.ASCII.GetBytes("e1"), array);
            engine.Notifications[0].State[0].Type.Should().Be(VM.Types.StackItemType.ByteString);

            // Pointer

            array.Clear();
            array.Add(new VM.Types.Pointer(Array.Empty<byte>(), 1));

            Assert.ThrowsException<InvalidOperationException>(() => engine.RuntimeNotify(Encoding.ASCII.GetBytes("e1"), array));

            // InteropInterface

            array.Clear();
            array.Add(new VM.Types.InteropInterface(new object()));
            engine.CurrentContext.GetState<ExecutionContextState>().Contract.Manifest.Abi.Events[0].Parameters[0].Type = ContractParameterType.InteropInterface;

            Assert.ThrowsException<NotSupportedException>(() => engine.RuntimeNotify(Encoding.ASCII.GetBytes("e1"), array));
        }

        [TestMethod]
        public void TestGetRandomSameBlock()
        {
            var tx = TestUtils.GetTransaction(UInt160.Zero);
            // Even if persisting the same block, in different ApplicationEngine instance, the random number should be different
            using var engine_1 = ApplicationEngine.Create(TriggerType.Application, tx, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);
            using var engine_2 = ApplicationEngine.Create(TriggerType.Application, tx, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);

            engine_1.LoadScript(new byte[] { 0x01 });
            engine_2.LoadScript(new byte[] { 0x01 });

            var rand_1 = engine_1.GetRandom();
            var rand_2 = engine_1.GetRandom();
            var rand_3 = engine_1.GetRandom();
            var rand_4 = engine_1.GetRandom();
            var rand_5 = engine_1.GetRandom();

            var rand_6 = engine_2.GetRandom();
            var rand_7 = engine_2.GetRandom();
            var rand_8 = engine_2.GetRandom();
            var rand_9 = engine_2.GetRandom();
            var rand_10 = engine_2.GetRandom();

            rand_1.Should().Be(BigInteger.Parse("271339657438512451304577787170704246350"));
            rand_2.Should().Be(BigInteger.Parse("98548189559099075644778613728143131367"));
            rand_3.Should().Be(BigInteger.Parse("247654688993873392544380234598471205121"));
            rand_4.Should().Be(BigInteger.Parse("291082758879475329976578097236212073607"));
            rand_5.Should().Be(BigInteger.Parse("247152297361212656635216876565962360375"));

            rand_1.Should().Be(rand_6);
            rand_2.Should().Be(rand_7);
            rand_3.Should().Be(rand_8);
            rand_4.Should().Be(rand_9);
            rand_5.Should().Be(rand_10);
        }

        [TestMethod]
        public void TestGetRandomDifferentBlock()
        {
            var tx_1 = TestUtils.GetTransaction(UInt160.Zero);

            var tx_2 = new Transaction
            {
                Version = 0,
                Nonce = 2083236893,
                ValidUntilBlock = 0,
                Signers = Array.Empty<Signer>(),
                Attributes = Array.Empty<TransactionAttribute>(),
                Script = Array.Empty<byte>(),
                SystemFee = 0,
                NetworkFee = 0,
                Witnesses = Array.Empty<Witness>()
            };

            using var engine_1 = ApplicationEngine.Create(TriggerType.Application, tx_1, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);
            // The next_nonce shuld be reinitialized when a new block is persisting
            using var engine_2 = ApplicationEngine.Create(TriggerType.Application, tx_2, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);

            var rand_1 = engine_1.GetRandom();
            var rand_2 = engine_1.GetRandom();
            var rand_3 = engine_1.GetRandom();
            var rand_4 = engine_1.GetRandom();
            var rand_5 = engine_1.GetRandom();

            var rand_6 = engine_2.GetRandom();
            var rand_7 = engine_2.GetRandom();
            var rand_8 = engine_2.GetRandom();
            var rand_9 = engine_2.GetRandom();
            var rand_10 = engine_2.GetRandom();

            rand_1.Should().Be(BigInteger.Parse("271339657438512451304577787170704246350"));
            rand_2.Should().Be(BigInteger.Parse("98548189559099075644778613728143131367"));
            rand_3.Should().Be(BigInteger.Parse("247654688993873392544380234598471205121"));
            rand_4.Should().Be(BigInteger.Parse("291082758879475329976578097236212073607"));
            rand_5.Should().Be(BigInteger.Parse("247152297361212656635216876565962360375"));

            rand_1.Should().NotBe(rand_6);
            rand_2.Should().NotBe(rand_7);
            rand_3.Should().NotBe(rand_8);
            rand_4.Should().NotBe(rand_9);
            rand_5.Should().NotBe(rand_10);
        }

        [TestMethod]
        public void TestInvalidUtf8LogMessage()
        {
            var tx_1 = TestUtils.GetTransaction(UInt160.Zero);
            using var engine = ApplicationEngine.Create(TriggerType.Application, tx_1, null, TestBlockchain.TheEpicChainSystem.GenesisBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, gas: 1100_00000000);
            var msg = new byte[]
            {
                68, 216, 160, 6, 89, 102, 86, 72, 37, 15, 132, 45, 76, 221, 170, 21, 128, 51, 34, 168, 205, 56, 10, 228, 51, 114, 4, 218, 245, 155, 172, 132
            };
            Assert.ThrowsException<ArgumentException>(() => engine.RuntimeLog(msg));
        }
    }
}

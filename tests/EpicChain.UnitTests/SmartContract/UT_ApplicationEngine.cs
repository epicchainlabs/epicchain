// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ApplicationEngine.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract.Manifest;
using EpicChain.UnitTests.Extensions;
using EpicChain.VM;
using System;
using System.Collections.Immutable;
using System.Linq;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public partial class UT_ApplicationEngine
    {
        private string eventName = null;

        [TestMethod]
        public void TestNotify()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(System.Array.Empty<byte>());
            ApplicationEngine.Notify += Test_Notify1;
            const string notifyEvent = "TestEvent";

            engine.SendNotification(UInt160.Zero, notifyEvent, new Array());
            eventName.Should().Be(notifyEvent);

            ApplicationEngine.Notify += Test_Notify2;
            engine.SendNotification(UInt160.Zero, notifyEvent, new Array());
            eventName.Should().Be(null);

            eventName = notifyEvent;
            ApplicationEngine.Notify -= Test_Notify1;
            engine.SendNotification(UInt160.Zero, notifyEvent, new Array());
            eventName.Should().Be(null);

            ApplicationEngine.Notify -= Test_Notify2;
            engine.SendNotification(UInt160.Zero, notifyEvent, new Array());
            eventName.Should().Be(null);
        }

        private void Test_Notify1(object sender, NotifyEventArgs e)
        {
            eventName = e.EventName;
        }

        private void Test_Notify2(object sender, NotifyEventArgs e)
        {
            eventName = null;
        }

        [TestMethod]
        public void TestCreateDummyBlock()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            byte[] SyscallSystemRuntimeCheckWitnessHash = new byte[] { 0x68, 0xf8, 0x27, 0xec, 0x8c };
            ApplicationEngine engine = ApplicationEngine.Run(SyscallSystemRuntimeCheckWitnessHash, snapshotCache, settings: TestProtocolSettings.Default);
            engine.PersistingBlock.Version.Should().Be(0);
            engine.PersistingBlock.PrevHash.Should().Be(TestBlockchain.TheEpicChainSystem.GenesisBlock.Hash);
            engine.PersistingBlock.MerkleRoot.Should().Be(new UInt256());
        }

        [TestMethod]
        public void TestCheckingHardfork()
        {
            var allHardforks = Enum.GetValues(typeof(Hardfork)).Cast<Hardfork>().ToList();

            var builder = ImmutableDictionary.CreateBuilder<Hardfork, uint>();
            builder.Add(Hardfork.HF_Aspidochelone, 0);
            builder.Add(Hardfork.HF_Basilisk, 1);

            var setting = builder.ToImmutable();

            // Check for continuity in configured hardforks
            var sortedHardforks = setting.Keys
                .OrderBy(h => allHardforks.IndexOf(h))
                .ToList();

            for (int i = 0; i < sortedHardforks.Count - 1; i++)
            {
                int currentIndex = allHardforks.IndexOf(sortedHardforks[i]);
                int nextIndex = allHardforks.IndexOf(sortedHardforks[i + 1]);

                // If they aren't consecutive, return false.
                var inc = nextIndex - currentIndex;
                inc.Should().Be(1);
            }

            // Check that block numbers are not higher in earlier hardforks than in later ones
            for (int i = 0; i < sortedHardforks.Count - 1; i++)
            {
                (setting[sortedHardforks[i]] > setting[sortedHardforks[i + 1]]).Should().Be(false);
            }
        }

        [TestMethod]
        public void TestSystem_Contract_Call_Permissions()
        {
            UInt160 scriptHash;
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            // Setup: put a simple contract to the storage.
            using (var script = new ScriptBuilder())
            {
                // Push True on stack and return.
                script.EmitPush(true);
                script.Emit(OpCode.RET);

                // Mock contract and put it to the Managemant's storage.
                scriptHash = script.ToArray().ToScriptHash();

                snapshotCache.DeleteContract(scriptHash);
                var contract = TestUtils.GetContract(script.ToArray(), TestUtils.CreateManifest("test", ContractParameterType.Any));
                contract.Manifest.Abi.Methods = new[]
                {
                    new ContractMethodDescriptor
                    {
                        Name = "disallowed",
                        Parameters = new ContractParameterDefinition[]{}
                    },
                    new ContractMethodDescriptor
                    {
                        Name = "test",
                        Parameters = new ContractParameterDefinition[]{}
                    }
                };
                snapshotCache.AddContract(scriptHash, contract);
            }

            // Disallowed method call.
            using (var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, null, ProtocolSettings.Default))
            using (var script = new ScriptBuilder())
            {
                // Build call script calling disallowed method.
                script.EmitDynamicCall(scriptHash, "disallowed");

                // Mock executing state to be a contract-based.
                engine.LoadScript(script.ToArray());
                engine.CurrentContext.GetState<ExecutionContextState>().Contract = new()
                {
                    Manifest = new()
                    {
                        Abi = new() { },
                        Permissions = new ContractPermission[]
                        {
                            new ContractPermission
                            {
                                Contract = ContractPermissionDescriptor.Create(scriptHash),
                                Methods = WildcardContainer<string>.Create(new string[]{"test"}) // allowed to call only "test" method of the target contract.
                            }
                        }
                    }
                };
                var currentScriptHash = engine.EntryScriptHash;

                Assert.AreEqual(VMState.FAULT, engine.Execute());
                Assert.IsTrue(engine.FaultException.ToString().Contains($"Cannot Call Method disallowed Of Contract {scriptHash.ToString()}"));
            }

            // Allowed method call.
            using (var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, null, ProtocolSettings.Default))
            using (var script = new ScriptBuilder())
            {
                // Build call script.
                script.EmitDynamicCall(scriptHash, "test");

                // Mock executing state to be a contract-based.
                engine.LoadScript(script.ToArray());
                engine.CurrentContext.GetState<ExecutionContextState>().Contract = new()
                {
                    Manifest = new()
                    {
                        Abi = new() { },
                        Permissions = new ContractPermission[]
                        {
                            new ContractPermission
                            {
                                Contract = ContractPermissionDescriptor.Create(scriptHash),
                                Methods = WildcardContainer<string>.Create(new string[]{"test"}) // allowed to call only "test" method of the target contract.
                            }
                        }
                    }
                };
                var currentScriptHash = engine.EntryScriptHash;

                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsInstanceOfType(engine.ResultStack.Peek(), typeof(VM.Types.Boolean));
                var res = (VM.Types.Boolean)engine.ResultStack.Pop();
                Assert.IsTrue(res.GetBoolean());
            }
        }
    }
}

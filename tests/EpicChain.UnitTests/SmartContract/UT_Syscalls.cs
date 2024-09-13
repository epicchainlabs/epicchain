// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Syscalls.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.TestKit.Xunit2;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests.Extensions;
using EpicChain.VM;
using EpicChain.VM.Types;
using System.Linq;
using Array = System.Array;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public partial class UT_Syscalls : TestKit
    {
        [TestMethod]
        public void System_Blockchain_GetBlock()
        {
            var tx = new Transaction()
            {
                Script = new byte[] { 0x01 },
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = Array.Empty<Signer>(),
                NetworkFee = 0x02,
                SystemFee = 0x03,
                Nonce = 0x04,
                ValidUntilBlock = 0x05,
                Version = 0x06,
                Witnesses = new Witness[] { new Witness() { VerificationScript = new byte[] { 0x07 } } },
            };

            var block = new TrimmedBlock()
            {
                Header = new Header
                {
                    Index = 0,
                    Timestamp = 2,
                    Witness = new Witness()
                    {
                        InvocationScript = Array.Empty<byte>(),
                        VerificationScript = Array.Empty<byte>()
                    },
                    PrevHash = UInt256.Zero,
                    MerkleRoot = UInt256.Zero,
                    PrimaryIndex = 1,
                    NextConsensus = UInt160.Zero,
                },
                Hashes = new[] { tx.Hash }
            };

            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using ScriptBuilder script = new();
            script.EmitDynamicCall(NativeContract.Ledger.Hash, "getBlock", block.Hash.ToArray());

            // Without block

            var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(1, engine.ResultStack.Count);
            Assert.IsTrue(engine.ResultStack.Peek().IsNull);

            // Not traceable block

            const byte Prefix_Transaction = 11;
            const byte Prefix_CurrentBlock = 12;

            TestUtils.BlocksAdd(snapshotCache, block.Hash, block);

            var height = snapshotCache[NativeContract.Ledger.CreateStorageKey(Prefix_CurrentBlock)].GetInteroperable<HashIndexState>();
            height.Index = block.Index + TestProtocolSettings.Default.MaxTraceableBlocks;

            snapshotCache.Add(NativeContract.Ledger.CreateStorageKey(Prefix_Transaction, tx.Hash), new StorageItem(new TransactionState
            {
                BlockIndex = block.Index,
                Transaction = tx
            }));

            engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(1, engine.ResultStack.Count);
            Assert.IsTrue(engine.ResultStack.Peek().IsNull);

            // With block

            height = snapshotCache[NativeContract.Ledger.CreateStorageKey(Prefix_CurrentBlock)].GetInteroperable<HashIndexState>();
            height.Index = block.Index;

            engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(1, engine.ResultStack.Count);

            var array = engine.ResultStack.Pop<VM.Types.Array>();
            Assert.AreEqual(block.Hash, new UInt256(array[0].GetSpan()));
        }

        [TestMethod]
        public void System_ExecutionEngine_GetScriptContainer()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            using ScriptBuilder script = new();
            script.EmitSysCall(ApplicationEngine.System_Runtime_GetScriptContainer);

            // Without tx

            var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.FAULT);
            Assert.AreEqual(0, engine.ResultStack.Count);

            // With tx

            var tx = new Transaction()
            {
                Script = new byte[] { 0x01 },
                Signers = new Signer[] {
                    new Signer()
                    {
                        Account = UInt160.Zero,
                        Scopes = WitnessScope.None,
                        AllowedContracts = Array.Empty<UInt160>(),
                        AllowedGroups = Array.Empty<ECPoint>(),
                        Rules = Array.Empty<WitnessRule>(),
                    }
                },
                Attributes = Array.Empty<TransactionAttribute>(),
                NetworkFee = 0x02,
                SystemFee = 0x03,
                Nonce = 0x04,
                ValidUntilBlock = 0x05,
                Version = 0x06,
                Witnesses = new Witness[] { new Witness() { VerificationScript = new byte[] { 0x07 } } },
            };

            engine = ApplicationEngine.Create(TriggerType.Application, tx, snapshotCache);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(1, engine.ResultStack.Count);

            var array = engine.ResultStack.Pop<VM.Types.Array>();
            Assert.AreEqual(tx.Hash, new UInt256(array[0].GetSpan()));
        }

        [TestMethod]
        public void System_Runtime_EpicPulseLeft()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using (var script = new ScriptBuilder())
            {
                script.Emit(OpCode.NOP);
                script.EmitSysCall(ApplicationEngine.System_Runtime_EpicPulseLeft);
                script.Emit(OpCode.NOP);
                script.EmitSysCall(ApplicationEngine.System_Runtime_EpicPulseLeft);
                script.Emit(OpCode.NOP);
                script.Emit(OpCode.NOP);
                script.Emit(OpCode.NOP);
                script.EmitSysCall(ApplicationEngine.System_Runtime_EpicPulseLeft);

                // Execute

                var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, epicpulse: 1_000_000_000);
                engine.LoadScript(script.ToArray());
                Assert.AreEqual(engine.Execute(), VMState.HALT);

                // Check the results

                CollectionAssert.AreEqual
                    (
                    engine.ResultStack.Select(u => (int)u.GetInteger()).ToArray(),
                    new int[] { 99_999_490, 99_998_980, 99_998_410 }
                    );
            }

            // Check test mode

            using (var script = new ScriptBuilder())
            {
                script.EmitSysCall(ApplicationEngine.System_Runtime_EpicPulseLeft);

                // Execute

                var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache);
                engine.LoadScript(script.ToArray());

                // Check the results

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsInstanceOfType(engine.ResultStack.Peek(), typeof(Integer));
                Assert.AreEqual(1999999520, engine.ResultStack.Pop().GetInteger());
            }
        }

        [TestMethod]
        public void System_Runtime_GetInvocationCounter()
        {
            ContractState contractA, contractB, contractC;
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            // Create dummy contracts

            using (var script = new ScriptBuilder())
            {
                script.EmitSysCall(ApplicationEngine.System_Runtime_GetInvocationCounter);

                contractA = TestUtils.GetContract(new byte[] { (byte)OpCode.DROP, (byte)OpCode.DROP }.Concat(script.ToArray()).ToArray());
                contractB = TestUtils.GetContract(new byte[] { (byte)OpCode.DROP, (byte)OpCode.DROP, (byte)OpCode.NOP }.Concat(script.ToArray()).ToArray());
                contractC = TestUtils.GetContract(new byte[] { (byte)OpCode.DROP, (byte)OpCode.DROP, (byte)OpCode.NOP, (byte)OpCode.NOP }.Concat(script.ToArray()).ToArray());
                contractA.Hash = contractA.Script.Span.ToScriptHash();
                contractB.Hash = contractB.Script.Span.ToScriptHash();
                contractC.Hash = contractC.Script.Span.ToScriptHash();

                // Init A,B,C contracts
                // First two drops is for drop method and arguments

                snapshotCache.DeleteContract(contractA.Hash);
                snapshotCache.DeleteContract(contractB.Hash);
                snapshotCache.DeleteContract(contractC.Hash);
                contractA.Manifest = TestUtils.CreateManifest("dummyMain", ContractParameterType.Any, ContractParameterType.String, ContractParameterType.Integer);
                contractB.Manifest = TestUtils.CreateManifest("dummyMain", ContractParameterType.Any, ContractParameterType.String, ContractParameterType.Integer);
                contractC.Manifest = TestUtils.CreateManifest("dummyMain", ContractParameterType.Any, ContractParameterType.String, ContractParameterType.Integer);
                snapshotCache.AddContract(contractA.Hash, contractA);
                snapshotCache.AddContract(contractB.Hash, contractB);
                snapshotCache.AddContract(contractC.Hash, contractC);
            }

            // Call A,B,B,C

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(contractA.Hash, "dummyMain", "0", 1);
                script.EmitDynamicCall(contractB.Hash, "dummyMain", "0", 1);
                script.EmitDynamicCall(contractB.Hash, "dummyMain", "0", 1);
                script.EmitDynamicCall(contractC.Hash, "dummyMain", "0", 1);

                // Execute

                var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, null, ProtocolSettings.Default);
                engine.LoadScript(script.ToArray());
                Assert.AreEqual(VMState.HALT, engine.Execute());

                // Check the results

                CollectionAssert.AreEqual
                    (
                    engine.ResultStack.Select(u => (int)u.GetInteger()).ToArray(),
                    new int[]
                        {
                        1, /* A */
                        1, /* B */
                        2, /* B */
                        1  /* C */
                        }
                    );
            }
        }
    }
}

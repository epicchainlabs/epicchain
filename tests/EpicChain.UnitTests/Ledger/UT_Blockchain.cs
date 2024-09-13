// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Blockchain.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.TestKit;
using Akka.TestKit.Xunit2;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using EpicChain.Wallets.XEP6;
using System;
using System.Linq;
using System.Numerics;

namespace EpicChain.UnitTests.Ledger
{
    [TestClass]
    public class UT_Blockchain : TestKit
    {
        private EpicChainSystem system;
        private Transaction txSample;
        private TestProbe senderProbe;

        [TestInitialize]
        public void Initialize()
        {
            system = TestBlockchain.TheEpicChainSystem;
            senderProbe = CreateTestProbe();
            txSample = new Transaction
            {
                Attributes = [],
                Script = Array.Empty<byte>(),
                Signers = [new Signer { Account = UInt160.Zero }],
                Witnesses = []
            };
            system.MemPool.TryAdd(txSample, TestBlockchain.GetTestSnapshotCache());
        }

        [TestCleanup]
        public void Clean()
        {
            TestBlockchain.ResetStore();
        }

        [TestMethod]
        public void TestValidTransaction()
        {
            var snapshot = TestBlockchain.TheEpicChainSystem.GetSnapshotCache();
            var walletA = TestUtils.GenerateTestWallet("123");
            var acc = walletA.CreateAccount();

            // Fake balance

            var key = new KeyBuilder(NativeContract.EpicPulse.Id, 20).Add(acc.ScriptHash);
            var entry = snapshot.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 100_000_000 * NativeContract.EpicPulse.Factor;
            snapshot.Commit();

            // Make transaction

            var tx = TestUtils.CreateValidTx(snapshot, walletA, acc.ScriptHash, 0);

            senderProbe.Send(system.Blockchain, tx);
            senderProbe.ExpectMsg<Blockchain.RelayResult>(p => p.Result == VerifyResult.Succeed);

            senderProbe.Send(system.Blockchain, tx);
            senderProbe.ExpectMsg<Blockchain.RelayResult>(p => p.Result == VerifyResult.AlreadyInPool);
        }

        internal static StorageKey CreateStorageKey(byte prefix, byte[] key = null)
        {
            byte[] buffer = GC.AllocateUninitializedArray<byte>(sizeof(byte) + (key?.Length ?? 0));
            buffer[0] = prefix;
            key?.CopyTo(buffer.AsSpan(1));
            return new()
            {
                Id = NativeContract.EpicChain.Id,
                Key = buffer
            };
        }


        [TestMethod]
        public void TestMaliciousOnChainConflict()
        {
            var snapshot = TestBlockchain.TheEpicChainSystem.GetSnapshotCache();
            var walletA = TestUtils.GenerateTestWallet("123");
            var accA = walletA.CreateAccount();
            var walletB = TestUtils.GenerateTestWallet("456");
            var accB = walletB.CreateAccount();
            ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: long.MaxValue);
            engine.LoadScript(Array.Empty<byte>());

            // Fake balance for accounts A and B.
            var key = new KeyBuilder(NativeContract.EpicPulse.Id, 20).Add(accA.ScriptHash);
            var entry = snapshot.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 100_000_000 * NativeContract.EpicPulse.Factor;
            snapshot.Commit();

            key = new KeyBuilder(NativeContract.EpicPulse.Id, 20).Add(accB.ScriptHash);
            entry = snapshot.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 100_000_000 * NativeContract.EpicPulse.Factor;
            snapshot.Commit();

            // Create transactions:
            //    tx1 conflicts with tx2 and has the same sender (thus, it's a valid conflict and must prevent tx2 from entering the chain);
            //    tx2 conflicts with tx3 and has different sender (thus, this conflict is invalid and must not prevent tx3 from entering the chain).
            var tx1 = TestUtils.CreateValidTx(snapshot, walletA, accA.ScriptHash, 0);
            var tx2 = TestUtils.CreateValidTx(snapshot, walletA, accA.ScriptHash, 1);
            var tx3 = TestUtils.CreateValidTx(snapshot, walletB, accB.ScriptHash, 2);

            tx1.Attributes = new TransactionAttribute[] { new Conflicts() { Hash = tx2.Hash }, new Conflicts() { Hash = tx3.Hash } };

            // Persist tx1.
            var block = new Block
            {
                Header = new Header()
                {
                    Index = 5, // allow tx1, tx2 and tx3 to fit into MaxValidUntilBlockIncrement.
                    MerkleRoot = UInt256.Zero,
                    NextConsensus = UInt160.Zero,
                    PrevHash = UInt256.Zero,
                    Witness = new Witness() { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
                },
                Transactions = new Transaction[] { tx1 },
            };
            byte[] onPersistScript;
            using (ScriptBuilder sb = new())
            {
                sb.EmitSysCall(ApplicationEngine.System_Contract_NativeOnPersist);
                onPersistScript = sb.ToArray();
            }
            using (ApplicationEngine engine2 = ApplicationEngine.Create(TriggerType.OnPersist, null, snapshot, block, TestBlockchain.TheEpicChainSystem.Settings, 0))
            {
                engine2.LoadScript(onPersistScript);
                if (engine2.Execute() != VMState.HALT) throw engine2.FaultException;
                engine2.SnapshotCache.Commit();
            }
            snapshot.Commit();

            // Run PostPersist to update current block index in native Ledger.
            // Relevant current block index is needed for conflict records checks.
            byte[] postPersistScript;
            using (ScriptBuilder sb = new())
            {
                sb.EmitSysCall(ApplicationEngine.System_Contract_NativePostPersist);
                postPersistScript = sb.ToArray();
            }
            using (ApplicationEngine engine2 = ApplicationEngine.Create(TriggerType.PostPersist, null, snapshot, block, TestBlockchain.TheEpicChainSystem.Settings, 0))
            {
                engine2.LoadScript(postPersistScript);
                if (engine2.Execute() != VMState.HALT) throw engine2.FaultException;
                engine2.SnapshotCache.Commit();
            }
            snapshot.Commit();

            // Add tx2: must fail because valid conflict is alredy on chain (tx1).
            senderProbe.Send(TestBlockchain.TheEpicChainSystem.Blockchain, tx2);
            senderProbe.ExpectMsg<Blockchain.RelayResult>(p => p.Result == VerifyResult.HasConflicts);

            // Add tx3: must succeed because on-chain conflict is invalid (doesn't have proper signer).
            senderProbe.Send(TestBlockchain.TheEpicChainSystem.Blockchain, tx3);
            senderProbe.ExpectMsg<Blockchain.RelayResult>(p => p.Result == VerifyResult.Succeed);
        }
    }
}

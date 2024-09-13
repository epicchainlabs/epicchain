// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_TransactionVerificationContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Moq;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using System;
using System.Collections.Generic;
using System.Numerics;
using System.Threading.Tasks;

namespace EpicChain.UnitTests.Ledger
{
    [TestClass]
    public class UT_TransactionVerificationContext
    {
        [ClassInitialize]
        public static void TestSetup(TestContext ctx)
        {
            _ = TestBlockchain.TheEpicChainSystem;
        }

        private Transaction CreateTransactionWithFee(long networkFee, long systemFee)
        {
            Random random = new();
            var randomBytes = new byte[16];
            random.NextBytes(randomBytes);
            Mock<Transaction> mock = new();
            mock.Setup(p => p.VerifyStateDependent(It.IsAny<ProtocolSettings>(), It.IsAny<ClonedCache>(), It.IsAny<TransactionVerificationContext>(), It.IsAny<IEnumerable<Transaction>>())).Returns(VerifyResult.Succeed);
            mock.Setup(p => p.VerifyStateIndependent(It.IsAny<ProtocolSettings>())).Returns(VerifyResult.Succeed);
            mock.Object.Script = randomBytes;
            mock.Object.NetworkFee = networkFee;
            mock.Object.SystemFee = systemFee;
            mock.Object.Signers = new[] { new Signer { Account = UInt160.Zero } };
            mock.Object.Attributes = Array.Empty<TransactionAttribute>();
            mock.Object.Witnesses = new[]
            {
                new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = Array.Empty<byte>()
                }
            };
            return mock.Object;
        }

        [TestMethod]
        public async Task TestDuplicateOracle()
        {
            // Fake balance
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: long.MaxValue);
            BigInteger balance = NativeContract.EpicPulse.BalanceOf(snapshotCache, UInt160.Zero);
            await NativeContract.EpicPulse.Burn(engine, UInt160.Zero, balance);
            _ = NativeContract.EpicPulse.Mint(engine, UInt160.Zero, 8, false);

            // Test
            TransactionVerificationContext verificationContext = new();
            var tx = CreateTransactionWithFee(1, 2);
            tx.Attributes = new TransactionAttribute[] { new OracleResponse() { Code = OracleResponseCode.ConsensusUnreachable, Id = 1, Result = Array.Empty<byte>() } };
            var conflicts = new List<Transaction>();
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue();
            verificationContext.AddTransaction(tx);

            tx = CreateTransactionWithFee(2, 1);
            tx.Attributes = new TransactionAttribute[] { new OracleResponse() { Code = OracleResponseCode.ConsensusUnreachable, Id = 1, Result = Array.Empty<byte>() } };
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeFalse();
        }

        [TestMethod]
        public async Task TestTransactionSenderFee()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: long.MaxValue);
            BigInteger balance = NativeContract.EpicPulse.BalanceOf(snapshotCache, UInt160.Zero);
            await NativeContract.EpicPulse.Burn(engine, UInt160.Zero, balance);
            _ = NativeContract.EpicPulse.Mint(engine, UInt160.Zero, 8, true);

            TransactionVerificationContext verificationContext = new();
            var tx = CreateTransactionWithFee(1, 2);
            var conflicts = new List<Transaction>();
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue();
            verificationContext.AddTransaction(tx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue();
            verificationContext.AddTransaction(tx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeFalse();
            verificationContext.RemoveTransaction(tx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue();
            verificationContext.AddTransaction(tx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeFalse();
        }

        [TestMethod]
        public async Task TestTransactionSenderFeeWithConflicts()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: long.MaxValue);
            BigInteger balance = NativeContract.EpicPulse.BalanceOf(snapshotCache, UInt160.Zero);
            await NativeContract.EpicPulse.Burn(engine, UInt160.Zero, balance);
            _ = NativeContract.EpicPulse.Mint(engine, UInt160.Zero, 3 + 3 + 1, true); // balance is enough for 2 transactions and 1 EpicPulse is left.

            TransactionVerificationContext verificationContext = new();
            var tx = CreateTransactionWithFee(1, 2);
            var conflictingTx = CreateTransactionWithFee(1, 1); // costs 2 e=EpiCpulse

            var conflicts = new List<Transaction>();
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue();
            verificationContext.AddTransaction(tx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue();
            verificationContext.AddTransaction(tx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeFalse();

            conflicts.Add(conflictingTx);
            verificationContext.CheckTransaction(tx, conflicts, snapshotCache).Should().BeTrue(); // 1 EpicPulse is left on the balance + 2 EpicPulse is free after conflicts removal => enough for one more trasnaction.
        }
    }
}

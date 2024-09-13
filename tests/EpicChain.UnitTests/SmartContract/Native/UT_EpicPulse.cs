// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_EpicPulse.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests.Extensions;
using System;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;

namespace EpicChain.UnitTests.SmartContract.Native
{
    [TestClass]
    public class UT_EpicPulse
    {
        private DataCache _snapshotCache;
        private Block _persistingBlock;

        [TestInitialize]
        public void TestSetup()
        {
            _snapshotCache = TestBlockchain.GetTestSnapshotCache();
            _persistingBlock = new Block { Header = new Header() };
        }

        [TestMethod]
        public void Check_Name() => NativeContract.EpicPulse.Name.Should().Be(nameof(EpicPulse));

        [TestMethod]
        public void Check_Symbol() => NativeContract.EpicPulse.Symbol(_snapshotCache).Should().Be("EpicPulse");

        [TestMethod]
        public void Check_Decimals() => NativeContract.EpicPulse.Decimals(_snapshotCache).Should().Be(8);

        [TestMethod]
        public async Task Check_BalanceOfTransferAndBurn()
        {
            var snapshot = _snapshotCache.CloneCache();
            var persistingBlock = new Block { Header = new Header { Index = 1000 } };
            byte[] from = Contract.GetBFTAddress(TestProtocolSettings.Default.StandbyValidators).ToArray();
            byte[] to = new byte[20];
            var supply = NativeContract.EpicPulse.TotalSupply(snapshot);
            supply.Should().Be(5200000050000000); // 3000000000000000 + 50000000 (epicchain holder reward)

            var storageKey = new KeyBuilder(NativeContract.Ledger.Id, 12);
            snapshot.Add(storageKey, new StorageItem(new HashIndexState { Hash = UInt256.Zero, Index = persistingBlock.Index - 1 }));
            var keyCount = snapshot.GetChangeSet().Count();
            // Check unclaim

            var unclaim = UT_EpicChain.Check_UnclaimedEpicPulse(snapshot, from, persistingBlock);
            unclaim.Value.Should().Be(new BigInteger(0.5 * 1000 * 100000000L));
            unclaim.State.Should().BeTrue();

            // Transfer

            NativeContract.EpicChain.Transfer(snapshot, from, to, BigInteger.Zero, true, persistingBlock).Should().BeTrue();
            Assert.ThrowsException<ArgumentNullException>(() => NativeContract.EpicChain.Transfer(snapshot, from, null, BigInteger.Zero, true, persistingBlock));
            Assert.ThrowsException<ArgumentNullException>(() => NativeContract.EpicChain.Transfer(snapshot, null, to, BigInteger.Zero, false, persistingBlock));
            NativeContract.EpicChain.BalanceOf(snapshot, from).Should().Be(100000000);
            NativeContract.EpicChain.BalanceOf(snapshot, to).Should().Be(0);

            NativeContract.EpicPulse.BalanceOf(snapshot, from).Should().Be(52000500_00000000);
            NativeContract.EpicPulse.BalanceOf(snapshot, to).Should().Be(0);

            // Check unclaim

            unclaim = UT_EpicChain.Check_UnclaimedEpicPulse(snapshot, from, persistingBlock);
            unclaim.Value.Should().Be(new BigInteger(0));
            unclaim.State.Should().BeTrue();

            supply = NativeContract.EpicPulse.TotalSupply(snapshot);
            supply.Should().Be(5200050050000000);

            snapshot.GetChangeSet().Count().Should().Be(keyCount + 3); // EpicPulse

            // Transfer

            keyCount = snapshot.GetChangeSet().Count();

            NativeContract.EpicPulse.Transfer(snapshot, from, to, 52000500_00000000, false, persistingBlock).Should().BeFalse(); // Not signed
            NativeContract.EpicPulse.Transfer(snapshot, from, to, 52000500_00000001, true, persistingBlock).Should().BeFalse(); // More than balance
            NativeContract.EpicPulse.Transfer(snapshot, from, to, 52000500_00000000, true, persistingBlock).Should().BeTrue(); // All balance

            // Balance of

            NativeContract.EpicPulse.BalanceOf(snapshot, to).Should().Be(52000500_00000000);
            NativeContract.EpicPulse.BalanceOf(snapshot, from).Should().Be(0);

            snapshot.GetChangeSet().Count().Should().Be(keyCount + 1); // All

            // Burn

            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, persistingBlock, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: 0);
            engine.LoadScript(Array.Empty<byte>());

            await Assert.ThrowsExceptionAsync<ArgumentOutOfRangeException>(async () =>
                await NativeContract.EpicPulse.Burn(engine, new UInt160(to), BigInteger.MinusOne));

            // Burn more than expected

            await Assert.ThrowsExceptionAsync<InvalidOperationException>(async () =>
                await NativeContract.EpicPulse.Burn(engine, new UInt160(to), new BigInteger(52000500_00000001)));

            // Real burn

            await NativeContract.EpicPulse.Burn(engine, new UInt160(to), new BigInteger(1));

            NativeContract.EpicPulse.BalanceOf(engine.SnapshotCache, to).Should().Be(5200049999999999);

            engine.SnapshotCache.GetChangeSet().Count().Should().Be(2);

            // Burn all
            await NativeContract.EpicPulse.Burn(engine, new UInt160(to), new BigInteger(5200049999999999));

            (keyCount - 2).Should().Be(engine.SnapshotCache.GetChangeSet().Count());

            // Bad inputs

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => NativeContract.EpicPulse.Transfer(engine.SnapshotCache, from, to, BigInteger.MinusOne, true, persistingBlock));
            Assert.ThrowsException<FormatException>(() => NativeContract.EpicPulse.Transfer(engine.SnapshotCache, new byte[19], to, BigInteger.One, false, persistingBlock));
            Assert.ThrowsException<FormatException>(() => NativeContract.EpicPulse.Transfer(engine.SnapshotCache, from, new byte[19], BigInteger.One, false, persistingBlock));
        }

        internal static StorageKey CreateStorageKey(byte prefix, uint key)
        {
            return CreateStorageKey(prefix, BitConverter.GetBytes(key));
        }

        internal static StorageKey CreateStorageKey(byte prefix, byte[] key = null)
        {
            byte[] buffer = GC.AllocateUninitializedArray<byte>(sizeof(byte) + (key?.Length ?? 0));
            buffer[0] = prefix;
            key?.CopyTo(buffer.AsSpan(1));
            return new()
            {
                Id = NativeContract.EpicPulse.Id,
                Key = buffer
            };
        }
    }
}

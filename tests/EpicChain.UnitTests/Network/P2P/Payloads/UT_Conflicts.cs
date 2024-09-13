// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Conflicts.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System;

namespace EpicChain.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_Conflicts
    {
        private const byte Prefix_Transaction = 11;
        private static readonly UInt256 _u = new UInt256(new byte[32] {
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01
            });

        [TestMethod]
        public void Size_Get()
        {
            var test = new Conflicts() { Hash = _u };
            test.Size.Should().Be(1 + 32);
        }

        [TestMethod]
        public void ToJson()
        {
            var test = new Conflicts() { Hash = _u };
            var json = test.ToJson().ToString();
            Assert.AreEqual(@"{""type"":""Conflicts"",""hash"":""0x0101010101010101010101010101010101010101010101010101010101010101""}", json);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = new Conflicts() { Hash = _u };

            var clone = test.ToArray().AsSerializable<Conflicts>();
            Assert.AreEqual(clone.Type, test.Type);

            // As transactionAttribute
            byte[] buffer = test.ToArray();
            var reader = new MemoryReader(buffer);
            clone = TransactionAttribute.DeserializeFrom(ref reader) as Conflicts;
            Assert.AreEqual(clone.Type, test.Type);

            // Wrong type
            buffer[0] = 0xff;
            Assert.ThrowsException<FormatException>(() =>
            {
                var reader = new MemoryReader(buffer);
                TransactionAttribute.DeserializeFrom(ref reader);
            });
        }

        [TestMethod]
        public void Verify()
        {
            var test = new Conflicts() { Hash = _u };
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var key = Ledger.UT_MemoryPool.CreateStorageKey(NativeContract.Ledger.Id, Prefix_Transaction, _u.ToArray());

            // Conflicting transaction is in the Conflicts attribute of some other on-chain transaction.
            var conflict = new TransactionState();
            snapshotCache.Add(key, new StorageItem(conflict));
            Assert.IsTrue(test.Verify(snapshotCache, new Transaction()));

            // Conflicting transaction is on-chain.
            snapshotCache.Delete(key);
            conflict = new TransactionState
            {
                BlockIndex = 123,
                Transaction = new Transaction(),
                State = VMState.NONE
            };
            snapshotCache.Add(key, new StorageItem(conflict));
            Assert.IsFalse(test.Verify(snapshotCache, new Transaction()));

            // There's no conflicting transaction at all.
            snapshotCache.Delete(key);
            Assert.IsTrue(test.Verify(snapshotCache, new Transaction()));
        }
    }
}

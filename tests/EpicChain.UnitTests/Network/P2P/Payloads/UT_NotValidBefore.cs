// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_NotValidBefore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract.Native;
using System;

namespace EpicChain.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_NotValidBefore
    {
        [TestMethod]
        public void Size_Get()
        {
            var test = new NotValidBefore();
            test.Size.Should().Be(5);
        }

        [TestMethod]
        public void ToJson()
        {
            var test = new NotValidBefore
            {
                Height = 42
            };
            var json = test.ToJson().ToString();
            Assert.AreEqual(@"{""type"":""NotValidBefore"",""height"":42}", json);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = new NotValidBefore();

            var clone = test.ToArray().AsSerializable<NotValidBefore>();
            Assert.AreEqual(clone.Type, test.Type);

            // As transactionAttribute

            byte[] buffer = test.ToArray();
            var reader = new MemoryReader(buffer);
            clone = TransactionAttribute.DeserializeFrom(ref reader) as NotValidBefore;
            Assert.AreEqual(clone.Type, test.Type);

            // Wrong type

            buffer[0] = 0xff;
            reader = new MemoryReader(buffer);
            try
            {
                TransactionAttribute.DeserializeFrom(ref reader);
                Assert.Fail();
            }
            catch (FormatException) { }
            reader = new MemoryReader(buffer);
            try
            {
                new NotValidBefore().Deserialize(ref reader);
                Assert.Fail();
            }
            catch (FormatException) { }
        }

        [TestMethod]
        public void Verify()
        {
            var test = new NotValidBefore();
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            test.Height = NativeContract.Ledger.CurrentIndex(snapshotCache) + 1;

            Assert.IsFalse(test.Verify(snapshotCache, new Transaction()));
            test.Height--;
            Assert.IsTrue(test.Verify(snapshotCache, new Transaction()));
        }
    }
}

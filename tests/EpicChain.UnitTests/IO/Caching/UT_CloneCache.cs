// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_CloneCache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace EpicChain.UnitTests.IO.Caching
{
    [TestClass]
    public class UT_CloneCache
    {
        private readonly MemoryStore store = new();
        private SnapshotCache myDataCache;
        private ClonedCache clonedCache;

        private static readonly StorageKey key1 = new() { Id = 0, Key = Encoding.UTF8.GetBytes("key1") };
        private static readonly StorageKey key2 = new() { Id = 0, Key = Encoding.UTF8.GetBytes("key2") };
        private static readonly StorageKey key3 = new() { Id = 0, Key = Encoding.UTF8.GetBytes("key3") };
        private static readonly StorageKey key4 = new() { Id = 0, Key = Encoding.UTF8.GetBytes("key4") };

        private static readonly StorageItem value1 = new(Encoding.UTF8.GetBytes("value1"));
        private static readonly StorageItem value2 = new(Encoding.UTF8.GetBytes("value2"));
        private static readonly StorageItem value3 = new(Encoding.UTF8.GetBytes("value3"));
        private static readonly StorageItem value4 = new(Encoding.UTF8.GetBytes("value4"));

        [TestInitialize]
        public void Init()
        {
            myDataCache = new(store);
            clonedCache = new ClonedCache(myDataCache);
        }

        [TestMethod]
        public void TestCloneCache()
        {
            clonedCache.Should().NotBeNull();
        }

        [TestMethod]
        public void TestAddInternal()
        {
            clonedCache.Add(key1, value1);
            clonedCache[key1].Should().Be(value1);

            clonedCache.Commit();
            Assert.IsTrue(myDataCache[key1].Value.Span.SequenceEqual(value1.Value.Span));
        }

        [TestMethod]
        public void TestDeleteInternal()
        {
            myDataCache.Add(key1, value1);
            clonedCache.Delete(key1);   //  trackable.State = TrackState.Deleted
            clonedCache.Commit();

            clonedCache.TryGet(key1).Should().BeNull();
            myDataCache.TryGet(key1).Should().BeNull();
        }

        [TestMethod]
        public void TestFindInternal()
        {
            clonedCache.Add(key1, value1);
            myDataCache.Add(key2, value2);
            store.Put(key3.ToArray(), value3.ToArray());

            var items = clonedCache.Find(key1.ToArray());
            items.ElementAt(0).Key.Should().Be(key1);
            items.ElementAt(0).Value.Should().Be(value1);
            items.Count().Should().Be(1);

            items = clonedCache.Find(key2.ToArray());
            items.ElementAt(0).Key.Should().Be(key2);
            value2.EqualsTo(items.ElementAt(0).Value).Should().BeTrue();
            items.Count().Should().Be(1);

            items = clonedCache.Find(key3.ToArray());
            items.ElementAt(0).Key.Should().Be(key3);
            value3.EqualsTo(items.ElementAt(0).Value).Should().BeTrue();
            items.Count().Should().Be(1);

            items = clonedCache.Find(key4.ToArray());
            items.Count().Should().Be(0);
        }

        [TestMethod]
        public void TestGetInternal()
        {
            clonedCache.Add(key1, value1);
            myDataCache.Add(key2, value2);
            store.Put(key3.ToArray(), value3.ToArray());

            value1.EqualsTo(clonedCache[key1]).Should().BeTrue();
            value2.EqualsTo(clonedCache[key2]).Should().BeTrue();
            value3.EqualsTo(clonedCache[key3]).Should().BeTrue();

            Action action = () =>
            {
                var item = clonedCache[key4];
            };
            action.Should().Throw<KeyNotFoundException>();
        }

        [TestMethod]
        public void TestTryGetInternal()
        {
            clonedCache.Add(key1, value1);
            myDataCache.Add(key2, value2);
            store.Put(key3.ToArray(), value3.ToArray());

            value1.EqualsTo(clonedCache.TryGet(key1)).Should().BeTrue();
            value2.EqualsTo(clonedCache.TryGet(key2)).Should().BeTrue();
            value3.EqualsTo(clonedCache.TryGet(key3)).Should().BeTrue();
            clonedCache.TryGet(key4).Should().BeNull();
        }

        [TestMethod]
        public void TestUpdateInternal()
        {
            clonedCache.Add(key1, value1);
            myDataCache.Add(key2, value2);
            store.Put(key3.ToArray(), value3.ToArray());

            clonedCache.GetAndChange(key1).Value = Encoding.Default.GetBytes("value_new_1");
            clonedCache.GetAndChange(key2).Value = Encoding.Default.GetBytes("value_new_2");
            clonedCache.GetAndChange(key3).Value = Encoding.Default.GetBytes("value_new_3");

            clonedCache.Commit();

            StorageItem value_new_1 = new(Encoding.UTF8.GetBytes("value_new_1"));
            StorageItem value_new_2 = new(Encoding.UTF8.GetBytes("value_new_2"));
            StorageItem value_new_3 = new(Encoding.UTF8.GetBytes("value_new_3"));

            value_new_1.EqualsTo(clonedCache[key1]).Should().BeTrue();
            value_new_2.EqualsTo(clonedCache[key2]).Should().BeTrue();
            value_new_3.EqualsTo(clonedCache[key3]).Should().BeTrue();
            value_new_2.EqualsTo(clonedCache[key2]).Should().BeTrue();
        }

        [TestMethod]
        public void TestCacheOverrideIssue2572()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var storages = snapshotCache.CreateSnapshot();

            storages.Add
                (
                new StorageKey() { Key = new byte[] { 0x00, 0x01 }, Id = 0 },
                new StorageItem() { Value = Array.Empty<byte>() }
                );
            storages.Add
                (
                new StorageKey() { Key = new byte[] { 0x01, 0x01 }, Id = 0 },
                new StorageItem() { Value = new byte[] { 0x05 } }
                );

            storages.Commit();

            var item = storages.GetAndChange(new StorageKey() { Key = new byte[] { 0x01, 0x01 }, Id = 0 });
            item.Value = new byte[] { 0x06 };

            var res = snapshotCache.TryGet(new StorageKey() { Key = new byte[] { 0x01, 0x01 }, Id = 0 });
            Assert.AreEqual("05", res.Value.Span.ToHexString());
            storages.Commit();
            res = snapshotCache.TryGet(new StorageKey() { Key = new byte[] { 0x01, 0x01 }, Id = 0 });
            Assert.AreEqual("06", res.Value.Span.ToHexString());
        }
    }
}

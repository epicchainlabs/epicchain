// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// UT_CloneCache.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using Neo.Persistence;
using Neo.SmartContract;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace Neo.UnitTests.IO.Caching
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
            var snapshot = TestBlockchain.GetTestSnapshot();
            var storages = snapshot.CreateSnapshot();

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

            var res = snapshot.TryGet(new StorageKey() { Key = new byte[] { 0x01, 0x01 }, Id = 0 });
            Assert.AreEqual("05", res.Value.Span.ToHexString());
            storages.Commit();
            res = snapshot.TryGet(new StorageKey() { Key = new byte[] { 0x01, 0x01 }, Id = 0 });
            Assert.AreEqual("06", res.Value.Span.ToHexString());
        }
    }
}

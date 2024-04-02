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
// UT_Cache.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO.Caching;
using System;
using System.Collections;
using System.Collections.Generic;

namespace Neo.UnitTests.IO.Caching
{
    class MyCache : Cache<int, string>
    {
        public MyCache(int max_capacity) : base(max_capacity) { }

        protected override int GetKeyForItem(string item)
        {
            return item.GetHashCode();
        }

        protected override void OnAccess(CacheItem item) { }

        public IEnumerator MyGetEnumerator()
        {
            IEnumerable enumerable = this;
            return enumerable.GetEnumerator();
        }
    }

    class CacheDisposableEntry : IDisposable
    {
        public int Key { get; set; }
        public bool IsDisposed { get; private set; }

        public void Dispose()
        {
            IsDisposed = true;
        }
    }

    class MyDisposableCache : Cache<int, CacheDisposableEntry>
    {
        public MyDisposableCache(int max_capacity) : base(max_capacity) { }

        protected override int GetKeyForItem(CacheDisposableEntry item)
        {
            return item.Key;
        }

        protected override void OnAccess(CacheItem item) { }

        public IEnumerator MyGetEnumerator()
        {
            IEnumerable enumerable = this;
            return enumerable.GetEnumerator();
        }
    }

    [TestClass]
    public class UT_Cache
    {
        MyCache cache;
        readonly int max_capacity = 4;

        [TestInitialize]
        public void Init()
        {
            cache = new MyCache(max_capacity);
        }

        [TestMethod]
        public void TestCount()
        {
            cache.Count.Should().Be(0);

            cache.Add("hello");
            cache.Add("world");
            cache.Count.Should().Be(2);

            cache.Remove("hello");
            cache.Count.Should().Be(1);
        }

        [TestMethod]
        public void TestIsReadOnly()
        {
            cache.IsReadOnly.Should().BeFalse();
        }

        [TestMethod]
        public void TestAddAndAddInternal()
        {
            cache.Add("hello");
            cache.Contains("hello").Should().BeTrue();
            cache.Contains("world").Should().BeFalse();
            cache.Add("hello");
            cache.Count.Should().Be(1);
        }

        [TestMethod]
        public void TestAddRange()
        {
            string[] range = { "hello", "world" };
            cache.AddRange(range);
            cache.Count.Should().Be(2);
            cache.Contains("hello").Should().BeTrue();
            cache.Contains("world").Should().BeTrue();
            cache.Contains("non exist string").Should().BeFalse();
        }

        [TestMethod]
        public void TestClear()
        {
            cache.Add("hello");
            cache.Add("world");
            cache.Count.Should().Be(2);
            cache.Clear();
            cache.Count.Should().Be(0);
        }

        [TestMethod]
        public void TestContainsKey()
        {
            cache.Add("hello");
            cache.Contains("hello").Should().BeTrue();
            cache.Contains("world").Should().BeFalse();
        }

        [TestMethod]
        public void TestContainsValue()
        {
            cache.Add("hello");
            cache.Contains("hello".GetHashCode()).Should().BeTrue();
            cache.Contains("world".GetHashCode()).Should().BeFalse();
        }

        [TestMethod]
        public void TestCopyTo()
        {
            cache.Add("hello");
            cache.Add("world");
            string[] temp = new string[2];

            Action action = () => cache.CopyTo(null, 1);
            action.Should().Throw<ArgumentNullException>();

            action = () => cache.CopyTo(temp, -1);
            action.Should().Throw<ArgumentOutOfRangeException>();

            action = () => cache.CopyTo(temp, 1);
            action.Should().Throw<ArgumentException>();

            cache.CopyTo(temp, 0);
            temp[0].Should().Be("hello");
            temp[1].Should().Be("world");
        }

        [TestMethod]
        public void TestRemoveKey()
        {
            cache.Add("hello");
            cache.Remove("hello".GetHashCode()).Should().BeTrue();
            cache.Remove("world".GetHashCode()).Should().BeFalse();
            cache.Contains("hello").Should().BeFalse();
        }

        [TestMethod]
        public void TestRemoveDisposableKey()
        {
            var entry = new CacheDisposableEntry() { Key = 1 };
            var dcache = new MyDisposableCache(100)
            {
                entry
            };

            entry.IsDisposed.Should().BeFalse();
            dcache.Remove(entry.Key).Should().BeTrue();
            dcache.Remove(entry.Key).Should().BeFalse();
            entry.IsDisposed.Should().BeTrue();
        }

        [TestMethod]
        public void TestRemoveValue()
        {
            cache.Add("hello");
            cache.Remove("hello").Should().BeTrue();
            cache.Remove("world").Should().BeFalse();
            cache.Contains("hello").Should().BeFalse();
        }

        [TestMethod]
        public void TestTryGet()
        {
            cache.Add("hello");
            cache.TryGet("hello".GetHashCode(), out string output).Should().BeTrue();
            output.Should().Be("hello");
            cache.TryGet("world".GetHashCode(), out string output2).Should().BeFalse();
            output2.Should().NotBe("world");
            output2.Should().BeNull();
        }

        [TestMethod]
        public void TestArrayIndexAccess()
        {
            cache.Add("hello");
            cache.Add("world");
            cache["hello".GetHashCode()].Should().Be("hello");
            cache["world".GetHashCode()].Should().Be("world");

            Action action = () =>
            {
                string temp = cache["non exist string".GetHashCode()];
            };
            action.Should().Throw<KeyNotFoundException>();
        }

        [TestMethod]
        public void TestGetEnumerator()
        {
            cache.Add("hello");
            cache.Add("world");
            int i = 0;
            foreach (string item in cache)
            {
                if (i == 0) item.Should().Be("hello");
                if (i == 1) item.Should().Be("world");
                i++;
            }
            i.Should().Be(2);
            cache.MyGetEnumerator().Should().NotBeNull();
        }

        [TestMethod]
        public void TestOverMaxCapacity()
        {
            int i = 1;
            for (; i <= max_capacity; i++)
            {
                cache.Add(i.ToString());
            }
            cache.Add(i.ToString());    // The first one will be deleted 
            cache.Count.Should().Be(max_capacity);
            cache.Contains((max_capacity + 1).ToString()).Should().BeTrue();
        }

        [TestMethod]
        public void TestDispose()
        {
            cache.Add("hello");
            cache.Add("world");
            cache.Dispose();

            Action action = () =>
            {
                int count = cache.Count;
            };
            action.Should().Throw<ObjectDisposedException>();
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ReflectionCache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO.Caching;
using System.IO;

namespace EpicChain.UnitTests.IO.Caching
{
    public class TestItem : ISerializable
    {
        public int Size => 0;
        public void Deserialize(ref MemoryReader reader) { }
        public void Serialize(BinaryWriter writer) { }
    }

    public class TestItem1 : TestItem { }

    public class TestItem2 : TestItem { }

    public enum MyTestEnum : byte
    {
        [ReflectionCache(typeof(TestItem1))]
        Item1 = 0x00,

        [ReflectionCache(typeof(TestItem2))]
        Item2 = 0x01,
    }

    public enum MyEmptyEnum : byte { }

    [TestClass]
    public class UT_ReflectionCache
    {
        [TestMethod]
        public void TestCreateFromEmptyEnum()
        {
            ReflectionCache<MyEmptyEnum>.Count.Should().Be(0);
        }

        [TestMethod]
        public void TestCreateInstance()
        {
            object item1 = ReflectionCache<MyTestEnum>.CreateInstance(MyTestEnum.Item1, null);
            (item1 is TestItem1).Should().BeTrue();

            object item2 = ReflectionCache<MyTestEnum>.CreateInstance(MyTestEnum.Item2, null);
            (item2 is TestItem2).Should().BeTrue();

            object item3 = ReflectionCache<MyTestEnum>.CreateInstance((MyTestEnum)0x02, null);
            item3.Should().BeNull();
        }

        [TestMethod]
        public void TestCreateSerializable()
        {
            object item1 = ReflectionCache<MyTestEnum>.CreateSerializable(MyTestEnum.Item1, new byte[0]);
            (item1 is TestItem1).Should().BeTrue();

            object item2 = ReflectionCache<MyTestEnum>.CreateSerializable(MyTestEnum.Item2, new byte[0]);
            (item2 is TestItem2).Should().BeTrue();

            object item3 = ReflectionCache<MyTestEnum>.CreateSerializable((MyTestEnum)0x02, new byte[0]);
            item3.Should().BeNull();
        }

        [TestMethod]
        public void TestCreateInstance2()
        {
            TestItem defaultItem = new TestItem1();
            object item2 = ReflectionCache<MyTestEnum>.CreateInstance(MyTestEnum.Item2, defaultItem);
            (item2 is TestItem2).Should().BeTrue();

            object item1 = ReflectionCache<MyTestEnum>.CreateInstance((MyTestEnum)0x02, new TestItem1());
            (item1 is TestItem1).Should().BeTrue();
        }
    }
}

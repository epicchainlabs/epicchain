// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_StorageKey.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;

namespace EpicChain.UnitTests.Ledger
{
    [TestClass]
    public class UT_StorageKey
    {
        [TestMethod]
        public void Id_Get()
        {
            var uut = new StorageKey { Id = 1, Key = new byte[] { 0x01 } };
            uut.Id.Should().Be(1);
        }

        [TestMethod]
        public void Id_Set()
        {
            int val = 1;
            StorageKey uut = new() { Id = val };
            uut.Id.Should().Be(val);
        }

        [TestMethod]
        public void Key_Set()
        {
            byte[] val = new byte[] { 0x42, 0x32 };
            StorageKey uut = new() { Key = val };
            uut.Key.Length.Should().Be(2);
            uut.Key.Span[0].Should().Be(val[0]);
            uut.Key.Span[1].Should().Be(val[1]);
        }

        [TestMethod]
        public void Equals_SameObj()
        {
            StorageKey uut = new();
            uut.Equals(uut).Should().BeTrue();
        }

        [TestMethod]
        public void Equals_Null()
        {
            StorageKey uut = new();
            uut.Equals(null).Should().BeFalse();
        }

        [TestMethod]
        public void Equals_SameHash_SameKey()
        {
            int val = 0x42000000;
            byte[] keyVal = TestUtils.GetByteArray(10, 0x42);
            StorageKey newSk = new StorageKey
            {
                Id = val,
                Key = keyVal
            };
            StorageKey uut = new() { Id = val, Key = keyVal };
            uut.Equals(newSk).Should().BeTrue();
        }

        [TestMethod]
        public void Equals_DiffHash_SameKey()
        {
            int val = 0x42000000;
            byte[] keyVal = TestUtils.GetByteArray(10, 0x42);
            StorageKey newSk = new StorageKey
            {
                Id = val,
                Key = keyVal
            };
            StorageKey uut = new() { Id = 0x78000000, Key = keyVal };
            uut.Equals(newSk).Should().BeFalse();
        }

        [TestMethod]
        public void Equals_SameHash_DiffKey()
        {
            int val = 0x42000000;
            byte[] keyVal = TestUtils.GetByteArray(10, 0x42);
            StorageKey newSk = new StorageKey
            {
                Id = val,
                Key = keyVal
            };
            StorageKey uut = new() { Id = val, Key = TestUtils.GetByteArray(10, 0x88) };
            uut.Equals(newSk).Should().BeFalse();
        }

        [TestMethod]
        public void GetHashCode_Get()
        {
            StorageKey uut = new() { Id = 0x42000000, Key = TestUtils.GetByteArray(10, 0x42) };
            uut.GetHashCode().Should().Be(1374529787);
        }

        [TestMethod]
        public void Equals_Obj()
        {
            StorageKey uut = new();
            uut.Equals(1u).Should().BeFalse();
            uut.Equals((object)uut).Should().BeTrue();
        }
    }
}

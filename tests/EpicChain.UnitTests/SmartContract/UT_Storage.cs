// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Storage.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Linq;
using System.Numerics;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_Storage
    {
        [TestMethod]
        public void TestStorageKey()
        {
            // Test data
            byte[] keyData = [0x00, 0x00, 0x00, 0x00, 0x12];
            var keyMemory = new ReadOnlyMemory<byte>(keyData);

            // Test implicit conversion from byte[] to StorageKey
            StorageKey storageKeyFromArray = keyData;
            Assert.AreEqual(0, storageKeyFromArray.Id);
            Assert.IsTrue(keyMemory.Span.ToArray().Skip(sizeof(int)).SequenceEqual(storageKeyFromArray.Key.Span.ToArray()));

            // Test implicit conversion from ReadOnlyMemory<byte> to StorageKey
            StorageKey storageKeyFromMemory = keyMemory;
            Assert.AreEqual(0, storageKeyFromMemory.Id);
            Assert.IsTrue(keyMemory.Span.ToArray().Skip(sizeof(int)).SequenceEqual(storageKeyFromMemory.Key.Span.ToArray()));

            // Test CreateSearchPrefix method
            byte[] prefix = { 0xAA };
            var searchPrefix = StorageKey.CreateSearchPrefix(0, prefix);
            var expectedPrefix = BitConverter.GetBytes(0).Concat(prefix).ToArray();
            Assert.IsTrue(expectedPrefix.SequenceEqual(searchPrefix));

            // Test Equals method
            var storageKey1 = new StorageKey { Id = 0, Key = keyMemory };
            var storageKey2 = new StorageKey { Id = 0, Key = keyMemory };
            var storageKeyDifferentId = new StorageKey { Id = 0 + 1, Key = keyMemory };
            var storageKeyDifferentKey = new StorageKey { Id = 0, Key = new ReadOnlyMemory<byte>([0x04]) };
            Assert.AreEqual(storageKey1, storageKey2);
            Assert.AreNotEqual(storageKey1, storageKeyDifferentId);
            Assert.AreNotEqual(storageKey1, storageKeyDifferentKey);
        }

        [TestMethod]
        public void TestStorageItem()
        {
            // Test data
            byte[] keyData = [0x00, 0x00, 0x00, 0x00, 0x12];
            BigInteger bigInteger = new BigInteger(1234567890);

            // Test implicit conversion from byte[] to StorageItem
            StorageItem storageItemFromArray = keyData;
            Assert.IsTrue(keyData.SequenceEqual(storageItemFromArray.Value.Span.ToArray()));

            // Test implicit conversion from BigInteger to StorageItem
            StorageItem storageItemFromBigInteger = bigInteger;
            Assert.AreEqual(bigInteger, (BigInteger)storageItemFromBigInteger);
        }
    }
}

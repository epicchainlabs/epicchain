// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_StorageIterator.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract.Iterators;
using EpicChain.VM.Types;
using System;
using System.Collections.Generic;

namespace EpicChain.UnitTests.SmartContract.Iterators
{
    [TestClass]
    public class UT_StorageIterator
    {
        [TestMethod]
        public void TestGeneratorAndDispose()
        {
            StorageIterator storageIterator = new(new List<(StorageKey, StorageItem)>().GetEnumerator(), 0, FindOptions.None);
            Assert.IsNotNull(storageIterator);
            Action action = () => storageIterator.Dispose();
            action.Should().NotThrow<Exception>();
        }

        [TestMethod]
        public void TestKeyAndValueAndNext()
        {
            List<(StorageKey, StorageItem)> list = new();
            StorageKey storageKey = new()
            {
                Key = new byte[1]
            };
            StorageItem storageItem = new()
            {
                Value = new byte[1]
            };
            list.Add((storageKey, storageItem));
            StorageIterator storageIterator = new(list.GetEnumerator(), 0, FindOptions.ValuesOnly);
            storageIterator.Next();
            Assert.AreEqual(new ByteString(new byte[1]), storageIterator.Value(null));
        }
    }
}

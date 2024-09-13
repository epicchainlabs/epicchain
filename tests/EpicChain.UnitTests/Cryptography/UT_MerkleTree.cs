// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_MerkleTree.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography;
using EpicChain.IO;
using System;
using System.Collections;
using System.Linq;

namespace EpicChain.UnitTests.Cryptography
{
    [TestClass]
    public class UT_MerkleTree
    {
        public UInt256 GetByteArrayHash(byte[] byteArray)
        {
            if (byteArray == null || byteArray.Length == 0) throw new ArgumentNullException();
            var hash = new UInt256(Crypto.Hash256(byteArray));
            return hash;
        }

        [TestMethod]
        public void TestBuildAndDepthFirstSearch()
        {
            byte[] array1 = { 0x01 };
            var hash1 = GetByteArrayHash(array1);

            byte[] array2 = { 0x02 };
            var hash2 = GetByteArrayHash(array2);

            byte[] array3 = { 0x03 };
            var hash3 = GetByteArrayHash(array3);

            UInt256[] hashes = { hash1, hash2, hash3 };
            MerkleTree tree = new MerkleTree(hashes);
            var hashArray = tree.ToHashArray();
            hashArray[0].Should().Be(hash1);
            hashArray[1].Should().Be(hash2);
            hashArray[2].Should().Be(hash3);
            hashArray[3].Should().Be(hash3);

            var rootHash = MerkleTree.ComputeRoot(hashes);
            var hash4 = Crypto.Hash256(hash1.ToArray().Concat(hash2.ToArray()).ToArray());
            var hash5 = Crypto.Hash256(hash3.ToArray().Concat(hash3.ToArray()).ToArray());
            var result = new UInt256(Crypto.Hash256(hash4.ToArray().Concat(hash5.ToArray()).ToArray()));
            rootHash.Should().Be(result);
        }

        [TestMethod]
        public void TestTrim()
        {
            byte[] array1 = { 0x01 };
            var hash1 = GetByteArrayHash(array1);

            byte[] array2 = { 0x02 };
            var hash2 = GetByteArrayHash(array2);

            byte[] array3 = { 0x03 };
            var hash3 = GetByteArrayHash(array3);

            UInt256[] hashes = { hash1, hash2, hash3 };
            MerkleTree tree = new MerkleTree(hashes);

            bool[] boolArray = { false, false, false };
            BitArray bitArray = new BitArray(boolArray);
            tree.Trim(bitArray);
            var hashArray = tree.ToHashArray();

            hashArray.Length.Should().Be(1);
            var rootHash = MerkleTree.ComputeRoot(hashes);
            var hash4 = Crypto.Hash256(hash1.ToArray().Concat(hash2.ToArray()).ToArray());
            var hash5 = Crypto.Hash256(hash3.ToArray().Concat(hash3.ToArray()).ToArray());
            var result = new UInt256(Crypto.Hash256(hash4.ToArray().Concat(hash5.ToArray()).ToArray()));
            hashArray[0].Should().Be(result);
        }
    }
}

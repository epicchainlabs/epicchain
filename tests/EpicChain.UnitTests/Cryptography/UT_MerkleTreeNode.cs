// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_MerkleTreeNode.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Text;

namespace EpicChain.UnitTests.Cryptography
{
    [TestClass]
    public class UT_MerkleTreeNode
    {
        private readonly MerkleTreeNode node = new MerkleTreeNode();

        [TestInitialize]
        public void TestSetup()
        {
            node.Hash = null;
            node.Parent = null;
            node.LeftChild = null;
            node.RightChild = null;
        }

        [TestMethod]
        public void TestConstructor()
        {
            byte[] byteArray = Encoding.ASCII.GetBytes("hello world");
            var hash = new UInt256(Crypto.Hash256(byteArray));
            node.Hash = hash;

            node.Hash.Should().Be(hash);
            node.Parent.Should().BeNull();
            node.LeftChild.Should().BeNull();
            node.RightChild.Should().BeNull();
        }

        [TestMethod]
        public void TestGetIsLeaf()
        {
            node.IsLeaf.Should().BeTrue();

            MerkleTreeNode child = new MerkleTreeNode();
            node.LeftChild = child;
            node.IsLeaf.Should().BeFalse();
        }

        [TestMethod]
        public void TestGetIsRoot()
        {
            node.IsRoot.Should().BeTrue();
        }
    }
}

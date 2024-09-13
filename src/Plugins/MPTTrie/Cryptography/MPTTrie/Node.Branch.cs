// Copyright (C) 2021-2024 EpicChain Labs.

//
// Node.Branch.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO;
using System.IO;

namespace EpicChain.Cryptography.MPTTrie
{
    partial class Node
    {
        public const int BranchChildCount = 17;
        public Node[] Children;

        public static Node NewBranch()
        {
            var n = new Node
            {
                type = NodeType.BranchNode,
                Reference = 1,
                Children = new Node[BranchChildCount],
            };
            for (int i = 0; i < BranchChildCount; i++)
            {
                n.Children[i] = new Node();
            }
            return n;
        }

        protected int BranchSize
        {
            get
            {
                int size = 0;
                for (int i = 0; i < BranchChildCount; i++)
                {
                    size += Children[i].SizeAsChild;
                }
                return size;
            }
        }

        private void SerializeBranch(BinaryWriter writer)
        {
            for (int i = 0; i < BranchChildCount; i++)
            {
                Children[i].SerializeAsChild(writer);
            }
        }

        private void DeserializeBranch(ref MemoryReader reader)
        {
            Children = new Node[BranchChildCount];
            for (int i = 0; i < BranchChildCount; i++)
            {
                var n = new Node();
                n.Deserialize(ref reader);
                Children[i] = n;
            }
        }
    }
}

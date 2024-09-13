// Copyright (C) 2021-2024 EpicChain Labs.

//
// Trie.Proof.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Persistence;
using System;
using System.Collections.Generic;

namespace EpicChain.Cryptography.MPTTrie
{
    partial class Trie
    {
        public bool TryGetProof(byte[] key, out HashSet<byte[]> proof)
        {
            var path = ToNibbles(key);
            if (path.Length == 0)
                throw new ArgumentException("could not be empty", nameof(key));
            if (path.Length > Node.MaxKeyLength)
                throw new ArgumentException("exceeds limit", nameof(key));
            proof = new HashSet<byte[]>(ByteArrayEqualityComparer.Default);
            return GetProof(ref root, path, proof);
        }

        private bool GetProof(ref Node node, ReadOnlySpan<byte> path, HashSet<byte[]> set)
        {
            switch (node.Type)
            {
                case NodeType.LeafNode:
                    {
                        if (path.IsEmpty)
                        {
                            set.Add(node.ToArrayWithoutReference());
                            return true;
                        }
                        break;
                    }
                case NodeType.Empty:
                    break;
                case NodeType.HashNode:
                    {
                        var newNode = cache.Resolve(node.Hash);
                        if (newNode is null) throw new InvalidOperationException("Internal error, can't resolve hash when mpt getproof");
                        node = newNode;
                        return GetProof(ref node, path, set);
                    }
                case NodeType.BranchNode:
                    {
                        set.Add(node.ToArrayWithoutReference());
                        if (path.IsEmpty)
                        {
                            return GetProof(ref node.Children[Node.BranchChildCount - 1], path, set);
                        }
                        return GetProof(ref node.Children[path[0]], path[1..], set);
                    }
                case NodeType.ExtensionNode:
                    {
                        if (path.StartsWith(node.Key.Span))
                        {
                            set.Add(node.ToArrayWithoutReference());
                            return GetProof(ref node.Next, path[node.Key.Length..], set);
                        }
                        break;
                    }
            }
            return false;
        }

        private static byte[] Key(byte[] hash)
        {
            byte[] buffer = new byte[hash.Length + 1];
            buffer[0] = Prefix;
            Buffer.BlockCopy(hash, 0, buffer, 1, hash.Length);
            return buffer;
        }

        public static byte[] VerifyProof(UInt256 root, byte[] key, HashSet<byte[]> proof)
        {
            using var memoryStore = new MemoryStore();
            foreach (byte[] data in proof)
                memoryStore.Put(Key(Crypto.Hash256(data)), [.. data, .. new byte[] { 1 }]);
            using ISnapshot snapshot = memoryStore.GetSnapshot();
            var trie = new Trie(snapshot, root, false);
            return trie[key];
        }
    }
}

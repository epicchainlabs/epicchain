// Copyright (C) 2021-2024 EpicChain Labs.

//
// Trie.Delete.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.Collections.Generic;

namespace EpicChain.Cryptography.MPTTrie
{
    partial class Trie
    {
        public bool Delete(byte[] key)
        {
            var path = ToNibbles(key);
            if (path.Length == 0)
                throw new ArgumentException("could not be empty", nameof(key));
            if (path.Length > Node.MaxKeyLength)
                throw new ArgumentException("exceeds limit", nameof(key));
            return TryDelete(ref root, path);
        }

        private bool TryDelete(ref Node node, ReadOnlySpan<byte> path)
        {
            switch (node.Type)
            {
                case NodeType.LeafNode:
                    {
                        if (path.IsEmpty)
                        {
                            if (!full) cache.DeleteNode(node.Hash);
                            node = new Node();
                            return true;
                        }
                        return false;
                    }
                case NodeType.ExtensionNode:
                    {
                        if (path.StartsWith(node.Key.Span))
                        {
                            var oldHash = node.Hash;
                            var result = TryDelete(ref node.Next, path[node.Key.Length..]);
                            if (!result) return false;
                            if (!full) cache.DeleteNode(oldHash);
                            if (node.Next.IsEmpty)
                            {
                                node = node.Next;
                                return true;
                            }
                            if (node.Next.Type == NodeType.ExtensionNode)
                            {
                                if (!full) cache.DeleteNode(node.Next.Hash);
                                node.Key = new([.. node.Key.Span, .. node.Next.Key.Span]);
                                node.Next = node.Next.Next;
                            }
                            node.SetDirty();
                            cache.PutNode(node);
                            return true;
                        }
                        return false;
                    }
                case NodeType.BranchNode:
                    {
                        bool result;
                        var oldHash = node.Hash;
                        if (path.IsEmpty)
                        {
                            result = TryDelete(ref node.Children[Node.BranchChildCount - 1], path);
                        }
                        else
                        {
                            result = TryDelete(ref node.Children[path[0]], path[1..]);
                        }
                        if (!result) return false;
                        if (!full) cache.DeleteNode(oldHash);
                        List<byte> childrenIndexes = new List<byte>(Node.BranchChildCount);
                        for (int i = 0; i < Node.BranchChildCount; i++)
                        {
                            if (node.Children[i].IsEmpty) continue;
                            childrenIndexes.Add((byte)i);
                        }
                        if (childrenIndexes.Count > 1)
                        {
                            node.SetDirty();
                            cache.PutNode(node);
                            return true;
                        }
                        var lastChildIndex = childrenIndexes[0];
                        var lastChild = node.Children[lastChildIndex];
                        if (lastChildIndex == Node.BranchChildCount - 1)
                        {
                            node = lastChild;
                            return true;
                        }
                        if (lastChild.Type == NodeType.HashNode)
                        {
                            lastChild = cache.Resolve(lastChild.Hash);
                            if (lastChild is null) throw new InvalidOperationException("Internal error, can't resolve hash");
                        }
                        if (lastChild.Type == NodeType.ExtensionNode)
                        {
                            if (!full) cache.DeleteNode(lastChild.Hash);
                            lastChild.Key = new([.. childrenIndexes.ToArray(), .. lastChild.Key.Span]);
                            lastChild.SetDirty();
                            cache.PutNode(lastChild);
                            node = lastChild;
                            return true;
                        }
                        node = Node.NewExtension(childrenIndexes.ToArray(), lastChild);
                        cache.PutNode(node);
                        return true;
                    }
                case NodeType.Empty:
                    {
                        return false;
                    }
                case NodeType.HashNode:
                    {
                        var newNode = cache.Resolve(node.Hash);
                        if (newNode is null) throw new InvalidOperationException("Internal error, can't resolve hash when mpt delete");
                        node = newNode;
                        return TryDelete(ref node, path);
                    }
                default:
                    return false;
            }
        }
    }
}

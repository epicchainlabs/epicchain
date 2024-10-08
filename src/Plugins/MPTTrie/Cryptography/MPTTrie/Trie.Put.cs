// Copyright (C) 2021-2024 EpicChain Labs.

//
// Trie.Put.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Cryptography.MPTTrie
{
    partial class Trie
    {
        private static ReadOnlySpan<byte> CommonPrefix(ReadOnlySpan<byte> a, ReadOnlySpan<byte> b)
        {
            var minLen = a.Length <= b.Length ? a.Length : b.Length;
            int i = 0;
            if (a.Length != 0 && b.Length != 0)
            {
                for (i = 0; i < minLen; i++)
                {
                    if (a[i] != b[i]) break;
                }
            }
            return a[..i];
        }

        public void Put(byte[] key, byte[] value)
        {
            var path = ToNibbles(key);
            var val = value;
            if (path.Length == 0 || path.Length > Node.MaxKeyLength)
                throw new ArgumentException("invalid", nameof(key));
            if (val.Length > Node.MaxValueLength)
                throw new ArgumentException("exceed limit", nameof(value));
            var n = Node.NewLeaf(val);
            Put(ref root, path, n);
        }

        private void Put(ref Node node, ReadOnlySpan<byte> path, Node val)
        {
            switch (node.Type)
            {
                case NodeType.LeafNode:
                    {
                        if (path.IsEmpty)
                        {
                            if (!full) cache.DeleteNode(node.Hash);
                            node = val;
                            cache.PutNode(node);
                            return;
                        }
                        var branch = Node.NewBranch();
                        branch.Children[Node.BranchChildCount - 1] = node;
                        Put(ref branch.Children[path[0]], path[1..], val);
                        cache.PutNode(branch);
                        node = branch;
                        break;
                    }
                case NodeType.ExtensionNode:
                    {
                        if (path.StartsWith(node.Key.Span))
                        {
                            var oldHash = node.Hash;
                            Put(ref node.Next, path[node.Key.Length..], val);
                            if (!full) cache.DeleteNode(oldHash);
                            node.SetDirty();
                            cache.PutNode(node);
                            return;
                        }
                        if (!full) cache.DeleteNode(node.Hash);
                        var prefix = CommonPrefix(node.Key.Span, path);
                        var pathRemain = path[prefix.Length..];
                        var keyRemain = node.Key.Span[prefix.Length..];
                        var child = Node.NewBranch();
                        Node grandChild = new Node();
                        if (keyRemain.Length == 1)
                        {
                            child.Children[keyRemain[0]] = node.Next;
                        }
                        else
                        {
                            var exNode = Node.NewExtension(keyRemain[1..].ToArray(), node.Next);
                            cache.PutNode(exNode);
                            child.Children[keyRemain[0]] = exNode;
                        }
                        if (pathRemain.IsEmpty)
                        {
                            Put(ref grandChild, pathRemain, val);
                            child.Children[Node.BranchChildCount - 1] = grandChild;
                        }
                        else
                        {
                            Put(ref grandChild, pathRemain[1..], val);
                            child.Children[pathRemain[0]] = grandChild;
                        }
                        cache.PutNode(child);
                        if (prefix.Length > 0)
                        {
                            var exNode = Node.NewExtension(prefix.ToArray(), child);
                            cache.PutNode(exNode);
                            node = exNode;
                        }
                        else
                        {
                            node = child;
                        }
                        break;
                    }
                case NodeType.BranchNode:
                    {
                        var oldHash = node.Hash;
                        if (path.IsEmpty)
                        {
                            Put(ref node.Children[Node.BranchChildCount - 1], path, val);
                        }
                        else
                        {
                            Put(ref node.Children[path[0]], path[1..], val);
                        }
                        if (!full) cache.DeleteNode(oldHash);
                        node.SetDirty();
                        cache.PutNode(node);
                        break;
                    }
                case NodeType.Empty:
                    {
                        Node newNode;
                        if (path.IsEmpty)
                        {
                            newNode = val;
                        }
                        else
                        {
                            newNode = Node.NewExtension(path.ToArray(), val);
                            cache.PutNode(newNode);
                        }
                        node = newNode;
                        if (val.Type == NodeType.LeafNode) cache.PutNode(val);
                        break;
                    }
                case NodeType.HashNode:
                    {
                        Node newNode = cache.Resolve(node.Hash);
                        if (newNode is null) throw new InvalidOperationException("Internal error, can't resolve hash when mpt put");
                        node = newNode;
                        Put(ref node, path, val);
                        break;
                    }
                default:
                    throw new InvalidOperationException("unsupport node type");
            }
        }
    }
}

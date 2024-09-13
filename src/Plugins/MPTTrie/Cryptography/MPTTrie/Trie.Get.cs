// Copyright (C) 2021-2024 EpicChain Labs.

//
// Trie.Get.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
        public byte[] this[byte[] key]
        {
            get
            {
                var path = ToNibbles(key);
                if (path.Length == 0)
                    throw new ArgumentException("could not be empty", nameof(key));
                if (path.Length > Node.MaxKeyLength)
                    throw new ArgumentException("exceeds limit", nameof(key));
                var result = TryGet(ref root, path, out var value);
                return result ? value.ToArray() : throw new KeyNotFoundException();
            }
        }

        public bool TryGetValue(byte[] key, out byte[] value)
        {
            value = default;
            var path = ToNibbles(key);
            if (path.Length == 0)
                throw new ArgumentException("could not be empty", nameof(key));
            if (path.Length > Node.MaxKeyLength)
                throw new ArgumentException("exceeds limit", nameof(key));
            var result = TryGet(ref root, path, out var val);
            if (result)
                value = val.ToArray();
            return result;
        }

        private bool TryGet(ref Node node, ReadOnlySpan<byte> path, out ReadOnlySpan<byte> value)
        {
            switch (node.Type)
            {
                case NodeType.LeafNode:
                    {
                        if (path.IsEmpty)
                        {
                            value = node.Value.Span;
                            return true;
                        }
                        break;
                    }
                case NodeType.Empty:
                    break;
                case NodeType.HashNode:
                    {
                        var newNode = cache.Resolve(node.Hash);
                        if (newNode is null) throw new InvalidOperationException("Internal error, can't resolve hash when mpt get");
                        node = newNode;
                        return TryGet(ref node, path, out value);
                    }
                case NodeType.BranchNode:
                    {
                        if (path.IsEmpty)
                        {
                            return TryGet(ref node.Children[Node.BranchChildCount - 1], path, out value);
                        }
                        return TryGet(ref node.Children[path[0]], path[1..], out value);
                    }
                case NodeType.ExtensionNode:
                    {
                        if (path.StartsWith(node.Key.Span))
                        {
                            return TryGet(ref node.Next, path[node.Key.Length..], out value);
                        }
                        break;
                    }
            }
            value = default;
            return false;
        }
    }
}

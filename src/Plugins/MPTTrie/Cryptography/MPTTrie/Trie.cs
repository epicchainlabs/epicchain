// Copyright (C) 2021-2024 EpicChain Labs.

//
// Trie.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Persistence;
using System;

namespace EpicChain.Cryptography.MPTTrie
{
    public partial class Trie
    {
        private const byte Prefix = 0xf0;
        private readonly bool full;
        private readonly ISnapshot store;
        private Node root;
        private readonly Cache cache;
        public Node Root => root;

        public Trie(ISnapshot store, UInt256 root, bool full_state = false)
        {
            this.store = store ?? throw new ArgumentNullException(nameof(store));
            cache = new Cache(store, Prefix);
            this.root = root is null ? new Node() : Node.NewHash(root);
            full = full_state;
        }

        private static byte[] ToNibbles(ReadOnlySpan<byte> path)
        {
            var result = new byte[path.Length * 2];
            for (int i = 0; i < path.Length; i++)
            {
                result[i * 2] = (byte)(path[i] >> 4);
                result[i * 2 + 1] = (byte)(path[i] & 0x0F);
            }
            return result;
        }

        private static byte[] FromNibbles(ReadOnlySpan<byte> path)
        {
            if (path.Length % 2 != 0) throw new FormatException($"MPTTrie.FromNibbles invalid path.");
            var key = new byte[path.Length / 2];
            for (int i = 0; i < key.Length; i++)
            {
                key[i] = (byte)(path[i * 2] << 4);
                key[i] |= path[i * 2 + 1];
            }
            return key;
        }

        public void Commit()
        {
            cache.Commit();
        }
    }
}

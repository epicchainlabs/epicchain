// Copyright (C) 2021-2024 EpicChain Labs.

//
// Cache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.IO;

namespace EpicChain.Cryptography.MPTTrie
{
    public class Cache
    {
        private enum TrackState : byte
        {
            None,
            Added,
            Changed,
            Deleted
        }

        private class Trackable
        {
            public Node Node;
            public TrackState State;
        }

        private readonly ISnapshot store;
        private readonly byte prefix;
        private readonly Dictionary<UInt256, Trackable> cache = new Dictionary<UInt256, Trackable>();

        public Cache(ISnapshot store, byte prefix)
        {
            this.store = store;
            this.prefix = prefix;
        }

        private byte[] Key(UInt256 hash)
        {
            byte[] buffer = new byte[UInt256.Length + 1];
            using (MemoryStream ms = new MemoryStream(buffer, true))
            using (BinaryWriter writer = new BinaryWriter(ms))
            {
                writer.Write(prefix);
                hash.Serialize(writer);
            }
            return buffer;
        }

        public Node Resolve(UInt256 hash)
        {
            if (cache.TryGetValue(hash, out Trackable t))
            {
                return t.Node?.Clone();
            }
            var n = store.TryGet(Key(hash))?.AsSerializable<Node>();
            cache.Add(hash, new Trackable
            {
                Node = n,
                State = TrackState.None,
            });
            return n?.Clone();
        }

        public void PutNode(Node np)
        {
            var n = Resolve(np.Hash);
            if (n is null)
            {
                np.Reference = 1;
                cache[np.Hash] = new Trackable
                {
                    Node = np.Clone(),
                    State = TrackState.Added,
                };
                return;
            }
            var entry = cache[np.Hash];
            entry.Node.Reference++;
            entry.State = TrackState.Changed;
        }

        public void DeleteNode(UInt256 hash)
        {
            var n = Resolve(hash);
            if (n is null) return;
            if (1 < n.Reference)
            {
                var entry = cache[hash];
                entry.Node.Reference--;
                entry.State = TrackState.Changed;
                return;
            }
            cache[hash] = new Trackable
            {
                Node = null,
                State = TrackState.Deleted,
            };
        }

        public void Commit()
        {
            foreach (var item in cache)
            {
                switch (item.Value.State)
                {
                    case TrackState.Added:
                    case TrackState.Changed:
                        store.Put(Key(item.Key), item.Value.Node.ToArray());
                        break;
                    case TrackState.Deleted:
                        store.Delete(Key(item.Key));
                        break;
                }
            }
            cache.Clear();
        }
    }
}

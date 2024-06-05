// Copyright (C) 2021-2024 The EpicChain Labs.
//
// MemorySnapshot.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.IO;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace Neo.Persistence
{
    internal class MemorySnapshot : ISnapshot
    {
        private readonly ConcurrentDictionary<byte[], byte[]> innerData;
        private readonly ImmutableDictionary<byte[], byte[]> immutableData;
        private readonly ConcurrentDictionary<byte[], byte[]> writeBatch;

        public MemorySnapshot(ConcurrentDictionary<byte[], byte[]> innerData)
        {
            this.innerData = innerData;
            immutableData = innerData.ToImmutableDictionary(ByteArrayEqualityComparer.Default);
            writeBatch = new ConcurrentDictionary<byte[], byte[]>(ByteArrayEqualityComparer.Default);
        }

        public void Commit()
        {
            foreach (var pair in writeBatch)
                if (pair.Value is null)
                    innerData.TryRemove(pair.Key, out _);
                else
                    innerData[pair.Key] = pair.Value;
        }

        public void Delete(byte[] key)
        {
            writeBatch[key] = null;
        }

        public void Dispose()
        {
        }

        public void Put(byte[] key, byte[] value)
        {
            writeBatch[key[..]] = value[..];
        }

        public IEnumerable<(byte[] Key, byte[] Value)> Seek(byte[] keyOrPrefix, SeekDirection direction = SeekDirection.Forward)
        {
            ByteArrayComparer comparer = direction == SeekDirection.Forward ? ByteArrayComparer.Default : ByteArrayComparer.Reverse;
            IEnumerable<KeyValuePair<byte[], byte[]>> records = immutableData;
            if (keyOrPrefix?.Length > 0)
                records = records.Where(p => comparer.Compare(p.Key, keyOrPrefix) >= 0);
            records = records.OrderBy(p => p.Key, comparer);
            return records.Select(p => (p.Key[..], p.Value[..]));
        }

        public byte[] TryGet(byte[] key)
        {
            immutableData.TryGetValue(key, out byte[] value);
            return value?[..];
        }

        public bool Contains(byte[] key)
        {
            return immutableData.ContainsKey(key);
        }
    }
}

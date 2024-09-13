// Copyright (C) 2021-2024 EpicChain Labs.

//
// MemoryStore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;

namespace EpicChain.Persistence
{
    /// <summary>
    /// An in-memory <see cref="IStore"/> implementation that uses ConcurrentDictionary as the underlying storage.
    /// </summary>
    public class MemoryStore : IStore
    {
        private readonly ConcurrentDictionary<byte[], byte[]> _innerData = new(ByteArrayEqualityComparer.Default);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Delete(byte[] key)
        {
            _innerData.TryRemove(key, out _);
        }

        public void Dispose()
        {
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ISnapshot GetSnapshot()
        {
            return new MemorySnapshot(_innerData);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Put(byte[] key, byte[] value)
        {
            _innerData[key[..]] = value[..];
        }

        public IEnumerable<(byte[] Key, byte[] Value)> Seek(byte[] keyOrPrefix, SeekDirection direction = SeekDirection.Forward)
        {
            if (direction == SeekDirection.Backward && keyOrPrefix?.Length == 0) yield break;

            var comparer = direction == SeekDirection.Forward ? ByteArrayComparer.Default : ByteArrayComparer.Reverse;
            IEnumerable<KeyValuePair<byte[], byte[]>> records = _innerData;
            if (keyOrPrefix?.Length > 0)
                records = records.Where(p => comparer.Compare(p.Key, keyOrPrefix) >= 0);
            records = records.OrderBy(p => p.Key, comparer);
            foreach (var pair in records)
                yield return (pair.Key[..], pair.Value[..]);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public byte[] TryGet(byte[] key)
        {
            if (!_innerData.TryGetValue(key, out byte[] value)) return null;
            return value[..];
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool Contains(byte[] key)
        {
            return _innerData.ContainsKey(key);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal void Reset()
        {
            _innerData.Clear();
        }
    }
}

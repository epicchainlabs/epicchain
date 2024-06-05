// Copyright (C) 2021-2024 The EpicChain Labs.
//
// StorageKey.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Cryptography;
using System;
using System.Buffers.Binary;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents the keys in contract storage.
    /// </summary>
    public sealed record StorageKey
    {
        /// <summary>
        /// The id of the contract.
        /// </summary>
        public int Id { get; init; }

        /// <summary>
        /// The key of the storage entry.
        /// </summary>
        public ReadOnlyMemory<byte> Key { get; init; }

        private byte[] cache = null;

        public StorageKey() { }

        internal StorageKey(byte[] cache)
        {
            this.cache = cache;
            Id = BinaryPrimitives.ReadInt32LittleEndian(cache);
            Key = cache.AsMemory(sizeof(int));
        }

        /// <summary>
        /// Creates a search prefix for a contract.
        /// </summary>
        /// <param name="id">The id of the contract.</param>
        /// <param name="prefix">The prefix of the keys to search.</param>
        /// <returns>The created search prefix.</returns>
        public static byte[] CreateSearchPrefix(int id, ReadOnlySpan<byte> prefix)
        {
            byte[] buffer = new byte[sizeof(int) + prefix.Length];
            BinaryPrimitives.WriteInt32LittleEndian(buffer, id);
            prefix.CopyTo(buffer.AsSpan(sizeof(int)));
            return buffer;
        }

        public bool Equals(StorageKey other)
        {
            if (other is null)
                return false;
            if (ReferenceEquals(this, other))
                return true;
            return Id == other.Id && Key.Span.SequenceEqual(other.Key.Span);
        }

        public override int GetHashCode()
        {
            return Id + (int)Key.Span.Murmur32(0);
        }

        public byte[] ToArray()
        {
            if (cache is null)
            {
                cache = new byte[sizeof(int) + Key.Length];
                BinaryPrimitives.WriteInt32LittleEndian(cache, Id);
                Key.CopyTo(cache.AsMemory(sizeof(int)));
            }
            return cache;
        }
    }
}

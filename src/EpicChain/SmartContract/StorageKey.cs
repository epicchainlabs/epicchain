// Copyright (C) 2021-2024 EpicChain Labs.

//
// StorageKey.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography;
using System;
using System.Buffers.Binary;
using System.Runtime.CompilerServices;

namespace EpicChain.SmartContract
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

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StorageKey(byte[] value) => new StorageKey(value);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StorageKey(ReadOnlyMemory<byte> value) => new StorageKey(value.Span.ToArray());

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StorageKey(ReadOnlySpan<byte> value) => new StorageKey(value.ToArray());
    }
}

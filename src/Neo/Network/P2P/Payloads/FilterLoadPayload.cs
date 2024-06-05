// Copyright (C) 2021-2024 The EpicChain Labs.
//
// FilterLoadPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.IO;
using System;
using System.IO;

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// This message is sent to load the <see cref="BloomFilter"/>.
    /// </summary>
    public class FilterLoadPayload : ISerializable
    {
        /// <summary>
        /// The data of the <see cref="BloomFilter"/>.
        /// </summary>
        public ReadOnlyMemory<byte> Filter;

        /// <summary>
        /// The number of hash functions used by the <see cref="BloomFilter"/>.
        /// </summary>
        public byte K;

        /// <summary>
        /// Used to generate the seeds of the murmur hash functions.
        /// </summary>
        public uint Tweak;

        public int Size => Filter.GetVarSize() + sizeof(byte) + sizeof(uint);

        /// <summary>
        /// Creates a new instance of the <see cref="FilterLoadPayload"/> class.
        /// </summary>
        /// <param name="filter">The fields in the filter will be copied to the payload.</param>
        /// <returns>The created payload.</returns>
        public static FilterLoadPayload Create(BloomFilter filter)
        {
            byte[] buffer = new byte[filter.M / 8];
            filter.GetBits(buffer);
            return new FilterLoadPayload
            {
                Filter = buffer,
                K = (byte)filter.K,
                Tweak = filter.Tweak
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Filter = reader.ReadVarMemory(36000);
            K = reader.ReadByte();
            if (K > 50) throw new FormatException();
            Tweak = reader.ReadUInt32();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.WriteVarBytes(Filter.Span);
            writer.Write(K);
            writer.Write(Tweak);
        }
    }
}

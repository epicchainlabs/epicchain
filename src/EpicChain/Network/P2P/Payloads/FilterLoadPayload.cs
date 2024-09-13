// Copyright (C) 2021-2024 EpicChain Labs.

//
// FilterLoadPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO;
using System;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
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

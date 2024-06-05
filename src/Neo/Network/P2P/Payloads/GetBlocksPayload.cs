// Copyright (C) 2021-2024 The EpicChain Labs.
//
// GetBlocksPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System;
using System.IO;

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// This message is sent to request for blocks by hash.
    /// </summary>
    public class GetBlocksPayload : ISerializable
    {
        /// <summary>
        /// The starting hash of the blocks to request.
        /// </summary>
        public UInt256 HashStart;

        /// <summary>
        /// The number of blocks to request.
        /// </summary>
        public short Count;

        public int Size => sizeof(short) + HashStart.Size;

        /// <summary>
        /// Creates a new instance of the <see cref="GetBlocksPayload"/> class.
        /// </summary>
        /// <param name="hash_start">The starting hash of the blocks to request.</param>
        /// <param name="count">The number of blocks to request. Set this parameter to -1 to request as many blocks as possible.</param>
        /// <returns>The created payload.</returns>
        public static GetBlocksPayload Create(UInt256 hash_start, short count = -1)
        {
            return new GetBlocksPayload
            {
                HashStart = hash_start,
                Count = count
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            HashStart = reader.ReadSerializable<UInt256>();
            Count = reader.ReadInt16();
            if (Count < -1 || Count == 0) throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(HashStart);
            writer.Write(Count);
        }
    }
}

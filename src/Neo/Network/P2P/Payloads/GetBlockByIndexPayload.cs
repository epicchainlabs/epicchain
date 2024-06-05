// Copyright (C) 2021-2024 The EpicChain Labs.
//
// GetBlockByIndexPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
    /// This message is sent to request for blocks by index.
    /// </summary>
    public class GetBlockByIndexPayload : ISerializable
    {
        /// <summary>
        /// The starting index of the blocks to request.
        /// </summary>
        public uint IndexStart;

        /// <summary>
        /// The number of blocks to request.
        /// </summary>
        public short Count;

        public int Size => sizeof(uint) + sizeof(short);

        /// <summary>
        /// Creates a new instance of the <see cref="GetBlockByIndexPayload"/> class.
        /// </summary>
        /// <param name="index_start">The starting index of the blocks to request.</param>
        /// <param name="count">The number of blocks to request. Set this parameter to -1 to request as many blocks as possible.</param>
        /// <returns>The created payload.</returns>
        public static GetBlockByIndexPayload Create(uint index_start, short count = -1)
        {
            return new GetBlockByIndexPayload
            {
                IndexStart = index_start,
                Count = count
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            IndexStart = reader.ReadUInt32();
            Count = reader.ReadInt16();
            if (Count < -1 || Count == 0 || Count > HeadersPayload.MaxHeadersCount)
                throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(IndexStart);
            writer.Write(Count);
        }
    }
}

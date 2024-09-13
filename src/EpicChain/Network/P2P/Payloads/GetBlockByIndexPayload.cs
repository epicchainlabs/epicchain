// Copyright (C) 2021-2024 EpicChain Labs.

//
// GetBlockByIndexPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
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

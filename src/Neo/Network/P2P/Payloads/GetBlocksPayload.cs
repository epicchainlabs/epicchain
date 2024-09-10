// Copyright (C) 2021-2024 EpicChain Labs.

//
// GetBlocksPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

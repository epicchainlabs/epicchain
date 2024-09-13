// Copyright (C) 2021-2024 EpicChain Labs.

//
// MerkleBlockPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections;
using System.IO;
using System.Linq;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents a block that is filtered by a <see cref="BloomFilter"/>.
    /// </summary>
    public class MerkleBlockPayload : ISerializable
    {
        /// <summary>
        /// The header of the block.
        /// </summary>
        public Header Header;

        /// <summary>
        /// The number of the transactions in the block.
        /// </summary>
        public int TxCount;

        /// <summary>
        /// The nodes of the transactions hash tree.
        /// </summary>
        public UInt256[] Hashes;

        /// <summary>
        /// The data in the <see cref="BloomFilter"/> that filtered the block.
        /// </summary>
        public ReadOnlyMemory<byte> Flags;

        public int Size => Header.Size + sizeof(int) + Hashes.GetVarSize() + Flags.GetVarSize();

        /// <summary>
        /// Creates a new instance of the <see cref="MerkleBlockPayload"/> class.
        /// </summary>
        /// <param name="block">The original block.</param>
        /// <param name="flags">The data in the <see cref="BloomFilter"/> that filtered the block.</param>
        /// <returns>The created payload.</returns>
        public static MerkleBlockPayload Create(Block block, BitArray flags)
        {
            MerkleTree tree = new(block.Transactions.Select(p => p.Hash).ToArray());
            tree.Trim(flags);
            byte[] buffer = new byte[(flags.Length + 7) / 8];
            flags.CopyTo(buffer, 0);
            return new MerkleBlockPayload
            {
                Header = block.Header,
                TxCount = block.Transactions.Length,
                Hashes = tree.ToHashArray(),
                Flags = buffer
            };
        }

        public void Deserialize(ref MemoryReader reader)
        {
            Header = reader.ReadSerializable<Header>();
            TxCount = (int)reader.ReadVarInt(ushort.MaxValue);
            Hashes = reader.ReadSerializableArray<UInt256>(TxCount);
            Flags = reader.ReadVarMemory((Math.Max(TxCount, 1) + 7) / 8);
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(Header);
            writer.WriteVarInt(TxCount);
            writer.Write(Hashes);
            writer.WriteVarBytes(Flags.Span);
        }
    }
}

// Copyright (C) 2021-2024 The EpicChain Labs.
//
// MerkleBlockPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System.Collections;
using System.IO;
using System.Linq;

namespace Neo.Network.P2P.Payloads
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

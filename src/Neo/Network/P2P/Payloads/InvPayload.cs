// Copyright (C) 2021-2024 The EpicChain Labs.
//
// InvPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System.Collections.Generic;
using System.IO;

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// This message is sent to relay inventories.
    /// </summary>
    public class InvPayload : ISerializable
    {
        /// <summary>
        /// Indicates the maximum number of inventories sent each time.
        /// </summary>
        public const int MaxHashesCount = 500;

        /// <summary>
        /// The type of the inventories.
        /// </summary>
        public InventoryType Type;

        /// <summary>
        /// The hashes of the inventories.
        /// </summary>
        public UInt256[] Hashes;

        public int Size => sizeof(InventoryType) + Hashes.GetVarSize();

        /// <summary>
        /// Creates a new instance of the <see cref="InvPayload"/> class.
        /// </summary>
        /// <param name="type">The type of the inventories.</param>
        /// <param name="hashes">The hashes of the inventories.</param>
        /// <returns>The created payload.</returns>
        public static InvPayload Create(InventoryType type, params UInt256[] hashes)
        {
            return new InvPayload
            {
                Type = type,
                Hashes = hashes
            };
        }

        /// <summary>
        /// Creates a group of the <see cref="InvPayload"/> instance.
        /// </summary>
        /// <param name="type">The type of the inventories.</param>
        /// <param name="hashes">The hashes of the inventories.</param>
        /// <returns>The created payloads.</returns>
        public static IEnumerable<InvPayload> CreateGroup(InventoryType type, UInt256[] hashes)
        {
            for (int i = 0; i < hashes.Length; i += MaxHashesCount)
            {
                int endIndex = i + MaxHashesCount;
                if (endIndex > hashes.Length) endIndex = hashes.Length;
                yield return new InvPayload
                {
                    Type = type,
                    Hashes = hashes[i..endIndex]
                };
            }
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Type = (InventoryType)reader.ReadByte();
            if (!Enum.IsDefined(typeof(InventoryType), Type))
                throw new FormatException();
            Hashes = reader.ReadSerializableArray<UInt256>(MaxHashesCount);
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write((byte)Type);
            writer.Write(Hashes);
        }
    }
}

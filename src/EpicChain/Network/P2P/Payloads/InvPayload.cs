// Copyright (C) 2021-2024 EpicChain Labs.

//
// InvPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
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

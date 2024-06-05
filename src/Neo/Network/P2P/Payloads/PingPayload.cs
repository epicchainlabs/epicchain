// Copyright (C) 2021-2024 The EpicChain Labs.
//
// PingPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
    /// Sent to detect whether the connection has been disconnected.
    /// </summary>
    public class PingPayload : ISerializable
    {
        /// <summary>
        /// The latest block index.
        /// </summary>
        public uint LastBlockIndex;

        /// <summary>
        /// The timestamp when the message was sent.
        /// </summary>
        public uint Timestamp;

        /// <summary>
        /// A random number. This number must be the same in
        /// <see cref="MessageCommand.Ping"/> and <see cref="MessageCommand.Pong"/> messages.
        /// </summary>
        public uint Nonce;

        public int Size =>
            sizeof(uint) +  //LastBlockIndex
            sizeof(uint) +  //Timestamp
            sizeof(uint);   //Nonce

        /// <summary>
        /// Creates a new instance of the <see cref="PingPayload"/> class.
        /// </summary>
        /// <param name="height">The latest block index.</param>
        /// <returns>The created payload.</returns>
        public static PingPayload Create(uint height)
        {
            Random rand = new();
            return Create(height, (uint)rand.Next());
        }

        /// <summary>
        /// Creates a new instance of the <see cref="PingPayload"/> class.
        /// </summary>
        /// <param name="height">The latest block index.</param>
        /// <param name="nonce">The random number.</param>
        /// <returns>The created payload.</returns>
        public static PingPayload Create(uint height, uint nonce)
        {
            return new PingPayload
            {
                LastBlockIndex = height,
                Timestamp = TimeProvider.Current.UtcNow.ToTimestamp(),
                Nonce = nonce
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            LastBlockIndex = reader.ReadUInt32();
            Timestamp = reader.ReadUInt32();
            Nonce = reader.ReadUInt32();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(LastBlockIndex);
            writer.Write(Timestamp);
            writer.Write(Nonce);
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// PingPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Extensions;
using EpicChain.IO;
using System;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
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

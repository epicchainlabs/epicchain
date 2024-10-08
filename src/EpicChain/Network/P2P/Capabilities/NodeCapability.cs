// Copyright (C) 2021-2024 EpicChain Labs.

//
// NodeCapability.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Network.P2P.Capabilities
{
    /// <summary>
    /// Represents the capabilities of a EpicChain node.
    /// </summary>
    public abstract class NodeCapability : ISerializable
    {
        /// <summary>
        /// Indicates the type of the <see cref="NodeCapability"/>.
        /// </summary>
        public readonly NodeCapabilityType Type;

        public virtual int Size => sizeof(NodeCapabilityType); // Type

        /// <summary>
        /// Initializes a new instance of the <see cref="NodeCapability"/> class.
        /// </summary>
        /// <param name="type">The type of the <see cref="NodeCapability"/>.</param>
        protected NodeCapability(NodeCapabilityType type)
        {
            Type = type;
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            if (reader.ReadByte() != (byte)Type)
            {
                throw new FormatException();
            }

            DeserializeWithoutType(ref reader);
        }

        /// <summary>
        /// Deserializes an <see cref="NodeCapability"/> object from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <returns>The deserialized <see cref="NodeCapability"/>.</returns>
        public static NodeCapability DeserializeFrom(ref MemoryReader reader)
        {
            NodeCapabilityType type = (NodeCapabilityType)reader.ReadByte();
            NodeCapability capability = type switch
            {
#pragma warning disable CS0612 // Type or member is obsolete
                NodeCapabilityType.TcpServer or NodeCapabilityType.WsServer => new ServerCapability(type),
#pragma warning restore CS0612 // Type or member is obsolete
                NodeCapabilityType.FullNode => new FullNodeCapability(),
                _ => throw new FormatException(),
            };
            capability.DeserializeWithoutType(ref reader);
            return capability;
        }

        /// <summary>
        /// Deserializes the <see cref="NodeCapability"/> object from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        protected abstract void DeserializeWithoutType(ref MemoryReader reader);

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write((byte)Type);
            SerializeWithoutType(writer);
        }

        /// <summary>
        /// Serializes the <see cref="NodeCapability"/> object to a <see cref="BinaryWriter"/>.
        /// </summary>
        /// <param name="writer">The <see cref="BinaryWriter"/> for writing data.</param>
        protected abstract void SerializeWithoutType(BinaryWriter writer);
    }
}

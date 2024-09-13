// Copyright (C) 2021-2024 EpicChain Labs.

//
// ServerCapability.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    /// Indicates that the node is a server.
    /// </summary>
    public class ServerCapability : NodeCapability
    {
        /// <summary>
        /// Indicates the port that the node is listening on.
        /// </summary>
        public ushort Port;

        public override int Size =>
            base.Size +     // Type
            sizeof(ushort); // Port

        /// <summary>
        /// Initializes a new instance of the <see cref="ServerCapability"/> class.
        /// </summary>
        /// <param name="type">The type of the <see cref="ServerCapability"/>. It must be <see cref="NodeCapabilityType.TcpServer"/> or <see cref="NodeCapabilityType.WsServer"/></param>
        /// <param name="port">The port that the node is listening on.</param>
        public ServerCapability(NodeCapabilityType type, ushort port = 0) : base(type)
        {
#pragma warning disable CS0612 // Type or member is obsolete
            if (type != NodeCapabilityType.TcpServer && type != NodeCapabilityType.WsServer)
#pragma warning restore CS0612 // Type or member is obsolete
            {
                throw new ArgumentException(nameof(type));
            }

            Port = port;
        }

        protected override void DeserializeWithoutType(ref MemoryReader reader)
        {
            Port = reader.ReadUInt16();
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Port);
        }
    }
}

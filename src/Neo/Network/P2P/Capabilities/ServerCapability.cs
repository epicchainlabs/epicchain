// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ServerCapability.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.Network.P2P.Capabilities
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

// Copyright (C) 2021-2024 EpicChain Labs.

//
// NetworkAddressWithTime.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Capabilities;
using System;
using System.IO;
using System.Linq;
using System.Net;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Sent with an <see cref="AddrPayload"/> to respond to <see cref="MessageCommand.GetAddr"/> messages.
    /// </summary>
    public class NetworkAddressWithTime : ISerializable
    {
        /// <summary>
        /// The time when connected to the node.
        /// </summary>
        public uint Timestamp;

        /// <summary>
        /// The address of the node.
        /// </summary>
        public IPAddress Address;

        /// <summary>
        /// The capabilities of the node.
        /// </summary>
        public NodeCapability[] Capabilities;

        /// <summary>
        /// The <see cref="IPEndPoint"/> of the Tcp server.
        /// </summary>
        public IPEndPoint EndPoint => new(Address, Capabilities.Where(p => p.Type == NodeCapabilityType.TcpServer).Select(p => (ServerCapability)p).FirstOrDefault()?.Port ?? 0);

        public int Size => sizeof(uint) + 16 + Capabilities.GetVarSize();

        /// <summary>
        /// Creates a new instance of the <see cref="NetworkAddressWithTime"/> class.
        /// </summary>
        /// <param name="address">The address of the node.</param>
        /// <param name="timestamp">The time when connected to the node.</param>
        /// <param name="capabilities">The capabilities of the node.</param>
        /// <returns>The created payload.</returns>
        public static NetworkAddressWithTime Create(IPAddress address, uint timestamp, params NodeCapability[] capabilities)
        {
            return new NetworkAddressWithTime
            {
                Timestamp = timestamp,
                Address = address,
                Capabilities = capabilities
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Timestamp = reader.ReadUInt32();

            // Address
            ReadOnlyMemory<byte> data = reader.ReadMemory(16);
            Address = new IPAddress(data.Span).UnMap();

            // Capabilities
            Capabilities = new NodeCapability[reader.ReadVarInt(VersionPayload.MaxCapabilities)];
            for (int x = 0, max = Capabilities.Length; x < max; x++)
                Capabilities[x] = NodeCapability.DeserializeFrom(ref reader);
            if (Capabilities.Select(p => p.Type).Distinct().Count() != Capabilities.Length)
                throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(Timestamp);
            writer.Write(Address.MapToIPv6().GetAddressBytes());
            writer.Write(Capabilities);
        }
    }
}

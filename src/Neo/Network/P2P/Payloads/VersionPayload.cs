// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// VersionPayload.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.Network.P2P.Capabilities;
using System;
using System.IO;
using System.Linq;

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// Sent when a connection is established.
    /// </summary>
    public class VersionPayload : ISerializable
    {
        /// <summary>
        /// Indicates the maximum number of capabilities contained in a <see cref="VersionPayload"/>.
        /// </summary>
        public const int MaxCapabilities = 32;

        /// <summary>
        /// The magic number of the network.
        /// </summary>
        public uint Network;

        /// <summary>
        /// The protocol version of the node.
        /// </summary>
        public uint Version;

        /// <summary>
        /// The time when connected to the node.
        /// </summary>
        public uint Timestamp;

        /// <summary>
        /// A random number used to identify the node.
        /// </summary>
        public uint Nonce;

        /// <summary>
        /// A <see cref="string"/> used to identify the client software of the node.
        /// </summary>
        public string UserAgent;

        /// <summary>
        /// The capabilities of the node.
        /// </summary>
        public NodeCapability[] Capabilities;

        public int Size =>
            sizeof(uint) +              // Network
            sizeof(uint) +              // Version
            sizeof(uint) +              // Timestamp
            sizeof(uint) +              // Nonce
            UserAgent.GetVarSize() +    // UserAgent
            Capabilities.GetVarSize();  // Capabilities

        /// <summary>
        /// Creates a new instance of the <see cref="VersionPayload"/> class.
        /// </summary>
        /// <param name="network">The magic number of the network.</param>
        /// <param name="nonce">The random number used to identify the node.</param>
        /// <param name="userAgent">The <see cref="string"/> used to identify the client software of the node.</param>
        /// <param name="capabilities">The capabilities of the node.</param>
        /// <returns></returns>
        public static VersionPayload Create(uint network, uint nonce, string userAgent, params NodeCapability[] capabilities)
        {
            return new VersionPayload
            {
                Network = network,
                Version = LocalNode.ProtocolVersion,
                Timestamp = DateTime.Now.ToTimestamp(),
                Nonce = nonce,
                UserAgent = userAgent,
                Capabilities = capabilities,
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Network = reader.ReadUInt32();
            Version = reader.ReadUInt32();
            Timestamp = reader.ReadUInt32();
            Nonce = reader.ReadUInt32();
            UserAgent = reader.ReadVarString(1024);

            // Capabilities
            Capabilities = new NodeCapability[reader.ReadVarInt(MaxCapabilities)];
            for (int x = 0, max = Capabilities.Length; x < max; x++)
                Capabilities[x] = NodeCapability.DeserializeFrom(ref reader);
            if (Capabilities.Select(p => p.Type).Distinct().Count() != Capabilities.Length)
                throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(Network);
            writer.Write(Version);
            writer.Write(Timestamp);
            writer.Write(Nonce);
            writer.WriteVarString(UserAgent);
            writer.Write(Capabilities);
        }
    }
}

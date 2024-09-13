// Copyright (C) 2021-2024 EpicChain Labs.

//
// FullNodeCapability.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.IO;

namespace EpicChain.Network.P2P.Capabilities
{
    /// <summary>
    /// Indicates that a node has complete block data.
    /// </summary>
    public class FullNodeCapability : NodeCapability
    {
        /// <summary>
        /// Indicates the current block height of the node.
        /// </summary>
        public uint StartHeight;

        public override int Size =>
            base.Size +    // Type
            sizeof(uint);  // Start Height

        /// <summary>
        /// Initializes a new instance of the <see cref="FullNodeCapability"/> class.
        /// </summary>
        /// <param name="startHeight">The current block height of the node.</param>
        public FullNodeCapability(uint startHeight = 0) : base(NodeCapabilityType.FullNode)
        {
            StartHeight = startHeight;
        }

        protected override void DeserializeWithoutType(ref MemoryReader reader)
        {
            StartHeight = reader.ReadUInt32();
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(StartHeight);
        }
    }
}

// Copyright (C) 2021-2024 The EpicChain Labs.
//
// FullNodeCapability.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System.IO;

namespace Neo.Network.P2P.Capabilities
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

// Copyright (C) 2021-2024 EpicChain Labs.

//
// IVerifiable.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Persistence;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents an object that can be verified in the EpicChain network.
    /// </summary>
    public interface IVerifiable : ISerializable
    {
        /// <summary>
        /// The hash of the <see cref="IVerifiable"/> object.
        /// </summary>
        UInt256 Hash => this.CalculateHash();

        /// <summary>
        /// The witnesses of the <see cref="IVerifiable"/> object.
        /// </summary>
        Witness[] Witnesses { get; set; }

        /// <summary>
        /// Deserializes the part of the <see cref="IVerifiable"/> object other than <see cref="Witnesses"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        void DeserializeUnsigned(ref MemoryReader reader);

        /// <summary>
        /// Gets the script hashes that should be verified for this <see cref="IVerifiable"/> object.
        /// </summary>
        /// <param name="snapshot">The snapshot to be used.</param>
        /// <returns>The script hashes that should be verified.</returns>
        UInt160[] GetScriptHashesForVerifying(DataCache snapshot);

        /// <summary>
        /// Serializes the part of the <see cref="IVerifiable"/> object other than <see cref="Witnesses"/>.
        /// </summary>
        /// <param name="writer">The <see cref="BinaryWriter"/> for writing data.</param>
        void SerializeUnsigned(BinaryWriter writer);
    }
}

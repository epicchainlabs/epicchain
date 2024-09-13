// Copyright (C) 2021-2024 EpicChain Labs.

//
// Helper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using System.IO;

namespace EpicChain.Network.P2P
{
    /// <summary>
    /// A helper class for <see cref="IVerifiable"/>.
    /// </summary>
    public static class Helper
    {
        /// <summary>
        /// Calculates the hash of a <see cref="IVerifiable"/>.
        /// </summary>
        /// <param name="verifiable">The <see cref="IVerifiable"/> object to hash.</param>
        /// <returns>The hash of the object.</returns>
        public static UInt256 CalculateHash(this IVerifiable verifiable)
        {
            using MemoryStream ms = new();
            using BinaryWriter writer = new(ms);
            verifiable.SerializeUnsigned(writer);
            writer.Flush();
            return new UInt256(ms.ToArray().Sha256());
        }

        /// <summary>
        /// Gets the data of a <see cref="IVerifiable"/> object to be hashed.
        /// </summary>
        /// <param name="verifiable">The <see cref="IVerifiable"/> object to hash.</param>
        /// <param name="network">The magic number of the network.</param>
        /// <returns>The data to hash.</returns>
        public static byte[] GetSignData(this IVerifiable verifiable, uint network)
        {
            using MemoryStream ms = new();
            using BinaryWriter writer = new(ms);
            writer.Write(network);
            writer.Write(verifiable.Hash);
            writer.Flush();
            return ms.ToArray();
        }
    }
}

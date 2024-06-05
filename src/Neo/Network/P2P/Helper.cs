// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Helper.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Cryptography;
using Neo.IO;
using Neo.Network.P2P.Payloads;
using System.IO;

namespace Neo.Network.P2P
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

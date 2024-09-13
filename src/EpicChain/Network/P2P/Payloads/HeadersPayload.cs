// Copyright (C) 2021-2024 EpicChain Labs.

//
// HeadersPayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// This message is sent to respond to <see cref="MessageCommand.GetHeaders"/> messages.
    /// </summary>
    public class HeadersPayload : ISerializable
    {
        /// <summary>
        /// Indicates the maximum number of headers sent each time.
        /// </summary>
        public const int MaxHeadersCount = 2000;

        /// <summary>
        /// The list of headers.
        /// </summary>
        public Header[] Headers;

        public int Size => Headers.GetVarSize();

        /// <summary>
        /// Creates a new instance of the <see cref="HeadersPayload"/> class.
        /// </summary>
        /// <param name="headers">The list of headers.</param>
        /// <returns>The created payload.</returns>
        public static HeadersPayload Create(params Header[] headers)
        {
            return new HeadersPayload
            {
                Headers = headers
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Headers = reader.ReadSerializableArray<Header>(MaxHeadersCount);
            if (Headers.Length == 0) throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(Headers);
        }
    }
}

// Copyright (C) 2021-2024 The EpicChain Labs.
//
// HeadersPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.Network.P2P.Payloads
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

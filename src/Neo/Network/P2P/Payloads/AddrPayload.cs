// Copyright (C) 2021-2024 The EpicChain Labs.
//
// AddrPayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
    /// This message is sent to respond to <see cref="MessageCommand.GetAddr"/> messages.
    /// </summary>
    public class AddrPayload : ISerializable
    {
        /// <summary>
        /// Indicates the maximum number of nodes sent each time.
        /// </summary>
        public const int MaxCountToSend = 200;

        /// <summary>
        /// The list of nodes.
        /// </summary>
        public NetworkAddressWithTime[] AddressList;

        public int Size => AddressList.GetVarSize();

        /// <summary>
        /// Creates a new instance of the <see cref="AddrPayload"/> class.
        /// </summary>
        /// <param name="addresses">The list of nodes.</param>
        /// <returns>The created payload.</returns>
        public static AddrPayload Create(params NetworkAddressWithTime[] addresses)
        {
            return new AddrPayload
            {
                AddressList = addresses
            };
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            AddressList = reader.ReadSerializableArray<NetworkAddressWithTime>(MaxCountToSend);
            if (AddressList.Length == 0)
                throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(AddressList);
        }
    }
}

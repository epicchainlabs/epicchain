// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Keys.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.Buffers.Binary;

namespace Neo.Plugins.StateService.Storage
{
    public static class Keys
    {
        public static byte[] StateRoot(uint index)
        {
            byte[] buffer = new byte[sizeof(uint) + 1];
            buffer[0] = 1;
            BinaryPrimitives.WriteUInt32BigEndian(buffer.AsSpan(1), index);
            return buffer;
        }

        public static readonly byte[] CurrentLocalRootIndex = { 0x02 };
        public static readonly byte[] CurrentValidatedRootIndex = { 0x04 };
    }
}

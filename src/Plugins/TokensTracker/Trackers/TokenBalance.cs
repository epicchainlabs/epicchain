// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TokenBalance.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System.Numerics;

namespace Neo.Plugins.Trackers
{
    public class TokenBalance : ISerializable
    {
        public BigInteger Balance;
        public uint LastUpdatedBlock;

        int ISerializable.Size =>
            Balance.GetVarSize() +    // Balance
            sizeof(uint);             // LastUpdatedBlock

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.WriteVarBytes(Balance.ToByteArray());
            writer.Write(LastUpdatedBlock);
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Balance = new BigInteger(reader.ReadVarMemory(32).Span);
            LastUpdatedBlock = reader.ReadUInt32();
        }
    }
}

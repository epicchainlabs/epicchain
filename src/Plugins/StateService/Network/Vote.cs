// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Vote.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.Plugins.StateService.Network
{
    class Vote : ISerializable
    {
        public int ValidatorIndex;
        public uint RootIndex;
        public ReadOnlyMemory<byte> Signature;

        int ISerializable.Size => sizeof(int) + sizeof(uint) + Signature.GetVarSize();

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(ValidatorIndex);
            writer.Write(RootIndex);
            writer.WriteVarBytes(Signature.Span);
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            ValidatorIndex = reader.ReadInt32();
            RootIndex = reader.ReadUInt32();
            Signature = reader.ReadVarMemory(64);
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// TokenTransferKey.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Buffers.Binary;
using System.IO;

namespace EpicChain.Plugins.Trackers
{
    public class TokenTransferKey : ISerializable
    {
        public UInt160 UserScriptHash { get; protected set; }
        public ulong TimestampMS { get; protected set; }
        public UInt160 AssetScriptHash { get; protected set; }
        public uint BlockXferNotificationIndex { get; protected set; }

        public TokenTransferKey(UInt160 userScriptHash, ulong timestamp, UInt160 assetScriptHash, uint xferIndex)
        {
            if (userScriptHash is null || assetScriptHash is null)
                throw new ArgumentNullException();
            UserScriptHash = userScriptHash;
            TimestampMS = timestamp;
            AssetScriptHash = assetScriptHash;
            BlockXferNotificationIndex = xferIndex;
        }
        public virtual void Serialize(BinaryWriter writer)
        {
            writer.Write(UserScriptHash);
            writer.Write(BitConverter.IsLittleEndian ? BinaryPrimitives.ReverseEndianness(TimestampMS) : TimestampMS);
            writer.Write(AssetScriptHash);
            writer.Write(BlockXferNotificationIndex);
        }

        public virtual void Deserialize(ref MemoryReader reader)
        {
            UserScriptHash.Deserialize(ref reader);
            TimestampMS = BitConverter.IsLittleEndian ? BinaryPrimitives.ReverseEndianness(reader.ReadUInt64()) : reader.ReadUInt64();
            AssetScriptHash.Deserialize(ref reader);
            BlockXferNotificationIndex = reader.ReadUInt32();
        }

        public virtual int Size =>
              UInt160.Length +    //UserScriptHash
              sizeof(ulong) +     //TimestampMS
              UInt160.Length +    //AssetScriptHash
              sizeof(uint);       //BlockXferNotificationIndex
    }
}

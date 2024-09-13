// Copyright (C) 2021-2024 EpicChain Labs.

//
// PrepareRequest.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Plugins.DBFTPlugin.Types;
using System;
using System.IO;
using System.Linq;

namespace EpicChain.Plugins.DBFTPlugin.Messages
{
    public class PrepareRequest : ConsensusMessage
    {
        public uint Version;
        public UInt256 PrevHash;
        public ulong Timestamp;
        public ulong Nonce;
        public UInt256[] TransactionHashes;

        public override int Size => base.Size
            + sizeof(uint)                      //Version
            + UInt256.Length                    //PrevHash
            + sizeof(ulong)                     //Timestamp
            + sizeof(ulong)                     // Nonce
            + TransactionHashes.GetVarSize();   //TransactionHashes

        public PrepareRequest() : base(ConsensusMessageType.PrepareRequest) { }

        public override void Deserialize(ref MemoryReader reader)
        {
            base.Deserialize(ref reader);
            Version = reader.ReadUInt32();
            PrevHash = reader.ReadSerializable<UInt256>();
            Timestamp = reader.ReadUInt64();
            Nonce = reader.ReadUInt64();
            TransactionHashes = reader.ReadSerializableArray<UInt256>(ushort.MaxValue);
            if (TransactionHashes.Distinct().Count() != TransactionHashes.Length)
                throw new FormatException();
        }

        public override bool Verify(ProtocolSettings protocolSettings)
        {
            if (!base.Verify(protocolSettings)) return false;
            return TransactionHashes.Length <= protocolSettings.MaxTransactionsPerBlock;
        }

        public override void Serialize(BinaryWriter writer)
        {
            base.Serialize(writer);
            writer.Write(Version);
            writer.Write(PrevHash);
            writer.Write(Timestamp);
            writer.Write(Nonce);
            writer.Write(TransactionHashes);
        }
    }
}

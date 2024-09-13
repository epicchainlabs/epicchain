// Copyright (C) 2021-2024 EpicChain Labs.

//
// ConsensusMessage.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Plugins.DBFTPlugin.Messages
{
    public abstract class ConsensusMessage : ISerializable
    {
        public readonly ConsensusMessageType Type;
        public uint BlockIndex;
        public byte ValidatorIndex;
        public byte ViewNumber;

        public virtual int Size =>
            sizeof(ConsensusMessageType) +  //Type
            sizeof(uint) +                  //BlockIndex
            sizeof(byte) +                  //ValidatorIndex
            sizeof(byte);                   //ViewNumber

        protected ConsensusMessage(ConsensusMessageType type)
        {
            if (!Enum.IsDefined(typeof(ConsensusMessageType), type))
                throw new ArgumentOutOfRangeException(nameof(type));
            Type = type;
        }

        public virtual void Deserialize(ref MemoryReader reader)
        {
            if (Type != (ConsensusMessageType)reader.ReadByte())
                throw new FormatException();
            BlockIndex = reader.ReadUInt32();
            ValidatorIndex = reader.ReadByte();
            ViewNumber = reader.ReadByte();
        }

        public static ConsensusMessage DeserializeFrom(ReadOnlyMemory<byte> data)
        {
            ConsensusMessageType type = (ConsensusMessageType)data.Span[0];
            Type t = typeof(ConsensusMessage);
            t = t.Assembly.GetType($"{t.Namespace}.{type}", false);
            if (t is null) throw new FormatException();
            return (ConsensusMessage)data.AsSerializable(t);
        }

        public virtual bool Verify(ProtocolSettings protocolSettings)
        {
            return ValidatorIndex < protocolSettings.ValidatorsCount;
        }

        public virtual void Serialize(BinaryWriter writer)
        {
            writer.Write((byte)Type);
            writer.Write(BlockIndex);
            writer.Write(ValidatorIndex);
            writer.Write(ViewNumber);
        }
    }
}

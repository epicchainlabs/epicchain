// Copyright (C) 2021-2024 EpicChain Labs.

//
// RecoveryMessage.CommitPayloadCompact.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Plugins.DBFTPlugin.Messages
{
    partial class RecoveryMessage
    {
        public class CommitPayloadCompact : ISerializable
        {
            public byte ViewNumber;
            public byte ValidatorIndex;
            public ReadOnlyMemory<byte> Signature;
            public ReadOnlyMemory<byte> InvocationScript;

            int ISerializable.Size =>
                sizeof(byte) +                  //ViewNumber
                sizeof(byte) +                  //ValidatorIndex
                Signature.Length +              //Signature
                InvocationScript.GetVarSize();  //InvocationScript

            void ISerializable.Deserialize(ref MemoryReader reader)
            {
                ViewNumber = reader.ReadByte();
                ValidatorIndex = reader.ReadByte();
                Signature = reader.ReadMemory(64);
                InvocationScript = reader.ReadVarMemory(1024);
            }

            void ISerializable.Serialize(BinaryWriter writer)
            {
                writer.Write(ViewNumber);
                writer.Write(ValidatorIndex);
                writer.Write(Signature.Span);
                writer.WriteVarBytes(InvocationScript.Span);
            }
        }
    }
}

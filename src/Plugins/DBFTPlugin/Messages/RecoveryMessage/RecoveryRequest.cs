// Copyright (C) 2021-2024 EpicChain Labs.

//
// RecoveryRequest.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.IO;

namespace EpicChain.Plugins.DBFTPlugin.Messages
{
    public class RecoveryRequest : ConsensusMessage
    {
        /// <summary>
        /// Timestamp of when the ChangeView message was created. This allows receiving nodes to ensure
        /// they only respond once to a specific RecoveryRequest request.
        /// In this sense, it prevents replay of the RecoveryRequest message from the repeatedly broadcast of Recovery's messages.
        /// </summary>
        public ulong Timestamp;

        public override int Size => base.Size
            + sizeof(ulong); //Timestamp

        public RecoveryRequest() : base(ConsensusMessageType.RecoveryRequest) { }

        public override void Deserialize(ref MemoryReader reader)
        {
            base.Deserialize(ref reader);
            Timestamp = reader.ReadUInt64();
        }

        public override void Serialize(BinaryWriter writer)
        {
            base.Serialize(writer);
            writer.Write(Timestamp);
        }
    }
}

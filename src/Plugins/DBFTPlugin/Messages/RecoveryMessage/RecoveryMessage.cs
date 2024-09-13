// Copyright (C) 2021-2024 EpicChain Labs.

//
// RecoveryMessage.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins.DBFTPlugin.Consensus;
using EpicChain.Plugins.DBFTPlugin.Types;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace EpicChain.Plugins.DBFTPlugin.Messages
{
    public partial class RecoveryMessage : ConsensusMessage
    {
        public Dictionary<byte, ChangeViewPayloadCompact> ChangeViewMessages;
        public PrepareRequest PrepareRequestMessage;
        /// The PreparationHash in case the PrepareRequest hasn't been received yet.
        /// This can be null if the PrepareRequest information is present, since it can be derived in that case.
        public UInt256 PreparationHash;
        public Dictionary<byte, PreparationPayloadCompact> PreparationMessages;
        public Dictionary<byte, CommitPayloadCompact> CommitMessages;

        public override int Size => base.Size
            + /* ChangeViewMessages */ ChangeViewMessages?.Values.GetVarSize() ?? 0
            + /* PrepareRequestMessage */ 1 + PrepareRequestMessage?.Size ?? 0
            + /* PreparationHash */ PreparationHash?.Size ?? 0
            + /* PreparationMessages */ PreparationMessages?.Values.GetVarSize() ?? 0
            + /* CommitMessages */ CommitMessages?.Values.GetVarSize() ?? 0;

        public RecoveryMessage() : base(ConsensusMessageType.RecoveryMessage) { }

        public override void Deserialize(ref MemoryReader reader)
        {
            base.Deserialize(ref reader);
            ChangeViewMessages = reader.ReadSerializableArray<ChangeViewPayloadCompact>(byte.MaxValue).ToDictionary(p => p.ValidatorIndex);
            if (reader.ReadBoolean())
            {
                PrepareRequestMessage = reader.ReadSerializable<PrepareRequest>();
            }
            else
            {
                int preparationHashSize = UInt256.Zero.Size;
                if (preparationHashSize == (int)reader.ReadVarInt((ulong)preparationHashSize))
                    PreparationHash = new UInt256(reader.ReadMemory(preparationHashSize).Span);
            }

            PreparationMessages = reader.ReadSerializableArray<PreparationPayloadCompact>(byte.MaxValue).ToDictionary(p => p.ValidatorIndex);
            CommitMessages = reader.ReadSerializableArray<CommitPayloadCompact>(byte.MaxValue).ToDictionary(p => p.ValidatorIndex);
        }

        public override bool Verify(ProtocolSettings protocolSettings)
        {
            if (!base.Verify(protocolSettings)) return false;
            return (PrepareRequestMessage is null || PrepareRequestMessage.Verify(protocolSettings))
                && ChangeViewMessages.Values.All(p => p.ValidatorIndex < protocolSettings.ValidatorsCount)
                && PreparationMessages.Values.All(p => p.ValidatorIndex < protocolSettings.ValidatorsCount)
                && CommitMessages.Values.All(p => p.ValidatorIndex < protocolSettings.ValidatorsCount);
        }

        internal ExtensiblePayload[] GetChangeViewPayloads(ConsensusContext context)
        {
            return ChangeViewMessages.Values.Select(p => context.CreatePayload(new ChangeView
            {
                BlockIndex = BlockIndex,
                ValidatorIndex = p.ValidatorIndex,
                ViewNumber = p.OriginalViewNumber,
                Timestamp = p.Timestamp
            }, p.InvocationScript)).ToArray();
        }

        internal ExtensiblePayload[] GetCommitPayloadsFromRecoveryMessage(ConsensusContext context)
        {
            return CommitMessages.Values.Select(p => context.CreatePayload(new Commit
            {
                BlockIndex = BlockIndex,
                ValidatorIndex = p.ValidatorIndex,
                ViewNumber = p.ViewNumber,
                Signature = p.Signature
            }, p.InvocationScript)).ToArray();
        }

        internal ExtensiblePayload GetPrepareRequestPayload(ConsensusContext context)
        {
            if (PrepareRequestMessage == null) return null;
            if (!PreparationMessages.TryGetValue(context.Block.PrimaryIndex, out PreparationPayloadCompact compact))
                return null;
            return context.CreatePayload(PrepareRequestMessage, compact.InvocationScript);
        }

        internal ExtensiblePayload[] GetPrepareResponsePayloads(ConsensusContext context)
        {
            UInt256 preparationHash = PreparationHash ?? context.PreparationPayloads[context.Block.PrimaryIndex]?.Hash;
            if (preparationHash is null) return Array.Empty<ExtensiblePayload>();
            return PreparationMessages.Values.Where(p => p.ValidatorIndex != context.Block.PrimaryIndex).Select(p => context.CreatePayload(new PrepareResponse
            {
                BlockIndex = BlockIndex,
                ValidatorIndex = p.ValidatorIndex,
                ViewNumber = ViewNumber,
                PreparationHash = preparationHash
            }, p.InvocationScript)).ToArray();
        }

        public override void Serialize(BinaryWriter writer)
        {
            base.Serialize(writer);
            writer.Write(ChangeViewMessages.Values.ToArray());
            bool hasPrepareRequestMessage = PrepareRequestMessage != null;
            writer.Write(hasPrepareRequestMessage);
            if (hasPrepareRequestMessage)
                writer.Write(PrepareRequestMessage);
            else
            {
                if (PreparationHash == null)
                    writer.WriteVarInt(0);
                else
                    writer.WriteVarBytes(PreparationHash.ToArray());
            }

            writer.Write(PreparationMessages.Values.ToArray());
            writer.Write(CommitMessages.Values.ToArray());
        }
    }
}

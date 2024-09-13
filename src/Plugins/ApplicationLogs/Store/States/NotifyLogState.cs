// Copyright (C) 2021-2024 EpicChain Labs.

//
// NotifyLogState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain;
using EpicChain.IO;
using EpicChain.SmartContract;

namespace EpicChain.Plugins.ApplicationLogs.Store.States
{
    public class NotifyLogState : ISerializable, IEquatable<NotifyLogState>
    {
        public UInt160 ScriptHash { get; protected set; } = new();
        public string EventName { get; protected set; } = string.Empty;
        public Guid[] StackItemIds { get; protected set; } = [];

        public static NotifyLogState Create(NotifyEventArgs notifyItem, Guid[] stackItemsIds) =>
            new()
            {
                ScriptHash = notifyItem.ScriptHash,
                EventName = notifyItem.EventName,
                StackItemIds = stackItemsIds,
            };

        #region ISerializable

        public virtual int Size =>
            ScriptHash.Size +
            EventName.GetVarSize() +
            StackItemIds.Sum(s => s.ToByteArray().GetVarSize());

        public virtual void Deserialize(ref MemoryReader reader)
        {
            ScriptHash.Deserialize(ref reader);
            EventName = reader.ReadVarString();

            // It should be safe because it filled from a transaction's notifications.
            uint aLen = reader.ReadUInt32();
            StackItemIds = new Guid[aLen];
            for (var i = 0; i < aLen; i++)
                StackItemIds[i] = new Guid(reader.ReadVarMemory().Span);
        }

        public virtual void Serialize(BinaryWriter writer)
        {
            ScriptHash.Serialize(writer);
            writer.WriteVarString(EventName ?? string.Empty);

            writer.Write((uint)StackItemIds.Length);
            for (var i = 0; i < StackItemIds.Length; i++)
                writer.WriteVarBytes(StackItemIds[i].ToByteArray());
        }

        #endregion

        #region IEquatable

        public bool Equals(NotifyLogState other) =>
            EventName == other.EventName && ScriptHash == other.ScriptHash &&
            StackItemIds.SequenceEqual(other.StackItemIds);

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(this, obj)) return true;
            return Equals(obj as NotifyLogState);
        }

        public override int GetHashCode()
        {
            var h = new HashCode();
            h.Add(ScriptHash);
            h.Add(EventName);
            foreach (var id in StackItemIds)
                h.Add(id);
            return h.ToHashCode();
        }

        #endregion
    }
}

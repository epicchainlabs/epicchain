// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionLogState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Plugins.ApplicationLogs.Store.States
{
    public class TransactionLogState : ISerializable, IEquatable<TransactionLogState>
    {
        public Guid[] NotifyLogIds { get; private set; } = Array.Empty<Guid>();

        public static TransactionLogState Create(Guid[] notifyLogIds) =>
            new()
            {
                NotifyLogIds = notifyLogIds,
            };

        #region ISerializable

        public virtual int Size =>
            sizeof(uint) +
            NotifyLogIds.Sum(s => s.ToByteArray().GetVarSize());

        public virtual void Deserialize(ref MemoryReader reader)
        {
            // It should be safe because it filled from a transaction's notifications.
            uint aLen = reader.ReadUInt32();
            NotifyLogIds = new Guid[aLen];
            for (int i = 0; i < aLen; i++)
                NotifyLogIds[i] = new Guid(reader.ReadVarMemory().Span);
        }

        public virtual void Serialize(BinaryWriter writer)
        {
            writer.Write((uint)NotifyLogIds.Length);
            for (int i = 0; i < NotifyLogIds.Length; i++)
                writer.WriteVarBytes(NotifyLogIds[i].ToByteArray());
        }

        #endregion

        #region IEquatable

        public bool Equals(TransactionLogState other) =>
            NotifyLogIds.SequenceEqual(other.NotifyLogIds);

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(this, obj)) return true;
            return Equals(obj as TransactionLogState);
        }

        public override int GetHashCode()
        {
            var h = new HashCode();
            foreach (var id in NotifyLogIds)
                h.Add(id);
            return h.ToHashCode();
        }

        #endregion
    }
}

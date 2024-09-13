// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractLogState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Ledger;
using EpicChain.SmartContract;

namespace EpicChain.Plugins.ApplicationLogs.Store.States
{
    public class ContractLogState : NotifyLogState, IEquatable<ContractLogState>
    {
        public UInt256 TransactionHash { get; private set; } = new();
        public TriggerType Trigger { get; private set; } = TriggerType.All;

        public static ContractLogState Create(Blockchain.ApplicationExecuted applicationExecuted, NotifyEventArgs notifyEventArgs, Guid[] stackItemIds) =>
            new()
            {
                TransactionHash = applicationExecuted.Transaction?.Hash ?? new(),
                ScriptHash = notifyEventArgs.ScriptHash,
                Trigger = applicationExecuted.Trigger,
                EventName = notifyEventArgs.EventName,
                StackItemIds = stackItemIds,
            };

        #region ISerializable

        public override int Size =>
            TransactionHash.Size +
            sizeof(byte) +
            base.Size;

        public override void Deserialize(ref MemoryReader reader)
        {
            TransactionHash.Deserialize(ref reader);
            Trigger = (TriggerType)reader.ReadByte();
            base.Deserialize(ref reader);
        }

        public override void Serialize(BinaryWriter writer)
        {
            TransactionHash.Serialize(writer);
            writer.Write((byte)Trigger);
            base.Serialize(writer);
        }

        #endregion

        #region IEquatable

        public bool Equals(ContractLogState other) =>
            Trigger == other.Trigger && EventName == other.EventName &&
            TransactionHash == other.TransactionHash && StackItemIds.SequenceEqual(other.StackItemIds);

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(this, obj)) return true;
            return Equals(obj as ContractLogState);
        }

        public override int GetHashCode() =>
            HashCode.Combine(TransactionHash, Trigger, base.GetHashCode());

        #endregion
    }
}

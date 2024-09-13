// Copyright (C) 2021-2024 EpicChain Labs.

//
// ExecutionLogState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Ledger;
using EpicChain.VM;

namespace EpicChain.Plugins.ApplicationLogs.Store.States
{
    public class ExecutionLogState : ISerializable, IEquatable<ExecutionLogState>
    {
        public VMState VmState { get; private set; } = VMState.NONE;
        public string Exception { get; private set; } = string.Empty;
        public long EpicPulseConsumed { get; private set; } = 0L;
        public Guid[] StackItemIds { get; private set; } = [];

        public static ExecutionLogState Create(Blockchain.ApplicationExecuted appExecution, Guid[] stackItemIds) =>
            new()
            {
                VmState = appExecution.VMState,
                Exception = appExecution.Exception?.InnerException?.Message ?? appExecution.Exception?.Message!,
                EpicPulseConsumed = appExecution.EpicPulseConsumed,
                StackItemIds = stackItemIds,
            };

        #region ISerializable

        public int Size =>
            sizeof(byte) +
            Exception.GetVarSize() +
            sizeof(long) +
            sizeof(uint) +
            StackItemIds.Sum(s => s.ToByteArray().GetVarSize());

        public void Deserialize(ref MemoryReader reader)
        {
            VmState = (VMState)reader.ReadByte();
            Exception = reader.ReadVarString();
            EpicPulseConsumed = reader.ReadInt64();

            // It should be safe because it filled from a transaction's stack.
            uint aLen = reader.ReadUInt32();
            StackItemIds = new Guid[aLen];
            for (int i = 0; i < aLen; i++)
                StackItemIds[i] = new Guid(reader.ReadVarMemory().Span);
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write((byte)VmState);
            writer.WriteVarString(Exception ?? string.Empty);
            writer.Write(EpicPulseConsumed);

            writer.Write((uint)StackItemIds.Length);
            for (int i = 0; i < StackItemIds.Length; i++)
                writer.WriteVarBytes(StackItemIds[i].ToByteArray());
        }

        #endregion

        #region IEquatable

        public bool Equals(ExecutionLogState other) =>
            VmState == other.VmState && Exception == other.Exception &&
            EpicPulseConsumed == other.EpicPulseConsumed && StackItemIds.SequenceEqual(other.StackItemIds);

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(this, obj)) return true;
            return Equals(obj as ExecutionLogState);
        }

        public override int GetHashCode()
        {
            var h = new HashCode();
            h.Add(VmState);
            h.Add(Exception);
            h.Add(EpicPulseConsumed);
            foreach (var id in StackItemIds)
                h.Add(id);
            return h.ToHashCode();
        }

        #endregion
    }
}

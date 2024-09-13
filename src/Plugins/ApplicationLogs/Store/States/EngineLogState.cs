// Copyright (C) 2021-2024 EpicChain Labs.

//
// EngineLogState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    public class EngineLogState : ISerializable, IEquatable<EngineLogState>
    {
        public UInt160 ScriptHash { get; private set; } = new();
        public string Message { get; private set; } = string.Empty;

        public static EngineLogState Create(UInt160 scriptHash, string message) =>
            new()
            {
                ScriptHash = scriptHash,
                Message = message,
            };

        #region ISerializable

        public virtual int Size =>
            ScriptHash.Size +
            Message.GetVarSize();

        public virtual void Deserialize(ref MemoryReader reader)
        {
            ScriptHash.Deserialize(ref reader);
            // It should be safe because it filled from a transaction's logs.
            Message = reader.ReadVarString();
        }

        public virtual void Serialize(BinaryWriter writer)
        {
            ScriptHash.Serialize(writer);
            writer.WriteVarString(Message ?? string.Empty);
        }

        #endregion

        #region IEquatable

        public bool Equals(EngineLogState other) =>
            ScriptHash == other.ScriptHash &&
            Message == other.Message;

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(this, obj)) return true;
            return Equals(obj as EngineLogState);
        }

        public override int GetHashCode() =>
            HashCode.Combine(ScriptHash, Message);

        #endregion
    }
}

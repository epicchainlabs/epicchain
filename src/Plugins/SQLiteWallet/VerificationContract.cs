// Copyright (C) 2021-2024 EpicChain Labs.

//
// VerificationContract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;

namespace EpicChain.Wallets.SQLite;

class VerificationContract : SmartContract.Contract, IEquatable<VerificationContract>, ISerializable
{
    public int Size => ParameterList.GetVarSize() + Script.GetVarSize();

    public void Deserialize(ref MemoryReader reader)
    {
        ReadOnlySpan<byte> span = reader.ReadVarMemory().Span;
        ParameterList = new ContractParameterType[span.Length];
        for (int i = 0; i < span.Length; i++)
        {
            ParameterList[i] = (ContractParameterType)span[i];
            if (!Enum.IsDefined(typeof(ContractParameterType), ParameterList[i]))
                throw new FormatException();
        }
        Script = reader.ReadVarMemory().ToArray();
    }

    public bool Equals(VerificationContract other)
    {
        if (ReferenceEquals(this, other)) return true;
        if (other is null) return false;
        return ScriptHash.Equals(other.ScriptHash);
    }

    public override bool Equals(object obj)
    {
        return Equals(obj as VerificationContract);
    }

    public override int GetHashCode()
    {
        return ScriptHash.GetHashCode();
    }

    public void Serialize(BinaryWriter writer)
    {
        writer.WriteVarBytes(ParameterList.Select(p => (byte)p).ToArray());
        writer.WriteVarBytes(Script);
    }
}

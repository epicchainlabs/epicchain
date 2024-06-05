// Copyright (C) 2021-2024 The EpicChain Labs.
//
// VerificationContract.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.IO;
using Neo.SmartContract;

namespace Neo.Wallets.SQLite;

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

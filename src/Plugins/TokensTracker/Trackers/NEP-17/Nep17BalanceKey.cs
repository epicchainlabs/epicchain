// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Nep17BalanceKey.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System;
using System.IO;

namespace Neo.Plugins.Trackers.NEP_17
{
    public class Nep17BalanceKey : IComparable<Nep17BalanceKey>, IEquatable<Nep17BalanceKey>, ISerializable
    {
        public readonly UInt160 UserScriptHash;
        public readonly UInt160 AssetScriptHash;

        public int Size => UInt160.Length + UInt160.Length;

        public Nep17BalanceKey() : this(new UInt160(), new UInt160())
        {
        }

        public Nep17BalanceKey(UInt160 userScriptHash, UInt160 assetScriptHash)
        {
            if (userScriptHash == null || assetScriptHash == null)
                throw new ArgumentNullException();
            UserScriptHash = userScriptHash;
            AssetScriptHash = assetScriptHash;
        }

        public int CompareTo(Nep17BalanceKey other)
        {
            if (other is null) return 1;
            if (ReferenceEquals(this, other)) return 0;
            int result = UserScriptHash.CompareTo(other.UserScriptHash);
            if (result != 0) return result;
            return AssetScriptHash.CompareTo(other.AssetScriptHash);
        }

        public bool Equals(Nep17BalanceKey other)
        {
            if (other is null) return false;
            if (ReferenceEquals(this, other)) return true;
            return UserScriptHash.Equals(other.UserScriptHash) && AssetScriptHash.Equals(AssetScriptHash);
        }

        public override bool Equals(Object other)
        {
            return other is Nep17BalanceKey otherKey && Equals(otherKey);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(UserScriptHash.GetHashCode(), AssetScriptHash.GetHashCode());
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(UserScriptHash);
            writer.Write(AssetScriptHash);
        }

        public void Deserialize(ref MemoryReader reader)
        {
            ((ISerializable)UserScriptHash).Deserialize(ref reader);
            ((ISerializable)AssetScriptHash).Deserialize(ref reader);
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// Xep17BalanceKey.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.IO;

namespace EpicChain.Plugins.Trackers.XEP_17
{
    public class Xep17BalanceKey : IComparable<Xep17BalanceKey>, IEquatable<Xep17BalanceKey>, ISerializable
    {
        public readonly UInt160 UserScriptHash;
        public readonly UInt160 AssetScriptHash;

        public int Size => UInt160.Length + UInt160.Length;

        public Xep17BalanceKey() : this(new UInt160(), new UInt160())
        {
        }

        public Xep17BalanceKey(UInt160 userScriptHash, UInt160 assetScriptHash)
        {
            if (userScriptHash == null || assetScriptHash == null)
                throw new ArgumentNullException();
            UserScriptHash = userScriptHash;
            AssetScriptHash = assetScriptHash;
        }

        public int CompareTo(Xep17BalanceKey other)
        {
            if (other is null) return 1;
            if (ReferenceEquals(this, other)) return 0;
            int result = UserScriptHash.CompareTo(other.UserScriptHash);
            if (result != 0) return result;
            return AssetScriptHash.CompareTo(other.AssetScriptHash);
        }

        public bool Equals(Xep17BalanceKey other)
        {
            if (other is null) return false;
            if (ReferenceEquals(this, other)) return true;
            return UserScriptHash.Equals(other.UserScriptHash) && AssetScriptHash.Equals(AssetScriptHash);
        }

        public override bool Equals(Object other)
        {
            return other is Xep17BalanceKey otherKey && Equals(otherKey);
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

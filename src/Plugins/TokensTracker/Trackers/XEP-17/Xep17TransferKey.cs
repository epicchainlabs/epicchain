// Copyright (C) 2021-2024 EpicChain Labs.

//
// Xep17TransferKey.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Plugins.Trackers.XEP_17
{
    public class Xep17TransferKey : TokenTransferKey, IComparable<Xep17TransferKey>, IEquatable<Xep17TransferKey>, ISerializable
    {
        public Xep17TransferKey() : base(new UInt160(), 0, new UInt160(), 0)
        {
        }

        public Xep17TransferKey(UInt160 userScriptHash, ulong timestamp, UInt160 assetScriptHash, uint xferIndex) : base(userScriptHash, timestamp, assetScriptHash, xferIndex)
        {
        }

        public int CompareTo(Xep17TransferKey other)
        {
            if (other is null) return 1;
            if (ReferenceEquals(this, other)) return 0;
            int result = UserScriptHash.CompareTo(other.UserScriptHash);
            if (result != 0) return result;
            int result2 = TimestampMS.CompareTo(other.TimestampMS);
            if (result2 != 0) return result2;
            int result3 = AssetScriptHash.CompareTo(other.AssetScriptHash);
            if (result3 != 0) return result3;
            return BlockXferNotificationIndex.CompareTo(other.BlockXferNotificationIndex);
        }

        public bool Equals(Xep17TransferKey other)
        {
            if (other is null) return false;
            if (ReferenceEquals(this, other)) return true;
            return UserScriptHash.Equals(other.UserScriptHash)
                   && TimestampMS.Equals(other.TimestampMS) && AssetScriptHash.Equals(other.AssetScriptHash)
                   && BlockXferNotificationIndex.Equals(other.BlockXferNotificationIndex);
        }

        public override bool Equals(Object other)
        {
            return other is Xep17TransferKey otherKey && Equals(otherKey);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(UserScriptHash.GetHashCode(), TimestampMS.GetHashCode(), AssetScriptHash.GetHashCode(), BlockXferNotificationIndex.GetHashCode());
        }
    }
}

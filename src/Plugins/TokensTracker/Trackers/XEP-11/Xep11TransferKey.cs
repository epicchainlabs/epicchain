// Copyright (C) 2021-2024 EpicChain Labs.

//
// Xep11TransferKey.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM.Types;
using System;
using System.IO;

namespace EpicChain.Plugins.Trackers.XEP_11
{
    public class Xep11TransferKey : TokenTransferKey, IComparable<Xep11TransferKey>, IEquatable<Xep11TransferKey>
    {
        public ByteString Token;
        public override int Size => base.Size + Token.GetVarSize();

        public Xep11TransferKey() : this(new UInt160(), 0, new UInt160(), ByteString.Empty, 0)
        {
        }

        public Xep11TransferKey(UInt160 userScriptHash, ulong timestamp, UInt160 assetScriptHash, ByteString tokenId, uint xferIndex) : base(userScriptHash, timestamp, assetScriptHash, xferIndex)
        {
            Token = tokenId;
        }

        public int CompareTo(Xep11TransferKey other)
        {
            if (other is null) return 1;
            if (ReferenceEquals(this, other)) return 0;
            int result = UserScriptHash.CompareTo(other.UserScriptHash);
            if (result != 0) return result;
            int result2 = TimestampMS.CompareTo(other.TimestampMS);
            if (result2 != 0) return result2;
            int result3 = AssetScriptHash.CompareTo(other.AssetScriptHash);
            if (result3 != 0) return result3;
            var result4 = BlockXferNotificationIndex.CompareTo(other.BlockXferNotificationIndex);
            if (result4 != 0) return result4;
            return (Token.GetInteger() - other.Token.GetInteger()).Sign;
        }

        public bool Equals(Xep11TransferKey other)
        {
            if (other is null) return false;
            if (ReferenceEquals(this, other)) return true;
            return UserScriptHash.Equals(other.UserScriptHash)
                   && TimestampMS.Equals(other.TimestampMS) && AssetScriptHash.Equals(other.AssetScriptHash)
                   && Token.Equals(other.Token)
                   && BlockXferNotificationIndex.Equals(other.BlockXferNotificationIndex);
        }

        public override bool Equals(Object other)
        {
            return other is Xep11TransferKey otherKey && Equals(otherKey);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(UserScriptHash.GetHashCode(), TimestampMS.GetHashCode(), AssetScriptHash.GetHashCode(), BlockXferNotificationIndex.GetHashCode(), Token.GetHashCode());
        }

        public override void Serialize(BinaryWriter writer)
        {
            base.Serialize(writer);
            writer.WriteVarBytes(Token.GetSpan());
        }

        public override void Deserialize(ref MemoryReader reader)
        {
            base.Deserialize(ref reader);
            Token = reader.ReadVarMemory();
        }
    }
}

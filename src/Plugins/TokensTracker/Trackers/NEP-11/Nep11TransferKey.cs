// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Nep11TransferKey.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.VM.Types;
using System;
using System.IO;

namespace Neo.Plugins.Trackers.NEP_11
{
    public class Nep11TransferKey : TokenTransferKey, IComparable<Nep11TransferKey>, IEquatable<Nep11TransferKey>
    {
        public ByteString Token;
        public override int Size => base.Size + Token.GetVarSize();

        public Nep11TransferKey() : this(new UInt160(), 0, new UInt160(), ByteString.Empty, 0)
        {
        }

        public Nep11TransferKey(UInt160 userScriptHash, ulong timestamp, UInt160 assetScriptHash, ByteString tokenId, uint xferIndex) : base(userScriptHash, timestamp, assetScriptHash, xferIndex)
        {
            Token = tokenId;
        }

        public int CompareTo(Nep11TransferKey other)
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

        public bool Equals(Nep11TransferKey other)
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
            return other is Nep11TransferKey otherKey && Equals(otherKey);
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

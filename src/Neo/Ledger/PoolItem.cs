// Copyright (C) 2021-2024 The EpicChain Labs.
//
// PoolItem.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Network.P2P.Payloads;
using System;

namespace Neo.Ledger
{
    /// <summary>
    /// Represents an item in the Memory Pool.
    ///
    ///  Note: PoolItem objects don't consider transaction priority (low or high) in their compare CompareTo method.
    ///       This is because items of differing priority are never added to the same sorted set in MemoryPool.
    /// </summary>
    internal class PoolItem : IComparable<PoolItem>
    {
        /// <summary>
        /// Internal transaction for PoolItem
        /// </summary>
        public readonly Transaction Tx;

        /// <summary>
        /// Timestamp when transaction was stored on PoolItem
        /// </summary>
        public readonly DateTime Timestamp;

        /// <summary>
        /// Timestamp when this transaction was last broadcast to other nodes
        /// </summary>
        public DateTime LastBroadcastTimestamp;

        internal PoolItem(Transaction tx)
        {
            Tx = tx;
            Timestamp = TimeProvider.Current.UtcNow;
            LastBroadcastTimestamp = Timestamp;
        }

        public int CompareTo(Transaction otherTx)
        {
            if (otherTx == null) return 1;
            int ret = (Tx.GetAttribute<HighPriorityAttribute>() != null).CompareTo(otherTx.GetAttribute<HighPriorityAttribute>() != null);
            if (ret != 0) return ret;
            // Fees sorted ascending
            ret = Tx.FeePerByte.CompareTo(otherTx.FeePerByte);
            if (ret != 0) return ret;
            ret = Tx.NetworkFee.CompareTo(otherTx.NetworkFee);
            if (ret != 0) return ret;
            // Transaction hash sorted descending
            return otherTx.Hash.CompareTo(Tx.Hash);
        }

        public int CompareTo(PoolItem otherItem)
        {
            if (otherItem == null) return 1;
            return CompareTo(otherItem.Tx);
        }
    }
}

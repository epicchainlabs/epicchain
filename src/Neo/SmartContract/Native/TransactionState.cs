// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TransactionState.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.Network.P2P.Payloads;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Linq;

namespace Neo.SmartContract.Native
{
    /// <summary>
    /// Represents a transaction that has been included in a block.
    /// </summary>
    public class TransactionState : IInteroperable
    {
        /// <summary>
        /// The block containing this transaction.
        /// </summary>
        public uint BlockIndex;

        /// <summary>
        /// The transaction, if the transaction is trimmed this value will be null
        /// </summary>
        public Transaction Transaction;

        /// <summary>
        /// The execution state
        /// </summary>
        public VMState State;

        private ReadOnlyMemory<byte> _rawTransaction;

        IInteroperable IInteroperable.Clone()
        {
            return new TransactionState
            {
                BlockIndex = BlockIndex,
                Transaction = Transaction,
                State = State,
                _rawTransaction = _rawTransaction
            };
        }

        void IInteroperable.FromReplica(IInteroperable replica)
        {
            TransactionState from = (TransactionState)replica;
            BlockIndex = from.BlockIndex;
            Transaction = from.Transaction;
            State = from.State;
            if (_rawTransaction.IsEmpty)
                _rawTransaction = from._rawTransaction;
        }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            BlockIndex = (uint)@struct[0].GetInteger();

            // Conflict record.
            if (@struct.Count == 1) return;

            // Fully-qualified transaction.
            _rawTransaction = ((ByteString)@struct[1]).Memory;
            Transaction = _rawTransaction.AsSerializable<Transaction>();
            State = (VMState)(byte)@struct[2].GetInteger();
        }

        StackItem IInteroperable.ToStackItem(ReferenceCounter referenceCounter)
        {
            if (Transaction is null)
                return new Struct(referenceCounter) { BlockIndex };
            if (_rawTransaction.IsEmpty)
                _rawTransaction = Transaction.ToArray();
            return new Struct(referenceCounter) { BlockIndex, _rawTransaction, (byte)State };
        }
    }
}

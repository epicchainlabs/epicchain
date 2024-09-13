// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Linq;

namespace EpicChain.SmartContract.Native
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

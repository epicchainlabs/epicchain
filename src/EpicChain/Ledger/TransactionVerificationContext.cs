// Copyright (C) 2021-2024 EpicChain Labs.

//
// TransactionVerificationContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract.Native;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace EpicChain.Ledger
{
    /// <summary>
    /// The context used to verify the transaction.
    /// </summary>
    public class TransactionVerificationContext
    {
        /// <summary>
        /// Store all verified unsorted transactions' senders' fee currently in the memory pool.
        /// </summary>
        private readonly Dictionary<UInt160, BigInteger> senderFee = new();

        /// <summary>
        /// Store oracle responses
        /// </summary>
        private readonly Dictionary<ulong, UInt256> oracleResponses = new();

        /// <summary>
        /// Adds a verified <see cref="Transaction"/> to the context.
        /// </summary>
        /// <param name="tx">The verified <see cref="Transaction"/>.</param>
        public void AddTransaction(Transaction tx)
        {
            var oracle = tx.GetAttribute<OracleResponse>();
            if (oracle != null) oracleResponses.Add(oracle.Id, tx.Hash);

            if (senderFee.TryGetValue(tx.Sender, out var value))
                senderFee[tx.Sender] = value + tx.SystemFee + tx.NetworkFee;
            else
                senderFee.Add(tx.Sender, tx.SystemFee + tx.NetworkFee);
        }

        /// <summary>
        /// Determine whether the specified <see cref="Transaction"/> conflicts with other transactions.
        /// </summary>
        /// <param name="tx">The specified <see cref="Transaction"/>.</param>
        /// <param name="conflictingTxs">The list of <see cref="Transaction"/> that conflicts with the specified one and are to be removed from the pool.</param>
        /// <param name="snapshot">The snapshot used to verify the <see cref="Transaction"/>.</param>
        /// <returns><see langword="true"/> if the <see cref="Transaction"/> passes the check; otherwise, <see langword="false"/>.</returns>
        public bool CheckTransaction(Transaction tx, IEnumerable<Transaction> conflictingTxs, DataCache snapshot)
        {
            BigInteger balance = NativeContract.EpicPulse.BalanceOf(snapshot, tx.Sender);
            senderFee.TryGetValue(tx.Sender, out var totalSenderFeeFromPool);

            BigInteger expectedFee = tx.SystemFee + tx.NetworkFee + totalSenderFeeFromPool;
            foreach (var conflictTx in conflictingTxs.Where(c => c.Sender.Equals(tx.Sender)))
                expectedFee -= (conflictTx.NetworkFee + conflictTx.SystemFee);
            if (balance < expectedFee) return false;

            var oracle = tx.GetAttribute<OracleResponse>();
            if (oracle != null && oracleResponses.ContainsKey(oracle.Id))
                return false;

            return true;
        }

        /// <summary>
        /// Removes a <see cref="Transaction"/> from the context.
        /// </summary>
        /// <param name="tx">The <see cref="Transaction"/> to be removed.</param>
        public void RemoveTransaction(Transaction tx)
        {
            if ((senderFee[tx.Sender] -= tx.SystemFee + tx.NetworkFee) == 0) senderFee.Remove(tx.Sender);

            var oracle = tx.GetAttribute<OracleResponse>();
            if (oracle != null) oracleResponses.Remove(oracle.Id);
        }
    }
}

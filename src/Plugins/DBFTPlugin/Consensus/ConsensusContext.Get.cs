// Copyright (C) 2021-2024 EpicChain Labs.

//
// ConsensusContext.Get.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Plugins.DBFTPlugin.Messages;
using EpicChain.SmartContract;
using System.Linq;
using System.Runtime.CompilerServices;

namespace EpicChain.Plugins.DBFTPlugin.Consensus
{
    partial class ConsensusContext
    {
        public ConsensusMessage GetMessage(ExtensiblePayload payload)
        {
            if (payload is null) return null;
            if (!cachedMessages.TryGetValue(payload.Hash, out ConsensusMessage message))
                cachedMessages.Add(payload.Hash, message = ConsensusMessage.DeserializeFrom(payload.Data));
            return message;
        }

        public T GetMessage<T>(ExtensiblePayload payload) where T : ConsensusMessage
        {
            return (T)GetMessage(payload);
        }

        private RecoveryMessage.ChangeViewPayloadCompact GetChangeViewPayloadCompact(ExtensiblePayload payload)
        {
            ChangeView message = GetMessage<ChangeView>(payload);
            return new RecoveryMessage.ChangeViewPayloadCompact
            {
                ValidatorIndex = message.ValidatorIndex,
                OriginalViewNumber = message.ViewNumber,
                Timestamp = message.Timestamp,
                InvocationScript = payload.Witness.InvocationScript
            };
        }

        private RecoveryMessage.CommitPayloadCompact GetCommitPayloadCompact(ExtensiblePayload payload)
        {
            Commit message = GetMessage<Commit>(payload);
            return new RecoveryMessage.CommitPayloadCompact
            {
                ViewNumber = message.ViewNumber,
                ValidatorIndex = message.ValidatorIndex,
                Signature = message.Signature,
                InvocationScript = payload.Witness.InvocationScript
            };
        }

        private RecoveryMessage.PreparationPayloadCompact GetPreparationPayloadCompact(ExtensiblePayload payload)
        {
            return new RecoveryMessage.PreparationPayloadCompact
            {
                ValidatorIndex = GetMessage(payload).ValidatorIndex,
                InvocationScript = payload.Witness.InvocationScript
            };
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public byte GetPrimaryIndex(byte viewNumber)
        {
            int p = ((int)Block.Index - viewNumber) % Validators.Length;
            return p >= 0 ? (byte)p : (byte)(p + Validators.Length);
        }

        public UInt160 GetSender(int index)
        {
            return Contract.CreateSignatureRedeemScript(Validators[index]).ToScriptHash();
        }

        /// <summary>
        /// Return the expected block size
        /// </summary>
        public int GetExpectedBlockSize()
        {
            return GetExpectedBlockSizeWithoutTransactions(Transactions.Count) + // Base size
                Transactions.Values.Sum(u => u.Size);   // Sum Txs
        }

        /// <summary>
        /// Return the expected block system fee
        /// </summary>
        public long GetExpectedBlockSystemFee()
        {
            return Transactions.Values.Sum(u => u.SystemFee);  // Sum Txs
        }

        /// <summary>
        /// Return the expected block size without txs
        /// </summary>
        /// <param name="expectedTransactions">Expected transactions</param>
        internal int GetExpectedBlockSizeWithoutTransactions(int expectedTransactions)
        {
            return
                sizeof(uint) +      // Version
                UInt256.Length +    // PrevHash
                UInt256.Length +    // MerkleRoot
                sizeof(ulong) +     // Timestamp
                sizeof(ulong) +     // Nonce
                sizeof(uint) +      // Index
                sizeof(byte) +      // PrimaryIndex
                UInt160.Length +    // NextConsensus
                1 + _witnessSize +  // Witness
                IO.Helper.GetVarSize(expectedTransactions);
        }
    }
}

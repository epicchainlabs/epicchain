// Copyright (C) 2021-2024 EpicChain Labs.

//
// Conflicts.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.Persistence;
using EpicChain.SmartContract.Native;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
{
    public class Conflicts : TransactionAttribute
    {
        /// <summary>
        /// Indicates the conflict transaction hash.
        /// </summary>
        public UInt256 Hash;

        public override TransactionAttributeType Type => TransactionAttributeType.Conflicts;

        public override bool AllowMultiple => true;

        public override int Size => base.Size + Hash.Size;

        protected override void DeserializeWithoutType(ref MemoryReader reader)
        {
            Hash = reader.ReadSerializable<UInt256>();
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Hash);
        }

        public override JObject ToJson()
        {
            JObject json = base.ToJson();
            json["hash"] = Hash.ToString();
            return json;
        }

        public override bool Verify(DataCache snapshot, Transaction tx)
        {
            // Only check if conflicting transaction is on chain. It's OK if the
            // conflicting transaction was in the Conflicts attribute of some other
            // on-chain transaction.
            return !NativeContract.Ledger.ContainsTransaction(snapshot, Hash);
        }

        public override long CalculateNetworkFee(DataCache snapshot, Transaction tx)
        {
            return tx.Signers.Length * base.CalculateNetworkFee(snapshot, tx);
        }
    }
}

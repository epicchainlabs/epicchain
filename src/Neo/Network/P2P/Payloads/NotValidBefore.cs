// Copyright (C) 2021-2024 The EpicChain Labs.
//
// NotValidBefore.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.Json;
using Neo.Persistence;
using Neo.SmartContract.Native;
using System.IO;

namespace Neo.Network.P2P.Payloads
{
    public class NotValidBefore : TransactionAttribute
    {
        /// <summary>
        /// Indicates that the transaction is not valid before this height.
        /// </summary>
        public uint Height;

        public override TransactionAttributeType Type => TransactionAttributeType.NotValidBefore;

        public override bool AllowMultiple => false;

        public override int Size => base.Size +
            sizeof(uint); // Height.

        protected override void DeserializeWithoutType(ref MemoryReader reader)
        {
            Height = reader.ReadUInt32();
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Height);
        }

        public override JObject ToJson()
        {
            JObject json = base.ToJson();
            json["height"] = Height;
            return json;
        }

        public override bool Verify(DataCache snapshot, Transaction tx)
        {
            var block_height = NativeContract.Ledger.CurrentIndex(snapshot);
            return block_height >= Height;
        }
    }
}

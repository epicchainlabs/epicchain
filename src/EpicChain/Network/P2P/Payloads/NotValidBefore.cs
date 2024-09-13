// Copyright (C) 2021-2024 EpicChain Labs.

//
// NotValidBefore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

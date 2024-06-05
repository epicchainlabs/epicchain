// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Utility.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Json;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract.Native;
using System.Linq;

namespace Neo.Plugins.RpcServer
{
    static class Utility
    {
        public static JObject BlockToJson(Block block, ProtocolSettings settings)
        {
            JObject json = block.ToJson(settings);
            json["tx"] = block.Transactions.Select(p => TransactionToJson(p, settings)).ToArray();
            return json;
        }

        public static JObject TransactionToJson(Transaction tx, ProtocolSettings settings)
        {
            JObject json = tx.ToJson(settings);
            json["sysfee"] = tx.SystemFee.ToString();
            json["netfee"] = tx.NetworkFee.ToString();
            return json;
        }
    }
}

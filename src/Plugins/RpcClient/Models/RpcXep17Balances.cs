// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcXep17Balances.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Json;
using EpicChain.Wallets;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace EpicChain.Network.RPC.Models
{
    public class RpcXep17Balances
    {
        public UInt160 UserScriptHash { get; set; }

        public List<RpcXep17Balance> Balances { get; set; }

        public JObject ToJson(ProtocolSettings protocolSettings)
        {
            JObject json = new();
            json["balance"] = Balances.Select(p => p.ToJson()).ToArray();
            json["address"] = UserScriptHash.ToAddress(protocolSettings.AddressVersion);
            return json;
        }

        public static RpcXep17Balances FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            RpcXep17Balances Xep17Balance = new()
            {
                Balances = ((JArray)json["balance"]).Select(p => RpcXep17Balance.FromJson((JObject)p, protocolSettings)).ToList(),
                UserScriptHash = json["address"].ToScriptHash(protocolSettings)
            };
            return Xep17Balance;
        }
    }

    public class RpcXep17Balance
    {
        public UInt160 AssetHash { get; set; }

        public BigInteger Amount { get; set; }

        public uint LastUpdatedBlock { get; set; }

        public JObject ToJson()
        {
            JObject json = new();
            json["assethash"] = AssetHash.ToString();
            json["amount"] = Amount.ToString();
            json["lastupdatedblock"] = LastUpdatedBlock;
            return json;
        }

        public static RpcXep17Balance FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            RpcXep17Balance balance = new()
            {
                AssetHash = json["assethash"].ToScriptHash(protocolSettings),
                Amount = BigInteger.Parse(json["amount"].AsString()),
                LastUpdatedBlock = (uint)json["lastupdatedblock"].AsNumber()
            };
            return balance;
        }
    }
}

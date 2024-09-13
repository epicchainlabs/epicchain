// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcXep17Transfers.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    public class RpcXep17Transfers
    {
        public UInt160 UserScriptHash { get; set; }

        public List<RpcXep17Transfer> Sent { get; set; }

        public List<RpcXep17Transfer> Received { get; set; }

        public JObject ToJson(ProtocolSettings protocolSettings)
        {
            JObject json = new();
            json["sent"] = Sent.Select(p => p.ToJson(protocolSettings)).ToArray();
            json["received"] = Received.Select(p => p.ToJson(protocolSettings)).ToArray();
            json["address"] = UserScriptHash.ToAddress(protocolSettings.AddressVersion);
            return json;
        }

        public static RpcXep17Transfers FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            RpcXep17Transfers transfers = new()
            {
                Sent = ((JArray)json["sent"]).Select(p => RpcXep17Transfer.FromJson((JObject)p, protocolSettings)).ToList(),
                Received = ((JArray)json["received"]).Select(p => RpcXep17Transfer.FromJson((JObject)p, protocolSettings)).ToList(),
                UserScriptHash = json["address"].ToScriptHash(protocolSettings)
            };
            return transfers;
        }
    }

    public class RpcXep17Transfer
    {
        public ulong TimestampMS { get; set; }

        public UInt160 AssetHash { get; set; }

        public UInt160 UserScriptHash { get; set; }

        public BigInteger Amount { get; set; }

        public uint BlockIndex { get; set; }

        public ushort TransferNotifyIndex { get; set; }

        public UInt256 TxHash { get; set; }

        public JObject ToJson(ProtocolSettings protocolSettings)
        {
            JObject json = new();
            json["timestamp"] = TimestampMS;
            json["assethash"] = AssetHash.ToString();
            json["transferaddress"] = UserScriptHash?.ToAddress(protocolSettings.AddressVersion);
            json["amount"] = Amount.ToString();
            json["blockindex"] = BlockIndex;
            json["transfernotifyindex"] = TransferNotifyIndex;
            json["txhash"] = TxHash.ToString();
            return json;
        }

        public static RpcXep17Transfer FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            return new RpcXep17Transfer
            {
                TimestampMS = (ulong)json["timestamp"].AsNumber(),
                AssetHash = json["assethash"].ToScriptHash(protocolSettings),
                UserScriptHash = json["transferaddress"]?.ToScriptHash(protocolSettings),
                Amount = BigInteger.Parse(json["amount"].AsString()),
                BlockIndex = (uint)json["blockindex"].AsNumber(),
                TransferNotifyIndex = (ushort)json["transfernotifyindex"].AsNumber(),
                TxHash = UInt256.Parse(json["txhash"].AsString())
            };
        }
    }
}

// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcVersion.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.Json;
using System;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.Network.RPC.Models
{
    public class RpcVersion
    {
        public class RpcProtocol
        {
            public uint Network { get; set; }
            public int ValidatorsCount { get; set; }
            public uint MillisecondsPerBlock { get; set; }
            public uint MaxValidUntilBlockIncrement { get; set; }
            public uint MaxTraceableBlocks { get; set; }
            public byte AddressVersion { get; set; }
            public uint MaxTransactionsPerBlock { get; set; }
            public int MemoryPoolMaxTransactions { get; set; }
            public ulong InitialEpicPulseDistribution { get; set; }
            public IReadOnlyDictionary<Hardfork, uint> Hardforks { get; set; }
            public IReadOnlyList<string> SeedList { get; set; }
            public IReadOnlyList<ECPoint> StandbyCommittee { get; set; }

            public JObject ToJson()
            {
                JObject json = new();
                json["network"] = Network;
                json["validatorscount"] = ValidatorsCount;
                json["msperblock"] = MillisecondsPerBlock;
                json["maxvaliduntilblockincrement"] = MaxValidUntilBlockIncrement;
                json["maxtraceableblocks"] = MaxTraceableBlocks;
                json["addressversion"] = AddressVersion;
                json["maxtransactionsperblock"] = MaxTransactionsPerBlock;
                json["memorypoolmaxtransactions"] = MemoryPoolMaxTransactions;
                json["InitialEpicPulseDistribution"] = InitialEpicPulseDistribution;
                json["hardforks"] = new JArray(Hardforks.Select(s => new JObject()
                {
                    // Strip HF_ prefix.
                    ["name"] = StripPrefix(s.Key.ToString(), "HF_"),
                    ["blockheight"] = s.Value,
                }));
                json["standbycommittee"] = new JArray(StandbyCommittee.Select(u => new JString(u.ToString())));
                json["seedlist"] = new JArray(SeedList.Select(u => new JString(u)));
                return json;
            }

            public static RpcProtocol FromJson(JObject json)
            {
                return new()
                {
                    Network = (uint)json["network"].AsNumber(),
                    ValidatorsCount = (int)json["validatorscount"].AsNumber(),
                    MillisecondsPerBlock = (uint)json["msperblock"].AsNumber(),
                    MaxValidUntilBlockIncrement = (uint)json["maxvaliduntilblockincrement"].AsNumber(),
                    MaxTraceableBlocks = (uint)json["maxtraceableblocks"].AsNumber(),
                    AddressVersion = (byte)json["addressversion"].AsNumber(),
                    MaxTransactionsPerBlock = (uint)json["maxtransactionsperblock"].AsNumber(),
                    MemoryPoolMaxTransactions = (int)json["memorypoolmaxtransactions"].AsNumber(),
                    InitialEpicPulseDistribution = (ulong)json["InitialEpicPulseDistribution"].AsNumber(),
                    Hardforks = new Dictionary<Hardfork, uint>(((JArray)json["hardforks"]).Select(s =>
                    {
                        var name = s["name"].AsString();
                        // Add HF_ prefix to the hardfork response for proper Hardfork enum parsing.
                        return new KeyValuePair<Hardfork, uint>(Enum.Parse<Hardfork>(name.StartsWith("HF_") ? name : $"HF_{name}"), (uint)s["blockheight"].AsNumber());
                    })),
                    SeedList = new List<string>(((JArray)json["seedlist"]).Select(s =>
                    {
                        return s.AsString();
                    })),
                    StandbyCommittee = new List<ECPoint>(((JArray)json["standbycommittee"]).Select(s =>
                    {
                        return ECPoint.Parse(s.AsString(), ECCurve.Secp256r1);
                    }))
                };
            }

            private static string StripPrefix(string s, string prefix)
            {
                return s.StartsWith(prefix) ? s.Substring(prefix.Length) : s;
            }
        }

        public int TcpPort { get; set; }

        public uint Nonce { get; set; }

        public string UserAgent { get; set; }

        public RpcProtocol Protocol { get; set; } = new();

        public JObject ToJson()
        {
            JObject json = new();
            json["network"] = Protocol.Network; // Obsolete
            json["tcpport"] = TcpPort;
            json["nonce"] = Nonce;
            json["useragent"] = UserAgent;
            json["protocol"] = Protocol.ToJson();
            return json;
        }

        public static RpcVersion FromJson(JObject json)
        {
            return new()
            {
                TcpPort = (int)json["tcpport"].AsNumber(),
                Nonce = (uint)json["nonce"].AsNumber(),
                UserAgent = json["useragent"].AsString(),
                Protocol = RpcProtocol.FromJson((JObject)json["protocol"])
            };
        }
    }
}

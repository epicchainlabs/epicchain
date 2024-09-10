// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcRawMemPool.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Neo.Json;
using System.Collections.Generic;
using System.Linq;

namespace Neo.Network.RPC.Models
{
    public class RpcRawMemPool
    {
        public uint Height { get; set; }

        public List<UInt256> Verified { get; set; }

        public List<UInt256> UnVerified { get; set; }

        public JObject ToJson()
        {
            JObject json = new();
            json["height"] = Height;
            json["verified"] = new JArray(Verified.Select(p => (JToken)p.ToString()));
            json["unverified"] = new JArray(UnVerified.Select(p => (JToken)p.ToString()));
            return json;
        }

        public static RpcRawMemPool FromJson(JObject json)
        {
            return new RpcRawMemPool
            {
                Height = uint.Parse(json["height"].AsString()),
                Verified = ((JArray)json["verified"]).Select(p => UInt256.Parse(p.AsString())).ToList(),
                UnVerified = ((JArray)json["unverified"]).Select(p => UInt256.Parse(p.AsString())).ToList()
            };
        }
    }
}

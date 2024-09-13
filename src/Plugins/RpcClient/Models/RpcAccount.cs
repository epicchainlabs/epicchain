// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcAccount.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Network.RPC.Models
{
    public class RpcAccount
    {
        public string Address { get; set; }

        public bool HasKey { get; set; }

        public string Label { get; set; }

        public bool WatchOnly { get; set; }

        public JObject ToJson()
        {
            return new JObject
            {
                ["address"] = Address,
                ["haskey"] = HasKey,
                ["label"] = Label,
                ["watchonly"] = WatchOnly
            };
        }

        public static RpcAccount FromJson(JObject json)
        {
            return new RpcAccount
            {
                Address = json["address"].AsString(),
                HasKey = json["haskey"].AsBoolean(),
                Label = json["label"]?.AsString(),
                WatchOnly = json["watchonly"].AsBoolean(),
            };
        }
    }
}

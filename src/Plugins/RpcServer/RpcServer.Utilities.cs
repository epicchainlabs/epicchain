// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcServer.Utilities.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Linq;

namespace EpicChain.Plugins.RpcServer
{
    partial class RpcServer
    {
        [RpcMethod]
        protected internal virtual JToken ListPlugins(JArray _params)
        {
            return new JArray(Plugin.Plugins
                .OrderBy(u => u.Name)
                .Select(u => new JObject
                {
                    ["name"] = u.Name,
                    ["version"] = u.Version.ToString(),
                    ["interfaces"] = new JArray(u.GetType().GetInterfaces()
                        .Select(p => p.Name)
                        .Where(p => p.EndsWith("Plugin"))
                        .Select(p => (JToken)p))
                }));
        }

        [RpcMethod]
        protected internal virtual JToken ValidateAddress(JArray _params)
        {
            string address = Result.Ok_Or(() => _params[0].AsString(), RpcError.InvalidParams.WithData($"Invlid address format: {_params[0]}"));
            JObject json = new();
            UInt160 scriptHash;
            try
            {
                scriptHash = address.ToScriptHash(system.Settings.AddressVersion);
            }
            catch
            {
                scriptHash = null;
            }
            json["address"] = address;
            json["isvalid"] = scriptHash != null;
            return json;
        }
    }
}

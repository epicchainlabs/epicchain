// Copyright (C) 2021-2024 EpicChain Labs.

//
// XEP6Contract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using System;
using System.Linq;

namespace EpicChain.Wallets.XEP6
{
    internal class XEP6Contract : Contract
    {
        public string[] ParameterNames;
        public bool Deployed;

        public static XEP6Contract FromJson(JObject json)
        {
            if (json == null) return null;
            return new XEP6Contract
            {
                Script = Convert.FromBase64String(json["script"].AsString()),
                ParameterList = ((JArray)json["parameters"]).Select(p => p["type"].GetEnum<ContractParameterType>()).ToArray(),
                ParameterNames = ((JArray)json["parameters"]).Select(p => p["name"].AsString()).ToArray(),
                Deployed = json["deployed"].AsBoolean()
            };
        }

        public JObject ToJson()
        {
            JObject contract = new();
            contract["script"] = Convert.ToBase64String(Script);
            contract["parameters"] = new JArray(ParameterList.Zip(ParameterNames, (type, name) =>
            {
                JObject parameter = new();
                parameter["name"] = name;
                parameter["type"] = type;
                return parameter;
            }));
            contract["deployed"] = Deployed;
            return contract;
        }
    }
}

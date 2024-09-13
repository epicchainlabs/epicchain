// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcInvokeResult.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Linq;

namespace EpicChain.Network.RPC.Models
{
    public class RpcInvokeResult
    {
        public string Script { get; set; }

        public VM.VMState State { get; set; }

        public long EpicPulseConsumed { get; set; }

        public StackItem[] Stack { get; set; }

        public string Tx { get; set; }

        public string Exception { get; set; }

        public string Session { get; set; }

        public JObject ToJson()
        {
            JObject json = new();
            json["script"] = Script;
            json["state"] = State;
            json["EpicPulseConsumed"] = EpicPulseConsumed.ToString();
            if (!string.IsNullOrEmpty(Exception))
                json["exception"] = Exception;
            try
            {
                json["stack"] = new JArray(Stack.Select(p => p.ToJson()));
            }
            catch (InvalidOperationException)
            {
                // ContractParameter.ToJson() may cause InvalidOperationException
                json["stack"] = "error: recursive reference";
            }
            if (!string.IsNullOrEmpty(Tx)) json["tx"] = Tx;
            return json;
        }

        public static RpcInvokeResult FromJson(JObject json)
        {
            RpcInvokeResult invokeScriptResult = new()
            {
                Script = json["script"].AsString(),
                State = json["state"].GetEnum<VMState>(),
                EpicPulseConsumed = long.Parse(json["EpicPulseConsumed"].AsString()),
                Exception = json["exception"]?.AsString(),
                Session = json["session"]?.AsString()
            };
            try
            {
                invokeScriptResult.Stack = ((JArray)json["stack"]).Select(p => Utility.StackItemFromJson((JObject)p)).ToArray();
            }
            catch { }
            invokeScriptResult.Tx = json["tx"]?.AsString();
            return invokeScriptResult;
        }
    }

    public class RpcStack
    {
        public string Type { get; set; }

        public string Value { get; set; }

        public JObject ToJson()
        {
            JObject json = new();
            json["type"] = Type;
            json["value"] = Value;
            return json;
        }

        public static RpcStack FromJson(JObject json)
        {
            return new RpcStack
            {
                Type = json["type"].AsString(),
                Value = json["value"].AsString()
            };
        }
    }
}

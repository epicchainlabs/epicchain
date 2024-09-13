// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcApplicationLog.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM;
using EpicChain.VM.Types;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.Network.RPC.Models
{
    public class RpcApplicationLog
    {
        public UInt256 TxId { get; set; }

        public UInt256 BlockHash { get; set; }

        public List<Execution> Executions { get; set; }

        public JObject ToJson()
        {
            JObject json = new JObject();
            if (TxId != null)
                json["txid"] = TxId.ToString();
            if (BlockHash != null)
                json["blockhash"] = BlockHash.ToString();
            json["executions"] = Executions.Select(p => p.ToJson()).ToArray();
            return json;
        }

        public static RpcApplicationLog FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            return new RpcApplicationLog
            {
                TxId = json["txid"] is null ? null : UInt256.Parse(json["txid"].AsString()),
                BlockHash = json["blockhash"] is null ? null : UInt256.Parse(json["blockhash"].AsString()),
                Executions = ((JArray)json["executions"]).Select(p => Execution.FromJson((JObject)p, protocolSettings)).ToList(),
            };
        }
    }

    public class Execution
    {
        public TriggerType Trigger { get; set; }

        public VMState VMState { get; set; }

        public long EpicPulseConsumed { get; set; }

        public string ExceptionMessage { get; set; }

        public List<StackItem> Stack { get; set; }

        public List<RpcNotifyEventArgs> Notifications { get; set; }

        public JObject ToJson()
        {
            JObject json = new();
            json["trigger"] = Trigger;
            json["vmstate"] = VMState;
            json["EpicPulseConsumed"] = EpicPulseConsumed.ToString();
            json["exception"] = ExceptionMessage;
            json["stack"] = Stack.Select(q => q.ToJson()).ToArray();
            json["notifications"] = Notifications.Select(q => q.ToJson()).ToArray();
            return json;
        }

        public static Execution FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            return new Execution
            {
                Trigger = json["trigger"].GetEnum<TriggerType>(),
                VMState = json["vmstate"].GetEnum<VMState>(),
                EpicPulseConsumed = long.Parse(json["EpicPulseConsumed"].AsString()),
                ExceptionMessage = json["exception"]?.AsString(),
                Stack = ((JArray)json["stack"]).Select(p => Utility.StackItemFromJson((JObject)p)).ToList(),
                Notifications = ((JArray)json["notifications"]).Select(p => RpcNotifyEventArgs.FromJson((JObject)p, protocolSettings)).ToList()
            };
        }
    }

    public class RpcNotifyEventArgs
    {
        public UInt160 Contract { get; set; }

        public string EventName { get; set; }

        public StackItem State { get; set; }

        public JObject ToJson()
        {
            JObject json = new();
            json["contract"] = Contract.ToString();
            json["eventname"] = EventName;
            json["state"] = State.ToJson();
            return json;
        }

        public static RpcNotifyEventArgs FromJson(JObject json, ProtocolSettings protocolSettings)
        {
            return new RpcNotifyEventArgs
            {
                Contract = json["contract"].ToScriptHash(protocolSettings),
                EventName = json["eventname"].AsString(),
                State = Utility.StackItemFromJson((JObject)json["state"])
            };
        }
    }
}
